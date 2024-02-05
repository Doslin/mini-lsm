#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::{Buf, BufMut, Bytes};
pub use iterator::BlockIterator;

pub(crate) const SIZEOF_U16: usize = std::mem::size_of::<u16>();
/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted key-value pairs.
/// 块是 LSM 树中读取和缓存的最小单位。它是已排序键值对的集合。
pub struct Block {
    pub(crate) data: Vec<u8>,
    pub(crate) offsets: Vec<u16>,
}

impl Block {
    /// Encode the internal data to the data layout illustrated in the tutorial
    /// Note: You may want to recheck if any of the expected field is missing from your output
    pub fn encode(&self) -> Bytes {
        // 把数据复制出来
        let mut buf = self.data.clone();
        let offsets_len = self.offsets.len();
        // 把偏移量写入
        for offset in &self.offsets {
            buf.put_u16(*offset)
        }
        buf.put_u16(offsets_len as u16);
        // 先是数据，然后是偏移量，最后是偏移量的长度
        buf.into()
    }

    /// Decode from the data layout, transform the input `data` to a single `Block`
    pub fn decode(data: &[u8]) -> Self {
        // get number of elements in the block
        // 偏移量的长度先取出来
        let entry_offsets_len = (&data[data.len() - SIZEOF_U16..]).get_u16() as usize;
        let data_end = data.len() - SIZEOF_U16 - entry_offsets_len * SIZEOF_U16;
        // 取出所有的 offset
        let offset_raw = &data[data_end..data.len() - SIZEOF_U16];
        // get offset array
        let offsets = offset_raw
            .chunks(SIZEOF_U16)
            .map(|mut x| x.get_u16())
            .collect();
        // retrieve data
        let data = data[0..data_end].to_vec();
        Self { data, offsets }
    }
}
