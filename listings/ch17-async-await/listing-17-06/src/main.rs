extern crate trpl; // required for mdbook test

// ANCHOR: all
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the spawned task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the main task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
// ANCHOR_END: all
