//! src/data_collections/mod.rs
pub mod definition;
pub mod shipping_box;

pub fn master(show: bool) {
    if show {
        println!("\n-- Data Collections");

        definition::master(false);

        shipping_box::master(false);
    }
}
