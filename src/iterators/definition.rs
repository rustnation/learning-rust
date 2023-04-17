pub fn master(show: bool) {
    if show {
        println!("--- ITERATORS ---");
        creating_an_iterator();
    }
}

fn creating_an_iterator() {
    println!("\n-- Creating and Iterator");
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}
