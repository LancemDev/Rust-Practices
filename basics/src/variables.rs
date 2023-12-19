fn main() {
    // Removal of mut will result in an error in the 2nd access of x
    let mut x = 3;
    println!("x is: {}", x);
    x = 4;
    println!("New x should be: {}", x);
}
