use std::{fmt, sync::RwLock};

#[derive(Debug, Clone)]
pub struct HashRecord {
    pub hash: u32,
    pub name: String,
    pub salary: u32,
}

impl fmt::Display for HashRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.hash, self.name, self.salary)
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    record: HashRecord,
    next: Option<Box<Node>>,
}

// Result types for operations
pub enum InsertResult {
    Success { record: HashRecord },
    Duplicate { hash: u32 },
}

pub enum DeleteResult {
    Success { record: HashRecord },
    NotFound { hash: u32 },
}

pub enum UpdateResult {
    Success {
        old_record: HashRecord,
        new_record: HashRecord,
    },
    NotFound {
        hash: u32,
    },
}

pub enum SearchResult {
    Found { record: HashRecord },
    NotFound { name: String },
}

pub struct HashTable {
    pub head: RwLock<Option<Box<Node>>>,
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

    pub fn insert(&self, key: &str, value: u32, priority: u32) -> InsertResult {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        let mut write_guard = self.head.write().unwrap();

        // Check for duplicates
        let mut cur_node = write_guard.as_deref();
        while let Some(node) = cur_node {
            if node.record.hash == hashed_val && node.record.name == key {
                return InsertResult::Duplicate { hash: hashed_val };
            }
            cur_node = node.next.as_deref();
        }

        let record = HashRecord {
            hash: hashed_val,
            name: key.to_string(),
            salary: value,
        };

        let new_node = Node {
            record: record.clone(),
            next: None,
        };

        // Insert at head if empty
        if write_guard.is_none() {
            *write_guard = Some(Box::new(new_node));
            return InsertResult::Success { record };
        }

        // Insert at tail
        let mut cur = write_guard.as_deref_mut();
        while let Some(node) = cur {
            if node.next.is_none() {
                node.next = Some(Box::new(new_node));
                return InsertResult::Success { record };
            }
            cur = node.next.as_deref_mut();
        }

        unreachable!()
    }

    pub fn delete(&self, key: &str, priority: u32) -> DeleteResult {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        let mut write_guard = self.head.write().unwrap();
        let mut cur = &mut *write_guard;

        loop {
            match cur {
                None => {
                    return DeleteResult::NotFound { hash: hashed_val };
                }
                Some(node) if node.record.hash == hashed_val && node.record.name == key => {
                    let result = DeleteResult::Success {
                        record: node.record.clone(),
                    };
                    *cur = node.next.take();
                    return result;
                }
                Some(node) => {
                    cur = &mut node.next;
                }
            }
        }
    }

    pub fn update_salary(&self, key: &str, value: u32, priority: u32) -> UpdateResult {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        let mut write_guard = self.head.write().unwrap();
        let mut cur = &mut *write_guard;

        while let Some(node) = cur {
            if node.record.hash == hashed_val && node.record.name == key {
                let old_record = node.record.clone();
                node.record.salary = value;
                let new_record = node.record.clone();

                return UpdateResult::Success {
                    old_record,
                    new_record,
                };
            }
            cur = &mut node.next;
        }

        UpdateResult::NotFound { hash: hashed_val }
    }

    pub fn search(&self, key: &str, priority: u32) -> SearchResult {
        let hashed_val = Self::jenkins_one_at_a_time_hash(key.as_bytes());
        let read_guard = self.head.read().unwrap();
        let mut cur = read_guard.as_deref();

        while let Some(r) = cur {
            if r.record.hash == hashed_val && r.record.name == key {
                return SearchResult::Found {
                    record: r.record.clone(),
                };
            }
            cur = r.next.as_deref();
        }

        SearchResult::NotFound {
            name: key.to_string(),
        }
    }

    // Sorted by hash
    pub fn get_all_records(&self) -> Vec<HashRecord> {
        let read_guard = self.head.read().unwrap();
        let mut vec: Vec<HashRecord> = Vec::new();
        let mut cur = read_guard.as_deref();

        while let Some(node) = cur {
            vec.push(node.record.clone());
            cur = node.next.as_deref();
        }

        vec.sort_by_key(|r| r.hash);
        vec
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
