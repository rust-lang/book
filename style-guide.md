# Pedoman Penulisan

## Prosa

* Gunakan huruf kapital pada tiap awalan huruf dalam kata untuk penulisan judul dari bab/bagian, contoh: `## Menghasilkan Sebuah Angka Rahasia` lebih baik daripada `## Menghasilkan sebuah angka rahasia`.
* Gunakan huruf miring untuk menyebutkan sebuah istilah, contoh: `merupakan sebuah *associated function* dari` lebih baik daripada `merupakan sebuah
   'associated function' dari`.
* Ketikan membahas tentang sebuah *method* di dalam prosa, JANGAN mengikutsertakan tanda kurung, contoh:
  `read_line` lebih baik dibandingkan `read_line()`.
* Gunakan *hard wrap* dengan 80 karakter.
* Hindari mencampur kode dengan kata dalam satu kalimat. Contoh: ``Ingat ketika kita menulis `use std::io`?`` lebih baik daripada ``Ingat ketika kita menggunakan `use`d`std::io`?``

## Kode

* Tambahkan nama berkas sebelum blok *markdown* untuk memperjelas berkas mana yang dibahas bila memungkinkan.
* Pada saat membuat perubahan pada kode,  Perjelas bagian mana kode yang dirubah dan yang mana masih tetap sama. Kami juga belum yakin bagaimana cara malakukan hal ini.
* Pisahkan baris kode yang panjang agar jumlah kode kurang dari 80 karakter jika memungkinkan.
* Gunakan `bash` *syntax highlighting* ketika membuat blok kode hasil dari *command line*

## Tautan

Setelah semua skrip selesai:

* Jika ada tautan yang sebaiknya tidak ditampilkan, tandai untuk diabaikan. 
  * Ini mencakup semua tautan "Bab XX" dalam buku, yang mana hanya digunakan untuk versi HTML.
* Buat tautan dalam buku dan *stdlib API doc* menjadi relatif agar dapat berfungsi dengan baik saat buku dibaca secara luring atau daring di docs.rust-lang.org
* Gunakan tautan *markdown* dan perhatikan bahwa tautan tersebut akan berubah menjadi `teks pada *url*` saat dicetak, jadi gunakan kata yang mudah dibaca pada format tersebut.
