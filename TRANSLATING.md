# Translation guidelines

Please see the [CONTRIBUTING.md] file for general contribution guidelines.
This file describes about the translation workflow.

[CONTRIBUTING.md]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

## Translation workflow

### Preparation

The book uses [mdbook-i18n-helpers](https://github.com/google/mdbook-i18n-helpers) as a translation framework.
The following tools are required.

* GNU gettext utilities ( `msgmerge` and `msgcat` )
* mdbook-i18n-helpers ( `cargo install mdbook-i18n-helpers` )

### Creating and Updating Translations

Please see the [mdbook-i18n-helpers USAGE](https://github.com/google/mdbook-i18n-helpers/blob/main/i18n-helpers/USAGE.md) file for the detailed usage of mdbook-i18n-helpers.
The summarized command list is below:

#### Generating a message template

The generated message templete `po/messages.pot` is required to create or update translations.

```bash
MDBOOK_OUTPUT='{"xgettext": {"pot-file": "messages.pot"}}' \
  mdbook build -d po
```

#### Creating a new translation resource

`xx` is [ISO 639](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) language code.

```bash
msginit -i po/messages.pot -l xx -o po/xx.po
```

#### Updating the exising translation resource

```bash
msgmerge --update po/xx.po po/messages.pot
```

### Editing translation resources

After generating a translation resource `po/xx.po`, you can write translation messages in `msgstr` entry of `po/xx.po`.
To build a translated book, the following command can be used.

```bash
MDBOOK_BOOK__LANGUAGE=xx mdbook build
MDBOOK_BOOK__LANGUAGE=xx mdbook serve
```

### Add a language entry

Please add a language entry in `.github/workflows/main.yml`, `theme/index.hbs`, and `src/bootstrap/src/core/build_steps/doc.rs` in [rust-lang/rust](https://github.com/rust-lang/rust) like below:

* `main.yml`

```yml
env:
  # Update the language picker in index.hbs to link new languages.
  LANGUAGES: xx yy zz
```

* `index.hbs`

```html
<ul id="language-list" class="theme-popup" aria-label="Languages" role="menu">
  <li role="none"><button role="menuitem" class="theme">
      <a id="en">English</a>
  </button></li>
  <li role="none"><button role="menuitem" class="theme">
      <a id="xx">XX language</a>
  </button></li>
  <li role="none"><button role="menuitem" class="theme">
      <a id="yy">YY language</a>
  </button></li>
  <li role="none"><button role="menuitem" class="theme">
      <a id="zz">ZZ language</a>
  </button></li>
</ul>
```

* `src/bootstrap/src/core/build_steps/doc.rs` in [rust-lang/rust](https://github.com/rust-lang/rust)

```rust
// build book
builder.ensure(RustbookSrc {
    target,
    name: "book".to_owned(),
    src: absolute_path.clone(),
    parent: Some(self),
    languages: vec!["xx", "yy", "zz"],
});
```
