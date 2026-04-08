# Stage 10: Scale and Sharding — Minimizing Contention

Stage 10 focuses on **Scaling**. We move away from a single "Global Lock" and introduce **Sharding** to allow true parallel throughput.

## 🎯 Learning Objectives
- Understand **Lock Contention**: The bottleneck of concurrent systems.
- Implement **Sharding**: Partitioning data across multiple independent locks.
- Learn about **Constant Routing**: Using hashing to find the correct shard.
- Discover why the number of shards is often a **Power of 2**.

## 🛠 The Challenge
In Stage 9, even with thread-safe atomics, we had a single `RwLock` protecting the entire cache. If 100 threads try to write to 100 different keys, they still have to wait for that one lock! In this stage, you will partition the cache into N independent shards, allowing different threads to read and write in parallel as long as they touch different shards.

### Key Questions to Consider:
- `❓` Does sharding improve performance for a single-threaded application?
- `🤔` How do we decide the optimal number of shards? (Hint: Think about your CPU core count).

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage10
```
Check `src/cache.rs` for the sharding logic and hashing puzzles!
