//! src/print_var_type/mod.rs
pub mod definition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Print Var Type");

        definition::master(true);
    }
}
