enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

pub fn master(show: bool) {
    if show {
        println!("--- Enum and Vectors");

        let tickets = vec![
            Ticket::Backstage(50.0, "Billy".to_owned()),
            Ticket::Standard(15.0),
            Ticket::Vip(30.0, "Amy".to_owned()),
        ];

        for ticket in tickets {
            match ticket {
                Ticket::Backstage(price, holder) => {
                    println!("Backstage Ticket Holder: {:?}, Price: {:?}", holder, price)
                }
                Ticket::Standard(price) => println!("Standard Ticket Price: {:?}", price),
                Ticket::Vip(price, holder) => {
                    println!("VIP Ticket Holder: {:?}, Price: {:?}", holder, price)
                }
            }
        }
    }
}
