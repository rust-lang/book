use assert_cmd::Command;

#[test]
fn supports_html_renderer() {
    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(["supports", "html"])
        .ok();
    assert!(cmd.is_ok());
}

#[test]
fn errors_for_other_renderers() {
    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(["supports", "total-nonsense"])
        .ok();
    assert!(cmd.is_err());
}

// It would be nice to add an actual fixture for an mdbook, but doing *that* is
// going to be a bit of a pain, and what I have should cover it for now.
