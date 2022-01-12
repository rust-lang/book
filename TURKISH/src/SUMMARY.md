# Rust Programlama Dili

[Rust Programlama Dili](title-page.md)
[Önsöz](foreword.md)
[Giriş](ch00-00-introduction.md)

## Başlarken

- [Başlarken](ch01-00-getting-started.md)
    - [Kurulum](ch01-01-installation.md)
    - [Merhaba, Dünya](ch01-02-hello-world.md)
    - [Merhaba, Cargo](ch01-03-hello-cargo.md)

- [Bir Tahmin Oyunu Programlamak](ch02-00-guessing-game-tutorial.md)

- [Ortak Programlama Kavramları](ch03-00-common-programming-concepts.md)
    - [Değişkenler ve Değişkenlik](ch03-01-variables-and-mutability.md)
    - [Veri Türleri](ch03-02-data-types.md)
    - [İşlevler](ch03-03-how-functions-work.md)
    - [Yorumlar](ch03-04-comments.md)
    - [Kontrol Akışı](ch03-05-control-flow.md)

- [Mülkiyeti Anlamak](ch04-00-understanding-ownership.md)
    - [Mülkiyet Nedir?](ch04-01-what-is-ownership.md)
    - [Referanslar ve Borçlanma](ch04-02-references-and-borrowing.md)
    - [Dilim Türü](ch04-03-slices.md)

- [İlişkili Verileri Yapılandırmak için Yapıları Kullanmak](ch05-00-structs.md)
    - [Yapıları Tanımlamak ve Örneklemek](ch05-01-defining-structs.md)
    - [Yapıları Kullanan Örnek Bir Program](ch05-02-example-structs.md)
    - [Metod Sözdizimi](ch05-03-method-syntax.md)

- [Sıralamalar ve Örüntü Eşleme](ch06-00-enums.md)
    - [Bir Sıralama Tanımlamak](ch06-01-defining-an-enum.md)
    - [Kontrol Akışı İşleci `match`](ch06-02-match.md)
    - [`if let` ile Kısa Kontrol Akışı](ch06-03-if-let.md)

## Temel Rust Okuryazarlığı

- [Büyüyen Projeleri Paketler, Sandıklar ve Modüller ile Yönetmek](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [Paketler ve Sandıklar](ch07-01-packages-and-crates.md)
    - [Kapsam ve Gizlilik Kontrolü İçin Modül Tanımlamak](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [Modül Ağacındaki Bir Öğeye Başvurmanın Yolları](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [`use` Anahtar Kelimesi ile Yolları Kapsama Getirmek](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [Modülleri Farklı Dosyalara Ayırmak](ch07-05-separating-modules-into-different-files.md)

- [Ortak Koleksiyonlar](ch08-00-common-collections.md)
    - [Değer Listelerini Vektör Kullanarak Depolamak](ch08-01-vectors.md)
    - [Dizgelerle UTF-8 Kodlu Metinleri Saklamak](ch08-02-strings.md)
    - [İlişkili Değerlere Sahip Anahtarları Eşleme Haritalarında Saklamak](ch08-03-hash-maps.md)

- [Hata Yönetimi](ch09-00-error-handling.md)
    - [`panic!` ile Kurtarılamaz Hatalar](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result` ile Kurtarılabilir Hatalar](ch09-02-recoverable-errors-with-result.md)
    - [`panic!` Olmak ya da `panic!` Olmamak](ch09-03-to-panic-or-not-to-panic.md)

- [Genellenmiş Türler, Özellikler ve Yaşam Süreleri](ch10-00-generics.md)
    - [Genellenmiş Veri Türleri](ch10-01-syntax.md)
    - [Özellikler: Paylaşılan Davranışı Tanımlamak](ch10-02-traits.md)
    - [Referansları Yaşam Süreleri ile  Doğrulamak](ch10-03-lifetime-syntax.md)

- [Otomatik Testler Yazmak](ch11-00-testing.md)
    - [Testler Nasıl Yazılır?](ch11-01-writing-tests.md)
    - [Testlerin Nasıl Çalıştırılacağını Denetlemek](ch11-02-running-tests.md)
    - [Test Organizasyonu](ch11-03-test-organization.md)

- [Bir I/O Projesi: Komut Satırı Programı Oluşturmak](ch12-00-an-io-project.md)
    - [Komut Satırı Argümanlarını Kabul Etmek](ch12-01-accepting-command-line-arguments.md)
    - [Bir Dosyayı Okumak](ch12-02-reading-a-file.md)
    - [Modülerlik ve Hata Yönetimini Geliştirmek İçin Yeniden Düzenlemek](ch12-03-improving-error-handling-and-modularity.md)
    - [Test Odaklı Geliştirme ile Kütüphane İşlevselliğini Artırmak](ch12-04-testing-the-librarys-functionality.md)
    - [Ortam Değişkenleriyle Çalışmak](ch12-05-working-with-environment-variables.md)
    - [Hata Mesajlarını Standart Çıktı Yerine Standart Hataya Yazmak](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Rust'a Göre Düşünmek

- [İşlevsel Dil Özellikleri: Yineleyiciler ve Kapamalar](ch13-00-functional-features.md)
    - [Kapamalar: Ortam Değişkenlerini Yakalayabilen İsimsiz İşlevler](ch13-01-closures.md)
    - [Yineleyiciler ile Bir Dizi Öğeyi İşlemek](ch13-02-iterators.md)
    - [I/O Projemizi Geliştirmek](ch13-03-improving-our-io-project.md)
    - [Performans Karşılaştırması: Döngüler vs. Yineleyiciler](ch13-04-performance.md)

- [Cargo ve Crates.io Hakkında Daha Fazla Bilgi](ch14-00-more-about-cargo.md)
    - [Sürüm Profilleriyle Derlemeleri Özelleştirmek](ch14-01-release-profiles.md)
    - [Bir Sandığı Crates.io Üzerinde Yayınlamak](ch14-02-publishing-to-crates-io.md)
    - [Cargo Çalışma Alanları](ch14-03-cargo-workspaces.md)
    - [Crates.io Üzerindeki İkili Sandıkları `cargo install` Komutuyla Yüklemek](ch14-04-installing-binaries.md)
    - [Özel Komutlarla Cargo Olanaklarını Genişletmek](ch14-05-extending-cargo.md)

- [Akıllı İşaretçiler](ch15-00-smart-pointers.md)
    - [Heap Üzerindeki Verilere İşaret Etmek İçin `Box<T>` Kullanmak](ch15-01-box.md)
    - [`Deref` Özelliği ile Akıllı İşaretçilere Normal Referanslar Gibi Davranmak](ch15-02-deref.md)
    - [`Drop` Özelliği ile Kodu Temizlik Amaçlı Çalıştırmak](ch15-03-drop.md)
    - [`Rc<T>`, Referans Sayılı Akıllı İşaretçi](ch15-04-rc.md)
    - [`RefCell<T>` ve İç Değişkenlik Modeli](ch15-05-interior-mutability.md)
    - [Referans Çevrimleri Bellek Sızıntısına Yol Açabilir](ch15-06-reference-cycles.md)

- [Korkusuz Eşzamanlılık](ch16-00-concurrency.md)
    - [Eşzamanlı Kod Çalıştırmak İçin İşlikleri Kullanmak](ch16-01-threads.md)
    - [İşlikler Arasında Veri Aktarmak Amacıyla Mesajlaşma](ch16-02-message-passing.md)
    - [Durum Paylaşımlı Eşzamanlılık](ch16-03-shared-state.md)
    - [`Sync` and `Send` Özellikleri ile Genişletilebilir Eşzamanlılık](ch16-04-extensible-concurrency-sync-and-send.md)

- [Rust'ın Nesne Yönelimli Programlama Özellikleri](ch17-00-oop.md)
    - [Nesne Yönelimli Dillerin Özellikleri](ch17-01-what-is-oo.md)
    - [Farklı Türden Değerlere İzin Veren Özellik Nesnelerini Kullanmak](ch17-02-trait-objects.md)
    - [Nesne Yönelimli Tasarım Kalıbı Uygulamak](ch17-03-oo-design-patterns.md)

## İleri Seviye Konular

- [Örüntü ve Eşleme](ch18-00-patterns.md)
    - [Örüntüler Her Yerde Kullanılabilir](ch18-01-all-the-places-for-patterns.md)
    - [Çürütülebilirlik: Bir Örüntünün Eşleşmeme İhtimali](ch18-02-refutability.md)
    - [Örüntü Sözdizimi](ch18-03-pattern-syntax.md)

- [Gelişmiş Özellikler](ch19-00-advanced-features.md)
    - [Emniyetsiz Rust](ch19-01-unsafe-rust.md)
    - [Gelişmiş Özellikler](ch19-03-advanced-traits.md)
    - [Gelişmiş Türler](ch19-04-advanced-types.md)
    - [Gelişmiş İşlev ve Kapamalar](ch19-05-advanced-functions-and-closures.md)
    - [Makrolar](ch19-06-macros.md)

- [Final Projesi: Çok İşlikli Web Sunucusu Oluşturmak](ch20-00-final-project-a-web-server.md)
    - [Tek İşlikli Bir Web Sunucusu Oluşturmak](ch20-01-single-threaded.md)
    - [Tek İşlikli Sunucumuzu Çok İşlikli Bir Sunucuya Dönüştürmek](ch20-02-multithreaded.md)
    - [Sorunsuzca Kapatmak ve Temizlik](ch20-03-graceful-shutdown-and-cleanup.md)

- [Ekler](appendix-00.md)
    - [A - Anahtar Kelimeler](appendix-01-keywords.md)
    - [B - İşleçler ve Semboller](appendix-02-operators.md)
    - [C - Türetilebilir Özellikler](appendix-03-derivable-traits.md)
    - [D - Faydalı Geliştirme Araçları](appendix-04-useful-development-tools.md)
    - [E - Sürümler](appendix-05-editions.md)
    - [F - Kitabın Çevirileri](appendix-06-translation.md)
    - [G - Rust Nasıl “Nightly Rust” Yapılır? ](appendix-07-nightly-rust.md)