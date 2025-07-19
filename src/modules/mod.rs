//! src/modules/mod.rs
pub mod inline_modules_activity;
pub mod inline_modules_demo;
pub mod mod_basics;

pub fn master(show: bool) {
    if show {
        println!("\n-- Modules");

        inline_modules_demo::master(false);
        inline_modules_activity::master(false);
        mod_basics::master(false);
    }
}
