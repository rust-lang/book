pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}