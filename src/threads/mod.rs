mod definition;
mod message_passing_between_threads;
mod move_data_between_threads;
mod mutexes;
mod preventing_race_condition;
mod scoped_threads;
mod sharing_data_between_threads;
mod threads_in_array;

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
    }
}
