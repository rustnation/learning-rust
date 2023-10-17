use crate::print_title;
use std::collections::HashMap;

struct Contact {
    name: String,
    phone: String,
}

type ContactName = String;
type ContactIndex = HashMap<ContactName, Contact>;

pub fn master(show: bool) {
    if show {
        print_title("Type Aliases Demo");

        let contact_name: ContactName = String::from("Joe");

        let contact = Contact {
            name: contact_name,
            phone: String::from("3177777777"),
        };

        println!("contact name: {:?}", contact.name);
        println!("contact phone: {:?}", contact.phone);

        let mut contact_index: ContactIndex = HashMap::new();

        add_contact(&mut contact_index, contact);
    }
}

fn add_contact(index: &mut ContactIndex, contact: Contact) {
    index.insert(contact.name.to_owned(), contact);
}
