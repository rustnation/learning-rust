//! src/numbers/complex_numbers.rs

use num::complex::Complex;

pub fn create_complex_number(show: bool) {
    if show {
        println!("--- Create Complex Number:");

        let a = Complex { re: 2.1, im: -1.2 };
        let b = Complex::new(11.1, 22.2);

        let result = a + b;

        println!("{} + {}i", result.re, result.im);
    }
}
