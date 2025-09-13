use std::thread;

pub fn index(show: bool) {
    if show {
        let data = vec![1, 2, 3, 4, 5, 6, 7];
        let mut handles = vec![];

        for &item in &data {
            handles.push(thread::spawn(move || {
                println!("Processed: {}", item * 2);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
