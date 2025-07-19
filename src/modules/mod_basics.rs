pub fn master(show: bool) {
    if show {
        println!("\n-- Module Basics");

        use print_things::*;

        prints_one_thing("Hello, world!".to_string());

        let my_billy = Billy::new(3);
        my_billy.print_billy();
    }
}

mod print_things {
    use std::fmt::Display;

    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{input}");
    }

    // Billy struct is public, but the parameter name inside it is private.
    // The user needs to use the public constructor to create an instance.
    #[derive(Debug)]
    pub struct Billy {
        name: String,
        pub times_to_print: u32,
    }

    impl Billy {
        pub fn new(times_to_print: u32) -> Self {
            Self {
                name: "Billy".to_string(),
                times_to_print,
            }
        }

        pub fn print_billy(&self) {
            for _ in 0..self.times_to_print {
                println!("{}", self.name);
            }
        }
    }
}
