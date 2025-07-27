## Bir Crate'i Crates.io'ya Yayımlamak

Projelerimizde [crates.io](https://crates.io/)<!-- ignore -->'dan paketleri bağımlılık olarak kullandık, ancak kendi paketlerinizi yayımlayarak kodunuzu başkalarıyla da paylaşabilirsiniz. [crates.io](https://crates.io/)<!-- ignore -->'daki crate kayıt sistemi, paketlerinizin kaynak kodunu dağıtır; yani esas olarak açık kaynak kodlarını barındırır.

Rust ve Cargo, yayımladığınız paketin başkaları tarafından bulunmasını ve kullanılmasını kolaylaştıran özelliklere sahiptir. Şimdi bu özelliklerden bazılarını ele alıp, ardından bir paketin nasıl yayımlanacağını açıklayacağız.

### Faydalı Dokümantasyon Yorumları Yazmak

Paketlerinizi doğru şekilde dokümante etmek, diğer kullanıcıların onları nasıl ve ne zaman kullanacaklarını bilmelerine yardımcı olur; bu nedenle dokümantasyon yazmaya zaman ayırmaya değer. 3. bölümde, Rust kodunu iki eğik çizgi `//` ile nasıl yorum satırı haline getireceğimizi tartıştık. Rust ayrıca, _dokümantasyon yorumu_ olarak bilinen ve HTML dokümantasyonu üreten özel bir yorum türüne sahiptir. HTML çıktısı, kamuya açık API öğeleri için dokümantasyon yorumlarının içeriğini gösterir; bu, crate'inizi _kullanmaya_ yönelik bilgi almak isteyen programcılar içindir, crate'inizin _nasıl uygulandığına_ yönelik değil.

Dokümantasyon yorumları, iki yerine üç eğik çizgi `///` ile başlar ve metni biçimlendirmek için Markdown notasyonunu destekler. Dokümantasyon yorumlarını, dokümante ettikleri öğenin hemen önüne yerleştirin. Liste 14-1, `my_crate` adlı bir crate'teki `add_one` fonksiyonu için dokümantasyon yorumlarını gösteriyor.

<Listing number="14-1" file-name="src/lib.rs" caption="Bir fonksiyon için dokümantasyon yorumu">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

</Listing>

Burada, `add_one` fonksiyonunun ne yaptığına dair bir açıklama, `Examples` başlıklı bir bölüm ve ardından `add_one` fonksiyonunun nasıl kullanılacağını gösteren bir kod örneği veriyoruz. Bu dokümantasyon yorumundan HTML dokümantasyonu oluşturmak için `cargo doc` komutunu çalıştırabiliriz. Bu komut, Rust ile birlikte gelen `rustdoc` aracını çalıştırır ve oluşturulan HTML dokümantasyonunu _target/doc_ dizinine koyar.

Kolaylık olması açısından, `cargo doc --open` komutunu çalıştırmak, mevcut crate'inizin dokümantasyonunun (ve tüm bağımlılıklarının) HTML'ini oluşturur ve sonucu bir web tarayıcısında açar. `add_one` fonksiyonuna gidin ve dokümantasyon yorumlarındaki metnin nasıl görüntülendiğini, Şekil 14-1'de gösterildiği gibi göreceksiniz.

<img alt="`my_crate`'in `add_one` fonksiyonu için oluşturulmuş HTML dokümantasyonu" src="img/trpl14-01.png" class="center" />

<span class="caption">Şekil 14-1: `add_one` fonksiyonu için HTML dokümantasyonu</span>

#### Sık Kullanılan Bölümler

Liste 14-1'de, HTML'de "Examples" başlıklı bir bölüm oluşturmak için `# Examples` Markdown başlığını kullandık. Crate yazarlarının dokümantasyonlarında sıkça kullandığı diğer bazı bölümler şunlardır:

- **Panics**: Dokümante edilen fonksiyonun hangi durumlarda panic yapabileceği. Fonksiyonu çağıranlar, programlarının panic yapmasını istemiyorlarsa, bu durumlarda fonksiyonu çağırmamaya dikkat etmelidir.
- **Errors**: Fonksiyon bir `Result` döndürüyorsa, hangi tür hataların oluşabileceğini ve bu hataların hangi koşullarda döndürülebileceğini açıklamak, çağıranların farklı hata türlerini farklı şekillerde ele alabilmeleri için faydalı olur.
- **Safety**: Fonksiyonun çağrılması `unsafe` ise (20. bölümde güvensizliği tartışacağız), fonksiyonun neden güvensiz olduğunu ve çağıranların uyması gereken kuralları açıklayan bir bölüm olmalıdır.

Çoğu dokümantasyon yorumu bu bölümlerin hepsine ihtiyaç duymaz, ancak bu liste, kodunuzu kullananların bilmek isteyeceği yönleri hatırlamanız için iyi bir kontrol listesidir.

#### Dokümantasyon Yorumları Test Olarak

Dokümantasyon yorumlarınıza örnek kod blokları eklemek, kütüphanenizin nasıl kullanılacağını göstermek için faydalı olur ve ek bir avantajı daha vardır: `cargo test` komutunu çalıştırmak, dokümantasyonunuzdaki kod örneklerini test olarak çalıştırır! Örnekli dokümantasyon kadar iyi bir şey yoktur. Ama kod değiştiği halde güncellenmemiş örnekler kadar kötü bir şey de yoktur. Eğer Liste 14-1'deki `add_one` fonksiyonu için dokümantasyonla birlikte `cargo test` çalıştırırsak, test sonuçlarında şöyle bir bölüm görürüz:

<!-- manuel-yenileme
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
aşağıda sadece doc-tests bölümünü kopyalayın
-->

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

Şimdi, fonksiyonu veya örneği değiştirip, örnekteki `assert_eq!` panic yaparsa ve tekrar `cargo test` çalıştırırsak, doc test'ler örnekle kodun birbirinden koptuğunu yakalar!

#### İçerik Yorumlarını Yorumlamak

`//!` ile başlayan dokümantasyon yorumları, yorumun _içerdiği_ öğeye dokümantasyon ekler; yorumdan _sonra gelen_ öğelere değil. Genellikle bu tür yorumları crate kök dosyasında (_src/lib.rs_ varsayılan olarak) veya bir modülün başında, crate'i veya modülü bir bütün olarak dokümante etmek için kullanırız.

Örneğin, `add_one` fonksiyonunu içeren `my_crate` crate'inin amacını açıklayan bir dokümantasyon eklemek için, _src/lib.rs_ dosyasının başına `//!` ile başlayan yorumlar ekleriz (Liste 14-2).

<Listing number="14-2" file-name="src/lib.rs" caption="`my_crate` crate'inin tamamı için dokümantasyon">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

</Listing>

Dikkat edin, `//!` ile başlayan son satırdan sonra herhangi bir kod yok. Çünkü yorumlara `//!` ile başladık, `///` ile değil; bu yüzden bu yorum, kendisinden sonra gelen öğeye değil, _içinde bulunduğu_ öğeye dokümantasyon ekler. Bu durumda, bu öğe crate kökü olan _src/lib.rs_ dosyasıdır. Bu yorumlar, tüm crate'i açıklar.

`cargo doc --open` komutunu çalıştırdığımızda, bu yorumlar crate'in dokümantasyonunun ana sayfasında, kamuya açık öğelerin listesi üzerinde görüntülenir (Şekil 14-2).

<img alt="Tüm crate için bir yorum içeren oluşturulmuş HTML dokümantasyonu" src="img/trpl14-02.png" class="center" />

<span class="caption">Şekil 14-2: `my_crate` için, crate'in tamamını açıklayan yorum dahil oluşturulmuş dokümantasyon</span>

Öğe içindeki dokümantasyon yorumları, özellikle crate ve modülleri açıklamak için faydalıdır. Bunları, kullanıcılara crate'in organizasyonunu anlamalarına yardımcı olmak için kullanın.

### `pub use` ile Kullanışlı Bir Kamuya Açık API Dışa Aktarmak

Kamuya açık API'nizin yapısı, bir crate yayımlarken önemli bir konudur. Crate'inizi kullanan kişiler, yapısına sizden daha az aşinadır ve crate'iniz büyük bir modül hiyerarşisine sahipse, istedikleri parçaları bulmakta zorlanabilirler.

7. bölümde, öğeleri `pub` anahtar kelimesiyle kamuya açık yapmayı ve `use` anahtar kelimesiyle scope'a getirmeyi ele aldık. Ancak, bir crate geliştirirken size mantıklı gelen yapı, kullanıcılarınız için çok da kullanışlı olmayabilir. Yapılarınızı birden fazla seviyeden oluşan bir hiyerarşide düzenlemek isteyebilirsiniz, ancak kullanıcılarınız, hiyerarşinin derinliklerinde tanımladığınız bir türü bulmakta zorlanabilir. Ayrıca, `use my_crate::some_module::another_module::UsefulType;` yazmak yerine `use my_crate::UsefulType;` yazmak isteyebilirler.

İyi haber şu ki, eğer bu yapı başkaları için kullanışlı değilse, dahili organizasyonunuzu yeniden düzenlemeniz gerekmez: bunun yerine, `pub use` kullanarak öğeleri yeniden dışa aktarabilir, kamuya açık yapının, özel yapınızdan farklı olmasını sağlayabilirsiniz. *Yeniden dışa aktarma* (re-exporting), bir konumdaki kamuya açık bir öğeyi başka bir konumda da kamuya açık hale getirir; sanki o öğe orada tanımlanmış gibi.

Örneğin, sanatsal kavramları modellemek için `art` adında bir kütüphane yaptığımızı varsayalım. Bu kütüphanede iki modül var: `PrimaryColor` ve `SecondaryColor` adlı iki enum içeren bir `kinds` modülü ve `mix` adlı bir fonksiyon içeren bir `utils` modülü (Liste 14-3).

<Listing number="14-3" file-name="src/lib.rs" caption="`kinds` ve `utils` modüllerine ayrılmış öğeleri olan bir `art` kütüphanesi">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

</Listing>

Şekil 14-3, bu crate için `cargo doc` ile oluşturulan dokümantasyonun ana sayfasının nasıl görüneceğini gösteriyor.

<img alt="`art` crate'inin, `kinds` ve `utils` modüllerini listeleyen oluşturulmuş dokümantasyonu" src="img/trpl14-03.png" class="center" />

<span class="caption">Şekil 14-3: `art` için, `kinds` ve `utils` modüllerini listeleyen dokümantasyonun ana sayfası</span>

Dikkat edin, `PrimaryColor` ve `SecondaryColor` türleri ile `mix` fonksiyonu ana sayfada listelenmemiştir. Onları görmek için `kinds` ve `utils`'e tıklamamız gerekir.

Bu kütüphaneye bağımlı başka bir crate, `art`'tan öğeleri scope'a getirmek için, şu anda tanımlı olan modül yapısını belirten `use` ifadelerine ihtiyaç duyar. Liste 14-4, `art` crate'inden `PrimaryColor` ve `mix` öğelerini kullanan bir crate örneğini gösteriyor.

<Listing number="14-4" file-name="src/main.rs" caption="Dahili yapısı dışa aktarılan `art` crate'inin öğelerini kullanan bir crate">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

</Listing>

Liste 14-4'teki kodun yazarı, `PrimaryColor`'ın `kinds` modülünde, `mix`'in ise `utils` modülünde olduğunu bulmak zorunda kaldı. `art` crate'inin modül yapısı, crate üzerinde çalışan geliştiriciler için daha anlamlıdır; kullananlar için ise kafa karıştırıcı olabilir. Dahili yapı, crate'in nasıl kullanılacağına dair bilgi vermez, aksine kafa karışıklığına yol açar çünkü kullanıcılar nerede arayacaklarını bulmak ve `use` ifadelerinde modül adlarını belirtmek zorundadır.

Kamuya açık API'den dahili organizasyonu kaldırmak için, Liste 14-3'teki `art` crate kodunu, öğeleri en üst seviyede yeniden dışa aktarmak için `pub use` ifadeleri ekleyerek değiştirebiliriz (Liste 14-5).

<Listing number="14-5" file-name="src/lib.rs" caption="Öğeleri yeniden dışa aktarmak için `pub use` ifadeleri eklemek">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

</Listing>

Bu crate için `cargo doc` ile oluşturulan API dokümantasyonu, artık ana sayfada yeniden dışa aktarılan öğeleri listeler ve bağlantı verir (Şekil 14-4); böylece `PrimaryColor`
