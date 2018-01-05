fn main() {
    let guess = "3";
    loop {
        let _guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
}
