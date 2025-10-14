mod config;
mod figure;
mod heading;
mod listing;
mod note;

pub use config::Mode;
pub use figure::TrplFigure as Figure;
pub use heading::TrplHeading as Heading;
pub use listing::TrplListing as Listing;
pub use note::TrplNote as Note;
use pulldown_cmark::{Options, Parser};

/// Convenience function to get a parser matching `mdbook::new_cmark_parser`.
///
/// This is implemented separately so we are decoupled from mdbook's dependency
/// versions and can update at will (albeit with care to stay aligned with what
/// mdbook does!) to later versions of `pulldown-cmark` and related tools.
///
/// Notes:
///
/// - `mdbook::new_cmark_parser` has an additional parameter which allows smart
///   punctuation to be enabled or disabled; we always enable it.
/// - We do not use footnotes in the text at present, but this goes out of its
///   way to match this up to the old footnotes behavior just to make sure the
///   parsing etc. is all the same.
pub fn parser(text: &str) -> Parser<'_> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_OLD_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    Parser::new_ext(text, opts)
}

#[derive(Debug, thiserror::Error)]
struct CompositeError(Vec<anyhow::Error>);

impl std::fmt::Display for CompositeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error(s) rewriting input: {}",
            self.0.iter().map(|e| format!("{e:?}")).collect::<String>()
        )
    }
}
