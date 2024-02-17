use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

use super::mem_table::mem_table::MemTable;
use anyhow::{Ok, Result};

#[derive(Debug)]
pub struct Engine {
    // to store the SSTable files
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
        let _ = fs::create_dir(path.clone());
        Ok(Self {
            ss_table_dir: path,
            mem_table: MemTable::new(),
            mem_table_size
        })
    }

    /// If Memtable size is full, Save the memtable in the disk 
    /// Insert in the new memtable
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
            self.mem_table.flush(&self.ss_table_dir, timestamp)?;
            self.mem_table = MemTable::new();
            self.mem_table.set(key, value, timestamp)?;
        }

        Ok(())
    }

    /// get will return the data stored
    /// At first, checks the memtable if available -> return
    /// If not in Memtable, start iterating over stored sstable in decreasing timestamp order 
    /// return the first value got or else None
    pub fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        // Check Memtable
        if let Some(entry) = self.mem_table.get(key) {
            return Some(entry);
        }

        // TODO: Write logic for checking sstables

        return None;        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flow() {
        let mut engine = Engine::new(
            "/home/adarsh/my_files/personal/lsm-database-engine/sstable".to_owned(), 
            1024
        ).unwrap();
        
        for i in 1..60 {
            println!("Iteration {}",i);
            engine.set(i.to_string().as_bytes().to_vec(), (i+1).to_string().as_bytes().to_vec()).unwrap();
        }
    }
}