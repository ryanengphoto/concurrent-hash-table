// hash_table.rs
use std::fs::File;
use std::io::Write;
use std::sync::RwLock;
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

pub struct HashTable {
    pub head: RwLock<Option<Box<HashRecord>>>,
}

impl HashTable {
    pub fn new() -> Self {
        HashTable {
            head: RwLock::new(None),
        }
    }

    fn jenkins_one_at_a_time_hash(key: &[u8]) -> u32 {
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

    pub fn insert(&self, key: &str, value: u32, priority: u32, log_file: &mut File) {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());

        log_to_file(
            log_file,
            priority,
            &format!("INSERT,{},{},{}", hashed_val, key, value),
        );
        let mut write_guard = self.head.write().unwrap();

        let mut cur_node = write_guard.as_deref();
        while let Some(node) = cur_node {
            if node.hash == hashed_val && node.name == key {
                println!("Insert failed.  Entry {} is a duplicate.", hashed_val);
                return;
            }
            cur_node = node.next.as_deref();
        }

        if write_guard.is_none() {
            *write_guard = Some(Box::new(HashRecord {
                hash: hashed_val,
                name: key.to_string(),
                salary: value,
                next: None,
            }));
            println!("Inserted {},{},{}", hashed_val, key, value);
            return;
        }

        let mut cur = write_guard.as_deref_mut();

        while let Some(node) = cur {
            if node.next.is_none() {
                node.next = Some(Box::new(HashRecord {
                    hash: hashed_val,
                    name: key.to_string(),
                    salary: value,
                    next: None,
                }));
                println!("Inserted {},{},{}", hashed_val, key, value);
                return;
            }
            cur = node.next.as_deref_mut();
        }
    }

    pub fn delete(&self, key: &str, priority: u32, log_file: &mut File) {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());

        log_to_file(
            log_file,
            priority,
            &format!("DELETE,{},{}", hashed_val, key),
        );
        let mut write_guard = self.head.write().unwrap();

        let mut cur = &mut *write_guard;

        loop {
            match cur {
                None => {
                    println!("Entry {} not deleted.  Not in database.", hashed_val);
                    return;
                }
                Some(node) if node.hash == hashed_val && node.name == key => {
                    let salary = node.salary;
                    *cur = node.next.take();
                    println!("Deleted record for {},{},{}", hashed_val, key, salary);
                    return;
                }
                Some(node) => {
                    cur = &mut node.next;
                }
            }
        }
    }

    pub fn update_salary(&self, key: &str, value: u32, priority: u32, log_file: &mut File) {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());

        log_to_file(
            log_file,
            priority,
            &format!("UPDATE,{},{},{}", hashed_val, key, value),
        );
        let mut write_guard = self.head.write().unwrap();

        let mut cur = &mut *write_guard;

        while let Some(node) = cur {
            if node.hash == hashed_val && node.name == key {
                let old_salary = node.salary;
                node.salary = value;
                println!(
                    "Updated record {} from {},{},{} to {},{},{}",
                    hashed_val, hashed_val, key, old_salary, hashed_val, key, value
                );
                return;
            }
            cur = &mut node.next;
        }

        println!("Update failed.  Entry {} not found.", hashed_val);
    }

    pub fn search(&self, key: &str, priority: u32, log_file: &mut File) -> Option<u32> {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());

        log_to_file(
            log_file,
            priority,
            &format!("SEARCH,{},{}", hashed_val, key),
        );
        let read_guard = self.head.read().unwrap();

        let mut cur = read_guard.as_deref();

        while let Some(r) = cur {
            if r.hash == hashed_val && r.name == key {
                let salary = r.salary;
                println!("Found: {},{},{}", r.hash, r.name, salary);
                return Some(salary);
            }
            cur = r.next.as_deref();
        }

        println!("Not Found:  {} not found.", key);
        None
    }

    pub fn print(&self, priority: Option<u32>, log_file: &mut File) {
        // Print out in thread log only if priority is Some
        match priority {
            Some(priority) => log_to_file(log_file, priority, "PRINT"),
            None => {}
        }

        let read_guard = self.head.read().unwrap();

        println!("Current Database:");

        let mut vec: Vec<&HashRecord> = Vec::new();
        let mut cur = read_guard.as_deref();

        while let Some(node) = cur {
            vec.push(node);
            cur = node.next.as_deref();
        }

        vec.sort_by_key(|r| r.hash);

        for r in vec {
            println!("{},{},{}", r.hash, r.name, r.salary);
        }
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
