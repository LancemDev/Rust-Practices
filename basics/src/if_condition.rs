// Control flow

fn main(){
    let x = 10;
    if x < 10 {
        println!("x is less than 10");
    }else{
        println!("x is equal to or greater than 10")
    }


    if x % 4 == 0{
        println!("Number is divisible by 4");
    } else if x % 5 == 0 && x % 10 == 0{
        println!("Number is divisible by 5");
    } else if x % 3 == 0{
        println!("Number is divisible by 3");
    } else if x % 2 == 0{
        println!("Number is divisible by 2");
    } else{
        println!("Number is not divisible by 2, 3, 4 or 5");
    }


    let a = 0;
    loop{
        if a > 10 {
            break;
        } else{
            println!("Iteration number: {}", a+1);
        }
    }
}