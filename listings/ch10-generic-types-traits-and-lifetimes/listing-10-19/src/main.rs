fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is {result}");
}
