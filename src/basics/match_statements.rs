pub fn master(show: bool) {
    if show {
        let evaluation = true;

        match evaluation {
            true => {
                println!("You have evaluated {}", evaluation);
            }
            false => {
                println!("You have not evaluated {}", evaluation);
            }
        }

        match_with_variables(false);
    }
}

fn match_with_variables(show: bool) {
    if show {
        let number = 7;

        match number {
            x if x % 2 == 0 => println!("{x} is an even number"),
            x if x % 2 != 0 => println!("{x} is an odd number"),
            _ => unreachable!(),
        }
    }
}