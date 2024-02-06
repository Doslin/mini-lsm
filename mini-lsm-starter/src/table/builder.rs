use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use bytes::{BufMut, Bytes};

use super::{BlockMeta, FileObject, SsTable};
use crate::{
    block::BlockBuilder,
    key::{KeyBytes, KeySlice},
    lsm_storage::BlockCache,
};

/// Builds an SSTable from key-value pairs.
pub struct SsTableBuilder {
    builder: BlockBuilder,
    first_key: Vec<u8>,
    last_key: Vec<u8>,
    data: Vec<u8>,
    pub(crate) meta: Vec<BlockMeta>,
    block_size: usize,
}

impl SsTableBuilder {
    /// Create a builder based on target block size.
    pub fn new(block_size: usize) -> Self {
        Self {
            builder: BlockBuilder::new(block_size),
            first_key: Vec::new(),
            last_key: Vec::new(),
            data: Vec::new(),
            meta: Vec::new(),
            block_size,
        }
    }

    /// Adds a key-value pair to SSTable.
    ///
    /// Note: You should split a new block when the current block is full.(`std::mem::replace` may
    /// be helpful here)
    pub fn add(&mut self, key: KeySlice, value: &[u8]) {
        if self.first_key.is_empty() {
            self.first_key.clear();
            self.first_key.extend(key.raw_ref());
        }
        if self.builder.add(key, value) {
            self.last_key.clear();
            self.last_key.extend(key.raw_ref());
            return;
        }
        // 当前 block 已满，需要新建 block
        self.finsh_block();
        // add the key-value pair to the next block
        assert!(self.builder.add(key, value));
        self.first_key.clear();
        self.first_key.extend(key.raw_ref());
        self.last_key.clear();
        self.last_key.extend(key.raw_ref());
    }

    /// Get the estimated size of the SSTable.
    ///
    /// Since the data blocks contain much more data than meta blocks, just return the size of data
    /// blocks here.
    pub fn estimated_size(&self) -> usize {
        self.data.len()
    }
    fn finsh_block(&mut self) {
        let builder = std::mem::replace(&mut self.builder, BlockBuilder::new(self.block_size));
        let encoded_block = builder.build().encode();
        let first_key_bt = Bytes::from(std::mem::take(&mut self.first_key));
        let last_key_bt = Bytes::from(std::mem::take(&mut self.last_key));
        // Bytes::from(std::mem::take(&mut self.last_key))
        // 会导致 last_key 被清空，所以需要使用 std::mem::take
        // Bytes::from(self.last_key.clone()) 这个比 std::mem::take 会更耗性能，因为会多一次 clone，但是 mem:take 会导致原来的变量被清空;
        self.meta.push(BlockMeta {
            offset: self.data.len(),
            first_key: KeyBytes::from_bytes(first_key_bt),
            last_key: KeyBytes::from_bytes(last_key_bt),
        });
        self.data.extend(encoded_block);
    }

    /// Builds the SSTable and writes it to the given path. Use the `FileObject` structure to manipulate the disk objects.
    pub fn build(
        mut self,
        id: usize,
        block_cache: Option<Arc<BlockCache>>,
        path: impl AsRef<Path>,
    ) -> Result<SsTable> {
        self.finsh_block();
        let mut buf = self.data;
        let meta_offset = buf.len();
        BlockMeta::encode_block_meta(&self.meta, &mut buf);
        buf.put_u32(meta_offset as u32);
        let file = FileObject::create(path.as_ref(), buf)?;
        Ok(SsTable {
            id,
            file,
            block_meta_offset: meta_offset,
            block_cache,
            first_key: self.meta.first().unwrap().first_key.clone(),
            last_key: self.meta.last().unwrap().last_key.clone(),
            block_meta: self.meta,
            bloom: None,
            max_ts: 0,
        })
    }

    #[cfg(test)]
    pub(crate) fn build_for_test(self, path: impl AsRef<Path>) -> Result<SsTable> {
        self.build(0, None, path)
    }
}
