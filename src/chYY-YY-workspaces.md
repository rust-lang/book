## Workspaces and Multiple Related Crates

Projects can define a *workspace*, which is a set of crates that will all share
the same *Cargo.lock* and output directory.

We weren’t technically precise about the difference between a *package* and a
*crate* until now, and casually people will use the words *package* and *crate*
interchangeably and everyone will understand.

But there are cases when it makes sense to have multiple crates in one project
because (insert reasons here). For example, (insert examples here, maybe
log/env_log in rust-lang-nursery?)

So let’s get more precise now. Earlier, we said:

> A *crate* is what we call a package of Rust code.

This definition is problematic because “package” does mean something specific
in terms of Cargo, but we used it in a general sense in that previous
definition.

Historically speaking, a “compilation unit” is the definition we use in Rust.
This comes from C and C++, which has three phases of going from “here’s my
source” to “here’s something I run”:

1. **Preprocessing**: this is where `cpp` (the c preprocessor, not “c plus
     plus”) does `#include` and all that jazz
2. **Compiling**: taking the output of preprocessing and turning it into an
     object file
3. **Linking**: taking one or many object files, putting them together, and
     getting an ELF binary or `.exe`

Rust doesn’t have a preprocessing step in this sense, and we call out to a
system linker, so `rustc` only really does step 2. So to be incredibly
technically correct, it’s “translation unit”, even though most people say
“compilation unit.“

In some sense, a crate is “that pile of source code you pass the root of to
`rustc`“, but that feels too circular to be good. “compilation unit” is
technically precise, but is gibberish to a lot of people. And a “package” is
“one or more crates”, which confusingly, gets wrapped up by `cargo package`
into a file which ends in `.crate`, but is secretly a tarball.

Whew!
