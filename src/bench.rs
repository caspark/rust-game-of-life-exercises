//! Provides rudimentary benchmarking support.
//!
//! Note: Rust actually has benchmarking built in to its test framework, but it's still behind a
//! feature flag, so you can't use it unless you swap to nightly Rust; for more info see
//! https://doc.rust-lang.org/stable/unstable-book/library-features/test.html

use game_of_life::GameOfLife;

pub fn run_bench(mut game: Box<dyn GameOfLife>, num_iterations: u32) {
    println!("Starting benchmark with {} iterations", num_iterations);
    for _ in 0..num_iterations {
        game.tick();
    }
    println!("Finished benchmark with {} iterations", num_iterations);
}
