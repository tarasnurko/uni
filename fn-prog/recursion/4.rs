use std::io;

fn count_steps(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    
    return count_steps(n - 1) + count_steps(n - 2);
}

fn main() {
    println!("Enter the number of steps:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    println!("Input: n = {}", n);
    println!("Output: {}", count_steps(n));
}
