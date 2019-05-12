## Extending Cargo with Custom Commands

Cargo is designed so you can extend it with new subcommands without having to
modify Cargo. If a binary in your `$PATH` is named `cargo-something`, you can
run it as if it was a Cargo subcommand by running `cargo something`. Custom
commands like this are also listed when you run `cargo --list`. Being able to
use `cargo install` to install extensions and then run them just like the
built-in Cargo tools is a super convenient benefit of Cargo’s design!

## Summary

Sharing code with Cargo and [crates.io](https://crates.io)<!-- ignore --> is
part of what makes the Rust ecosystem useful for many different tasks. Rust’s
standard library is small and stable, but crates are easy to share, use, and
improve on a timeline different from that of the language. Don’t be shy about
sharing code that’s useful to you on [crates.io](https://crates.io)<!-- ignore
-->; it’s likely that it will be useful to someone else as well!
