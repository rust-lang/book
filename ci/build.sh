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

cargo run -- ../../src

cd ../..

echo 'Spellchecking...'
bash ci/spellcheck.sh list
echo 'Testing...'
mdbook test
echo 'Building...'
mdbook build
echo 'Linting for local file paths...'
cargo run --bin lfp src
