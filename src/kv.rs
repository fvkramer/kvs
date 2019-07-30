// use std::collections::HashMap;
use std::path::{Path, PathBuf};


use crate::Result;

#[allow(non_snake_case)]
pub struct KvStore {
    // path: &Path,
    // map: HashMap<String, String>
}


impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        Ok(KvStore {
            // path,
        })
    }

    pub fn new() -> KvStore {
        unimplemented!();
    }

    pub fn get(&self, key: String) -> Result<(Option<String>)> {
        unimplemented!();
        // self.map.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        unimplemented!();
        // self.map.insert(key, value);
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        unimplemented!();
        // self.map.remove(&key);
    }
}