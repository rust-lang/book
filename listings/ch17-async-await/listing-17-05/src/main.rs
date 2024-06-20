use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // ANCHOR: task
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(1)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(1)).await;
        }
        // ANCHOR_END: task
    });
}