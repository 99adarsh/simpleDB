use super::red_black_tree::red_black_tree::{Color, NodePtr, RedBlackTree, Side, Status};
use anyhow::Result;
use std::{fs::{File, OpenOptions}, io::{BufWriter, Write}, mem::size_of, path::PathBuf};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct SSTableEntry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: u128
}

impl SSTableEntry {
    pub fn new(
        key: Vec<u8>,
        value: Vec<u8>,
        timestamp: u128
    ) -> Self {
        Self {
            key,
            value,
            timestamp
        }
    }
}
#[derive(Serialize,Deserialize)]
pub struct SSTable(Vec<SSTableEntry>);

#[derive(Debug)]
pub struct MemTable {
    pub size: usize,
    db_store: RedBlackTree<Vec<u8>, Vec<u8>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self {
            size: 0,
            db_store: RedBlackTree::new(),
        }
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>, timestamp: u128) -> Result<()> {
        
        match self.db_store.value(&key) {
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

    pub fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        let node = self.db_store.find_node(&key);
        if node.is_null() || node.is_deleted() {
            return None;
        }
        return Some(node.value().unwrap());
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
    /// Iterate over RB tree and store the entries in the BufWriter to flush in the disk at once
    /// Create name using timestamp, will be helpful in compaction
    pub fn flush(&self,path: &PathBuf, timestamp: u128) -> Result<()> {
        
        let file_name = path.join(timestamp.to_string() + ".sst");
        // new file must be opened using append or write access
        let file = OpenOptions::new().append(true).create(true).open(&file_name)?;

        // default capacity for bufwriter is 8KB
        let mut file = BufWriter::new(file);

        let sstable = self.create_sorted_string_table();
        let encoded_sstable = bincode::serialize(&sstable)?;
        file.write_all(&encoded_sstable)?;
        file.flush()?;

        Ok(())
    }

    // Think about writing format in sstable
    fn create_sorted_string_table(&self) -> Vec<SSTableEntry> {
        let mut iter = self.db_store.root.into_iter();
        let mut sstable = vec![];
        while let Some(node) = iter.next() {
            if node.key().is_none() || node.value().is_none() || node.timestamp().is_none() {
                panic!("Node key/value/timestamp is none");
            }
            let sstable_entry = SSTableEntry::new(
                node.key().unwrap(),
                node.value().unwrap(),
                node.timestamp().unwrap()
            );
            sstable.push(sstable_entry);
        }
        sstable
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
