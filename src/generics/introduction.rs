pub fn index(show: bool) {
    if show {
        println!("{}", identity(7));
        println!("{}", identity(7.7));
        println!("{}", identity("seven"));
        println!("{}", identity(String::from("seven.seven")));
        println!("{}", identity(true));
    }
}

fn identity<T>(value: T) -> T {
    value
}
