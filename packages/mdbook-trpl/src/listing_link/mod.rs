use mdbook::{
    book::Book,
    errors::Result,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use regex::Regex;

pub struct ListingLinkPreprocessor;

impl Preprocessor for ListingLinkPreprocessor {
    fn name(&self) -> &str {
        "trpl-listing-link"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let re = Regex::new(r"Listing\s(\d+-\d+)").unwrap();

        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                chapter.content = re.replace_all(&chapter.content, r##"<a href="#listing-$1" class="listing-link">Listing $1</a>"##).to_string();
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "markdown" || renderer == "test"
    }
}
