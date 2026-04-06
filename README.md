# Rust Workshop 2026: Building a Concurrent Cache

Welcome to the **Rust Workshop 2026** at **Rust India Conference**! This repository contains the hands-on labs, assignments, and examples for the workshop.

---

## 🎯 Purpose of the Repository

This repository is designed as a progressive learning journey. You'll move from basic ownership and memory management to building a high-performance, thread-safe, concurrent cache. 

The goal is to go beyond "just making it work" and understand **why** behind Rust's design choices, focusing on zero-cost abstractions, memory safety, and fearless concurrency.

## 🧭 How to Walk Through it

This is a **multi-crate Rust workspace**. Each folder in `crates/` represents a specific stage of the workshop:

- **`crates/stage1`**: Foundations , Ownership, Strings, and the "Allocation Problem".
- **`crates/stage2`**: Domain Modeling , Overcoming primitive obsession with the NewType pattern and traits.
- **`crates/stage3`**: Abstractions & Generics , Moving from Strings to Universal Types using the Borrow trait.
- **`crates/stage6`**: Concurrency & Thread Safety , Protecting shared data with locking mechanisms.

### 🚨 Important: Go Stage by Stage
The workshop is designed to be cumulative. Concepts introduced in Stage 1 are essential for Stage 2, and so on. **It is highly recommended to complete each stage sequentially.**

Each stage starts with a template that you will fill in as we progress through the workshop book.

## 📖 Workshop Book

For the full theoretical background, step-by-step instructions, and deep dives, follow the official documentation:

👉 **[tech-lessons.in/rust-workshop-2026/](https://tech-lessons.in/rust-workshop-2026/)**

## 🤔 Socratic Learning

Throughout the codebase, you will find special icons designed to help you think like a Rust developer:

| Icon | Meaning                                                                                          |
|:-----|:-------------------------------------------------------------------------------------------------|
| `❓`  | **Observations**: Highlights a specific behavior or "problem" in the current code state.         |
| `🤔` | **Questions**: Socratic prompts to challenge your understanding of ownership, memory, or safety. |
| `💡` | **Insights**: Helpful tips, ergonomic hints, or potential optimizations to explore.              |

We encourage you to try implementing the logic, observing the compiler errors, and answering these questions as you go.

## 🚀 Getting Started

We've added custom Cargo aliases to make it easy to run tests for specific sections of the workshop.

### 1. Prerequisites

Ensure you have Rust installed on your machine:

```bash
# Install Rust (Mac/Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone the Repository

```bash
git clone https://github.com/SarthakMakhija/rust-workshop-labs.git
cd rust-workshop-labs
```

### 3. Running Tests

> [!NOTE] 
> **All tests will fail initially!** Each stage is a template where key logic is replaced with `unimplemented!()`. Your goal throughout the workshop is to make these tests pass.

To verify your progress for a specific stage, use the following commands:

| Command          | Action                         |
|:-----------------|:-------------------------------|
| `cargo stage1`   | Run tests for Stage 1          |
| `cargo stage2`   | Run tests for Stage 2          |
| `cargo stage3`   | Run tests for Stage 3          |
| `cargo stage6`   | Run tests for Stage 6          |
| `cargo test-all` | Run all tests in the workspace |

---
*Created for the Rust India Conference 2026 workshop.*
