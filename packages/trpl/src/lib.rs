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

pub use futures::executor::block_on;
pub use tokio::main as async_main;
