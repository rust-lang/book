# Copyright 2016 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

set -e

export PATH=$PATH:/home/travis/.cargo/bin;

# feature check
cd ci/stable-check

cargo run -- ../../first-edition/src
cargo run -- ../../second-edition/src
cargo run -- ../../2018-edition/src

cd ../..

# tests for the first edition
cd first-edition
echo 'Testing first edition...'
mdbook test
echo 'Building first edition...'
mdbook build

cd ..

# tests for the second edition
cd second-edition
echo 'Spellchecking second edition...'
bash ../ci/spellcheck.sh list
echo 'Testing second edition...'
mdbook test
echo 'Building second edition...'
mdbook build
echo 'Linting second edition for local file paths...'
cargo run --bin lfp src

cd ..

# tests for the 2018 edition
cd 2018-edition
echo 'Spellchecking 2018 edition...'
bash ../ci/spellcheck.sh list
echo 'Testing 2018 edition...'
mdbook test
echo 'Building 2018 edition...'
mdbook build
echo 'Linting 2018 edition for local file paths...'
cargo run --bin lfp src
