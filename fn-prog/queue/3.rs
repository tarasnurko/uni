struct RecentCounter {
    requests: Vec<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter { requests: Vec::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.requests.push(t);
        while self.requests[0] < t - 3000 {
            self.requests.remove(0);
        }
        self.requests.len() as i32
    }
}

fn main() {
    let mut recent_counter = RecentCounter::new();
    println!("{}", recent_counter.ping(1));    // 1
    println!("{}", recent_counter.ping(100));  // 2
    println!("{}", recent_counter.ping(3001)); // 3
    println!("{}", recent_counter.ping(3002)); // 3
}
