//! src/messenger/mod.rs
pub mod definition;
pub mod mock_messenger;

pub fn master(show: bool) {
    if show {
        println!("\n-- Messenger");

        definition::master(true);
    }
}
