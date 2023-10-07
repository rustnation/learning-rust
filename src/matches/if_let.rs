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
    }
}
