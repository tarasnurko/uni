fn decode_string(s: &str) -> String {
    let mut stack: Vec<(String, usize)> = Vec::new();
    let mut current_string = String::new();
    let mut current_num = 0;

    for c in s.chars() {
        if c.is_digit(10) {
            current_num = current_num * 10 + c.to_digit(10).unwrap() as usize;
        } else if c == '[' {
            stack.push((current_string.clone(), current_num));
            current_string = String::new();
            current_num = 0;
        } else if c == ']' {
            if let Some((prev_string, num)) = stack.pop() {
                current_string = prev_string + &current_string.repeat(num);
            }
        } else {
            current_string.push(c);
        }
    }

    current_string
}

fn main() {
    let s1 = "3[a]2[bc]";
    let s2 = "3[a2[c]]";
    let s3 = "2[abc]3[cd]ef";

    println!("Decoded string: {}", decode_string(s1)); // aaabcbc
    println!("Decoded string: {}", decode_string(s2)); // accaccacc
    println!("Decoded string: {}", decode_string(s3)); // abcabccdcdcdef
}
