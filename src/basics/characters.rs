pub fn master(show: bool) {
    if show {
        println!("--- Character Types ---");

        let c = 'z';
        println!("value of c is: {c}");

        let z: char = '⎈';
        println!("value of z is: {z}");

        let kannada: char = '\u{CA0}';
        println!("value of kannada is: {kannada}");

        let heart_eyed_cat = '😻';
        println!("value of heart eyed cat: {}", heart_eyed_cat);
    }
}
