use std::io;

// Define a function to check if the given string is palindrome or not
fn is_palindrome(input: &str) -> bool {
    let input = input.trim().to_lowercase(); // Convert input to lowercase and remove leading/trailing whitespace
    let reversed = input.chars().rev().collect::<String>(); // Reverse the string
    input == reversed // Check if the original and reversed strings are equal
}

fn main() {
    println!("Enter a string to check if it's a palindrome:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); // Remove newline character
    
    if is_palindrome(input) {
        println!("'{}' is a palindrome!", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
