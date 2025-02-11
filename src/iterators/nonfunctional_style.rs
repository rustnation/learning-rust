//! src/iterators/nonfunctional_style.rs
pub fn master(show: bool) {
    if show {
        println!("Non-functional style");

        let mut new_vec = Vec::new();
        let mut counter = 1;

        loop {
            new_vec.push(counter);
            counter += 1;
            if counter == 10 {
                break;
            }
        }

        println!("{new_vec:?}");
    }
}
