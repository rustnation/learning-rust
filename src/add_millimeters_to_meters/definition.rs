use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub fn master(show: bool) {
    if show {
        println!("-- Implementing the Add trait on Millimeters to add Millimeters and Meters");
    }
}
