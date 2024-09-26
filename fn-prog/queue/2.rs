struct MyQueue {
    data: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue { data: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    fn front(&self) -> Option<&i32> {
        self.data.first()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

struct MyStack {
    queue1: MyQueue,
    queue2: MyQueue,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            queue1: MyQueue::new(),
            queue2: MyQueue::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue1.push(x);
    }

    fn pop(&mut self) -> i32 {
        while self.queue1.len() > 1 {
            self.queue2.push(self.queue1.pop().unwrap());
        }
        let top = self.queue1.pop().unwrap();
        std::mem::swap(&mut self.queue1, &mut self.queue2);
        top
    }

    fn top(&mut self) -> i32 {
        while self.queue1.len() > 1 {
            self.queue2.push(self.queue1.pop().unwrap());
        }
        let top = self.queue1.pop().unwrap();
        self.queue2.push(top);
        std::mem::swap(&mut self.queue1, &mut self.queue2);
        top
    }

    fn empty(&self) -> bool {
        self.queue1.is_empty() && self.queue2.is_empty()
    }
}

fn main() {
    let mut my_stack = MyStack::new();
    my_stack.push(1);
    my_stack.push(2);
    println!("{}", my_stack.top()); // 2
    println!("{}", my_stack.pop()); // 2
    println!("{}", my_stack.empty()); // false
}
