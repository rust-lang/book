// ANCHOR: here
use std::fmt::{Debug, Formatter};

// struct with generic type T and two const
// generics N1 and N2 both of which are usize.
struct ArrayPair<T, const N1: usize, const N2: usize> {
    left: [T; N1],
    right: [T; N2],
}

// impl of Debug for ArrayPair
impl<T: Debug, const N1: usize, const N2: usize> Debug for ArrayPair<T, N1, N2>
{
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}

// enum with generic type T and const generic N of type usize.
enum Either<T, const N : usize> {
    A([T;N]),
    B
}

// function Foo with const generics N of type usize and M of type bool.
fn foo<const N: usize, const M: bool>(_: ArrayPair<i32,N,N>) {}

// trait definition with generic type U with const generics N of type usize.
trait Bar<U, const N: usize> {}
// ANCHOR_END: here