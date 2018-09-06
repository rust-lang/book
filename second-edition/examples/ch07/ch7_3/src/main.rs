pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}
use a::series::of::nested_modules;

fn main() {
    nested_modules();
}