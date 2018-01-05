fn main() {
  let guess
  : u32
  = "42".parse().expect("Not a number!");
  println!("guess = {}",guess);
}