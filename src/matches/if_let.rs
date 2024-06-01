pub fn master(show: bool) {
    if show {
        println!("\n-- if let else...");

        let maybe_user = Some("Bill");
        match maybe_user {
            Some(user) => println!("user: {:?}", user),
            None => println!("no user"),
        }

        // this is used when you don't care when there is no user
        if let Some(user) = maybe_user {
            println!("user: {:?}", user)
        }

        other_if_let();
    }
}

fn other_if_let() {
    let scores = vec![7, 77, 777, 7777];

    for index in 0..10 {
        if let Some(number) = scores.get(index) {
            println!("The number is: {number}")
        }
    }
}
