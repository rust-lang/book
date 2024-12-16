#!/bin/bash

set -eu

cargo build --release

cargo install --locked --path ./packages/mdbook-trpl --offline

mkdir -p tmp
rm -rf tmp/*.md
rm -rf tmp/markdown

# Render the book as Markdown to include all the code listings
MDBOOK_OUTPUT__MARKDOWN=1 mdbook build nostarch

# Get all the Markdown files
# TODO: what was this doing and why?!?
# find tmp/markdown -name "${1:-\"\"}*.md" -print0 | \
find tmp/markdown -name "*.md" -print0 | \
# Extract just the filename so we can reuse it easily.
xargs -0 basename | \
# Remove all links followed by `<!-- ignore -->``, then
# Change all remaining links from Markdown to italicized inline text.
while IFS= read -r filename; do
  < "tmp/markdown/$filename" ./target/release/remove_links \
    | ./target/release/link2print \
    | ./target/release/remove_markup \
    | ./target/release/remove_hidden_lines \
    | ./target/release/cleanup_blockquotes > "tmp/$filename"
done
# Concatenate the files into the `nostarch` dir.
./target/release/concat_chapters tmp nostarch
