#!/bin/bash

set -eu

cargo build --release

mkdir -p tmp
rm -rf tmp/*.md

# Get all the markdown files in the src dir,
ls src/*.md | \
# except for SUMMARY.md.
grep -v SUMMARY.md | \
# Extract just the filename so we can reuse it easily.
xargs -n 1 basename | \
# Remove all links followed by <!-- ignore -->, then
# Change all remaining links from markdown to italicized inline text.
while IFS= read -r filename; do
  < "src/$filename" cargo run --bin remove_links | cargo run --bin link2print > "tmp/$filename"
done
# Concat the files into the nostarch dir.
cargo run --bin concat_chapters tmp nostarch
