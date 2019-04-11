#!/bin/bash

set -eu

cargo build --release

mkdir -p tmp
rm -rf tmp/*.md

# Get all the Markdown files in the src dir,
ls src/${1:-""}*.md | \
# except for `SUMMARY.md`.
grep -v SUMMARY.md | \
# Extract just the filename so we can reuse it easily.
xargs -n 1 basename | \
# Remove all links followed by `<!-- ignore -->``, then
# Change all remaining links from Markdown to italicized inline text.
while IFS= read -r filename; do
  < "src/$filename" ./target/release/remove_links \
    | ./target/release/link2print \
    | ./target/release/remove_markup > "tmp/$filename"
done
# Concatenate the files into the `nostarch` dir.
./target/release/concat_chapters tmp nostarch
