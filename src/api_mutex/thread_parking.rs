use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn master(show: bool) {
    if show {
        println!("--- Thread parking");

        let queue = Mutex::new(VecDeque::new());

        thread::scope(|s| {
            // Consuming thread
            let t = s.spawn(|| loop {
                let item = queue.lock().unwrap().pop_front();

                if let Some(item) = item {
                    dbg!(item);
                } else {
                    thread::park();
                }
            });

            // Producing thread
            for i in 0.. {
                queue.lock().unwrap().push_back(i);
                t.thread().unpark();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
}
