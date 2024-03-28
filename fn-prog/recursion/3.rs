use std::io;

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    println!("Enter the value of n:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u64 = input.trim().parse().expect("Invalid input");

    println!("Input: n = {}", n);
    println!("Output: {}", fibonacci(n));
}
