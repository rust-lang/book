struct Foo<const N: u32>;

impl<const N: u32> Foo<N> {
    const X: u32 = 5*N;
}

fn main() {
    assert_eq!(10, Foo::<2>::X);
}