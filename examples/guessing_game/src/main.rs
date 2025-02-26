// import library `io` to handle with I/O
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
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
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess: {}", guess);
}
