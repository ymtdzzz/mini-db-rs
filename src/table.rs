use anyhow::Result;

use crate::{btree::BTree, buffer::BufferPoolManager, disk::PageId, tuple};

#[derive(Debug)]
pub struct SimpleTable {
    pub meta_page_id: PageId,
    // primary key index
    pub num_key_elems: usize,
}

impl SimpleTable {
    // initialize the table
    pub fn create(&mut self, bufmgr: &mut BufferPoolManager) -> Result<()> {
        let btree = BTree::create(bufmgr)?;
        self.meta_page_id = btree.meta_page_id;
        Ok(())
    }

    pub fn insert(&self, bufmgr: &mut BufferPoolManager, record: &[&[u8]]) -> Result<()> {
        let btree = BTree::new(self.meta_page_id);
        // key (index)
        let mut key = vec![];
        tuple::encode(record[..self.num_key_elems].iter(), &mut key);
        // valuep
        let mut value = vec![];
        tuple::encode(record[self.num_key_elems..].iter(), &mut value);
        btree.insert(bufmgr, &key, &value)?;
        Ok(())
    }
}
