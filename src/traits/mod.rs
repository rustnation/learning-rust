//! src/traits/mod.rs
use crate::print_title;

pub mod activity;
pub mod activity_trait_objects;
pub mod complex_trait;
pub mod copy_trait;
pub mod default_trait;
pub mod definition;
pub mod demo;
pub mod demo_trait_objets;
pub mod derives;
pub mod from_trait;
pub mod impl_display;
pub mod media_aggregator;
pub mod returning_traits_with_dyn;
pub mod simple_trait;
pub mod with_generics;

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

        // Copy Trait
        copy_trait::master(false);
    }
}
