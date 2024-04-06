use std::io;

// Define function to check if a given number is prime or not.
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    if num <= 3 {
        return true; // 2 and 3 are prime
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false; // Numbers divisible by 2 or 3 are not prime (except for 2 and 3 themselves)
    }

    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false; // Numbers divisible by i or i + 2 are not prime
        }
        i += 6;
    }
    true
}

fn main() {
    println!("Enter a number to check if it's prime:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num: u64 = input.trim().parse().expect("Not a valid number");

    //  Check if the entered number is prime using our `is_prime` function,
    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}
