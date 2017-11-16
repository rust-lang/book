#!/bin/bash
# Copyright 2016 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

set -eu

cargo build --release

mkdir -p tmp
rm -rf tmp/*.md

# Get all the markdown files in the src dir,
ls src/${1:-""}*.md | \
# except for SUMMARY.md.
grep -v SUMMARY.md | \
# Extract just the filename so we can reuse it easily.
xargs -n 1 basename | \
# Remove all links followed by <!-- ignore -->, then
# Change all remaining links from markdown to italicized inline text.
while IFS= read -r filename; do
  < "src/$filename" ./target/release/remove_links \
    | ./target/release/link2print \
    | ./target/release/remove_markup > "tmp/$filename"
done
# Concat the files into the nostarch dir.
./target/release/concat_chapters tmp nostarch
