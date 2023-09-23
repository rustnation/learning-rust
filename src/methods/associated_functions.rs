#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn master(show: bool) {
    if show {
        let sq = Rectangle::square(7);

        println!("The width of sq is: {}", sq.width);
        println!("The height of sq is: {}", sq.height);
    }
}
