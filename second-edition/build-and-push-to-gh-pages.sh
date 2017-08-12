rev=$(git rev-parse --short HEAD)
rm -rf book
mdbook build
cd book
git init
git remote add upstream "https://github.com/rinthel/rust-lang-book-ko"
git fetch upstream
git reset upstream/gh-pages
touch .
git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
cd ..