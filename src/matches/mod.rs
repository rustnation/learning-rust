//! src/matches/mod.rs
pub mod catch_all;
pub mod definition;
pub mod if_let;
pub mod match_control_flow;
pub mod match_guard;
pub mod match_vector;
pub mod multiple_placeholders;
pub mod pattern_match;
pub mod placeholder;
pub mod while_let;

pub fn master(show: bool) {
    if show {
        println!("\n-- Match Expression");

        definition::index(false);

        if_let::master(false);

        match_guard::master(false);

        catch_all::master(false);

        placeholder::master(false);

        multiple_placeholders::master(false);

        match_vector::master(false);

        while_let::master(false);

        match_control_flow::master(false);

        pattern_match::index(false);
    }
}
