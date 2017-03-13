#!/bin/bash
# Copyright 2016 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

set -o errexit -o nounset

export PATH=$PATH:/home/travis/.cargo/bin;

rev=$(git rev-parse --short HEAD)

# make a scratch directory, this will correspond to
# https://rust-lang.github.io/book
mkdir book

# build the index page into it
rustdoc index.md -o book/

# build the first edition into it
mdbook build first-edition
mv first-edition/book book/first-edition/


# build the second edition into it
mdbook build second-edition
mv second-edition/book book/second-edition/

# move into the book dir and push it

cd book

git init
git config user.name "Steve Klabnik"
git config user.email "steve@steveklabnik.com"

git remote add upstream "https://$GH_TOKEN@github.com/rust-lang/book.git"
git fetch upstream
git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages > /dev/null 2>&1
