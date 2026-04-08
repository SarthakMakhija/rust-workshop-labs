# Stage 2: Primitive Obsession & Type Safety

In Stage 2, we move away from "Primitive Obsession" by introducing strong typing for our cache keys and values.

## 🎯 Learning Objectives
- Master the **NewType Pattern** in Rust.
- Understand **Zero-Cost Abstractions**: How wrapping a type doesn't penalize runtime performance.
- Learn the **`From` Trait** for ergonomic type conversions.
- Implement **`PartialEq`** to allow direct comparisons between custom types and primitives.

## 🛠 The Challenge
Using raw `String` for everything makes our code fragile. If we accidentally swap a `key` and a `value` in a function call, the compiler won't save us. By introducing `CacheKey` and `CacheValue` structs, we turn these logic errors into compiler errors.

### Key Questions to Consider:
- `❓` Does wrapping a `String` in a struct like `struct CacheKey(String)` increase memory usage?
- `🤔` How does implementing `From<&str>` make our tests cleaner and more readable?

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage2
```
Check `src/cache.rs` for the Socratic questions about domain modeling!
