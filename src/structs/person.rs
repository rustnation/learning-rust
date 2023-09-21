struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{:?}", data);
}

pub fn master(show: bool) {
    if show {
        println!("\n--- Person Struct");

        let people = vec![
            Person {
                name: String::from("Magally"),
                fav_color: String::from("red"),
                age: 45,
            },
            Person {
                name: String::from("Alejo"),
                fav_color: String::from("blue"),
                age: 14,
            },
            Person {
                name: String::from("Tala"),
                fav_color: String::from("pink"),
                age: 10,
            },
        ];

        for person in people {
            if person.age <= 14 {
                print(&person.name);
                print(&person.fav_color);
            }
        }
    }
}
