// hash_table.rs
use std::sync::RwLock;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct HashRecord {
    pub hash: u32,
    pub name: String,
    pub salary: u32,
    pub next: Option<Box<HashRecord>>,
}

pub fn current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

fn log_to_file(log_file: &mut File, priority: u32, message: &str) {
    let timestamp = current_timestamp();
    let log_entry = format!("{}: THREAD {} {}\n", timestamp, priority, message);
    let _ = log_file.write_all(log_entry.as_bytes());
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
        hash
    }

    pub fn insert(&mut self, key: &str, value: u32, priority: u32, log_file: &mut File) {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        
        log_to_file(log_file, priority, &format!("INSERT,{},{},{}", hashed_val, key, value));
        
        // Check if key already exists (with read lock)
        log_to_file(log_file, priority, "READ LOCK ACQUIRED");
        let _read_guard = self.lock.read().unwrap();
        let mut check_cur = self.head.as_ref();
        while let Some(node) = check_cur {
            if node.hash == hashed_val && node.name == key {
                drop(_read_guard);
                log_to_file(log_file, priority, "READ LOCK RELEASED");
                println!("Insert failed.  Entry {} is a duplicate.", hashed_val);
                return;
            }
            check_cur = node.next.as_ref();
        }
        drop(_read_guard);
        log_to_file(log_file, priority, "READ LOCK RELEASED");

        // Acquire write lock for insertion
        log_to_file(log_file, priority, "WRITE LOCK ACQUIRED");
        let _write_guard = self.lock.write().unwrap();
        
        let mut cur = &mut self.head;

        if cur.is_none() {
            *cur = Some(Box::new(HashRecord {
                hash: hashed_val,
                name: key.to_string(),
                salary: value,
                next: None,
            }));
            drop(_write_guard);
            log_to_file(log_file, priority, "WRITE LOCK RELEASED");
            println!("Inserted {},{},{}", hashed_val, key, value);
            return;
        }

        while let Some(node) = cur {
            if node.next.is_none() {
                node.next = Some(Box::new(HashRecord {
                    hash: hashed_val,
                    name: key.to_string(),
                    salary: value,
                    next: None,
                }));
                drop(_write_guard);
                log_to_file(log_file, priority, "WRITE LOCK RELEASED");
                println!("Inserted {},{},{}", hashed_val, key, value);
                return;
            }
            cur = &mut node.next;
        }
    }

    pub fn delete(&mut self, key: &str, priority: u32, log_file: &mut File) {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        
        log_to_file(log_file, priority, &format!("DELETE,{},{}", hashed_val, key));
        log_to_file(log_file, priority, "WRITE LOCK ACQUIRED");
        let _write_guard = self.lock.write().unwrap();

        let mut cur = &mut self.head;

        loop {
            let should_delete = match cur.as_ref() {
                Some(node) => node.hash == hashed_val && node.name == key,
                None => break,
            };

            if should_delete {
                if let Some(node) = cur.as_mut() {
                    let salary = node.salary;
                    *cur = node.next.take();
                    drop(_write_guard);
                    log_to_file(log_file, priority, "WRITE LOCK RELEASED");
                    println!("Deleted record for {},{},{}", hashed_val, key, salary);
                    return;
                }
            }

            cur = &mut cur.as_mut().unwrap().next;
        }

        drop(_write_guard);
        log_to_file(log_file, priority, "WRITE LOCK RELEASED");
        println!("Entry {} not deleted.  Not in database.", hashed_val);
    }

    pub fn update_salary(&mut self, key: &str, value: u32, priority: u32, log_file: &mut File) {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        
        log_to_file(log_file, priority, &format!("UPDATE,{},{},{}", hashed_val, key, value));
        log_to_file(log_file, priority, "WRITE LOCK ACQUIRED");
        let _write_guard = self.lock.write().unwrap();
        
        let mut cur = &mut self.head;

        while let Some(node) = cur {
            if node.hash == hashed_val && node.name == key {
                let old_salary = node.salary;
                node.salary = value;
                drop(_write_guard);
                log_to_file(log_file, priority, "WRITE LOCK RELEASED");
                println!("Updated record {} from {},{},{} to {},{},{}", 
                    hashed_val, hashed_val, key, old_salary, hashed_val, key, value);
                return;
            }
            cur = &mut node.next;
        }
        
        drop(_write_guard);
        log_to_file(log_file, priority, "WRITE LOCK RELEASED");
        println!("Update failed.  Entry {} not found.", hashed_val);
    }

    pub fn search(&self, key: &str, priority: u32, log_file: &mut File) -> Option<u32> {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        
        log_to_file(log_file, priority, &format!("SEARCH,{},{}", hashed_val, key));
        log_to_file(log_file, priority, "READ LOCK ACQUIRED");
        let _read_guard = self.lock.read().unwrap();
        
        let mut cur = self.head.as_deref();

        while let Some(r) = cur {
            if r.hash == hashed_val && r.name == key {
                let salary = r.salary;
                drop(_read_guard);
                log_to_file(log_file, priority, "READ LOCK RELEASED");
                println!("Found: {},{},{}", r.hash, r.name, salary);
                return Some(salary);
            }
            cur = r.next.as_deref();
        }

        drop(_read_guard);
        log_to_file(log_file, priority, "READ LOCK RELEASED");
        println!("Not Found:  {} not found.", key);
        None
    }

    pub fn print(&self, priority: u32, log_file: &mut File) {
        log_to_file(log_file, priority, "PRINT");
        log_to_file(log_file, priority, "READ LOCK ACQUIRED");
        let _read = self.lock.read().unwrap();
        
        println!("Current Database:");

        let mut vec: Vec<&HashRecord> = Vec::new();
        let mut cur = self.head.as_deref();

        while let Some(node) = cur {
            vec.push(node);
            cur = node.next.as_deref();
        }

        vec.sort_by_key(|r| r.hash);

        for r in vec {
            println!("{},{},{}", r.hash, r.name, r.salary);
        }

        drop(_read);
        log_to_file(log_file, priority, "READ LOCK RELEASED");
    }
}

mod tests {

    #[test]
    fn test_hash() {
        use super::HashTable;

        let cases = vec![
            ("a", 0xca2e9442),
            ("The quick brown fox jumps over the lazy dog", 0x519e91f5),
        ];

        for (input, expected) in cases {
            let hash_value = HashTable::jenkins_one_at_a_time_hash(input.as_bytes());
            assert_eq!(
                expected, hash_value,
                "Hash mismatch: computed {:x}, expected {:x}",
                hash_value, expected
            );
        }
    }
}
