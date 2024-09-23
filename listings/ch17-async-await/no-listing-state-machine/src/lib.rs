extern crate trpl; // required for mdbook test

// ANCHOR: enum
enum PageTitleFuture<'a> {
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { response: trpl::Response },
}
// ANCHOR_END: enum
