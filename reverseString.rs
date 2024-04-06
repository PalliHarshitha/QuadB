use std::io;

// Define a function to reverse string.
fn main() {
    println!("Enter a string to reverse:");

    //  Read the input from user and store it in mutable variable 'input'.
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line"); // Handle any errors that occur during the inout reading

    let reversed_string: String = input.trim().chars().rev().collect();

    println!("Reversed string: {}", reversed_string);
}
