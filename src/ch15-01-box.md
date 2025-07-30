## `Box<T>` Kullanarak Veriye Heap Üzerinden İşaret Etmek

En basit akıllı işaretçi, türü `Box<T>` olarak yazılan kutudur (box). _Box_ yapısı, veriyi yığın (stack) yerine heap üzerinde saklamanızı sağlar. Yığında kalan ise, heap'teki veriye işaret eden işaretçidir. Yığın ve heap arasındaki farkı gözden geçirmek için 4. Bölüme bakabilirsiniz.

Box'ların, verilerini yığın yerine heap'te saklamaları dışında herhangi bir performans maliyeti yoktur. Ancak, çok fazla ek yetenekleri de yoktur. Genellikle şu durumlarda kullanılırlar:

- Derleme zamanında boyutu bilinemeyen bir türünüz olduğunda ve bu türdeki bir değeri kesin boyut gerektiren bir bağlamda kullanmak istediğinizde
- Büyük miktarda veriniz olduğunda ve sahipliği devretmek, ancak verinin kopyalanmadığından emin olmak istediğinizde
- Bir değerin sahipliğini almak ve yalnızca belirli bir trait'i uygulamasına önem vermek, belirli bir türe sahip olmasına değil

İlk durumu ["Box ile Özyinelemeli Türleri Etkinleştirmek"](#box-ile-özyinelemeli-türleri-etkinleştirmek) bölümünde göstereceğiz. İkinci durumda, büyük miktarda verinin sahipliğini devretmek uzun sürebilir çünkü veri yığın üzerinde kopyalanır. Bu durumda performansı artırmak için, büyük veriyi bir kutuda heap'te saklayabiliriz. Böylece, yığında yalnızca küçük bir işaretçi kopyalanır, asıl veri ise heap'te bir yerde kalır. Üçüncü durum ise _trait nesnesi_ olarak bilinir ve 18. Bölümdeki ["Farklı Türde Değerler İçin Trait Nesneleri Kullanmak"](ch18-02-trait-objects.md#ortak-davranışları-soyutlamak-için-trait-nesneleri-kullanmak) bölümünde ayrıntılı olarak ele alınacaktır. Burada öğrendiklerinizi orada tekrar kullanacaksınız!

### `Box<T>` ile Veriyi Heap'te Saklamak

`Box<T>`'nin heap'te veri saklama kullanımını tartışmadan önce, sözdizimini ve bir kutuda saklanan değerlerle nasıl etkileşime geçileceğini ele alalım.

Liste 15-1, bir kutu kullanarak bir `i32` değerinin heap'te nasıl saklanacağını gösteriyor.

<Listing number="15-1" file-name="src/main.rs" caption="Bir kutu kullanarak bir `i32` değerini heap'te saklamak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

</Listing>

`b` değişkenini, heap'te ayrılmış olan `5` değerine işaret eden bir `Box` olarak tanımlıyoruz. Bu program `b = 5` yazdıracaktır; bu durumda, kutudaki veriye, sanki bu veri yığında saklanıyormuş gibi erişebiliriz. Sahip olunan herhangi bir değer gibi, bir kutu kapsamdan çıktığında (örneğin `b`'nin `main` sonunda olduğu gibi) bellekten silinir. Hem kutu (yığında saklanan) hem de işaret ettiği veri (heap'te saklanan) serbest bırakılır.

Tek bir değeri heap'te saklamak çok kullanışlı değildir, bu yüzden kutuları bu şekilde tek başına sık kullanmazsınız. Çoğu durumda, tek bir `i32` gibi değerlerin yığında saklanması daha uygundur. Şimdi, kutuların, kutu olmadan tanımlayamayacağımız türleri tanımlamamıza nasıl olanak sağladığı bir duruma bakalım.

### Box ile Özyinelemeli Türleri Etkinleştirmek

_Bir özyinelemeli türün_ bir değeri, kendisinin başka bir değerini de içerebilir. Özyinelemeli türler bir sorun oluşturur çünkü Rust, bir türün ne kadar yer kaplayacağını derleme zamanında bilmek ister. Ancak, özyinelemeli türlerdeki değerlerin iç içe geçmesi teorik olarak sonsuza kadar devam edebilir, bu yüzden Rust değerin ne kadar alana ihtiyacı olduğunu bilemez. Kutuların boyutu bilindiğinden, özyinelemeli tür tanımında bir kutu kullanarak özyinelemeli türleri etkinleştirebiliriz.

Özyinelemeli bir tür örneği olarak, _cons listesi_ni inceleyelim. Bu, fonksiyonel programlama dillerinde yaygın olarak bulunan bir veri türüdür. Tanımlayacağımız cons listesi türü, özyineleme dışında oldukça basittir; bu nedenle, örnekte işleyeceğimiz kavramlar, özyinelemeli türlerle ilgili daha karmaşık durumlarda da faydalı olacaktır.

#### Cons Listesi Hakkında Daha Fazla Bilgi

_Cons listesi_, Lisp programlama dili ve türevlerinden gelen, iç içe geçmiş çiftlerden oluşan ve bağlı listenin (linked list) Lisp versiyonu olan bir veri yapısıdır. Adını, iki argümandan yeni bir çift oluşturan Lisp'teki `cons` fonksiyonundan (construct function) alır. Bir değer ve başka bir çift içeren bir çift üzerinde `cons` çağırarak, özyinelemeli çiftlerden oluşan cons listeleri oluşturabiliriz.

Örneğin, `1, 2, 3` listesini içeren bir cons listesinin sözde kod gösterimi şöyle olur:

```text
(1, (2, (3, Nil)))
```

Cons listesindeki her öğe iki eleman içerir: mevcut öğenin değeri ve bir sonraki öğe. Listedeki son öğe ise yalnızca `Nil` adlı bir değer içerir ve sonraki bir öğe yoktur. Cons listesi, özyinelemeli olarak `cons` fonksiyonu çağrılarak üretilir. Özyinelemenin taban durumu için kullanılan kanonik ad `Nil`'dir. Bunun, 6. Bölümde tartışılan "null" veya "nil" kavramıyla aynı olmadığını unutmayın; burada "nil" geçersiz veya olmayan bir değeri ifade etmez.

Cons listesi Rust'ta yaygın olarak kullanılan bir veri yapısı değildir. Rust'ta bir öğe listesine ihtiyacınız olduğunda çoğu zaman `Vec<T>` daha iyi bir seçimdir. Diğer, daha karmaşık özyinelemeli veri türleri ise çeşitli durumlarda faydalıdır, ancak bu bölümde cons listesiyle başlayarak, kutuların özyinelemeli bir veri türünü nasıl tanımlamamıza olanak sağladığını görebiliriz.

Liste 15-2, bir cons listesi için bir enum tanımını içeriyor. Bu kod henüz derlenmeyecek çünkü `List` türünün boyutu bilinmiyor; bunu göstereceğiz.

<Listing number="15-2" file-name="src/main.rs" caption="`i32` değerlerinden oluşan bir cons listesi veri yapısını temsil eden enum tanımının ilk denemesi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

</Listing>

> Not: Bu örnek için yalnızca `i32` değerleri tutan bir cons listesi uyguluyoruz. 10. Bölümde tartıştığımız gibi, generics kullanarak herhangi bir türde değer tutabilen bir cons listesi de tanımlayabilirdik.

`List` türünü kullanarak `1, 2, 3` listesini saklamak, 15-3 numaralı listedeki kod gibi olurdu.

<Listing number="15-3" file-name="src/main.rs" caption="`List` enum'unu kullanarak `1, 2, 3` listesini saklamak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

</Listing>

İlk `Cons` değeri `1` ve başka bir `List` değeri tutar. Bu `List` değeri, `2` ve başka bir `List` değeri tutan başka bir `Cons` değeridir. Bu `List` değeri, `3` ve bir `List` değeri (sonunda `Nil`) tutan bir başka `Cons` değeridir. `Nil`, listenin sonunu belirten özyinelemeli olmayan varyanttır.

15-3 numaralı listedeki kodu derlemeye çalışırsak, 15-4 numaralı listede gösterilen hatayı alırız.

<Listing number="15-4" caption="Özyinelemeli bir enum tanımlamaya çalışırken aldığımız hata">

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

</Listing>

Hata, bu türün "sonsuz boyuta sahip" olduğunu gösteriyor. Bunun nedeni, `List`'i özyinelemeli bir varyantla tanımlamış olmamız: doğrudan kendisinden başka bir değer tutuyor. Sonuç olarak, Rust bir `List` değerini saklamak için ne kadar alana ihtiyacı olduğunu belirleyemiyor. Bu hatayı neden aldığımızı inceleyelim. Önce, Rust'ın özyinelemeli olmayan bir türün ne kadar yer kaplayacağını nasıl belirlediğine bakalım.

#### Özyinelemeli Olmayan Bir Türün Boyutunu Hesaplamak

6-2 numaralı listede, 6. Bölümde enum tanımlarını tartışırken tanımladığımız `Message` enum'unu hatırlayın:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

Bir `Message` değeri için ne kadar alan ayrılacağını belirlemek için, Rust her varyantı inceler ve en çok alana ihtiyaç duyan varyantı bulur. Sadece bir varyant kullanılacağı için, bir `Message` değerinin ihtiyaç duyacağı en fazla alan, varyantlardan en büyüğünün kaplayacağı alandır.

Bunu, Rust'ın 15-2 numaralı listedeki `List` enum'u gibi özyinelemeli bir türün ne kadar alana ihtiyacı olduğunu belirlemeye çalışırken ne olduğuyla karşılaştırın. Derleyici, `Cons` varyantına bakarak başlar; bu, bir `i32` ve bir `List` değeri tutar. Yani, `Cons`'un ihtiyacı olan alan, bir `i32`'nin boyutu artı bir `List`'in boyutudur. `List` türünün ne kadar belleğe ihtiyacı olduğunu bulmak için, derleyici varyantlara bakar ve yine `Cons` varyantından başlar. `Cons` bir `i32` ve bir `List` tutar ve bu süreç sonsuza kadar devam eder, bu da Şekil 15-1'de gösterilmiştir.

<img alt="Sonsuz bir Cons listesi: 'Cons' olarak etiketlenmiş bir dikdörtgen, iki daha küçük dikdörtgene bölünmüş. İlk daha küçük dikdörtgen 'i32' etiketini, ikinci daha küçük dikdörtgen ise 'Cons' etiketini ve dıştaki 'Cons' dikdörtgeninin daha küçük bir versiyonunu tutuyor. 'Cons' dikdörtgenleri, en küçük rahatça boyutlandırılmış dikdörtgen bir sonsuzluk sembolü tutana kadar kendilerinin daha küçük ve daha küçük versiyonlarını tutmaya devam ediyor, bu da bu tekrarın sonsuza kadar devam ettiğini gösteriyor." src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Şekil 15-1: Sonsuz `List`'in sonsuz `Cons` varyantlarından oluşması</span>

#### `Box<T>` Kullanarak Bilinen Boyuta Sahip Özyinelemeli Tür Elde Etmek

Rust, özyinelemeli tanımlanan türler için ne kadar alan ayıracağını bilemediğinden, derleyici şu yardımcı öneriyle birlikte bir hata verir:

```text
help: döngüyü kırmak için biraz dolaylılık ekleyin (ör. `Box`, `Rc` veya `&`)
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

Buradaki _dolaylılık_ (indirection), bir değeri doğrudan saklamak yerine, veri yapısını değeri dolaylı olarak, yani ona işaret eden bir işaretçiyle saklayacak şekilde değiştirmemiz gerektiği anlamına gelir.

Bir `Box<T>` bir işaretçi olduğundan, Rust her zaman bir `Box<T>`'nin ne kadar alana ihtiyacı olduğunu bilir: bir işaretçinin boyutu, işaret ettiği verinin miktarına göre değişmez. Bu, `Cons` varyantında doğrudan başka bir `List` değeri yerine bir `Box<T>` koyabileceğimiz anlamına gelir. `Box<T>`, bir sonraki `List` değerine işaret eder ve bu değer heap'te saklanır, `Cons` varyantının içinde değil. Kavramsal olarak hâlâ listelerden oluşan bir listeye sahibiz, ancak bu uygulama artık öğeleri birbirinin içine koymak yerine yan yana koymak gibidir.

15-2 ve 15-3 numaralı listelerdeki `List` enum'unun tanımını ve kullanımını, derlenecek şekilde 15-5 numaralı listedeki koda dönüştürebiliriz.

<Listing number="15-5" file-name="src/main.rs" caption="Bilinen boyuta sahip olmak için `Box<T>` kullanan `List` tanımı">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

</Listing>

`Cons` varyantı, bir `i32`'nin boyutuna ek olarak kutunun işaretçi verisini saklayacak kadar alana ihtiyaç duyar. `Nil` varyantı ise hiçbir değer saklamaz, bu yüzden yığında `Cons`'tan daha az alana ihtiyaç duyar. Artık herhangi bir `List` değerinin, bir `i32` boyutu artı bir kutunun işaretçi verisi kadar yer kaplayacağını biliyoruz. Bir kutu kullanarak sonsuz, özyinelemeli zinciri kırdık ve derleyici artık bir `List` değerini saklamak için ne kadar alana ihtiyacı olduğunu belirleyebiliyor. Şekil 15-2, `Cons` varyantının şimdi nasıl göründüğünü gösteriyor.

<img alt="Bir dikdörtgen 'Cons' etiketli iki daha küçük dikdörtgene bölünmüş. İlk daha küçük dikdörtgen 'i32' etiketini taşırken, ikinci daha küçük dikdörtgen 'Box' etiketini taşır ve içinde 'usize' etiketli bir dikdörtgen bulunur, bu da kutunun işaretçi verisinin sonlu boyutunu temsil eder." src="img/trpl15-02.svg" class="center" />

<span class="caption">Şekil 15-2: `Cons`'un bir `Box` tuttuğu için sonsuz boyutta olmaması</span>

Kutular yalnızca dolaylılık ve heap tahsisi sağlar; diğer akıllı işaretçi türlerinde göreceğimiz gibi özel yetenekleri yoktur. Ayrıca, bu özel yeteneklerin getirdiği performans maliyetine de sahip değillerdir; bu nedenle, yalnızca dolaylılığın gerektiği cons listesi gibi durumlarda faydalı olabilirler. Kutuların başka kullanım alanlarını 18. Bölümde göreceğiz.

`Box<T>` türü, `Deref` trait'ini uyguladığı için bir akıllı işaretçidir; bu, `Box<T>` değerlerinin referans gibi kullanılmasına olanak tanır. Bir `Box<T>` değeri kapsamdan çıktığında, kutunun işaret ettiği heap verisi de `Drop` trait'i sayesinde temizlenir. Bu iki trait, bu bölümün geri kalanında ele alacağımız diğer akıllı işaretçi türlerinin sunduğu işlevsellik için daha da önemli olacaktır. Şimdi bu iki trait'i daha ayrıntılı inceleyelim.

[trait-objects]: ch18-02-trait-objects.md#ortak-davranışları-soyutlamak-için-trait-nesneleri-kullanmak
