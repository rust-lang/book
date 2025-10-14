#!/bin/bash

set -eu

# Build book `trpl` crate dependency in the location where the listings will go
# looking for it so they can compile correctly.
echo 'Building book dependencies in tmp/packages...'
mkdir -p tmp/packages
cp -r packages/trpl tmp/packages/trpl
cd tmp/packages/trpl
 # hide the output; if it fails, debug then.
cargo clean > /dev/null 2>&1
cargo build > /dev/null 2>&1
cd - > /dev/null

# Build the book before making any changes for comparison of the output.
echo 'Building book into tmp/book-before before updating...'
mdbook build -d tmp/book-before

# Rustfmt all listings
echo 'Formatting all listings...'
find -s listings -name Cargo.toml -print0 | while IFS= read -r -d '' f; do
    dir_to_fmt=$(dirname "$f")

    # There are a handful of listings we don't want to rustfmt and skipping
    # doesn't work; those will have a file in their directory that explains why.
    if [ ! -f "${dir_to_fmt}/rustfmt-ignore" ]; then
        cd "$dir_to_fmt"
        cargo fmt --all && true
        cd - > /dev/null
    fi
done

# Get listings without anchor comments in tmp by compiling a release listings
# artifact
echo 'Generate listings without anchor comments...'
cargo run --bin release_listings

root_dir=$(pwd)

echo 'Regenerating output...'
# For any listings where we show the output,
find -s listings -name output.txt -print0 | while IFS= read -r -d '' f; do
    build_directory=$(dirname "$f")
    full_build_directory="${root_dir}/${build_directory}"
    full_output_path="${full_build_directory}/output.txt"
    tmp_build_directory="tmp/${build_directory}"

    cd "$tmp_build_directory"

    # Save the previous compile time; we're going to keep it to minimize diff
    # churn
    compile_time=$(sed -E -ne "s/.*Finished \`(dev|test)\` profile \[unoptimized \+ debuginfo] target\(s\) in ([0-9.]*).*/\2/p" "${full_output_path}")

    # Save the hash from the first test binary; we're going to keep it to
    # minimize diff churn
    test_binary_hash=$(sed -E -ne 's@.*Running [^[:space:]]+( [^[:space:]\(\)]+)? \(target/debug/deps/[^-]*-([^\s]*)\)@\2@p' "${full_output_path}" | head -n 1)

    # Act like this is the first time this listing has been built
    cargo clean > /dev/null 2>&1

    # Run the command in the existing output file
    cargo_command=$(sed -ne 's/$ \(.*\)/\1/p' "${full_output_path}")

    # Clear the output file of everything except the command
    echo "$ ${cargo_command}" > "${full_output_path}"

    # Regenerate the output and append to the output file. Turn some warnings
    # off to reduce output noise, and use one test thread to get consistent
    # ordering of tests in the output when the command is `cargo test`.
    RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 $cargo_command >> "${full_output_path}" 2>&1 || true

    # Set the project file path to the projects directory plus the crate name
    # instead of a path to the computer of whoever is running this
    sed -i '' -E -e 's@(Compiling|Checking) ([^\)]*) v0.1.0 (.*)@\1 \2 v0.1.0 (file:///projects/\2)@' "${full_output_path}"

    # Likewise, use a "default" installation directory for rustup's install
    # location so the version of the source is not a path on the computer of
    # whoever is doing the update. This does two substitutions:
    #
    # - Replaces the path up to `.rustup/toolchains` with `file:///home`, while
    #   preserving leading spaces and the `-->`.
    # - Replaces the version-and-architecture-triple with just the version, so
    #   e.g. `1.82-aarch64-apple-darwin` becomes `1.82`.
    sed -i '' -E -e 's@^([[:space:]]*-->[[:space:]]+).*(\.rustup/toolchains/[[:digit:]]+\.[[:digit:]]+)([^/]*)@\1file:///home/\2@' "${full_output_path}"

    # Similarly, replace Miri paths
    sed -i '' -E -e "s@Running \`(.*)\.rustup/toolchains/nightly([^/]*)/bin/cargo-miri runner target/miri/([^/]*)/debug/([^/]*)@Running \`file:///home/.rustup/toolchains/nightly/bin/cargo-miri runner target/miri/debug/\4@" "${full_output_path}"

    # Restore the previous compile time, if there is one
    if [ -n  "${compile_time}" ]; then
        sed -i '' -E -e "s/Finished \`(dev|test)\` profile \[unoptimized \+ debuginfo] target\(s\) in [0-9.]*/Finished \`\1\` profile [unoptimized + debuginfo] target(s) in ${compile_time}/" "${full_output_path}"
    fi

    # Restore the previous test binary hash, if there is one
    if [ -n "${test_binary_hash}" ]; then
        replacement='s@Running ([^[:space:]]+)( [^[:space:]\(\)]+)? \(target/debug/deps/([^-]*)-([^\s]*)\)@Running \1\2 (target/debug/deps/\3-'
        replacement+="${test_binary_hash}"
        replacement+=')@g'
        sed -i '' -E -e "${replacement}" "${full_output_path}"
    fi

    # Clean again
    cargo clean > /dev/null 2>&1

    cd - > /dev/null
done

# Build the book after making all the changes
echo 'Building book into tmp/book-after after updating...'
mdbook build -d tmp/book-after

# Run the megadiff script that removes all files that are the same, leaving only files to audit
echo 'Removing tmp files that had no changes from the update...'
./tools/megadiff.sh

echo 'Done.'
