# Stage 9: Foundation of Concurrency : Send & Sync

In Stage 9, we dive deep into the traits that make Rust's "Fearless Concurrency" possible: **Send** and **Sync**.

## 🎯 Learning Objectives
- Master the **Send** trait: Moving types across thread boundaries.
- Master the **Sync** trait: Sharing types across thread boundaries.
- Understand **Thread Safety** at the type level.
- Learn why some types (like `Rc`) are "Not Thread Safe".

## 🛠 The Challenge
Up until now, we've used `Arc` and `RwLock` to share data. But why do they work? Why can't we just use a regular pointer? In this stage, you will explore how the compiler uses `Send` and `Sync` to automatically verify that your concurrent code is free of data races.

### Key Questions to Consider:
- `❓` If a type is `Send`, does it automatically mean it's also `Sync`?
- `🤔` What happens if you try to pass an `Rc<V>` to a spawned thread? (Hint: The compiler will have something to say about it!).

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage9
```
Prepare to confront the compiler's strict thread-safety checks in `src/cache.rs`!
