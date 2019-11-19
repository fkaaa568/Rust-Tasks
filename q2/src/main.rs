use std::io;
fn main() {
    let mut  number = String::new();
    io::stdin().read_line(&mut number).expect("");

    let number : i32 = number
    .trim().parse().unwrap();


    if number > 0 {
        println!("The Number is positive");
    }

    else if number < 0 {
        println!("The Number is Negative");
    }

    else{
        println!("The Number is Zero");
    }
}


