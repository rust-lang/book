#!/bin/bash
# Alternative script to build PDF using Pandoc and XeLaTeX
# This provides better RTL and Arabic font support

set -e

echo "==================================="
echo "Building Arabic Rust Book PDF"
echo "Using Pandoc + XeLaTeX"
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

# Check if pandoc is installed
if ! command -v pandoc &> /dev/null; then
    echo -e "${RED}Error: pandoc is not installed${NC}"
    echo "Please install pandoc from: https://pandoc.org/installing.html"
    exit 1
fi

# Check if xelatex is installed
if ! command -v xelatex &> /dev/null; then
    echo -e "${YELLOW}Warning: xelatex is not installed${NC}"
    echo "XeLaTeX is recommended for proper Arabic font support."
    echo "Install instructions:"
    echo "  Ubuntu/Debian: sudo apt-get install texlive-xetex texlive-lang-arabic"
    echo "  macOS: brew install --cask mactex"
    echo ""
    echo "Continuing anyway, but PDF generation may fail..."
    echo ""
fi

# Build the book plugins first
echo -e "${GREEN}Building mdbook plugins...${NC}"
if [ -d "packages/mdbook-trpl" ]; then
    cargo install --locked --path packages/mdbook-trpl --force 2>&1 | grep -v "warning:" || true
fi

# Build HTML version first
echo ""
echo -e "${GREEN}Building HTML version...${NC}"
mdbook build

# Create output directory
mkdir -p pdf_output

# Merge all HTML content (this is a simplified approach)
# For a real implementation, you might want to process the SUMMARY.md file
echo ""
echo -e "${GREEN}Processing book content...${NC}"

# Create a temporary markdown file with all chapters
TEMP_MD="pdf_output/combined.md"
echo "---" > "$TEMP_MD"
echo "title: The Rust Programming Language" >> "$TEMP_MD"
echo "author: Steve Klabnik, Carol Nichols, Chris Krycho" >> "$TEMP_MD"
echo "lang: ar" >> "$TEMP_MD"
echo "dir: rtl" >> "$TEMP_MD"
echo "---" >> "$TEMP_MD"
echo "" >> "$TEMP_MD"

# Combine markdown files from src directory in order
if [ -f "src/SUMMARY.md" ]; then
    # Extract chapter references from SUMMARY.md and concatenate them
    grep -oP '\(.*\.md\)' src/SUMMARY.md | tr -d '()' | while read -r file; do
        if [ -f "src/$file" ]; then
            echo "" >> "$TEMP_MD"
            cat "src/$file" >> "$TEMP_MD"
            echo "" >> "$TEMP_MD"
            echo "\\newpage" >> "$TEMP_MD"
            echo "" >> "$TEMP_MD"
        fi
    done
else
    # Fallback: just concatenate all markdown files
    for file in src/*.md; do
        if [ -f "$file" ] && [ "$(basename "$file")" != "SUMMARY.md" ]; then
            cat "$file" >> "$TEMP_MD"
            echo "" >> "$TEMP_MD"
            echo "\\newpage" >> "$TEMP_MD"
            echo "" >> "$TEMP_MD"
        fi
    done
fi

# Generate PDF using Pandoc with XeLaTeX
echo ""
echo -e "${GREEN}Generating PDF with Pandoc...${NC}"
pandoc "$TEMP_MD" -o "pdf_output/rust-book-arabic.pdf" \
    --pdf-engine=xelatex \
    --variable mainfont="Amiri" \
    --variable sansfont="Noto Sans Arabic" \
    --variable monofont="Courier New" \
    --variable fontsize=12pt \
    --variable papersize=a4 \
    --variable geometry:margin=2cm \
    --variable dir=rtl \
    --variable lang=ar \
    --table-of-contents \
    --toc-depth=3 \
    --number-sections \
    --highlight-style=tango \
    --metadata title="لغة البرمجة Rust" \
    2>&1 || {
        echo -e "${RED}PDF generation failed.${NC}"
        echo "This might be due to:"
        echo "  1. Missing fonts (install Amiri and Noto Sans Arabic)"
        echo "  2. Missing LaTeX packages"
        echo "  3. Content formatting issues"
        exit 1
    }

# Clean up temporary file
rm -f "$TEMP_MD"

echo ""
echo -e "${GREEN}==================================="
echo "Build completed successfully!"
echo "===================================${NC}"
echo ""
echo "PDF generated at: pdf_output/rust-book-arabic.pdf"
echo ""
echo "To view the PDF:"
echo "  xdg-open pdf_output/rust-book-arabic.pdf  # Linux"
echo "  open pdf_output/rust-book-arabic.pdf      # macOS"
echo ""
