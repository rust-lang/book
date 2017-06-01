// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


// FIXME: We have some long lines that could be refactored, but it's not a big deal.
// ignore-tidy-linelength

extern crate regex;

use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use regex::{Regex, Captures};

fn main() {

    write_md(parse_links(parse_references(read_md())));
}

fn read_md() -> String {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => buffer,
        Err(error) => panic!(error),
    }
}

fn write_md(output: String) {
    write!(io::stdout(), "{}", output).unwrap();
}

fn parse_references(buffer: String) -> (String, HashMap<String, String>) {
    let mut ref_map = HashMap::new();
    // FIXME: Currently doesn't handle "title" in following line
    let re = Regex::new(r###"(?m)\n?^ {0,3}\[([^]]+)\]:[[:blank:]]*(.*)$"###).unwrap();
    let output = re.replace_all(&buffer, |caps: &Captures| {
        let key = caps.at(1).unwrap().to_owned().to_uppercase();
        let val = caps.at(2).unwrap().to_owned();
        if ref_map.insert(key, val).is_some() {
            panic!("Did not expect markdown page to have duplicate reference");
        }
        "".to_string()
    });
    (output, ref_map)
}

fn parse_links((buffer, ref_map): (String, HashMap<String, String>)) -> String {
    // FIXME: check which punctuation is allowed by spec
    let re = Regex::new(r###"(?:(?P<pre>(?:```(?:[^`]|`[^`])*`?\n```\n)|(?:[^[]`[^`\n]+[\n]?[^`\n]*`))|(?:\[(?P<name>[^]]+)\](?:(?:\([[:blank:]]*(?P<val>[^")]*[^ ])(?:[[:blank:]]*"[^"]*")?\))|(?:\[(?P<key>[^]]*)\]))?))"###).expect("could not create regex");
    let error_code = Regex::new(r###"^E\d{4}$"###).expect("could not create regex");
    let output = re.replace_all(&buffer, |caps: &Captures| {
        match caps.name("pre") {
            Some(pre_section) => format!("{}", pre_section.to_owned()),
            None => {
                let name = caps.name("name").expect("could not get name").to_owned();
                // Really we should ignore text inside code blocks,
                // this is a hack to not try to treat `#[derive()]`,
                // `[profile]`, `[test]`, or `[E\d\d\d\d]` like a link
                if name.starts_with("derive(") ||
                   name.starts_with("profile") ||
                   name.starts_with("test") ||
                   error_code.is_match(&name) {
                    return name
                }

                let val = match caps.name("val") {
                    // [name](link)
                    Some(value) => value.to_owned(),
                    None => {
                        match caps.name("key") {
                            Some(key) => {
                                match key {
                                    // [name][]
                                    "" => format!("{}", ref_map.get(&name.to_uppercase()).expect(&format!("could not find url for the link text `{}`", name))),
                                    // [name][reference]
                                    _ => format!("{}", ref_map.get(&key.to_uppercase()).expect(&format!("could not find url for the link text `{}`", key))),
                                }
                            }
                            // [name] as reference
                            None => format!("{}", ref_map.get(&name.to_uppercase()).expect(&format!("could not find url for the link text `{}`", name))),
                        }
                    }
                };
                format!("{} at *{}*", name, val)
            }
        }
    });
    output
}

#[cfg(test)]
mod tests {
    fn parse(source: String) -> String {
        super::parse_links(super::parse_references(source))
    }

    #[test]
    fn parses_inline_link() {
        let source = r"This is a [link](http://google.com) that should be expanded".to_string();
        let target = r"This is a link at *http://google.com* that should be expanded".to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn parses_multiline_links() {
        let source = r"This is a [link](http://google.com) that
should appear expanded. Another [location](/here/) and [another](http://gogogo)"
            .to_string();
        let target = r"This is a link at *http://google.com* that
should appear expanded. Another location at */here/* and another at *http://gogogo*"
            .to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn parses_reference() {
        let source = r"This is a [link][theref].
[theref]: http://example.com/foo
more text"
            .to_string();
        let target = r"This is a link at *http://example.com/foo*.
more text"
            .to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn parses_implicit_link() {
        let source = r"This is an [implicit][] link.
[implicit]: /The Link/"
            .to_string();
        let target = r"This is an implicit at */The Link/* link.".to_string();
        assert_eq!(parse(source), target);
    }
    #[test]
    fn parses_refs_with_one_space_indentation() {
        let source = r"This is a [link][ref]
 [ref]: The link"
            .to_string();
        let target = r"This is a link at *The link*".to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn parses_refs_with_two_space_indentation() {
        let source = r"This is a [link][ref]
  [ref]: The link"
            .to_string();
        let target = r"This is a link at *The link*".to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn parses_refs_with_three_space_indentation() {
        let source = r"This is a [link][ref]
   [ref]: The link"
            .to_string();
        let target = r"This is a link at *The link*".to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    #[should_panic]
    fn rejects_refs_with_four_space_indentation() {
        let source = r"This is a [link][ref]
    [ref]: The link"
            .to_string();
        let target = r"This is a link at *The link*".to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn ignores_optional_inline_title() {
        let source = r###"This is a titled [link](http://example.com "My title")."###.to_string();
        let target = r"This is a titled link at *http://example.com*.".to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn parses_title_with_puctuation() {
        let source = r###"[link](http://example.com "It's Title")"###.to_string();
        let target = r"link at *http://example.com*".to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn parses_name_with_punctuation() {
        let source = r###"[I'm here](there)"###.to_string();
        let target = r###"I'm here at *there*"###.to_string();
        assert_eq!(parse(source), target);
    }
    #[test]
    fn parses_name_with_utf8() {
        let source = r###"[user’s forum](the user’s forum)"###.to_string();
        let target = r###"user’s forum at *the user’s forum*"###.to_string();
        assert_eq!(parse(source), target);
    }


    #[test]
    fn parses_reference_with_punctuation() {
        let source = r###"[link][the ref-ref]
[the ref-ref]:http://example.com/ref-ref"###
            .to_string();
        let target = r###"link at *http://example.com/ref-ref*"###.to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn parses_reference_case_insensitively() {
        let source = r"[link][Ref]
[ref]: The reference"
            .to_string();
        let target = r"link at *The reference*".to_string();
        assert_eq!(parse(source), target);
    }
    #[test]
    fn parses_link_as_reference_when_reference_is_empty() {
        let source = r"[link as reference][]
[link as reference]: the actual reference"
            .to_string();
        let target = r"link as reference at *the actual reference*".to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn parses_link_without_reference_as_reference() {
        let source = r"[link] is alone
[link]: The contents"
            .to_string();
        let target = r"link at *The contents* is alone".to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    #[ignore]
    fn parses_link_without_reference_as_reference_with_asterisks() {
        let source = r"*[link]* is alone
[link]: The contents"
            .to_string();
        let target = r"*link* at *The contents* is alone".to_string();
        assert_eq!(parse(source), target);
    }
    #[test]
    fn ignores_links_in_pre_sections() {
        let source = r###"```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```
"###
            .to_string();
        let target = source.clone();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn ignores_links_in_quoted_sections() {
        let source = r###"do not change `[package]`."###.to_string();
        let target = source.clone();
        assert_eq!(parse(source), target);
    }
    #[test]
    fn ignores_links_in_quoted_sections_containing_newlines() {
        let source = r"do not change `this [package]
is still here` [link](ref)"
            .to_string();
        let target = r"do not change `this [package]
is still here` link at *ref*"
            .to_string();
        assert_eq!(parse(source), target);
    }

    #[test]
    fn ignores_links_in_pre_sections_while_still_handling_links() {
        let source = r###"```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```
Another [link]
more text
[link]: http://gohere
"###
            .to_string();
        let target = r###"```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```
Another link at *http://gohere*
more text
"###
            .to_string();
        assert_eq!(parse(source), target);
    }
    #[test]
    fn ignores_quotes_in_pre_sections() {
        let source = r###"```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
src/main.rs:23:21: 23:35 error: mismatched types [E0308]
src/main.rs:23     match guess.cmp(&secret_number) {
                                   ^~~~~~~~~~~~~~
src/main.rs:23:21: 23:35 help: run `rustc --explain E0308` to see a detailed explanation
src/main.rs:23:21: 23:35 note: expected type `&std::string::String`
src/main.rs:23:21: 23:35 note:    found type `&_`
error: aborting due to previous error
Could not compile `guessing_game`.
```
"###
            .to_string();
        let target = source.clone();
        assert_eq!(parse(source), target);
    }
    #[test]
    fn ignores_short_quotes() {
        let source = r"to `1` at index `[0]` i".to_string();
        let target = source.clone();
        assert_eq!(parse(source), target);
    }
    #[test]
    fn ignores_pre_sections_with_final_quote() {
        let source = r###"```bash
$ cargo run
   Compiling points v0.1.0 (file:///projects/points)
error: the trait bound `Point: std::fmt::Display` is not satisfied [--explain E0277]
 --> src/main.rs:8:29
8 |>     println!("Point 1: {}", p1);
  |>                             ^^
<std macros>:2:27: 2:58: note: in this expansion of format_args!
<std macros>:3:1: 3:54: note: in this expansion of print! (defined in <std macros>)
src/main.rs:8:5: 8:33: note: in this expansion of println! (defined in <std macros>)
note: `Point` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
note: required by `std::fmt::Display::fmt`
```
`here` is another [link](the ref)
"###.to_string();
        let target = r###"```bash
$ cargo run
   Compiling points v0.1.0 (file:///projects/points)
error: the trait bound `Point: std::fmt::Display` is not satisfied [--explain E0277]
 --> src/main.rs:8:29
8 |>     println!("Point 1: {}", p1);
  |>                             ^^
<std macros>:2:27: 2:58: note: in this expansion of format_args!
<std macros>:3:1: 3:54: note: in this expansion of print! (defined in <std macros>)
src/main.rs:8:5: 8:33: note: in this expansion of println! (defined in <std macros>)
note: `Point` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
note: required by `std::fmt::Display::fmt`
```
`here` is another link at *the ref*
"###.to_string();
        assert_eq!(parse(source), target);
    }
    #[test]
    fn parses_adam_p_cheatsheet() {
        let source = r###"[I'm an inline-style link](https://www.google.com)

[I'm an inline-style link with title](https://www.google.com "Google's Homepage")

[I'm a reference-style link][Arbitrary case-insensitive reference text]

[I'm a relative reference to a repository file](../blob/master/LICENSE)

[You can use numbers for reference-style link definitions][1]

Or leave it empty and use the [link text itself][].

URLs and URLs in angle brackets will automatically get turned into links.
http://www.example.com or <http://www.example.com> and sometimes
example.com (but not on Github, for example).

Some text to show that the reference links can follow later.

[arbitrary case-insensitive reference text]: https://www.mozilla.org
[1]: http://slashdot.org
[link text itself]: http://www.reddit.com"###
            .to_string();

        let target = r###"I'm an inline-style link at *https://www.google.com*

I'm an inline-style link with title at *https://www.google.com*

I'm a reference-style link at *https://www.mozilla.org*

I'm a relative reference to a repository file at *../blob/master/LICENSE*

You can use numbers for reference-style link definitions at *http://slashdot.org*

Or leave it empty and use the link text itself at *http://www.reddit.com*.

URLs and URLs in angle brackets will automatically get turned into links.
http://www.example.com or <http://www.example.com> and sometimes
example.com (but not on Github, for example).

Some text to show that the reference links can follow later.
"###
            .to_string();
        assert_eq!(parse(source), target);
    }



}
