use mdbook::{
    book::Book,
    errors::Result,
    preprocess::{Preprocessor, PreprocessorContext},
    utils::new_cmark_parser,
    BookItem,
};
use pulldown_cmark::{html, Event};
use pulldown_cmark_to_cmark::cmark;
use xmlparser::{Token, Tokenizer};

/// A preprocessor for rendering listings more elegantly.
///
/// Given input like this:
///
/// ````markdown
/// <Listing number="1-2" file-name="src/main.rs" caption="Some *text*, yeah?">
///
/// ```rust
/// fn main() {}
/// ```
///
/// </Listing>
///
/// ````
///
/// With no configuration, or with `output-mode = "default"`, it renders the
/// following Markdown to be further preprocessed or rendered to HTML:
///
/// ````markdown
/// <figure class="listing">
/// <span class="file-name">Filename: src/main.rs</span>
///
/// ```rust
/// fn main() {}
/// ```
///
/// <figcaption>Listing 1-2: Some <em>text</em>, yeah?</figcaption>
///
/// </figure>
/// ````
///
/// When `output-mode = "simple"` in the configuration, it instead emits:
///
/// ````markdown
/// Filename: src/main.rs
///
/// ```rust
/// fn main() {}
/// ```
///
/// Listing 1-2: Some *text*, yeah?
/// ````
pub struct TrplListing;

impl Preprocessor for TrplListing {
    fn name(&self) -> &str {
        "trpl-listing"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let config = ctx
            .config
            .get_preprocessor(self.name())
            .ok_or(Error::NoConfig)?;

        let key = String::from("output-mode");
        let mode = config
            .get(&key)
            .map(|value| match value.as_str() {
                Some(s) => Mode::try_from(s).map_err(|_| Error::BadValue {
                    key,
                    value: value.to_string(),
                }),
                None => Err(Error::BadValue {
                    key,
                    value: value.to_string(),
                }),
            })
            .transpose()?
            .unwrap_or(Mode::Default);

        let mut errors: Vec<String> = vec![];
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                match rewrite_listing(&chapter.content, mode) {
                    Ok(rewritten) => chapter.content = rewritten,
                    Err(reason) => errors.push(reason),
                }
            }
        });

        if errors.is_empty() {
            Ok(book)
        } else {
            Err(CompositeError(errors.join("\n")).into())
        }
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "markdown"
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("No config for trpl-listing")]
    NoConfig,

    #[error("Bad config value '{value}' for key '{key}'")]
    BadValue { key: String, value: String },
}

#[derive(Debug, thiserror::Error)]
#[error("Error(s) rewriting input: {0}")]
struct CompositeError(String);

#[derive(Debug, Clone, Copy)]
enum Mode {
    Default,
    Simple,
}

/// Trivial marker struct to indicate an internal error.
///
/// The caller has enough info to do what it needs without passing data around.
struct ParseErr;

impl TryFrom<&str> for Mode {
    type Error = ParseErr;

    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            "default" => Ok(Mode::Default),
            "simple" => Ok(Mode::Simple),
            _ => Err(ParseErr),
        }
    }
}

fn rewrite_listing(src: &str, mode: Mode) -> Result<String, String> {
    let parser = new_cmark_parser(src, true);

    struct State<'e> {
        current_listing: Option<Listing>,
        events: Vec<Result<Event<'e>, String>>,
    }

    let final_state = parser.fold(
        State {
            current_listing: None,
            events: vec![],
        },
        |mut state, ev| {
            match ev {
                Event::Html(tag) => {
                    if tag.starts_with("<Listing") {
                        let listing_result = Tokenizer::from(tag.as_ref())
                            .flatten()
                            .fold(ListingBuilder::new(), |builder, token| {
                                match token {
                                    Token::Attribute {
                                        local, value, ..
                                    } => {
                                        match local.as_str() {
                                            "number" => builder
                                                .with_number(value.as_str()),
                                            "caption" => builder
                                                .with_caption(value.as_str()),
                                            "file-name" => builder
                                                .with_file_name(value.as_str()),
                                            _ => builder, // TODO: error on extra attrs?
                                        }
                                    }
                                    _ => builder,
                                }
                            })
                            .build();

                        match listing_result {
                            Ok(listing) => {
                                let opening_event = match mode {
                                    Mode::Default => {
                                        let opening_html =
                                            listing.opening_html();
                                        Event::Html(opening_html.into())
                                    }
                                    Mode::Simple => {
                                        let opening_text =
                                            listing.opening_text();
                                        Event::Text(opening_text.into())
                                    }
                                };

                                state.current_listing = Some(listing);
                                state.events.push(Ok(opening_event));
                            }
                            Err(reason) => state.events.push(Err(reason)),
                        }
                    } else if tag.starts_with("</Listing>") {
                        let trailing = if !tag.ends_with('>') {
                            tag.replace("</Listing>", "")
                        } else {
                            String::from("")
                        };

                        match state.current_listing {
                            Some(listing) => {
                                let closing_event = match mode {
                                    Mode::Default => {
                                        let closing_html =
                                            listing.closing_html(&trailing);
                                        Event::Html(closing_html.into())
                                    }
                                    Mode::Simple => {
                                        let closing_text =
                                            listing.closing_text(&trailing);
                                        Event::Text(closing_text.into())
                                    }
                                };

                                state.current_listing = None;
                                state.events.push(Ok(closing_event));
                            }
                            None => state.events.push(Err(String::from(
                                "Closing `</Listing>` without opening tag.",
                            ))),
                        }
                    } else {
                        state.events.push(Ok(Event::Html(tag)));
                    }
                }
                ev => state.events.push(Ok(ev)),
            };
            state
        },
    );

    if final_state.current_listing.is_some() {
        return Err("Unclosed listing".into());
    }

    let (events, errors): (Vec<_>, Vec<_>) =
        final_state.events.into_iter().partition(|e| e.is_ok());

    if !errors.is_empty() {
        return Err(errors
            .into_iter()
            .map(|e| e.unwrap_err())
            .collect::<Vec<String>>()
            .join("\n"));
    }

    let mut buf = String::with_capacity(src.len() * 2);
    cmark(events.into_iter().map(|ok| ok.unwrap()), &mut buf)
        .map_err(|e| format!("{e}"))?;
    Ok(buf)
}

#[derive(Debug)]
struct Listing {
    number: String,
    caption: String,
    file_name: Option<String>,
}

impl Listing {
    fn opening_html(&self) -> String {
        let figure = String::from("<figure class=\"listing\">\n");

        match self.file_name.as_ref() {
            Some(file_name) => format!(
                "{figure}<span class=\"file-name\">Filename: {file_name}</span>\n",
            ),
            None => figure,
        }
    }

    fn closing_html(&self, trailing: &str) -> String {
        format!(
            r#"<figcaption>Listing {number}: {caption}</figcaption>
</figure>{trailing}"#,
            number = self.number,
            caption = self.caption
        )
    }

    fn opening_text(&self) -> String {
        self.file_name
            .as_ref()
            .map(|file_name| format!("\nFilename: {file_name}\n"))
            .unwrap_or_default()
    }

    fn closing_text(&self, trailing: &str) -> String {
        format!(
            "Listing {number}: {caption}{trailing}",
            number = self.number,
            caption = self.caption,
        )
    }
}

struct ListingBuilder<'a> {
    number: Option<&'a str>,
    caption: Option<&'a str>,
    file_name: Option<&'a str>,
}

impl<'a> ListingBuilder<'a> {
    fn new() -> ListingBuilder<'a> {
        ListingBuilder {
            number: None,
            caption: None,
            file_name: None,
        }
    }

    fn with_number(mut self, value: &'a str) -> Self {
        self.number = Some(value);
        self
    }

    fn with_caption(mut self, value: &'a str) -> Self {
        self.caption = Some(value);
        self
    }

    fn with_file_name(mut self, value: &'a str) -> Self {
        self.file_name = Some(value);
        self
    }

    fn build(self) -> Result<Listing, String> {
        let number = self
            .number
            .ok_or_else(|| String::from("Missing number"))?
            .to_owned();

        let caption = self
            .caption
            .map(|caption_source| {
                let events = new_cmark_parser(caption_source, true);
                let mut buf = String::with_capacity(caption_source.len() * 2);
                html::push_html(&mut buf, events);

                // This is not particularly principled, but since the only
                // place it is used is here, for caption source handling, it
                // is “fine”.
                buf.replace("<p>", "").replace("</p>", "").replace('\n', "")
            })
            .ok_or_else(|| String::from("Missing caption"))?
            .to_owned();

        Ok(Listing {
            number,
            caption,
            file_name: self.file_name.map(String::from),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Note: This inserts an additional backtick around the re-emitted code.
    /// It is not clear *why*, but that seems to be an artifact of the rendering
    /// done by the `pulldown_cmark_to_cmark` crate.
    #[test]
    fn default_mode_works() {
        let result = rewrite_listing(
            r#"<Listing number="1-2" caption="A write-up which *might* include inline Markdown like `code` etc." file-name="src/main.rs">

```rust
fn main() {}
```

</Listing>"#,
            Mode::Default,
        );

        assert_eq!(
            &result.unwrap(),
            r#"<figure class="listing">
<span class="file-name">Filename: src/main.rs</span>

````rust
fn main() {}
````

<figcaption>Listing 1-2: A write-up which <em>might</em> include inline Markdown like <code>code</code> etc.</figcaption>
</figure>"#
        );
    }

    #[test]
    fn simple_mode_works() {
        let result = rewrite_listing(
            r#"<Listing number="1-2" caption="A write-up which *might* include inline Markdown like `code` etc." file-name="src/main.rs">

```rust
fn main() {}
```

</Listing>"#,
            Mode::Simple,
        );

        assert_eq!(
            &result.unwrap(),
            r#"
Filename: src/main.rs

````rust
fn main() {}
````

Listing 1-2: A write-up which <em>might</em> include inline Markdown like <code>code</code> etc."#
        );
    }

    #[test]
    fn actual_listing() {
        let result = rewrite_listing(
            r#"Now open the *main.rs* file you just created and enter the code in Listing 1-1.

<Listing number="1-1" file-name="main.rs" caption="A program that prints `Hello, world!`">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

Save the file and go back to your terminal window"#,
            Mode::Default,
        );

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"Now open the *main.rs* file you just created and enter the code in Listing 1-1.

<figure class="listing">
<span class="file-name">Filename: main.rs</span>

````rust
fn main() {
    println!("Hello, world!");
}
````

<figcaption>Listing 1-1: A program that prints <code>Hello, world!</code></figcaption>
</figure>

Save the file and go back to your terminal window"#
        );
    }

    #[test]
    fn no_filename() {
        let result = rewrite_listing(
            r#"This is the opening.

<Listing number="1-1" caption="This is the caption">

```rust
fn main() {}
```

</Listing>

This is the closing."#,
            Mode::Default,
        );

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"This is the opening.

<figure class="listing">

````rust
fn main() {}
````

<figcaption>Listing 1-1: This is the caption</figcaption>
</figure>

This is the closing."#
        );
    }

    /// Check that the config options are correctly handled.
    ///
    /// Note: none of these tests particularly exercise the *wiring*. They just
    /// assume that the config itself is done correctly. This is a small enough
    /// chunk of code that it easy to verify by hand at present. If it becomes
    /// more complex in the future, it would be good to revisit and integrate
    /// the same kinds of tests as the unit tests above here.
    #[cfg(test)]
    mod config {
        use super::*;

        // TODO: what *should* the behavior here be? I *think* it should error,
        // in that there is a problem if it is invoked without that info.
        #[test]
        fn no_config() {
            let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {}
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
            let input_json = input_json.as_bytes();
            let (ctx, book) =
                mdbook::preprocess::CmdPreprocessor::parse_input(input_json)
                    .unwrap();
            let result = TrplListing.run(&ctx, book);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(format!("{err}"), "No config for trpl-listing");
        }

        #[test]
        fn empty_config() {
            let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "trpl-listing": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
            let input_json = input_json.as_bytes();
            let (ctx, book) =
                mdbook::preprocess::CmdPreprocessor::parse_input(input_json)
                    .unwrap();
            let result = TrplListing.run(&ctx, book);
            assert!(result.is_ok());
        }

        #[test]
        fn specify_default() {
            let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "trpl-listing": {
                                "output-mode": "default"
                            }
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
            let input_json = input_json.as_bytes();
            let (ctx, book) =
                mdbook::preprocess::CmdPreprocessor::parse_input(input_json)
                    .unwrap();
            let result = TrplListing.run(&ctx, book);
            assert!(result.is_ok());
        }

        #[test]
        fn specify_simple() {
            let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "trpl-listing": {
                                "output-mode": "simple"
                            }
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
            let input_json = input_json.as_bytes();
            let (ctx, book) =
                mdbook::preprocess::CmdPreprocessor::parse_input(input_json)
                    .unwrap();
            let result = TrplListing.run(&ctx, book);
            assert!(result.is_ok());
        }

        #[test]
        fn specify_invalid() {
            let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "trpl-listing": {
                                "output-mode": "nonsense"
                            }
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
            let input_json = input_json.as_bytes();
            let (ctx, book) =
                mdbook::preprocess::CmdPreprocessor::parse_input(input_json)
                    .unwrap();
            let result = TrplListing.run(&ctx, book);
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(
                format!("{err}"),
                "Bad config value '\"nonsense\"' for key 'output-mode'"
            );
        }
    }
}
