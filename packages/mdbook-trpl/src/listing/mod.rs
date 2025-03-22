use anyhow::anyhow;
use html_parser::Dom;
use mdbook::{
    book::Book,
    errors::Result,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use pulldown_cmark::{html, Event};
use pulldown_cmark_to_cmark::cmark;

use crate::{config::Mode, CompositeError};

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
        let mode = Mode::from_context(ctx, self.name())?;

        let mut errors = vec![];
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                match rewrite_listing(&chapter.content, mode) {
                    Ok(rewritten) => chapter.content = rewritten,
                    Err(reason) => errors.push(anyhow!(reason)),
                }
            }
        });

        if errors.is_empty() {
            Ok(book)
        } else {
            Err(CompositeError(errors).into())
        }
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "markdown" || renderer == "test"
    }
}

fn rewrite_listing(src: &str, mode: Mode) -> Result<String, String> {
    match mode {
        Mode::Default => {
            let final_state = crate::parser(src).try_fold(
                RewriteState {
                    current: None,
                    events: vec![],
                },
                |mut state, ev| -> Result<RewriteState, String> {
                    match ev {
                        Event::Html(tag) => {
                            if tag.starts_with("<Listing") {
                                state.open_listing(tag, mode)?;
                            } else if tag.starts_with("</Listing>") {
                                state.close_listing(tag);
                            } else {
                                state.events.push(Ok(Event::Html(tag)));
                            }
                        }
                        ev => state.events.push(Ok(ev)),
                    };
                    Ok(state)
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
        Mode::Simple => {
            // The output text should be very slightly *shorter* than the input,
            // so we know this is a reasonable size for the buffer.
            let mut rewritten = String::with_capacity(src.len());
            let mut current_closing = None;
            for line in src.lines() {
                if line.starts_with("<Listing") && (line.ends_with(">")) {
                    let listing =
                        ListingBuilder::from_tag(&line)?.build(Mode::Simple);
                    rewritten.push_str(&listing.opening_text());
                    current_closing = Some(listing.closing_text("\n"));
                } else if line == "</Listing>" {
                    let closing =
                        current_closing.as_ref().ok_or_else(|| {
                            String::from(
                                "Closing `</Listing>` without opening tag.",
                            )
                        })?;
                    rewritten.push_str(closing);
                } else {
                    rewritten.push_str(line);
                    rewritten.push('\n');
                }
            }

            // Since we always push a `'\n'` onto the end of the new string and
            // `.lines()` does not tell us whether there *was* such a character,
            // this makes the output match the input, and thus avoids adding new
            // newlines after conversion.
            if !src.ends_with('\n') {
                rewritten.pop();
            }

            Ok(rewritten)
        }
    }
}

struct RewriteState<'e> {
    current: Option<Listing>,
    events: Vec<Result<Event<'e>, String>>,
}

impl<'e> RewriteState<'e> {
    fn open_listing(
        &mut self,
        tag: pulldown_cmark::CowStr<'_>,
        mode: Mode,
    ) -> Result<(), String> {
        let listing = ListingBuilder::from_tag(&tag)?.build(mode);
        let opening_event = Event::Html(listing.opening_html().into());

        self.current = Some(listing);
        self.events.push(Ok(opening_event));
        Ok(())
    }

    fn close_listing(&mut self, tag: pulldown_cmark::CowStr<'_>) {
        let trailing = if !tag.ends_with('>') {
            tag.replace("</Listing>", "")
        } else {
            String::from("")
        };

        match &self.current {
            Some(listing) => {
                let closing_event =
                    Event::Html(listing.closing_html(&trailing).into());

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
        let id_attribute = self
            .number
            .as_ref()
            .map(|number| format!(" id=\"listing-{number}\""))
            .unwrap_or_default();

        let figure = format!("<figure class=\"listing\"{id_attribute}>\n");

        match self.file_name.as_ref() {
            Some(file_name) => format!(
                "{figure}<span class=\"file-name\">Filename: {file_name}</span>\n",
            ),
            None => figure,
        }
    }

    fn closing_html(&self, trailing: &str) -> String {
        match (&self.number, &self.caption) {
            (Some(number), caption) => {
                let caption_text = caption
                    .as_ref()
                    .map(|caption| format!(": {}", caption))
                    .unwrap_or_default();
                let listing_a_tag = format!(
                    "<a href=\"#listing-{number}\">Listing {number}</a>"
                );
                format!(
                    r#"<figcaption>{listing_a_tag}{caption_text}</figcaption>
</figure>{trailing}"#
                )
            }
            (None, Some(caption)) => format!(
                r#"<figcaption>{caption}</figcaption>
</figure>{trailing}"#
            ),
            (None, None) => format!("</figure>{trailing}"),
        }
    }

    fn opening_text(&self) -> String {
        self.file_name
            .as_ref()
            .map(|file_name| format!("{file_name}\n"))
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

/// Note: Although this has the same structure as [`Listing`], it does not have
/// the same *semantics*. In particular, this has the *source* for the `caption`
/// while `Listing` has the *rendered* version.
struct ListingBuilder {
    number: Option<String>,
    caption: Option<String>,
    file_name: Option<String>,
}

impl ListingBuilder {
    fn from_tag(tag: &str) -> Result<ListingBuilder, String> {
        let to_parse = format!("{tag}</Listing>");
        Dom::parse(&to_parse)
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
            .try_fold(
                ListingBuilder {
                    number: None,
                    caption: None,
                    file_name: None,
                },
                |builder, (key, maybe_value)| match (key.as_str(), maybe_value)
                {
                    ("number", Some(value)) => Ok(builder.with_number(value)),

                    ("caption", Some(value)) => Ok(builder.with_caption(value)),

                    ("file-name", Some(value)) => {
                        Ok(builder.with_file_name(value))
                    }

                    (attr @ "file-name", None)
                    | (attr @ "caption", None)
                    | (attr @ "number", None) => {
                        Err(format!("Missing value for attribute: '{attr}'"))
                    }

                    (attr, _) => {
                        Err(format!("Unsupported attribute name: '{attr}'"))
                    }
                },
            )
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

    fn build(self, mode: Mode) -> Listing {
        let caption = match mode {
            Mode::Default => self.caption.map(|caption_source| {
                let events = crate::parser(&caption_source);
                let mut buf = String::with_capacity(caption_source.len() * 2);
                html::push_html(&mut buf, events);

                // This is not particularly principled, but since the only
                // place it is used is here, for caption source handling, it
                // is “fine”.
                buf.replace("<p>", "").replace("</p>", "").replace('\n', "")
            }),
            Mode::Simple => self.caption,
        };

        Listing {
            number: self.number.map(String::from),
            caption,
            file_name: self.file_name.map(String::from),
        }
    }
}

#[cfg(test)]
mod tests;
