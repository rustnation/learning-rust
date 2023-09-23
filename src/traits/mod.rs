use crate::print_title;

mod activity;
mod default_trait;
mod definition;
mod demo;
mod derives;
mod media_aggregator;
mod returning_traits_with_dyn;

pub fn master(show: bool) {
    if show {
        print_title("Traits");

        // Trait Demo
        demo::master(false);

        // Trait Activity
        activity::master(false);

        // Trait Definition
        definition::master(false);

        // Default Trait
        default_trait::master(false);

        // Trait using Derive Attribute
        derives::master(false);

        // Returning Traits with dyn
        returning_traits_with_dyn::master(false);

        // Media Aggregator
        media_aggregator::master(false);
    }
}
