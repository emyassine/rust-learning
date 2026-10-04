#![no_implicit_prelude]

extern crate std;
extern crate rand;

use rand::prelude::*;

use std::cmp::Ordering;
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

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
