pub fn takes_long_type(_f: Box<Fn() + Send + 'static>) {
    // ...snip...
}

pub fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // ...snip...
    Box::new(|| ())
}

fn main() {
    let _f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));
}
