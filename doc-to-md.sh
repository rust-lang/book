#!/bin/bash

set -eu

xsltproc tools/docx-to-md.xsl tmp/word/document.xml | \
fold -w 80 -s | \
sed -e "s/ *$//" \
> nostarch/chapter02.md
