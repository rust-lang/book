cd ..
rev=$(git rev-parse --short HEAD)
cd second-edition
rm -rf book
mdbook build
cd book
git init
git remote add upstream "https://rinthel@github.com/rinthel/rust-lang-book-ko"
git fetch upstream
git reset upstream/gh-pages
touch .
git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
cd ..