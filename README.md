# Bahasa Pemrograman Rust

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)

Repositori ini memuat sumber buku "The Rust Programming Language".

[Buku ini tersedia dalam bentuk cetak dan digital dari No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust

Anda juga dapat membaca buku ini gratis secara daring. Silahkan cek buku yang     tersedia dalam rilis [stable], [beta], atau [nightly] terbaru dari Rust. Harap diperhatikan kalau ada masalah dalam versi rilis buku tersebut, kemungkinan sudah diatasi di repositori ini, karena rilis tersebut jarang diperbarui.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

Silahkan lihat *[releases]* untuk mengunduh kode dari semua daftar kode yang ada di buku.

[releases]: https://github.com/rust-lang/book/releases

## Persyaratan

Membuat buku ini memerlukan [mdBook], idealnya gunakan versi yang sama     dengan  rust-lang/rust pada [berkas ini][rust-mdbook]. Caranya yaitu:

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --vers [version-num]
```

## Pembuatan

Untuk membuat buku ini, ketik:

```bash
$ mdbook build
```

Hasilnya akan berada di dalam sub-direktori `book`. Untuk memeriksanya, bukalah menggunakan peramban Anda.

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

## Kontribusi

Kami sangat senang Anda membantu! Silakan lihat [CONTRIBUTING.md][contrib] untuk mempelajari apa saja jenis kontribusi yang kami butuhkan.

[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md

### Penerjemahan

Kami sangat senang Anda mebantu menerjemahkan buku ini! Silakan lihat label [Translations] untuk bergabung dalam proses penerjemahan. Buatlah isu baru untuk memulai menerjemahkan dengan bahasa baru! Kami menantikan [dukungan mdbook] untuk berbagai bahasa sebelum kami menggabungkan semuanya, anda bisa mulai kapanpun!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[dukungan mdbook]: https://github.com/rust-lang-nursery/mdBook/issues/5

## Pemeriksaan Ejaan Kata

Untuk memeriksa kesalahan pengejaan pada berkas, Anda dapat menggunakan skrip `spellcheck.sh`
yang tersedia di dalam direktori `ci`. Skrip tersebut membutuhkan kamus untuk kata-kata baku yang mana sudah disediakan di `ci/dictionary.txt`. Jika skrip tersebut menyebabkan *false positive* atau positif palsu (seperti misalnya, Anda menggunakan kata `BTreeMap` yang dianggap tidak baku oleh skrip),
Anda perlu menambahkan kata tersebut secara manual ke dalam `ci/dictionary.txt` (Tolong dijaga urutannya agar tetap konsisten).
