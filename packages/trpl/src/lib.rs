//! A support crate for _The Rust Programming Language_.
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
use std::{future::Future, pin::pin, time::Duration};
use tokio::time;

// Re-exports, to be used like `trpl::join`.
pub use futures::{
    future::{self, join, join3, join_all, Either},
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
        unbounded_channel as channel, UnboundedReceiver as Receiver,
        UnboundedSender as Sender,
    },
    task::{spawn as spawn_task, yield_now},
    time::{interval, sleep},
};

pub use tokio_stream::{
    iter as stream_from_iter,
    wrappers::{IntervalStream, UnboundedReceiverStream as ReceiverStream},
    Stream, StreamExt,
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

/// Race a future against a specified timeout duration.
///
/// Under the hood, this uses `tokio::time::timeout`, but instead of returning
/// Tokio's internal error type in the case of a failure, this simply returns
/// the duration which had elapsed. (It actually provides strictly *more* info
/// than Tokio's error does, though it does not `impl Error`.)
pub async fn timeout<F>(
    duration: Duration,
    future: F,
) -> Result<F::Output, Duration>
where
    F: Future,
{
    time::timeout(duration, future).await.map_err(|_| duration)
}

/// Run two futures, taking whichever finishes first and canceling the other.
///
/// Notice that this is built on [`futures::future::select`], which has the
/// same overall semantics but does *not* drop the slower future. The idea there
/// is that you can work with the first result and then later *also* continue
/// waiting for the second future.
///
/// We use the `race` semantics, where the slower future is simply dropped, for
/// the sake of simplicity in the examples: no need to deal with the tuple and
/// intentionally ignore the second future this way!
pub async fn race<A, B, F1, F2>(f1: F1, f2: F2) -> Either<A, B>
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
