use tempfile::tempdir;

use crate::{
    compact::{CompactionOptions, SimpleLeveledCompactionOptions},
    lsm_storage::{LsmStorageOptions, MiniLsm},
};

use super::harness::{check_compaction_ratio, compaction_bench};

#[test]
fn test_integration() {
    let dir = tempdir().unwrap();
    //  /Users/doude/code/rust/mini-lsm/mini-lsm-starter/src/tests/week1_day6.rs:123 这里的启动就没问题
    //  这里只是开启了 CompactionOptions ,怎么
    // 自己知道的太少了,看起来废劲
    // 
    let storage = MiniLsm::open(
        &dir,
        LsmStorageOptions::default_for_week2_test(CompactionOptions::Simple(
            SimpleLeveledCompactionOptions {
                level0_file_num_compaction_trigger: 2,
                max_levels: 3,
                size_ratio_percent: 200,
            },
        )),
    )
    .unwrap();


    compaction_bench(storage.clone());
    check_compaction_ratio(storage.clone());
}
