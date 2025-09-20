pub fn index(show: bool) {
    if show {
        let mut availability = is_item_in_stock(true, true);
        show_result(availability);

        availability = is_item_in_stock(true, false);
        show_result(availability);

        availability = is_item_in_stock(false, false);
        show_result(availability);
    }
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_stock {
        Option::Some(false)
    } else {
        Option::None
    }
}

fn show_result(availability: Option<bool>) {
    match availability {
        Option::Some(value) => {
            if value {
                println!("item is in system and item is in stock");
            } else {
                println!("item is in system but is not in stock");
            }
        }
        Option::None => println!("Item does not exist in our system"),
    }
}
