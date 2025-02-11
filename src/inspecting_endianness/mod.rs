//! src/inspecting_endianness/mod.rs
use crate::print_title;
use std::mem::transmute;

pub fn master(show: bool) {
    if show {
        print_title("Inspecting Endianness");

        let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let little_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];

        let a: i32 = unsafe { transmute(big_endian) };
        let b: i32 = unsafe { transmute(little_endian) };

        println!("{} vs  {}", a, b);
    }
}
