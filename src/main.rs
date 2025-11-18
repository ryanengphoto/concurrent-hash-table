mod hash_table;

use hash_table::HashTable;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let hash_table = Arc::new(Mutex::new(HashTable::new()));

    let file = File::open("commands.txt").expect("commands.txt not found");
    let reader = BufReader::new(file);

    let mut handles = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let table_clone = Arc::clone(&hash_table);
        let handle = thread::spawn(move || {
            println!("Executing command: {}", line);
            
            let mut table = table_clone.lock().unwrap();

            let parts: Vec<&str> = line.split(',').collect();

            match parts[0] {
                "insert" => table.insert(parts[1].to_string(), parts[2].parse().unwrap()),
                "update" => table.updateSalary(parts[1].to_string(), parts[2].parse().unwrap()),
                "delete" => table.delete(parts[1].to_string()),
                "search" => table.search(parts[1].to_string()),
                _ => println!("Unknown command: {}", parts[0])
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
