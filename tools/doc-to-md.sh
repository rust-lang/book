#!/bin/bash

set -eu

# Get all the docx files in the tmp dir.
find tmp -name '*.docx' -print0 | \
# Extract just the filename so we can reuse it easily.
xargs -0 basename -s .docx | \
while IFS= read -r filename; do
    # Make a directory to put the XML in.
    mkdir -p "tmp/$filename"
    # Unzip the docx to get at the XML.
    unzip -o "tmp/$filename.docx" -d "tmp/$filename"
    # Convert to markdown with XSL.
    xsltproc tools/docx-to-md.xsl "tmp/$filename/word/document.xml" | \
    # Hard wrap at 80 chars at word bourdaries.
    fold -w 80 -s | \
    # Remove trailing whitespace and save in the `nostarch` dir for comparison.
    sed -e "s/ *$//" > "nostarch/$filename.md"
done
