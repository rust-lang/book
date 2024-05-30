use std::time::Duration;

fn main() {
    trpl::block_on(async { trpl::select!(todo!()) });
}
