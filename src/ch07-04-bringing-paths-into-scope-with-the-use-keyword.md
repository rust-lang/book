## `use` Anahtar Sözcüğüyle Yolları Kapsama Almak

İşlevleri çağırmak için yolları yazmak zorunda kalmak rahatsız edici ve
tekrarlı olabilir. Listing 7-7'de, `add_to_waitlist` işlevine mutlak veya göreceli yol
seçmiş olsak da, `add_to_waitlist` işlevini her çağırmak istediğimizde
`front_of_house` ve `hosting`'i de belirtmek zorundaydık. Neyse ki, bu süreci basitleştirmenin bir
yolu var: `use` anahtar sözcüğüyle bir yola kısayol oluşturabiliriz
ve ardından kapsamın geri kalanında daha kısa adı kullanabiliriz.

Listing 7-11'de, `crate::front_of_house::hosting` modülünü
`eat_at_restaurant` işlevinin kapsamına getiriyoruz, böylece
`eat_at_restaurant` içindeki `add_to_waitlist` işlevini çağırmak için sadece
`hosting::add_to_waitlist` belirtmemiz yeterli oluyor.

<Listing number="7-11" file-name="src/lib.rs" caption="Bringing a module into scope with `use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

Bir kapsamda `use` ve bir yol eklemek, dosya sisteminde sembolik bir bağlantı oluşturmaya benzer.
Crate köküne `use crate::front_of_house::hosting` ekleyerek,
`hosting` artık o kapsamda geçerli bir ad haline gelir, sanki `hosting`
modülü crate kökünde tanımlanmış gibi. `use` ile kapsama alınan yollar da,
diğer yollar gibi gizlilik kontrolünden geçer.

`use` komutunun yalnızca `use` komutunun kullanıldığı belirli kapsam için kısayol oluşturduğunu unutmayın.
Listing 7-12, `eat_at_restaurant` işlevini `customer` adlı yeni bir alt modüle taşır.
Bu modül, `use` komutundan farklı bir kapsamda olduğundan, işlev gövdesi derlenmez.
Listing 7-12. `eat_at_restaurant` işlevini `customer` adlı yeni bir alt modüle taşıma

<Listing number="7-12" file-name="src/lib.rs" caption="A `use` statement only applies in the scope it’s in.">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

Derleyici hatası, kısayolun artık
`customer` modülü içinde geçerli olmadığını gösteriyor:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

`use` komutunun artık kendi kapsamında kullanılmadığına dair bir uyarı da olduğunu unutmayın! Bu
sorunu gidermek için, `use` komutunu da `customer` modülüne taşıyın veya
çocuk modül olan `customer` modülünde `super::hosting` ile üst modüldeki kısayola
bağlantı verin.

### İdiomatik `use` Yollarının Oluşturulması

Listing 7-11'de, neden `use
crate::front_of_house::hosting` belirttikten sonra `hosting::add_to_waitlist`'i
`eat_at_restaurant` içinde `hosting::add_to_waitlist`'i çağırdığımızı merak etmiş olabilirsiniz.
Aynı sonucu elde etmek için, Listing 7-13'te olduğu gibi `add_to_waitlist` işlevine kadar `use` yolunu belirtmek yerine.

<Listing number="7-13" file-name="src/lib.rs" caption="Bringing the `add_to_waitlist` function into scope with `use`, which is unidiomatic">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

Listing 7-11 ve Listing 7-13 aynı görevi yerine getirmesine rağmen, Listing
7-11, `use` ile bir işlevi kapsam içine almak için kullanılan geleneksel yöntemdir.
`use` ile işlevin üst modülünü kapsam içine almak, işlevi çağırırken üst modülü
belirtmemiz gerektiği anlamına gelir. Fonksiyonu çağırırken üst modülü belirtmek,
fonksiyonun yerel olarak tanımlanmadığını açıkça gösterirken, tam yolun tekrarını en aza indirir.
Listing 7-13'teki kod, `add_to_waitlist`'in nerede tanımlandığını
belirsiz kılar.

Öte yandan, `use` ile yapıları, enumları ve diğer öğeleri getirirken,
tam yolu belirtmek gelenekseldir. Listing 7-14, standart kütüphanenin `HashMap` yapısını ikili bir
kutu kapsamına getirmenin geleneksel yolunu
göstermektedir.

<Listing number="7-14" file-name="src/main.rs" caption="Bringing `HashMap` into scope in an idiomatic way">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

Bu deyimin arkasında güçlü bir neden yoktur: bu sadece ortaya çıkan bir gelenektir
ve insanlar Rust kodunu bu şekilde okumaya ve yazmaya alışmıştır.

Bu deyimin istisnası, aynı ada sahip iki öğeyi
`use` deyimleriyle kapsam içine almamızdır, çünkü Rust buna izin vermez. Listing 7-15,
 aynı ada sahip ancak
farklı üst modüllere sahip iki `Result` türünü kapsam içine almayı ve bunlara nasıl atıfta bulunulacağını gösterir.

<Listing number="7-15" file-name="src/lib.rs" caption="Bringing two types with the same name into the same scope requires using their parent modules.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

Gördüğünüz gibi, üst modülleri kullanmak iki `Result` türünü birbirinden ayırır.
Bunun yerine `use std::fmt::Result` ve `use std::io::Result` belirtseydik,
aynı kapsamda iki `Result` türü olurdu ve Rust, `Result` kullandığımızda hangisini
kastettiğimizi bilemezdi.

### `as` Anahtar Sözcüğüyle Yeni İsimler Sağlama

`use` ile aynı isimdeki iki türü aynı kapsama alanına getirme sorununa başka bir çözüm daha vardır:
yolun ardından, `as` ve tür için yeni bir yerel isim veya _alias_ belirtebiliriz.
Listing 7-16, Listing 7-15'teki kodu, iki `Result` türünden birini `as` kullanarak yeniden adlandırarak yazmanın başka bir yolunu göstermektedir.
Listing 7-16. Listing 7-15'teki kodu yeniden yazma

<Listing number="7-16" file-name="src/lib.rs" caption="Renaming a type when it’s brought into scope with the `as` keyword">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

İkinci `use` deyiminde, `std::io::Result` türü için yeni isim `IoResult` seçtik.
Bu isim, aynı zamanda kapsamımıza aldığımız `std::fmt`'den gelen `Result` ile
çakışmayacaktır. Listing 7-15 ve Listing 7-16,
deyimsel olarak kabul edilir, bu yüzden seçim size kalmıştır!

### `pub use` ile İsimleri Yeniden Dışa Aktarma

`use` anahtar sözcüğüyle bir ismi kapsam içine aldığımızda, bu isim onu içe aktardığımız
kapsam içinde özel hale gelir. Bu kapsam dışındaki kodların, o isim bu kapsam içinde tanımlanmış gibi
o isme başvurmasını sağlamak için, `pub` ve
`use`'u birleştirebiliriz. Bu teknik, bir öğeyi
kapsama alanına getirirken aynı zamanda diğerlerinin de kendi
kapsama alanlarına getirebilmelerini sağladığımız için _yeniden dışa aktarma_ olarak adlandırılır.

Listing 7-17, Listing 7-11'deki kodu, kök modüldeki `use`
`pub use` olarak değiştirilmiş halini gösterir.

<Listing number="7-17" file-name="src/lib.rs" caption="Making a name available for any code to use from a new scope with `pub use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

Bu değişiklikten önce, harici kod `add_to_waitlist`
işlevini
`restaurant::front_of_house::hosting::add_to_waitlist()` yolunu kullanarak çağırmak zorundaydı, bu da
`front_of_house` modülünün `pub` olarak işaretlenmesini gerektiriyordu. Artık bu `pub
use`, `hosting` modülünü kök modülden yeniden dışa aktardığından, harici kod
bunun yerine `restaurant::hosting::add_to_waitlist()` yolunu kullanabilir.

Yeniden dışa aktarma, kodunuzun iç yapısı, kodunuzu çağıran programcıların
alan hakkında düşündüklerinden farklı olduğunda yararlıdır. Örneğin,
bu restoran metaforunda, restoranı işleten kişiler
“restoranın önü” ve “restoranın arkası” hakkında düşünürler. Ancak restoranı ziyaret eden müşteriler
muhtemelen restoranın bölümlerini bu terimlerle düşünmezler. `pub
use` ile kodumuzu tek bir yapı ile yazabilir, ancak farklı bir yapı ortaya çıkarabiliriz.
Böylece, kütüphanemiz üzerinde çalışan programcılar ve kütüphaneyi çağıran programcılar için
kütüphanemiz iyi organize olur. `pub use`'un başka bir örneğine ve bunun crate'inizin belgelerini nasıl etkilediğine
14. bölümdeki [“`pub use` ile Kullanışlı Bir Genel API Dışa Aktarma”][ch14-pub-use]<!-- ignore --> bölümünde bakacağız.

### Harici Paketleri Kullanma

2. bölümde, rastgele sayılar elde etmek için `rand` adlı harici bir
paket kullanan bir tahmin oyunu projesi programladık. Projemizde `rand` kullanmak için,
_Cargo.toml_ dosyasına şu satırı ekledik:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

`rand`'ı _Cargo.toml_ dosyasına bağımlılık olarak eklemek, Cargo'ya
`rand` paketini ve [crates.io](https://crates.io/) adresinden tüm bağımlılıkları indirmesini ve
`rand`'ı projemizde kullanılabilir hale getirmesini söyler.

Ardından, `rand` tanımlarını paketimizin kapsamına dahil etmek için,
`rand` ile başlayan bir `use` satırı ekledik ve kapsam içine almak istediğimiz öğeleri listeledik.
Bölüm 2'deki [“Rastgele Sayı Üretme”][rand]<!-- ignore --> bölümünde, `Rng` özelliğini kapsam içine aldığımızı ve `rand::thread_rng` işlevini çağırdığımızı hatırlayın:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Rust topluluğunun üyeleri,
[crates.io](https://crates.io/) adresinde birçok paket sunmuştur ve bunlardan herhangi birini paketinize eklemek için
aynı adımları izlemeniz gerekir: bunları paketinizin _Cargo.toml_ dosyasında listelemek ve
`use` komutunu kullanarak öğeleri kutusundan kapsam içine almak.

Standart `std` kütüphanesi de paketimizin dışında bulunan bir crate olduğunu unutmayın.
Standart kütüphane Rust diliyle birlikte geldiği için,
`std`'yi dahil etmek için _Cargo.toml_ dosyasını değiştirmemiz gerekmez. Ancak,
oradaki öğeleri paketimizin kapsamına almak için `use` ile ona başvurmamız gerekir. Örneğin,
`HashMap` ile şu satırı kullanırız:

```rust
use std::collections::HashMap;
```

Bu, standart kütüphane
kutusunun adı olan `std` ile başlayan mutlak bir yoldur.

### İç içe geçmiş yollar kullanarak büyük `use` listelerini temizleme

Aynı crate veya aynı modülde tanımlanmış birden fazla öğe kullanıyorsak,
her öğeyi ayrı bir satırda listelemek dosyalarımızda çok fazla dikey alan kaplayabilir.
Örneğin, Listing 2-4'teki tahmin oyununda kullandığımız bu iki `use` ifadesi,
`std` öğelerini kapsam içine alır:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

Bunun yerine, iç içe geçmiş yollar kullanarak aynı öğeleri tek bir satırda
kapsama alabiliriz. Bunu, yolun ortak kısmını belirtip ardından iki
iki nokta üst üste işareti ve ardından yolların farklı olan kısımlarının listesini
kıvrımlı parantezlerle çevreleyerek yaparız, Listing 7-18'de gösterildiği gibi.

<Listing number="7-18" file-name="src/main.rs" caption="Specifying a nested path to bring multiple items with the same prefix into scope">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

Daha büyük programlarda, aynı kutu veya modülden iç içe geçmiş yollar kullanarak birçok öğeyi kapsam içine almak,
gerekli olan ayrı `use` deyimlerinin sayısını
büyük ölçüde azaltabilir!

Yerleşik bir yolu, yolun herhangi bir seviyesinde kullanabiliriz; bu, bir alt yolu paylaşan
iki `use` ifadesini birleştirirken kullanışlıdır. Örneğin, Listing 7-19 iki
`use` ifadesini gösterir: biri `std::io`'yu kapsam içine alır, diğeri ise
`std::io::Write`'ı kapsam içine alır.

<Listing number="7-19" file-name="src/lib.rs" caption="Two `use` statements where one is a subpath of the other">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

Bu iki yolun ortak kısmı `std::io`'dur ve bu, ilk yolun tamamıdır.
Bu iki yolu tek bir `use` ifadesinde birleştirmek için, Listing 7-20'de gösterildiği gibi iç içe geçmiş yolda `self` kullanabiliriz.
Listing 7-20. İç içe geçmiş yolun `use` ifadesinde `self` kullanımı

<Listing number="7-20" file-name="src/lib.rs" caption="Combining the paths in Listing 7-19 into one `use` statement">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

Bu satır, `std::io` ve `std::io::Write` 'yi kapsam içine alır.

### Glob Operatörü

Bir yolda tanımlanan _tüm_ genel öğeleri kapsam içine almak istiyorsak,
o yolu `*` glob operatörünün ardından belirtiriz:

```rust
use std::collections::*;
```

Bu `use` ifadesi, `std::collections` içinde tanımlanan tüm genel öğeleri
geçerli kapsama alanına getirir. Glob operatörünü kullanırken dikkatli olun! Glob,
kapsamda hangi isimlerin olduğunu ve programınızda kullanılan bir ismin
nerede tanımlandığını anlamayı zorlaştırabilir. Ayrıca, bağımlılık tanımlarını değiştirirse,
içe aktardığınız öğeler de değişir ve bu, bağımlılığı yükselttiğinizde derleyici hatalarına yol açabilir,
örneğin bağımlılık aynı kapsamdaki tanımınızla aynı ada sahip bir tanım eklediğinde.

Glob operatörü genellikle test sırasında test edilen her şeyi
`tests` modülüne getirmek için kullanılır; bunu 11. bölümdeki [“Testler Nasıl Yazılır”][writing-tests]<!-- ignore --> bölümünde ele alacağız. Glob operatörü bazen
prelude deseninin bir parçası olarak da kullanılır: bu desen hakkında daha fazla
bilgi için [standart kütüphane belgelerine](../std/prelude/index.html#other-preludes)<!-- ignore --> bakın.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.md#pub-use-ile-kullanışlı-bir-kamuya-açık-api-dışa-aktarmak
[rand]: ch02-00-guessing-game-tutorial.md#rastgele-sayı-oluşturma
[writing-tests]: ch11-01-writing-tests.md#how-to-write-tests
