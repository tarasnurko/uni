use std::io;

fn reverse_string(input: &str) -> String {
  if input.is_empty() {
      return String::new();
  }
  let last_char = input.chars().last().unwrap();
  let remaining_chars: String = input.chars().take(input.len() - 1).collect();
  let reversed_remaining_chars = reverse_string(&remaining_chars);
  format!("{}{}", last_char, reversed_remaining_chars)
}

fn main() {
  println!("Enter string: ");

  let mut input_string = String::new();

  match io::stdin().read_line(&mut input_string) {
    Ok(_) => {
      let reversed_string = reverse_string(input_string.as_str());
      println!("{}", reversed_string);
    }
    Err(_) => println!("Error while reading input")
  }
}