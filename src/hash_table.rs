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

    pub fn jenkins_one_at_a_time_hash(key: &[u8]) -> u32 {
        let mut hash: u32 = 0;

        for &byte in key {
            hash = hash.wrapping_add(byte as u32);
            hash = hash.wrapping_add(hash << 10);
            hash ^= hash >> 6;
        }

        hash = hash.wrapping_add(hash << 3);
        hash ^= hash >> 11;
        hash = hash.wrapping_add(hash << 15);

        return hash;
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        let _write_guard = self.lock.write().unwrap();
        let hashed_val = HashTable::jenkins_one_at_a_time_hash(key.as_bytes());

        // TODO: check for duplicates before inserting,
        // probably return Result

        let new_record = HashRecord {
            hash: hashed_val,
            name: key.to_string(),
            salary: value,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_record));
    }

    pub fn delete(&mut self, key: &str) {
        let _write_guard = self.lock.write().unwrap();
        // TODO: delete node
        println!("Deleting");
    }

    pub fn updateSalary(&mut self, key: &str, value: u32) {
        let _write_guard = self.lock.write().unwrap();
        let hashed_val = HashTable::jenkins_one_at_a_time_hash(key.as_bytes());
        // TODO: find node and update salary
    }

    pub fn search(&self, key: &str) -> Option<u32> {
        self.search_record(key).map(|record| record.salary)
    }

    fn search_record(&self, key: &str) -> Option<&HashRecord> {
        let _read_guard = self.lock.read().unwrap();

        let mut current = self.head.as_deref();
        let hashed_val = HashTable::jenkins_one_at_a_time_hash(key.as_bytes());

        while let Some(record) = current {
            if record.hash == hashed_val && record.name == key {
                return Some(record);
            }

            current = record.next.as_deref();
        }

        None
    }

    /*

    pub fn print(&mut self, line: &str) {
        let _write_guard = self.lock.write().unwrap();
        // TODO: find node and update salary
        println!("{}", line);
    }

    pub fn print_all(&self) {
        let _read_guard = self.lock.read().unwrap();
        // TODO: iterate list and print
        println!("Printing all records...");
    }

    */
}
