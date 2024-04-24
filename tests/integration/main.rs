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

use trpl::async_main;

#[test]
fn re_exported_macro_works() {
    #[async_main]
    async fn demo() -> &'static str {
        let val = async { "Hello" }.await;
        assert_eq!(val, "Hello", "Async is usable in async_main function");
        val
    }

    assert_eq!(demo(), "Hello", "value returns correctly");
}
