// main.rs
mod hash_table;

use hash_table::HashTable;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let hash_table = Arc::new(HashTable::new());
    let log_file = Arc::new(Mutex::new(
        File::create("hash.log").expect("Failed to create hash.log"),
    ));

    let file = File::open("commands.txt").expect("commands.txt not found");
    let reader = BufReader::new(file);

    let mut handles = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let table = Arc::clone(&hash_table);
        let log_clone = Arc::clone(&log_file);

        let handle = thread::spawn(move || {
            let parts: Vec<&str> = line.split(',').collect();

            if parts.len() < 3 {
                println!("Invalid command format: {}", line);
                return;
            }

            let command = parts[0].trim();
            let name = parts[1].trim();
            let priority: u32 = parts[parts.len() - 1].trim().parse().unwrap();

            let mut log = log_clone.lock().unwrap();

            match command {
                "insert" => {
                    let salary: u32 = parts[2].trim().parse().unwrap();
                    table.insert(name, salary, priority, &mut log);
                }
                "update" => {
                    let salary: u32 = parts[2].trim().parse().unwrap();
                    table.update_salary(name, salary, priority, &mut log);
                }
                "delete" => {
                    table.delete(name, priority, &mut log);
                }
                "search" => {
                    table.search(name, priority, &mut log);
                }
                "print" => {
                    table.print(priority, &mut log);
                }
                _ => println!("Unknown command: {}", command),
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final print as required by assignment
    let mut log = log_file.lock().unwrap();
    hash_table.print(0, &mut log);
}
