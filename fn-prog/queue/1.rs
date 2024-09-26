struct Queue {
    data: Vec<(char, usize)>,
}

impl Queue {
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn push(&mut self, value: (char, usize)) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<(char, usize)> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    fn front(&self) -> Option<&(char, usize)> {
        self.data.first()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn first_uniq_char(s: String) -> i32 {
    let mut char_count = [0; 256];
    let mut queue = Queue::new();

    for (i, c) in s.chars().enumerate() {
        let index = c as usize;
        char_count[index] += 1;
        if char_count[index] == 1 {
            queue.push((c, i));
        }
    }

    while let Some(&(c, i)) = queue.front() {
        if char_count[c as usize] == 1 {
            return i as i32;
        } else {
            queue.pop();
        }
    }

    -1
}

fn main() {
    let s1 = "leopard".to_string();
    let s2 = "loveleopard".to_string();
    let s3 = "aabb".to_string();

    println!("First unique character index: {}", first_uniq_char(s1)); // 0
    println!("First unique character index: {}", first_uniq_char(s2)); // 2
    println!("First unique character index: {}", first_uniq_char(s3)); // -1
}
