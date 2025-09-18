//! src/matches/definition.rs
pub fn index(show: bool) {
    if show {
        println!("\n-- Match Expression");

        definition();

        multiple_values();

        match_string();
    }
}

fn definition() {
    println!("--- Definition");
    let allowed = false;

    match allowed {
        true => println!("Allowed because is true"),
        false => println!("Denied because is false"),
    }
}

// match expression is checked by the compiler
// match considers all possibilities
fn multiple_values() {
    println!("--- Multiple Values");
    let score = 7;

    match score {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its something else"),
    }
}

fn match_string() {
    println!("--- Multiple Values in String");
    let name = "willtrd";

    match name {
        "alejo" => println!("Hello Alejo"),
        "tala" => println!("Hello Tala"),
        "maggy" => println!("Hello Maggy"),
        "leo" => println!("Hello Leo"),
        "willtrd" => println!("Hello Will The Rust Developer"),
        _ => println!("You are not member from this family"),
    }
}
