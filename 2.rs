use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&mut self) -> i32 {
    self.count += 1;
    self.count
}

    fn get_count(&self) -> i32 {
        self.count
    }
}

fn main() {
    let counter = Arc::new(Mutex::new(Counter::new()));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut counter = counter_clone.lock().unwrap();
                counter.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.lock().unwrap().get_count());
}
