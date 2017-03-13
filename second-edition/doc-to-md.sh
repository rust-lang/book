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

# Get all the docx files in the tmp dir,
ls tmp/*.docx | \
# Extract just the filename so we can reuse it easily.
xargs -n 1 basename -s .docx | \
while IFS= read -r filename; do
  # Make a directory to put the XML in
  mkdir -p "tmp/$filename"
  # Unzip the docx to get at the xml
  unzip -o "tmp/$filename.docx" -d "tmp/$filename"
  # Convert to markdown with XSL
  xsltproc tools/docx-to-md.xsl "tmp/$filename/word/document.xml" | \
  # Hard wrap at 80 chars at word bourdaries
  fold -w 80 -s | \
  # Remove trailing whitespace & save in the nostarch dir for comparison
  sed -e "s/ *$//" > "nostarch/$filename.md"
done
