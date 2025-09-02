# AGENTS.md - LeetCode Rust Repository Guide

## Build/Test Commands
- **Build project**: `cargo build` (use `cargo build --release` for optimized builds)
- **Run specific problem**: `cargo run --bin p0001` (replace `p0001` with problem number)
- **Run all tests**: `cargo test`
- **Run single test**: `cargo test p0001` (tests for specific problem) or `cargo test test_example1` (specific test function)
- **Run tests with output**: `cargo test -- --nocapture`

## Code Structure Guidelines
- **File naming**: Problems use format `p{number:04}.rs` (e.g., `p0001.rs`, `p0123.rs`) in `src/bin/`
- **Solution format**: All solutions use `impl Solution` pattern matching LeetCode's format
- **Main function**: Contains `struct Solution;` followed by `impl Solution { ... }` block
- **Method naming**: Primary solution method matches problem context (e.g., `two_sum`, `add_two_numbers`)
- **Alternative methods**: Use suffix `_alt` or descriptive names (e.g., `two_sum_brute_force`)

## Code Style Guidelines
- **Imports**: Use `use leetcode::{ListNode, TreeNode};` for common data structures, standard library imports at top
- **Solution methods**: Public methods (`pub fn`) for main solutions, private (`fn`) for helpers
- **Alternative approaches**: Mark with `#[allow(dead_code)]` if not used in main
- **Documentation**: Include problem description, examples in rustdoc format (`//!` for file-level, `///` for methods)
- **Complexity analysis**: Include time/space complexity in method documentation
- **Error handling**: Use `Option`/`Result` types, avoid panics in solutions
- **Naming**: Snake_case for functions/variables, PascalCase for types, descriptive names

## Implementation Pattern
```rust
struct Solution;

impl Solution {
    /// Main solution method
    ///
    /// Time Complexity: O(?)
    /// Space Complexity: O(?)
    pub fn solution_name(/* params */) -> /* return type */ {
        // Implementation
    }

    /// Alternative solution (if needed)
    #[allow(dead_code)]
    pub fn solution_name_alt(/* params */) -> /* return type */ {
        // Alternative approach
    }

    /// Helper method (if needed)
    fn helper_method(/* params */) -> /* return type */ {
        // Helper implementation
    }
}
```

## Testing Guidelines
- **Test structure**: Include comprehensive test cases with `#[test]`
- **Test naming**: Use descriptive test function names (e.g., `test_example1`, `test_edge_cases`)
- **Method calls**: Use `Solution::method_name()` format in tests
- **Edge cases**: Test boundary conditions, empty inputs, single elements
- **Multiple solutions**: If implementing alternatives, test consistency between approaches
- **Expected vs actual**: Use `assert_eq!(actual, expected)` format

## Main Function Format
```rust
fn main() {
    let test_cases = vec![
        (input1, expected1),
        (input2, expected2),
        // ...
    ];

    for (i, (input, expected)) in test_cases.iter().enumerate() {
        println!("Example {}: input = {:?}", i + 1, input);
        let result = Solution::method_name(input.clone());
        println!("Output: {:?}", result);
        println!("Expected: {:?}", expected);
        println!("Passed: {}", result == *expected);
        println!();
    }
}
```

## Common Utilities
- Use `ListNode::from_vec()` and `ListNode::to_vec()` for linked list operations
- Use `time_it(|| Solution::your_method())` for performance measurement
- Import common collections: `HashMap`, `HashSet`, `VecDeque`, `BinaryHeap`
- Access helper utilities from `use leetcode::*;`

## Template Usage
- Copy `template/template.rs` when starting new problems
- Replace placeholder comments with actual problem description
- Update method names and signatures to match the problem
- Implement test cases based on provided examples
- Add complexity analysis to method documentation

## Best Practices
- **Readability**: Write self-documenting code with clear variable names
- **Efficiency**: Prioritize time complexity, then space complexity
- **Testing**: Include edge cases and validate all examples from problem statement
- **Documentation**: Explain approach and key insights in method comments
- **Consistency**: Follow the established patterns across all solutions