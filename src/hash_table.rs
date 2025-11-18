use std::sync::RwLock;

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
        hash
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        let mut cur = &mut self.head;

        if cur.is_none() {
            let _write_guard = self.lock.write().unwrap();
            *cur = Some(Box::new(HashRecord {
                hash: hashed_val,
                name: key.to_string(),
                salary: value,
                next: None,
            }));
            println!("Inserted at head");
            return;
        }

        while let Some(node) = cur {
            if node.hash == hashed_val && node.name == key {
                println!("Error: Key already exists: {}", key);
                return;
            }

            if node.next.is_none() {
                let _write_guard = self.lock.write().unwrap();
                node.next = Some(Box::new(HashRecord {
                    hash: hashed_val,
                    name: key.to_string(),
                    salary: value,
                    next: None,
                }));
                println!("Appended new record: {} -> {}", key, value);
                return;
            }

            cur = &mut node.next;
        }
    }

    pub fn delete(&mut self, key: &str) {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        let _write_guard = self.lock.write().unwrap();

        let mut cur = &mut self.head;

        loop {
            let should_delete = match cur {
                Some(node) => node.hash == hashed_val && node.name == key,
                None => break,
            };

            if should_delete {
                if let Some(node) = cur {
                    *cur = node.next.take();
                    println!("Deleted {}", key);
                    return;
                }
            }

            cur = &mut cur.as_mut().unwrap().next;
        }

        println!("Key not found: {}", key);
    }


    pub fn updateSalary(&mut self, key: &str, value: u32) {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        let mut cur = &mut self.head;

        while let Some(node) = cur {
            if node.hash == hashed_val && node.name == key {
                node.salary = value;
                return;
            }
            cur = &mut node.next;
        }
    }

    pub fn search(&self, key: &str) -> Option<u32> {
        self.search_record(key).map(|r| r.salary)
    }

    fn search_record(&self, key: &str) -> Option<&HashRecord> {
        let _read = self.lock.read().unwrap();

        let mut cur = self.head.as_deref();
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());

        while let Some(r) = cur {
            if r.hash == hashed_val && r.name == key {
                return Some(r);
            }
            cur = r.next.as_deref();
        }

        None
    }

    pub fn print(&self) {
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
            println!("{} | {} | {}", r.hash, r.name, r.salary);
        }
    }
}
