#!/bin/bash

set -eu

OLD_EDITION=2021
NEW_EDITION=2024

find listings/** -name "Cargo.toml" -exec sed -i '' "s/edition = \"$OLD_EDITION\"/edition = \"$NEW_EDITION\"/g" '{}' \;
