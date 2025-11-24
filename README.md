# Longest Palindrome Finder

A Rust program that finds the longest palindromic substring in a given text using a brute force algorithm.

## Overview

This project implements a simple but effective solution to find the longest palindrome in any input string. A palindrome is a word, phrase, or sequence that reads the same backward as forward (e.g., "racecar", "level", "noon").

## Algorithm

The program uses a **brute force approach**:

1. Generate all possible substrings from the input text
2. Check each substring to see if it's a palindrome
3. Keep track of the longest palindrome found
4. Return the result

**Time Complexity:** O(n³)
- O(n²) substrings
- O(n) time to check each substring for palindrome

**Space Complexity:** O(n)
- Storage for character vector and temporary strings

## Features

- Accepts user input from stdin
- Handles edge cases (empty input, single characters)
- Displays the longest palindrome found and its length
- Clear error messages for invalid input

## Building and Running

### Prerequisites

- Rust 1.56 or later
- Cargo (comes with Rust)

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

Then enter your text when prompted:

```
Enter a text:
hello world
Longest palindrome: "ll"
Length: 2
```

## Examples

### Example 1: Clear Palindrome
```
Input: racecar
Output: Longest palindrome: "racecar"
        Length: 7
```

### Example 2: Multiple Character Palindromes
```
Input: babad
Output: Longest palindrome: "bab"
        Length: 3
```

### Example 3: Single Characters
```
Input: hello world
Output: Longest palindrome: "ll"
        Length: 2
```

### Example 4: No Multi-character Palindromes
```
Input: abcd
Output: No palindrome found!
```

## Project Structure

```
palindrome/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   └── main.rs        # Main program code
└── README.md          # This file
```

## Code Structure

### `is_palindrome(s: &str) -> bool`
Checks if a given string is a palindrome by comparing characters from both ends.

**Parameters:**
- `s`: A string slice to check

**Returns:**
- `true` if the string is a palindrome, `false` otherwise

### `find_longest_palindrome(text: &str) -> String`
Implements the brute force algorithm to find the longest palindromic substring.

**Parameters:**
- `text`: The input string to search

**Returns:**
- The longest palindrome found as a `String`
- Empty string if no palindrome exists

### `main()`
Handles user input and output, coordinating the palindrome search.

## Memory Management

The program efficiently manages memory through Rust's ownership system:

1. **Input borrowing**: The input string is borrowed (passed as `&str`) to functions, not copied
2. **Temporary allocations**: Character vectors and substring strings are created as needed and automatically freed when out of scope
3. **Result ownership**: The longest palindrome is returned and owned by the calling function

## Future Optimizations

The current brute force approach can be optimized using:

- **Expand Around Center**: O(n²) time, O(1) space
- **Dynamic Programming**: O(n²) time, O(n²) space
- **Manacher's Algorithm**: O(n) time, O(n) space

## Author

Created as a learning project for Rust fundamentals including:
- Ownership and borrowing
- String manipulation
- Pattern matching and control flow
- Function definitions and return types
