# AGENTS.md - Rust Kitabı Türkçe Çevirisi

Bu proje Resmi Rust Kitabı'nın (`The Rust Programming Language`) Türkçe çevirisini
içerir. Amaç: kitabı Türkçeye çevirip GitHub Pages üzerinden herkese ücretsiz
sunmak.

- Orijinal kitap: https://doc.rust-lang.org/book/
- Çeviri : https://rust-kitabi-turkce.github.io/rust-turkce-kitap/

## YAPI

- `original/src/*.md` -> Orijinal İngilizce kaynak (ASLA değiştirme)
- `src/*.md` -> Türkçe çeviri (buraya yazılır, buradan build edilir)
- `listings/` -> Kod örnekleri (`{{#rustdoc_include}}` ile `src/`'den çekilir). SADECE string/yorum çevirisi yapılır (aşağıdaki kurallara bak).
- `packages/mdbook-trpl/` -> mdBook preprocessor'ları (build için gerekli). ASLA değiştirme.
- `glossary.json` -> Çeviri sözlüğü. Terim kararları burada.
- `.github/workflows/deploy.yml` -> main'e push olunca build + GitHub Pages deploy'u.
- `book/` -> Build çıktısı (geçici, gitignore'da).

## ÇEVİRİ SÜRECİ

- `original/src/chXX-YY-*.md` dosyasını okuyup `src/chXX-YY-*.md`'ye Türkçe çevir.
  Kritik: `src/SUMMARY.md`'deki başlıkları da Türkçeye çevir (dosya adları ve
  sıralama DEĞİŞMEZ).
- Her çeviriden sonra, o kısmın çevirisi tamamen bittiğinde `ceviri-durumu.md`'ye ekle
- Bir bölümü çevirirken onun kod örneklerini de Türkçeye çevir. (listings klasörü altında). Kod örnekleri için kurallar aşağıda.

### ASLA ÇEVİRME / DOKUNMA

- ` ``` ` ile çevrili Rust kodunun YAPISI: satırlar, ifadeler, fonksiyon çağrıları, operatörler BİREBİR korunur
- `{{#rustdoc_include}}`, `{{#include}}`, `{{#playground}}` satırları (BİREBİR korunur)
- Teknik adlar: `cargo`, `rustc`, `rustfmt`, `mdbook`, `Cargo.toml`, `main.rs` vb.
- Değişken adları, fonksiyon adları, tür adları, struct/enum/trait adları, metot adları, alan adları, `let`/`fn`/`impl` gibi anahtar kelimeler
- Teknik terimler (kod içinde geçse bile): `ownership`, `borrow`, `lifetime`, `trait`, `enum` vb. olduğu gibi kalır
- İngilizce `CODE_x` / `Listing x-x` etiketleri (çevirme, silme)
- `<span class="filename">...</span>` içeriği (dosya adı kalır, etiket çevrilebilir)
- URL'ler ve `[text](hedef)` bağlantılarının hedef kısmı (anchor'ların görünen metni çevrilir, hedef DEĞİŞMEZ)
- `<img>` etiketlerinin `src`/`class` öznitelikleri (`alt` metni çevrilebilir)

### ÇEVİR

- Düz anlatım metinleri, bölüm başlıkları (`#`, `##`), tablo metinleri, `<img alt="...">`
- `src/SUMMARY.md` başlıkları (Türkçe, ama aynı sıra ve dosya adları)

### KOD İÇİNDEKİ STRING ve YORUM ÇEVİRİSİ

Kod örneklerinde **kodun amacını/çalışma mantığını DEĞİŞTİRMEDEN** şunlar Türkçeye çevrilir:

- String literal'ler ve bunların basıldığı çıktılar: `println!("Sayıyı tahmin et!")`, `println!("Tebrikler, bildin!")` vb.
- Yorum satırları: `// ...`, `/* ... */`, `/// ...` (dokümantasyon yorumları)
- `panic!`, `assert!`, `assert_eq!`, `unwrap_or_else`, `expect`, `eprintln!` gibi hata/çıktı mesajlarındaki kullanıcıya görünen metinler
- Çevrilen string'lerin karşılığı olan komut çıktıları (`output.txt` dosyaları) DA çevrilir ve kodla TUTARLI olur

**ASLA çevrilmeyen (kod davranışını etkilediği için):**

- Değişken/tür/fonksiyon/alan adları, `let`, `fn`, `impl`, `match` gibi anahtar kelimeler
- String **karşılaştırmaları** ve girdi olarak kullanılan string'ler (ör. `if guess == "quit"`, `std::env::args`, dosya adları) — bunlar programın davranışını belirler, değiştirilemez
- İçinde string geçen ama anlamlı olan sabitler (`const MESSAGE: &str = ...`) adı ve kullanımı değişmez
- Gerçek derleyici/hata çıktıları (`error[E0308]` gibi) İngilizce kalır

Kural özeti: **String/yorum çevirisi kodun davranışını değiştirmemeli; görünen metni Türkçeleştirip teknik tanımlayıcıları (identifier) aynen korumalısın.**

## TERMİNOLOJİ

- **glossary.json'a %100 uy.** Terim bilinmiyorsa sözlüğe uygun girdi ekleme — önce sor.
- İlk kullanım: `sahiplik (ownership)` -> sonraki kullanımlarda yalnızca `sahiplik`
- Kod içinde identifier olarak geçen terimler (ör. `ownership` değişken adı): olduğu gibi kalır. Kod yorumu/string'inde geçiyorsa sözlükteki Türkçe karşılığıyla çevrilir.

## TON ve STİL

- "Sen" dili: okuyucuya doğrudan hitap et
- Doğal, akıcı Türkçe; birebir kelime kelime çevirme
- Türkçe yazım kurallarına dikkat et (de/da, ki ayrımı, noktalama)
- Bilimsel/teknik metin dili: resmi ama yapay olmayan bir ton

## BUILD / DEPLOY

- Lokal: `mdbook build` (çıktı `book/`)
- main'e her push'ta GitHub Actions kitabı build edip `gh-pages` branch'ine atar.
- Deploy sonrası kontrol: `https://rust-kitabi-turkce.github.io/rust-turkce-kitap/`
