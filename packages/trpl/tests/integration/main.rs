//! Integration tests for the crate.
//!
//! These all live in a *single* integration test crate, `tests/integration`,
//! because each integration test is a dedicated binary crate which has to be
//! compiled separately. While that is not really a problem for a crate this
//! small, we have chosen to follow this “best practice” here as a good example.
//!
//! For more details on why you might prefer this pattern see [this post][post].
//!
//! [post]: https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html

use trpl::{block_on, sleep, spawn_task};

/// This test is foundational for all the others, as they depend on `block_on`.
///
/// If we mess this up, *all* the tests below will fail -- so by the same token,
/// if all the tests below are failing, this one probably is too; fix it and the
/// others will likely start working again.
#[test]
fn re_exported_block_on_works() {
    let val = block_on(async { "Hello" });
    assert_eq!(val, "Hello");
}

#[test]
fn re_exported_spawn_works() {
    let result = block_on(async {
        let handle_a = spawn_task(async { "Hello" });
        let handle_b = spawn_task(async { "Goodbye" });
        vec![handle_a.await.unwrap(), handle_b.await.unwrap()]
    });

    assert_eq!(result, vec!["Hello", "Goodbye"]);
}

#[test]
fn re_exported_sleep_works() {
    let val = block_on(async {
        sleep(std::time::Duration::from_micros(1)).await;
        "Done!"
    });
    assert_eq!(val, "Done!");
}
