//! src/structs/tuple_structs.rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct ColorRgb(u8, u8, u8);

#[derive(Debug)]
struct SizeAndColor {
    size: u32,
    color: ColorRgb,
}

// Tuple struct with named fields
// Hours, Minutes
struct ShortDuration(u32, u32);

// Tuple struct with named fields
// Years, Months
struct LongDuration(u32, u32);

pub fn index(show: bool) {
    if show {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        println!("Color: {:?} {:?} {:?}", black.0, black.1, black.2);
        println!("Origin: {:?} {:?} {:?}", origin.0, origin.1, origin.2);

        // Using ColorRgb and SizeAndColor
        let my_color = ColorRgb(50, 0, 50);

        let size_and_color = SizeAndColor {
            size: 150,
            color: my_color,
        };

        println!("size_and_color: {:?}", size_and_color);
        println!("size_and_color size: {:?}", size_and_color.size);
        println!("size_and_color color: {:?}", size_and_color.color);
        println!("size_and_color color 1: {:?}", size_and_color.color.0);
        println!("size_and_color color 2: {:?}", size_and_color.color.1);
        println!("size_and_color color 3: {:?}", size_and_color.color.2);

        tuple_structs(false);
    }
}

// working with tuple structs
pub fn tuple_structs(show: bool) {
    if show {
        let short = ShortDuration(5, 30);
        let long = LongDuration(2, 6);

        println!("Short Duration: {:?} {:?}", short.0, short.1);
        println!("Long Duration: {:?} {:?}", long.0, long.1);
    }
}
