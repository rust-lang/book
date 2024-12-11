use super::*;

#[test]
fn text_without_figures_is_ignored() {
    let actual = rewrite_figure("This is some basic text.").unwrap();
    assert_eq!(actual, "This is some basic text.");
}

#[test]
fn text_with_figure_replaces_it_with_simple_text() {
    let actual = rewrite_figure(
        r#"<figure>

<img src="http://www.example.com/some-image.jpg">

<figcaption>Figure 12-34: Look at this cool picture!</figcaption>

</figure>"#,
    )
    .unwrap();

    let expected = r#"

<img src="http://www.example.com/some-image.jpg">

Figure 12-34: Look at this cool picture!

"#;

    assert_eq!(actual, expected);
}

#[test]
fn unclosed_figure() {
    let result = rewrite_figure("<figure>");
    let actual = format!("{:?}", result.unwrap_err());
    assert_eq!(actual, "Unclosed `<figure>`");
}

#[test]
fn empty_caption() {
    let result = rewrite_figure(
        "<figure>
<figcaption></figcaption>
</figure>",
    );
    let actual = format!("{:?}", result.unwrap_err());
    assert_eq!(actual, "Missing caption in `<figcaption>`");
}

#[test]
fn unclosed_caption() {
    let result = rewrite_figure(
        "<figure>
<figcaption>
</figure>",
    );
    let actual = format!("{:?}", result.unwrap_err());
    assert_eq!(actual, "Unclosed `<figcaption>`");
}
