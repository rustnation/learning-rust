//! src/modules/mod.rs
mod inline_modules_activity;
mod inline_modules_demo;

pub fn master(show: bool) {
    if show {
        println!("\n-- Modules");

        inline_modules_demo::master(false);
        inline_modules_activity::master(true);
    }
}
