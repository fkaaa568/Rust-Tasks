use std::io;
fn main() {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("");
     
    let number : u32 = number.trim().parse().unwrap();
    check_number(number);
}

fn check_number(number:u32)->u32{
    if number %2==0{
        println!(" {} is Even Number",number);
    }
    else{
        println!("{} is Odd Number",number);
    };
    number
}
