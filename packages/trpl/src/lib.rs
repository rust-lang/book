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

use std::future::Future;

pub use futures::future::join;
pub use tokio::{runtime::Runtime, task::spawn as spawn_task, time::sleep};

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
