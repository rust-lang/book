#!/bin/bash

set -eu

# Build the book before making any changes for comparison of the output.
mdbook build -d tmp/book-before

# TODO rustfmt everything
# for f in listings/*
# do
#   rustfmt
# done

for f in listings/*/*/output.txt
do
    build_directory=$(dirname $f)
    cd $build_directory

    # Save the previous compile time
    compile_time=$(sed -ne "s/.*Finished dev \[unoptimized \+ debuginfo] target(s) in \([0-9.]*\).*/\1/p" output.txt)

    # Regenerate output
    cargo clean
    cargo run &> output.txt

    # Set the file path to the projects directory plus the crate name
    sed -i '' -e "s/Compiling \([a-z_]*\) v0.1.0 (.*)/Compiling \1 v0.1.0 (file:\/\/\/projects\/\1)/" output.txt

    # Restore the previous compile time
    sed -i '' -e "s/Finished dev \[unoptimized \+ debuginfo] target(s) in [0-9.]*/Finished dev [unoptimized + debuginfo] target(s) in ${compile_time}/" output.txt

    cd -
done

# Build the book after making all the changes
mdbook build -d tmp/book-after

# Run the megadiff script that removes all files that are the same, leaving only files to audit
./tools/megadiff.sh
