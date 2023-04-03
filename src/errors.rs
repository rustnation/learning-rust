pub mod closures;

pub fn master(show: bool) {
    if show {
        println!("--- Errors using closures ---");
        closures::master();
    }
}
