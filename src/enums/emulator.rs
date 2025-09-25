#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh"),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

pub fn index(show: bool) {
    if show {
        let some_option = MyOption::Some(77);
        println!("some option: {:?}", some_option);

        let none_option = MyOption::None;
        println!("{}", none_option.unwrap_or(7));

        let none_option_panic = MyOption::None;
        println!("{}", none_option_panic.unwrap());
    }
}
