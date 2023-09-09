pub mod generics;
pub mod closures;
pub mod iterators;
pub mod doc;
pub mod tests;
pub mod boxes;
pub mod custom_smart_pointer;
pub mod rc_t;
pub mod messenger;
pub mod tree_data_structure;
pub mod threads;
pub mod joinhandle;
pub mod message_passing;
pub mod api_mutex;
pub mod object_oriented;
pub mod draw;
pub mod post;
pub mod patterns;
pub mod destructuring_structs;
pub mod unsafe_code;
pub mod extern_functions;
pub mod static_variable;
pub mod unsafe_trait;
pub mod operator_overloading;
pub mod add_millimeters_to_meters;
pub mod vectors;
pub mod futures;
pub mod get_type;
pub mod mutable_reference;
pub mod decisions;
pub mod match_expression;

pub fn print_title(title: &str) {
    println!(" ");
    println!("### {title} ###");
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!(
            "(Read more form {}...",
            self.summarize_author()
        )
    }
}
