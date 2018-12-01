#!/bin/bash
pushd ../src
pandoc -o ../epub/rust-book.epub -f markdown-tex_math_dollars \
	../epub/metadata.txt\
	ch*.md\
	appendix*.md
popd


