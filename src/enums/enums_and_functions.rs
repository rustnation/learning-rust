#[derive(Debug)]
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature");
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {temperature}");
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with delicate cycle for {fabric_type}");
            }
        }
    }
}

pub fn index(show: bool) {
    if show {
        wash_laundry(LaundryCycle::Cold);
        wash_laundry(LaundryCycle::Hot { temperature: 70 });
        wash_laundry(LaundryCycle::Delicate(String::from("silk")));

        LaundryCycle::Cold.wash_laundry();
        let hot_cycle = LaundryCycle::Hot { temperature: 70 };
        hot_cycle.wash_laundry();

        let delicate_cycle = LaundryCycle::Delicate(String::from("silk"));
        delicate_cycle.wash_laundry();
    }
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Running the laundry with cold temperature");
        }
        LaundryCycle::Hot { temperature } => {
            println!("Running the laundry with a temperature of {temperature}");
        }
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running the laundry with delicate cycle for {fabric_type}");
        }
    }
}
