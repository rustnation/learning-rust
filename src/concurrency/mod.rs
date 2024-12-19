mod definition;
mod message_passing;
mod sharing_state_concurrently;

pub fn master(show: bool) {
    if show {
        println!("--- Concurrency");
        definition::master(false);
        message_passing::master(false);
        sharing_state_concurrently::master(false);
    }
}
