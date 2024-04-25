use pulldown_cmark::{
    html::push_html,
    Event::{self, *},
    Parser, Tag, TagEnd,
};

fn main() {
    todo!("Now this as an mdbook preprocessor!");
}

fn rewrite(text: &str) -> String {
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
                    let note_start = r#"<section class="note" aria-label="Note" aria-role="note">"#;
                    events.push(Html(note_start.into()));
                    events.push(Start(Tag::Paragraph));
                    events.push(Text(content.replace("Note: ", "").into()));
                    state = InNote;
                } else {
                    events.append(blockquote_events);
                    events.push(Text(content));
                }
            }

            (Start(Tag::Paragraph), StartingBlockquote(ref mut events)) => {
                events.push(Start(Tag::Paragraph));
            }

            (End(TagEnd::BlockQuote), InNote) => {
                events.push(Html("</section>".into()));
                state = Default;
            }

            (event, _) => {
                events.push(event);
            }
        }
    }

    let mut buf = String::new();
    push_html(&mut buf, events.into_iter());
    buf
}

use State::*;

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
            processed.as_str(),
            "<p>Hello, world.</p>\n<p>This is some text.</p>\n"
        );
    }

    #[test]
    fn with_note() {
        let text = "> Note: This is some text.\n> It keeps going.";
        let processed = rewrite(text);
        assert_eq!(
            processed.as_str(),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<p>This is some text.\nIt keeps going.</p>\n</section>"
        );
    }

    #[test]
    fn regular_blockquote() {
        let text = "> This is some text.\n> It keeps going.";
        let processed = rewrite(text);
        assert_eq!(
            processed.as_str(),
            "<blockquote>\n<p>This is some text.\nIt keeps going.</p>\n</blockquote>\n"
        );
    }

    #[test]
    fn combined() {
        let text = "> Note: This is some text.\n> It keeps going.\n\nThis is regular text.\n\n> This is a blockquote.\n";
        let processed = rewrite(text);
        assert_eq!(
            processed.as_str(),
            "<section class=\"note\" aria-label=\"Note\" aria-role=\"note\">\n<p>This is some text.\nIt keeps going.</p>\n</section>\n<p>This is regular text.</p>\n<blockquote>\n<p>This is a blockquote.</p>\n</blockquote>\n"
        );
    }
}
