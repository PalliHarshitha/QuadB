use std::io;

// Define a function to find the longest common prefix among a list of strings
fn longest_common_prefix(strings: &[String]) -> String {
    // If the input list of strings is empty, return an empty string
    if strings.is_empty() {
        return String::new();
    }

    //  Get the first string in the list and set it as the current prefix
    let mut prefix = strings[0].clone();

    // Iterate through the rest of the strings in the list
    for s in &strings[1..] {
        // While the current string does not start with the prefix
        while !s.starts_with(&prefix) {
            // Remove the last character from the prefix
            prefix.pop();
        }
    }

    prefix
}

fn main() {
    // Input strings from the user
    println!("Enter the strings separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let strings: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // Find the longest common prefix
    let common_prefix = longest_common_prefix(&strings);

    // Print the result
    println!("Longest common prefix: {}", common_prefix);
}
