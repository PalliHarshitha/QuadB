use std::io;

// Define a function to check the first occurence of an integer in the given sorted array
fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        // Check if the current element matches the target
        if num == target {
            return Some(index);
        } else if num > target {
            break; // Since the array is sorted, if we encounter a number greater than the target, we can stop searching
        }
    }
    None
}

fn main() {
    println!("Enter a sorted array of integers separated by spaces:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a valid integer"))
        .collect();

    println!("Enter the target number:");
    let mut target = String::new();
    io::stdin().read_line(&mut target)
        .expect("Failed to read line");
    let target: i32 = target.trim().parse().expect("Not a valid integer");

    // Call the find_first_occurrence function to find the first occurrence of the target in the array
    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The index of the first occurrence of {} is {}", target, index),
        None => println!("The target number {} was not found in the array.", target),
    }
}
