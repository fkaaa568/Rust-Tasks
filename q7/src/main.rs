use std::io;
fn main() {
  let mut vowels = String::new();
  io::stdin().read_line(&mut vowels).expect("Failed");

if vowels.trim() == "A" || vowels.trim() == "E"|| vowels.trim() == "I"|| vowels.trim() == "O"|| vowels.trim() == "U"{
  println!("The letter :{} is vowel",vowels);
}
else{
  println!("The letter :{} is not vovel",vowels);
};

}



