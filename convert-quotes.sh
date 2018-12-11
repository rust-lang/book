#!/bin/bash
# Copyright 2017 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

set -eu

dir=$1

mkdir -p "tmp/$dir"

for f in $dir/*.md
do
    cat "$f" | cargo run --bin convert_quotes > "tmp/$f"
    mv "tmp/$f" "$f"
done
