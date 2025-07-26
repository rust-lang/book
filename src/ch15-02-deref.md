## Akıllı İşaretçileri `Deref` ile Normal Referanslar Gibi Kullanmak

<!-- Eski bağlantı, kaldırmayın -->

<a id="treating-smart-pointers-like-regular-references-with-the-deref-trait"></a>

`Deref` trait'ini uygulamak, _dereference operatörü_ `*`'ın (çarpma veya glob operatörüyle karıştırılmamalı) davranışını özelleştirmenizi sağlar. `Deref`'i öyle bir şekilde uygularsanız, bir akıllı işaretçi normal bir referans gibi davranabilir ve referanslar üzerinde çalışan kodunuzu akıllı işaretçilerle de kullanabilirsiniz.

Önce, dereference operatörünün normal referanslarla nasıl çalıştığına bakalım. Sonra, `Box<T>` gibi davranan özel bir tür tanımlamaya çalışacağız ve dereference operatörünün neden yeni tanımladığımız türde referans gibi çalışmadığını göreceğiz. `Deref` trait'ini uygulamanın, akıllı işaretçilerin referanslara benzer şekilde çalışmasını nasıl mümkün kıldığını inceleyeceğiz. Ardından Rust'ın _deref zorlama_ (deref coercion) özelliğine ve bunun referanslar veya akıllı işaretçilerle çalışmamıza nasıl olanak tanıdığına bakacağız.

<!-- Eski bağlantılar, kaldırmayın -->

<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>
<a id="following-the-pointer-to-the-value"></a>

### Referansı Değere Takip Etmek

Normal bir referans bir işaretçi türüdür ve bir işaretçiyi, başka bir yerde saklanan bir değere giden bir ok gibi düşünebilirsiniz. 15-6 numaralı listede, bir `i32` değerine referans oluşturup, ardından referansı takip etmek için dereference operatörünü kullanıyoruz.

<Listing number="15-6" file-name="src/main.rs" caption="Bir referansı bir `i32` değerine takip etmek için dereference operatörünü kullanmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

</Listing>

`x` değişkeni `5` değerinde bir `i32` tutar. `y`'yi, `x`'e bir referans olacak şekilde ayarlıyoruz. `x`'in `5`'e eşit olduğunu doğrulayabiliriz. Ancak, `y`'deki değeri doğrulamak istersek, referansın işaret ettiği değere ulaşmak için `*y` kullanmamız gerekir (bu yüzden _dereference_ denir), böylece derleyici gerçek değeri karşılaştırabilir. `y`'yi dereference ettiğimizde, `y`'nin işaret ettiği tamsayı değerine erişebiliriz ve bunu `5` ile karşılaştırabiliriz.

Eğer `assert_eq!(5, y);` yazmaya çalışırsak, şu derleme hatasını alırız:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

Bir sayı ile o sayının referansını karşılaştırmak mümkün değildir çünkü bunlar farklı türlerdir. Referansın işaret ettiği değere ulaşmak için dereference operatörünü kullanmalıyız.

### `Box<T>`'yi Referans Gibi Kullanmak

15-6 numaralı listedeki kodu, bir referans yerine bir `Box<T>` kullanacak şekilde yeniden yazabiliriz; 15-7 numaralı listede, `Box<T>` üzerinde kullanılan dereference operatörü, referans üzerinde kullanılanla aynı şekilde çalışır.

<Listing number="15-7" file-name="src/main.rs" caption="Bir `Box<i32>` üzerinde dereference operatörünü kullanmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

15-7 ve 15-6 numaralı listeler arasındaki ana fark, burada `y`'yi, `x`'in kopyalanmış değerine işaret eden bir kutu olarak ayarlamamızdır; referans yerine kutu kullanıyoruz. Son doğrulamada, kutunun işaretçisini takip etmek için dereference operatörünü, `y` bir referansken yaptığımız gibi kullanabiliriz. Şimdi, `Box<T>`'yi özel yapan ve dereference operatörünü kullanmamıza olanak tanıyan şeyi, kendi kutu türümüzü tanımlayarak keşfedeceğiz.

### Kendi Akıllı İşaretçimizi Tanımlamak

Standart kütüphanenin sağladığı `Box<T>` türüne benzer bir sarmalayıcı tür oluşturalım ve akıllı işaretçi türlerinin varsayılan olarak referanslardan nasıl farklı davrandığını görelim. Sonra, dereference operatörünü kullanma yeteneğini nasıl ekleyeceğimize bakacağız.

> Not: Şimdi oluşturacağımız `MyBox<T>` türü ile gerçek `Box<T>` arasında büyük bir fark var: Bizim sürümümüz veriyi heap'te saklamayacak. Bu örnekte odak noktamız `Deref`, bu yüzden verinin nerede saklandığı, işaretçi benzeri davranıştan daha az önemli.

`Box<T>` türü nihayetinde tek elemanlı bir tuple struct olarak tanımlanır, bu yüzden 15-8 numaralı listede `MyBox<T>` türünü aynı şekilde tanımlıyoruz. Ayrıca, `Box<T>`'de tanımlı olan `new` fonksiyonuna karşılık gelen bir `new` fonksiyonu da tanımlayacağız.

<Listing number="15-8" file-name="src/main.rs" caption="Bir `MyBox<T>` türü tanımlamak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

</Listing>

`MyBox` adında bir struct tanımlıyoruz ve herhangi bir türde değer tutabilmek için bir genel parametre `T` bildiriyoruz. `MyBox` türü, tek elemanlı bir tuple struct'tır ve bu eleman `T` türündedir. `MyBox::new` fonksiyonu, `T` türünde bir parametre alır ve bu değeri tutan bir `MyBox` örneği döndürür.

15-7 numaralı listedeki `main` fonksiyonunu 15-8 numaralı listeye ekleyip, `Box<T>` yerine tanımladığımız `MyBox<T>` türünü kullanalım. 15-9 numaralı listedeki kod derlenmeyecek çünkü Rust, `MyBox`'ı dereference etmeyi bilmiyor.

<Listing number="15-9" file-name="src/main.rs" caption="`MyBox<T>`'yi referanslar ve `Box<T>` ile kullandığımız gibi kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

Ortaya çıkan derleme hatası şudur:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

`MyBox<T>` türümüz dereference edilemiyor çünkü bu yeteneği türümüze eklemedik. `*` operatörüyle dereference etmeyi mümkün kılmak için, `Deref` trait'ini uygularız.

<!-- Eski bağlantı, kaldırmayın -->

<a id="treating-a-type-like-a-reference-by-implementing-the-deref-trait"></a>

### `Deref` Trait'ini Uygulamak

10. Bölümdeki ["Bir Türde Trait Uygulamak"](ch10-02-traits.html#implementing-a-trait-on-a-type) bölümünde tartışıldığı gibi, bir trait'i uygulamak için trait'in gerekli metotlarını sağlamamız gerekir. Standart kütüphanede sağlanan `Deref` trait'i, `self`'i ödünç alan ve içteki veriye bir referans döndüren `deref` adlı bir metot uygulamamızı gerektirir. 15-10 numaralı listede, `MyBox<T>` tanımına eklenebilecek bir `Deref` uygulaması yer alıyor.

<Listing number="15-10" file-name="src/main.rs" caption="`MyBox<T>` üzerinde `Deref` uygulamak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

`type Target = T;` sözdizimi, `Deref` trait'i için ilişkili bir tür tanımlar. İlişkili türler, genel bir parametre bildirmenin biraz farklı bir yoludur, ancak şimdilik bunları dert etmeyin; 20. Bölümde daha ayrıntılı ele alacağız.

`deref` metodunun gövdesini `&self.0` ile dolduruyoruz, böylece `deref`, `*` operatörüyle erişmek istediğimiz değere bir referans döndürür; 5. Bölümdeki ["Adlandırılmamış Alanlarla Tuple Struct Kullanmak"](ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) bölümünden hatırlayacağınız gibi, `.0` tuple struct'ın ilk değerine erişir. 15-9 numaralı listedeki `main` fonksiyonu artık derlenir ve doğrulamalar geçer!

`Deref` trait'i olmadan, derleyici yalnızca `&` referanslarını dereference edebilir. `deref` metodu, derleyiciye, `Deref` uygulayan herhangi bir türün değerini alıp, bildiği şekilde dereference edebileceği bir `&` referansı elde etme yeteneği kazandırır.

15-9 numaralı listede `*y` yazdığımızda, perde arkasında Rust aslında şu kodu çalıştırır:

```rust,ignore
*(y.deref())
```

Rust, `*` operatörünü bir `deref` çağrısı ve ardından normal bir dereference ile değiştirir, böylece `deref` metodunu çağırmamız gerekip gerekmediğini düşünmemize gerek kalmaz. Bu Rust özelliği, elimizde normal bir referans mı yoksa `Deref` uygulayan bir tür mü olduğuna bakmaksızın aynı şekilde çalışan kod yazmamıza olanak tanır.

`deref` metodunun bir değere referans döndürmesinin ve parantez dışındaki normal dereference'ın (`*(y.deref())`) hâlâ gerekli olmasının nedeni, sahiplik sistemiyle ilgilidir. Eğer `deref` metodu değeri doğrudan döndürseydi, değer `self`'ten taşınırdı. Bu durumda veya dereference operatörünü kullandığımız çoğu durumda, `MyBox<T>` içindeki değerin sahipliğini almak istemeyiz.

Dikkat edin, `*` operatörü her kullandığımızda yalnızca bir kez `deref` çağrısı ile değiştirilir. `*` operatörünün yerine yapılan bu dönüşüm sonsuz döngüye girmez; sonuçta `i32` türünde bir veri elde ederiz ve bu, 15-9 numaralı listedeki `assert_eq!`'deki `5` ile eşleşir.

### Fonksiyonlar ve Metotlarla Otomatik Deref Zorlamaları

_Deref zorlama_ (deref coercion), `Deref` trait'ini uygulayan bir türün referansını başka bir türe dönüştürür. Örneğin, deref zorlama, `&String`'i `&str`'ye dönüştürebilir çünkü `String`, `Deref` trait'ini öyle uygular ki `&str` döndürür. Deref zorlama, fonksiyon ve metotlara argüman olarak verdiğimizde Rust'ın sağladığı bir kolaylıktır ve yalnızca `Deref` trait'ini uygulayan türlerde çalışır. Bir türün değerine referans verdiğimizde, fonksiyon veya metot tanımındaki parametre türüyle eşleşmiyorsa, Rust otomatik olarak bir dizi `deref` çağrısı yaparak türü dönüştürür.

Deref zorlama, fonksiyon ve metot çağrıları yazarken programcıların `&` ve `*` ile açıkça referans ve dereference eklemesini azaltmak için Rust'a eklenmiştir. Bu özellik, referanslar veya akıllı işaretçilerle çalışabilen daha fazla kod yazmamıza da olanak tanır.

Deref zorlamayı çalışırken görmek için, 15-8 numaralı listede tanımladığımız `MyBox<T>` türünü ve 15-10 numaralı listedeki `Deref` uygulamasını kullanalım. 15-11 numaralı listede, parametresi `&str` olan bir fonksiyon tanımı var.

<Listing number="15-11" file-name="src/main.rs" caption="Parametresi `&str` olan bir `hello` fonksiyonu">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

`hello` fonksiyonunu, argüman olarak bir string slice ile çağırabiliriz, örneğin `hello("Rust");`. Deref zorlama, 15-12 numaralı listede gösterildiği gibi, `MyBox<String>` türünde bir değerin referansıyla `hello` fonksiyonunu çağırmamıza olanak tanır.

<Listing number="15-12" file-name="src/main.rs" caption="Deref zorlama sayesinde bir `MyBox<String>` değerinin referansıyla `hello` fonksiyonunu çağırmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

Burada, argüman olarak `&m` ile `hello` fonksiyonunu çağırıyoruz; bu, bir `MyBox<String>` değerinin referansıdır. 15-10 numaralı listede `MyBox<T>` üzerinde `Deref` trait'ini uyguladığımız için, Rust `&MyBox<String>`'i `&String`'e dönüştürmek için `deref` çağırabilir. Standart kütüphane, `String` üzerinde `&str` döndüren bir `Deref` uygulaması sağlar ve bu, `Deref`'in API dokümantasyonunda yer alır. Rust, `&String`'i `&str`'ye dönüştürmek için tekrar `deref` çağırır ve bu, `hello` fonksiyonunun tanımıyla eşleşir.

Rust deref zorlama uygulamasaydı, 15-12 numaralı listedeki kod yerine 15-13 numaralı listedeki kodu yazmamız gerekirdi.

<Listing number="15-13" file-name="src/main.rs" caption="Rust'ta deref zorlama olmasaydı yazmamız gereken kod">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

`(*m)`, `MyBox<String>`'i bir `String`'e dereference eder. Ardından `&` ve `[..]`, `String`'in tamamına eşit bir string slice alır ve bu, `hello` fonksiyonunun imzasıyla eşleşir. Deref zorlama olmadan bu kod, tüm bu sembollerle okumak, yazmak ve anlamak açısından daha zordur. Deref zorlama, Rust'ın bu dönüşümleri otomatik olarak yapmasını sağlar.

İlgili türler için `Deref` trait'i tanımlandığında, Rust türleri analiz eder ve parametre türüyle eşleşen bir referans elde etmek için gerektiği kadar `Deref::deref` çağrısı kullanır. `Deref::deref`'in kaç kez eklenmesi gerektiği derleme zamanında çözülür, bu nedenle deref zorlama kullanmanın çalışma zamanı maliyeti yoktur!

### Deref Zorlamanın Değiştirilebilirlikle Etkileşimi

Nasıl ki `Deref` trait'iyle değiştirilemez referanslarda `*` operatörünü özelleştirebiliyorsanız, değiştirilebilir referanslarda da `*` operatörünü özelleştirmek için `DerefMut` trait'ini kullanabilirsiniz.

Rust, üç durumda türleri ve trait uygulamalarını bulduğunda deref zorlama yapar:

1. `&T`'den `&U`'ya, eğer `T: Deref<Target=U>` ise
2. `&mut T`'den `&mut U`'ya, eğer `T: DerefMut<Target=U>` ise
3. `&mut T`'den `&U`'ya, eğer `T: Deref<Target=U>` ise

İlk iki durum aynıdır, ancak ikincisi değiştirilebilirliği uygular. İlk durumda, elinizde bir `&T` varsa ve `T`, `Deref`'i bir tür `U` için uyguluyorsa, şeffaf bir şekilde bir `&U` elde edebilirsiniz. İkinci durumda, aynı deref zorlama değiştirilebilir referanslar için de geçerlidir.

Üçüncü durum daha karmaşıktır: Rust, değiştirilebilir bir referansı değiştirilemez bir referansa da zorlayabilir. Ancak tersi _mümkün değildir_: değiştirilemez referanslar asla değiştirilebilir referanslara zorlanmaz. Ödünç alma kuralları gereği, elinizde bir değiştirilebilir referans varsa, bu referans o veriye tek referans olmalıdır (aksi takdirde program derlenmezdi). Bir değiştirilebilir referansı bir değiştirilemez referansa dönüştürmek ödünç alma kurallarını asla bozmaz. Bir değiştirilemez referansı değiştirilebilir referansa dönüştürmek ise, ilk değiştirilemez referansın o veriye tek değiştirilemez referans olmasını gerektirir, ancak ödünç alma kuralları bunu garanti etmez. Bu nedenle, Rust bir değiştirilemez referansı değiştirilebilir referansa dönüştürmenin mümkün olduğunu varsayamaz.

[impl-trait]: ch10-02-traits.html#implementing-a-trait-on-a-type
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
