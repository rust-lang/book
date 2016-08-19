## Comments

All programmers strive to make their code easy to understand, but sometimes
extra explanation is warranted. In these cases, we leave notes in our source
code that the compiler will ignore but people reading the source code may find
useful. These notes are called *comments*.

Here’s a simple comment:

```rust
// Hello, world.
```

In Rust, comments must start with two slashes and will last until the end of
the line. For comments that extend beyond a single line, you'll need to
include `//` on each line, like this:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Comments can also be placed at the end of lines of code:

Filename: src/main.rs

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.
}
```

But you’ll more often see them above, like so:

Filename: src/main.rs

```rust
fn main() {
    // I’m feeling lucky today.
    let lucky_number = 7;
}
```

That’s all there is to it. Comments are not particularly complicated.
