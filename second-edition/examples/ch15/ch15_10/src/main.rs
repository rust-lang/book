fn main() {
    let mut s = String::from("hello");

    let _r1 = &mut s;
    let _r2 = &mut s;
}
