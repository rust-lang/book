#!/bin/bash

set -eu

OLD_EDITION=2018
NEW_EDITION=2021

find listings/** -name "Cargo.toml" -exec sed -i '' "s/edition = \"$OLD_EDITION\"/edition = \"$NEW_EDITION\"/g" '{}' \;
