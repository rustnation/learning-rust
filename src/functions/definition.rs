//! src/functions/definition.rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn master(show: bool) {
    if show {
        println!("function definition");

        let x = add(1, 1);
        let y = add(2, 2);
        let z = add(3, 3);

        println!("x = {}, y = {}, z = {}", x, y, z);

        open_store(false, Some("Envigado"));
        bake_pizza(false);
        swim_in_profit(false);
    }
}

fn open_store(show: bool, neighborhood: Option<&str>) {
    if show {
        println!("Opening my pizza store {}", neighborhood.unwrap_or(""));
    }
}

fn bake_pizza(show: bool) {
    if show {
        println!("Baking a pizza");
    }
}

fn swim_in_profit(show: bool) {
    if show {
        println!("So much $$$, so little time");
    }
}
