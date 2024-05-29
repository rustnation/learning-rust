//! src/threads/mutexes.rs

/*
A `Mutex` allows only one thread to access some data at any given time.
To access the data, a thread must first acquire the mutex's lock.
 */

use std::sync::{Arc, Mutex};
use std::thread;

pub fn master(show: bool) {
    if show {
        println!("--- Mutexes");

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
}
