#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito(Meat),
    Bowl(Meat),
    VeganPlate,
}

pub fn index(show: bool) {
    if show {
        let lunch = RestaurantItem::Burrito(Meat::Steak);
        let dinner = RestaurantItem::Bowl(Meat::Chicken);
        let abandoned_meal = RestaurantItem::VeganPlate;

        println!("lunch: {:?}", lunch);
        if let RestaurantItem::Burrito(meat) = lunch {
            println!("lunch add-ons: {:?}", meat);
        }

        println!("dinner: {:?}", dinner);
        if let RestaurantItem::Bowl(meat) = dinner {
            println!("dinner add-ons: {:?}", meat);
        }

        println!("abandoned meal: {:?}", abandoned_meal);
    }
}
