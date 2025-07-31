## İleri Düzey Fonksiyonlar ve Kapatıcılar (Closures)

Bu bölümde, fonksiyonlar ve kapatıcılarla (closure) ilgili bazı ileri düzey özellikler, fonksiyon işaretçileri ve closure döndürme gibi konular ele alınacaktır.

### Fonksiyon İşaretçileri

Fonksiyonlara closure geçirmeyi daha önce konuştuk; ayrıca normal fonksiyonları da fonksiyonlara parametre olarak geçirebilirsiniz! Bu teknik, yeni bir closure tanımlamak yerine zaten tanımlı bir fonksiyonu geçirmek istediğinizde kullanışlıdır. Fonksiyonlar, closure trait'i olan `Fn` ile karıştırılmaması gereken küçük harfli `fn` tipine dönüştürülebilir. `fn` tipi, _fonksiyon işaretçisi_ (function pointer) olarak adlandırılır. Fonksiyon işaretçileriyle fonksiyonları başka fonksiyonlara argüman olarak geçirebilirsiniz.

Bir parametrenin fonksiyon işaretçisi olduğunu belirtmek için kullanılan söz dizimi, closure'lara benzer; bu, 20-28'de gösterilmiştir. Burada, parametresine 1 ekleyen `add_one` fonksiyonunu tanımladık. `do_twice` fonksiyonu iki parametre alır: herhangi bir `i32` parametresi alıp `i32` döndüren bir fonksiyon işaretçisi ve bir `i32` değeri. `do_twice`, `f` fonksiyonunu iki kez çağırır, her seferinde `arg` değerini geçirir ve iki fonksiyon çağrısının sonucunu toplar. `main` fonksiyonu, `do_twice`'ı `add_one` ve `5` argümanlarıyla çağırır.

<Listing number="20-28" file-name="src/main.rs" caption="Bir argüman olarak fonksiyon işaretçisi almak için `fn` tipini kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

Bu kod "The answer is: 12" çıktısını verir. `do_twice` fonksiyonundaki `f` parametresinin, bir `i32` parametresi alıp bir `i32` döndüren bir `fn` olduğunu belirtiriz. Ardından, `do_twice` fonksiyonu gövdesinde `f`'yi çağırabiliriz. `main`'de, fonksiyon adı olan `add_one`'ı `do_twice`'a ilk argüman olarak geçebiliriz.

Closure'lardan farklı olarak, `fn` bir trait değil bir tiptir; bu yüzden parametre tipini doğrudan `fn` olarak belirtiriz, closure trait'lerinden biriyle trait sınırı belirten jenerik tip tanımı yapmamıza gerek yoktur.

Fonksiyon işaretçileri, üç closure trait'inin (`Fn`, `FnMut`, `FnOnce`) hepsini uygular; yani, closure bekleyen bir fonksiyona her zaman fonksiyon işaretçisi geçirebilirsiniz. Fonksiyonlarınızı hem closure hem de fonksiyon kabul edebilecek şekilde yazmak için, closure trait'lerinden biriyle jenerik tip kullanmak en iyisidir.

Bununla birlikte, yalnızca `fn` kabul etmek isteyebileceğiniz bir örnek, closure'ların olmadığı harici kodlarla arayüz kurarken olur: C fonksiyonları argüman olarak fonksiyon kabul edebilir, ancak closure kavramı yoktur.

Hem satır içi tanımlı bir closure hem de adlandırılmış bir fonksiyon kullanabileceğiniz bir örnek olarak, standart kütüphanedeki `Iterator` trait'inin sağladığı `map` metodunun kullanımına bakalım. Bir sayı vektörünü string vektörüne dönüştürmek için `map` metodunu kullanırken, 20-29'da olduğu gibi bir closure kullanabiliriz.

<Listing number="20-29" caption="Sayıları string'e dönüştürmek için `map` metoduyla closure kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-29/src/main.rs:here}}
```

</Listing>

Ya da closure yerine argüman olarak bir fonksiyon adı verebiliriz. 20-30'da bunun nasıl görüneceği gösterilmiştir.

<Listing number="20-30" caption="Sayıları string'e dönüştürmek için `String::to_string` metodunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-30/src/main.rs:here}}
```

</Listing>

Burada, birden fazla `to_string` fonksiyonu bulunduğu için, ["İleri Düzey Trait'ler"][advanced-traits]<!-- ignore --> bölümünde bahsettiğimiz tam nitelikli sözdizimini kullanmamız gerekir.

Burada, `ToString` trait'inde tanımlı olan `to_string` fonksiyonunu kullanıyoruz, ki bu da standart kütüphanenin `Display`'i uygulayan herhangi bir tür için uyguladığı bir trait'tir.

Hatırlarsanız, 6. Bölümdeki ["Enum değerleri"][enum-values]<!-- ignore --> kısmında, tanımladığımız her enum varyantının aynı zamanda bir başlatıcı fonksiyon haline geldiğinden bahsetmiştik. Bu başlatıcı fonksiyonları, closure'ları alan yöntemler için argüman olarak belirtebileceğimiz fonksiyon işaretçileri olarak kullanabiliriz; bu, 20-31'de görüldüğü gibidir.

<Listing number="20-31" caption="Sayılar üzerinden bir `Status` örneği oluşturmak için enum başlatıcılarını kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-31/src/main.rs:here}}
```

</Listing>

Burada, `map`'in çağrıldığı aralıkta bulunan her `u32` değeri için `Status::Value` örnekleri oluşturuyoruz; bunu `Status::Value`'nın başlatıcı fonksiyonu ile yapıyoruz. Bazı insanlar bu stili tercih ederken bazıları closure kullanmayı tercih ediyor. Her iki stil de aynı koda derlenir, bu yüzden sizin için hangisi daha açıksa o stili kullanın.

### Closure Döndürme

Kapatıcılar trait'ler tarafından temsil edilir, bu da doğrudan kapatıcı döndüremeyeceğiniz anlamına gelir. Bir trait döndürmek isteyebileceğiniz çoğu durumda, bunun yerine o trait'i uygulayan somut tipi fonksiyonun dönüş değeri olarak kullanabilirsiniz. Ancak, kapatıcılar söz konusu olduğunda genellikle bunu yapamazsınız çünkü onların döndürülebilir somut bir tipi yoktur. Örneğin, kapanış herhangi bir değeri kapsıyorsa, `fn` işaretçisi gibi davranan bir dönüş tipi kullanmanıza izin verilmez.

Bunun yerine, genellikle 10. Bölümde öğrendiğimiz `impl Trait` sözdizimini kullanırsınız. Herhangi bir fonksiyon tipini döndürebilirsiniz; `Fn`, `FnOnce` ve `FnMut` kullanarak. Örneğin, 20-32'deki kod sorunsuz bir şekilde çalışacaktır.

<Listing number="20-32" caption="Fonksiyondan bir closure döndürmek için `impl Trait` sözdizimini kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-32/src/lib.rs}}
```

</Listing>

Ancak, 13. Bölümdeki ["Kapatıcı Tipi Çıkarımı ve
Açıklaması"][closure-types]<!-- ignore --> kısmında belirttiğimiz gibi, her kapatıcı kendi benzersiz tipidir. Aynı imzaya sahip ancak farklı uygulamalara sahip birden fazla fonksiyonla çalışmanız gerektiğinde, bunlar için bir trait nesnesi kullanmanız gerekecektir. 20-33'de gösterilen kodu yazarsanız ne olacağını düşünün.

<Listing file-name="src/main.rs" number="20-33" caption="Farklı `impl Fn` döndüren fonksiyonlar için bir `Vec<T>` oluşturmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-33/src/main.rs}}
```

</Listing>

Burada, her ikisi de `impl Fn(i32) -> i32` döndüren iki fonksiyonumuz var: `returns_closure` ve `returns_initialized_closure`. Dikkat edin ki, döndürdükleri kapatıcılar farklıdır, her ne kadar aynı tipi uygulıyor olsalar da. Bunu derlemeye çalışırsak, Rust bize bunun çalışmayacağını bildirir:

```text
{{#include ../listings/ch20-advanced-features/listing-20-33/output.txt}}
```

Hata mesajı, `impl Trait` döndürdüğümüzde Rust'ın bizim için nasıl benzersiz bir _opak tip_ oluşturduğunu, yani Rust'ın bizim için oluşturduğu şeylerin detaylarına girmeden göremediğimiz bir tipi oluşturduğunu söyler. Bu yüzden, bu fonksiyonlar her ne kadar aynı trait'i (`Fn(i32) -> i32`) uygulayan kapatıcılar döndürse de, Rust'ın her biri için ürettiği opak tipler farklıdır. (Bu, 17. Bölümde gördüğümüz gibi, aynı çıktı tipine sahip farklı async blokları için Rust'ın nasıl farklı somut tipler ürettiğine benzer; bkz. [“Herhangi Bir Sayıda Gelecek ile Çalışmak”][any-number-of-futures] Bölüm 17. Bu sorunun birkaç kez çözümünü gördük: bir trait nesnesi kullanabiliriz, bu da 20-34'de gösterilmiştir.

<Listing number="20-34" caption="Aynı tipe sahip olmaları için `Box<dyn Fn>` döndüren fonksiyonlar için bir `Vec<T>` oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-34/src/main.rs:here}}
```

</Listing>

Bu kod sorunsuz bir şekilde derlenecektir. Trait nesneleri hakkında daha fazla bilgi için, 18. Bölümdeki ["Farklı Tip Değerlerine İzin Veren Trait Nesnelerini Kullanma"][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore
--> kısmına bakın.

Şimdi, makrolara bakalım!

[advanced-traits]: ch20-02-advanced-traits.md#i̇leri-düzey-traitler
[enum-values]: ch06-01-defining-an-enum.md#enum-değerleri
[closure-types]: ch13-01-closures.md#kapanışlarda-tür-çıkarımı-ve-açık-tür-bildirimi
[any-number-of-futures]: ch17-03-more-futures.md
[using-trait-objects-that-allow-for-values-of-different-types]: ch18-02-trait-objects.md#ortak-davranışları-soyutlamak-için-trait-nesneleri-kullanmak
