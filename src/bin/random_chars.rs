use rand::distributions::Alphanumeric;
use rand::prelude::*;
use std::iter;
use std::io::{stdin};

/// cargo run --bin random_chars
fn main() {
    println!("Specify length of password...");
    
    // Read integer from user input
    // https://stackoverflow.com/a/30355925/9823455
    let mut input_value = String::new();
    stdin().read_line(&mut input_value).expect("failed to read from stdin");
    let input_value = &input_value;

    // trim input
    // parse it
    let test = input_value.trim().parse::<u32>();
    
    // return if test is not an integer
    match test {
        Ok(ok) => println!("You've specified: {}\n", ok),
        Err(e) => return println!("Error: ({})", e), 
    };

    // unwrap the Result
    let test = test.unwrap();

    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(test as usize) // cast `u32` to `usize`
        .collect();
    println!("{}", chars)
}