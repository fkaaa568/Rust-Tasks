use std::io;
fn main() {
let mut circle = String::new();
io::stdin().read_line(&mut circle).expect("");

let circle : f32 = circle 
.trim()
.parse().
unwrap();
println!("The Area Of Circle : {} ", radius_of(circle));
}

fn radius_of(x:f32)->f32{

    let mut circum = 3.14 as f32;
    circum*x*x
}