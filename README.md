# mdbook-trpl-note

This is a *very* simple [preprocessor][pre] for [mdBook][mdbook], focused specifically on the content of _The Rust Programming Language_  book. This preprocessor takes Markdown like this—

```markdown
> Note: This is some material we want to provide more emphasis for, because it
> is important in some way!

Some text.

> ## Some subject
>
> Here is all the important things to know about that particular subject.
```

—and rewrites the Markdown to this:

```html
<section class="note" aria-role="note">

This is some material we want to provide more emphasis for, because it is
important in some way!

</section>

Some text.

<section class="note" aria-role="note">

## Some subject

Here is all the important things to know about that particular subject.

</section>
```

This allows using the relatively standard Markdown convention of (incorrectly!) using blockquotes for “callouts” or “notes” like this, while still producing semantic HTML which conveys the actual intent.

> [!NOTE]
> This is *not* a full “admonition” preprocessor, and it is not remotely compliant with [the GitHub “alert” syntax][alerts]. It exists almost entirely for the sake of providing better semantic HTML for _The Rust Programming Language_ book with a minimum of disruption to existing workflows!
>
> You are probably better off using one of the other existing alert/admonition preprocessors:
>
> - [mdbook-alerts][mdbook-alerts]
> - [mdbook-admonish][mdbook-admonish]

[pre]: https://rust-lang.github.io/mdBook/format/configuration/preprocessors.html
[mdbook]: https://github.com/rust-lang/mdBook
[alerts]: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#alerts
[mdbook-alerts]: https://github.com/lambdalisue/rs-mdbook-alerts
[mdbook-admonish]: https://github.com/tommilligan/mdbook-admonish
