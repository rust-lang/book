#!/bin/bash

set -eu

# So that we can do this in a single pass, check *here* if there are any “dirty”
# files from Git, and bail if so.
git diff --quiet || { echo "Git working directory is not clean"; exit 1; }

OLD_EDITION=2021
NEW_EDITION=2024

# Start by preparing all listings for the edition.
find listings -name "Cargo.toml" -exec cargo fix --allow-dirty --edition --manifest-path '{}' \;

# Update the edition itself
find listings -name "Cargo.toml" -exec sed -i '' "s/edition = \"$OLD_EDITION\"/edition = \"$NEW_EDITION\"/g" '{}' \;

# Update all id
find listings -name "Cargo.toml" -exec cargo fix --allow-dirty --edition-idioms --manifest-path {} \;
