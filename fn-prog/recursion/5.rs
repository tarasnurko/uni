use std::io;

fn power(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    } else if n < 0 {
        return 1.0 / power(x, -n);
    } else if n % 2 == 0 {
        let half_pow = power(x, n / 2);
        return half_pow * half_pow;
    } else {
        return x * power(x, n - 1);
    }
}

fn main() {
    println!("Enter the base number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: f64 = input.trim().parse().expect("Invalid input");

    println!("Enter the exponent:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input");

    println!("{} raised to the power of {} is: {}", x, n, power(x, n));
}
