fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}