use std::io;

// Define a function to find the median og given integers in the sorted array
fn find_median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        let mid_right = arr[n / 2];
        let mid_left = arr[n / 2 - 1];
        return (mid_left as f64 + mid_right as f64) / 2.0;
    } else {
        return arr[n / 2] as f64;
    }
}

fn main() {
    // Input sorted array from the user
    println!("Enter the sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    // Find the median of the array
    let median = find_median(&arr);

    // Print the result
    println!("Median of the array: {}", median);
}
