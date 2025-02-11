//! src/structs/struct_update_syntax.rs
use crate::structs::definition::User;

pub fn master(show: bool) {
    if show {
        let user1 = User {
            active: true,
            username: String::from("bellerophon"),
            email: String::from("bellerophon@rustdeveloper.io"),
            sign_in_count: 7,
        };

        println!("user1: {:?}", user1);

        let user2 = User {
            email: String::from("bellerophon@rustdeveloper.com"),
            ..user1
        };

        println!("user2: {:?}", user2);
    }
}
