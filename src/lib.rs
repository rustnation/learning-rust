pub fn print_title(title: &str) {
    println!(" ");
    println!("### {title} ###");
}

pub trait Summary {
    fn summarize(&self) -> String;
}
