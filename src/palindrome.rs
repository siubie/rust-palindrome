//! Palindrome detection and search algorithms.
//!
//! This module provides functions for checking if text is a palindrome
//! and finding the longest palindrome substring within a given text.

/// Check if a character slice is a palindrome
///
/// # Arguments
///
/// * `chars` - A slice of characters to check
///
/// # Returns
///
/// `true` if the character slice reads the same forwards and backwards,
/// `false` otherwise
///
/// # Examples
///
/// ```
/// let chars: Vec<char> = "racecar".chars().collect();
/// assert!(palindrome::is_palindrome(&chars));
/// ```
pub fn is_palindrome(chars: &[char]) -> bool {
    let len = chars.len();

    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

/// Find the longest palindrome substring using brute force algorithm
///
/// This function uses a brute force approach to check all possible substrings
/// and returns the longest one that is a palindrome. The algorithm has O(n³)
/// time complexity but is optimized to minimize memory allocations.
///
/// # Arguments
///
/// * `text` - A string slice to search within
///
/// # Returns
///
/// A `String` containing the longest palindrome found. Returns an empty string
/// if no palindrome is found (which shouldn't happen as single characters are
/// palindromes).
///
/// # Examples
///
/// ```
/// let result = palindrome::find_longest_palindrome("hello racecar world");
/// assert_eq!(result, " racecar ");
/// ```
pub fn find_longest_palindrome(text: &str) -> String {
    // Create vector of characters from input string
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut best_start = 0;
    let mut best_len = 0;

    // Brute force: check all substrings
    for i in 0..n {
        for j in i + 1..=n {
            let substring_len = j - i;

            // Check if substring is palindrome by passing character slice
            if is_palindrome(&chars[i..j]) && substring_len > best_len {
                best_start = i;
                best_len = substring_len;
            }
        }
    }

    // Only create the final String once from the longest palindrome found
    chars[best_start..best_start + best_len].iter().collect()
}
