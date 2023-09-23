trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

struct Square {
    side: i32,
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    side_a: i32,
    side_b: i32,
    side_c: i32,
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.side_a + self.side_b + self.side_c
    }
}

fn print_perimeter(shape: impl Perimeter) {
    let perimeter = shape.calculate_perimeter();
    println!("Perimeter: {:?}", perimeter);
}

pub fn master(show: bool) {
    if show {
        println!("\n--- Traits Activity");

        let square = Square { side: 7 };
        let triangle = Triangle {
            side_a: 7,
            side_b: 7,
            side_c: 7,
        };

        print_perimeter(square);
        print_perimeter(triangle);
    }
}
