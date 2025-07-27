## İleri Düzey Tipler

Rust'ın tip sisteminde, şimdiye kadar bahsettiğimiz ama henüz ayrıntılı olarak ele almadığımız bazı özellikler vardır. Öncelikle, yeni tiplerin (newtype) neden faydalı olduğunu inceleyerek genel olarak yeni tiplerden bahsedeceğiz. Ardından, yeni tiplere benzer ama biraz farklı bir anlamsal yapıya sahip olan tip takma adları (type alias) özelliğine geçeceğiz. Ayrıca `!` tipi ve dinamik boyutlu tipleri de tartışacağız.

### Tip Güvenliği ve Soyutlama için Yeni Tip Desenini Kullanmak

Bu bölüm, daha önceki ["Dış Trait'leri Dış Tipler Üzerinde Uygulamak için Yeni Tip Desenini Kullanma"][using-the-newtype-pattern]<!-- ignore --> başlığını okuduğunuzu varsayar. Yeni tip deseni, şimdiye kadar tartıştıklarımızın ötesinde, değerlerin asla karıştırılmamasını statik olarak sağlamak ve bir değerin birimini belirtmek gibi görevler için de faydalıdır. 20-16'da yeni tiplerin birimleri belirtmek için nasıl kullanıldığına dair bir örnek görmüştünüz: `Millimeters` ve `Meters` yapıları, `u32` değerlerini bir yeni tip içinde sarmalıyordu. Eğer bir fonksiyonu `Millimeters` tipinde bir parametreyle yazarsak, yanlışlıkla bu fonksiyona `Meters` veya düz bir `u32` değeriyle çağrı yapmaya çalışırsak program derlenmez.

Yeni tip deseni, bir tipin bazı implementasyon detaylarını soyutlamak için de kullanılabilir: yeni tip, içteki özel tipten farklı bir genel API sunabilir.

Yeni tipler ayrıca içsel implementasyonu gizleyebilir. Örneğin, bir kişinin kimliğini ismiyle eşleştiren bir `HashMap<i32, String>`'i saran bir `People` tipi sağlayabiliriz. `People` kullanan kod, yalnızca bizim sunduğumuz genel API ile etkileşime girer; örneğin, `People` koleksiyonuna bir isim eklemek için bir metod. Bu kod, isimlere dahili olarak bir `i32` kimliği atadığımızı bilmek zorunda değildir. Yeni tip deseni, uygulama detaylarını gizlemek için hafif bir kapsülleme (encapsulation) yoludur; bu konuyu 18. Bölümde ["Uygulama Detaylarını Gizleyen Kapsülleme"][encapsulation-that-hides-implementation-details]<!-- ignore --> başlığında tartışmıştık.

### Tip Takma Adları ile Tip Eşanlamlıları Oluşturmak

Rust, mevcut bir tipe başka bir ad vermek için _tip takma adı_ (type alias) tanımlama olanağı sunar. Bunun için `type` anahtar kelimesini kullanırız. Örneğin, `i32` için `Kilometers` adında bir takma ad oluşturabiliriz:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

Artık `Kilometers` takma adı, `i32` için bir _eşanlamlı_ (synonym) olur; 20-16'da oluşturduğumuz `Millimeters` ve `Meters` tiplerinden farklı olarak, `Kilometers` ayrı ve yeni bir tip değildir. `Kilometers` tipine sahip değerler, `i32` tipine sahip değerlerle aynı şekilde muamele görür:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

`Kilometers` ve `i32` aynı tip olduğu için, her iki tipteki değerleri birbirine ekleyebilir ve `Kilometers` değerlerini `i32` parametresi alan fonksiyonlara geçirebiliriz. Ancak bu yöntemi kullanırsak, daha önce tartıştığımız yeni tip deseninin sağladığı tip denetimi avantajlarını elde edemeyiz. Yani, bir yerde `Kilometers` ve `i32` değerlerini karıştırırsak, derleyici hata vermez.

Tip eşanlamlılarının ana kullanım amacı, tekrarı azaltmaktır. Örneğin, şöyle uzun bir tipimiz olabilir:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Bu uzun tipi fonksiyon imzalarında ve tip açıklamalarında tekrar tekrar yazmak yorucu ve hata yapmaya açık olabilir. 20-25'teki gibi kodlarla dolu bir projeyi hayal edin.

<Listing number="20-25" caption="Uzun bir tipi birçok yerde kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

Bir tip takma adı, bu kodu daha yönetilebilir hale getirir ve tekrarı azaltır. 20-26'da, bu uzun tip için `Thunk` adında bir takma adı tanıttık ve tipin tüm kullanımlarını daha kısa takma adı `Thunk` ile değiştirebildik.

<Listing number="20-26" caption="Tekrarı azaltmak için `Thunk` adında bir tip takma adı tanıtmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

Bu kod çok daha okunaklı ve yazması daha kolay! Anlamlı bir isim seçmek, niyetinizi iletmeye yardımcı olabilir (_thunk_, daha sonra değerlendirilecek kod anlamına gelen bir kelimedir, bu yüzden saklanan bir kapanış için uygun bir isimdir).

Tip takma adları ayrıca `Result<T, E>` tipi ile birlikte tekrarın azaltılması için yaygın olarak kullanılır. Standart kütüphanedeki `std::io` modülünü düşünün. G/Ç işlemleri genellikle işlemlerin başarısız olduğu durumları ele almak için `Result<T, E>` döndürür. Bu kütüphane, tüm olası G/Ç hatalarını temsil eden bir `std::io::Error` yapısına sahiptir. `std::io` içindeki birçok fonksiyon, `E`'si `std::io::Error` olan `Result<T, E>` döndürecektir; örneğin, `Write` trait'indeki bu fonksiyonlar:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

`Result<..., Error>` çok tekrar ediyor. Bu nedenle, `std::io` bu tip takma adı bildirimini yapmaktadır:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

Bu bildirim `std::io` modülünde bulunduğundan, tam nitelikli takma ad `std::io::Result<T>` olarak kullanılabilir; yani, `E`'si `std::io::Error` olarak doldurulmuş bir `Result<T, E>`. `Write` trait fonksiyonlarının imza görünümleri şu şekilde olmaktadır:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

Tip takma adı iki şekilde yardımcı olur: kodun yazılmasını kolaylaştırır _ve_ `std::io`'nun tamamında tutarlı bir arayüz sağlar. Bir takma ad olduğu için, bu sadece başka bir `Result<T, E>`'dir, bu da onunla çalışan herhangi bir yöntemi kullanabileceğimiz anlamına gelir, ayrıca `?` operatörü gibi özel sözdizimleri.

### Asla Dönmeyen Hiç Tipi

Rust'ın, tip teorisi dilinde _boş tip_ (empty type) olarak bilinen özel bir tipi vardır çünkü hiç değeri yoktur. Biz buna _asla tip_ (never type) demeyi tercih ediyoruz çünkü bir fonksiyon asla dönmeyecekse dönüş tipi yerinde durur. İşte bir örnek:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

Bu kod, "bar fonksiyonu asla dönmez" şeklinde okunur. Asla dönmeyen fonksiyonlara _sapma yapan fonksiyonlar_ denir. `!` tipi değerlerini oluşturamayız, bu yüzden `bar` asla mümkün olmayacak şekilde dönemez.

Ama hiç değeri olamayacak bir tipin ne faydası var? Hatırlayın, bu kod 2-5'te, sayı tahmin etme oyununun bir parçasıydı; burada birazını 20-27'de yeniden ürettik.

<Listing number="20-27" caption="`continue` ile biten bir `match`">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

</Listing>

O zamanlar, bu kodda bazı detayları atlamıştık. 6. Bölümde ["`match` Kontrol Akışı Operatörü"][the-match-control-flow-operator]<!-- ignore --> başlığında, `match` kollarının hepsinin aynı tipi döndürmesi gerektiğini tartışmıştık. Yani, örneğin, aşağıdaki kod çalışmaz:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

Bu kodda `guess`'in tipi hem bir tam sayı _hem_ bir dize olmak zorunda kalır ve Rust, `guess`'in yalnızca bir tipe sahip olmasını gerektirir. Peki, `continue` ne döndürüyor? 20-27'deki kodda bir kolu `u32` döndürürken diğerinin `continue` ile bitmesine nasıl izin verildi?

Tahmin ettiğiniz gibi, `continue` bir `!` değerine sahiptir. Yani, Rust `guess`'in tipini hesaplarken, birincisi `u32` değeriyle ve diğeri `!` değeriyle olan her iki `match` koluna da bakar. Çünkü `!` asla bir değere sahip olamaz, Rust `guess`'in tipinin `u32` olduğunu belirler.

Bu davranışı tanımlamanın resmi yolu, `!` tipindeki ifadelerin herhangi bir diğer tipe zorlanabileceğidir. Bu `match` kolunu `continue` ile bitirmemize izin veriliyor çünkü `continue` bir değer döndürmüyor; bunun yerine, kontrolü döngünün başına geri taşıyor, bu yüzden `Err` durumunda `guess`'e bir değer atamıyoruz.

Asla tip, `panic!` makrosuyla da kullanışlıdır. Hatırlayın, bir değer üreten veya bu tanıma sahip `Option<T>` değerleri üzerinde panik yapan `unwrap` fonksiyonu:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

Bu kodda, 20-27'deki `match`teki gibi aynı şey olur: Rust, `val`'ın tipinin `T` ve `panic!`'in tipinin `!` olduğunu görür, bu yüzden genel `match` ifadesinin sonucu `T`'dir. Bu kod çalışır çünkü `panic!` bir değer üretmez; programı sonlandırır. `None` durumunda, `unwrap`'dan bir değer döndürmeyeceğiz, bu yüzden bu kod geçerlidir.

Bir `!` değerine sahip olan bir diğer ifade de bir `loop`tur:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

Burada, döngü asla bitmez, bu yüzden `!` ifadenin değeridir. Ancak, bir `break` eklersek bu doğru olmazdı çünkü döngü `break`e ulaştığında sona ererdi.

### Dinamik Olarak Boyutlandırılan Tipler ve `Sized` Trait'i

Rust, belirli tiplerin, belirli bir tip için bir değerin ne kadar alan kaplayacağını bilmesi gibi belirli ayrıntıları bilmelidir. Bu, tip sisteminin bir köşesini başlangıçta biraz kafa karıştırıcı hale getirir: _dinamik olarak boyutlandırılan tipler_ (DSTs) kavramı. Bu türler, yalnızca çalışma zamanında boyutunu bilebileceğimiz değerlerle kod yazmamıza olanak tanır.

Kullanımda olan bir dinamik olarak boyutlandırılan tip olan `str`'in ayrıntılarına dalalım, bu türü kitap boyunca kullanıyoruz. Evet, `&str` değil, yalnızca `str` başlı başına bir DST'dir. String'in ne kadar uzun olduğunu çalışma zamanında bilemeyeceğimiz için, `str` tipinde bir değişken oluşturamayız, ayrıca `str` tipinde bir argüman da alamayız. Aşağıdaki çalışmayan kodu düşünün:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust, belirli bir tip için herhangi bir değerin ne kadar bellek kaplayacağını bilmelidir ve bir tipin tüm değerleri aynı miktarda bellek kullanmalıdır. Eğer Rust, bu kodu yazmamıza izin verseydi, bu iki `str` değeri aynı miktarda alan kaplamak zorunda kalırdı. Ama farklı uzunlukları var: `s1`'in 12 bayta, `s2`'in ise 15 bayta ihtiyacı var. Bu yüzden, dinamik olarak boyutlandırılan bir tipi içeren bir değişken oluşturmak mümkün değildir.

Peki, ne yapıyoruz? Bu durumda, zaten cevabı biliyorsunuz: `s1` ve `s2`'nin tipini `str` yerine `&str` yapıyoruz. 4. Bölümde ["String Dilimleri"][string-slices]<!-- ignore --> hatırlayın, dilim veri yapısı yalnızca başlangıç konumunu ve dilimin uzunluğunu saklar. Bu yüzden, bir `&T` tek bir değerdir ve `T`'nin bulunduğu bellek adresini saklarken, bir `&str` _iki_ değerdir: `str`'nin adresi ve uzunluğu. Bu nedenle, bir `&str` değerinin boyutunu derleme zamanında her zaman bilebiliriz: bu, bir `usize`'nin uzunluğunun iki katıdır. Yani, `&str`'nin boyutunu, hangi uzunlukta bir dizeye atıfta bulunduğuna bakılmaksızın her zaman biliriz. Genel olarak, dinamik olarak boyutlandırılan tipler bu şekilde Rust'ta kullanılır: dinamik bilgilere ait boyutu saklayan ekstra bir meta veriye sahiptirler. Dinamik olarak boyutlandırılan tiplerin altın kuralı, dinamik olarak boyutlandırılan tiplerden oluşan değerleri her zaman bir tür işaretçi arkasında tutmamız gerektiğidir.

`str` ile her türlü işaretçiyi birleştirebiliriz: örneğin, `Box<str>` veya `Rc<str>`. Aslında, bunu daha önce farklı bir dinamik olarak boyutlandırılan türle görmüştünüz: traitler. Her trait, `&dyn Trait` veya `Box<dyn Trait>` (`Rc<dyn Trait>` de işe yarar) gibi bir işaretçi arkasında başvurabileceğimiz dinamik olarak boyutlandırılan bir türdür.

DST'lerle çalışmak için, Rust bir tipin boyutunun derleme zamanında biliniş bilindiğini belirlemek için `Sized` trait'ini sağlar. Bu trait, boyutu derleme zamanında bilinen her şey için otomatik olarak uygulanır. Ayrıca, Rust, her genel fonksiyon tanımına `Sized` üzerinde örtük bir sınır ekler. Yani, şöyle bir genel fonksiyon tanımı:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

aslında şöyle yazılmış gibi muamele görür:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

Varsayılan olarak, genel fonksiyonlar yalnızca derleme zamanında bilinen boyuta sahip tipler üzerinde çalışır. Ancak, bu kısıtlamayı gevşetmek için aşağıdaki özel sözdizimini kullanabilirsiniz:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

`?Sized` üzerindeki bir trait sınırı, "`T` boyutlandırılmış olabilir veya olmayabilir" anlamına gelir ve bu notasyon, genel tiplerin varsayılan olarak derleme zamanında bilinen bir boyuta sahip olması gerektiği kısıtlamasını geçersiz kılar. Bu anlama sahip `?Trait` sözdizimi yalnızca `Sized` için mevcuttur, başka herhangi bir trait için değil.

Ayrıca, `t` parametresinin tipini `T`'den `&T`'ye değiştirdiğimizi unutmayın. Tip boyutu bilinmiyor olabileceğinden, her türlü işaretçi arkasında kullanmalıyız. Bu durumda, bir referans seçtik.

Sonraki bölümümüzde fonksiyonlar ve kapanışlar hakkında konuşacağız!

[encapsulation-that-hides-implementation-details]: ch18-01-what-is-oo.html#encapsulation-that-hides-implementation-details
[string-slices]: ch04-03-slices.html#string-slices
[the-match-control-flow-operator]: ch06-02-match.html#the-match-control-flow-operator
[using-trait-objects-that-allow-for-values-of-different-types]: ch18-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[using-the-newtype-pattern]: ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
