//! src/external_crates/demo.rs
use humantime::format_duration;
use std::time::Duration;

pub fn master(show: bool) {
    if show {
        println!("\n--- External Crates Demo");

        let d = Duration::from_secs(9876);
        println!("Format Duration: {}", format_duration(d));
    }
}
