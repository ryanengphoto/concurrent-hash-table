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
            // TODO: parse command line
            println!("Executing command: {}", line);

            // lock table for access
            let mut table = table_clone.lock().unwrap();

            let (command, _args) = line.split_once(',').unwrap_or((&line, ""));

            match command {
                "insert" => table.insert(line),
                "update" => table.update(line),
                "delete" => table.delete(line),
                "search" => table.search(line),
                "print" => table.print(line),
                _ => println!("Unknown command: {}", command)
            }

            // TODO: call insert/delete/search/print based on command
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
