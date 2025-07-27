## Makrolar

Bu kitap boyunca `println!` gibi makrolar kullandık, ancak makroların ne olduğunu ve nasıl çalıştığını tam olarak
incelemedik. _Makro_ terimi, Rust'ta bir dizi özelliği ifade eder: `macro_rules!` ile _bildirimsel_ makrolar ve üç tür
_prosedürel_ makro:
- `#[derive]` özniteliği ile eklenen kodu tanımlayan özel `#[derive]` makroları

- Yapılar ve enumlarda kullanılan `derive` özniteliği ile eklenen kodu belirten özel `#[derive]` makroları
  - Herhangi bir öğede kullanılabilen özel öznitelikleri tanımlayan öznitelik benzeri makrolar
- İşlev çağrılarına benzeyen ancak argümanları olarak belirtilen belirteçler üzerinde çalışan işlev benzeri makrolar
- Herhangi bir öğede kullanılabilen özel öznitelikleri tanımlayan öznitelik benzeri makrolar
- İşlev çağrılarına benzeyen ancak argümanları olarak belirtilen belirteçler üzerinde çalışan işlev benzeri makrolar

Bunların her birini sırayla ele alacağız, ancak önce, zaten fonksiyonlarımız varken neden
makrolara ihtiyaç duyduğumuzu inceleyelim.

### Makrolar ve İşlevler Arasındaki Fark

Temel olarak, makrolar başka kodlar yazan kodlar yazmanın bir yoludur ve
bu, _metaprogramlama_ olarak bilinir. Ek C'de, çeşitli özelliklerin uygulamasını sizin için oluşturan `derive`
özelliğini ele alıyoruz. Kitap boyunca
`println!` ve `vec!` makrolarını da kullandık. Tüm bu
makrolar, manuel olarak yazdığınız koddan daha fazla kod üretmek için _genişler_.

Metaprogramlama, yazmanız ve
bakımını yapmanız gereken kod miktarını azaltmak için kullanışlıdır, bu da fonksiyonların rollerinden biridir. Ancak, makrolar
fonksiyonların sahip olmadığı bazı ek güçlere sahiptir.

Bir fonksiyon imzası, fonksiyonun sahip olduğu parametrelerin sayısını ve türünü
belirtmelidir. Makrolar ise değişken sayıda parametre alabilir:
`println!(“hello”)`'u tek bir argümanla veya
`println!(“hello {}”, name)`'u iki argümanla çağırabiliriz. Ayrıca, makrolar derleyici kodun anlamını yorumlamadan önce genişletilir,
bu nedenle bir makro, örneğin, belirli bir tür üzerinde bir özelliği uygulayabilir.
Bir fonksiyon bunu yapamaz, çünkü çalışma zamanında çağrılır ve bir özellik
derleme zamanında uygulanmalıdır.

Fonksiyon yerine makro kullanmanın dezavantajı, makro
tanımlarının fonksiyon tanımlarından daha karmaşık olmasıdır, çünkü
Rust kodu yazan Rust kodu yazıyorsunuz. Bu dolaylılık nedeniyle, makro tanımları
genellikle fonksiyon tanımlarından daha zor okunur, anlaşılır ve bakımı
zordur.

Makrolar ve fonksiyonlar arasındaki bir diğer önemli fark, makroları bir dosyada çağırmadan
önce tanımlamanız veya kapsam içine almanız gerekmesidir,
fonksiyonların ise herhangi bir yerde tanımlayıp herhangi bir yerde çağırabilmenizin aksine.

### Genel Metaprogramlama için `macro_rules!` ile Bildirimsel Makrolar

Rust'ta en yaygın olarak kullanılan makro türü _bildirimsel makro_'dur. Bunlar
bazen “örneklerle makrolar”, “`macro_rules!` makroları”
veya sadece “makrolar” olarak da adlandırılır. Temel olarak, bildirimsel makrolar Rust `match` ifadesine benzer bir şey yazmanıza
olanak tanır. Bölüm 6'da tartışıldığı gibi,
`match` ifadeleri bir ifadeyi alan, ifadenin
sonuç değerini desenlerle karşılaştıran ve ardından eşleşen desenle ilişkili kodu
çalıştıran kontrol yapılarıdır. Makrolar ayrıca bir değeri belirli kodlarla ilişkili
kalıplarla karşılaştırır: bu durumda değer, makroya aktarılan
Rust kaynak kodunun tam metnidir; kalıplar, bu kaynak kodunun
yapısı ile karşılaştırılır; ve her kalıpla ilişkili kod, eşleştiğinde
makroya aktarılan kodu değiştirir. Tüm bunlar derleme sırasında
gerçekleşir.

Bir makro tanımlamak için `macro_rules!` yapısını kullanırsınız. `vec!` makrosunun nasıl tanımlandığını inceleyerek
`macro_rules!`'un nasıl kullanıldığını keşfedelim. Bölüm 8'de
`vec!` makrosunu belirli değerlere sahip yeni bir vektör oluşturmak için nasıl kullanabileceğimizi
inceledik. Örneğin, aşağıdaki makro üç tamsayı içeren yeni bir vektör oluşturur
:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

`vec!` makrosunu kullanarak iki tamsayıdan oluşan bir vektör veya beş dizgi diliminden oluşan bir vektör de oluşturabiliriz. Aynı şeyi bir fonksiyon kullanarak yapamayız, çünkü değerlerin sayısını veya türünü önceden bilemeyiz.  Liste 20-35, `vec!` makrosunun biraz basitleştirilmiş bir tanımını göstermektedir.

<Listing number="20-35" file-name="src/lib.rs" caption="A simplified version of the `vec!` macro definition">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-35/src/lib.rs}}
```

</Listing>

> Not: Standart kütüphanedeki `vec!` makrosunun gerçek tanımı,
> doğru miktarda belleği önceden ayırmak için kod içerir. Bu kod,
> örneği daha basit hale getirmek için burada dahil etmediğimiz bir optimizasyondur.

`#[macro_export]` açıklaması, bu makronun, makronun tanımlandığı kutu
kapsamına alındığında kullanılabilir hale getirilmesi gerektiğini belirtir.
Bu açıklama olmadan, makro kapsam içine alınamaz. Bu açıklama olmadan, makro kapsam içine alınamaz.

Ardından, makro tanımını `macro_rules!` ve tanımladığımız makronun
adını ünlem işareti olmadan başlatırız. Bu durumda,
`vec` adının ardından makro tanımının gövdesini belirten küme parantezleri gelir.

`vec!` gövdesindeki yapı, `match`
ifadesinin yapısına benzer. Burada `( $( $x:expr ),* )` desenine sahip bir kol var,
ardından `=>` ve bu desenle ilişkili kod bloğu geliyor. Desen eşleşirse,
ilişkili kod bloğu çıkarılır. Bu makrodaki tek desen bu olduğu için,
eşleşmenin tek bir geçerli yolu vardır; diğer desenler hata ile sonuçlanır.
Daha karmaşık makrolar birden fazla kola sahiptir.
`match`

Makro tanımlarında geçerli desen sözdizimi, Bölüm 19'da ele alınan desen sözdiziminden farklıdır, çünkü makro desenleri değerlerle değil, Rust kod yapısıyla eşleştirilir. Liste 20-29'daki desen parçalarının ne anlama geldiğini inceleyelim; tam makro desen sözdizimi için [Rust Referansı][ref]'ye bakın.

İlk olarak, tüm deseni kapsayan bir parantez kümesi kullanıyoruz. Makro sisteminde, desene uyan Rust kodunu içerecek bir değişkeni tanımlamak için dolar işareti (`$`) kullanıyoruz. Dolar işareti, makro sisteminde desene uyan Rust kodunu içerecek bir değişkeni tanımladığımızı açıkça belirtir.

Öncelikle, tüm kalıbı kapsayacak şekilde bir dizi parantez kullanıyoruz. Makro sisteminde, kalıpla eşleşen Rust kodunu içerecek bir değişkeni
bildirmek için dolar işareti (`$`) kullanıyoruz. Dolar işareti, bunun normal bir Rust değişkeni değil, bir makro değişkeni olduğunu
açıkça belirtir.
Ardından, parantez içindeki kalıpla eşleşen değerleri yakalayan bir dizi parantez gelir. Bu değerler, Ardından,
parantez içindeki desenle eşleşen değerleri yakalayan bir parantez kümesi gelir
ve bu değerler değiştirme kodunda kullanılır. `$()` içinde `$x:expr` vardır, bu da herhangi bir
Rust ifadesiyle eşleşir ve ifadeye `$x` adını verir.

`$()`'nin ardından gelen virgül, `$()` içindeki kodla eşleşen kodun her bir örneği arasında
literal virgül ayırıcı karakterinin
bulunması gerektiğini belirtir. `*`, kalıbın `*`'den önce gelen her şeyin sıfır veya daha fazlasıyla eşleştiğini
belirtir.

Bu makroyu `vec![1, 2, 3];` ile çağırdığımızda, `$x` kalıbı üç
kez `1`, `2` ve `3` ifadeleriyle eşleşir.

Şimdi bu kol ile ilişkili kod gövdesindeki kalıba bakalım:
`$()*` içindeki `temp_vec.push()`, kalıbın kaç kez eşleştiğine bağlı olarak
`$()` ile eşleşen her parça için sıfır veya daha fazla kez oluşturulur.
`$x`, eşleşen her ifade ile değiştirilir. Bu makroyu `vec![1, 2, 3];` ile çağırdığımızda,
bu makro çağrısını değiştiren oluşturulan kod
aşağıdaki gibi olacaktır:

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

Herhangi bir türden herhangi bir sayıda argüman alabilen ve
belirtilen öğeleri içeren bir vektör oluşturmak için kod üretebilen bir makro tanımladık.

Makro yazma hakkında daha fazla bilgi edinmek için çevrimiçi belgelere veya
Daniel Keep tarafından başlatılan ve Lukas Wirth tarafından devam ettirilen [“The Little Book of Rust Macros”][tlborm] gibi diğer kaynaklara başvurun.

### Özniteliklerden Kod Oluşturmak için Prosedürel Makrolar

Makroların ikinci biçimi, daha çok bir işlev gibi davranan (ve bir prosedür türü olan)
prosedürel makrodur. _Prosedürel makrolar_, bazı kodları girdi olarak kabul eder,
bu kodlar üzerinde işlem yapar ve çıktı olarak bazı kodlar üretir;
bildirimsel makroların yaptığı gibi kalıplarla eşleştirme yapıp kodu başka kodlarla
değiştirmez. Prosedürel makroların üç türü vardır: özel `derive`,
özellik benzeri ve işlev benzeri. Hepsi de benzer şekilde çalışır.

Prosedürel makrolar oluştururken, tanımlar özel bir kutu türüne sahip kendi kutularında
bulunmalıdır. Bunun karmaşık teknik nedenleri vardır ve gelecekte
bunu ortadan kaldırmayı umuyoruz. Listing 20-36'da, bir
prosedürel makro tanımlamayı gösteriyoruz; burada `some_attribute`, belirli bir
makro çeşidini kullanmak için bir yer tutucudur.

<Listing number="20-36" file-name="src/lib.rs" caption="An example of defining a procedural macro">

```rust,ignore
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

</Listing>

Prosedürel makroyu tanımlayan işlev, girdi olarak bir `TokenStream` alır
ve çıktı olarak bir `TokenStream` üretir. `TokenStream` türü, Rust ile birlikte gelen
`proc_macro` kütüphanesi tarafından tanımlanır ve bir dizi
token'ı temsil eder. Bu, makronun özüdür: makronun üzerinde çalıştığı kaynak kodu
giriş `TokenStream`'i oluşturur ve makronun ürettiği kod
çıktı `TokenStream`'dir. İşlevin ayrıca, hangi tür prosedürel makro oluşturduğumuzu belirten
bir özniteliği vardır. Aynı crate'de birden fazla türde prosedürel makroya sahip olabiliriz.

Farklı türdeki prosedürel makrolara bakalım. Önce

Farklı türdeki prosedürel makroları inceleyelim. Önce
özel bir `derive` makrosu ile başlayıp, ardından diğer biçimleri farklı kılan
küçük farklılıkları açıklayacağız.

### Özel bir `derive` Makrosu Nasıl Yazılır

`HelloMacro` adlı bir özellik tanımlayan ve `hello_macro` adlı bir işlevle ilişkili olan
`hello_macro` adlı bir kutu oluşturalım. Kullanıcılarımızın her bir türü için `HelloMacro` özelliğini
uygulamalarını sağlamak yerine,
kullanıcıların türlerini `#[derive(HelloMacro)]` ile açıklama ekleyerek
`hello_macro` işlevinin varsayılan uygulamasını alabilmeleri için bir prosedürel makro sağlayacağız.
fonksiyonunun varsayılan uygulamasını alabilmeleri için prosedürel bir makro sağlayacağız. Varsayılan uygulama, `Hello, Macro! My name is
TypeName!` yazdırır; burada `TypeName`, bu özelliğin tanımlandığı türün adıdır.
Diğer bir deyişle, başka bir programcının bizim crate'imizi kullanarak Listing 20-37 gibi kod yazmasını sağlayan bir crate yazacağız.

<Listing number="20-37" file-name="src/main.rs" caption="The code a user of our crate will be able to write when using our procedural macro">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-37/src/main.rs}}
```

</Listing>

Bu kod, işimiz bittiğinde `Merhaba, Makro! Benim adım Pancakes!` yazısını yazdıracaktır.
İlk adım, şöyle bir yeni kütüphane kutusu oluşturmaktır:

```console
$ cargo new hello_macro --lib
```

Ardından, `HelloMacro` özelliğini ve bununla ilişkili işlevi tanımlayacağız:

<Listing file-name="src/lib.rs" number="20-38" caption="`derive` makrosu ile kullanacağımız basit bir trait">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-38/hello_macro/src/lib.rs}}
```

</Listing>

Bir özellik ve onun işlevi var. Bu noktada, crate kullanıcımız, Listing 20-39'da olduğu gibi, istenen işlevselliği elde etmek için özelliği uygulayabilir.

<Listing number="20-39" file-name="src/main.rs" caption="Kullanıcıların `HelloMacro` trait'ini elle uyguladığı hali">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-39/pancakes/src/main.rs}}
```

</Listing>

Ancak, `hello_macro` ile kullanmak istedikleri her tür için uygulama bloğunu yazmaları
gerekecektir; biz onları bu işten kurtarmak istiyoruz.
Bu işi.

Ayrıca, `hello_macro` işlevine, özelliğin uygulandığı türün adını yazdıracak
varsayılan uygulama sağlayamıyoruz: Rust yansıma yeteneklerine sahip olmadığı için,
çalışma zamanında türün adını arayamaz. Derleme zamanında kod üretmek için bir makroya ihtiyacımız var.
Bir sonraki adım, prosedürel makroyu tanımlamaktır.

Bir sonraki adım, prosedürel makroyu tanımlamaktır. Bu yazının yazıldığı tarihte,
prosedürel makrolar kendi kutusunda bulunmalıdır. Sonunda, bu kısıtlama
kaldırılabilir. Kutuları ve makro kutularını yapılandırma kuralı şu şekildedir:
`foo` adlı bir kutu için, özel `derive` prosedürel makro kutusu
`foo_derive` olarak adlandırılır. `hello_macro` projemizin içinde `hello_macro_derive` adlı yeni bir kutu
başlatalım:

```console
$ cargo new hello_macro_derive --lib
```

İki kutu birbiriyle yakından ilişkili olduğundan, prosedürel makro kutusunu
`hello_macro` kutusunun dizininde oluşturuyoruz. `hello_macro` içindeki özellik
tanımını değiştirirsek, `hello_macro_derive` içindeki prosedürel makronun
uygulamasını da değiştirmemiz gerekecektir. İki kutu ayrı ayrı yayınlanmalı ve
bu kutuları kullanan programcılar her ikisini de bağımlılık olarak eklemeli ve
her ikisini de kapsam içine almalıdır. Bunun yerine,
`hello_macro` crate'inin `hello_macro_derive`'i bağımlılık olarak kullanmasını ve
prosedürel makro kodunu yeniden dışa aktarmasını sağlayabiliriz. Ancak, projeyi yapılandırma şeklimiz,
programcıların `derive` işlevselliğini istemese bile `hello_macro`'yu kullanabilmelerini
sağlamaktadır.

`hello_macro_derive` kutusunu prosedürel makro kutusu olarak tanımlamamız gerekiyor.
Birazdan göreceğiniz gibi, `syn` ve `quote` kutularının işlevlerine de ihtiyacımız olacak,
bu yüzden bunları bağımlılıklar olarak eklememiz gerekiyor. `hello_macro_derive` için
_Cargo.toml_ dosyasına aşağıdakileri ekleyin:

<Listing file-name="hello_macro_derive/Cargo.toml">

```toml
{{#include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```

</Listing>

Prosedürel makroyu tanımlamaya başlamak için, Listing 20-40'taki kodu
`hello_macro_derive` crate'i için _src/lib.rs_ dosyanıza yerleştirin. Bu kod,
`impl_hello_macro` fonksiyonu için bir tanım ekleyene kadar derlenmeyecektir.

<Listing number="20-40" file-name="hello_macro_derive/src/lib.rs" caption="Rust kodunu işlemek için çoğu prosedürel makro kutusunda bulunması gereken kod">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/src/lib.rs}}
```

</Listing>

Kodun, `TokenStream`'i ayrıştırmaktan sorumlu olan `hello_macro_derive`
fonksiyonu ve sözdizimi ağacını dönüştürmekten sorumlu olan `impl_hello_macro`
fonksiyonu olarak ikiye ayrıldığını unutmayın: bu, prosedürel makro yazmayı daha
kolay hale getirir. Dış işlevdeki kod
(bu durumda `hello_macro_derive`) gördüğünüz veya oluşturduğunuz hemen hemen her
prosedürel makro kutusunda aynı olacaktır. İç işlevin gövdesinde
belirttiğiniz kod (bu durumda `impl_hello_macro`) prosedürel makronuzun amacına
bağlı olarak farklı olacaktır.

Üç yeni kutu tanıttık: `proc_macro`, [`syn`] ve [`quote`].
`proc_macro` kutusu Rust ile birlikte gelir, bu yüzden onu _Cargo.toml_'deki
bağımlılıklara eklememize gerek kalmadı. `proc_macro` kutusu, kodumuzdan Rust kodunu okumamızı ve
işlememizi sağlayan derleyicinin API'sıdır.

`syn` kutusunu kullanarak bir dizeden Rust kodunu, üzerinde işlem yapabileceğimiz bir veri yapısına
ayrıştırabiliriz. `quote` kutusu ise `syn` veri yapılarını tekrar
Rust koduna dönüştürür. Bu kutular, işlemek isteyebileceğimiz her türlü Rust
kodunu ayrıştırmayı çok daha kolay hale getirir: Rust kodu için tam bir ayrıştırıcı yazmak kolay
bir iş değildir.

Kütüphanemizin bir kullanıcısı bir tür üzerinde `#[derive(HelloMacro)]` belirttiğinde
`hello_macro_derive` işlevi çağrılacaktır. Bu, burada `hello_macro_derive` işlevini `proc_macro_derive` ile
açıklamış ve özellik adımızla eşleşen `HelloMacro` adını belirtmiş olmamız sayesinde
mümkün olmaktadır; bu, çoğu prosedürel makronun izlediği
geleneksel bir uygulamadır.

`hello_macro_derive` işlevi önce `input`'u bir
`TokenStream`'den, yorumlayıp işlem yapabileceğimiz bir veri yapısına dönüştürür.
Burada `syn` devreye girer. `syn` içindeki `parse` işlevi, bir `TokenStream` alır ve
parslenmiş Rust kodunu temsil eden bir `DeriveInput` yapısı döndürür.
Listing 20-41, `struct Pancakes;` dizesini parseleyerek elde ettiğimiz `DeriveInput`
yapısının ilgili kısımlarını göstermektedir.

<Listing number="20-41" caption="Makronun özniteliğine sahip kodu ayrıştırdığımızda elde ettiğimiz `DeriveInput` örneği">

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

</Listing>

Bu yapının alanları, ayrıştırdığımız Rust kodunun `Pancakes` adlı birim yapısı
olduğunu gösterir. Bu yapıda, her türlü Rust kodunu tanımlamak için daha fazla
alan vardır; daha fazla bilgi için [`syn``DeriveInput` belgelerine][syn-docs] bakın.

Yakında, dahil etmek istediğimiz yeni Rust kodunu oluşturacağımız `impl_hello_macro` işlevini tanımlayacağız.
Ancak bunu yapmadan önce, `derive` makromuzun çıktısının da bir `TokenStream` olduğunu unutmayın.
Döndürülen `TokenStream`, Döndürülen `TokenStream`,
krate kullanıcılarımızın yazdığı koda eklenir, böylece crate'lerini derlediklerinde, değiştirilmiş `TokenStream`'de sağladığımız ek işlevselliği elde ederler.

`syn::parse` işlevinin çağrısı burada başarısız olursa, `hello_macro_derive` işlevinin paniğe kapılması için `unwrap` işlevini çağırdığımızı fark etmiş olabilirsiniz. Prosedürel makromuzun hatalarda paniğe kapılması gerekir, çünkü `proc_macro_derive` işlevleri, prosedürel makro API'sına uymak için `Result` yerine `TokenStream` döndürmelidir. Bu örneği `unwrap` kullanarak basitleştirdik; üretim kodunda, `panic!` veya `expect` kullanarak neyin yanlış gittiğine dair daha spesifik hata mesajları sağlamalısınız.

Artık, `TokenStream`'den `DeriveInput` örneğine dönüştürmek için gerekli olan Rust koduna sahip olduğumuza göre, Listing 20-42'de gösterildiği gibi, `HelloMacro` özelliğini anotlanmış tür üzerinde uygulayan kodu oluşturalım.

<Listing number="20-42" file-name="hello_macro_derive/src/lib.rs" caption="Ayrıştırılmış Rust kodunu kullanarak `HelloMacro` trait'ini uygulamak">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-42/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

</Listing>

`ast.ident` kullanarak, anotlanmış türün adını (tanımlayıcı) içeren bir `Ident` yapısı örneği elde ederiz. Listing 20-33'teki yapı, Listing 20-31'deki kod üzerinde `impl_hello_macro` işlevini çalıştırdığımızda, elde ettiğimiz `ident`'in `ident` alanı `“Pancakes”` değerine sahip olacağını gösterir. Böylece, `ident` alanının değeri `“Pancakes”` olur. Böylece, Listing 20-34'teki `name` değişkeni, yazdırıldığında Listing 20-37'deki yapının adı olan `“Pancakes”` dizesi olacak bir `Ident` yapısı örneği içerecektir.

`quote!` makrosu, döndürmek istediğimiz Rust kodunu tanımlamamızı sağlar. Derleyici, `quote!` makrosunun yürütülmesinin doğrudan sonucundan farklı bir şey bekler, bu yüzden onu bir `TokenStream`'e dönüştürmemiz gerekir. Bunu, bu ara temsilini tüketen ve gerekli `TokenStream` türünde bir değer döndüren `into` yöntemini çağırarak yaparız.

`quote!` makrosu ayrıca bazı çok kullanışlı şablon mekanizmaları da sağlar: `#name` girebiliriz ve `quote!` bunu `name` değişkenindeki değerle değiştirir. Normal makroların çalışma şekline benzer şekilde bazı tekrarlamalar da yapabilirsiniz. Kapsamlı bir giriş için [`quote` crate belgelerine][quote-docs] bakın.

Prosedürel makromuzun, kullanıcının `#name` kullanarak elde edebileceğimiz, anotasyon yaptığı tür için `HelloMacro` özelliğinin bir uygulamasını oluşturmasını istiyoruz. Özellik uygulaması, sağlamak istediğimiz işlevselliği içeren `hello_macro` adlı tek bir işleve sahiptir: `Hello, Macro! My name is` ve ardından anotasyon yapılan türün adını yazdırmak.

Burada kullanılan `stringify!` makrosu Rust'a yerleşiktir. Rust ifadesi, örneğin `1 + 2`, derleme sırasında ifadeyi `“1 + 2”` gibi bir string literal'a dönüştürür. Bu, ifadeyi değerlendirip sonucu `String`'e dönüştüren `format!` veya `println!` makrolarından farklıdır. `#name` girdisinin, harfiyen yazdırılacak bir ifade olma ihtimali vardır, bu yüzden `stringify!` kullanıyoruz. `stringify!` kullanmak, derleme sırasında `#name`'i bir string literal'a dönüştürerek bir tahsisat tasarrufu da sağlar.

Bu noktada, `cargo build` hem `hello_macro` hem de `hello_macro_derive`'de başarıyla tamamlanmalıdır. Bu kutuları Listing 20-31'deki koda bağlayarak prosedürel makronun nasıl çalıştığını görelim! _projects_ dizininde `cargo new pancakes` kullanarak yeni bir ikili proje oluşturun. `pancakes` kutusunun _Cargo.toml_ dosyasına `hello_macro` ve `hello_macro_derive` bağımlılıklarını eklememiz gerekiyor. `hello_macro` ve `hello_macro_derive` sürümlerinizi [crates.io](https://crates.io/) adresinde yayınlıyorsanız, bunlar normal bağımlılıklar olacaktır; yayınlamıyorsanız, bunları aşağıdaki gibi `path` bağımlılıkları olarak belirtebilirsiniz:

```toml
{{#include ../listings/ch20-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:7:9}}
```

Listing 20-37'deki kodu _src/main.rs_ dosyasına ekleyin ve `cargo run` komutunu çalıştırın: `Hello, Macro! My name is Pancakes!` yazısı görüntülenmelidir. Prosedürel makrodan `HelloMacro` özelliğinin uygulaması, `pancakes` kutusunun bunu uygulamasına gerek kalmadan dahil edildi; `#[derive(HelloMacro)]` özelliği, özelliğin uygulamasını ekledi.

Şimdi, diğer prosedürel makro türlerinin özel `derive` makrolarından nasıl farklı olduğunu inceleyelim.

### Öznitelik benzeri makrolar

Öznitelik benzeri makrolar, özel `derive` makrolarına benzer, ancak `derive` özniteliği için kod üretmek yerine, yeni öznitelikler oluşturmanıza izin verir. Ayrıca daha esnektirler: `derive` yalnızca yapılar ve enumlar için çalışır; öznitelikler ise işlevler gibi diğer öğelere de uygulanabilir. Özellik benzeri makro kullanmanın bir örneği. Bir web uygulama çerçevesi kullanırken işlevleri açıklama amaçlı `route` adlı bir özelliğiniz olduğunu varsayalım:

```rust,ignore
#[route(GET, "/")]
fn index() {
```

Bu `#[route]` özniteliği, çerçeve tarafından prosedürel bir makro olarak tanımlanacaktır. Makro tanımlama işlevinin imzası şöyle görünecektir:

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Burada, `TokenStream` türünde iki parametre var. İlki, özniteliğin içeriği için: `GET, “/”` kısmı. İkincisi ise özniteliğin eklendiği öğenin gövdesi: bu durumda, `fn index() {}` ve fonksiyonun geri kalan gövdesi. Bunun dışında, öznitelik benzeri makrolar, özel `derive` makrolarıyla aynı şekilde çalışır:

Bunun dışında, öznitelik benzeri makrolar özel `derive` makroları ile aynı şekilde çalışır: `proc-macro` kutu türü ile bir kutu oluşturur ve istediğiniz kodu üreten bir işlev uygularsınız!

### İşlev benzeri makrolar

İşlev benzeri makrolar, işlev çağrılarına benzeyen makroları tanımlar. `macro_rules!` makrolarına benzer şekilde, işlevlerden daha esnektirler; örneğin, bilinmeyen sayıda argüman alabilirler. Ancak, `macro_rules!` makroları yalnızca daha önce [“Genel Metaprogramlama için Bildirimsel Makrolar `macro_rules!` for General Metaprogramming”][decl]<!-- ignore --> bölümünde tartıştığımız eşleşme benzeri sözdizimi kullanılarak tanımlanabilir. İşlev benzeri makrolar bir `TokenStream` parametresi alır ve tanımları, diğer iki tür prosedürel makro gibi Rust kodu kullanarak bu `TokenStream`'i işler. İşlev benzeri bir makro örneği, şu şekilde çağrılabilen `sql!` makrosudur:

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

Bu makro, içindeki SQL ifadesini ayrıştırır ve sözdizimsel olarak doğru olup olmadığını kontrol eder, bu da `macro_rules!` makrosunun yapabileceğinden çok daha karmaşık bir işlemdir. `sql!` makrosu şu şekilde tanımlanır:

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

Bu tanım, özel `derive` makrosunun imzasına benzer: parantez içindeki tokenleri alır ve oluşturmak istediğimiz kodu döndürürüz.
## Özet

Vay canına! Artık araç kutunuzda, muhtemelen sık kullanmayacağınız bazı Rust özellikleri var, ancak bunların çok özel durumlarda kullanılabileceğini bileceksiniz. Birkaç karmaşık konuyu tanıttık, böylece bunları hata mesajı önerilerinde veya başkalarının kodlarında gördüğünüzde, bu kavramları ve sözdizimini tanıyabileceksiniz. Bu bölümü, çözüm bulmak için kılavuz olarak kullanın.

Şimdi, kitap boyunca tartıştığımız her şeyi uygulamaya koyup bir proje daha yapacağız!

[ref]: ../reference/macros-by-example.md
[tlborm]: https://veykril.github.io/tlborm/
[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote
[syn-docs]: https://docs.rs/syn/2.0/syn/struct.DeriveInput.md
[quote-docs]: https://docs.rs/quote
[decl]: #declarative-macros-with-macro_rules-for-general-metaprogramming