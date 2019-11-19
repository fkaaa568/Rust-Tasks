use std::io;
fn main() {
   let mut feet = String::new();
   io::stdin().read_line(&mut feet).expect("Failed");

    let feet : f32 = feet.trim().parse().unwrap();

    let mut centimeter = String::new();
   io::stdin().read_line(&mut centimeter).expect("Failed");

    let centimeter : f32 = centimeter.trim().parse().unwrap();
    
    let cm = feet / centimeter*100 as f32;
    println!("Enter the Height in feet is {} ",feet);
    println!("There are {} in cm {} ft",cm,feet);


}
