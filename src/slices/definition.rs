pub fn master(show: bool) {
    if show {
        let action_hero = String::from("Arnold Schwarzenegger");
        let first_name = &action_hero[0..6];
        println!("First Name: {first_name}");

        let last_name = &action_hero[7..21];
        println!("Last Name: {last_name}");
    }
}
