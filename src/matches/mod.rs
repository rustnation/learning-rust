mod definition;
mod if_let;
mod match_guard;

pub fn master(show: bool) {
    if show {
        println!("\n-- Match Expression");

        definition::master(false);

        if_let::master(false);

        match_guard::master(false);
    }
}
