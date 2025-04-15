#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // Use Arc to make the vector thread-safe and shareable
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // Clone Arc for each thread
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            // Sum every 8th number starting from offset
            let sum: u32 = child_numbers
                .iter()
                .enumerate()
                .filter(|(i, _)| i % 8 == offset)
                .map(|(_, &n)| n)
                .sum();

            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles {
        handle.join().unwrap();
    }
}
