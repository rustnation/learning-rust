//! src/api_mutex/mod.rs
pub mod definition;
pub mod mutexes;
pub mod thread_parking;

pub fn master(show: bool) {
    if show {
        // Array Definition
        definition::master(false);

        // Mutex in Practice
        mutexes::master(false);

        // Thread Parking
        thread_parking::master(false);
    }
}
