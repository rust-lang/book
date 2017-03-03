export PATH=$PATH:/home/travis/.cargo/bin;

# tests for the second edition

cd second-edition
bash spellcheck.sh list || return 1;
mdbook test || return 1;
mdbook build || return 1;
cargo run --bin lfp src || return 1;
