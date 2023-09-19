mod definition;
mod parameters;
mod associated_functions;
mod multiple_impl_blocks;

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
