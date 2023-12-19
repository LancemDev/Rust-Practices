fn main() {
    // the variable to be shadowed can't be  mutable
    let x = 2;
    // the value of x keeps on being shadowed hence the term shadowing
    let x = x * 3;
    let x = x + 100;
    println!("The value of x is: {}", x);

    let spacing = "     ";
    println!("{}",spacing.len())
}
