// ANCHOR: here
fn foo<const N: usize>() {}

fn bar<T, const M: usize>() {
    foo::<M>(); // ok: `M` is a const parameter
    foo::<2021>(); // ok: `2021` is a literal
    foo::<{20 * 100 + 20 * 10 + 1}>(); // ok: const expression contains no generic parameters

    foo::<{ M + 1 }>(); // error: const expression contains the generic parameter `M`
    foo::<{ std::mem::size_of::<T>() }>(); // error: const expression contains the generic parameter `T`

    let _: [u8; M]; // ok: `M` is a const parameter
    let _: [u8; std::mem::size_of::<T>()]; // error: const expression contains the generic parameter `T`
}
//ANCHOR_END: here

fn main() {}