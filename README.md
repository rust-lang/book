# The Rust Programming Language

![Build Status](https://github.com/renomureza/rust-book-id/workflows/CI/badge.svg)

Repositori ini berisi sumber dari buku "The Rust Programming Language".

[Buku ini tersedia dalam bentuk _dead-tree_ dari No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Anda juga dapat membaca buku ini secara online gratis. Silakan lihat buku tersebut yang dikirim dalam rilis Rust [stable], [beta], atau [nighly] terbaru. Ketahuilah bahwa masalah dalam versi tersebut mungkin telah diperbaiki dalam repositori ini, karena rilis tersebut jarang diperbarui.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

Lihat [rilis] untu mengunduh hanya kode dari semua daftar kode yang muncul dalam buku.

[releases]: https://github.com/rust-lang/book/releases

## Persyaratan

Untuk mambangun buku membutuhkan [mdBook], idealnya versi yang sama dengan yang digunkaan rust-lang/rust dalam file ini [this file][rust-mdbook]. Untuk mendapatkannya:

[mdBook]: https://github.com/rust-lang-nursery/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --version <version_num>
```

## Membangun

Untuk membangun buku, jalankan perintah:

```bash
$ mdbook build src-id
```

Output akan berada di subdirektori `src-id/book`. Untuk memeriksanya, buka di browser web Anda.

_Firefox:_

```bash
$ firefox src-id/book/index.html                       # Linux
$ open -a "Firefox" src-id/book/index.html             # OS X
$ Start-Process "firefox.exe" .\src-id\book\index.html # Windows (PowerShell)
$ start firefox.exe .\src-id\book\index.html           # Windows (Cmd)
```

_Chrome:_

```bash
$ google-chrome src-id/book/index.html                 # Linux
$ open -a "Google Chrome" src-id/book/index.html       # OS X
$ Start-Process "chrome.exe" .\src-id\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\src-id\book\index.html            # Windows (Cmd)
```

Untuk menjalankan dalam mode pengembangan (untuk melihat langsung hasil selama penerjemahan):

```bash
$ mdbook serve src-id
```

Untuk menjalankan pengujian:

```bash
$ mdbook test src-id
```

## Contributing

Kami akan senang bantuan Anda! Silakan lihat [CONTRIBUTING.md][contrib] untuk mempelajari tentang jenis kontribuasi yang kami cari.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Karena buku tersebut [dicetak][nostarch], dan karena kami ingin agar versi online buku tersebut tetap dekat dengan versi cetaknya jika memungkinkan, mungkin perlu waktu lebih lama dari biasanya bagi kami untuk menangani masalah atau permintaan penarikan Anda.

Sejauh ini kami telah melakukan revisi besar bertepatan dengan [Rust
Editions](https://doc.rust-lang.org/edition-guide/). Di antara revisi yang besar itu, kami hanya akan memperbaiki kesalahan. Jika masalah atau _pull request_ Anda tidak benar-benar memperbaiki kesalahan, itu mungkin menunggu sampai waktu berikutnya kami mengerjakan revisi besar: perkirakan dalam urutan bulan atau tahun. Terimakasih atas kesabaran Anda!

### Terjemahan

Kami akan senang membantu [menerjemahkan] buku! Lihat label Terjemahan untuk bergabung dalam upaya yang sedang berlangsung. Buka terbitan baru untuk mulai mengerjakan bahasa baru! Kami sedang menunggu [dukungan mdbook] untuk beberapa bahasa sebelum kami menggabungkannya, tetapi jangan ragu untuk memulai!

[menerjemahkan]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[dukungan mdbook]: https://github.com/rust-lang-nursery/mdBook/issues/5

## Pemeriksaan ejaan

Untuk memindai file sumber dari kesalahan ejaan, Anda dapat menggunakan skrip `spellcheck.sh` yang tersedia di direktori `ci`. Dibutuhkan kamus kata-kata yang valid, yang disediakan dalam `ci/dictionary.txt`. Jika skrip menghasilkan positif palsu (katakanlah, Anda menggunakan kata `BTreeMap` yang dianggap tidak valid oleh skrip), Anda perlu menambahkan kata ini ke `ci/dictionary.txt` (pertahankan urutan yang diurutkan untuk konsistensi).
