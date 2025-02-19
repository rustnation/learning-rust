pub fn master(show: bool) {
    if show {
        countdown(7);
    }
}

fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff!");
    } else {
        println!("{} seconds to blastoff...", seconds);
        countdown(seconds - 1);
    }
}
