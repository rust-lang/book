use anyhow::anyhow;
use mdbook::{
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use pulldown_cmark::{Event, Tag, TagEnd};
use pulldown_cmark_to_cmark::cmark;

use crate::{CompositeError, Mode};

pub struct TrplHeading;

impl Preprocessor for TrplHeading {
    fn name(&self) -> &str {
        "trpl-heading"
    }

    fn run(
        &self,
        ctx: &PreprocessorContext,
        mut book: Book,
    ) -> anyhow::Result<Book> {
        let mode = Mode::from_context(ctx, self.name())?;

        let mut errors = vec![];
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                match rewrite_headings(&chapter.content, mode) {
                    Ok(rewritten) => chapter.content = rewritten,
                    Err(reason) => errors.push(reason),
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

fn rewrite_headings(src: &str, mode: Mode) -> anyhow::Result<String> {
    // Don't rewrite anything for the default mode.
    if mode == Mode::Default {
        return Ok(src.into());
    }

    #[derive(Default)]
    struct State<'e> {
        in_heading: bool,
        events: Vec<Event<'e>>,
    }

    let final_state: State = crate::parser(src).try_fold(
        State::default(),
        |mut state, event| -> anyhow::Result<State> {
            if state.in_heading {
                match event {
                    // When we see the start or end of any of the inline tags
                    // (emphasis, strong emphasis, or strikethrough), or any
                    // inline HTML tags, we just skip emitting them. As dumb as
                    // that may seem, it does the job!
                    Event::Start(
                        Tag::Emphasis | Tag::Strong | Tag::Strikethrough,
                    )
                    | Event::End(
                        TagEnd::Emphasis
                        | TagEnd::Strong
                        | TagEnd::Strikethrough,
                    )
                    | Event::InlineHtml(_) => { /* skip */ }

                    // For code, we just emit the body of the inline code block,
                    // unchanged (the wrapping backticks are not present here).
                    Event::Code(code) => {
                        state.events.push(Event::Text(code));
                    }

                    // Assume headings are well-formed; you cannot have a nested
                    // headings, so we don't have to check heading level.
                    Event::End(TagEnd::Heading(_)) => {
                        state.in_heading = false;
                        state.events.push(event);
                    }
                    _ => state.events.push(event),
                }
            } else if matches!(event, Event::Start(Tag::Heading { .. })) {
                state.events.push(event);
                state.in_heading = true;
            } else {
                state.events.push(event);
            }

            Ok(state)
        },
    )?;

    if final_state.in_heading {
        return Err(anyhow!("Unclosed heading"));
    }

    let mut rewritten = String::new();
    cmark(final_state.events.into_iter(), &mut rewritten)?;
    Ok(rewritten)
}

#[cfg(test)]
mod tests;
