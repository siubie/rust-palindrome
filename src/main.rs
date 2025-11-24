use std::io;

/// Check if a string is a palindrome, borrowing the input string
fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

/// Find the longest palindrome using brute force, borrowing the input string
fn find_longest_palindrome(text: &str) -> String {
    //create new vector of characters from borrowed input string
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut longest = String::new();

    // Brute force: check all substrings
    for i in 0..n {
        for j in i + 1..=n {
            let substring: String = chars[i..j].iter().collect();

            // Check if substring is palindrome lend the substring
            if is_palindrome(&substring) && substring.len() > longest.len() {
                longest = substring;
            }
        }
    }

    longest
}

fn main() {
    println!("Enter a text:");
    //mutable String to hold user input
    let mut input = String::new();

    //read user input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Remove newline character, leading and trailing whitespace
    // input now holds reference to trimmed string
    let input = input.trim();

    if input.is_empty() {
        println!("Error: Input cannot be empty");
        return;
    }

    // Find the longest palindrome in the input string, type is String pass input by reference
    let longest_palindrome = find_longest_palindrome(input);

    //if no palindrome found, print message
    if longest_palindrome.is_empty() {
        println!("No palindrome found!");
    } else {
        println!("Longest palindrome: \"{}\"", longest_palindrome);
        println!("Length: {}", longest_palindrome.len());
    }
}
