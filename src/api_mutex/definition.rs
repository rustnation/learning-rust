use std::sync::Mutex;

pub fn master(show: bool) {
    if show {
        println!("-- The API of Mutex<T>");
        definition();
    }
}

fn definition() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
