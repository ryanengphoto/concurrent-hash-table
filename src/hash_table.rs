use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct HashRecord {
    pub hash: u32,
    pub name: String,
    pub salary: u32,
    pub next: Option<Box<HashRecord>>,
}

#[derive(Debug)]
pub struct HashTable {
    pub head: Option<Box<HashRecord>>,
    pub lock: RwLock<()>,
}

impl HashTable {
    pub fn new() -> Self {
        HashTable {
            head: None,
            lock: RwLock::new(()),
        }
    }

    pub fn insert(&mut self, line: String) {
        let _write_guard = self.lock.write().unwrap();
        // TODO: compute Jenkins hash and insert/update node
        println!("Inserting");
    }

    pub fn update(&mut self, line: String) {
        let _write_guard = self.lock.write().unwrap();
        // TODO: compute Jenkins hash and insert/update node
        println!("Updating");
    }

    pub fn delete(&mut self, line: String) {
        let _write_guard = self.lock.write().unwrap();
        // TODO: delete node
        println!("Deleting");
    }

    pub fn update_salary(&mut self, line: String) {
        let _write_guard = self.lock.write().unwrap();
        // TODO: find node and update salary
        println!("Updating");
    }

    pub fn search(&self, line: String) {
        let _read_guard = self.lock.read().unwrap();
        // TODO: search for node
        println!("Searching for");
    }

    pub fn print(&mut self, line: String) {
        let _write_guard = self.lock.write().unwrap();
        // TODO: find node and update salary
        println!("{}", line);
    }

    pub fn print_all(&self) {
        let _read_guard = self.lock.read().unwrap();
        // TODO: iterate list and print
        println!("Printing all records...");
    }
}
