use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{KvsError, Result};

#[allow(non_snake_case)]
pub struct KvStore {
    path: PathBuf,
}


impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        Ok(KvStore {
            path,
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
        Ok(())
        // self.map.insert(key, value);
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        unimplemented!();
        // self.map.remove(&key);
    }
}
