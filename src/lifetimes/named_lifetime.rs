#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    date_founded: u32,
}

pub fn master(show: bool) {
    if show {
        println!("Named Lifetime");

        let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];

        let my_city = City {
            name: &city_names[0],
            date_founded: 1902,
        };

        println!("{} was founded in {}", my_city.name, my_city.date_founded);
    }
}
