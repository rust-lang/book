#!/bin/bash

set -eu

cargo build --release

mkdir -p tmp
rm -rf tmp/*.md
rm -rf tmp/markdown

# Render the book as Markdown to include all the code listings
MDBOOK_OUTPUT__MARKDOWN=1 mdbook build -d tmp

# Get all the Markdown files
ls tmp/markdown/${1:-""}*.md | \
# Extract just the filename so we can reuse it easily.
xargs -n 1 basename | \
# Remove all links followed by `<!-- ignore -->``, then
# Change all remaining links from Markdown to italicized inline text.
while IFS= read -r filename; do
  < "tmp/markdown/$filename" ./target/release/remove_links \
    | ./target/release/link2print \
    | ./target/release/remove_markup \
    | ./target/release/remove_hidden_lines > "tmp/$filename"
done
# Concatenate the files into the `nostarch` dir.
./target/release/concat_chapters tmp nostarch
