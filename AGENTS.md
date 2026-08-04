# AGENTS.md - Rust Book TR -

Bu proje Resmi Rust Kitabı'nın Türkçe çevirisini içerir. (https://doc.rust-lang.org/book/)


## YAPI

- `original/src/*.md` -> Orijinal İngilizce (dokunma)
- `src/*.md` -> Türkçe çeviri (burası build edilir)

## ÇEVİRİ KURALLARI

- glossary.json'a %100 uy
- **CODE_x** ASLA çevirme/silme
- İlk kullanım: sahiplik (ownership) -> sonra sahiplik
- Ton: sen dili
- Yorum satırlarını bile çevirme

## OPENCODE KOMUTLARI

- /cevir ch04-01 -> direkt çevir
- /kontrol ch04-01 -> validate
