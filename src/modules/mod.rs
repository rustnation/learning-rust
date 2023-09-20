mod inline_modules_demo;
mod inline_modules_activity;

pub fn master(show: bool) {
    if show {
        println!("\n-- Modules");

        inline_modules_demo::master(false);
        inline_modules_activity::master(true);
    }
}
