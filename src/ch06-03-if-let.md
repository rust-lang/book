## `if let` ve `let else` ile Kısa Kontrol Akışı

`if let` sözdizimi, `if` ve `let` ifadeleri daha az ayrıntılı bir şekilde birleştirerek,
bir desene uyan değerleri işlerken geri kalanını yok saymanızı sağlar. Listing 6-6'daki
programı ele alalım. Bu program,
`config_max` değişkenindeki `Option<u8>` değerini eşleştirir, ancak değer `Some`
varyantıysa kodu yürütmek ister.

<Listing number="6-6" caption="A `match` that only cares about executing code when the value is `Some`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

</Listing>

Değer `Some` ise, değeri `Some` varyantında yazdırırız. Bunu, değeri kalıp içindeki `max` değişkenine bağlayarak yaparız.
`None` değeriyle hiçbir şey yapmak istemeyiz.
`match` ifadesini karşılamak için, tek bir varyantı işledikten sonra `_ =>
()` eklememiz gerekir. Bu, eklemek için can sıkıcı bir kalıp koddur.
Bunun yerine, bunu `if let` kullanarak daha kısa bir şekilde yazabiliriz.

Bunun yerine, `if let` kullanarak bunu daha kısa bir şekilde yazabiliriz. Aşağıdaki
kod, Listing 6-6'daki `match` ile aynı şekilde davranır:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

`if let` sözdizimi, eşitlik işareti ile ayrılmış bir desen ve bir ifade alır.
Bu, `match` ile aynı şekilde çalışır; burada ifade `match`'e verilir ve desen
`match`'in ilk koludur. Bu durumda desen
`Some(max)` şeklindedir ve `max`, `Some` içindeki değere bağlanır. Daha sonra,
`if let` bloğunun gövdesinde `max`'ı, karşılık gelen `match` kolunda kullandığımız
şekilde kullanabiliriz. `if let` bloğundaki kod, değer desenle eşleştiğinde
çalışır.

`if let` kullanmak, daha az yazma, daha az girinti ve daha az kalıp kod anlamına gelir.
Ancak, `match`'in uyguladığı ve hiçbir durumu atlamadığınızdan emin olmanızı sağlayan kapsamlı kontrolü kaybedersiniz.
`match` ve `if let` arasında seçim yapmak,
belirli bir durumda ne yaptığınıza ve kapsamlı kontrolü kaybetmenin karşılığında
özlü olmanın uygun bir ödün olup olmadığına bağlıdır.

Diğer bir deyişle, `if let`'i, değer bir kalıpla eşleştiğinde kodu çalıştıran ve diğer tüm değerleri yok sayan bir `match` için
sözdizimi şekerlemesi olarak düşünebilirsiniz.

`if let` ile bir `else` ekleyebiliriz.
`else` ile birlikte gelen kod bloğu, `if let` ve `else` ile eşdeğer olan
`match` ifadesindeki `_` durumuyla birlikte gelen kod bloğu ile aynıdır. Listing 6-4'teki
`Coin` enum tanımını hatırlayın, burada `Quarter` varyantı da bir
`UsState` değeri içeriyordu. Gördüğümüz tüm çeyrek olmayan paraları saymak ve aynı zamanda
çeyreklerin durumunu da bildirmek istersek, bunu bir `match`
ifadesi ile şu şekilde yapabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

Ya da şöyle bir `if let` ve `else` ifadesi kullanabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

## `let...else` ile “Mutlu Yol”da kalmak

Yaygın olarak kullanılan yöntem, bir değer mevcut olduğunda bazı hesaplamalar yapmak ve
aksi takdirde varsayılan bir değer döndürmektir. `UsState` değerine sahip madeni paralar örneğimize devam edersek,
çeyrek madeni paranın üzerindeki durumun yaşına bağlı olarak komik bir şey söylemek istersek,
`UsState` üzerinde durumun yaşını kontrol etmek için bir yöntem ekleyebiliriz,
şöyle:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

Ardından, madeni paranın türüne göre eşleştirme yapmak için `if let` kullanabiliriz ve Listing 6-7'de olduğu gibi koşul gövdesine bir `state`
değişkeni ekleyebiliriz.

<Listing number="6-7" caption="Checking whether a state existed in 1900 by using conditionals nested inside an `if let`.">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:describe}}
```

</Listing>

Bu işin görülmesini sağlar, ancak işi `if
let` ifadesinin gövdesine itmiştir ve yapılacak iş daha karmaşıksa, üst düzey dalların birbirleriyle nasıl ilişkili olduğunu tam olarak takip etmek zor olabilir.
Ayrıca, ifadelerin bir değer üretmesi gerçeğinden yararlanarak, Listing 6-8'de olduğu gibi, `if let` ifadesinden `state`'i üretmek veya erken dönmek için de yararlanabiliriz. (Benzer şekilde, Ayrıca, ifadelerin bir değer ürettiği gerçeğinden yararlanarak,
Listing 6-8'de olduğu gibi, `if let` ifadesinden
`state` değerini üretmek veya erken dönmek için de kullanabiliriz. (Aynı şeyi `match` ile de yapabilirsiniz.)

<Listing number="6-8" caption="Using `if let` to produce a value or return early.">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

</Listing>

Bu, kendi başına biraz can sıkıcı bir durumdur! `if
let` ifadesinin bir dalı bir değer üretir, diğer dalı ise işlevden tamamen çıkar.

Bu yaygın kalıbı daha güzel ifade etmek için Rust'ta `let...else` vardır.
`let...else` sözdizimi sol tarafta bir kalıp, sağ tarafta bir ifade alır
ve `if let` ile çok benzerdir, ancak `if` dalı yoktur, sadece
`else` dalı vardır. Desen eşleşirse, dış kapsamdaki desenden değeri bağlar.
Desen eşleşmezse, program
fonksiyondan dönmesi gereken `else` dalına akar.

Listing 6-9'da, `if let` yerine `let...else` kullanıldığında Listing 6-8'in nasıl göründüğünü görebilirsiniz.
Listing 6-9

<Listing number="6-9" caption="Using `let...else` to clarify the flow through the function.">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

</Listing>

Bu şekilde, fonksiyonun ana gövdesinde “mutlu yol”da kaldığını ve
`if let`'in yaptığı gibi iki dal için önemli ölçüde farklı kontrol akışı
olmadığını unutmayın.

Programınızda `match` kullanarak ifade etmek için çok ayrıntılı bir mantık
varsa, `if let` ve `let...else`'nin de Rust araç kutunuzda olduğunu unutmayın.


## Özet

Şimdi, enumları kullanarak bir dizi numaralandırılmış değerden biri olabilen özel türler
oluşturmayı ele aldık. Standart kütüphanenin `Option<T>` türünün, hata önlemek için tür
sistemini kullanmanıza nasıl yardımcı olduğunu gösterdik. Enum değerlerinin içinde
veri varsa, işlemek istediğiniz durum sayısına bağlı olarak `match` veya `if let` kullanarak
bu değerleri çıkarabilir ve kullanabilirsiniz.

Rust programlarınız artık yapıları ve enumları kullanarak alanınızdaki kavramları ifade edebilir.
API'nizde kullanmak üzere özel türler oluşturmak, tür güvenliğini sağlar:
derleyici, işlevlerinizin yalnızca her işlevin beklediği türdeki değerleri almasını sağlar.


Kullanıcılarınıza kullanımı kolay ve yalnızca ihtiyaç duydukları şeyleri ortaya çıkaran iyi organize edilmiş bir API sağlamak için, şimdi Rust'un modüllerine geçelim.
