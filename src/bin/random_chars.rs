extern crate rand;

use rand::distributions::Alphanumeric;
use rand::prelude::*;
use std::iter;

/// cargo run --bin random_chars
fn main() {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(7)
        .collect();
    println!("Random chars: {}", chars)
}