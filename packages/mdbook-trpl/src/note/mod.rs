use mdbook::{
    book::Book,
    errors::Result,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use pulldown_cmark::{
    Event::{self, *},
    Tag, TagEnd,
};
use pulldown_cmark_to_cmark::cmark;

/// A simple preprocessor for semantic notes in _The Rust Programming Language_.
///
/// Takes in Markdown like this:
///
/// ```markdown
/// > Note: This is a note.
/// ```
///
/// Spits out Markdown like this:
///
/// ```markdown
/// <section class="note" aria-role="note">
///
/// This is a note.
///
/// </section>
/// ```
pub struct TrplNote;

impl Preprocessor for TrplNote {
    fn name(&self) -> &str {
        "simple-note-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                chapter.content = rewrite(&chapter.content);
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "markdown" || renderer == "test"
    }
}

pub fn rewrite(text: &str) -> String {
    let parser = crate::parser(text);

    let mut events = Vec::new();
    let mut state = Default;

    for event in parser {
        match (&mut state, event) {
            (Default, Start(Tag::BlockQuote(_))) => {
                state = StartingBlockquote(vec![Start(Tag::BlockQuote(None))]);
            }

            (StartingBlockquote(blockquote_events), Text(content)) => {
                if content.starts_with("Note: ") {
                    // This needs the "extra" `SoftBreak`s so that when the final rendering pass
                    // happens, it does not end up treating the internal content as inline *or*
                    // treating the HTML tags as inline tags:
                    //
                    // - Content inside HTML blocks is only rendered as Markdown when it is
                    //   separated from the block HTML elements: otherwise it gets treated as inline
                    //   HTML and *not* rendered.
                    // - Along the same lines, an HTML tag that happens to be directly adjacent to
                    //   the end of a previous Markdown block will end up being rendered as part of
                    //   that block.
                    events.extend([
                        SoftBreak,
                        SoftBreak,
                        Html(
                            r#"<section class="note" aria-role="note">"#.into(),
                        ),
                        SoftBreak,
                        SoftBreak,
                        Start(Tag::Paragraph),
                        Text(content),
                    ]);
                    state = InNote;
                } else {
                    events.append(blockquote_events);
                    events.push(Text(content));
                    state = Default;
                }
            }

            (
                StartingBlockquote(_blockquote_events),
                heading @ Start(Tag::Heading { .. }),
            ) => {
                events.extend([
                    SoftBreak,
                    SoftBreak,
                    Html(r#"<section class="note" aria-role="note">"#.into()),
                    SoftBreak,
                    SoftBreak,
                    heading,
                ]);
                state = InNote;
            }

            (StartingBlockquote(ref mut events), Start(tag)) => {
                events.push(Start(tag));
            }

            (InNote, End(TagEnd::BlockQuote(_))) => {
                // As with the start of the block HTML, the closing HTML must be
                // separated from the Markdown text by two newlines.
                events.extend([
                    SoftBreak,
                    SoftBreak,
                    Html("</section>".into()),
                ]);
                state = Default;
            }

            (_, event) => {
                events.push(event);
            }
        }
    }

    let mut buf = String::new();
    cmark(events.into_iter(), &mut buf).unwrap();
    buf
}

use State::*;

#[derive(Debug)]
enum State<'e> {
    Default,
    StartingBlockquote(Vec<Event<'e>>),
    InNote,
}

#[cfg(test)]
mod tests;
