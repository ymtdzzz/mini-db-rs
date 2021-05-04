use anyhow::Result;
use mini_db_rs::{
    btree::BTree,
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
};

fn main() -> Result<()> {
    let disk = DiskManager::open("large.btr")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(
        &mut bufmgr,
        mini_db_rs::btree::SearchMode::Key(vec![
            0xec, 0x2c, 0xdd, 0x0e, 0x4d, 0x0c, 0x94, 0x67, 0x30, 0x58, 0xc7, 0xd7, 0xbe, 0x7b,
            0x85, 0xd2,
        ]),
    )?;

    let (key, value) = iter.next(&mut bufmgr)?.unwrap();
    println!("{:02x?} = {:02x?}", key, value);
    Ok(())
}
