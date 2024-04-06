use std::io;

// Define function to merge two sorted arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new(); // Create an empty vector to store the merged result
    let (mut i, mut j) = (0, 0); // Initialize two indices for traversing arr1 and arr2

    // Loop until either arr1 or arr2 is exhausted
    while i < arr1.len() && j < arr2.len() {
        //  Compare the current elements of both arrays and add
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    // Add remaining elements from arr1 if any
    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    // Add remaining elements from arr2 if any
    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result  // Return the merged array
}

fn main() {
    println!("Enter the first sorted array (space-separated):");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1)
        .expect("Failed to read line");

    let arr1: Vec<i32> = input1.trim().split_whitespace()
        .map(|s| s.parse().expect("Not a valid integer"))
        .collect();

    println!("Enter the second sorted array (space-separated):");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2)
        .expect("Failed to read line");

    let arr2: Vec<i32> = input2.trim().split_whitespace()
        .map(|s| s.parse().expect("Not a valid integer"))
        .collect();

    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged sorted array: {:?}", merged_array);
}
