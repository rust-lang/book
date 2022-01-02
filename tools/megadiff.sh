#!/bin/bash

set -eu

# Remove files that are never affected by rustfmt or are otherwise uninteresting
rm -rf tmp/book-before/css/ tmp/book-before/theme/ tmp/book-before/img/ tmp/book-before/*.js \
       tmp/book-before/FontAwesome tmp/book-before/*.css tmp/book-before/*.png \
       tmp/book-before/*.json tmp/book-before/print.html

rm -rf tmp/book-after/css/ tmp/book-after/theme/ tmp/book-after/img/ tmp/book-after/*.js \
      tmp/book-after/FontAwesome tmp/book-after/*.css tmp/book-after/*.png \
      tmp/book-after/*.json tmp/book-after/print.html

# Get all the html files before
find tmp/book-before -name '*.html' -print0 | \
# Extract just the filename so we can reuse it easily.
xargs -n 1 basename | \
while IFS= read -r filename; do
    # Remove any files that are the same before and after
    diff "tmp/book-before/$filename" "tmp/book-after/$filename" > /dev/null \
      && rm "tmp/book-before/$filename" "tmp/book-after/$filename"
done
