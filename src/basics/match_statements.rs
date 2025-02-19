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
    }
}