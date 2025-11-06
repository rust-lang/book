#!/bin/bash
# Script to build PDF version of "The Rust Programming Language" book (Arabic)
# This script requires mdbook and mdbook-pdf to be installed

set -e

echo "==================================="
echo "Building Arabic Rust Book PDF"
echo "==================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if mdbook is installed
if ! command -v mdbook &> /dev/null; then
    echo -e "${RED}Error: mdbook is not installed${NC}"
    echo "Please install mdbook using:"
    echo "  cargo install mdbook --locked"
    exit 1
fi

# Check if mdbook-pdf is installed
if ! command -v mdbook-pdf &> /dev/null; then
    echo -e "${YELLOW}Warning: mdbook-pdf is not installed${NC}"
    echo "Installing mdbook-pdf..."
    cargo install mdbook-pdf
fi

# Build the book plugins first
echo -e "${GREEN}Building mdbook plugins...${NC}"
if [ -d "packages/mdbook-trpl" ]; then
    cargo install --locked --path packages/mdbook-trpl --force
fi

# Build HTML version (required as intermediate step)
echo ""
echo -e "${GREEN}Building HTML version...${NC}"
mdbook build

# Build PDF version
echo ""
echo -e "${GREEN}Building PDF version...${NC}"
mdbook-pdf

echo ""
echo -e "${GREEN}==================================="
echo "Build completed successfully!"
echo "===================================${NC}"
echo ""
echo "The PDF should be available in: book/pdf/output.pdf"
echo ""
