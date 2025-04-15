use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();

    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();

    for handle in handles {
        // Wait for the thread to finish and collect the result
        let elapsed_ms = handle.join().expect("Thread panicked");
        results.push(elapsed_ms);
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
