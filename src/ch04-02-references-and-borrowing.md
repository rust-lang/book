## References and Borrowing

First, a code snippet:

```rust
fn main() {
  let mut s = String::new();
  s.push_str("WE MADE IT!");
}
```

Then, an editor:

<pre class="aquascope">
fn main() {
  let s = String::from("Hello");
  println!("{}", s.len());
  s.push_str(" world");
  s.into_bytes();

  let mut s2 = String::from("Hello");
  s2.push_str(" world");
}
</pre>