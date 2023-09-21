pub mod departments;
pub mod pig_latin;
pub mod vector;

pub fn master(show: bool) {
    if show {
        println!("\n--- Chapter 08");

        vector::master();

        pig_latin::master();

        departments::master();
    }
}
