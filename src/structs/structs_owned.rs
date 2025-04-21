//! src/structs/structs_owned.rs
struct LineItem {
    name: String,
    count: i32,
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Structs Owned");

        let receipt = vec![
            LineItem {
                name: "cereal".to_owned(),
                count: 1,
            },
            LineItem {
                name: String::from("fruit"),
                count: 3,
            },
        ];

        for item in receipt {
            println!("Item: {:?}", item.name);
            println!("Quantity: {:?}", item.count);
        }
    }
}
