use anyhow::Result;
use mini_db_rs::{
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
    query::{Filter, PlanNode, SeqScan},
    tuple,
};

fn main() -> Result<()> {
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    // SELECT * FROM hoge WHERE id >= 'w' AND id < 'z' AND first_name < 'Dave';
    let plan = Filter {
        cond: &|record| record[1].as_slice() < b"Dave",
        inner_plan: &SeqScan {
            table_meta_page_id: PageId(0),
            search_mode: mini_db_rs::query::TupleSearchMode::Key(&[b"w"]),
            while_cond: &|pkey| pkey[0].as_slice() < b"z",
        },
    };
    let mut exec = plan.start(&mut bufmgr)?;

    while let Some(record) = exec.next(&mut bufmgr)? {
        println!("{:?}", tuple::Pretty(&record));
    }
    Ok(())
}
