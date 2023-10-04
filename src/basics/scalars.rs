pub fn master(show: bool) {
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
