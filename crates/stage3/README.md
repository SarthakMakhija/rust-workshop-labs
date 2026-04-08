# Stage 3: Generic Abstractions : Flexibility & Performance

In Stage 3, we evolve our cache from a specific `String`-based implementation to a truly universal, generic data structure.

## 🎯 Learning Objectives
- Master **Generics** in Rust and the concept of **Monomorphization**.
- Use **Trait Bounds** (`Hash`, `Eq`) to define what types can be used as keys.
- Solve the "Allocation Problem" using the **`Borrow` Trait**.
- Understand **Deref Coercion** and its role in ergonomic lookups.

## 🛠 The Challenge
In Stage 1, we learned that calling `String::from("key")` just to look up a value is expensive because it allocates memory on the heap. In this stage, you will implement `get_improved<Q>`, which allows lookups using a reference (like `&str`) even when the cache owns a different type (like `String`).

### Key Questions to Consider:
- `❓` If the cache stores `String`, how does the `Borrow` trait allow us to query it with a `&str` without allocating?
- `🤔` What is the `?Sized` bound, and why is it necessary for types like `str`?

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage3
```
Detailed instructions and Socratic prompts await you in `src/cache.rs`!
