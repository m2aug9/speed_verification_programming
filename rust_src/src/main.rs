use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let mut number: i32 = 0;
    let mut pi: f64 = 0.0;
    while number <= 100_000_000 {
        pi = pi + ((-1f64).powi(number) / (2 * number + 1) as f64);
        number = number + 1;
    }
    println!("pi = {}", pi * 4f64);
    let end = start.elapsed();
    println!("{}.{:03}sec", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
