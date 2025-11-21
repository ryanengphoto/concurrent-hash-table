// main.rs
mod hash_table;
mod logger;

use hash_table::{DeleteResult, HashTable, InsertResult, SearchResult, UpdateResult};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use crate::logger::{LogMessage, ThreadLogger};

enum Command {
    Insert { name: String, salary: u32 },
    Delete { name: String },
    Update { name: String, salary: u32 },
    Search { name: String },
    Print,
}

struct CommandWithPriority {
    command: Command,
    priority: u32,
}

struct TurnManager {
    current_turn: Mutex<u32>,
    condvar: Condvar,
}

impl TurnManager {
    fn new(start_turn: u32) -> Self {
        TurnManager {
            current_turn: Mutex::new(start_turn),
            condvar: Condvar::new(),
        }
    }
}

fn main() {
    let logger = Arc::new(ThreadLogger::new("hash.log"));
    let hash_table = Arc::new(HashTable::new(Arc::clone(&logger)));

    let file = File::open("commands.txt").expect("commands.txt not found");
    let reader = BufReader::new(file);

    let mut commands = vec![];

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
        commands.push(CommandWithPriority { command, priority });
    }

    // Sort commands by priority
    commands.sort_by_key(|k| k.priority);

    let turn_manager = Arc::new(TurnManager::new(0));
    let mut handles = vec![];

    for (thread_id, CommandWithPriority { command, priority }) in commands.into_iter().enumerate() {
        let table = Arc::clone(&hash_table);
        let turn_manager_clone = Arc::clone(&turn_manager);

        let logger = Arc::clone(&logger);
        let handle = thread::spawn(move || {
            logger.log_id(
                thread_id as u32,
                LogMessage::Custom("WAITING FOR MY TURN".to_string()),
            );

            let mut turn = turn_manager_clone.current_turn.lock().unwrap();

            while *turn != thread_id as u32 {
                turn = turn_manager_clone.condvar.wait(turn).unwrap();
            }

            *turn += 1;

            turn_manager_clone.condvar.notify_all();

            logger.log_id(
                thread_id as u32,
                LogMessage::Custom("AWAKENED FOR WORK".to_string()),
            );

            match command {
                Command::Insert { name, salary } => {
                    let result = table.insert(&name, salary, priority);
                    match result {
                        InsertResult::Success { record } => {
                            println!("Inserted {}", record);
                        }
                        InsertResult::Duplicate { hash } => {
                            println!("Duplicate entry for {},{}", hash, name);
                        }
                    }
                }
                Command::Delete { name } => {
                    let result = table.delete(&name, priority);
                    match result {
                        DeleteResult::Success { record } => {
                            println!("Deleted record for {}", record);
                        }
                        DeleteResult::NotFound { .. } => {
                            println!("{} not found", name);
                        }
                    }
                }
                Command::Update { name, salary } => {
                    let result = table.update_salary(&name, salary, priority);
                    match result {
                        UpdateResult::Success {
                            old_record,
                            new_record,
                        } => {
                            println!(
                                "Updated record {} from {} to {}",
                                old_record.hash, old_record, new_record
                            );
                        }
                        UpdateResult::NotFound { hash } => {
                            println!("Update failed. Entry {} not found.", hash);
                        }
                    }
                }
                Command::Search { name } => {
                    let result = table.search(&name, priority);
                    match result {
                        SearchResult::Found { record } => {
                            println!("Found: {}", record);
                        }
                        SearchResult::NotFound { name } => {
                            println!("{} not found.", name);
                        }
                    }
                }
                Command::Print => {
                    println!("Current Database:");
                    let result = table.get_all_records(priority);
                    result.iter().for_each(|record| {
                        println!("{}", record);
                    });
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final compulsory stdout print. This prints with thread ID 0,
    // since all threads have completed and we're calling this from the main thread.
    println!("Final Table:");
    hash_table.get_all_records(0).iter().for_each(|record| {
        println!("{}", record);
    });

    // This is also called AFTER the thread log, so it won't include the final read lock
    // acquisition - the original expected output doesn't.
    // Final log summary of table to hash.log along with lock statistics.
    hash_table.log_summary();
}
