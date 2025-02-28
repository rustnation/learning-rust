#[derive(Debug)]
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

pub fn master(show: bool) {
    if show {
        let mocha = Coffee {
            price: 7.7,
            name: String::from("Mocha"),
            is_hot: true,
        };

        println!("Mocha: {mocha:?}");
        println!(
            "My {} this morning cost {}. It is {} that is was hot.",
            mocha.name, mocha.price, mocha.is_hot
        );
    }
}
