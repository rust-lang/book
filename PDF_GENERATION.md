# PDF Generation Guide

This document explains how to generate a PDF version of "The Rust Programming Language" book in Arabic.

## Quick Start

The simplest way to generate a PDF is to use the provided build script:

```bash
./build-pdf.sh
```

This will automatically:
1. Check for required dependencies
2. Install mdbook-pdf if needed
3. Build the HTML version
4. Generate the PDF

The PDF will be available at: `book/pdf/output.pdf`

## Prerequisites

### Required

1. **Rust and Cargo**: Install from [rustup.rs](https://rustup.rs/)

2. **mdbook**: Install using cargo
   ```bash
   cargo install mdbook --locked
   ```

3. **mdbook custom plugins**: Install from this repository
   ```bash
   cargo install --locked --path packages/mdbook-trpl --force
   ```

### For PDF Generation

Choose one of the following methods:

#### Method 1: mdbook-pdf (Recommended)

Install mdbook-pdf:
```bash
cargo install mdbook-pdf
```

Then build:
```bash
mdbook-pdf
```

**Note**: mdbook-pdf requires Chrome/Chromium to be installed on your system.

#### Method 2: Using Pandoc (Alternative)

If you prefer using Pandoc for better RTL support:

1. Install Pandoc: [pandoc.org/installing.html](https://pandoc.org/installing.html)

2. Install XeLaTeX (for proper Arabic font support):
   - **Ubuntu/Debian**: `sudo apt-get install texlive-xetex texlive-lang-arabic`
   - **macOS**: `brew install --cask mactex`
   - **Windows**: Install [MiKTeX](https://miktex.org/)

3. Build HTML first:
   ```bash
   mdbook build
   ```

4. Convert to PDF using Pandoc:
   ```bash
   pandoc book/*.html -o output.pdf \
     --pdf-engine=xelatex \
     --variable mainfont="Amiri" \
     --variable dir=rtl \
     --variable lang=ar \
     --toc \
     --number-sections
   ```

#### Method 3: Print to PDF (Manual)

1. Build the HTML version:
   ```bash
   mdbook build
   ```

2. Open the book in your web browser:
   ```bash
   firefox book/index.html  # or your preferred browser
   ```

3. Use the browser's print function (Ctrl+P or Cmd+P) and select "Save as PDF"

4. Ensure "Background graphics" is enabled for proper styling

## Configuration

The PDF generation is configured in `book.toml`:

```toml
[output.pdf]
enable = true
additional-css = ["theme/pdf.css"]
paper-size = "a4"
margin-top = 20
margin-bottom = 20
margin-left = 20
margin-right = 20
```

### PDF Styling

The file `theme/pdf.css` contains styles specifically for PDF generation:
- Right-to-left (RTL) text direction
- Arabic font support
- Proper code block formatting (LTR)
- Page break handling
- Print-optimized styling

You can customize the PDF appearance by editing this file.

## Font Requirements

For proper Arabic text rendering in PDFs, ensure these fonts are installed on your system:

- **Amiri** (Recommended): [GitHub - Amiri Font](https://github.com/aliftype/amiri)
- **Noto Naskh Arabic**: Included in most Linux distributions
- **Traditional Arabic**: Available on Windows
- **Arial Unicode MS**: Fallback option

### Installing Amiri Font

**Ubuntu/Debian:**
```bash
sudo apt-get install fonts-amiri
```

**macOS:**
```bash
brew tap homebrew/cask-fonts
brew install --cask font-amiri
```

**Windows:**
Download from [GitHub releases](https://github.com/aliftype/amiri/releases) and install.

## Troubleshooting

### Issue: mdbook-pdf fails with Chrome error

**Solution**: Install Chrome or Chromium:
- Ubuntu: `sudo apt-get install chromium-browser`
- macOS: `brew install --cask google-chrome`

### Issue: Arabic text appears broken or reversed

**Solution**:
1. Ensure you're using XeLaTeX (if using Pandoc)
2. Verify Arabic fonts are installed
3. Check that RTL settings are properly configured in `book.toml` and `theme/pdf.css`

### Issue: Code blocks appear in RTL

**Solution**: The `theme/pdf.css` already handles this, but if issues persist:
1. Verify the CSS is being loaded
2. Check that code blocks have the proper `ltr` direction in the CSS

### Issue: Build fails with missing dependencies

**Solution**: Install the mdbook plugins:
```bash
cargo install --locked --path packages/mdbook-trpl --force
```

## Advanced Options

### Custom PDF Output Name

To specify a custom output name with mdbook-pdf:

```bash
mdbook-pdf --output rust-book-arabic.pdf
```

### Adjusting PDF Quality

Edit `book.toml` and add print quality settings:

```toml
[output.pdf]
enable = true
additional-css = ["theme/pdf.css"]
paper-size = "a4"
margin-top = 20
margin-bottom = 20
margin-left = 20
margin-right = 20
# Optional: Add custom settings
scale = 1.0
display-header-footer = true
```

## CI/CD Integration

To generate PDFs automatically in CI/CD pipelines, add this to your workflow:

```yaml
- name: Install dependencies
  run: |
    cargo install mdbook --locked
    cargo install mdbook-pdf
    cargo install --locked --path packages/mdbook-trpl --force

- name: Build PDF
  run: ./build-pdf.sh

- name: Upload PDF artifact
  uses: actions/upload-artifact@v3
  with:
    name: rust-book-arabic-pdf
    path: book/pdf/output.pdf
```

## Additional Resources

- [mdbook Documentation](https://rust-lang.github.io/mdBook/)
- [mdbook-pdf GitHub](https://github.com/HollowMan6/mdbook-pdf)
- [Pandoc User Guide](https://pandoc.org/MANUAL.html)
- [Arabic Typography in LaTeX](https://www.ctan.org/pkg/arabxetex)

## Support

For issues specific to:
- **Book content**: Open an issue in this repository
- **mdbook-pdf**: Visit [mdbook-pdf issues](https://github.com/HollowMan6/mdbook-pdf/issues)
- **Arabic rendering**: Check font installation and ensure XeLaTeX is being used

---

Happy PDF generation! 📚
