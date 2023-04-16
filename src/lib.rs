pub mod generics;
pub mod closures;
pub mod tests;

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
