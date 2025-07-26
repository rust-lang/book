## Yorumlar

Tüm programcılar kodlarının kolay anlaşılır olması için çaba gösterirler, ancak bazen
fazladan açıklama yapılması gerekebilir. Bu gibi durumlarda programcılar
kaynak kodlarına derleyicinin göz ardı edeceği ancak
kaynak kodunu okuyan kişilerin yararlı bulabileceği _yorumlar_ bırakırlar.

İşte basit bir yorum:

```rust
// hello, world
```

Rust'ta, deyimsel yorum stili bir yorumu iki eğik çizgi ile başlatır ve
yorumu satırın sonuna kadar devam eder. tek bir satırın ötesine uzanan yorumlar için, aşağıdaki gibi her satıra `//` eklemeniz gerekir:

```rust
// Yani burada karmaşık bir şey yapıyoruz, bunu yapmak için
// birden fazla yorum satırına ihtiyacımız olacak kadar uzun! Vay be! Umarım bu yorum
// neler olup bittiğini açıklar.
```

Yorumlar, kod içeren satırların sonuna da yerleştirilebilir:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Ancak daha sık olarak bu formatta kullanıldıklarını göreceksiniz; yorum,
adresinde, açıklama yaptığı kodun üzerinde ayrı bir satırda yer alır:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust ayrıca başka bir yorum türüne, dokümantasyon yorumlarına da sahiptir; bunları
Bölüm 14'ün [“Crates.io'da bir Sandık Yayınlama”][yayınlama]<!-- ignore -->
kısmında tartışacağız.

[yayınlama]: ch14-02-publishing-to-crates-io.html
