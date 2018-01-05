fn main() {
    trait Foo {}

    struct Bar<'a> {
        pub x: &'a i32,
    }

    impl<'a> Foo for Bar<'a> {}

    let num = 5;

    let _obj = Box::new(Bar { x: &num }) as Box<Foo>;
}
