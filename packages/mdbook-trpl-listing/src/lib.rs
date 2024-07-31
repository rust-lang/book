use html_parser::Dom;
use mdbook::{
    book::Book,
    errors::Result,
    preprocess::{Preprocessor, PreprocessorContext},
    utils::new_cmark_parser,
    BookItem,
};
use pulldown_cmark::{html, Event};
use pulldown_cmark_to_cmark::cmark;

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
        renderer == "html" || renderer == "markdown" || renderer == "test"
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
    let final_state = new_cmark_parser(src, true).try_fold(
        ListingState {
            current: None,
            events: vec![],
        },
        |mut state, ev| {
            match ev {
                Event::Html(tag) => {
                    if tag.starts_with("<Listing") {
                        state.open_listing(tag, mode)?;
                    } else if tag.starts_with("</Listing>") {
                        state.close_listing(tag, mode);
                    } else {
                        state.events.push(Ok(Event::Html(tag)));
                    }
                }
                ev => state.events.push(Ok(ev)),
            };
            Ok::<ListingState<'_>, String>(state)
        },
    )?;

    if final_state.current.is_some() {
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

struct ListingState<'e> {
    current: Option<Listing>,
    events: Vec<Result<Event<'e>, String>>,
}

impl<'e> ListingState<'e> {
    fn open_listing(
        &mut self,
        tag: pulldown_cmark::CowStr<'_>,
        mode: Mode,
    ) -> Result<(), String> {
        // We do not *keep* the version constructed here, just temporarily
        // construct it so the HTML parser, which expects properly closed tags
        // to parse it as a *tag* rather than a *weird text node*, which accept
        // it and provide a useful view of it.
        let to_parse = tag.to_owned().to_string() + "</Listing>";
        let listing = Dom::parse(&to_parse)
            .map_err(|e| e.to_string())?
            .children
            .into_iter()
            .filter_map(|node| match node {
                html_parser::Node::Element(element) => Some(element.attributes),
                html_parser::Node::Text(_) | html_parser::Node::Comment(_) => {
                    None
                }
            })
            .flatten()
            .try_fold(ListingBuilder::new(), |builder, (key, maybe_value)| {
                match (key.as_str(), maybe_value) {
                    ("number", Some(value)) => Ok(builder.with_number(value)),
                    ("number", None) => {
                        Err(String::from("number attribute without value"))
                    }
                    ("caption", Some(value)) => Ok(builder.with_caption(value)),
                    ("caption", None) => {
                        Err(String::from("caption attribute without value"))
                    }
                    ("file-name", Some(value)) => {
                        Ok(builder.with_file_name(value))
                    }
                    ("file-name", None) => {
                        Err(String::from("file-name attribute without value"))
                    }

                    _ => Ok(builder), // TODO: error on extra attrs?
                }
            })?
            .build();

        let opening_event = match mode {
            Mode::Default => {
                let opening_html = listing.opening_html();
                Event::Html(opening_html.into())
            }
            Mode::Simple => {
                let opening_text = listing.opening_text();
                Event::Text(opening_text.into())
            }
        };

        self.current = Some(listing);
        self.events.push(Ok(opening_event));
        Ok(())
    }

    fn close_listing(&mut self, tag: pulldown_cmark::CowStr<'_>, mode: Mode) {
        let trailing = if !tag.ends_with('>') {
            tag.replace("</Listing>", "")
        } else {
            String::from("")
        };

        match &self.current {
            Some(listing) => {
                let closing_event = match mode {
                    Mode::Default => {
                        let closing_html = listing.closing_html(&trailing);
                        Event::Html(closing_html.into())
                    }
                    Mode::Simple => {
                        let closing_text = listing.closing_text(&trailing);
                        Event::Text(closing_text.into())
                    }
                };

                self.current = None;
                self.events.push(Ok(closing_event));
            }
            None => {
                self.events.push(Err(String::from(
                    "Closing `</Listing>` without opening tag.",
                )));
            }
        }
    }
}

#[derive(Debug)]
struct Listing {
    number: Option<String>,
    caption: Option<String>,
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
        match (&self.number, &self.caption) {
            (Some(number), Some(caption)) => format!(
                r#"<figcaption>Listing {number}: {caption}</figcaption>
</figure>{trailing}"#
            ),
            (None, Some(caption)) => format!(
                r#"<figcaption>{caption}</figcaption>
</figure>{trailing}"#
            ),
            (Some(number), None) => format!(
                r#"<figcaption>Listing {number}</figcaption>
</figure>{trailing}"#
            ),
            (None, None) => format!("</figure>{trailing}"),
        }
    }

    fn opening_text(&self) -> String {
        self.file_name
            .as_ref()
            .map(|file_name| format!("\nFilename: {file_name}\n"))
            .unwrap_or_default()
    }

    fn closing_text(&self, trailing: &str) -> String {
        match (&self.number, &self.caption) {
            (Some(number), Some(caption)) => {
                format!("Listing {number}: {caption}{trailing}")
            }
            (None, Some(caption)) => format!("{caption}{trailing}"),
            (Some(number), None) => format!("Listing {number}{trailing}"),
            (None, None) => trailing.into(),
        }
    }
}

struct ListingBuilder {
    number: Option<String>,
    caption: Option<String>,
    file_name: Option<String>,
}

impl ListingBuilder {
    fn new() -> ListingBuilder {
        ListingBuilder {
            number: None,
            caption: None,
            file_name: None,
        }
    }

    fn with_number(mut self, value: String) -> Self {
        self.number = Some(value);
        self
    }

    fn with_caption(mut self, value: String) -> Self {
        self.caption = Some(value);
        self
    }

    fn with_file_name(mut self, value: String) -> Self {
        self.file_name = Some(value);
        self
    }

    fn build(self) -> Listing {
        let caption = self.caption.map(|caption_source| {
            let events = new_cmark_parser(&caption_source, true);
            let mut buf = String::with_capacity(caption_source.len() * 2);
            html::push_html(&mut buf, events);

            // This is not particularly principled, but since the only
            // place it is used is here, for caption source handling, it
            // is “fine”.
            buf.replace("<p>", "").replace("</p>", "").replace('\n', "")
        });

        Listing {
            number: self.number.map(String::from),
            caption,
            file_name: self.file_name.map(String::from),
        }
    }
}

#[cfg(test)]
mod tests;
