struct MyCircularQueue {
    data: Vec<i32>,
    head: i32,
    tail: i32,
    size: i32,
    capacity: i32,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            data: vec![-1; k as usize],
            head: -1,
            tail: -1,
            size: 0,
            capacity: k,
        }
    }

    fn enQueue(&mut self, value: i32) -> bool {
        if self.isFull() {
            return false;
        }
        if self.isEmpty() {
            self.head = 0;
        }
        self.tail = (self.tail + 1) % self.capacity;
        self.data[self.tail as usize] = value;
        self.size += 1;
        true
    }

    fn deQueue(&mut self) -> bool {
        if self.isEmpty() {
            return false;
        }
        if self.head == self.tail {
            self.head = -1;
            self.tail = -1;
        } else {
            self.head = (self.head + 1) % self.capacity;
        }
        self.size -= 1;
        true
    }

    fn Front(&self) -> i32 {
        if self.isEmpty() {
            return -1;
        }
        self.data[self.head as usize]
    }

    fn Rear(&self) -> i32 {
        if self.isEmpty() {
            return -1;
        }
        self.data[self.tail as usize]
    }

    fn isEmpty(&self) -> bool {
        self.size == 0
    }

    fn isFull(&self) -> bool {
        self.size == self.capacity
    }
}

fn main() {
    let mut my_circular_queue = MyCircularQueue::new(3);
    println!("{}", my_circular_queue.enQueue(1)); // true
    println!("{}", my_circular_queue.enQueue(2)); // true
    println!("{}", my_circular_queue.enQueue(3)); // true
    println!("{}", my_circular_queue.enQueue(4)); // false
    println!("{}", my_circular_queue.Rear()); // 3
    println!("{}", my_circular_queue.isFull()); // true
    println!("{}", my_circular_queue.deQueue()); // true
    println!("{}", my_circular_queue.enQueue(4)); // true
    println!("{}", my_circular_queue.Rear()); // 4
}
