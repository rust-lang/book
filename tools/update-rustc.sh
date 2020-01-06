#!/bin/bash

set -eu

# Build the book before making any changes for comparison of the output.
mdbook build -d tmp/book-before

# TODO rustfmt everything
# for f in listings/*
# do
#   rustfmt
#   cargo build to get any Cargo.lock changes?
# done

# Get listings without anchor comments in tmp by compiling a release listings artifact
cargo run --bin release_listings

root_dir=$(pwd)

# For any listings where we show the output,
for f in listings/*/*/output.txt
do
    build_directory=$(dirname $f)
    full_build_directory="${root_dir}/${build_directory}"
    full_output_path="${full_build_directory}/output.txt"
    tmp_build_directory="tmp/${build_directory}"

    cd $tmp_build_directory

    # Save the previous compile time
    compile_time=$(sed -ne "s/.*Finished dev \[unoptimized \+ debuginfo] target(s) in \([0-9.]*\).*/\1/p" ${full_output_path})

    # Act like this is the first time this listing has been built
    cargo clean

    # Run the command in the existing output file
    cargo_command=$(sed -ne "s/$ \(.*\)/\1/p" ${full_output_path})

    # Clear the output file of everything except the command
    echo "$ ${cargo_command}" > ${full_output_path}

    # Regenerate the output and append to the output file. Turn some warnings
    # off to reduce output noise, and use one test thread to get consistent
    # ordering of tests in the output when the command is `cargo test`.
    RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 $cargo_command >> ${full_output_path} 2>&1 || true

    # Set the project file path to the projects directory plus the crate name
    sed -i '' -E -e "s/(Compiling|Checking) ([^\)]*) v0.1.0 (.*)/\1 \2 v0.1.0 (file:\/\/\/projects\/\2)/" ${full_output_path}

    # Restore the previous compile time, if there is one
    if [ -n  "${compile_time}" ]; then
        sed -i '' -e "s/Finished dev \[unoptimized \+ debuginfo] target(s) in [0-9.]*/Finished dev [unoptimized + debuginfo] target(s) in ${compile_time}/" ${full_output_path}
    fi

    cd - > /dev/null
done

# Build the book after making all the changes
mdbook build -d tmp/book-after

# Run the megadiff script that removes all files that are the same, leaving only files to audit
./tools/megadiff.sh
