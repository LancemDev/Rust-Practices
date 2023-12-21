fn main(){
    another_function(7,6);
    let x = five();
    println!("the function returns: {}", x);
    println!("The sum of 4 and 12 is: {}", add_these_two(12, 4));
}

fn another_function(x:i8, y:i8){
    println!("The sum of the two is: {}", x+y);
}


// i32 is the data type of the return value
// used after the ->
// note that 5*2 is an expression and hence doesn't need a semicolon
fn five() -> i32{
    5*2
}

fn add_these_two(x:i32, y:i32) -> i32{
    x+y
}