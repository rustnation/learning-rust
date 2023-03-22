pub mod basics;
use self::basics::variables;
use self::basics::constants;
use self::basics::shadowing;
use self::basics::scalar_types;
use self::basics::floating_point;
use self::basics::numeric_operations;
use self::basics::boolean_type;
use self::basics::character_type;

fn main() {
    // Variables
    variables::variables();

    // Constants
    constants::constants();

    // Shadowing
    shadowing::shadowing();

    // Scalar Types
    scalar_types::scalar_types();

    // Floating Points
    floating_point::floating_points();

    // Numeric Operations
    numeric_operations::numeric_operations();

    // Boolean Types
    boolean_type::boolean_type();

    // Character Types
    character_type::character_type();
}
