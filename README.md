# LeetCode Solutions in Rust 🦀

A modern, organized collection of LeetCode problem solutions written in Rust using the `impl Solution` pattern that matches LeetCode's official format.

## Project Structure

```
leetcode/
├── Cargo.toml          # Project configuration
├── README.md           # This file
├── AGENTS.md           # Detailed guidelines for AI agents
├── template/
│   └── template.rs     # Template for new problems
├── src/
│   ├── lib.rs         # Common utilities (ListNode, TreeNode, helpers)
│   └── bin/           # Individual problem solutions
│       ├── p0001.rs   # Problem 1: Two Sum
│       ├── p0002.rs   # Problem 2: Add Two Numbers
│       ├── p3025.rs   # Problem 3025: Find the Number of Ways to Place People I
│       └── ...        # More problems
```

## Code Structure

All solutions follow LeetCode's standard `impl Solution` pattern:

```rust
struct Solution;

impl Solution {
    /// Main solution method with complexity analysis
    ///
    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    pub fn solution_name(/* parameters */) -> /* return type */ {
        // Implementation here
    }
    
    /// Alternative approach (if applicable)
    #[allow(dead_code)]
    pub fn solution_name_alt(/* parameters */) -> /* return type */ {
        // Alternative implementation
    }
    
    /// Helper methods (private)
    fn helper_method(/* parameters */) -> /* return type */ {
        // Helper implementation
    }
}
```

## Quick Start

### Running Individual Problems

```bash
# Run a specific problem
cargo run --bin p0001

# Run problem 2
cargo run --bin p0002

# Run problem 3025
cargo run --bin p3025
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific problem
cargo test p0001

# Run tests with output showing println! statements
cargo test -- --nocapture

# Run a specific test function
cargo test test_example1
```

### Adding a New Problem

1. Copy the template:
   ```bash
   cp template/template.rs src/bin/p0042.rs
   ```

2. Update the problem details:
   - Replace problem number and title in file header
   - Add problem description and examples
   - Update method names to match the problem

3. Implement your solution:
   - Replace placeholder methods with actual implementation
   - Add complexity analysis in documentation
   - Include alternative approaches if beneficial

4. Add comprehensive tests:
   - Test all provided examples
   - Include edge cases
   - Test alternative solutions for consistency

5. Run and validate:
   ```bash
   cargo run --bin p0042
   cargo test --bin p0042
   ```

## Features

### Common Utilities

The `src/lib.rs` file provides commonly used data structures and utilities:

- **ListNode**: Singly-linked list with helper methods
  ```rust
  use leetcode::ListNode;
  
  // Create from vector
  let list = ListNode::from_vec(vec![1, 2, 3]);
  
  // Convert to vector
  let vec = ListNode::to_vec(list);
  ```

- **TreeNode**: Binary tree node for tree problems

- **Helper functions**:
  - `print_vec(&vec)` - Pretty print vectors
  - `time_it(|| Solution::your_method())` - Measure execution time

### Example Usage in Solutions

```rust
use leetcode::ListNode;

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        // Use ListNode utilities
        // Implementation here
    }
}

fn main() {
    // Test with ListNode helpers
    let l1 = ListNode::from_vec(vec![2, 4, 3]);
    let l2 = ListNode::from_vec(vec![5, 6, 4]);
    let result = Solution::add_two_numbers(l1, l2);
    println!("Result: {:?}", ListNode::to_vec(result));
}
```

## Problem Naming Convention

- **Format**: `p{number:04}.rs` (e.g., `p0001.rs`, `p0042.rs`, `p1337.rs`)
- **Zero-padded** 4-digit numbers for consistent sorting
- **Method names** match problem context (e.g., `two_sum`, `add_two_numbers`)

## Development Workflow

### Daily Practice Routine

1. **Start a new problem**:
   ```bash
   cp template/template.rs src/bin/p$(printf "%04d" 123).rs
   ```

2. **Develop iteratively**:
   ```bash
   # Quick testing during development
   cargo run --bin p0123
   
   # Validate with comprehensive tests
   cargo test --bin p0123
   ```

3. **Compare approaches**:
   - Implement multiple solutions in the same `impl Solution` block
   - Use descriptive method names (`solution_name`, `solution_name_brute_force`)
   - Test consistency between approaches
   - Benchmark with `time_it()` helper

### Testing Strategy

- **Comprehensive coverage**: Test all examples from problem statement
- **Edge cases**: Empty inputs, single elements, boundary conditions
- **Method consistency**: If implementing multiple approaches, test they produce same results
- **Clear assertions**: Use descriptive test function names and `assert_eq!`

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::method_name(input), expected);
    }

    #[test]
    fn test_edge_cases() {
        // Test boundary conditions
        assert_eq!(Solution::method_name(edge_input), expected);
    }

    #[test]
    fn test_alternative_consistency() {
        // If multiple solutions exist, test they agree
        let input = /* test case */;
        assert_eq!(
            Solution::method_name(input.clone()),
            Solution::method_name_alt(input)
        );
    }
}
```

## Performance Optimization

- **Development**: Use `cargo run` for quick iteration
- **Benchmarking**: Use `cargo build --release` for optimized performance testing
- **Profiling**: Use `time_it()` helper to measure method execution time
- **Multiple approaches**: Implement both optimal and educational (e.g., brute force) solutions

## Examples in This Repository

### Basic Problems
- **p0001.rs** - Two Sum: HashMap optimization vs brute force
- **p0002.rs** - Add Two Numbers: Linked list manipulation with iterative and recursive approaches

### Advanced Problems  
- **p3025.rs** - Find the Number of Ways to Place People I: Geometric algorithms with comprehensive edge case testing

Each solution demonstrates:
- Clean `impl Solution` structure
- Comprehensive test coverage
- Multiple solution approaches when educational
- Time/space complexity analysis
- Proper use of common utilities

## Dependencies

This project primarily uses Rust's standard library. Common imports include:
```rust
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::{min, max, Ordering};
use leetcode::{ListNode, TreeNode};
```

To add external dependencies, update `Cargo.toml`:
```toml
[dependencies]
# Example: for additional data structures
indexmap = "2.0"
```

## Contributing

When adding new solutions:

1. **Follow the `impl Solution` pattern** consistently
2. **Include comprehensive test cases** covering all examples and edge cases  
3. **Add clear documentation** with problem description and complexity analysis
4. **Consider multiple approaches** when educational value exists
5. **Use descriptive naming** for methods and test functions

## Guidelines for AI Agents

See `AGENTS.md` for detailed guidelines on code structure, testing patterns, and implementation standards.

## License

This project is for educational purposes. LeetCode problems are owned by LeetCode.