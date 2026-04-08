# Stage 1: Foundations — The Allocation Problem

Welcome to the first stage of the workshop! Here, we lay the groundwork for our concurrent cache by exploring Rust's core memory model.

## 🎯 Learning Objectives
- Understand **Ownership** and **Move semantics**.
- Explore the relationship between **Strings** and **Heaps**.
- Identify the **"Allocation Problem"**: Why passing owned data to a lookup function is a performance bottleneck.

## 🛠 The Challenge
In this stage, you will implement a basic `put` and `get` for a `HashMap<String, String>`. While simple, this implementation forces you to confront Rust's strict ownership rules.

### Key Questions to Consider:
- `❓` If the Cache owns the data, why can't we just return a reference to it in `get`?
- `🤔` Why do we need to call `.clone()` on a `String` just to see if it exists in the map?

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage1
```
Follow the Socratic icons `❓`, `🤔`, and `💡` in `src/cache.rs` to guide your implementation.
