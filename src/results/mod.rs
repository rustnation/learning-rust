//! src/results/mod.rs
mod activity;
mod definition;
mod question_mark_operator;

pub fn master(show: bool) {
    if show {
        println!("\n-- Results");

        definition::master(false);

        activity::master(false);

        question_mark_operator::master(true);
    }
}
