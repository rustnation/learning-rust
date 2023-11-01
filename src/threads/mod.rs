mod definition;
mod preventing_race_condition;
pub fn master(show: bool) {
    if show {
        println!("\n-- Threads");

        definition::master(true);

        preventing_race_condition::master(false);
    }
}
