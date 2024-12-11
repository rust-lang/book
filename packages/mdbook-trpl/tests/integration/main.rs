mod note {
    use assert_cmd::Command;
    #[test]
    fn supports_html_renderer() {
        let cmd = Command::cargo_bin("mdbook-trpl-note")
            .unwrap()
            .args(["supports", "html"])
            .ok();
        assert!(cmd.is_ok());
    }

    #[test]
    fn errors_for_other_renderers() {
        let cmd = Command::cargo_bin("mdbook-trpl-note")
            .unwrap()
            .args(["supports", "total-nonsense"])
            .ok();
        assert!(cmd.is_err());
    }
}
