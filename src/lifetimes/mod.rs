//! src/lifetimes/mod.rs
use crate::print_title;

pub mod activity;
pub mod anonymous_lifetime;
pub mod compare_activity;
pub mod definition;
pub mod demo;
pub mod example;
pub mod named_lifetime;

pub fn master(show: bool) {
    if show {
        print_title("Lifetimes");

        definition::master(false);

        example::master(false);

        demo::master(false);

        activity::master(false);

        compare_activity::master(false);

        named_lifetime::master(false);

        anonymous_lifetime::master(false);
    }
}
