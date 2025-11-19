// main.rs
mod hash_table;

use hash_table::HashTable;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

enum Command {
    Insert { name: String, salary: u32 },
    Delete { name: String },
    Update { name: String, salary: u32 },
    Search { name: String },
    Print,
}

fn main() {
    let hash_table = Arc::new(HashTable::new());
    let log_file = Arc::new(Mutex::new(
        File::create("hash.log").expect("Failed to create hash.log"),
    ));

    let file = File::open("commands.txt").expect("commands.txt not found");
    let reader = BufReader::new(file);

    let mut handles = vec![];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 2 {
            println!("Invalid command format: {}", line);
            continue;
        }

        let command_str = parts[0].trim();
        let priority: u32 = parts[parts.len() - 1].trim().parse().unwrap();

        let command = match command_str {
            "insert" => Command::Insert {
                name: parts[1].trim().to_string(),
                salary: parts[2].trim().parse().unwrap(),
            },
            "delete" => Command::Delete {
                name: parts[1].trim().to_string(),
            },
            "update" => Command::Update {
                name: parts[1].trim().to_string(),
                salary: parts[2].trim().parse().unwrap(),
            },
            "search" => Command::Search {
                name: parts[1].trim().to_string(),
            },
            "print" => Command::Print,
            "threads" => {
                // "threads" command is no longer used.
                continue;
            }
            _ => {
                println!("Unknown command: {}", command_str);
                continue;
            }
        };

        let table = Arc::clone(&hash_table);
        let log_clone = Arc::clone(&log_file);

        let handle = thread::spawn(move || {
            let mut log = log_clone.lock().unwrap();

            match command {
                Command::Insert { name, salary } => {
                    table.insert(&name, salary, priority, &mut log);
                }
                Command::Delete { name } => {
                    table.delete(&name, priority, &mut log);
                }
                Command::Update { name, salary } => {
                    table.update_salary(&name, salary, priority, &mut log);
                }
                Command::Search { name } => {
                    table.search(&name, priority, &mut log);
                }
                Command::Print => {
                    table.print(priority, &mut log);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final print as required by assignment
    let mut log = log_file.lock().unwrap();
    hash_table.print(u32::MAX, &mut log);
}
