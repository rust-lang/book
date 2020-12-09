fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = rand::random::<usize>();
    let index = index | 15;
    let index = index & 15;

    let element = a[index];

    println!("The value of element is: {}", element);
}
