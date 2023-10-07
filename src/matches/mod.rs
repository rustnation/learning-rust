mod definition;
mod if_let;

pub fn master(show: bool) {
    if show {
        println!("\n-- Match Expression");

        definition::master(false);

        if_let::master(true);
    }
}
