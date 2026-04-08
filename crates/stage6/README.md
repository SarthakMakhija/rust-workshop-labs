# Stage 6: Fearless Concurrency — Mutex & RwLock

Stage 6 marks our transition into a multi-threaded world. We introduce the synchronization primitives required to protect shared data from concurrent access.

## 🎯 Learning Objectives
- Understand **Thread-Safe Mutation** in Rust.
- Learn about **`Mutex`** (Mutual Exclusion) vs **`RwLock`** (Read-Write Lock).
- Master the **RAII Guard Pattern**: Automatic locking and unlocking.
- Identify **Poisoned Locks** and how to handle them.

## 🛠 The Challenge
In single-threaded Rust, we have the "Shared XOR Mutable" rule. In multi-threaded Rust, this rule still applies, but we need the help of the OS and special hardware instructions to enforce it across different CPU cores. You will implement a cache protected by an `RwLock`, allowing many readers or one writer at a time.

### Key Questions to Consider:
- `❓` Why is using an `RwLock` often better than a `Mutex` for a cache?
- `🤔` What happens to the "Lock Tax" as the number of CPU cores increases? (Hint: See if everyone can read at the same time).

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage6
```
Explore the concurrency logic in `src/cache.rs`!
