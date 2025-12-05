//! A library for palindrome detection and searching.
//!
//! This crate provides efficient algorithms for checking if text is a palindrome
//! and finding the longest palindrome substring within a given text.
//!
//! # Examples
//!
//! ```
//! use palindrome::find_longest_palindrome;
//!
//! let text = "hello racecar world";
//! let longest = find_longest_palindrome(text);
//! println!("Longest palindrome: {}", longest);
//! ```
//!
//! # Modules
//!
//! - `palindrome` - Core palindrome algorithms
//! - `io` - Input/output utilities (private, used by binary only)

pub mod palindrome;
pub mod io;

// Re-export the main public API
pub use palindrome::{find_longest_palindrome, is_palindrome};
