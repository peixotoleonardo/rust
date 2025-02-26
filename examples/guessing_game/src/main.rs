// import library `io` to handle with I/O
use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::rng().random_range(1..=100);

    /*
     * create a infinite loop
     */
    loop {
        /*
         * let` is used to create a variable.
         * By default, variables are immutable.
         * We can use `mut`to allow it.
         *
         * The syntax :: in the ::new line indicates
         * that new i an associated function on the
         * String.
         */
        let mut guess = String::new();

        io::stdin()
            /*
             * & indicates that guess is a reference.
             * References are immutable by default, so
             * we need use `mut` to indicate this reference
             * is mutable.
             */
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("there was a error: {err}");
                continue;
            }
        };

        println!("You guess {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
