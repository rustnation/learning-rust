# Learning Rust

`Learning Rust` is one of those things every man must do before he dies.

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

## Install rand with cargo

```commandline
cargo add rand
```

## Algorithms

- [Quick Sort](https://www.hackertouch.com/rust-data-structures-and-algorithms/quick-sort-in-rust.html)

## Resources

- The Rust Programming Language 2nd Edition
- Programming Rust 2nd Edition
- Rust Web Programming
- Rust Programming Master Class: From Beginner to Expert
- The Complete Rust Programming Course

### Free Resources

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
