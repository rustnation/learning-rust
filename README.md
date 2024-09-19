# Learning Rust

`Learning Rust` is one of those things every man must do before he dies.

## Neovim Configuration

In this section I am going to register the set of features I want to install in `Neovim` to create a better development workflow.

- Find how to create code templates.
- Implement GitHub.
- Create shortcuts to navigate the `NvimTree`.
- Create shortcuts for debugging Rust applications.
- Organize the `terminal` layout. [Done]
- Integrate LazyGit [Done]

## Variable Ownership

### Rust enforces the following rules

- Values are owned by the variables assigned to them.
- As soon as the variable moves out of the scope of where it was defined, it is then deallocated from the memory.
- Values can be referenced and altered if we adhere to the rules for copying, moving, immutable borrowing, and mutable borrowing.

### Enforcing of these rules protect against the following errors

- Use after frees.
- Dangling pointers.
- Double frees.
- Segmentation faults.
- Buffer overrun.

### Copying Variables

Copying occurs when a value is copied. Once it has been copied, the new variable owns the value, while the exiting variable also owns its own value.

### Moving Variables

Moving refers to when the value is moved from one variable to another. However, unlike copying, the original variable no longer owns the value.

### Immutable borrowing of variables

An immutable borrow occurs when a variable can be referenced by another variable without having to clone or copy it.
If the borrowed variable falls out of scope, then it is not deallocated from the memory and the original reference to the value can still be used.

## Install rand with cargo

```commandline
cargo add rand
```

## Algorithms

- [Quick Sort](https://www.hackertouch.com/rust-data-structures-and-algorithms/quick-sort-in-rust.html)

## Clippy

### Prevent to run analyze with clippy in a function

Append before the function the following:

```commandline
#[allow(clippy::all)]
fn living_in_fooville(&self) -> Self {
```

### Run clippy

```commandline
cargo clippy -- -D warnings
```

## Books

- The Rust Programming Language 2nd Edition ${\color{orange}[Done]}$
- Programming Rust 2nd Edition ${\color{orange}[Done]}$
- Rust Web Development ${\color{orange}[Done]}$
- Zero to Production in Rust [In Progress]
- Rust Web Programming
- Command-Line Rust [In Progress]
- Rust Atomics and Locks [In Progress]
- Rust for Rustaceans
- Rust in Action
- Black Hat Rust
- Rust Intermediate Mastering Backend
- Rust Advanced Mastering Backend

## Courses

- Zero to Mastery [In Progress]
- Rust Programming Master Class: From Beginner to Expert
- The Complete Rust Programming Course

### Free Resources

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust Data Structures and Algorithms](https://www.hackertouch.com/rust-data-structures-and-algorithms.html)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [Actix Web](https://actix.rs/)

### Enums

- [Get Started with Rust: Enums](https://serokell.io/blog/enums-and-pattern-matching)
