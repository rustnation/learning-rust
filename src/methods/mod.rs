//! src/methods/mod.rs
pub mod associated_functions;
pub mod definition;
pub mod multiple_impl_blocks;
pub mod parameters;

pub fn master(show: bool) {
    if show {
        // Methods Definition
        definition::master(false);

        // Methods with Parameters
        parameters::master(false);

        // Associated Functions
        associated_functions::master(false);

        // Multiple impl Blocks
        multiple_impl_blocks::master(false);
    }
}
