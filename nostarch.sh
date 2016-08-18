#!/bin/bash

set -eu

mkdir -p tmp

# Get all the markdown files in the src dir,
ls src/*.md | \
# except for SUMMARY.md.
grep -v SUMMARY.md | \
# Extract just the filename so we can reuse it easily.
xargs -n 1 basename | \
# Change all the links from markdown to italicized inline text.
while IFS= read -r filename; do
  cargo run --bin link2print < "src/$filename" > "tmp/$filename"
done
# Concat the files into the nostarch dir.
cargo run --bin concat_chapters tmp nostarch
