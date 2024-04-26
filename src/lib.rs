use mdbook::{
    book::Book,
    errors::Result,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use pulldown_cmark::{
    Event::{self, *},
    Parser, Tag, TagEnd,
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
/// <section class="note" aria-label="Note" aria-role="note">
///
/// This is a note.
///
/// </section>
/// ```
pub struct SimpleNote;

impl Preprocessor for SimpleNote {
    fn name(&self) -> &str {
        "simple-note-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<mdbook::book::Book> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                chapter.content = rewrite(&chapter.content);
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

pub fn rewrite(text: &str) -> String {
    let parser = Parser::new(text);

    let mut events = Vec::new();
    let mut state = Default;

    for event in parser {
        match (event, &mut state) {
            (Start(Tag::BlockQuote), Default) => {
                state = StartingBlockquote(vec![Start(Tag::BlockQuote)]);
            }

            (Text(content), StartingBlockquote(blockquote_events)) => {
                if content.starts_with("Note: ") {
                    // This needs the “extra” `SoftBreak`s so that when the
                    // final rendering pass happens, it does not end up treating
                    // the internal content as inline: content inside HTML
                    // blocks is only rendered as Markdown when it is separated
                    // from the block HTML elements: otherwise it gets treated
                    // as inline HTML and *not* rendered.
                    events.extend([
                        Html(r#"<section class="note" aria-label="Note" aria-role="note">"#.into()),
                        SoftBreak,
                        SoftBreak,
                        Start(Tag::Paragraph),
                        Text(content.replace("Note: ", "").into()),
                    ]);
                    state = InNote;
                } else {
                    events.append(blockquote_events);
                    events.push(Text(content));
                }
            }

            (heading @ Start(Tag::Heading { .. }), StartingBlockquote(_blockquote_events)) => {
                events.extend([
                    Html(r#"<section class="note" aria-label="Note" aria-role="note">"#.into()),
                    SoftBreak,
                    SoftBreak,
                    heading,
                ]);
                state = InNote;
            }

            (Start(Tag::Paragraph), StartingBlockquote(ref mut events)) => {
                events.push(Start(Tag::Paragraph));
            }

            (End(TagEnd::BlockQuote), InNote) => {
                // As with the start of the block HTML, the closing HTML must be
                // separated from the Markdown text by two newlines.
                events.extend([SoftBreak, SoftBreak, Html("</section>".into())]);
                state = Default;
            }

            (event, _) => {
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
mod tests {
    use super::*;

    #[test]
    fn no_note() {
        let text = "Hello, world.\n\nThis is some text.";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<p>Hello, world.</p>\n<p>This is some text.</p>\n"
        );
    }

    #[test]
    fn with_note() {
        let text = "> Note: This is some text.\n> It keeps going.";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<p>This is some text.\nIt keeps going.</p>\n</section>"
        );
    }

    #[test]
    fn regular_blockquote() {
        let text = "> This is some text.\n> It keeps going.";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<blockquote>\n<p>This is some text.\nIt keeps going.</p>\n</blockquote>\n"
        );
    }

    #[test]
    fn combined() {
        let text = "> Note: This is some text.\n> It keeps going.\n\nThis is regular text.\n\n> This is a blockquote.\n";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<p>This is some text.\nIt keeps going.</p>\n</section>\n<p>This is regular text.</p>\n<blockquote>\n<p>This is a blockquote.</p>\n</blockquote>\n"
        );
    }

    #[test]
    fn with_h1() {
        let text = "> # Header\n > And then some note content.";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<h1>Header</h1>\n<p>And then some note content.</p>\n</section>"
        );
    }

    #[test]
    fn with_h2() {
        let text = "> ## Header\n > And then some note content.";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<h2>Header</h2>\n<p>And then some note content.</p>\n</section>"
        );
    }

    #[test]
    fn with_h3() {
        let text = "> ### Header\n > And then some note content.";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<h3>Header</h3>\n<p>And then some note content.</p>\n</section>"
        );
    }

    #[test]
    fn with_h4() {
        let text = "> #### Header\n > And then some note content.";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<h4>Header</h4>\n<p>And then some note content.</p>\n</section>"
        );
    }

    #[test]
    fn with_h5() {
        let text = "> ##### Header\n > And then some note content.";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<h5>Header</h5>\n<p>And then some note content.</p>\n</section>"
        );
    }

    #[test]
    fn with_h6() {
        let text = "> ###### Header\n > And then some note content.";
        let processed = rewrite(text);
        assert_eq!(
            render_markdown(&processed),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<h6>Header</h6>\n<p>And then some note content.</p>\n</section>"
        );
    }

    fn render_markdown(text: &str) -> String {
        let parser = Parser::new(text);
        let mut buf = String::new();
        pulldown_cmark::html::push_html(&mut buf, parser);
        buf
    }
}
