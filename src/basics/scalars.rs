//! src/basics/scalars.rs
pub fn index(show: bool) {
    if show {
        println!("--- Scalar Types ---");
        println!("--- Integer Types ---");
        let vi8: i8 = -128;
        println!("vi8 is of type i8 and its value is: {vi8}");
        let vu8: u8 = 255;
        println!("vu8 is of type u8 and its value is: {vu8}");
        println!("Possible Types:");
        println!("8-bit i8 ui");
        println!("16-bit i16 u16");
        println!("32-bit i32 u32");
        println!("64-bit i64 u64");
        println!("128-bit i128 u128");
        println!("arch isize usize (depends on the architecture)");
    }
}

// Type     Smallest Value                                              Largest Value
// i8       -128                                                        127
// u8       0                                                           255
// i16      -32,768                                                     32,767
// u16      0                                                           65,535
// i32      -2,147,483,648                                              2,147,483,647
// u32      0                                                           4,294,967,295
// i64      -9,223,372,036,854,775,808                                  9,223,372,036,854,775,808
// u64      0                                                           18,446,744,073,709,551,616
// i128     -170,141,183,460,469,231,731,687,303,715,884,105,728        170,141,183,460,469,231,731,687,303,715,884,105,727
// u128     0                                                           340,282,366,920,938,463,463,374,607,431,768,211,455
