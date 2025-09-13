pub fn index(show: bool) {
    if show {
        println!("{:?}", area(Shape::Circle(32.0)));
        println!("{:?}", area(Shape::Rectangle(32.0, 32.0)));
        println!("{:?}", area(Shape::Triangle(32.0, 32.0, 32.0)));
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}
