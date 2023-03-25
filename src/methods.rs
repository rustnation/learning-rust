pub mod definition;
pub mod parameters;
pub mod associated_functions;
pub mod multiple_impl_blocks;

pub fn master() {
    // Methods Definition
    definition::master();

    // Methods with Parameters
    parameters::master();

    // Associated Functions
    associated_functions::master();

    // Multiple impl Blocks
    multiple_impl_blocks::master();
}
