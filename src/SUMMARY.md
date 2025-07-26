# Rust Programlama Dili

[Rust Programlama Dili](title-page.md)
[Önsöz](foreword.md)
[Giriş](ch00-00-introduction.md)

## Başlangıç

- [Başlangıç](ch01-00-getting-started.md)
  - [Kurulum](ch01-01-installation.md)
  - [Merhaba, Dünya!](ch01-02-hello-world.md)
  - [Merhaba, Cargo!](ch01-03-hello-cargo.md)

- [Tahmin Oyunu Programlama](ch02-00-guessing-game-tutorial.md)

- [Yaygın Programlama Kavramları](ch03-00-common-programming-concepts.md)
  - [Değişkenler ve Değişkenlik](ch03-01-variables-and-mutability.md)
  - [Veri Türleri](ch03-02-data-types.md)
  - [İşlevler](ch03-03-how-functions-work.md)
  - [Yorumlar](ch03-04-comments.md)
  - [Kontrol Akışı](ch03-05-control-flow.md)

- [Sahipliği Anlamak](ch04-00-understanding-ownership.md)
  - [Sahiplik Nedir?](ch04-01-what-is-ownership.md)
- [Referanslar ve Ödünç Alma](ch04-02-references-and-borrowing.md)
- [Slice Türü](ch04-03-slices.md)

- [Yapıları Kullanarak İlgili Verileri Yapılandırma](ch05-00-structs.md)
  - [Yapıları Tanımlama ve Örnekleme](ch05-01-defining-structs.md)
  - [Yapıları Kullanan Bir Örnek Program](ch05-02-example-structs.md)
  - [Yöntem Sözdizimi](ch05-03-method-syntax.md)

- [Enumlar ve Desen Eşleştirme](ch06-00-enums.md)
  - [Enum Tanımlama](ch06-01-defining-an-enum.md)
  - [`match` Kontrol Akışı Yapısı](ch06-02-match.md)
  - [`if let` ve `let else` ile Kısa Kontrol Akışı](ch06-03-if-let.md)

## Temel Rust Bilgisi

- [Paketler, Crates ve Modüllerle Büyüyen Projeleri Yönetme](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
- [Paketler ve Crates](ch07-01-packages-and-crates.md)
  - [Kapsamı ve Gizliliği Kontrol Etmek için Modülleri Tanımlama](ch07-02-defining-modules-to-control-scope-and-privacy.md)
- [Modül Ağacındaki Bir Öğeye Başvurmak için Yollar](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
  - [`use` Anahtar Sözcüğüyle Yolları Kapsama Almak](ch07-04-use-anahtar-sözcüğüyle-yolları-kapsama-alamak.md)
- [Modülleri Farklı Dosyalara Ayırmak](ch07-05-modülleri-farklı-dosyalara-ayırmak.md)

- [Yaygın Koleksiyonlar](ch08-00-common-collections.md)
  - [Vektörlerle Değer Listelerini Depolama](ch08-01-vectors.md)
  - [UTF-8 Kodlu Metinleri Dizgilerle Depolama](ch08-02-strings.md)
  - [Hash Haritalarında İlişkili Değerlerle Anahtarları Depolama](ch08-03-hash-maps.md)

- [Hata İşleme](ch09-00-error-handling.md)
  - [`panic!` ile Kurtarılamayan Hatalar](ch09-01-unrecoverable-errors-with-panic.md)
  - [`Result` ile Kurtarılabilir Hatalar](ch09-02-recoverable-errors-with-result.md)
  - [`panic!` kullanmak mı, kullanmamak mı?](ch09-03-to-panic-or-not-to-panic.md)

- [Genel Tipler, Özellikler ve Ömürler](ch10-00-generics.md)
  - [Genel Veri Türleri](ch10-01-syntax.md)
- [Özellikler: Paylaşılan Davranışları Tanımlama](ch10-02-traits.md)
- [Ömürlerle Referansları Doğrulama](ch10-03-lifetime-syntax.md)

- [Otomatik Testler Yazma](ch11-00-testing.md)
  - [Testler Nasıl Yazılır](ch11-01-writing-tests.md)
  - [Testlerin Çalıştırılma Şeklinin Kontrol Edilmesi](ch11-02-running-tests.md)
  - [Testlerin Düzenlenmesi](ch11-03-test-organization.md)

- [Bir G/Ç Projesi: Komut Satırı Programı Oluşturma](ch12-00-an-io-project.md)
  - [Komut Satırı Argümanlarını Kabul Etme](ch12-01-accepting-command-line-arguments.md)
  - [Dosya Okuma](ch12-02-reading-a-file.md)
  - [Modülerliği ve Hata İşlemeyi İyileştirmek için Yeniden Yapılandırma](ch12-03-improving-error-handling-and-modularity.md)
  - [Test Odaklı Geliştirme ile Kütüphanenin İşlevselliğini Geliştirme](ch12-04-testing-the-librarys-functionality.md)
  - [Ortam Değişkenleriyle Çalışma](ch12-05-working-with-environment-variables.md)
- [Hata Mesajlarını Standart Çıktı Yerine Standart Hata Çıktısına Yazma](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Rust ile Düşünmek

- [İşlevsel Dil Özellikleri: İteratörler ve Kapanışlar](ch13-00-functional-features.md)
  - [Kapanışlar: Ortamlarını Yakalayan Anonim İşlevler](ch13-01-closures.md)
  - [İteratörlerle Bir Dizi Öğeyi İşleme](ch13-02-iterators.md)
  - [I/O Projemizi Geliştirme](ch13-03-improving-our-io-project.md)
  - [Performans Karşılaştırması: Döngüler ve İteratörler](ch13-04-performance.md)

- [Cargo ve Crates.io Hakkında Daha Fazla Bilgi](ch14-00-more-about-cargo.md)
- [Sürüm Profilleriyle Derlemeleri Özelleştirme](ch14-01-release-profiles.md)
- [Crates.io'ya Crate Yayınlama](ch14-02-publishing-to-crates-io.md)
  - [Cargo Çalışma Alanları](ch14-03-cargo-workspaces.md)
  - [`cargo install` ile Crates.io'dan İkili Dosyaları Yükleme](ch14-04-installing-binaries.md)
  - [Özel Komutlarla Cargo'yu Genişletme](ch14-05-extending-cargo.md)

- [Akıllı İşaretçiler](ch15-00-smart-pointers.md)
  - [`Box<T>` Kullanarak Yığın Üzerindeki Verilere İşaret Etme](ch15-01-box.md)
  - [`Deref` ile Akıllı İşaretçileri Normal Referanslar Gibi Ele Alma](ch15-02-deref.md)
- [`Drop` Özelliği ile Temizleme Sırasında Kod Çalıştırma](ch15-03-drop.md)
- [`Rc<T>`, Referans Sayımlı Akıllı İşaretçi](ch15-04-rc.md)
  - [`RefCell<T>` ve İç Değişkenlik Deseni](ch15-05-interior-mutability.md)
  - [Referans Döngüleri Bellek Sızıntısına Neden Olabilir](ch15-06-reference-cycles.md)

- [Korkusuz Eşzamanlılık](ch16-00-concurrency.md)
  - [İş Parçacıklarını Kullanarak Kodu Eşzamanlı Olarak Çalıştırma](ch16-01-threads.md)
  - [Mesaj Aktarımını Kullanarak İş Parçacıkları Arasında Veri Aktarımı](ch16-02-message-passing.md)
  - [Paylaşılan Durum Eşzamanlılığı](ch16-03-shared-state.md)
  - [`Send` ve `Sync` Özellikleriyle Genişletilebilir Eşzamanlılık](ch16-04-extensible-concurrency-sync-and-send.md)

- [Asenkron Programlamanın Temelleri: Async, Await, Futures ve Streams](ch17-00-async-await.md)
  - [Futures ve Async Sözdizimi](ch17-01-futures-and-syntax.md)
- [Async ile Eşzamanlılık Uygulama](ch17-02-concurrency-with-async.md)
- [Herhangi Bir Sayıda Futures ile Çalışma](ch17-03-more-futures.md)
  - [Akışlar: Sıralı Futures](ch17-04-streams.md)
  - [Async Özelliklerine Yakından Bakış](ch17-05-traits-for-async.md)
  - [Futures, Görevler ve İş Parçacıkları](ch17-06-futures-tasks-threads.md)

- [Rust'un Nesneye Yönelik Programlama Özellikleri](ch18-00-oop.md)
  - [Nesneye Yönelik Dillerin Özellikleri](ch18-01-what-is-oo.md)
  - [Farklı Türlerdeki Değerlere İzin Veren Özellik Nesnelerini Kullanma](ch18-02-trait-objects.md)
  - [Nesne Yönelimli Tasarım Desenini Uygulama](ch18-03-oo-design-patterns.md)

## İleri Düzey Konular

- [Desenler ve Eşleştirme](ch19-00-patterns.md)
  - [Desenlerin Kullanılabileceği Tüm Yerler](ch19-01-all-the-places-for-patterns.md)
  - [Çürütülebilirlik: Bir Desenin Eşleşmede Başarısız Olma Olasılığı](ch19-02-refutability.md)
  - [Desen Sözdizimi](ch19-03-pattern-syntax.md)

- [Gelişmiş Özellikler](ch20-00-advanced-features.md)
  - [Güvenli Olmayan Rust](ch20-01-unsafe-rust.md)
  - [Gelişmiş Özellikler](ch20-02-advanced-traits.md)
  - [Gelişmiş Türler](ch20-03-advanced-types.md)
  - [Gelişmiş İşlevler ve Kapanışlar](ch20-04-advanced-functions-and-closures.md)
  - [Makrolar](ch20-05-macros.md)

- [Son Proje: Çok İş Parçacıklı Bir Web Sunucusu Oluşturma](ch21-00-final-project-a-web-server.md)
  - [Tek İş Parçacıklı Web Sunucusu Oluşturma](ch21-01-single-threaded.md)
- [Tek İş Parçacıklı Sunucumuzu Çok İş Parçacıklı Sunucuya Dönüştürme](ch21-02-multithreaded.md)
  - [Zarif Kapatma ve Temizleme](ch21-03-graceful-shutdown-and-cleanup.md)

- [Ek](appendix-00.md)
- [A - Anahtar Kelimeler](appendix-01-keywords.md)
  - [B - Operatörler ve Semboller](appendix-02-operators.md)
- [C - Türetilebilir Özellikler](appendix-03-derivable-traits.md)
- [D - Yararlı Geliştirme Araçları](appendix-04-useful-development-tools.md)
  - [E - Baskılar](appendix-05-editions.md)
  - [F - Kitabın Çevirileri](appendix-06-translation.md)
  - [G - Rust'un Yapılışı ve “Nightly Rust”](appendix-07-nightly-rust.md)
