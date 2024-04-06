use std::io;

// Define a function to find the kth smallest element in the array
fn kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
    arr.sort(); // Sort the array in ascending order
    arr.get(k - 1).copied() // Get the kth element (0-indexed) if it exists
}

fn main() {
    // Input array from the user
    println!("Enter the elements of the array separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    // Input k from the user
    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read input");
    let k: usize = k_input.trim().parse().expect("Invalid input");

    // Find the kth smallest element
    match kth_smallest(&mut arr.clone(), k) {
        Some(val) => println!("The {}th smallest element is: {}", k, val),
        None => println!("Invalid value of k"),
    }
}
