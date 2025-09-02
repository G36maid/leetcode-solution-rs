# AGENTS.md - LeetCode Rust Repository Guide

## Build/Test Commands
- **Build project**: `cargo build` (use `cargo build --release` for optimized builds)
- **Run specific problem**: `cargo run --bin p0001` (replace `p0001` with problem number)
- **Run all tests**: `cargo test`
- **Run single test**: `cargo test p0001` (tests for specific problem) or `cargo test test_example1` (specific test function)
- **Run tests with output**: `cargo test -- --nocapture`

## Code Style Guidelines
- **File naming**: Problems use format `p{number:04}.rs` (e.g., `p0001.rs`, `p0123.rs`) in `src/bin/`
- **Imports**: Use `use leetcode::{ListNode, TreeNode};` for common data structures, standard library imports at top
- **Functions**: Public functions for solutions (`pub fn solution(...)`), alternative approaches as `solution_alt`
- **Documentation**: Include problem description, examples in rustdoc format (`//!` for file-level, `///` for functions)
- **Error handling**: Use `Option`/`Result` types, avoid panics in solutions
- **Naming**: Snake_case for functions/variables, PascalCase for types, descriptive names
- **Testing**: Include comprehensive test cases with `#[test]`, test edge cases, use descriptive test function names

## Common Utilities
- Use `ListNode::from_vec()` and `ListNode::to_vec()` for linked list operations
- Use `time_it(|| your_function())` for performance measurement
- Import common collections: `HashMap`, `HashSet`, `VecDeque`, `BinaryHeap`