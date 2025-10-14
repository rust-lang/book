//! Fix incorrect round-tripping of block quotes in `pulldown-cmark-to-cmark`:
//!
//! - Eliminate extraneous leading `>`
//! - Eliminate extraneous indent.
//!
//! Note: later versions of `pulldown-cmark-to-cmark` will likely fix this, so
//! check when upgrading it if it is still necessary!

use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .unwrap_or_else(|e| panic!("{e}"));
        buffer
    };

    let fixed = cleanup_blockquotes(input);
    print!("{fixed}");
}

fn cleanup_blockquotes(input: String) -> String {
    let normal_start = EXTRA_SPACE.replace_all(&input, ">");
    let sans_empty_leading = EMPTY_LEADING.replace_all(&normal_start, "\n\n");
    sans_empty_leading.to_string()
}

lazy_static! {
    static ref EXTRA_SPACE: Regex = Regex::new("(?m)^ >").unwrap();
    static ref EMPTY_LEADING: Regex = Regex::new("\n\n> ?\n").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extra_space() {
        let input = " > Hello".to_string();
        let actual = cleanup_blockquotes(input);
        assert_eq!(actual, "> Hello");
    }

    #[test]
    fn empty_leading() {
        let input = "\n\n>\n> Hello".into();
        let actual = cleanup_blockquotes(input);
        assert_eq!(actual, "\n\n> Hello");
    }

    #[test]
    fn leading_after_extra_space_cleaned_up() {
        let input = r#"Start

>
> Note: Hey.

Wrap."#
            .into();

        let actual = cleanup_blockquotes(input);
        assert_eq!(
            actual,
            r#"Start

> Note: Hey.

Wrap."#
        );
    }

    /// This particular input was the result of running any of the mdbook
    /// preprocessors which use `pulldown-cmark-to-cmark@<=18.0.0`.
    #[test]
    fn regression_ch17_example() {
        //  This is an example of the original motivating input which we are fixing.
        let input = r#"
We have to explicitly await both of these futures, because futures in Rust are
*lazy*: they don’t do anything until you ask them to with `await`. (In fact,
Rust will show a compiler warning if you don’t use a future.) This should
remind you of our discussion of iterators [back in Chapter 13][iterators-lazy].
Iterators do nothing unless you call their `next` method—whether directly, or
using `for` loops or methods such as `map` which use `next` under the hood. With
futures, the same basic idea applies: they do nothing unless you explicitly ask
them to. This laziness allows Rust to avoid running async code until it’s
actually needed.

 >
 > Note: This is different from the behavior we saw when using `thread::spawn` in
 > the previous chapter, where the closure we passed to another thread started
 > running immediately. It’s also different from how many other languages
 > approach async! But it’s important for Rust. We’ll see why that is later.

Once we have `response_text`, we can then parse it into an instance of the
`Html` type using `Html::parse`. Instead of a raw string, we now have a data
type we can use to work with the HTML as a richer data structure. In particular,
we can use the `select_first` method to find the first instance of a given CSS
selector. By passing the string `"title"`, we’ll get the first `<title>`
element in the document, if there is one. Because there may not be any matching
element, `select_first` returns an `Option<ElementRef>`. Finally, we use the
`Option::map` method, which lets us work with the item in the `Option` if it’s
present, and do nothing if it isn’t. (We could also use a `match` expression
here, but `map` is more idiomatic.) In the body of the function we supply to
`map`, we call `inner_html` on the `title_element` to get its content, which is
a `String`. When all is said and done, we have an `Option<String>`.
"#.to_string();

        let actual = cleanup_blockquotes(input);
        assert_eq!(
            actual,
            r#"
We have to explicitly await both of these futures, because futures in Rust are
*lazy*: they don’t do anything until you ask them to with `await`. (In fact,
Rust will show a compiler warning if you don’t use a future.) This should
remind you of our discussion of iterators [back in Chapter 13][iterators-lazy].
Iterators do nothing unless you call their `next` method—whether directly, or
using `for` loops or methods such as `map` which use `next` under the hood. With
futures, the same basic idea applies: they do nothing unless you explicitly ask
them to. This laziness allows Rust to avoid running async code until it’s
actually needed.

> Note: This is different from the behavior we saw when using `thread::spawn` in
> the previous chapter, where the closure we passed to another thread started
> running immediately. It’s also different from how many other languages
> approach async! But it’s important for Rust. We’ll see why that is later.

Once we have `response_text`, we can then parse it into an instance of the
`Html` type using `Html::parse`. Instead of a raw string, we now have a data
type we can use to work with the HTML as a richer data structure. In particular,
we can use the `select_first` method to find the first instance of a given CSS
selector. By passing the string `"title"`, we’ll get the first `<title>`
element in the document, if there is one. Because there may not be any matching
element, `select_first` returns an `Option<ElementRef>`. Finally, we use the
`Option::map` method, which lets us work with the item in the `Option` if it’s
present, and do nothing if it isn’t. (We could also use a `match` expression
here, but `map` is more idiomatic.) In the body of the function we supply to
`map`, we call `inner_html` on the `title_element` to get its content, which is
a `String`. When all is said and done, we have an `Option<String>`.
"#
        );
    }
}
