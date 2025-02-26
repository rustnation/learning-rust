//! src/compiler_directives/mod.rs
pub mod demo;

pub fn master(show: bool) {
    if show {
        println!("-- Compiler Directives --");

        #[allow(unused_variables)]
        let message = "unused variable";

        #[allow(unused_variables)]
        let expected_message = "other unused variable";

        show_message(false);

        demo::master(false);
    }
}

// allow unused variables but in a function
#[allow(unused_variables)]
fn show_message(show: bool) {
    if show {
        let message = "show message";
        let expected_message = "expected message";
    }
}
