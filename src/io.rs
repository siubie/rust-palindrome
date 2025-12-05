//! Input/output handling for the palindrome CLI application.

use std::io;

/// Read input from standard input
///
/// Prompts the user to enter text and reads a line from stdin.
///
/// # Returns
///
/// A `Result` containing the trimmed input string on success,
/// or an error message on failure.
pub fn read_input() -> Result<String, String> {
    println!("Enter a text:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read line: {}", e))?;

    Ok(input.trim().to_string())
}

/// Validate that the input is not empty
///
/// # Arguments
///
/// * `input` - A string slice to validate
///
/// # Returns
///
/// `Ok(())` if the input is valid, or an `Err` with an error message if empty.
pub fn validate_input(input: &str) -> Result<(), String> {
    if input.is_empty() {
        Err("Input cannot be empty".to_string())
    } else {
        Ok(())
    }
}

/// Display the palindrome search result
///
/// Prints the longest palindrome found and its length. If the palindrome
/// is empty, indicates that no palindrome was found.
///
/// # Arguments
///
/// * `palindrome` - The longest palindrome string found
pub fn display_result(palindrome: &str) {
    if palindrome.is_empty() {
        println!("No palindrome found!");
    } else {
        println!("Longest palindrome: \"{}\"", palindrome);
        println!("Length: {}", palindrome.len());
    }
}
