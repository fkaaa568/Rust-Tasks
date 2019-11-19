use std::io;
fn main() {
    
let mut base = String::new();
io::stdin().read_line(&mut base).expect("Failed");
println!("Enter magnitude of Triangle base : {} ",base);

let base :u32 = base.trim().parse().unwrap();

let mut height = String::new();
io::stdin().read_line(&mut height).expect("Failed");

let height :u32 = height.trim().parse().unwrap();
println!("Enter magnitude of Triangle height :{}",height);

let add = base + height;

println!("Area of a Triangle with Height :{} and Base :{} is :{} ",height,base,add);

}

