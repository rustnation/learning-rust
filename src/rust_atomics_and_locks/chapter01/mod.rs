mod thread_example;

pub fn master(show: bool) {
    if show {
        println!("---Chapter 01");

        thread_example::master(true);
    }
}
