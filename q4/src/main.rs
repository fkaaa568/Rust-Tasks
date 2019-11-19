use std::io;

fn main() {
    let mut sphere = String::new();
    io::stdin().read_line(&mut sphere).expect("");

    let sphere : f32 = sphere.trim().parse().unwrap();
    
    const PI : f32 = 3.14;
    let mut  sphere = 4.0/3.0 * PI * (sphere * sphere * sphere);
        
    print!("Enter The Radius of Sphere \n");
    println!("Volume of Sphere with radius is {}",sphere);

}