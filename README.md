# LeetCode Solutions in Rust 🦀

A modern, organized collection of LeetCode problem solutions written in Rust using the 2018+ module system.

## Project Structure

```
leetcode/
├── Cargo.toml          # Project configuration
├── README.md           # This file
├── template.rs         # Template for new problems
├── src/
│   ├── lib.rs         # Common utilities (ListNode, TreeNode, helpers)
│   └── bin/           # Individual problem solutions
│       ├── p0001.rs   # Problem 1: Two Sum
│       ├── p0002.rs   # Problem 2: Add Two Numbers
│       └── ...        # More problems
```

## Quick Start

### Running Individual Problems

```bash
# Run a specific problem
cargo run --bin p0001

# Run problem 2
cargo run --bin p0002
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific problem
cargo test p0001

# Run tests with output
cargo test -- --nocapture
```

### Adding a New Problem

1. Copy the template:
   ```bash
   cp template.rs src/bin/p0042.rs
   ```

2. Update the problem details in the file header

3. Implement your solution in the `solution()` function

4. Add test cases

5. Run and test:
   ```bash
   cargo run --bin p0042
   cargo test p0042
   ```

That's it! No need to update any `mod.rs` files or module declarations.

## Features

### Common Utilities

The `src/lib.rs` file provides commonly used data structures and utilities:

- **ListNode**: Singly-linked list with helper methods
  - `ListNode::from_vec(vec![1, 2, 3])` - Create from vector
  - `ListNode::to_vec(list)` - Convert to vector

- **TreeNode**: Binary tree node

- **Helper functions**:
  - `print_vec(&vec)` - Pretty print vectors
  - `time_it(|| your_function())` - Measure execution time

### Example Usage

```rust
use leetcode::ListNode;

// Create a linked list from vector
let list = ListNode::from_vec(vec![1, 2, 3]);

// Convert back to vector for testing
let result = ListNode::to_vec(some_list);
assert_eq!(result, vec![1, 2, 3]);
```

## Problem Naming Convention

- Format: `p{number:04}.rs` (e.g., `p0001.rs`, `p0042.rs`, `p1337.rs`)
- Zero-padded 4-digit numbers for easy sorting
- Descriptive comments in file headers

## Workflow Tips

### Daily Practice Routine

1. **Start a new problem**:
   ```bash
   cp template.rs src/bin/p$(printf "%04d" 123).rs
   ```

2. **Develop with quick feedback**:
   ```bash
   cargo run --bin p0123  # Test your solution
   cargo test p0123       # Run unit tests
   ```

3. **Compare different approaches**:
   - Implement multiple solutions in the same file
   - Use `solution()`, `solution_alt()`, etc.
   - Benchmark with the `time_it()` helper

### Testing Strategy

- **Include multiple test cases** in each solution file
- **Test edge cases** (empty inputs, single elements, etc.)
- **Use descriptive test names** that explain the scenario
- **Add inline examples** in doc comments

## Dependencies

Current project uses only standard library. To add external dependencies:

```toml
[dependencies]
# Example: for additional data structures
indexmap = "1.0"
```

## Performance Tips

- Use `cargo build --release` for optimized builds when benchmarking
- The `time_it()` helper function can measure execution time
- Consider multiple approaches (brute force vs. optimized) for learning

## Contributing

When adding new solutions:

1. Follow the existing code style
2. Include comprehensive test cases
3. Add clear problem description and examples
4. Consider multiple solution approaches when educational

## Examples

Check out the included examples:
- `p0001.rs` - Two Sum (HashMap approach + brute force comparison)
- `p0002.rs` - Add Two Numbers (Linked list manipulation)

## License

This project is for educational purposes. LeetCode problems are owned by LeetCode.