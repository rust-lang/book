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

// trait definition with generic type U with const generics N.
trait Bar<U, const N: usize> {}

// ANCHOR: here
// --snip--
fn main() {

    const Z: usize = 3;

    // ArrayPair with T=i32, N1=1, N2=3.
    let _ : ArrayPair<i32, 1, 3> = ArrayPair::<i32, 1, Z> {
        left : [0], right : [0,1,2]
        };

    // ArrayPair with T=i32, N1=2, N2=2.
    let x : ArrayPair::<i32, 2, 2> = ArrayPair::<i32, 2, 2> {
        left : [0,1], right : [2,3]
        };

    // Either enum with T=char, N=2.
    let _: Either<char,2> = Either::B;

    // Either enum with T=char, N=0.
    let _: Either<char,0> = Either::A([]);

    // foo with N=2, M=true.
    foo::<2,true>(x);
}

// impl Bar<U,6> for ArrayPair with T=&'a str, N1=5 and N2 still generic.
impl<'a, U, const N2: usize> Bar<U, 6> for ArrayPair::<&'a str,5,N2> {}

// impl Bar<U,7> for ArrayPair with T=&'a str, N1 and N2 still generic.
impl<'a, U, const N1: usize, const N2: usize> Bar::<U, 7> for ArrayPair::<&'a str,N1,N2> {}
// ANCHOR_END: here