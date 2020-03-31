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
echo 'Checking for broken links...'
rustup toolchain install nightly -c rust-docs --profile=minimal
curl -sSLo linkcheck.sh \
    https://raw.githubusercontent.com/rust-lang/rust/master/src/tools/linkchecker/linkcheck.sh
# Cannot use --all here because of the generated redirect pages aren't available.
sh linkcheck.sh book
