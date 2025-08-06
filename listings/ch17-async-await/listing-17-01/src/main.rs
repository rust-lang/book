extern crate trpl; // required for mdbook test

fn main() {
    // TODO: we'll add this next!
}

// ANCHOR: all
use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
// ANCHOR_END: all
