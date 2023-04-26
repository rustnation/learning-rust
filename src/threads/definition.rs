use std::thread;
use std::time::Duration;

pub fn master(show: bool) {
    if show {
        println!("-- Creating a New Thread with Spawn");
        definition();
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
