fn main() {
    // variable
    let _x: i32 = 12;
    // Note the use of "{}". Apparently that's the naming convention of string literals
    println!("{}",_x);

    // Note that there's a naming convention for constants
    const MAX_POINTS: u32 = 100_000;
    println!("{}",MAX_POINTS);
}
