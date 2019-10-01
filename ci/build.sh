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
