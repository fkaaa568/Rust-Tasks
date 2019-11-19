use std::io;
fn main() {

let mut numerator = String::new();
io::stdin().read_line(&mut numerator).expect("");

let numerator : u32 = numerator
.trim().parse().unwrap();


let mut Denominator = String::new();
io::stdin().read_line(&mut Denominator).expect("");

let Denominator : u32 = Denominator
.trim().parse().unwrap();

if numerator % 2 == 0 {
    
    println!("{} is Completely Divisible by {}",numerator,Denominator);
}
else{
    print!("{} is not Divisible by {}",numerator,Denominator);
}
}