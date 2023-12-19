use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Creates a mutable variable guess and makes it a new String instance
    let mut guess = String::new();
    // let x = 1;
    // let y = 2;
        println!("--Guessing game--");
    loop{
        guess.clear();

        println!("Enter a random number");

        //Reads input and stores it in the mutable var guess
        io::stdin().read_line(&mut guess)
        .expect("Failed to do the thing");

        let guess: u32 = guess.trim().parse()
        .expect("Please type in a number");

        // I believe this a functions to the random crate for the version I'm using
        let secret = rand::thread_rng()
        // For a different rand crate version there is a possibility that the syntax will change in the arguments. For this I'm using "0.8.12"
        .gen_range(1, 101);

        println!("The secret number is: {}", secret);
        // println!("x = {}, y = {}", y, x); // Notice sequence of variables

        match guess.cmp(&secret){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed it right man");
                break;
            }
        }
    }
}
