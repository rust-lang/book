use mdbook::{
    book::Book,
    errors::Result,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use regex::Regex;
use std::collections::HashMap;

pub struct ListingLinkPreprocessor;

impl Preprocessor for ListingLinkPreprocessor {
    fn name(&self) -> &str {
        "trpl-listing-link"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let number_re = Regex::new(r#"id="listing-(\d+-\d+)"#).unwrap();
        let listing_re = Regex::new(r"Listing\s(\d+-\d+)").unwrap();
        let mut listings: HashMap<String, String> = HashMap::new();

        // Collect all number patterns and their corresponding filenames
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                if let Some(ref path) = chapter.path {
                    let filename = path.to_string_lossy().to_string();
                    for cap in number_re.captures_iter(&chapter.content) {
                        let number = cap[1].to_string();
                        listings.insert(number, filename.clone());
                    }
                }
            }
        });

        // Replace listing references with absolute href with filename and listing number hash
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                chapter.content = listing_re.replace_all(&chapter.content, |caps: &regex::Captures| {
                    let listing_number = &caps[1];
                    let href = listings.get(listing_number).map_or_else(
                        || format!("#listing-{}", listing_number),
                        |file| format!("{}#listing-{}", file, listing_number)
                    );
                    format!(r##"<a href="{}" class="listing-link">Listing {}</a>"##, href, listing_number)
                }).to_string();
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "markdown" || renderer == "test"
    }
}
