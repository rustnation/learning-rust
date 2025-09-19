#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    NonDiary { kind: String },
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
            Milk::NonDiary { kind } => {
                println!("You've got the {kind} milk!");
            }
        }
    }
}

pub fn index(show: bool) {
    if show {
        Milk::Whole.drink();
        Milk::Lowfat(7);
        Milk::Lowfat(77);
        Milk::NonDiary {
            kind: String::from("Ulta fat"),
        };

        let my_beverage = Milk::Lowfat(7);

        // in this example we are validating all the posibilities
        if let Milk::Lowfat(percent) = my_beverage {
            println!("Your beverage is {percent}% milk");
        }

        let beverage = Milk::NonDiary {
            kind: String::from("Oat"),
        };

        if let Milk::NonDiary { kind } = beverage {
            println!("Your beverage is {kind} milk");
        }
    }
}
