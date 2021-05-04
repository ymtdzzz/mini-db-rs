use anyhow::Result;
use mini_db_rs::{
    btree::{BTree, SearchMode},
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
};

fn main() -> Result<()> {
    let disk = DiskManager::open("test.btr")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree
        .search(&mut bufmgr, SearchMode::Key(b"Hyogo".to_vec()))
        .unwrap();
    let (key, value) = iter.next(&mut bufmgr)?.unwrap();
    println!("{:02x?} = {:02x?}", key, value);
    Ok(())
}
