pub fn index(show: bool) {
    if show {
        let (first, second) = make_tuple(String::from("Learning Rust"), 7);
        println!("first: {:?}, second: {:?}", first, second);

        let (team, score) = make_tuple("Bayern", 9.7);
        println!("team: {:?}, score: {:?}", team, score);

        let (price, alternative_price) = make_tuple::<f32, f64>(77.0, 77.7);
        println!(
            "price: {:?}, alternative_price: {:?}",
            price, alternative_price
        );
    }
}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
