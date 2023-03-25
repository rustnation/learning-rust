struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn master() {
    let mut user1 = User {
        active: true,
        username: String::from("bellerophon"),
        email: String::from("bellerophon@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("bellerophon@rustdeveloper.net");
    println!("{}", user1.username);
    println!("{} {}", user1.active, user1.sign_in_count);

    let mut user2 = build_user(String::from("chimera@example.com"), String::from("chimera"));

    user2.email = String::from("chimera@rustdeveloer.net");
    println!("{}", user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
