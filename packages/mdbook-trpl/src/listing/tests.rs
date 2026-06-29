use super::*;

/// Note: This inserts an additional backtick around the re-emitted code.
/// It is not clear *why*, but that seems to be an artifact of the rendering
/// done by the `pulldown_cmark_to_cmark` crate.
#[test]
fn default_mode_works() {
    let result = rewrite_listing(
        r#"<Listing number="1-2" caption="A write-up which *might* include inline Markdown like `code` etc." file-name="src/main.rs">

```rust
fn main() {}
```

</Listing>"#,
        Mode::Default,
    );

    assert_eq!(
        &result.unwrap(),
        r##"<figure class="listing" id="listing-1-2">
<span class="file-name">Filename: src/main.rs</span>

````rust
fn main() {}
````

<figcaption><a href="#listing-1-2">Listing 1-2</a>: A write-up which <em>might</em> include inline Markdown like <code>code</code> etc.</figcaption>
</figure>"##
    );
}

#[test]
fn simple_mode_works() {
    let result = rewrite_listing(
        r#"Leading text.

<Listing number="1-2" caption="A write-up which *might* include inline Markdown like `code` etc." file-name="src/main.rs">

```rust
fn main() {}
```

</Listing>

Trailing text."#,
        Mode::Simple,
    );

    assert_eq!(
        &result.unwrap(),
        r#"Leading text.

src/main.rs

```rust
fn main() {}
```

Listing 1-2: A write-up which *might* include inline Markdown like `code` etc.

Trailing text."#
    );
}

#[test]
fn listing_with_embedded_angle_brackets() {
    let result = rewrite_listing(
        r#"<Listing number="34-5" caption="This has a `Box<T>` in it.">

```rust
fn get_a_box_of<T>(t: T) -> Box<T> {
    Box::new(T)
}
```

</Listing>"#,
        Mode::Default,
    );

    assert_eq!(
        &result.unwrap(),
        r##"<figure class="listing" id="listing-34-5">

````rust
fn get_a_box_of<T>(t: T) -> Box<T> {
    Box::new(T)
}
````

<figcaption><a href="#listing-34-5">Listing 34-5</a>: This has a <code>Box&lt;T&gt;</code> in it.</figcaption>
</figure>"##
    );
}

#[test]
fn actual_listing() {
    let result = rewrite_listing(
        r#"Now open the *main.rs* file you just created and enter the code in Listing 1-1.

<Listing number="1-1" file-name="main.rs" caption="A program that prints `Hello, world!`">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

Save the file and go back to your terminal window"#,
        Mode::Default,
    );

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        r##"Now open the *main.rs* file you just created and enter the code in Listing 1-1.

<figure class="listing" id="listing-1-1">
<span class="file-name">Filename: main.rs</span>

````rust
fn main() {
    println!("Hello, world!");
}
````

<figcaption><a href="#listing-1-1">Listing 1-1</a>: A program that prints <code>Hello, world!</code></figcaption>
</figure>

Save the file and go back to your terminal window"##
    );
}

#[test]
fn no_filename() {
    let result = rewrite_listing(
        r#"This is the opening.

<Listing number="1-1" caption="This is the caption">

```rust
fn main() {}
```

</Listing>

This is the closing."#,
        Mode::Default,
    );

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        r##"This is the opening.

<figure class="listing" id="listing-1-1">

````rust
fn main() {}
````

<figcaption><a href="#listing-1-1">Listing 1-1</a>: This is the caption</figcaption>
</figure>

This is the closing."##
    );
}

#[test]
fn without_number() {
    let result = rewrite_listing(
        r#"<Listing file-name="src/main.rs">

```rust
fn main() {}
```

</Listing>"#,
        Mode::Default,
    );

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        r#"<figure class="listing">
<span class="file-name">Filename: src/main.rs</span>

````rust
fn main() {}
````

</figure>"#
    );
}

#[test]
fn with_unsupported_attr_name() {
    let result = rewrite_listing(
        "<Listing invalid-attr>

```rust
fn main() {}
```

</Listing>",
        Mode::Default,
    );

    assert_eq!(
        result,
        Err(String::from("Unsupported attribute name: 'invalid-attr'"))
    )
}

#[test]
fn with_unsupported_attr_name_with_arg() {
    let result = rewrite_listing(
        r#"<Listing invalid-attr="123">

```rust
fn main() {}
```

</Listing>"#,
        Mode::Default,
    );

    assert_eq!(
        result,
        Err(String::from("Unsupported attribute name: 'invalid-attr'"))
    )
}

#[cfg(test)]
mod missing_value {
    use super::*;

    #[test]
    fn for_number() {
        let result = rewrite_listing(
            r#"<Listing number>

```rust
fn main() {}
```

</Listing>"#,
            Mode::Default,
        );

        assert_eq!(
            result,
            Err(String::from("Missing value for attribute: 'number'"))
        )
    }

    #[test]
    fn for_caption() {
        let result = rewrite_listing(
            r#"<Listing caption>

```rust
fn main() {}
```

</Listing>"#,
            Mode::Default,
        );

        assert_eq!(
            result,
            Err(String::from("Missing value for attribute: 'caption'"))
        )
    }

    #[test]
    fn for_file_name() {
        let result = rewrite_listing(
            r#"<Listing file-name>

```rust
fn main() {}
```

</Listing>"#,
            Mode::Default,
        );

        assert_eq!(
            result,
            Err(String::from("Missing value for attribute: 'file-name'"))
        )
    }
}
