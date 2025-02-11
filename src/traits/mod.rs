//! src/traits/mod.rs
use crate::print_title;

mod activity;
mod activity_trait_objects;
mod complex_trait;
mod default_trait;
mod definition;
mod demo;
mod demo_trait_objets;
mod derives;
mod from_trait;
mod impl_display;
mod media_aggregator;
mod returning_traits_with_dyn;
mod simple_trait;
mod with_generics;

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

        // Demo Trait Objects
        demo_trait_objets::master(false);

        // Activity Trait Objects
        activity_trait_objects::master(false);

        // From Trait
        from_trait::master(false);

        // With Generics
        with_generics::master(false);

        // Simple Trait Example
        simple_trait::master(false);

        // Implement Display Trait
        impl_display::master(false);

        // Implement Complex Trait
        complex_trait::master(false);
    }
}
