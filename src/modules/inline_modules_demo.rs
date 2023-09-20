pub fn master(show: bool) {
    if show {
        println!("\n--- Inline Modules Demo");

        greet::hallo();
        greet::goodbye();

        let result1 = math::add(6, 1);
        println!("result: {}", result1);

        let result2 = math::sub(14, 7);
        println!("result: {}", result2);
    }
}

mod greet {
    pub fn hallo() {
        println!("hallo");
    }

    pub fn goodbye() {
        println!("tschüss");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}
