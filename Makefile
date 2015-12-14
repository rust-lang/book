default:
	mdbook build

test:
	find . -name "*.md" | xargs -I{} rustdoc --test {}

all: default test
