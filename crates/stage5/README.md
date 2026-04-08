# Stage 5: Interior Mutability : The RefCell Escape Hatch

Welcome to Stage 5! We are introducing **Interior Mutability**, a powerful pattern that allows us to bypass some of the compiler's strict compile-time checks in exchange for runtime safety.

## 🎯 Learning Objectives
- Understand the purpose of **`RefCell`** and **`borrow_mut()`**.
- Learn about **Shared Mutability**: Mutating through a shared reference (`&`).
- Explore the **Entry Pattern**: Combining lookup and mutation atomically.
- Discover the risks of **Runtime Panics** (BorrowError).

## 🛠 The Challenge
In a shared environment (like a cache), you often have a shared reference to the entire structure, but you need to mutate a specific piece of data. `RefCell` allows us to move the borrow-checking logic from **compile-time** to **runtime**, enabling more granular access patterns.

### Key Questions to Consider:
- `❓` If `RefCell` allows mutation through `&`, does that mean we can ignore Rust's safety rules?
- `🤔` What happens if two different parts of your code try to call `.borrow_mut()` on the same `RefCell` at the same time? (Hint: Watch out for panics!).

## 🚀 Getting Started
Ensure you are in the project root and run:
```bash
cargo stage5
```
Check `src/cache.rs` to see how we use the "Granular Mutability" pattern.
