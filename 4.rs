use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let data: Vec<i32> = (1..=1_000_000_000).collect(); 
    let mut handles = vec![];

    for _ in 0..100 { 
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            for i in 0..data.len() {
                data[i] *= 2; 
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); 
    }

    println!("Первый элемент: {}", data.lock().unwrap()[0]);
}
