# concurrent-hash-table
Joseph Zalusky and Ryan Eng

## Usage
There are no dependencies in the program, simply run the below from the root directory `../src`.
```cargo run main```
This can be done from eustis3 with no other commands needed.

## Rust vs. C for this assignment
The Rust implementation of the concurrent hash table assignment differs from a C implementation in a few key ways. Rust enforces memory safety at compile time through ownership, borrowing, and lifetimes. The borrow checker ensures that references never outlive the data they point to and only allows access to a single reference at a time. This means that memory bugs won't be present in the same way ythat they could potentially be in the C implementation, however it also means that the writing of the program requires more effort upfront as it won't compile in the first place. 

Rust also uses RAII-based locks, meaning that acquiring a lock returns a guard object which automatically releases the lock when it goes out of scope. This removes the need for explicit lock release calls and reduces the chance of deadlocks or forgetting to release a lock. In C, this would have to be all handled manually.

The main disadvantage of Rust is that these safety checks can make the code more verbose and sometimes harder to work with initially, especially when dealing with references and lifetimes. In contrast, C allows more freedom and sometimes more straightforward pointer manipulation, but that freedom comes at the cost of potential memory corruption or subtle concurrency bugs. Overall, Rust provides a safer, more robust foundation for concurrent hash tables, especially when multiple threads are involved, despite being potentially more invovled work at the start.

## AI Usage
AI was used primarily for a boilerplate for both main.rs and hash_table.rs, as well as subsequent debugging after writing code. It generated some of the structure of the program and was used for more trivial tasks such as converting Jenkin's Hash function from C to rust, it did not however do the core of the concurrency logic. Prompts focused on creating outlines and analyzing code that was written by us to help evaluate issues with compilation and give recommendations on how to resolve them. 

GitHub CoPilot was additionally used to review PRs which is present in our repos. This caught some minor issues.
