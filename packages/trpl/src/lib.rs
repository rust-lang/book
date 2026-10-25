//! A support crate for [_The Rust Programming Language_][trpl].
//!
//! [trpl]: https://doc.rust-lang.org/book
//!
//! This crate mostly just re-exports items from *other* crates. It exists for
//! two main reasons:
//!
//! 1. So that as you read along in _The Rust Programming Language_, you can
//!    add just one dependency, rather than however many we end up with, and
//!    likewise use only one set of imports.
//!
//! 2. So that we can more easily guarantee it keeps building and working. Since
//!    we control the contents of this crate and when it changes, readers will
//!    never be broken by upstream changes, e.g. if Tokio does a breaking 2.0
//!    release at some point.

// For direct use within the `trpl` crate, *not* re-exported.
use std::{future::Future, pin::pin};

use futures::future;

// Re-exports, to be used like `trpl::join`.
pub use futures::{
    future::{Either, join, join_all, join3},
    join,
};
pub use tokio::{
    fs::read_to_string,
    runtime::Runtime,
    // We use the `unbounded` variants because they most closely match the APIs
    // from `std::sync::mpsc::channel`. Tokio's API choices are interesting:
    //
    // | `tokio::sync::mpsc` | `std::sync::mpsc` |
    // | ------------------- | ----------------- |
    // | `channel`           | `sync_channel`    |
    // | `unbounded_channel` | `channel`         |
    //
    // The book collapses these differences for pedagogical simplicity, so that
    // readers are not asking why `unbounded` is now important and can focus on
    // the more important differences between sync and async APIs.
    sync::mpsc::{
        UnboundedReceiver as Receiver, UnboundedSender as Sender,
        unbounded_channel as channel,
    },
    task::{JoinHandle, spawn as spawn_task, yield_now},
    time::{interval, sleep},
};

pub use tokio_stream::{
    Stream, StreamExt, iter as stream_from_iter,
    wrappers::{IntervalStream, UnboundedReceiverStream as ReceiverStream},
};

/// Run a single future to completion on a bespoke Tokio `Runtime`.
///
/// Every time you call this, a new instance of `tokio::runtime::Runtime` will
/// be created (see the implementation for details: it is trivial). This is:
///
/// - Reasonable for teaching purposes, in that you do not generally need to set
///   up more than one runtime anyway, and especially do not in basic code like
///   we are showing!
///
/// - Not *that* far off from what Tokio itself does under the hood in its own
///   `tokio::main` macro for supporting `async fn main`.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}

/// This function has been renamed to `block_on`; please see its documentation.
/// This function remains to maintain compatibility with the online versions
/// of the book that use the name `run`.
pub fn run<F: Future>(future: F) -> F::Output {
    block_on(future)
}

/// Run two futures, taking whichever finishes first and canceling the other.
///
/// Notice that this is built on [`futures::future::select`], which has the
/// same overall semantics but does *not* drop the slower future. The idea there
/// is that you can work with the first result and then later *also* continue
/// waiting for the second future.
///
/// We drop the slower future for the sake of simplicity in the examples: no
/// need to deal with the tuple and intentionally ignore the second future this
/// way!
///
/// Note that this only works as “simply” as it does because:
///
/// - It takes ownership of the futures.
/// - It internally *pins* the futures.
/// - It throws away (rather than returning) the unused future (which is why it
///   can get away with pinning them).
pub async fn select<A, B, F1, F2>(f1: F1, f2: F2) -> Either<A, B>
where
    F1: Future<Output = A>,
    F2: Future<Output = B>,
{
    let f1 = pin!(f1);
    let f2 = pin!(f2);
    match future::select(f1, f2).await {
        Either::Left((a, _f2)) => Either::Left(a),
        Either::Right((b, _f1)) => Either::Right(b),
    }
}

/// This function has been renamed to `select`; please see its documentation.
/// This function remains to maintain compatibility with the online versions
/// of the book that use the name `race`.
pub async fn race<A, B, F1, F2>(f1: F1, f2: F2) -> Either<A, B>
where
    F1: Future<Output = A>,
    F2: Future<Output = B>,
{
    select(f1, f2).await
}

/// Fetch data from a URL. For more convenient use in _The Rust Programming
/// Language_, panics instead of returning a [`Result`] if the request fails.
pub async fn get(url: &str) -> Response {
    Response(reqwest::get(url).await.unwrap())
}

/// A thin wrapper around [`reqwest::Response`] to make the demos in _The Rust
/// Programming Language_ substantially nicer to use.
pub struct Response(reqwest::Response);

impl Response {
    /// Get the full response text.
    ///
    /// If the response cannot be deserialized, this panics instead of returning
    /// a [`Result`] (for convenience in the demo).
    pub async fn text(self) -> String {
        self.0.text().await.unwrap()
    }
}

/// A thin wrapper around [`scraper::Html`] to make the demos in _The Rust
/// Programming Language_ substantially nicer to use.
pub struct Html {
    inner: scraper::Html,
}

impl Html {
    /// Parse an HTML document from a string.
    ///
    /// This is just a thin wrapper around `scraper::Html::parse_document` to
    /// keep the exported API surface simpler.
    pub fn parse(source: &str) -> Html {
        Html {
            inner: scraper::Html::parse_document(source),
        }
    }

    /// Get the first item in the document matching a string selector. Returns
    /// Some()
    ///
    /// If the selector is not a valid CSS selector, panics rather than
    /// returning a [`Result`] for convenience.
    pub fn select_first<'a>(
        &'a self,
        selector: &'a str,
    ) -> Option<scraper::ElementRef<'a>> {
        let selector = scraper::Selector::parse(selector).unwrap();
        self.inner.select(&selector).nth(0)
    }
}
