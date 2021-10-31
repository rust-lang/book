#!/bin/bash

set -eu

mkdir -p tmp/src
rm -rf tmp/*.md

for f in src/${1:-""}*.md
do
    cargo run --bin convert_quotes < "$f" > "tmp/$f"
    mv "tmp/$f" "$f"
done
