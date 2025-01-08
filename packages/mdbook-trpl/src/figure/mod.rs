use anyhow::{anyhow, Result};
use html_parser::{Dom, Node};
use mdbook::{book::Book, preprocess::Preprocessor, BookItem};

use pulldown_cmark::Event;
use pulldown_cmark_to_cmark::cmark;

use crate::{config::Mode, CompositeError};

/// A simple preprocessor to rewrite `<figure>`s with `<img>`s.
///
/// This is a no-op by default; it only operates on the book chapters when the
/// `[preprocessor.trpl-figure]` has `output-mode = "simple"`.
///
/// Takes in Markdown containing like this:
///
/// ```markdown
/// <figure>
///
/// <img src="http://www.example.com/some-cool-image.jpg">
///
/// <figcaption>Figure 1-2: A description of the image</figcaption>
///
/// </figure>
/// ```
///
/// Spits out Markdown like this:
///
/// ```markdown
///
/// <img src="http://www.example.com/some-cool-image.jpg">
///
/// Figure 1-2: A description of the image
///
/// ```
pub struct TrplFigure;

impl TrplFigure {
    pub fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "markdown" || renderer == "test"
    }
}

impl Preprocessor for TrplFigure {
    fn name(&self) -> &str {
        "trpl-figure"
    }

    fn run(
        &self,
        ctx: &mdbook::preprocess::PreprocessorContext,
        mut book: Book,
    ) -> Result<Book> {
        // The `<figure>`-based output is only replaced in the `Simple` mode.
        let Mode::Simple = Mode::from_context(ctx, self.name())? else {
            return Ok(book);
        };

        let mut errors = vec![];
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                match rewrite_figure(&chapter.content) {
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
}

const OPEN_FIGURE: &'static str = "<figure>";
const CLOSE_FIGURE: &'static str = "</figure>";

const OPEN_CAPTION: &'static str = "<figcaption>";
const CLOSE_CAPTION: &'static str = "</figcaption>";

fn rewrite_figure(text: &str) -> Result<String> {
    let final_state = crate::parser(text).try_fold(
        State {
            current: None,
            events: Vec::new(),
        },
        |mut state, event| {
            match (event, &mut state.current) {
                // -- Open figure
                (Event::Html(tag), None) if tag.starts_with(OPEN_FIGURE) => {
                    let mut figure = Figure::new();
                    figure.events.push(Event::Text("\n".into()));
                    state.current.replace(figure);
                }

                (Event::Html(tag), Some(_)) if tag.starts_with(OPEN_FIGURE) => {
                    return Err(anyhow!(
                        "Opening `<figure>` when already in a `<figure>`"
                    ))
                }

                // -- Close figure
                (Event::Html(tag), Some(figure))
                    if tag.starts_with(CLOSE_FIGURE) =>
                {
                    if figure.in_caption {
                        return Err(anyhow!("Unclosed `<figcaption>`"));
                    }

                    state.events.append(&mut figure.events);
                    state.events.push(Event::Text("\n".into()));
                    let _ = state.current.take();
                }

                (Event::Html(tag), None) if tag.trim() == CLOSE_FIGURE => {
                    return Err(anyhow!(bad_close(CLOSE_FIGURE, OPEN_CAPTION)));
                }

                // -- Start captions
                // We do not allow nested captions, but if we have not yet
                // started a caption, it is legal to start one, and we
                // intentionally ignore that event entirely other than tracking
                // that we have started a caption. We will push the body of the
                // caption into the figure’s events when we hit them.
                //
                // Note: this does not support `<figcaption class="...">`.
                (Event::Html(tag), Some(fig))
                    if tag.starts_with(OPEN_CAPTION) =>
                {
                    if fig.in_caption {
                        return Err(anyhow!(bad_open(OPEN_CAPTION)));
                    } else {
                        if tag.trim().ends_with(CLOSE_CAPTION) {
                            let text = Dom::parse(tag.as_ref())?
                                .children
                                .into_iter()
                                .filter_map(text_of)
                                .collect::<String>();

                            if text.is_empty() {
                                return Err(anyhow!(
                                    "Missing caption in `<figcaption>`"
                                ));
                            }

                            fig.events.push(Event::Text(text.into()));
                        } else {
                            fig.events.push(Event::Text("\n".into()));
                            fig.in_caption = true;
                        }
                    }
                }

                (Event::Html(tag), None) if tag.starts_with(OPEN_CAPTION) => {
                    return Err(anyhow!(bad_open(OPEN_CAPTION)))
                }

                // -- Close captions
                (Event::Html(tag), Some(fig))
                    if tag.trim() == CLOSE_CAPTION =>
                {
                    if fig.in_caption {
                        fig.events.push(Event::Text("\n".into()));
                        fig.in_caption = false;
                    } else {
                        return Err(anyhow!(bad_close(
                            CLOSE_CAPTION,
                            OPEN_CAPTION
                        )));
                    }
                }

                (Event::Html(tag), None) if tag.trim() == CLOSE_CAPTION => {
                    return Err(anyhow!(bad_close(CLOSE_CAPTION, OPEN_FIGURE)));
                }

                // Otherwise, if in the body of a figure, push whatever other
                // events without modification into the figure state.
                (ev, Some(ref mut figure)) => figure.events.push(ev),

                // And if not in a figure, no modifications whatsoever.
                (ev, None) => state.events.push(ev),
            }
            Ok(state)
        },
    )?;

    if final_state.current.is_some() {
        return Err(anyhow!("Unclosed `<figure>`"));
    }

    let mut rewritten = String::new();
    cmark(final_state.events.into_iter(), &mut rewritten)?;
    Ok(rewritten)
}

fn text_of(node: Node) -> Option<String> {
    match node {
        Node::Text(text) => Some(text),
        Node::Element(element) => {
            Some(element.children.into_iter().filter_map(text_of).collect())
        }
        Node::Comment(_) => None,
    }
}

fn bad_open(tag: &str) -> String {
    format!("Opening `<{tag}>` while not in a `<figure>`.")
}

fn bad_close(close: &str, required_open: &str) -> String {
    format!("Closing `<{close}>` while not in a `<{required_open}>`.")
}

#[derive(Debug)]
struct State<'e> {
    current: Option<Figure<'e>>,
    events: Vec<Event<'e>>,
}

#[derive(Debug)]
struct Figure<'e> {
    events: Vec<Event<'e>>,
    in_caption: bool,
}

impl<'e> Figure<'e> {
    fn new() -> Figure<'e> {
        Figure {
            events: vec![],
            in_caption: false,
        }
    }
}

#[cfg(test)]
mod tests;
