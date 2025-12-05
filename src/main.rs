use palindrome::find_longest_palindrome;
use palindrome::io;

fn main() {
    // Read input from user
    let input = match io::read_input() {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Validate input
    if let Err(e) = io::validate_input(&input) {
        eprintln!("Error: {}", e);
        return;
    }

    // Find the longest palindrome
    let longest_palindrome = find_longest_palindrome(&input);

    // Display the result
    io::display_result(&longest_palindrome);
}
