//! src/webserver/mod.rs
use crate::print_title;

mod web;

pub fn master(show: bool) {
    if show {
        print_title("Webserver");

        web::master(true);
    }
}
