fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        match token.as_str() {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            }
            _ => {
                let num = token.parse::<i32>().unwrap();
                stack.push(num);
            }
        }
    }

    stack.pop().unwrap()
}

fn main() {
    let tokens1 = vec!["2", "1", "+", "3", "*"].into_iter().map(String::from).collect();
    let tokens2 = vec!["4", "13", "5", "/", "+"].into_iter().map(String::from).collect();
    let tokens3 = vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"].into_iter().map(String::from).collect();

    println!("Result: {}", eval_rpn(tokens1)); // 9
    println!("Result: {}", eval_rpn(tokens2)); // 6
    println!("Result: {}", eval_rpn(tokens3)); // 22
}
