Please change the error output on page 242 to this:

```
warning: unused `std::result::Result` that must be used
  --> src/main.rs:17:5
   |
17 |     run(config);
   |     ^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
   = note: this `Result` may be an `Err` variant, which should be handled
```
