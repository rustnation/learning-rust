enum Status {
    Queued,
    Running,
    Failed,
}

pub fn master(show: bool) {
    if show {
        println!("\n--- Enums Demo");

        print_status(Status::Queued);
        print_status(Status::Running);
        print_status(Status::Failed);
    }
}

fn print_status(status: Status) {
    match status {
        Status::Queued => println!("queued"),
        Status::Running => println!("running"),
        Status::Failed => println!("failed"),
    }
}
