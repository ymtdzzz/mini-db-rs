use anyhow::Result;
use mini_db_rs::{
    btree::BTree,
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
    tuple,
};

fn main() -> Result<()> {
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut search_key = vec![];
    tuple::encode([b"y"].iter(), &mut search_key);
    let mut iter = btree.search(&mut bufmgr, mini_db_rs::btree::SearchMode::Key(search_key))?;

    while let Some((key, value)) = iter.next(&mut bufmgr)? {
        let mut record = vec![];
        tuple::decode(&key, &mut record);
        tuple::decode(&value, &mut record);
        println!("{:?}", tuple::Pretty(&record));
    }
    Ok(())
}
