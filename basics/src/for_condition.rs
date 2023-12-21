// Control flow
fn main(){
    let x: [i32; 5] = [3,1,2,3,4,];

    for element in x.iter(){
        println!("Element: {}", element);
    }

    // Countdown using a for loop
    // rev() reverses the sequence
    for number in (1..4).rev(){
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}