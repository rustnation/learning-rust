//! src/enums/options.rs
// The option type encodes the very common scenario in which a value could be something
// or it could be nothing.

pub fn index(show: bool) {
    if show {
        // represents a present value
        let some_number = Some(7);
        let some_char = Some('e');

        // represents an absent value
        // Rust here needs a concret type value
        let absent_number: Option<i32> = None;

        println!("The value of some_number is: {:?}", some_number);
        println!("The value of some_chart is: {:?}", some_char);
        println!("The value of absent_number is {:?}", absent_number);

        demo(false);
        demo_idiomatic(false);
    }
}

fn demo(show: bool) {
    if show {
        let musical_instruments = [
            String::from("Guitar"),
            String::from("Drums"),
            String::from("Bass"),
        ];

        let bass = musical_instruments.get(2);
        println!("{:?}", bass);

        // we can use unwrap because we know that bass variable has a value
        // but we cannot trust on this
        let valid_instrument = bass.unwrap();
        println!("valid instrument: {valid_instrument}");

        let invalid_instrument = musical_instruments.get(77);
        println!("{:?}", invalid_instrument);
        // if we use unwrap here, we are going to get a panic
        // invalid_instrument.unwrap(); // this will fail because there is not value

        // unwrap and expect are optimistic
    }
}

fn demo_idiomatic(show: bool) {
    if show {
        let musical_instruments = [
            String::from("Guitar"),
            String::from("Drums"),
            String::from("Bass"),
        ];

        let bass = musical_instruments.get(2);
        match bass {
            Option::Some(instrument) => {
                println!("instrument: {:?}", instrument);
            }
            Option::None => {
                println!("no instrument found");
            }
        }

        let invalid_instrument = musical_instruments.get(77);
        match invalid_instrument {
            Option::Some(instrument) => {
                println!("instrument: {:?}", instrument);
            }
            Option::None => {
                println!("no instrument found");
            }
        }
    }
}
