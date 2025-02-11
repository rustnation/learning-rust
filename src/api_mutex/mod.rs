//! src/api_mutex/mod.rs
mod definition;
mod mutexes;
mod thread_parking;

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
