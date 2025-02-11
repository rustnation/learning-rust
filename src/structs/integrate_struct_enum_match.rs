//! src/structs/integrate_struct_enum_match.rs
enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("flavor: sparkling"),
        Flavor::Sweet => println!("flavor: sweet"),
        Flavor::Fruity => println!("flavor: fruity"),
    }
    println!("oz: {}", drink.fluid_oz);
}

pub fn master(show: bool) {
    if show {
        println!("\n-- Integrate Struct, Enum & Match");

        // Sweet Drink
        let sweet = Drink {
            flavor: Flavor::Sweet,
            fluid_oz: 7.0,
        };
        print_drink(sweet);

        // Fruity Drink
        let fruity = Drink {
            flavor: Flavor::Fruity,
            fluid_oz: 17.0,
        };
        print_drink(fruity);

        // Sparkling Drink
        let sparkling = Drink {
            flavor: Flavor::Sparkling,
            fluid_oz: 27.0,
        };
        print_drink(sparkling);
    }
}
