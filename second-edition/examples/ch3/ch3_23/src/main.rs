fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let v1 = x.0;
    let v2 = x.1;
    let v3 = x.2;

    println!("The value of (x,y,z) is: ({},{},{})", v1, v2, v3);
}
