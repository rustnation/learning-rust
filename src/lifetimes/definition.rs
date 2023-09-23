pub fn master(show: bool) {
    if show {
        println!("-- Lifetimes Definition");

        let result = longest("one", "two");
        println!("The result is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
