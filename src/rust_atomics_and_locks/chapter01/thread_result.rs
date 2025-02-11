//! src/rust_atomics_and_locks/chapter01/thread_result.rs
use std::thread;

pub fn master(show: bool) {
    if show {
        println!("Thread Result");

        let numbers = Vec::from_iter(0..=1000);

        let t = thread::spawn(move || {
            let len = numbers.len();
            let sum = numbers.iter().sum::<usize>();
            sum / len
        });

        let average = t.join().unwrap();

        println!("average: {average}");
    }
}
