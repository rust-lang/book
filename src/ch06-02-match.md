<!-- Old heading. Do not remove or links may break. -->

<a id="the-match-control-flow-operator"></a>

## `match` Kontrol Akışı Yapısı

Rust, `match` adı verilen son derece güçlü bir kontrol akışı yapısına sahiptir.
bir değeri bir dizi kalıpla karşılaştırmanıza ve ardından
kodunu hangi kalıpla eşleştiğine göre belirler. Kalıplar gerçek değerlerden oluşabilir,
değişken adları, joker karakterler ve diğer pek çok şey; [Bölüm 19][ch19-00-patterns]<!-- ignore --> tüm farklı kalıp türlerini kapsar
ve ne yaptıkları. Eşleştirme`nin gücü, eşleştirmenin ifade gücünden gelir.
kalıpları ve derleyicinin tüm olası durumların olduğunu doğrulaması gerçeği
ele alınmıştır.

Bir `match` ifadesini bozuk para ayıklama makinesi gibi düşünün: bozuk paralar kayar
üzerinde çeşitli büyüklüklerde delikler bulunan bir raydan aşağı iner ve her bozuk para
karşılaştığı ilk deliğe sığar. Aynı şekilde, değerler de
bir `match` içindeki her bir desen aracılığıyla ve ilk desende “fits,” değeri
değer, yürütme sırasında kullanılmak üzere ilişkili kod bloğuna düşer.

Madeni paralardan bahsetmişken, onları `match` kullanarak bir örnek olarak kullanalım! Yazabiliriz
fonksiyonu bilinmeyen bir ABD madeni parasını alır ve sayma işlemine benzer şekilde
makine, hangi madeni para olduğunu belirler ve gösterildiği gibi değerini sent olarak verir
Liste 6-3'te.


<Listing number="6-3" caption="An enum and a `match` expression that has the variants of the enum as its patterns">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

</Listing>

`value_in_cents` işlevindeki `match` ifadesini inceleyelim. Öncelikle,
`match` anahtar kelimesini ve ardından bir ifadeyi listeliyoruz. Bu durumda ifade,
`coin` değeridir. Bu, `if` ile kullanılan koşullu ifadeye çok benziyor, ancak
büyük bir fark var: `if` ile koşul, bir
Boolean değeri olarak değerlendirilmelidir, ancak burada herhangi bir tür olabilir. Bu örnekteki `coin` türü,
ilk satırda tanımladığımız `Coin` enum'dur.

Sırada `match` kolları var. Bir kol iki bölümden oluşur: bir desen ve bazı kodlar.
Buradaki ilk kol, `Coin::Penny` değeri olan bir desene ve ardından deseni ve çalıştırılacak kodu ayıran `=>`
işlemcisine sahiptir. Bu durumda kod
sadece `1` değeridir. Her kol, bir sonraki koldan virgülle ayrılır.

`match` ifadesi yürütüldüğünde, sonuç değeri sırayla her kolun
deseniyle karşılaştırılır. Bir desen değerle eşleşirse, o desenle ilişkili kod
yürütülür. Desen değerle eşleşmezse,
yürütme bir madeni para ayırma makinesinde olduğu gibi bir sonraki kola devam eder.
İhtiyacımız olduğu kadar çok kol olabilir: Listing 6-3'te, `match` ifademizin dört kolu vardır.

Her kol ile ilişkili kod bir ifadedir ve eşleşen koldaki ifadenin sonuç değeri,
tüm `match` ifadesi için döndürülen değerdir.
`match` ifadesinin sonuç değeri,

Eşleşme kolu kodu kısa ise genellikle küme parantezleri kullanmayız,
Listing 6-3'te olduğu gibi her kol sadece bir değer döndürür. Eşleşme kolunda birden fazla
kod satırı çalıştırmak istiyorsanız, küme parantezleri kullanmanız gerekir ve kolun
ardından gelen virgül isteğe bağlıdır. Örneğin, aşağıdaki kod
“Lucky penny!” yazdırır, ancak yine de bloğun son değeri olan `1`'i döndürür:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### Değerlere Bağlanan Kalıplar

Eşleştirme kolları, desene uyan değerlerin parçalarına bağlanabilmeleri açısından da
kullanışlıdır. Bu sayede, enum varyantlarından değerleri çıkarabiliriz.


Örnek olarak, enum varyantlarımızdan birini içinde veri tutacak şekilde değiştirelim.
1999'dan 2008'e kadar, Amerika Birleşik Devletleri 50 eyaletin her biri için farklı
tasarımlara sahip çeyrek dolarlar bastı. Başka hiçbir madeni para eyalet
tasarımlarına sahip değildi, bu yüzden sadece çeyrek dolarlar bu ekstra değere sahipti. Bu bilgiyi
`enum`'umuza, `Quarter` varyantını içinde depolanan bir `UsState` değeri
içerecek şekilde değiştirerek ekleyebiliriz, bunu Listing 6-4'te yaptık.

<Listing number="6-4" caption="A `Coin` enum in which the `Quarter` variant also holds a `UsState` value">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

</Listing>

Bir arkadaşımızın 50 eyalet çeyreğini toplamaya çalıştığını hayal edelim.
Bozuk paralarımızı para türüne göre sıralarken, her çeyreğin ait olduğu eyaletin
adını da söyleyeceğiz, böylece arkadaşımızın sahip olmadığı bir çeyrek varsa,
onu koleksiyonuna ekleyebilsin.

Bu kodun eşleştirme ifadesinde, `Coin::Quarter` varyantının değerleriyle eşleşen
desene `state` adlı bir değişken ekliyoruz. Bir
`Coin::Quarter` eşleştiğinde, `state` değişkeni o çeyreklik madalyonun ait olduğu
eyaletin değerine bağlanacaktır. Ardından, o kol için kodda `state` değişkenini şu şekilde kullanabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

`value_in_cents(Coin::Quarter(UsState::Alaska))` çağrısı yaparsak, `coin`
`Coin::Quarter(UsState::Alaska)` olur. Bu değeri her bir eşleşme koluyla karşılaştırdığımızda,
`Coin::Quarter(state)`'ye ulaşana kadar hiçbiri eşleşmez. Bu
noktada, `state` için bağlama değeri `UsState::Alaska` olacaktır. Daha sonra
bu bağlamayı `println!` ifadesinde kullanabiliriz, böylece `Quarter` için `Coin` enum varyantından iç
durum değerini elde ederiz.

### `Option<T>` ile eşleştirme

Önceki bölümde, `Option<T>` kullanırken `Some`
durumundan iç `T` değerini almak istedik; `Coin` enumunda yaptığımız gibi, `match` kullanarak `Option<T>` ile de çalışabiliriz!
Madeni paraları karşılaştırmak yerine,
`Option<T>` varyantlarını karşılaştıracağız, ancak `match` ifadesinin çalışma şekli aynı
kalacaktır.

Diyelim ki, bir `Option<i32>` alan ve içinde bir değer varsa
bu değere 1 ekleyen bir işlev yazmak istiyoruz. İçinde bir değer yoksa,
işlev `None` değerini döndürmeli ve herhangi bir
işlem yapmaya çalışmamalıdır.

Bu işlev, `match` sayesinde yazması çok kolaydır ve
Listing 6-5 gibi görünecektir.

<Listing number="6-5" caption="A function that uses a `match` expression on an `Option<i32>`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

</Listing>

`plus_one` işlevinin ilk yürütülmesini daha ayrıntılı olarak inceleyelim.
`plus_one(five)` işlevini çağırdığımızda, `plus_one` işlevinin gövdesindeki `x` değişkeni
`Some(5)` değerine sahip olacaktır. Ardından bunu her bir eşleşme koluyla karşılaştırırız:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

`Some(5)` değeri `None` deseniyle eşleşmediğinden, bir sonraki kola geçiyoruz:
`None`

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)`, `Some(i)` ile eşleşir mi? Eşleşir! Aynı varyantımız var. `i`,
`Some` içindeki değere bağlanır, bu nedenle `i`, `5` değerini alır.
Eşleşme kolundaki kod daha sonra yürütülür, bu nedenle `i` değerine 1 ekleriz ve toplam `6` değerini içeren yeni bir
`Some` değeri oluştururuz.

Şimdi, Listing 6-5'teki `plus_one`'ın ikinci çağrısını ele alalım, burada `x`
`None`'dur. `match`'e gireriz ve ilk kol ile karşılaştırırız:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Eşleşiyor! Eklenecek bir değer olmadığı için program durur ve
`=>` sağ tarafındaki `None` değerini döndürür. İlk kol eşleştiği için diğer
kollar karşılaştırılmaz.

`match` ve enum'ları birleştirmek birçok durumda kullanışlıdır. Rust kodunda bu
deseni sıkça göreceksiniz: bir enum'a karşı `match`, değişkeni içindeki
verilere bağlayın ve ardından buna göre kodu çalıştırın. Başlangıçta biraz zor olabilir, ancak
alıştığınızda, tüm dillerde olmasını isteyeceksiniz. Bu,
kullanıcıların sürekli olarak en sevdiği özelliktir.

### Eşleşmeler Kapsamlıdır

`match` ile ilgili tartışmamız gereken bir başka husus daha var: dalların kalıpları
tüm olasılıkları kapsamalıdır. Hatalı olduğu için derlenemeyen `plus_one` işlevimizin bu
sürümünü ele alalım:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

`None` durumunu ele almadık, bu nedenle bu kod bir hata oluşturacaktır. Neyse ki, bu
Rust'un yakalayabileceği bir hatadır. Bu kodu derlemeye çalışırsak, şu
hatayı alırız:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust, her olası durumu kapsamadığımızı bilir ve hatta hangi
deseni unuttuğumuzu bile bilir! Rust'ta eşleşmeler _kapsamlıdır_: kodun geçerli olması için her türlü
olasılığı tüketmeliyiz. Özellikle
`Option<T>` durumunda, Rust, `None` durumunu açıkça ele almayı unutmamızı engelleyerek
`None` durumunu açıkça ele almamızı engellediğinde, null olabilecek bir durumda bir değerimiz olduğunu varsaymamızı önler ve böylece
daha önce bahsedilen milyar dolarlık hatayı imkansız hale getirir.

### Her şeyi kapsayan desenler ve `_` yer tutucusu

Enum'ları kullanarak, birkaç belirli değer için özel eylemler de gerçekleştirebiliriz, ancak
diğer tüm değerler için tek bir varsayılan eylem gerçekleştiririz. Bir oyun uyguladığımızı düşünün,
 zar attığınızda 3 gelirse, oyuncunuz hareket etmez, bunun yerine
yeni bir şık şapka alır. 7 gelirse, oyuncunuz şık şapkasını kaybeder. Diğer tüm
değerler için, oyuncunuz oyun tahtasında o sayı kadar ilerler. İşte
bu mantığı uygulayan bir `match`, zar atma sonucu
rastgele bir değer yerine sabit kodlanmış ve diğer tüm mantık,
gerçekleştirilmesi bu örneğin kapsamı dışında olduğu için gövdesi olmayan
fonksiyonlarla temsil edilmiştir:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

İlk iki kol için, desenler `3` ve `7` gibi gerçek değerlerdir.
Diğer tüm olası değerleri kapsayan son kol için, desen
`other` adını verdiğimiz değişkendir. `other` kolu için çalışan kod,
değişkeni `move_player` işlevine aktararak kullanır.

Bu kod derlenir, ancak `u8`'in sahip olabileceği tüm olası değerleri listelememiş olsak da
`u8`'in sahip olabileceği tüm olası değerleri listelememiş olsak da derlenir, çünkü son desen özellikle listelenmemiş tüm değerlerle eşleşir.
Bu her şeyi kapsayan desen, `match`'in kapsamlı olması gerekliliğini karşılar.
Desenler sırayla değerlendirildiğinden, her şeyi kapsayan kolu en sona koymamız gerektiğini unutmayın.
Her şeyi kapsayan kolu daha öne koyarsak, diğer
kollar asla çalışmaz, bu nedenle her şeyi kapsayan kolun arkasına kol eklediğimizde Rust bizi uyarır!

Rust'ta, her şeyi kapsayan bir desen istediğimizde ancak her şeyi kapsayan desendeki değeri
_kullanmak_ istemediğimizde kullanabileceğimiz bir desen de vardır: `_`, herhangi bir değeri eşleştiren ve o değere bağlanmayan özel bir desendir.
Bu, Rust'a değeri kullanmayacağımızı söyler, böylece Rust kullanılmayan bir değişken hakkında bizi uyarmaz.
Oyunun kurallarını değiştirelim: artık 3 veya 7 dışında herhangi bir şey atarsanız,

Oyunun kurallarını değiştirelim: artık 3 veya
7 dışında bir sayı attığınızda, tekrar atmanız gerekir. Artık catch-all değerini kullanmamıza gerek yok, bu yüzden
`other` adlı değişken yerine `_` kullanacak şekilde kodumuzu değiştirebiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Bu örnek de kapsamlılık gerekliliğini karşılamaktadır, çünkü son kolun diğer tüm değerlerini açıkça
görmezden geliyoruz; hiçbir şeyi unutmadık.

Son olarak, oyunun kurallarını bir kez daha değiştireceğiz, böylece 3 veya 7 dışında bir sayı attığınızda
sıranızda başka hiçbir şey olmayacak. Bunu,
`_` koluna ait kod olarak birim değeri ([“Tuple Türü”][tuples]<!-- ignore --> bölümünde bahsettiğimiz boş tuple türü) kullanarak ifade edebiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Burada, Rust'a açıkça, önceki kolun deseniyle eşleşmeyen başka hiçbir değeri kullanmayacağımızı
ve bu durumda hiçbir
kodu çalıştırmak istemediğimizi söylüyoruz.

Desenler ve eşleştirme hakkında daha fazla bilgiyi [Bölüm 19][ch19-00-patterns]<!-- ignore -->. Şimdilik, `match` ifadesinin
biraz uzun olduğu durumlarda yararlı olabilecek
`if let` sözdizimine geçeceğiz.

[tuples]: ch03-02-data-types.md#the-tuple-type
[ch19-00-patterns]: ch19-00-patterns.md
