#!/bin/bash

set -e

export PATH=$PATH:/home/travis/.cargo/bin;

echo 'Spellchecking...'
bash ci/spellcheck.sh list
echo 'Testing...'
mdbook test
echo 'Building...'
mdbook build
echo 'Linting for local file paths...'
cargo run --bin lfp src
echo 'Validating references'
for file in src/*.md ; do
    echo Checking references in $file
    cargo run --quiet --bin link2print < $file > /dev/null
done
