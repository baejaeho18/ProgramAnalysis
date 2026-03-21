# CSE552 Assignment 1: Type Analysis

Implement a type analysis for a subset of Rust.

## Requirements

- [rustup](https://rustup.rs/)

## Usage

```
cargo run                       # read from stdin
cargo run -- path/to/input.rs   # read from file
cargo test                      # run tests
```

## Files

- `src/analysis.rs` — **the only file you need to change and submit**
- `src/types.rs` — type definitions used by the analysis
- `src/union_find.rs` — union-find data structure (optional to use)
- `src/tests.rs` — tests
