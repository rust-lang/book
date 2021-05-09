fn foo<const B: bool>() {
    if B {
        println!("B is true");
    }
    else {
        println!("B is false");
    }
}

fn main() {
    println!("1");
    foo::<true>();
    println!("2");
    foo::<false>();
    println!("3");
}