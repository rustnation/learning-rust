#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(7) => {
                println!("Delicious, 7% milk is my favorite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {percent} percent version!");
            }
            Milk::Whole => {
                println!("You've got the whole milk!");
            }
        }
    }
}

pub fn index(show: bool) {
    if show {
        Milk::Whole.drink();
        Milk::Lowfat(7);
        Milk::Lowfat(77);
    }
}
