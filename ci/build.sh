export PATH=$PATH:/home/travis/.cargo/bin;

# tests for the second edition

cd second-edition
bash spellcheck.sh list
mdbook test
mdbook build
cargo run --bin lfp src
