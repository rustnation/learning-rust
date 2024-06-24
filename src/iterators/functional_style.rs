pub fn master(show: bool) {
    if show {
        println!("Functional style");

        let new_vec = (1..).take(10).collect::<Vec<i32>>();
        println!("{new_vec:?}");
    }
}
