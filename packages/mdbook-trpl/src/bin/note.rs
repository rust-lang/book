use std::io;

use clap::{self, Parser, Subcommand};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};

use mdbook_trpl::Note;

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let simple_note = Note;
    if let Some(Command::Supports { renderer }) = cli.command {
        return if simple_note.supports_renderer(&renderer) {
            Ok(())
        } else {
            Err(format!("Renderer '{renderer}' is unsupported"))
        };
    }

    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())
        .map_err(|e| format!("blah: {e}"))?;
    let processed = simple_note.run(&ctx, book).map_err(|e| format!("{e}"))?;
    serde_json::to_writer(io::stdout(), &processed).map_err(|e| format!("{e}"))
}

/// A simple preprocessor for semantic notes in _The Rust Programming Language_.
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Is the renderer supported?
    ///
    /// All renderers are supported! This is the contract for mdBook.
    Supports { renderer: String },
}
