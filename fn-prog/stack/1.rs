fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => {
                if Some(c) != stack.pop() {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

fn main() {
    let s1 = String::from("()");
    let s2 = String::from("()[]{}");
    let s3 = String::from("(]");
    
    println!("{} -> {}", s1, is_valid(s1.clone())); // true
    println!("{} -> {}", s2, is_valid(s2.clone())); // true
    println!("{} -> {}", s3, is_valid(s3.clone())); // false
}
