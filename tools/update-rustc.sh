#!/bin/bash

set -eu

# Build the book before making any changes for comparison of the output.
echo 'Building book into `tmp/book-before` before updating...'
mdbook build -d tmp/book-before

# Rustfmt all listings
echo 'Formatting all listings...'
find -s listings -name Cargo.toml -print0 | while IFS= read -r -d '' f; do
    dir_to_fmt=$(dirname $f)

    # There are a handful of listings we don't want to rustfmt and skipping
    # doesn't work; those will have a file in their directory that explains why.
    if [ ! -f "${dir_to_fmt}/rustfmt-ignore" ]; then
        cd $dir_to_fmt
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
    build_directory=$(dirname $f)
    full_build_directory="${root_dir}/${build_directory}"
    full_output_path="${full_build_directory}/output.txt"
    tmp_build_directory="tmp/${build_directory}"

    cd $tmp_build_directory

    # Save the previous compile time; we're going to keep it to minimize diff
    # churn
    compile_time=$(sed -E -ne 's/.*Finished (dev|test) \[unoptimized \+ debuginfo] target\(s\) in ([0-9.]*).*/\2/p' ${full_output_path})

    # Save the hash from the first test binary; we're going to keep it to
    # minimize diff churn
    test_binary_hash=$(sed -E -ne 's@.*Running [^[:space:]]+ \(target/debug/deps/[^-]*-([^\s]*)\)@\1@p' ${full_output_path} | head -n 1)

    # Act like this is the first time this listing has been built
    cargo clean

    # Run the command in the existing output file
    cargo_command=$(sed -ne 's/$ \(.*\)/\1/p' ${full_output_path})

    # Clear the output file of everything except the command
    echo "$ ${cargo_command}" > ${full_output_path}

    # Regenerate the output and append to the output file. Turn some warnings
    # off to reduce output noise, and use one test thread to get consistent
    # ordering of tests in the output when the command is `cargo test`.
    RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 $cargo_command >> ${full_output_path} 2>&1 || true

    # Set the project file path to the projects directory plus the crate name
    # instead of a path to the computer of whoever is running this
    sed -i '' -E -e 's@(Compiling|Checking) ([^\)]*) v0.1.0 (.*)@\1 \2 v0.1.0 (file:///projects/\2)@' ${full_output_path}

    # Restore the previous compile time, if there is one
    if [ -n  "${compile_time}" ]; then
        sed -i '' -E -e "s/Finished (dev|test) \[unoptimized \+ debuginfo] target\(s\) in [0-9.]*/Finished \1 [unoptimized + debuginfo] target(s) in ${compile_time}/" ${full_output_path}
    fi

    # Restore the previous test binary hash, if there is one
    if [ -n "${test_binary_hash}" ]; then
        replacement='s@Running ([^[:space:]]+) \(target/debug/deps/([^-]*)-([^\s]*)\)@Running \1 (target/debug/deps/\2-'
        replacement+="${test_binary_hash}"
        replacement+=')@g'
        sed -i '' -E -e "${replacement}" ${full_output_path}
    fi

    cd - > /dev/null
done

# Build the book after making all the changes
echo 'Building book into `tmp/book-after` after updating...'
mdbook build -d tmp/book-after

# Run the megadiff script that removes all files that are the same, leaving only files to audit
echo 'Removing tmp files that had no changes from the update...'
./tools/megadiff.sh

echo 'Done.'
