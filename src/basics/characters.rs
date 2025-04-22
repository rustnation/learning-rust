//! src/basics/characters.rs
pub fn index(show: bool) {
    if show {
        println!("--- Character Types ---");

        let c = 'z';
        println!("value of c is: {c}");

        let z: char = '𝒵';
        println!("value of z is: {z}");

        let kannada: char = '\u{CA0}';
        println!("value of kannada is: {kannada}");

        let heart_eyed_cat = '😻';
        println!("value of heart eyed cat: {}", heart_eyed_cat);

        character_example(false);
    }
}

fn character_example(show: bool) {
    if show {
        let first_initial = 'L';
        let emoji = '🐶';

        println!(
            "first_initial: {}\n emoji: {}",
            first_initial.is_alphabetic(),
            emoji.is_alphabetic()
        );
    }
}
