use std::sync::mpsc;
use std::thread;

pub fn master(show: bool) {
    if show {
        println!("---Message Passing Between Threads");

        let (sender, receiver) = mpsc::channel();

        let _ = thread::spawn(move || match receiver.recv() {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        });

        let data = 42;
        match sender.send(data) {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        };
    }
}
