extern crate trpl; // required for mdbook test

use trpl::Html;

fn main() {
    // TODO: we'll add this next!
}

async fn page_title(url: &str) -> Option<String> {
    // ANCHOR: chaining
    let response_text = trpl::get(url).await.text().await;
    // ANCHOR_END: chaining
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
