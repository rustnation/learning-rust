macro_rules! greetings {
    ($x: expr) => {
        println!("Hello, {}", $x);
    };
}

pub fn master(show: bool) {
    if show {
        println!("--- Macros");

        declarative_macros(false);
    }
}

fn declarative_macros(show: bool) {
    if show {
        println!("Declarative Macros");

        greetings!("Earthly"); // prints: "Hello, Earthly"
    }
}
