## Extending Cargo with Custom Commands

Cargo is designed so you can extend it with new subcommands without having to
modify Cargo itself. If a binary in your `$PATH` is named `cargo-something`,
you can run it as if it were a Cargo subcommand by running `cargo something`.
Custom commands like this are also listed when you run `cargo --list`. Being
able to `cargo install` extensions and then run them just like the built-in
Cargo tools is a super convenient benefit of Cargo’s design!

## Summary

Sharing code with Cargo and crates.io is part of what makes the Rust ecosystem
useful for many different tasks. Rust’s standard library is small and stable,
but crates are easy to share, use, and improve on a timeline different from the
language itself. Don’t be shy about sharing code that’s useful to you on
Crates.io; it’s likely that it will be useful to someone else as well!
