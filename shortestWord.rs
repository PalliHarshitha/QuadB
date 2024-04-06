use std::io;

// Define a function if the shortest word of all other words in the sentence
fn shortest_word(sentence: &str) -> Option<&str> {
    sentence
        .split_whitespace()
        .min_by_key(|word| word.len())
}

fn main() {
    // Input string from the user
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Find the shortest word in the string
    if let Some(shortest) = shortest_word(&input) {
        println!("Shortest word: {}", shortest);
    } else {
        println!("No words found in the input");
    }
}
