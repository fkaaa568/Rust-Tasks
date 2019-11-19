
    use std::io;
fn main() {
    println!("Please Enter The String");

   let mut Strings = String::new();
   io::stdin().read_line(&mut Strings).expect("");

    println!("How Many Copies Do you Have");
   let mut number = String::new();
    io::stdin().read_line(&mut number).expect("failed");

    let number : u32 = number.trim().parse().unwrap();

     let mut text = String::new();
   io::stdin().read_line(&mut text).expect("");
   
   for a in 0..number{
       text = text.to_string() + &text;
   }
       println!("{}",text);

}
