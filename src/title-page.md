# Lenguaje de programación Rust

*por Steve Klabnik y Carol Nichols, con contribuciones de la comunidad de la roya*

Esta versión del texto asume que estás usando Rust 1.50 o posterior con
`edición="2018"` en *Cargo.toml* de todos los proyectos para usar Rust 2018 Edition
de Rust. Ver la sección ["Instalación" del capítulo 1][install]<!-- ignore -->
para instalar o actualizar Rust, y ver el nuevo [Apéndice E][editions]<!-- ignore
--> para obtener información sobre las ediciones.

La edición 2018 del lenguaje Rust incluye una serie de mejoras que
hacen que Rust sea más ergonómico y fácil de aprender. Esta iteración del libro
contiene una serie de cambios para reflejar esas mejoras:

- Chapter 7, “Managing Growing Projects with Packages, Crates, and Modules,”
  has been mostly rewritten. The module system and the way paths work in the
  2018 Edition were made more consistent.
- Chapter 10 has new sections titled “Traits as Parameters” and “Returning
  Types that Implement Traits” that explain the new `impl Trait` syntax.
- Chapter 11 has a new section titled “Using `Result<T, E>` in Tests” that
  shows how to write tests that use the `?` operator.
- The “Advanced Lifetimes” section in Chapter 19 was removed because compiler
  improvements have made the constructs in that section even rarer.
- The previous Appendix D, “Macros,” has been expanded to include procedural
  macros and was moved to the “Macros” section in Chapter 19.
- Appendix A, “Keywords,” also explains the new raw identifiers feature that
  enables code written in the 2015 Edition and the 2018 Edition to interoperate.
- Appendix D is now titled “Useful Development Tools” and covers recently
  released tools that help you write Rust code.
- We fixed a number of small errors and imprecise wording throughout the book.
  Thank you to the readers who reported them!

Note that any code in earlier iterations of *The Rust Programming Language*
that compiled will continue to compile without `edition="2018"` in the
project’s *Cargo.toml*, even as you update the Rust compiler version you’re
using. That’s Rust’s backward compatibility guarantees at work!

The HTML format is available online at
[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)
and offline with installations of Rust made with `rustup`; run `rustup docs
--book` to open.

This text is available in [paperback and ebook format from No Starch
Press][nsprust].

[install]: ch01-01-installation.html
[editions]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust
