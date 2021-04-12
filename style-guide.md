# Style Guide

## Prose

* Lebih dipilih title case untuk judul chapter/section, contoh: `## Generating a Secret
  Number` lebih dipilih dibandingkan `## Generating a secret number`.
* Lebih dipilih cetak miring dibandingkan single quotes saat menyebutkan suatu istilah, contoh: `is an
  *associated function* of` lebih dipilih dibandingkan dengan `is an ‘associated function’ of`.
* Ketikan berbicara tentang suatu method di dalam prose, JANGAN mengikutsertakan parentheses, contoh:
  `read_line` lebih dipilih dibandingkan `read_line()`.
* Lakukan hard wrap 80 karakter.
* Lebih dipilih untuk tidak mencampur kode dan not-kode di dalam satu kata, contoh: ``Remember when we wrote
  `use std::io`?`` lebih dipilih dibandingkan dengan ``Remember when we `use`d `std::io`?``

## Code

* Tambahkan nama file sebelum blok markdown untuk membuat lebih jelas file yang mana yang sedang kita bicarakan, apabila memungkinkan.
* Pada saat membuat perubahan terhadap kode,  tolong disebutkan dengan jelas bagian mana dari kode yang dirubah dan yang mana masih tetap sama. Kami juga belum yakin bagai mana cara malakukan hal ini.
* Bagi line kode yang panjang sebisanya untuk menjaga kode kurang dari 80 karakter jika memungkinkan.
* Gunakan `bash` syntax highlighting untuk output block kode dari command line.

## Links

Pada saat semua script selesai:

* Jika ada link yang sebaiknya tidak ditampilkan, tandai untuk diabaikan. 
  * Ini mencakup semya link intra-book "Chapter XX", yang *sebaiknya* di-link untuk versi HTML
* Buat link intra-book dan stdlib API doc menjadi link relative sehingga mereka bekerja pada saat buku tersebut dibaca offline atau online di docs.rust-lang.org
* Gunakan link markdown dan harap selalu diingat bahwa mereka akan dirubah menjadi `text at
  *url*`, jadi sampaikan mereka dengan cara yang dapat dibaca dengan baik dalam format tersebut.
