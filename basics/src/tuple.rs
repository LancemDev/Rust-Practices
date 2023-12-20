fn main(){

    /*
    -------COMPOUND TYPES-------
     */


    // TUPLES
    // let tup: (i8, u64, f32) = (500, 200, 300);

    let tup= (12, 34,14);
    let(x,y,z) = tup;
    println!("The tuple is: ({}, {}, {})", x, y, z);

    let tup_2: (i32, i8, i32) = (12, 3, 12);
    //deconstruction
    let (a, b, c) = tup_2;
    println!("The second tuple: ({}, {}, {})", a, b, c);
}