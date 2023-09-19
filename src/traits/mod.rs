mod definition;
mod derives;
mod returning_traits_with_dyn;
mod media_aggregator;

pub fn master(show: bool) {
    if show {
        // Trait Definition
        definition::master();

        // Trait using Derive Attribute
        derives::master();

        // Returning Traits with dyn
        returning_traits_with_dyn::master();

        // Media Aggregator
        media_aggregator::master();
    }
}
