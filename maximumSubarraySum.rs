use std::io;

// Define a function to find the maximum sunarray sum
fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = std::i32::MIN;
    let mut current_sum = 0;

    for &num in nums {
        current_sum = current_sum.max(0) + num;
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    println!("Enter the array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    //  Convert string into vector of integers
    let nums: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Not a valid integer"))
        .collect();

    // Find the maximum subarray sum
    let max_sum = max_subarray_sum(&nums);
    println!("Maximum subarray sum: {}", max_sum);
}
