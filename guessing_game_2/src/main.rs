use std::io;
use rand::Rng;

fn main() {

    // Creates a mutable variable guess and makes it a new String instance
    let mut guess = String::new();
    // let x = 1;
    // let y = 2;
    println!("--Guessing game--");
    println!("Enter a random number");

    //Reads input and stores it in the mutable var guess
    io::stdin().read_line(&mut guess)
    .expect("Failed to do the thing");

    // I believe this a functions to the random crate for the version I'm using
    let secret = rand::thread_rng()
    // For a different random crate version there is a possibility that the syntax will change in the arguments
    .gen_range(1, 101);

    println!("The secret number is: {}", secret);
    // println!("x = {}, y = {}", y, x); // Notice sequence of variables
}
