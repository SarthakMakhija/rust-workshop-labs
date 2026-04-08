# Stage 7: Reference Counting — Shared Ownership with Arc

In Stage 7, we solve the problem of sharing data across many different handles using **Arc** (Atomic Reference Counting).

## 🎯 Learning Objectives
- Understand **Shared Ownership** with `Arc`.
- Compare **`Rc`** (non-thread-safe) vs **`Arc`** (thread-safe).
- Learn how to return shared ownership from a lock.
- Identify the performance cost of **increments/decrements** on the reference count.

## 🛠 The Challenge
In Stage 6, our `get` function had to return a clone of the value because it couldn't return a reference that outlived the lock. If our values are large (like a 10MB data block), this cloning is prohibitively expensive. In this stage, you will wrap our values in an `Arc`, allowing many threads to "own" a handle to the same underlying data without copying it.

### Key Questions to Consider:
- `❓` If `Arc<V>` allows many threads to read the data, can we also mutate the data through the `Arc`?
- `🤔` What happens to the memory when the very last `Arc` handle is dropped?

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage7
```
Check `src/cache.rs` to see how shared ownership simplifies our API!
