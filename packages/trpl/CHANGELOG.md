# CHANGELOG

## 0.3.0

This is intended to be a backwards-compatible release.

- Added aliases of method names to better align with other async crates' terminology after tech
  review feedback, including:
  - `select` instead of `race`
  - `block_on` instead of `run`
- Upgraded Rust, the edition, `ring`, and `quinn-proto`
- Switched to `rustls`


## 0.2.0

- Added `get`, `Response`, and `Html` to support more examples in chapter 17.

## 0.1.0

Initial release! Adds support code for the first draft of the new async chapter of the book.
