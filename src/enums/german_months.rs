#[derive(Debug)]
enum GermanMonths {
    Januar,
    Februar,
    Marz,
    April,
    Mai,
    Juni,
    Juli,
    August,
    September,
    Oktober,
    November,
    Dezember,
}

pub fn index(show: bool) {
    if show {
        let months = [
            GermanMonths::Januar,
            GermanMonths::Februar,
            GermanMonths::Marz,
            GermanMonths::April,
            GermanMonths::Mai,
            GermanMonths::Juni,
            GermanMonths::Juli,
            GermanMonths::August,
            GermanMonths::September,
            GermanMonths::Oktober,
            GermanMonths::November,
            GermanMonths::Dezember,
        ];

        for month in months.iter() {
            match month {
                GermanMonths::Januar => println!("Januar"),
                GermanMonths::Februar => println!("Februar"),
                GermanMonths::Marz => println!("März"),
                GermanMonths::April => println!("April"),
                GermanMonths::Mai => println!("Mai"),
                GermanMonths::Juni => println!("Juni"),
                GermanMonths::Juli => println!("Juli"),
                GermanMonths::August => println!("August"),
                GermanMonths::September => println!("September"),
                GermanMonths::Oktober => println!("Oktober"),
                GermanMonths::November => println!("November"),
                GermanMonths::Dezember => println!("Dezember"),
            }
        }
    }
}
