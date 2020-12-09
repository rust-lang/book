fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = std::env::args()
        .nth(1)
        .and_then(|x| x.parse().ok())
        .unwrap_or(15);
    let index = index | 15;
    let index = index & 15;
// At this point, irrespective of the input,
// index has the value 15.

    let element = a[index];

    println!("The value of element is: {}", element);
}
