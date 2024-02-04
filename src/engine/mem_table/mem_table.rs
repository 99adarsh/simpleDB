use super::red_black_tree::red_black_tree::{Color, NodePtr, RedBlackTree, Side, Status};
use anyhow::Result;
use std::{fs::{File, OpenOptions}, io::BufWriter, mem::size_of, path::PathBuf};
// pub struct MemTable<K:Ord,V:Clone> {
//     pub size: u64,
//     pub store: RedBlackTree<K,V>
// }
pub struct MemTable {
    pub size: usize,
    pub db_store: RedBlackTree<Vec<u8>, Vec<u8>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self {
            size: 0,
            db_store: RedBlackTree::new(),
        }
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>, timestamp: u128) -> Result<()> {
        
        match self.db_store.get_value(&key) {
            Some(v) => {
                if value.len() > v.len() {
                    self.size += value.len() - v.len();
                } else {
                    self.size -= v.len() - value.len();
                }
            },
            // this means node is not present in the tree(Refer implementation)
            None => {
                self.size += self.get_max_entry_size(&key, &value);
            }
        }

        self.db_store
            .insert_or_replace(key, value, timestamp, Status::Available);

        Ok(())
    }

    pub fn delete(&mut self, key: Vec<u8>, timestamp: u128) -> Result<()> {

        if !self.db_store.has_node(&key) {
            let empty_value = Vec::new();
            self.size += self.get_max_entry_size(&key, &empty_value);
            self.db_store
                .insert_or_replace(key, empty_value, timestamp, Status::Deleted);
        } else {
            self.db_store.delete_key(&key, timestamp);
        }

        Ok(())
    }

    /// Flush Memtable in the disk
    /// 
    /// Creates a file in the sstable directory
    /// Iterate over the RB tree and store the entries in the BufWriter to flush at once in the disk
    pub fn flush(&self,path: PathBuf, timestamp: u128) -> Result<()> {
        
        let file_name = path.join(timestamp.to_string() + ".sst");
        let file = OpenOptions::new().create(true).open(file_name)?;
        // default capacity for bufwriter is 8KB
        let file = BufWriter::new(file);

        // function in the tree to iterate and store in this bufwriter



        Ok(())
    }

    // Think about writing format in sstable
    fn create_sorted_string(&self, file: &mut BufWriter<File>) {
        let mut iter = self.db_store.root.into_iter();

        // while let Some(x) = iter.next() {

        // }
    }

    // pub fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>> {
    //     self.db_store.get_value(key)
    // }

    #[inline]
    pub fn get_max_entry_size(&self, key: &Vec<u8>, value: &Vec<u8>) -> usize {
        
        let new_insert_size = key.len()
            + value.len()
            + 16
            + 3 * size_of::<NodePtr<Vec<u8>, Vec<u8>>>()
            + size_of::<Side>()
            + size_of::<Color>()
            + size_of::<Status>();

        return new_insert_size;
    }
}
