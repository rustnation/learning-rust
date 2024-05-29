mod definition;
mod preventing_race_condition;
mod mutexes;

pub fn master(show: bool) {
    if show {
        println!("\n-- Threads");

        definition::master(false);

        preventing_race_condition::master(false);

        mutexes::master(false);
    }
}
