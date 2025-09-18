#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl(Meat),
    VeganPlate,
}

pub fn index(show: bool) {
    if show {
        let mut lunch = RestaurantItem::Burrito {
            meat: Meat::Steak,
            beans: Beans::Pinto,
        };
        let dinner = RestaurantItem::Bowl(Meat::Chicken);
        let abandoned_meal = RestaurantItem::VeganPlate;

        println!("lunch: {:?}", lunch);
        if let RestaurantItem::Burrito { meat, beans } = lunch {
            println!("lunch meat add-ons: {:?}", meat);
            println!("lunch beans add-ons: {:?}", beans);
        }

        lunch = RestaurantItem::Burrito {
            meat: Meat::Chicken,
            beans: Beans::Black,
        };

        println!("lunch variant: {:?}", lunch);
        if let RestaurantItem::Burrito { meat, beans } = lunch {
            println!("lunch varian meat add-ons: {:?}", meat);
            println!("lunch varian beans add-ons: {:?}", beans);
        }

        println!("dinner: {:?}", dinner);
        if let RestaurantItem::Bowl(meat) = dinner {
            println!("dinner add-ons: {:?}", meat);
        }

        println!("abandoned meal: {:?}", abandoned_meal);
    }
}
