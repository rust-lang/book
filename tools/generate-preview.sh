#!/usr/bin/env bash

mdbook build
cp ./tools/preview-robots.txt ./book/robots.txt
ghp-import -m "rebuild GitHub Pages from generated-book" book
git push origin gh-pages
