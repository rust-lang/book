#!/usr/bin/env bash

mdbook build
cp ./tools/preview-robots.txt ./book/robots.txt
