fn main() {
    let value = 98_222_000;
    println!("value = {}", value);

    let mut value = 0xff;
    println!("value = {}", value);
    value = 0x_ff;
    println!("value = {}", value);

    let mut value = 0o77;
    println!("value = {}", value);
    value = 0o_77;
	println!("value = {}", value);

    let mut value = 0b1111_0000;
    println!("value = {}", value);
    value = 0b1_111_0000;
    println!("value = {}", value);

    let value = b'A';
    println!("value = {}", value);
}
