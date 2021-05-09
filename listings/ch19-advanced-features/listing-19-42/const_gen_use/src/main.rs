#[derive(Debug)]
struct Foo {
    a : u32,
    b : i64,
    c : Option<u8>
}

/// builder struct for Foo
struct FooBuilder<const ASET: bool, const BSET: bool, const CSET: bool> {
    opa : Option<u32>,
    opb : Option<i64>,
    opc : Option<u8>
}

impl Foo {

    /// creates the FooBuilder
    fn new_builder() -> FooBuilder<false, false, false> {
        FooBuilder::<false, false, false> {
            opa : None,
            opb : None,
            opc : None
        }
    }
}

impl<const ASET: bool, const BSET: bool, const CSET: bool> FooBuilder<ASET, BSET, CSET> {

    /// function setting field 'a'.
    fn set_a(self, a:u32) -> FooBuilder<true, BSET, CSET>{
        FooBuilder::<true, BSET, CSET> {
            opa : Some(a),
            opb : self.opb,
            opc : self.opc
        }
    }

    /// function setting field 'b'.
    fn set_b(self, b:i64) -> FooBuilder<ASET, true, CSET>{
        FooBuilder::<ASET, true, CSET> {
            opa : self.opa,
            opb : Some(b),
            opc : self.opc
        }
    }

    /// function setting field 'c'.
    fn set_c(self, c:u8) -> FooBuilder<ASET, BSET, true>{
        FooBuilder::<ASET, BSET, true> {
            opa : self.opa,
            opb : self.opb,
            opc : Some(c)
        }
    }
}

impl<const CSET: bool> FooBuilder<true, true, CSET> {

    /// function building Foo from FooBuilder
    fn build(self) -> Foo {
        match (self.opa, self.opb, self.opc) {
            (Some(a), Some(b), c) => Foo{a,b,c},
            _ => unreachable!(),
        }
    }
}

fn main() {

    let foo1 = Foo::new_builder()
                .set_a(1)
                .set_b(2)
                .set_c(3)
                .build();
    println!("Foo1: {:?}", foo1);

    let foo2 = Foo::new_builder()
                .set_a(1)
                .set_b(2)
                .build();
    println!("Foo2: {:?}", foo2);

    /* Uncomment for compiler error
    let foo3 = Foo::new_builder()
                .set_a(1)
                .set_c(3)
                .build();
    println!("Foo3: {:?}", foo3);
    */
}