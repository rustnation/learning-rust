mod activity;
mod definition;
mod demo;
mod derives;
mod media_aggregator;
mod returning_traits_with_dyn;

pub fn master(show: bool) {
    if show {
        println!("\n-- Traits");

        // Trait Demo
        demo::master(false);

        // Trait Activity
        activity::master(true);

        // Trait Definition
        definition::master(false);

        // Trait using Derive Attribute
        derives::master(false);

        // Returning Traits with dyn
        returning_traits_with_dyn::master(false);

        // Media Aggregator
        media_aggregator::master(false);
    }
}
