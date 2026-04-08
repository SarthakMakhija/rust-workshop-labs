# Stage 12: Atomic Mechanics : Metrics & False Sharing

Stage 12 is about **Observability** and **Extreme Optimization**. We add atomic performance counters and tackle hardware-level bottlenecks like False Sharing.

## 🎯 Learning Objectives
- Master **Atomic Operations** and **Memory Ordering** (`Relaxed`, `Acquire`, `Release`).
- Understand **MESI Protocols** and how CPU caches communicate.
- Solve the **"False Sharing"** problem using **`CachePadded`**.
- Implement high-performance metrics counters (Hits, Misses, Puts).

## 🛠 The Challenge
In a high-throughput system, even a simple shared counter can become a bottleneck. If two counters sit on the same **64-byte Cache Line**, two different CPU cores will constantly "fight" for ownership of that line. In this stage, you will use the `crossbeam-utils` crate to pad your counters, ensuring they reside on different cache lines and scales linearly with your CPU.

### Key Questions to Consider:
- `❓` Why is using a `Mutex` around a simple integer counter "overkill"?
- `🤔` What is the "Invisible Storm"? (Hint: How does Core 1's update force Core 2 to trash its cache?).

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage12
```
Check `src/stats_counter.rs` for the atomic logic and padding tests!
