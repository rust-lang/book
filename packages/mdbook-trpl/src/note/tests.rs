use super::*;

#[test]
fn no_note() {
    let text = "Hello, world.\n\nThis is some text.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<p>Hello, world.</p>\n<p>This is some text.</p>\n"
    );
}

#[test]
fn with_note() {
    let text = "> Note: This is some text.\n> It keeps going.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<p>Note: This is some text.\nIt keeps going.</p>\n</section>"
    );
}

#[test]
fn regular_blockquote() {
    let text = "> This is some text.\n> It keeps going.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<blockquote>\n<p>This is some text.\nIt keeps going.</p>\n</blockquote>\n"
    );
}

#[test]
fn combined() {
    let text = "> Note: This is some text.\n> It keeps going.\n\nThis is regular text.\n\n> This is a blockquote.\n";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<p>Note: This is some text.\nIt keeps going.</p>\n</section>\n<p>This is regular text.</p>\n<blockquote>\n<p>This is a blockquote.</p>\n</blockquote>\n"
    );
}

#[test]
fn blockquote_then_note() {
    let text = "> This is quoted.\n\n> Note: This is noted.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<blockquote>\n<p>This is quoted.</p>\n</blockquote>\n<section class=\"note\" aria-role=\"note\">\n<p>Note: This is noted.</p>\n</section>"
    );
}

#[test]
fn note_then_blockquote() {
    let text = "> Note: This is noted.\n\n> This is quoted.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<p>Note: This is noted.</p>\n</section>\n<blockquote>\n<p>This is quoted.</p>\n</blockquote>\n"
    );
}

#[test]
fn with_h1_note() {
    let text = "> # Header\n > And then some note content.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<h1>Header</h1>\n<p>And then some note content.</p>\n</section>"
    );
}

#[test]
fn with_h2_note() {
    let text = "> ## Header\n > And then some note content.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<h2>Header</h2>\n<p>And then some note content.</p>\n</section>"
    );
}

#[test]
fn with_h3_note() {
    let text = "> ### Header\n > And then some note content.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<h3>Header</h3>\n<p>And then some note content.</p>\n</section>"
    );
}

#[test]
fn with_h4_note() {
    let text = "> #### Header\n > And then some note content.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<h4>Header</h4>\n<p>And then some note content.</p>\n</section>"
    );
}

#[test]
fn with_h5_note() {
    let text = "> ##### Header\n > And then some note content.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<h5>Header</h5>\n<p>And then some note content.</p>\n</section>"
    );
}

#[test]
fn with_h6_note() {
    let text = "> ###### Header\n > And then some note content.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<h6>Header</h6>\n<p>And then some note content.</p>\n</section>"
    );
}

#[test]
fn h1_then_blockquote() {
    let text =
        "> # Header\n > And then some note content.\n\n> This is quoted.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<section class=\"note\" aria-role=\"note\">\n<h1>Header</h1>\n<p>And then some note content.</p>\n</section>\n<blockquote>\n<p>This is quoted.</p>\n</blockquote>\n"
    );
}

#[test]
fn blockquote_then_h1_note() {
    let text =
        "> This is quoted.\n\n> # Header\n > And then some note content.";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<blockquote>\n<p>This is quoted.</p>\n</blockquote>\n<section class=\"note\" aria-role=\"note\">\n<h1>Header</h1>\n<p>And then some note content.</p>\n</section>"
    );
}

#[test]
fn blockquote_with_strong() {
    let text = "> **Bold text in a paragraph.**";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<blockquote>\n<p><strong>Bold text in a paragraph.</strong></p>\n</blockquote>\n"
    );
}

#[test]
fn normal_table() {
    let text = "| Header 1 | Header 2 |\n| -------- | -------- |\n| Text 123 | More 456 |";
    let processed = rewrite(text);

    assert_eq!(
        processed,
        "|Header 1|Header 2|\n|--------|--------|\n|Text 123|More 456|",
        "It strips some whitespace but otherwise leaves the table intact."
    );
}

#[test]
fn table_in_note() {
    let text = "> Note: table stuff.\n\n| Header 1 | Header 2 |\n| -------- | -------- |\n| Text 123 | More 456 |";
    let processed = rewrite(text);

    assert_eq!(
        processed,
        "\n\n<section class=\"note\" aria-role=\"note\">\n\nNote: table stuff.\n\n</section>\n\n|Header 1|Header 2|\n|--------|--------|\n|Text 123|More 456|",
        "It adds the note markup but leaves the table untouched, to be rendered as Markdown."
    );
}

#[test]
fn table_in_quote() {
    let text = "> A table.\n\n| Header 1 | Header 2 |\n| -------- | -------- |\n| Text 123 | More 456 |";
    let processed = rewrite(text);
    assert_eq!(
        render_markdown(&processed),
        "<blockquote>\n<p>A table.</p>\n</blockquote>\n<table><thead><tr><th>Header 1</th><th>Header 2</th></tr></thead><tbody>\n<tr><td>Text 123</td><td>More 456</td></tr>\n</tbody></table>\n",
        "It renders blockquotes with nested tables as expected."
    );
}

fn render_markdown(text: &str) -> String {
    let parser = crate::parser(text);
    let mut buf = String::new();
    pulldown_cmark::html::push_html(&mut buf, parser);
    buf
}
