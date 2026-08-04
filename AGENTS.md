# AGENTS.md - Rust Kitabı Türkçe Çevirisi

Bu proje Resmi Rust Kitabı'nın (`The Rust Programming Language`) Türkçe çevirisini
içerir. Amaç: kitabı Türkçeye çevirip GitHub Pages üzerinden herkese ücretsiz
sunmak.

- Orijinal kitap: https://doc.rust-lang.org/book/
- Çeviri : https://rust-kitabi-turkce.github.io/rust-turkce-kitap/

## YAPI

- `original/src/*.md` -> Orijinal İngilizce kaynak (ASLA değiştirme)
- `src/*.md` -> Türkçe çeviri (buraya yazılır, buradan build edilir)
- `listings/` -> Kod örnekleri (`{{#rustdoc_include}}` ile `src/`'den çekilir). ASLA değiştirme.
- `packages/mdbook-trpl/` -> mdBook preprocessor'ları (build için gerekli). ASLA değiştirme.
- `glossary.json` -> Çeviri sözlüğü. Terim kararları burada.
- `.github/workflows/deploy.yml` -> main'e push olunca build + GitHub Pages deploy'u.
- `book/` -> Build çıktısı (geçici, gitignore'da).

## ÇEVİRİ SÜRECİ

`original/src/chXX-YY-*.md` dosyasını okuyup `src/chXX-YY-*.md`'ye Türkçe çevir.
Kritik: `src/SUMMARY.md`'deki başlıkları da Türkçeye çevir (dosya adları ve
sıralama DEĞİŞMEZ).

### ASLA ÇEVİRME / DOKUNMA

- Kod blokları: ` ``` ` ile çevrili Rust kodu, `{{#rustdoc_include}}`, `{{#include}}`, `{{#playground}}` satırları (BİREBİR korunur)
- `listings/` içindeki hiçbir dosya (kod örnekleri, `output.txt`)
- Teknik adlar: `cargo`, `rustc`, `rustfmt`, `mdbook`, `Cargo.toml`, `main.rs` vb.
- Kod içindeki string'ler, yorum satırları, fonksiyon/değişken/tür adları, komut çıktıları
- İngilizce `CODE_x` / `Listing x-x` etiketleri (çevirme, silme)
- `<span class="filename">...</span>` içeriği (dosya adı kalır, etiket çevrilebilir)
- URL'ler ve `[text](hedef)` bağlantılarının hedef kısmı (anchor'ların görünen metni çevrilir, hedef DEĞİŞMEZ)
- `<img>` etiketlerinin `src`/`class` öznitelikleri (`alt` metni çevrilebilir)

### ÇEVİR

- Düz anlatım metinleri, bölüm başlıkları (`#`, `##`), tablo metinleri, `<img alt="...">`
- `src/SUMMARY.md` başlıkları (Türkçe, ama aynı sıra ve dosya adları)

## TERMİNOLOJİ

- **glossary.json'a %100 uy.** Terim bilinmiyorsa sözlüğe uygun girdi ekleme — önce sor.
- İlk kullanım: `sahiplik (ownership)` -> sonraki kullanımlarda yalnızca `sahiplik`
- Kod içinde geçen terimler (ör. `ownership`): olduğu gibi kalır.

## TON ve STİL

- "Sen" dili: okuyucuya doğrudan hitap et
- Doğal, akıcı Türkçe; birebir kelime kelime çevirme
- Türkçe yazım kurallarına dikkat et (de/da, ki ayrımı, noktalama)
- Bilimsel/teknik metin dili: resmi ama yapay olmayan bir ton

## BUILD / DEPLOY

- Lokal: `mdbook build` (çıktı `book/`)
- main'e her push'ta GitHub Actions kitabı build edip `gh-pages` branch'ine atar.
- Deploy sonrası kontrol: `https://rust-kitabi-turkce.github.io/rust-turkce-kitap/`

