## Enum Tanımlama

Yapılar, ilgili alanları ve verileri bir araya getirmenin bir yolunu sunarken,
örneğin `Rectangle` ile `width` ve `height` gibi, enumlar bir değerin olası bir değerler kümesinden biri olduğunu belirtmenin bir yolunu sunar.
Örneğin, `Rectangle`'ın `Circle` ve `Triangle`'ı da içeren olası şekiller kümesinden biri olduğunu belirtmek isteyebiliriz.
`Üçgen`'i de içeren olası şekillerden biri olduğunu söylemek isteyebiliriz. Bunu yapmak için Rust, bu olasılıkları bir enum olarak kodlamamıza izin verir.

Kodda ifade etmek isteyebileceğimiz bir duruma bakalım ve bu durumda enumların
neden yararlı ve yapıdan daha uygun olduğunu görelim. IP adresleriyle çalışmamız gerektiğini varsayalım.
Şu anda IP adresleri için iki ana standart kullanılmaktadır:
dördüncü sürüm ve altıncı sürüm. Bunlar, programımızın karşılaşacağı tek olasılıklar olduğundan
programımızın karşılaşacağı tek olasılıklar olduğundan, tüm olası
varyantları _enumerate_ edebiliriz, bu da enumerasyonun adını aldığı yerdir.

Herhangi bir IP adresi, dördüncü sürüm veya altıncı sürüm adresi olabilir, ancak
aynı anda her ikisi birden olamaz. IP adreslerinin bu özelliği, enum veri
yapısını uygun hale getirir, çünkü bir enum değeri yalnızca varyantlarından biri olabilir.
Hem sürüm dört hem de sürüm altı adresleri temelde IP
adresleridir, bu nedenle kod herhangi bir IP adresi türüne uygulanan durumları işlerken
aynı tür olarak ele alınmalıdır.

Bu kavramı kodda, bir `IpAddrKind` numaralandırması tanımlayarak ve
bir IP adresinin olabileceği olası türleri, `V4` ve `V6` listeleyerek ifade edebiliriz. Bunlar
numaralandırmanın varyantlarıdır:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` artık kodumuzun başka yerlerinde kullanabileceğimiz özel bir veri türüdür.

### Enum Değerleri

`IpAddrKind`'in iki varyantının her birinin örneklerini şu şekilde oluşturabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Enum'un varyantlarının tanımlayıcısının altında ad alanı olduğunu ve ikisini ayırmak için çift iki nokta üst üste kullandığımızı unutmayın.
Bu, artık her iki değer de
`IpAddrKind::V4` ve `IpAddrKind::V6` aynı türde olduğu için kullanışlıdır: `IpAddrKind`.
Örneğin, herhangi bir `IpAddrKind` alan bir işlev tanımlayabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

Ve bu işlevi her iki varyantla da çağırabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Enum kullanmanın daha da fazla avantajı vardır. IP adresi türümüzü daha fazla düşündüğümüzde,
şu anda gerçek IP adresi _verilerini_ depolamanın bir yolu yok;
sadece _türünü_ biliyoruz. 5. Bölümde yapıları öğrendiğinize göre,
Listing 6-1'de gösterildiği gibi bu sorunu yapılarla çözmek isteyebilirsiniz.
Listing 6-1. Yapılarla IP Adresleri

<Listing number="6-1" caption="Storing the data and `IpAddrKind` variant of an IP address using a `struct`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

</Listing>

Burada, iki alanı olan bir `IpAddr` yapısı tanımladık: `kind` alanı
`IpAddrKind` türündedir (daha önce tanımladığımız enum) ve `address` alanı
`String` türündedir. Bu yapının iki örneği var. İlki `home`,
ve `kind` değeri `IpAddrKind::V4` olup, ilişkili adres
verisi `127.0.0.1`'dir. İkinci örnek ise `loopback`. Bu örneğin `kind` değeri, `IpAddrKind`'in diğer
varyantı olan `V6` olup, ilişkili adresi `::1`'dir
. `kind` ve `address` değerlerini bir araya getirmek için bir yapı kullandık,
böylece varyant değerle ilişkilendirildi.

Ancak, aynı kavramı sadece bir enum kullanarak temsil etmek daha kısadır:
yapı içindeki bir enum yerine, verileri doğrudan her enum
varyantına koyabiliriz. `IpAddr` enumunun bu yeni tanımı, hem `V4` hem de `V6`
varyantlarının ilişkili `String` değerlerine sahip olacağını belirtir:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Enum'un her bir varyantına doğrudan veri ekliyoruz, bu nedenle ekstra bir yapıya gerek yok.
Burada, enum'ların nasıl çalıştığına dair başka bir ayrıntıyı da daha kolay görebiliriz:
tanımladığımız her enum varyantının adı, enum'un bir örneğini oluşturan bir işlev haline gelir.
Yani, `IpAddr::V4()`, bir `String` argümanı alan ve `IpAddr` türünde bir örnek döndüren bir işlev çağrısıdır. Yani, `IpAddr::V4()`, bir `String` argümanı alan ve `IpAddr` türünde bir örnek döndüren bir işlev çağrısıdır.
Enum'u tanımlamanın bir sonucu olarak bu yapıcı işlev otomatik olarak tanımlanır.
Enum'un bir değişkenini oluşturmak için, enum'un bir değişkenini içeren bir değişken tanımlamanız yeterlidir.
Örneğin,

Yapı yerine enum kullanmanın bir başka avantajı da, her bir varyantın
farklı türlerde ve miktarlarda ilişkili verilere sahip olabilmesidir. Sürüm dört IP
adresleri her zaman 0 ile 255 arasında değerlere sahip dört sayısal bileşene sahiptir.
`V4` adreslerini dört `u8` değeri olarak depolamak, ancak
`V6` adreslerini tek bir `String` değeri olarak ifade etmek istersek, bunu bir yapı ile
yapamayız. Enumlar bu durumu kolaylıkla halleder:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Sürüm dört ve sürüm altı IP adreslerini depolamak için veri yapılarını tanımlamanın birkaç farklı yolunu gösterdik.
Ancak, IP adreslerini depolamak ve bunların türünü kodlamak o kadar yaygın bir işlemdir ki, [standart
kütüphanede kullanabileceğimiz bir tanım vardır!]
[IpAddr]<!-- yok say --> [IpAddr]<!-- ignore --> Standart kütüphanenin `IpAddr`'yi nasıl tanımladığına
bakalım: tanımladığımız ve kullandığımız enum ve varyantların aynısına sahiptir,
ancak adres verilerini varyantların içine iki farklı yapı şeklinde gömer ve
bu yapılar her varyant için farklı şekilde tanımlanır:
```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Bu kod, enum varyantının içine her türlü veriyi koyabileceğinizi gösterir:
örneğin, dizeler, sayısal türler veya yapılar. Hatta başka bir
enum bile ekleyebilirsiniz! Ayrıca, standart kütüphane türleri genellikle sizin
düşünebileceğinizden çok daha karmaşık değildir.

Standart kütüphanede `IpAddr` için bir tanım bulunsa da,
standart kütüphanenin tanımını kapsamımıza almadığımız için
çakışma olmadan kendi tanımımızı oluşturabilir ve kullanabiliriz. Türleri kapsamımıza alma konusunda
daha fazla bilgiyi 7. bölümde ele alacağız.

Listing 6-2'deki enum örneğine bakalım: bu örnekte, varyantlara çok çeşitli
türler gömülüdür.

<Listing number="6-2" caption="A `Message` enum whose variants each store different amounts and types of values">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

</Listing>

Bu enum, farklı türlere sahip dört varyant içerir:

- `Quit`: Hiçbir veri içermez
- `Move`: Yapı gibi adlandırılmış alanlar içerir
- `Write`: Tek bir `String` içerir
- `ChangeColor`: Üç `i32` değeri içerir

Listing 6-2'deki gibi varyantları olan bir enum tanımlamak, farklı türde yapı tanımları tanımlamaya benzer, ancak enum
`struct` anahtar kelimesini kullanmaz ve tüm varyantlar `Message`
türü altında gruplandırılır. Aşağıdaki yapılar, önceki enum
varyantlarının içerdiği verilerin aynısını içerebilir:
- `Structure`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Ancak, her biri kendi türüne sahip farklı yapılar kullanmış olsaydık,
Listing 6-2'de tanımlanan tek bir tür olan `Message` enum ile olduğu kadar kolay bir şekilde
bu tür mesajları alan bir işlev tanımlayamazdık.

Enumlar ve yapılar arasında bir benzerlik daha vardır: `impl` kullanarak yapılarda yöntemler tanımlayabildiğimiz gibi,
enumlarda da yöntemler tanımlayabiliriz.
İşte `Message` enumunda tanımlayabileceğimiz `call` adlı bir yöntem:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

Yöntemin gövdesi, `self` kullanarak yöntemi çağırdığımız değeri alır.
Bu örnekte, `Message::Write(String::from(“hello”))` değerine sahip `m` değişkenini
oluşturduk ve `m.call()` çalıştırıldığında `call` yönteminin gövdesinde `self`
bu değeri alır.

Standart kütüphanede çok yaygın ve kullanışlı olan başka bir enum'a bakalım:
`Option`.

### `Option` Enum ve Null Değerlere Göre Avantajları

Bu bölümde, standart kütüphane tarafından tanımlanan başka bir enum olan `Option`'ın
bir vaka çalışması ele alınmaktadır. `Option` türü, bir değerin bir şey olabileceği veya hiçbir şey olamayacağı
çok yaygın bir senaryoyu kodlar.

Örneğin, boş olmayan bir listenin ilk öğesini isterseniz, bir değer alırsınız.
Boş bir listenin ilk öğesini isterseniz, hiçbir şey almazsınız.
Bu kavramı tür sistemi açısından ifade etmek, derleyicinin, ele almanız gereken tüm durumları ele alıp almadığınızı kontrol edebileceği anlamına gelir; bu
işlevsellik, diğer programlama dillerinde son derece yaygın olan hataları önleyebilir.
Bu, programlama dilinin bir özelliği olarak, programcının tüm olası durumları ele alması
gerektiği anlamına gelir.

Programlama dili tasarımı genellikle hangi özellikleri dahil edeceğiniz açısından düşünülür,
ancak hariç tuttuğunuz özellikler de önemlidir. Rust, diğer birçok dilde bulunan
null özelliğine sahip değildir. _Null_, orada bir değer olmadığı anlamına gelen bir
değerdir. Null özelliğine sahip dillerde, değişkenler her zaman iki durumdan birinde olabilir:
null veya null olmayan.

2009 yılında yaptığı “Null References: The Billion Dollar Mistake” (Null Referanslar: Milyar Dolarlık Hata) başlıklı sunumunda, null'un mucidi Tony
Hoare şöyle demiştir:

> Ben buna milyar dolarlık hatam diyorum. O zamanlar, nesne yönelimli bir dilde referanslar için ilk
> kapsamlı tip sistemini tasarlıyordum. Amacım
> referansların tüm kullanımının, derleyici tarafından otomatik olarak gerçekleştirilen
> kontrollerle tamamen güvenli olmasını sağlamaktı. Ancak, null referansı ekleme
> cazibesine karşı koyamadım, çünkü bunu uygulamak çok kolaydı.
> Bu, sayısız hataya, güvenlik açığına ve sistem
> çökmelerine yol açtı ve muhtemelen son kırk yılda milyarlarca dolarlık
> zarar ve kayba neden oldu.

Null değerlerin sorunu, null bir değeri null olmayan bir değer olarak kullanmaya çalıştığınızda
bir tür hata almanızdır. Bu null veya null olmayan
özelliği yaygın olduğu için, bu tür hataları yapmak son derece kolaydır.

Ancak, null'un ifade etmeye çalıştığı kavram hala yararlıdır: bir
null, şu anda geçersiz veya bir nedenden dolayı mevcut olmayan bir değerdir.

Sorun aslında kavramda değil, belirli bir
uygulamada. Bu nedenle, Rust'ta null değerler yoktur, ancak bir değerin mevcut veya mevcut olmadığı kavramını kodlayabilen bir enum vardır.
Bu enum
`Option<T>`'dir ve [standart kütüphane tarafından][option]<!-- ignore -->
aşağıdaki gibi tanımlanmıştır:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` enum o kadar kullanışlıdır ki, önsözde bile yer almaktadır; onu
kapsama açıkça dahil etmenize gerek yoktur. Varyantları da önsözde yer
almaktadır: `Option::` önekini kullanmadan doğrudan `Some` ve `None` kullanabilirsiniz.
`Option<T>` enum hala normal bir enumdur ve `Some(T)` ve
`None` hala `Option<T>` türünün varyantlarıdır.

`<T>` sözdizimi, henüz bahsetmediğimiz bir Rust özelliğidir. Bu,
genel bir tür parametresidir ve genel türleri 10. bölümde daha ayrıntılı olarak ele alacağız.
Şimdilik bilmeniz gereken tek şey, `<T>`'nin `Option` enumunun `Some` varyantının
herhangi bir türden bir veri parçası tutabileceği ve `T` yerine kullanılan her
somut türün genel `Option<T>` türünü farklı bir tür haline getirdiği
dir. İşte `Option` değerlerini sayı türlerini ve karakter türlerini tutmak için
kullanmanın bazı örnekleri:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

`some_number` türünün türü `Option<i32>`'dir. `some_char` türünün türü ise
`Option<char>`'dir, bu farklı bir türdür. Rust bu türleri çıkarabilir çünkü
`Some` varyantı içinde bir değer belirttik. `absent_number` için Rust,
genel `Option` türünü açıklamamızı gerektirir: derleyici, yalnızca
`None` değerine bakarak karşılık gelen `Some` varyantının tutacağı türü
çıkaramaz. Burada Rust'a, `absent_number`'ın türünün
`Option<i32>` olduğunu belirtiriz.

`Some` değerine sahip olduğumuzda, bir değerin mevcut olduğunu ve bu değerin
`Some` içinde tutulduğunu biliriz. `None` değerine sahip olduğumuzda, bu bir anlamda
null ile aynı anlama gelir: geçerli bir değerimiz yoktur. Peki, `Option<T>`'ye sahip olmak
null'a sahip olmaktan neden daha iyidir?

Kısaca, `Option<T>` ve `T` (burada `T` herhangi bir tür olabilir) farklı
türler olduğundan, derleyici `Option<T>` değerini kesin olarak geçerli bir değermiş gibi
kullanmamıza izin vermez. Örneğin, bu kod derlenmez, çünkü
`i8` değerini `Option<i8>` değerine eklemeye çalışır:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Bu kodu çalıştırırsak, aşağıdaki gibi bir hata mesajı alırız:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Yoğun! Aslında, bu hata mesajı Rust'un
`i8` ve `Option<i8>` türlerini nasıl ekleyeceğini anlamadığı anlamına gelir, çünkü bunlar farklı türlerdir. Rust'ta
`i8` gibi bir türün değerine sahip olduğumuzda, derleyici her zaman geçerli bir değerimiz olmasını sağlar. Bu değeri kullanmadan önce null olup olmadığını kontrol etmek zorunda kalmadan
güvenle devam edebiliriz.
Sadece `Option<i8>` (veya Sadece `Option<i8>` (veya
çalıştığımız değer türü ne olursa olsun) olduğunda, bir değer olmaması
olasılığından endişelenmemiz gerekir ve derleyici, değeri kullanmadan önce
bu durumu ele almamızı sağlar.

Diğer bir deyişle, `Option<T>` ile `T` işlemleri gerçekleştirebilmek için önce
`Option<T>`'yi `T`'ye dönüştürmeniz gerekir. Genel olarak, bu, null ile ilgili en yaygın sorunlardan birini
yakalamaya yardımcı olur: aslında null olan bir şeyin null olmadığını varsaymak.

Null olmayan bir değeri yanlış bir şekilde varsayma riskini ortadan kaldırmak, kodunuza
daha fazla güvenmenize yardımcı olur. Null olabilecek bir değere sahip olmak için,
o değerin türünü `Option<T>` olarak açıkça seçmelisiniz.
Ardından, bu değeri kullandığınızda, değerin null olduğu durumu açıkça
ele almanız gerekir. Bir değerin türü `Option<T>` olmayan her yerde,
değerin null olmadığını güvenle varsayabilirsiniz. Bu, null'un yaygınlığını sınırlamak ve Rust kodunun güvenliğini artırmak için Rust'ta
kasıtlı olarak alınan bir tasarım kararıdır.
Bu, null'un yaygınlığını sınırlamak ve Rust kodunun güvenliğini artırmak için

Peki, `Option<T>` türünde bir değeriniz olduğunda, bu değeri kullanabilmek için `Some` varyantından `T` değerini nasıl elde edersiniz?
`Option<T>` enum'u, çeşitli durumlarda yararlı olan çok sayıda yönteme sahiptir;
bunları [belgelerinde][docs]<!-- ignore -->.
`Option<T>` üzerindeki yöntemlere aşina olmak, `Option<T>` üzerindeki yöntemlere aşina olmak,
Rust ile olan yolculuğunuzda son derece yararlı olacaktır.
Genel olarak, bir `Option<T>` değerini kullanmak için,

her varyantı işleyecek bir koda ihtiyacınız vardır. Yalnızca bir
`Some(T)` değeriniz olduğunda çalışacak ve bu kodun iç `T` değerini kullanmasına izin verilen bir koda ihtiyacınız vardır.
`None` değeriniz olduğunda yalnızca başka bir kodun çalışmasını istiyorsunuz ve bu kodun `None` değeriniz olduğunda çalışacak başka bir kod istersiniz ve bu kodun
`T` değeri kullanılamaz. `match` ifadesi, enumlarla kullanıldığında tam da bunu yapan bir kontrol akışı yapısıdır:
enumun hangi varyantına sahip olduğuna bağlı olarak farklı kodlar çalıştırır ve bu kodlar, eşleşen değerin içindeki verileri kullanabilir.
`match` ifadesi, enumlarla kullanıldığında tam da bunu yapan bir kontrol akışı yapısıdır:
enumun hangi varyantına sahip olduğuna bağlı olarak farklı kodlar çalıştırır ve bu kodlar, eşleşen değerin içindeki verileri kullanabilir.
`match` ifadesi, enumlarla kullanıldığında tam da bunu yapan bir kontrol akışı yapısıdır:

[IpAddr]: ../std/net/enum.IpAddr.md
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
