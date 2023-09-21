use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn master(show: bool) {
    if show {
        println!("-- Creating a New Thread with Spawn");
        definition();

        println!("-- Spawned Thread");
        spawned_thread();

        println!("-- Sharing Mutex");
        sharing_mutex();
    }
}

fn definition() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

fn spawned_thread() {
    let v = vec![1, 2, 3, 4, 5, 6, 7];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn sharing_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
