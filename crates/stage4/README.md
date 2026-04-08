# Stage 4: Mutation vs Aliasing — The Ownership Paradox

Stage 4 introduces the concept of **Safe Mutation**. We explore how to update data in the cache while adhering to Rust's fundamental rule: "Shared XOR Mutable."

## 🎯 Learning Objectives
- Master the **Exclusive Mutability** rule (`&mut`).
- Understand the **Ownership Paradox**: Why you can't have a reference to a key and a mutable reference to the map at the same time.
- Use **Closures** to perform in-place updates.
- Explore the scope of a mutable borrow.

## 🛠 The Challenge
Updating a value in a `HashMap` often requires a delicate dance. If we hold a reference to the data, the compiler won't let us mutate the container. In this stage, you will learn how to use the `Entry` API or manual scoping to perform atomic-like updates.

### Key Questions to Consider:
- `❓` Why does the compiler complain if we try to read a value and then immediately mutate it in the same scope?
- `🤔` How do closures help us decouple the "lookup" from the "mutation"?

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage4
```
Dive into `src/cache.rs` to solve these mutability puzzles!
