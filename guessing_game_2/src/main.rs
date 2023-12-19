use std::io;

fn main() {
    let mut guess = String::new();
    println!("--Guessing game--");
    println!("Enter a random number");

    io::stdin().read_line(&mut guess)
    .expect("Failed to do the thing");

    println!("We guessed correctly: {}", guess);
}
