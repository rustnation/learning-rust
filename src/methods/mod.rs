mod associated_functions;
mod definition;
mod multiple_impl_blocks;
mod parameters;

pub fn master(show: bool) {
    if show {
        // Methods Definition
        definition::master();

        // Methods with Parameters
        parameters::master();

        // Associated Functions
        associated_functions::master();

        // Multiple impl Blocks
        multiple_impl_blocks::master();
    }
}
