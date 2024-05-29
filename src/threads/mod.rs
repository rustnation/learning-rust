mod definition;
mod mutexes;
mod preventing_race_condition;

pub fn master(show: bool) {
    if show {
        println!("\n-- Threads");

        definition::master(false);

        preventing_race_condition::master(false);

        mutexes::master(false);
    }
}
