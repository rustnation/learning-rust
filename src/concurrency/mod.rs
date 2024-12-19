mod definition;
mod message_passing;

pub fn master(show: bool) {
    if show {
        println!("--- Concurrency");
        definition::master(false);
        message_passing::master(true);
    }
}
