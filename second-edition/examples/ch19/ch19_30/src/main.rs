type Thunk = Box<Fn() + Send + 'static>;

pub fn takes_long_type(_f: Thunk) {
    // ...snip...
}

pub fn returns_long_type() -> Thunk {
    // ...snip...
    Box::new(|| ())
}

fn main() {
    let _f: Thunk = Box::new(|| println!("hi"));
}