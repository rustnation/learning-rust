pub fn master(show: bool) {
    if show {
        println!("-- Variables --");

        let apples = 50;
        println!("Number of apples {}", apples);

        let oranges = 77;
        println!("Number of oranges {}", oranges);

        let fruits = apples + oranges;
        println!("Number of fruits {}", fruits);
    }
}