use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // Get all listings from the `listings` directory
    let listings_dir = Path::new("listings");

    // Put the results in the `tmp/listings` directory
    let out_dir = Path::new("tmp/listings");

    // Clear out any existing content in `tmp/listings`
    if out_dir.is_dir() {
        fs::remove_dir_all(out_dir)?;
    }

    // Create a new, empty `tmp/listings` directory
    fs::create_dir(out_dir)?;

    // For each chapter in the `listings` directory,
    for chapter in fs::read_dir(listings_dir)? {
        let chapter = chapter?;
        let chapter_path = chapter.path();

        if !chapter_path.is_dir() {
            eprintln!(
                "'{}' is not a directory, skipping",
                chapter_path.display()
            );
            continue;
        }

        let chapter_name = chapter_path
            .file_name()
            .expect("Chapter should've had a name");

        // Create a corresponding chapter dir in `tmp/listings`
        let output_chapter_path = out_dir.join(chapter_name);
        fs::create_dir(&output_chapter_path).map_err(|e| {
            format!(
                "could not create dir at '{}': {e}",
                output_chapter_path.display()
            )
        })?;

        // For each listing in the chapter directory,
        for listing in fs::read_dir(&chapter_path).map_err(|e| {
            format!("Could not read '{}': {e}", chapter_path.display())
        })? {
            let listing = listing.map_err(|e| {
                format!(
                    "bad dir entry listing in {}: {e}",
                    chapter_path.display()
                )
            })?;
            let listing_path = listing.path();

            if !listing_path.is_dir() {
                eprintln!(
                    "'{}' is not a directory, skipping",
                    listing_path.display(),
                );
                continue;
            }

            let listing_name = listing_path
                .file_name()
                .expect("Listing should've had a name");

            // Create a corresponding listing dir in the tmp chapter dir
            let output_listing_dir = output_chapter_path.join(listing_name);
            fs::create_dir(&output_listing_dir).map_err(|e| {
                format!(
                    "could not create dir '{}': {e}",
                    output_listing_dir.display()
                )
            })?;

            // Copy all the cleaned files in the listing to the tmp directory
            copy_cleaned_listing_files(listing_path, output_listing_dir)?;
        }
    }

    // Create a compressed archive of all the listings
    let tarfile = File::create("tmp/listings.tar.gz")?;
    let encoder =
        flate2::write::GzEncoder::new(tarfile, flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    archive.append_dir_all("listings", "tmp/listings")?;

    // Assure whoever is running this that the script exiting successfully, and remind them
    // where the generated file ends up
    println!("Release tarball of listings in tmp/listings.tar.gz");

    Ok(())
}

// Cleaned listings will not contain:
//
// - `target` directories
// - `output.txt` files used to display output in the book
// - `rustfmt-ignore` files used to signal to update-rustc.sh the listing shouldn't be formatted
// - anchor comments or snip comments
// - empty `main` functions in `lib.rs` files used to trick rustdoc
fn copy_cleaned_listing_files(
    from: PathBuf,
    to: PathBuf,
) -> Result<(), Box<dyn Error>> {
    for item in fs::read_dir(&from).map_err(|e| {
        format!("Could not read_dir on '{}': {e}", from.display())
    })? {
        let item = item.map_err(|e| {
            format!("invalid dir entry in {}: {e}", from.display())
        })?;
        let item_path = item.path();

        let item_name =
            item_path.file_name().expect("Item should've had a name");
        let output_item = to.join(item_name);

        if item_path.is_dir() {
            // Don't copy `target` directories
            if item_name != "target" {
                fs::create_dir(&output_item).map_err(|e| {
                    format!(
                        "Could not create output directory '{}': {e}",
                        output_item.display()
                    )
                })?;
                copy_cleaned_listing_files(item_path, output_item)?;
            }
        } else {
            // Don't copy output files or files that tell update-rustc.sh not to format
            if item_name != "output.txt" && item_name != "rustfmt-ignore" {
                let item_extension = item_path.extension();
                if item_extension.is_some() && item_extension.unwrap() == "rs" {
                    copy_cleaned_rust_file(
                        item_name,
                        &item_path,
                        &output_item,
                    )?;
                } else {
                    // Copy any non-Rust files without modification
                    fs::copy(&item_path, &output_item).map_err(|e| {
                        format!(
                            "Could not copy from '{}' to '{}': {e}",
                            item_path.display(),
                            output_item.display()
                        )
                    })?;
                }
            }
        }
    }

    Ok(())
}

lazy_static! {
    static ref ANCHOR_OR_SNIP_COMMENTS: Regex = Regex::new(
        r"(?x)
    //\s*ANCHOR:\s*[\w_-]+      # Remove all anchor comments
    |
    //\s*ANCHOR_END:\s*[\w_-]+  # Remove all anchor ending comments
    |
    //\s*--snip--               # Remove all snip comments
    "
    )
    .unwrap();
}

lazy_static! {
    static ref EMPTY_MAIN: Regex = Regex::new(r"fn main\(\) \{}").unwrap();
}

// Cleaned Rust files will not contain:
//
// - anchor comments or snip comments
// - empty `main` functions in `lib.rs` files used to trick rustdoc
fn copy_cleaned_rust_file(
    item_name: &std::ffi::OsStr,
    from: &PathBuf,
    to: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let from_buf = BufReader::new(File::open(from)?);
    let mut to_buf = BufWriter::new(File::create(to)?);

    for line in from_buf.lines() {
        let line = line?;
        if !ANCHOR_OR_SNIP_COMMENTS.is_match(&line)
            && (item_name != "lib.rs" || !EMPTY_MAIN.is_match(&line))
        {
            writeln!(&mut to_buf, "{line}")?;
        }
    }

    to_buf.flush()?;

    Ok(())
}
