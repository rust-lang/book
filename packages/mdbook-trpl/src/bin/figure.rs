use std::io;

use clap::{self, Parser, Subcommand};

use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_trpl::Figure;

fn main() -> Result<(), String> {
    match Cli::parse().command {
        Some(Command::Supports { renderer }) => {
            if Figure.supports_renderer(&renderer) {
                Ok(())
            } else {
                Err(format!("Renderer '{renderer}' is unsupported"))
            }
        }
        None => {
            let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())
                .map_err(|e| format!("{e}"))?;
            let processed =
                Figure.run(&ctx, book).map_err(|e| format!("{e}"))?;
            serde_json::to_writer(io::stdout(), &processed)
                .map_err(|e| format!("{e}"))
        }
    }
}

/// A simple preprocessor for handling figures with images in _The Rust
/// Programming Language_ book.
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Is the renderer supported?
    ///
    /// Supported renderers are `'html'`, `'markdown'`, and `'test'`.
    Supports { renderer: String },
}
