//! src/threads/mod.rs
pub mod definition;
pub mod message_passing_between_threads;
pub mod move_data_between_threads;
pub mod mutexes;
pub mod preventing_race_condition;
pub mod scoped_threads;
pub mod sending_multiple_values;
pub mod sharing_data_between_threads;
pub mod threads_in_array;
pub mod multithreading;

pub fn master(show: bool) {
    if show {
        println!("\n-- Threads");

        definition::master(false);

        preventing_race_condition::master(false);

        mutexes::master(false);

        sharing_data_between_threads::master(false);

        message_passing_between_threads::master(false);

        move_data_between_threads::master(false);

        threads_in_array::master(false);

        scoped_threads::master(false);

        sending_multiple_values::master(false);
        
        multithreading::index(false);
    }
}
