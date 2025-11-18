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
        
        let hashed_val = HashTable::jenkins_one_at_a_time_hash(key.as_bytes());

        // start linked list traversal
        let mut cur = &mut self.head;

        // if list empty, just set head
        if cur.is_none() {
            let _write_guard = self.lock.write().unwrap();
            *cur = Some(Box::new(HashRecord {hash: hashed_val, name: key, salary: value, next: None,}));
            println!("Inserted at head");
            return;
        }

        // traverse to the end
        while let Some(ref mut node) = cur {
            if node.hash == hashed_val && node.name == key {
                println!("Error: Key already exists: {}", key);
                return;
            }

            if node.next.is_none() {
                let _write_guard = self.lock.write().unwrap();
                node.next = Some(Box::new(HashRecord {hash: hashed_val, name: key, salary: value, next: None,}));
                println!("Appended new record: {} -> {}", key, value);
                return;
            }

            cur = &mut node.next;
        }
    }


    pub fn delete(&mut self, key: &str) {
        let hashed_val = HashTable::jenkins_one_at_a_time_hash(key.as_bytes());
        let _write_guard = self.lock.write().unwrap();

        let mut cur = &mut self.head;

        while let Some(ref mut node) = cur {
            if node.hash == hashed_val && node.name == key {
                *cur = node.next.take();
                println!("Deleted {}", key);
                return;
            } else {
                cur = &mut node.next;
            }
        }

        println!("Key not found: {}", key);
    }

    pub fn updateSalary(&mut self, key: &str, value: u32) {
        let hashed_val = HashTable::jenkins_one_at_a_time_hash(key.as_bytes());

        // start linked list traversal
        let mut cur = &mut self.head;

        if cur.is_none() {
            return;
        }

        // traverse to the end
        while let Some(ref mut node) = cur {
            if node.hash == hashed_val && node.name == key {
                node.salary = value;
                return;
            }
            if node.next.is_none() {
                return;
            }

            cur = &mut node.next;
        }
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
}
