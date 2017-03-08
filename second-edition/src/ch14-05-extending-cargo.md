## Extending Cargo with Custom Commands

Cargo is designed to be extensible with new subcommands without having to
modify Cargo itself. If a binary in your `$PATH` is named `cargo-something`,
you can run it as if it were a Cargo subcommand by running `cargo something`.
Custom commands like this are also listed when you run `cargo --list`. It's
convenient to `cargo install` extensions to Cargo then be able to run them just
like the built-in Cargo tools!

## Summary

Sharing code with Cargo and crates.io is part of what makes the Rust ecosystem
useful for many different tasks. Rust's standard library is small and stable,
but crates are easy to share, use, and improve on a different timeline than the
language itself. Don't be shy about sharing code that's useful to you on
crates.io; it's likely that it will be useful to someone else as well!
