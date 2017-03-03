#!/bin/bash

set -o errexit -o nounset

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
