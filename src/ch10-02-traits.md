## Özellikler: Ortak Davranışın Tanımlanması

Bir _trait_ belirli bir tipin sahip olduğu ve
diğer tiplerle paylaşabileceği işlevselliği tanımlar. Paylaşılan davranışı soyut bir şekilde tanımlamak için özellikleri kullanabiliriz. Genel bir tipin
belirli bir davranışa sahip herhangi bir tip olabileceğini belirtmek için
_trait bounds_ kullanabiliriz.

> Not: Özellikler, bazı farklılıkları olmakla birlikte, diğer
> dillerinde genellikle _interfaces_ olarak adlandırılan bir özelliğe benzer.

### Bir Özellik Tanımlama

Bir türün davranışı, o tür üzerinde çağırabileceğimiz yöntemlerden oluşur. Tüm
türleri üzerinde aynı yöntemleri çağırabiliyorsak, farklı
türleri aynı davranışı paylaşır. Özellik tanımları, bir amacı gerçekleştirmek için gerekli olan bir dizi davranışı
tanımlamak üzere yöntem imzalarını bir araya getirmenin bir yoludur.

Örneğin, çeşitli türde ve
miktarda metin tutan birden fazla yapımız olduğunu varsayalım:
belirli bir konumda dosyalanmış bir haberi tutan bir `NewsArticle` yapısı ve en fazla 280 karaktere sahip olabilen bir `SocialPost`
ile birlikte yeni bir gönderi mi, yeniden gönderi mi yoksa başka bir gönderiye
yanıt mı olduğunu gösteren meta veriler.

Bir `NewsArticle` veya
`SocialPost` örneğinde depolanabilecek verilerin özetlerini
görüntüleyebilen `aggregator` adlı bir medya toplayıcı kütüphane sandığı yapmak istiyoruz. Bunu yapmak için, her türden bir özete ihtiyacımız var ve
adresinden bir örnek üzerinde bir `summarize` yöntemini çağırarak bu özeti talep edeceğiz. Liste
10-12, bu
davranışını ifade eden bir public `Summary` özelliğinin tanımını göstermektedir.

<Listing number="10-12" file-name="src/lib.rs" caption="A `Summary` trait that consists of the behavior provided by a `summarize` method">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

</Listing>

Burada, `trait` anahtar sözcüğünü ve ardından bu durumda `Summary` olan özelliğin adını
kullanarak bir özellik bildiriyoruz. Ayrıca,
adresinde birkaç örnekte göreceğimiz gibi, bu crate'e bağlı
crate'lerinin de bu özelliği kullanabilmesi için özelliği `pub` olarak bildiriyoruz. Küme parantezlerinin içinde, bu özelliği uygulayan türlerin davranışlarını tanımlayan
yöntem imzalarını bildiririz;
bu durumda `fn summarize(&self) -> String`dir.

Yöntem imzasından sonra, küme
parantezleri içinde bir uygulama sağlamak yerine noktalı virgül kullanırız. Bu özelliği uygulayan her tür
yöntemin gövdesi için kendi özel davranışını sağlamalıdır. Derleyici, `Summary` özelliğine sahip herhangi bir türün tam olarak bu imza ile tanımlanmış `summarize`
yöntemine sahip olacağını
zorunlu kılacaktır.

Bir özellik, gövdesinde birden fazla yönteme sahip olabilir: yöntem imzaları her satırda bir tane olmak üzere
listelenir ve her satır noktalı virgülle biter.

### Bir Tür Üzerinde Özellik Uygulamak

Artık `Summary` özelliğinin yöntemlerinin istenen imzalarını tanımladığımıza göre,
medya toplayıcımızdaki türler üzerinde bunu uygulayabiliriz. Liste 10-13,
`NewsArticle` yapısı üzerinde,
`summarize` dönüş değerini oluşturmak için
başlığı, yazarı ve konumu kullanan `Summary` özelliğinin bir uygulamasını göstermektedir. `SocialPost` yapısı için, `summarize` özelliğini
kullanıcı adı ve ardından gönderi içeriğinin
zaten 280 karakterle sınırlı olduğunu varsayarak gönderinin tüm metni olarak tanımlarız.

<Listing number="10-13" file-name="src/lib.rs" caption="Implementing the `Summary` trait on the `NewsArticle` and `SocialPost` types">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

</Listing>

Bir tip üzerinde bir özellik uygulamak, normal metotları uygulamaya benzer. Aradaki
fark, `impl`den sonra uygulamak istediğimiz özellik adını yazmamız,
ardından `for` anahtar sözcüğünü kullanmamız ve ardından özelliği uygulamak istediğimiz
türün adını belirtmemizdir. Impl` bloğunun içine, özellik tanımının tanımladığı
yöntem imzalarını koyuyoruz. Her
imzasından sonra noktalı virgül eklemek yerine, küme parantezleri kullanıyoruz ve yöntem gövdesini, özelliğin yöntemlerinin belirli bir tür için sahip olmasını istediğimiz belirli
davranışıyla dolduruyoruz.

Artık kütüphane `NewsArticle` ve
`SocialPost` üzerinde `Summary` özelliğini uyguladığına göre, crate kullanıcıları
`NewsArticle` ve `SocialPost` örnekleri üzerindeki özellik yöntemlerini normal yöntemleri çağırdığımız şekilde çağırabilir. Tek
farkı, kullanıcının
türlerinin yanı sıra özelliği de kapsama getirmesi gerektiğidir. İşte ikili bir crate'in `aggregator`
kütüphane crate'imizi nasıl kullanabileceğine dair bir örnek:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

Bu kod şu çıktıyı üretir: `1 new post: horse_ebooks: of course, as you probably already know, people`.

`Aggregator` crate'ine bağımlı olan diğer crate'ler de kendi tiplerinde `Summary` uygulamak için `Summary`
trait'ini kapsam içine alabilirler. adresinde dikkat edilmesi gereken bir kısıtlama, bir özelliği bir tür üzerinde yalnızca özellik veya
türü ya da her ikisi de crate'imiz için yerelse uygulayabileceğimizdir. Örneğin,
`aggregator` crate işlevselliğimizin bir parçası olarak `SocialPost` gibi özel bir tür üzerinde `Display` gibi standart
kütüphane özelliklerini uygulayabiliriz çünkü `SocialPost` türü
`aggregator` crate'imiz için yereldir. Ayrıca `Summary` özelliğini
`aggregator` crate'imizde `Vec<T>` üzerinde uygulayabiliriz çünkü `Summary` özelliği `aggregator`
crate'imiz için yereldir.

Ancak harici özellikleri harici türler üzerinde uygulayamayız. Örneğin,
`Display` özelliğini `aggregator` crate'imiz içinde `Vec<T>` üzerinde uygulayamayız çünkü
`Display` ve `Vec<T>` standart kütüphanede tanımlanmıştır ve
bizim `aggregator` crate'imiz için yerel değildir. Bu kısıtlama,
_coherence_ ve daha spesifik olarak _orphan rule_ adı verilen bir özelliğin parçasıdır, çünkü
ana türü mevcut değildir. Bu kural, başkalarının kodunun
sizin kodunuzu bozamamasını veya tam tersini sağlar. Bu kural olmasaydı, iki crate aynı tip için aynı özelliği
uygulayabilirdi ve Rust hangi uygulamayı
kullanacağını bilemezdi.

### Varsayılan Uygulamalar

Bazen her türdeki tüm yöntemler için uygulama gerektirmek yerine
bir özellikteki yöntemlerin bazıları veya tümü için varsayılan davranışa sahip olmak yararlıdır.
Daha sonra, özelliği belirli bir tür üzerinde uygularken, her yöntemin varsayılan davranışını koruyabilir veya
geçersiz kılabiliriz.

Liste 10-14'te,
Liste 10-12'de yaptığımız gibi yalnızca yöntem imzasını tanımlamak yerine,
`Summary` özelliğinin `summarize` yöntemi için varsayılan bir dize belirtiyoruz.

<Listing number="10-14" file-name="src/lib.rs" caption="Defining a `Summary` trait with a default implementation of the `summarize` method">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

</Listing>

`NewsArticle` örneklerini özetlemek üzere varsayılan bir uygulama kullanmak için
adresinde `impl` Summary for NewsArticle {}` şeklinde boş bir `impl` bloğu belirtiyoruz.

Artık `NewsArticle`
üzerinde `summarize` yöntemini doğrudan tanımlamıyor olsak da, varsayılan bir uygulama sağladık ve
`NewsArticle`'ın `Summary` özelliğini uyguladığını belirttik. Sonuç olarak,
adresinden `NewsArticle` örneğindeki `summarize` yöntemini aşağıdaki gibi çağırabiliriz:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

Bu kod şu çıktıyı üretir: `New article available! (Read more...)`.

Varsayılan bir uygulama oluşturmak, Listing 10-13'teki `SocialPost` üzerindeki `Summary` uygulamasında
herhangi bir değişiklik yapmamızı gerektirmez. Bunun nedeni
varsayılan bir uygulamayı geçersiz kılmak için kullanılan sözdiziminin
varsayılan bir uygulaması olmayan bir özellik yöntemini uygulamak için kullanılan
sözdizimiyle aynı olmasıdır.

Varsayılan uygulamalar
diğer yöntemlerin varsayılan bir uygulaması olmasa bile aynı özellikteki diğer yöntemleri çağırabilir. Bu şekilde, bir özellik
birçok yararlı işlevsellik sağlayabilir ve uygulayıcıların yalnızca
bunun küçük bir bölümünü belirtmesini gerektirebilir. Örneğin, `Summary` özelliğini, uygulaması gerekli olan bir
`summarize_author` yöntemine sahip olacak şekilde tanımlayabilir ve ardından
`summarize_author` yöntemini çağıran varsayılan bir uygulamaya sahip bir
`summarize` yöntemi tanımlayabiliriz:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

Bu `Summary` versiyonunu kullanmak için, trait'i bir tipe uygularken yalnızca `summarize_author`'ı tanımlamamız yeterlidir.

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

`summarize_author` metodunu tanımladıktan sonra,
`SocialPost` yapısının örnekleri üzerinde `summarize` metodunu çağırabiliriz ve `summarize` metodunun varsayılan uygulaması, sağladığımız
`summarize_author` tanımını çağıracaktır. `summarize_author` uygulamasını gerçekleştirdiğimiz için, `Summary` özelliği bize daha fazla kod yazmamızı gerektirmeden
`summarize` yönteminin davranışını vermiştir. İşte
bunun nasıl göründüğü:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

Bu kod şunu yazdırır: `1 yeni gönderi: (Daha fazlası için @horse_ebooks...)`.

Aynı yöntemin bir
geçersiz kılma uygulamasından varsayılan uygulamayı çağırmanın mümkün olmadığını unutmayın.

### Parametre Olarak Özellikler

Artık özellikleri nasıl tanımlayacağınızı ve uygulayacağınızı bildiğinize göre, birçok farklı türü kabul eden fonksiyonları tanımlamak için
özelliklerini nasıl kullanacağımızı keşfedebiliriz. Listing 10-13'te `NewsArticle` ve `SocialPost` türleri üzerinde uyguladığımız
`Summary` özelliğini, `Summary`
özelliğini uygulayan bir türden olan `item` parametresi üzerinde `summarize` yöntemini
çağıran bir `notify` işlevi tanımlamak için kullanacağız. Bunu yapmak için, aşağıdaki gibi `impl Trait` sözdizimini kullanırız:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

`item` parametresi için somut bir tür yerine, `impl`
anahtar sözcüğünü ve özellik adını belirtiriz. Bu parametre,
belirtilen özelliği uygulayan herhangi bir türü kabul eder. `Notify` gövdesinde, `Summarize` gibi `Summary` özelliğinden gelen `item`
üzerindeki herhangi bir yöntemi çağırabiliriz. adresinden `notify` fonksiyonunu çağırabilir ve `NewsArticle` veya `SocialPost` fonksiyonlarının herhangi bir örneğini aktarabiliriz. işlevini `String` veya `i32` gibi başka bir türle çağıran kod
adresinde derlenmeyecektir çünkü bu türler `Summary` özelliğini uygulamamaktadır.

<!-- Old headings. Do not remove or links may break. -->

<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### Trait Bound Sözdizimi

Basit durumlar için `impl Trait` sözdizimi işe yarar, ancak aslında _trait bound_ olarak bilinen daha uzun bir form için
sugar sözdizimidir; şuna benzer:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
 println!("Breaking news! {}", item.summarize());
}
```

Bu uzun form önceki bölümdeki örneğe eşdeğerdir ancak
daha ayrıntılıdır. Özellik sınırlarını
genel tip parametresinin bildirimiyle birlikte iki nokta üst üste işaretinden sonra ve açılı parantezlerin içine yerleştiririz.

Basit
durumlarında `impl Trait` sözdizimi kullanışlıdır ve daha özlü bir kod sağlarken, daha kapsamlı trait bound sözdizimi diğer
durumlarında daha fazla karmaşıklık ifade edebilir. Örneğin, `Summary` uygulayan iki parametreye sahip olabiliriz. Bunu
adresinde `impl Trait` sözdizimi ile yapmak aşağıdaki gibi görünür:

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

Bu fonksiyonun `item1` ve
`item2` parametrelerinin farklı tiplere sahip olmasına izin vermesini istiyorsak (her iki tip de `Summary` uyguladığı sürece) `impl Trait` kullanmak uygundur. Ancak
her iki parametreyi de aynı tipe sahip olmaya zorlamak istiyorsak, aşağıdaki gibi bir
trait bound kullanmalıyız:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

`öğe1` ve `öğe2`
parametrelerinin türü olarak belirtilen `T` genel türü, işlevi `öğe1` ve `öğe2` için argüman olarak aktarılan
değerinin somut türünün aynı olması gerektiği şekilde kısıtlar.

#### `+` Sözdizimiyle Birden Fazla Özellik Sınırının Belirtilmesi

Birden fazla özellik bağı da belirleyebiliriz. Diyelim ki `notify` öğesinin
görüntüleme biçimlendirmesinin yanı sıra `summarize` özelliğini de kullanmasını istedik: `notify`
tanımında `item` öğesinin hem `Display` hem de `Summary` özelliklerini uygulaması gerektiğini belirtiriz. Bunu
adresinde `+` sözdizimini kullanarak yapabiliriz:

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

+` sözdizimi, genel tipler üzerindeki özellik sınırlarıyla da geçerlidir:

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

Belirtilen iki özellik sınırı ile `notify` gövdesi `summarize`
çağrısı yapabilir ve `item`i biçimlendirmek için `{}` kullanabilir.

#### `where` Cümleleri ile Daha Net Özellik Sınırları

Çok fazla özellik sınırı kullanmanın dezavantajları vardır. Her jenerik kendi trait
sınırlarına sahiptir, bu nedenle birden fazla jenerik tip parametresi olan fonksiyonlar, fonksiyonun adı ve parametre listesi arasında çok sayıda
trait bound bilgisi içerebilir ve
fonksiyon imzasının okunmasını zorlaştırabilir. Bu nedenle, Rust,
imzasından sonra bir `where` cümlesi içinde özellik sınırlarını belirtmek için alternatif
sözdizimine sahiptir. Yani, bunu yazmak yerine:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

bunun gibi bir `where` cümlesi kullanabiliriz:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

Bu işlevin imzası daha az karmaşıktır: işlev adı, parametre listesi,
ve dönüş türü birbirine yakındır, çok sayıda özellik içermeyen bir işleve benzer
sınırları.

### Özellikleri Uygulayan Geri Dönüş Türleri

Burada gösterildiği gibi, bir trait uygulayan bir türün
değerini döndürmek için return konumunda `impl Trait` sözdizimini de kullanabiliriz:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

Dönüş türü için `impl Summary` kullanarak,
`returns_summarizable` işlevinin somut türü adlandırmadan `Summary`
özelliğini uygulayan bir tür döndürdüğünü belirtiyoruz. Bu durumda, `returns_summarizable`
bir `SocialPost` döndürür, ancak bu işlevi çağıran kodun bunu
bilmesine gerek yoktur.

Bir dönüş türünü yalnızca uyguladığı özelliğe göre belirtme yeteneği
özellikle
Bölüm 13'te ele aldığımız kapamalar ve yineleyiciler bağlamında kullanışlıdır. Kapanışlar ve yineleyiciler yalnızca derleyicinin bildiği tipler ya da belirtilmesi çok uzun olan
tipleri oluşturur. `Impl Trait` sözdizimi, çok uzun bir tür yazmanıza gerek kalmadan
bir fonksiyonun `Iterator` özelliğini uygulayan bir tür döndürdüğünü
kısaca belirtmenizi sağlar.

Ancak, `impl Trait` özelliğini yalnızca tek bir tür döndürüyorsanız kullanabilirsiniz. Örneğin
için, dönüş türü `impl Summary` olarak belirtilen ve
ile bir `NewsArticle` veya bir `SocialPost` döndüren bu kod çalışmayacaktır:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

Derleyicide `impl Trait` sözdiziminin nasıl uygulandığına ilişkin
kısıtlamaları nedeniyle bir `NewsArticle` veya bir `SocialPost` döndürülmesine izin verilmez.
Bu davranışa sahip bir fonksiyonun nasıl yazılacağını [“Using Trait
Objects That Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore bölümünde ele alacağız
--> Bölüm 18'in bir bölümü.

### Yöntemleri Koşullu Olarak Uygulamak için Özellik Sınırlarını Kullanma

Genel tip parametreleri kullanan bir `impl` bloğu ile bağlı bir özellik kullanarak,
belirtilen
özelliklerini uygulayan tipler için yöntemleri koşullu olarak uygulayabiliriz. Örneğin, Liste 10-15'teki `Pair<T>` türü her zaman
`new` fonksiyonunu uygulayarak yeni bir `Pair<T>` örneği döndürür (Bölüm 5'teki
[“Yöntem Tanımlama”][methods]<!-- ignore --> bölümünden `Self`
'nin `impl` bloğunun türü için bir tür takma adı olduğunu hatırlayın, bu durumda
`Pair<T>`). Ancak bir sonraki `impl` bloğunda, `Pair<T>` yalnızca iç tipi `T`, karşılaştırmaya _ve_ yazdırmaya olanak sağlayan `PartialOrd` özelliğini
uyguluyorsa
`cmp_display` yöntemini uygular.

<Listing number="10-15" file-name="src/lib.rs" caption="Conditionally implementing methods on a generic type depending on trait bounds">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

</Listing>

Ayrıca
başka bir özelliği uygulayan herhangi bir tür için bir özelliği koşullu olarak uygulayabiliriz. Bir özelliğin
sınırlarını karşılayan herhangi bir tür üzerindeki uygulamalarına _blanket uygulamaları_ denir ve
Rust standart kütüphanesinde yaygın olarak kullanılır. Örneğin, standart kütüphane, `Display` özelliğini uygulayan herhangi bir tür üzerinde
`ToString` özelliğini uygular. Standart kütüphanedeki `impl`
bloğu bu koda benzer:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Standart kütüphane bu genel uygulamaya sahip olduğundan, `ToString` özelliği tarafından tanımlanan
`to_string` yöntemini
`Display` özelliğini uygulayan herhangi bir tür üzerinde çağırabiliriz. Örneğin, tamsayılar `Display` özelliğini uyguladığından, tamsayıları karşılık gelen
`String` değerlerine şu şekilde dönüştürebiliriz:

```rust
let s = 3.to_string();
```

Blanket uygulamaları,
“Implementors” bölümündeki özellik belgelerinde görünür.

Özellikler ve özellik sınırları
yinelemeyi azaltmak için genel tip parametrelerini kullanan kodlar yazmamızı ve aynı zamanda derleyiciye genel
tipinin belirli bir davranışa sahip olmasını istediğimizi belirtmemizi sağlar. Derleyici daha sonra kodumuzla birlikte kullanılan tüm somut tiplerin
doğru davranışı sağlayıp sağlamadığını kontrol etmek için trait bound
bilgilerini kullanabilir. Dinamik olarak yazılan dillerde, yöntemi tanımlamayan bir tür üzerinde bir yöntem çağırırsak
çalışma zamanında bir hata alırız. Ancak
Rust bu hataları derleme zamanına taşır, böylece kodumuz daha çalışmadan önce
sorunları düzeltmek zorunda kalırız. Ek olarak, derleme
zamanında zaten kontrol ettiğimiz için çalışma zamanında davranış kontrolü yapan
kodu yazmak zorunda kalmayız. Bunu yapmak, jeneriklerin esnekliğinden
vazgeçmek zorunda kalmadan performansı artırır.

[using-trait-objects-that-allow-for-values-of-different-types]: ch18-02-trait-objects.md#using-trait-objects-that-allow-for-values-of-different-types
[methods]: ch05-03-method-syntax.md#defining-methods
