use std::io;
fn main() {
    let mut x1 = String::new();
    io::stdin().read_line(&mut x1).expect("Failed");

    let x1 : u32 = x1.trim().parse().unwrap();
    println!("Enter Coordinate for x1:{}",x1);

    let mut y1 = String::new();
    io::stdin().read_line(&mut y1).expect("Failed");

    let y1 : u32 = y1.trim().parse().unwrap();
    println!("Enter Coordinate for y1:{}",y1);

    let mut x2 = String::new();
    io::stdin().read_line(&mut x2).expect("Failed");

    let x2 : u32 = x2.trim().parse().unwrap();
    println!("Enter Coordinate for x2:{}",x2);

    let mut y2 = String::new();
    io::stdin().read_line(&mut y2).expect("Failed");

    let y2 : u32 = y2.trim().parse().unwrap();
    println!("Enter Coordinate for y2:{}",y2);

    let distance = ((x2 - x1)*(x2-x1))+((y2-y1)*(y2-1));
    
    println!("The Between points ({},{}) and ({},{}) is {}",x1,x2,y1,y2,distance);
}


