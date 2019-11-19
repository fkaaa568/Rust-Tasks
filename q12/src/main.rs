use std::io;
fn main() {
    println!("Enter Height in cm");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed");
    
    let height :f32 = height.trim().parse().unwrap();

    println!("Enter weight  in kg");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Failed");
    
    let weight :f32 = weight.trim().parse().unwrap();

    let bmi = height/weight*2.1; 

    println!("Your BMI IS {} ",bmi);
    
}
