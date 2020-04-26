#!/bin/bash

set -eu

dir=$1

mkdir -p "tmp/$dir"

for f in $dir/*.md
do
    cat "$f" | cargo run --bin convert_quotes > "tmp/$f"
    mv "tmp/$f" "$f"
done
