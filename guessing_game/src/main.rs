use crate::prelude::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod error;
mod prelude;

// other ideas 
//      - return an error type when failing to parse to u32
//      - offer a hint if the user types "hint"
//      - publish to an app

fn main() {
    println!("Guess the number!");

    loop {

        let secret_number = rand::thread_rng().gen_range(1..=100);

        // println!("The secret number is: {secret_number}");

        loop {
            println!("Please guess a number between 1 and 100, type it here:");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = guess.trim().parse().or_else(
                || {
                    Error::Generic(f!("Invalid entry: [{guess}]"
                    ))
                });

            println!("You guessed: {guess}");

            // continue at https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }
        }
    }
}