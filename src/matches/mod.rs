mod catch_all;
mod definition;
mod if_let;
mod match_guard;
mod multiple_placeholders;
mod placeholder;

pub fn master(show: bool) {
    if show {
        println!("\n-- Match Expression");

        definition::master(false);

        if_let::master(false);

        match_guard::master(false);

        catch_all::master(false);

        placeholder::master(false);

        multiple_placeholders::master(false);
    }
}
