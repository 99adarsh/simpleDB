use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

use super::mem_table::mem_table::MemTable;
use anyhow::{Ok, Result};

pub struct Engine {
    // to store the SSTable
    ss_table_dir: PathBuf,
    mem_table: MemTable, 
    // keep memtable size with some buffer space
    mem_table_size: usize  
}

impl Engine {
    pub fn new(
        storage_path: String,
        mem_table_size: usize
    ) -> Result<Self> {
        let path = PathBuf::from(storage_path);
        let _ = fs::create_dir(path.clone())?;
        Ok(Self {
            ss_table_dir: path,
            mem_table: MemTable::new(),
            mem_table_size
        })
    }

    /// If Memtable size is full,
    /// Save the memtable in the disk 
    /// Insert in the memtable
    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();

        let new_entry_size = self.mem_table.get_max_entry_size(&key, &value);
        if self.mem_table_size >= self.mem_table.size + new_entry_size {
            self.mem_table.set(key, value, timestamp)?;
        } else {
            // store the memtable in the disk
            self.mem_table = MemTable::new();
            self.mem_table.set(key, value, timestamp)?;
        }

        Ok(())
    }
}