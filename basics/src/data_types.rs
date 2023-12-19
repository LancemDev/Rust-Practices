fn main() {
    // let _guess: u32 = "42".parse().expect("Something's wrong!");

    /*
    Scalar Types: rust has primari; 4 scalar types. Integers, floating-point numbers, Boolean and characters 
    */

    // INTEGERS
    // u means unisigned and i means signed
    let x: u32 = 200;
    println!("Unsigned integer: {}", x);

    let y: i32 = -200;
    println!("Signed integer: {}", y);

    let z: i64 = -20000000000000000;
    println!("Larger signed integer: {}", z);

    // FLOATING-POINT NUMBERS
    let float1: f32 = 3.011;
    println!("Floating point number: {}", float1);

    // BOOLEAN 
    let f: bool = false;
    println!("Boolean:  {}", f);

    // CHARACTERS
    // not the use of single quotes
    let char1 = 'c';
    let char2 = 'f';
    println!("The characters for the 2 are: {}, {}",char1, char2 );
}
