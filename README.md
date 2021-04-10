# The Rust Programming Language

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)

Repositori ini memuat sumber buku "The Rust Programming Language".

[Buku ini tersedia dalam format dead-tree dari No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust

Anda juga dapat membaca buku ini online secara gratis. Harap diperhatikan bahwa buku ini dirilis dengan status Rust terbaru [stable], [beta], atau [nightly]. Harap diperhatikan kalau ada masalah dalam versi tersebut, kemungkinan sudah diatasi di repositori ini, karena rilis tersebut lebih jarang diupdate.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

Silakan lihat [releases] untuk mendownload hanya kode dari semua daftar kode yang ada di buku.

[releases]: https://github.com/rust-lang/book/releases

## Requirements

Membuat buku memerlukan [mdBook], yang idealnya mengacu pada versi yang sama dengan
rust-lang/rust yang digunakan dalam [file ini][rust-mdbook]. Untuk mendapatkannya :

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --vers [version-num]
```

## Building

Untuk membuat buku, ketik:

```bash
$ mdbook build
```

Hasilnya akan berada di dalam sub-direktori `book`. Untuk memeriksanya, bukan file nya menggunakan browser Anda.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Untuk menjalankan test:

```bash
$ mdbook test
```

## Contributing

Kami sangat senang Anda membantu! Silakan lihat [CONTRIBUTING.md][contrib] untuk mempelajari apa saja jenis kontribusi yang kami butuhkan.

[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

### Translations

Kami sangat senang Anda menterjemahkan buku! Silakan lihat labal [Translations] untuk bergabung dalam usaha yang saat ini sedang dalam progres. Buka "new issue" untuk memulai mengerjakan bahasa baru! Kami sedang menunggu [mdbook support] untuk berbagai bahasa sebelum kami melakukan merge, tapi silakan Anda mulai saja!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang-nursery/mdBook/issues/5

## Spellchecking

Untuk melakukan scan terhadap file sumber dalam upaya mengetahui kesalahan spelling, Anda dapat menggunakan script `spellcheck.sh`
yang tersedia di dalam direktori `ci`. Dibutuhkan kamus untuk kata-kata yang valid yang mana sudah disediakan di dalam `ci/dictionary.txt`. Jika script yang disediakan menyebabkan false positive (seperti misalnya, Anda menggunakan kata `BTreeMap` yang dianggap invalid oleh script),
Anda perlu menambahkan kata tersebut secara manual ke dalam `ci/dictionary.txt` (tolong dijaga urutannya tetap konsisten).
