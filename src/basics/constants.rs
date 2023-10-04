use crate::print_title;

const MAX_SPEED: i32 = 9000;

pub fn master(show: bool) {
    if show {
        print_title("Constants");

        definition();

        let speed = clamp_speed(9001);
        println!("Speed: {}", speed);
    }
}

fn definition() {
    print_title("Constants Definition");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of const THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
}

fn clamp_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED
    } else {
        speed
    }
}
