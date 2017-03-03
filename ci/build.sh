export PATH=$PATH:/home/travis/.cargo/bin;

# tests for the second edition

cd second-edition
bash spellcheck.sh list || return false;
mdbook test || return false;
mdbook build || return false;
cargo run --bin lfp src || return false;
