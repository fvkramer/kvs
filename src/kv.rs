use std::collections::HashMap;
use std::path::{Path, PathBuf};


use crate::Result;

#[allow(non_snake_case)]
pub struct KvStore {
    path: PathBuf,
    map: HashMap<String, String>
}


impl KvStore {
    pub fn open(_path: &Path) -> Result<KvStore> {
        unimplemented!();
    }

    pub fn new() -> KvStore {
        unimplemented!();
    }

    pub fn get(&self, key: String) {
        unimplemented!();
        // self.map.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        unimplemented!();
        // self.map.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        unimplemented!();
        // self.map.remove(&key);
    }
}
