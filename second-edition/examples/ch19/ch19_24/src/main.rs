trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) {
        println!("Baz’s impl of Foo");
    }
}

impl Bar for Baz {
    fn f(&self) {
        println!("Baz’s impl of Bar");
    }
}

impl Baz {
    fn f(&self) {
        println!("Baz's impl");
    }
}

fn main() {
    let b = Baz;
    b.f();
}
