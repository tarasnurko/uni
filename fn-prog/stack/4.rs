struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.stack_out.is_empty() {
            while let Some(val) = self.stack_in.pop() {
                self.stack_out.push(val);
            }
        }
        self.stack_out.pop()
    }

    fn peek(&mut self) -> Option<i32> {
        if self.stack_out.is_empty() {
            while let Some(val) = self.stack_in.pop() {
                self.stack_out.push(val);
            }
        }
        self.stack_out.last().cloned()
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

fn main() {
    let mut my_queue = MyQueue::new();
    my_queue.push(1); // [1]
    my_queue.push(2); // [1, 2]
    println!("peek: {:?}", my_queue.peek()); //1
    println!("pop: {:?}", my_queue.pop()); // 1, queue is [2]
    println!("empty: {:?}", my_queue.empty()); // false
}
