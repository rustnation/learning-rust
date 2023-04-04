pub mod closures;
pub mod error_propagation;

pub fn master(show: bool) {
    if show {
        println!("--- Errors using closures ---");
        closures::master();

        println!("--- Return Errors ---");
        error_propagation::master();
    }
}
