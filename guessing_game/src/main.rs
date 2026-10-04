#![no_implicit_prelude]

extern crate std;
extern crate rand;

use rand::prelude::*;
use std::io;
use std::println;
use std::string::String;

fn main() {

    println!("Guess the number!");

    /* Added this */
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");

    // NO CHANGES HERE YET

    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if guess == secret_number {
    println!("You guessed: {guess}");
    }
}
