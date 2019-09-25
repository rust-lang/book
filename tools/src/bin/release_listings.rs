#[macro_use] extern crate lazy_static;

use std::error::Error;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let listings_dir = Path::new("listings");
    let out_dir = Path::new("tmp/listings");

    if out_dir.is_dir() {
        fs::remove_dir_all(out_dir)?;
    }

    fs::create_dir(out_dir)?;

    for chapter in fs::read_dir(listings_dir)? {
        let chapter = chapter?;
        let chapter_path = chapter.path();

        let chapter_name = chapter_path.file_name().expect("Chapter should've had a name");

        // Create corresponding chapter dir in tmp
        let output_chapter_path = out_dir.join(chapter_name);
        fs::create_dir(&output_chapter_path)?;

        for listing in fs::read_dir(chapter_path)? {
            let listing = listing?;
            let listing_path = listing.path();

            let listing_name = listing_path.file_name().expect("Listing should've had a name");

            // Create corresponding listing dir in tmp
            let output_listing_dir = output_chapter_path.join(listing_name);
            fs::create_dir(&output_listing_dir)?;

            copy_cleaned_listing_files(listing_path, output_listing_dir)?;
        }
    }

    let tarfile = File::create("tmp/listings.tar.gz")?;
    let encoder = flate2::write::GzEncoder::new(tarfile, flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    archive.append_dir_all("listings", "tmp/listings")?;

    fs::remove_dir_all(out_dir)?;

    println!("Release tarball of listings in tmp/listings.tar.gz");

    Ok(())
}

fn copy_cleaned_listing_files(from: PathBuf, to: PathBuf) -> Result<(), Box<dyn Error>> {
    for item in fs::read_dir(from)? {
        let item = item?;
        let item_path = item.path();

        let item_name = item_path.file_name().expect("Item should've had a name");
        let output_item = to.join(item_name);

        if item_path.is_dir() {
            // Don't copy `target` directories
            if item_name != "target" {
                fs::create_dir(&output_item)?;
                copy_cleaned_listing_files(item_path, output_item)?;
            }
        } else {
            let item_extension = item_path.extension();
            if item_extension.is_some() && item_extension.unwrap() == "rs" {
                copy_cleaned_rust_file(item_name, &item_path, &output_item)?;
            } else {
                // Copy any non-Rust files without modification
                fs::copy(item_path, output_item)?;
            }
        }
    }

    Ok(())
}

lazy_static! {
    static ref ANCHOR_OR_SNIP_COMMENTS: Regex = Regex::new(r"(?x)
    //\s*ANCHOR:\s*[\w_-]+      # Remove all anchor comments
    |
    //\s*ANCHOR_END:\s*[\w_-]+  # Remove all anchor ending comments
    |
    //\s*--snip--               # Remove all snip comments
    ").unwrap();
}

lazy_static! {
    static ref EMPTY_MAIN: Regex = Regex::new(r"fn main\(\) \{}").unwrap();
}

fn copy_cleaned_rust_file(item_name: &std::ffi::OsStr, from: &PathBuf, to: &PathBuf) -> Result<(), Box<dyn Error>> {
    let from_buf = BufReader::new(File::open(from)?);
    let mut to_buf = BufWriter::new(File::create(to)?);

    for line in from_buf.lines() {
        let line = line?;
        if !ANCHOR_OR_SNIP_COMMENTS.is_match(&line) {
            if item_name != "lib.rs" || !EMPTY_MAIN.is_match(&line) {
                writeln!(&mut to_buf, "{}", line)?;
            }
        }
    }

    to_buf.flush()?;

    Ok(())
}
