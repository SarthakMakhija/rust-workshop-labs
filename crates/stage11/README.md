# Stage 11: Real-World Latency : Time-To-Live & Background Cleanup

In Stage 11, we add a critical production feature: **Cache Expiration**. We implement Time-To-Live (TTL) and spawn background threads to keep our memory usage bounded.

## 🎯 Learning Objectives
- Understand **Time-To-Live (TTL)** and metadata management.
- Learn how to spawn **Background Cleaner Threads**.
- Explore **Non-Cooperative Cleanup**: Removing data without the user asking for it.
- Identify the risks of **Dangling Threads** and memory leaks.

## 🛠 The Challenge
A cache that never forgets is just a memory leak! In this stage, you will add a `Duration` to every `put` operation. You will also implement a background loop that periodically scans shards and removes expired entries. This introduces a new challenge: how do we share the shards safely between the main application handles and the cleaner thread?

### Key Questions to Consider:
- `❓` If the background thread is busy cleaning Shard A, does it block a user from reading from Shard B?
- `🤔` What happens to the background thread if the `Cache` object is dropped? (Hint: Does the thread keep running forever?).

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage11
```
Check `src/cache.rs` and the new `src/shard.rs` for the expiration logic!
