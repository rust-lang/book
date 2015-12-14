#!/bin/bash

set -o errexit -o nounset

rev=$(git rev-parse --short HEAD)

cd book

git init
git config user.name "Steve Klabnik"
git config user.email "steve@steveklabnik.com"

git remote add upstream "https://$GH_TOKEN@github.com/rust-lang/rust-by-example.git"
git fetch upstream
git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
