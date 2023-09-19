pub mod vector;
pub mod pig_latin;
pub mod departments;

pub fn master(show: bool) {
    if show {
        println!("\n--- Chapter 08");

        vector::master();

        pig_latin::master();

        departments::master();
    }
}
