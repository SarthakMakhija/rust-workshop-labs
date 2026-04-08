# Stage 13: Graceful Termination — Lifecycle & Type-States

In Stage 13, we master **System Lifecycles**. We implement the **Handle-Body Pattern** and use **Atomic Coordination** to ensure our cache shuts down safely without leaking threads or memory.

## 🎯 Learning Objectives
- Master the **Handle-Body Pattern**: Separating the public `Cache` (handle) from the shared `CacheInner` (data).
- Implement the **Type-State Pattern**: Using Rust's ownership system to enforce safe state transitions.
- Coordinate **Graceful Shutdown**: Signaling background threads to stop and joining them safely.
- Use **AtomicBool** for cross-thread signaling.

## 🛠 The Challenge
In a production system, shutting down is just as important as starting up. If we simply drop the `Cache` handle, our background threads might keep running forever. In this stage, you will decouple the `Cache` struct into a handle that can be cloned safely, and an "Inner" struct that contains the actual data. You will also implement a `shutdown()` method that consumes the handle and ensures all background work is finished.

### Key Questions to Consider:
- `❓` Why does the `shutdown` method take `self` by value? (Hint: Can you shut down a cache twice?).
- `🤔` How do we ensure that new `put`/`get` calls return an error while the system is in the process of shutting down?

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage13
```
Check `src/cache.rs` for the lifecycle coordination logic!
