#[allow(non_snake_case)]
pub struct KvStore {}


impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn get(&self, _key: String) -> Option<String> {
        panic!();
    }

    pub fn set(&self, _key: String, _value: String) {
        panic!();
    }

    pub fn remove(&self, _key: String) {
        panic!();
    }
}
