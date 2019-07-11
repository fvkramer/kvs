
#[allow(non_snake_case)]
pub mod KvStore {
    pub struct Store {

    }

    pub fn new() -> Store {
        Store { }
    }

    impl Store {
        pub fn set(&self, _key: String, _value: String)  {
            panic!(); 
        }

        pub fn get(&self, _key: String) -> Option<String> {
            panic!();
        }

        pub fn remove(&self, _key: String) {
            panic!();
        }
    }
}