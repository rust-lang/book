# Comments

We strive to make our programs easy to understand, but sometimes, some extra
explanation is warranted. We can leave notes in our source code that the
compiler will ignore. These notes are called ‘comments’.

Here’s a comment:

```rust
// Hello, world.
```

Comments start with two slashes, and last until the end of the line. Larger
comments will need more lines:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Comments can also go at the end of lines:

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.
}
```

But you’ll more often see them above:

```rust
fn main() {
    // I’m feeling lucky today.
    let lucky_number = 7;
}
```

That’s all there is to it. Comments are not particularly complicated.
