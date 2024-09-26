fn longest_valid_parentheses(s: String) -> i32 {
    let mut max_len = 0;
    let mut stack = vec![-1];

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push(i as i32);
        } else {
            stack.pop();
            if stack.is_empty() {
                stack.push(i as i32);
            } else {
                max_len = max_len.max(i as i32 - stack.last().unwrap());
            }
        }
    }

    max_len
}

fn main() {
    let s1 = "(()".to_string();
    let s2 = ")()())".to_string();
    let s3 = "".to_string();

    println!("Longest valid parentheses: {}", longest_valid_parentheses(s1)); // 2
    println!("Longest valid parentheses: {}", longest_valid_parentheses(s2)); // 4
    println!("Longest valid parentheses: {}", longest_valid_parentheses(s3)); // 0
}
