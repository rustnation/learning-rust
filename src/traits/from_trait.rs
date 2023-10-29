use crate::print_title;

#[derive(Debug)]
struct Uppercase(String);

impl From<String> for Uppercase {
    fn from(data: String) -> Self {
        Uppercase(data.to_uppercase())
    }
}

impl From<&str> for Uppercase {
    fn from(data: &str) -> Self {
        Uppercase(data.to_uppercase())
    }
}

pub fn master(show: bool) {
    if show {
        print_title("From Trait");

        let upper = Uppercase::from("lowercase");
        println!("{:?}", upper);
        let upper: Uppercase = "lowercase".into();
        println!("{:?}", upper);
    }
}
