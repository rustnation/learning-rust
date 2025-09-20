pub fn index(show: bool) {
    if show {
        // using TurboFishOperator
        println!("{}", identity::<i8>(7));
        println!("{}", identity::<u32>(7));
        println!("{}", identity::<f64>(7.7));

        // without TurboFishOperator
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
