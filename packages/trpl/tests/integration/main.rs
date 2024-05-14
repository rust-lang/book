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

use trpl::{async_main, block_on};

/// This test makes sure the re-exported version of the `tokio::main` macro,
/// which is applied like `#[tokio::main] async fn some_fn() { … }`, continues
/// to work. However, tests cannot use `async fn`, so to test it, we need to
/// have a non-`async` test function, which then applies the macro to an `async`
/// function in its body, and invokes *that*.
#[test]
fn re_exported_macro_works() {
    #[async_main]
    async fn internal() -> &'static str {
        let val = async { "Hello" }.await;
        assert_eq!(val, "Hello", "Async is usable in async_main function");
        val
    }

    assert_eq!(internal(), "Hello", "value returns correctly");
}

#[test]
fn re_exported_block_on_works() {
    let val = block_on(async { "Hello" });
    assert_eq!(val, "Hello");
}
