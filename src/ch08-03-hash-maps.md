## Hash Haritalarında İlişkili Değerlerle Anahtarları Depolama

Yaygın koleksiyonlarımızın sonuncusu _hash haritası_dır. `HashMap<K, V>` türü,
_hashing
fonksiyonu kullanarak `K` türündeki anahtarların `V` türündeki değerlere eşlenmesini depolar. Bu fonksiyon, bu anahtar ve değerlerin belleğe nasıl yerleştirileceğini belirler.
Birçok programlama dili bu tür veri yapısını destekler, ancak genellikle
_hash_, _map_, _object_, _hash table_,
_dictionary_ veya _associative array_ gibi farklı isimler kullanırlar.

Hash haritaları, vektörlerde olduğu gibi bir dizin kullanarak değil,
herhangi bir türde olabilen bir anahtar kullanarak verileri aramak istediğinizde kullanışlıdır. Örneğin,
bir oyunda, her takımın puanını, her anahtarın takım adı ve değerlerin her takımın puanı olduğu bir hash haritasında
takip edebilirsiniz. Bir takım adı verildiğinde,
o takımın puanını alabilirsiniz.

Bu bölümde hash haritalarının temel API'sini inceleyeceğiz, ancak standart kütüphanede `HashMap<K, V>` üzerinde tanımlanan işlevlerde
daha birçok yararlı özellik bulunmaktadır.
Her zaman olduğu gibi, daha fazla bilgi için standart kütüphane belgelerine bakın.

Translated with DeepL.com (free version)

### Yeni Bir Hash Haritası Oluşturma

Boş bir hash haritası oluşturmanın bir yolu, `new` kullanmak ve
`insert` ile öğeler eklemektir. Listing 8-20'de, isimleri _Blue_ ve _Yellow_ olan iki takımın puanlarını takip ediyoruz.
Mavi takım 10 puanla başlıyor ve
Sarı takım 50 puanla başlıyor.

<Listing number="8-20" caption="Creating a new hash map and inserting some keys and values">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

</Listing>

Öncelikle, standart kütüphanenin koleksiyonlar bölümünden `HashMap`'i `use` etmemiz gerektiğini unutmayın.
Üç yaygın koleksiyonumuzdan bu en az
sık kullanılanıdır, bu nedenle önsözde otomatik olarak kapsama dahil edilen özellikler arasında
yer almaz. Hash haritaları da standart kütüphaneden daha az destek alır;
örneğin, bunları oluşturmak için yerleşik bir makro yoktur.

Vektörler gibi, hash haritaları da verilerini yığın üzerinde depolar. Bu `HashMap`,
`String` türünde anahtarlar ve `i32` türünde değerler içerir. Vektörler gibi, hash haritaları da
homojendir: tüm anahtarlar aynı türe sahip olmalı ve tüm değerler
aynı türe sahip olmalıdır.

### Hash Haritasındaki Değerlere Erişme

Listing 8-21'de gösterildiği gibi, `get`
yöntemine anahtarını sağlayarak hash haritasından bir değer alabiliriz.

<Listing number="8-21" caption="Accessing the score for the Blue team stored in the hash map">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

</Listing>

Burada, `score` mavi takımla ilişkili değeri alacak ve
sonuç `10` olacaktır. `get` yöntemi bir `Option<&V>` döndürür; eğer hash haritasında o anahtar için bir değer yoksa,
`get` `None` döndürür. Bu program,
`Option`'ı `copied`'i çağırarak bir `Option<i32>` yerine bir
`unwrap_or`'u çağırarak `score`'u sıfıra ayarlar.


Hash haritasındaki her anahtar-değer çiftini, vektörlerde yaptığımız gibi benzer bir şekilde
`for` döngüsü kullanarak yineleyebiliriz:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Bu kod, her çifti rastgele bir sırayla yazdıracaktır:

```text
Yellow: 50
Blue: 10
```

### Hash Haritaları ve Sahiplik

`i32` gibi `Copy` özelliğini uygulayan türler için, değerler hash haritasına kopyalanır.
`String` gibi sahip olunan değerler için, değerler taşınır ve
hash haritası bu değerlerin sahibi olur, Listing 8-22'de gösterildiği gibi.

<Listing number="8-22" caption="Showing that keys and values are owned by the hash map once they’re inserted">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

</Listing>

`insert` çağrısı ile hash haritasına taşındıktan sonra `field_name` ve `field_value` değişkenlerini kullanamayız.


Hash haritasına değerlere referanslar eklersek, değerler hash haritasına taşınmaz.
Referansların işaret ettiği değerler, en azından hash haritası geçerli olduğu sürece geçerli olmalıdır.

Bu konular hakkında daha fazla bilgi için Bu konular hakkında daha fazla bilgiyi
[“Ömür Süresi ile Referansları Doğrulama”][validating-references-with-lifetimes]<!-- ignore --> bölümünde bulabilirsiniz.

### Hash Haritasını Güncelleme

Anahtar ve değer çiftlerinin sayısı artabilir, ancak her bir benzersiz anahtar
aynı anda yalnızca bir değerle ilişkilendirilebilir (ancak bunun tersi geçerli değildir:
örneğin, hem Mavi takım hem de Sarı takım `scores` hash haritasında `10`
değerini saklayabilir).

Hash haritasındaki verileri değiştirmek istediğinizde, bir anahtara zaten bir değer atanmışsa
bu durumu nasıl ele alacağınıza karar vermelisiniz. Eski değeri tamamen göz ardı ederek
eski değeri yeni değerle değiştirebilirsiniz. Eski değeri koruyup yeni değeri
göz ardı edebilir, yalnızca anahtarın zaten bir değeri yoksa yeni değeri ekleyebilirsiniz.
Ya da eski değeri ve yeni değeri birleştirebilirsiniz. Bunların her birinin nasıl yapıldığını
inceleyelim!

#### Bir Değeri Üzerine Yazma

Bir anahtar ve bir değeri bir hash haritasına ekledikten sonra aynı anahtarı
farklı bir değerle eklediğimizde, o anahtarla ilişkili değer değiştirilir.
Listing 8-23'teki kodda `insert` iki kez çağrılsa da, hash haritası
sadece bir anahtar-değer çifti içerecektir, çünkü her iki seferde de Blue
takımının anahtarı için değeri ekliyoruz.

<Listing number="8-23" caption="Replacing a value stored with a particular key">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

</Listing>

This code will print `{"Blue": 25}`. The original value of `10` has been
overwritten.

<!-- Old headings. Do not remove or links may break. -->

<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Anahtar ve Değerin Yalnızca Anahtar Mevcut Değilse Eklenmesi

Hash haritasında belirli bir anahtarın bir değerle birlikte zaten mevcut olup olmadığını kontrol etmek ve ardından aşağıdaki eylemleri gerçekleştirmek yaygın bir uygulamadır: anahtar hash haritasında mevcutsa, mevcut değer olduğu gibi kalmalıdır; anahtar mevcut değilse, anahtar ve değeri eklenmelidir.

Hash haritaları, kontrol etmek istediğiniz anahtarı parametre olarak alan `entry` adlı özel bir API'ye sahiptir.

Hash haritaları, kontrol etmek istediğiniz anahtarı parametre olarak alan `entry` adlı özel bir API'ye sahiptir.
`entry` yönteminin dönüş değeri, var olan veya olmayan bir değeri temsil eden
`Entry` adlı bir enumdur. Diyelim ki
Sarı takımın anahtarının bir değeri olup olmadığını kontrol etmek istiyoruz.
 Eğer yoksa, `50` değerini eklemek istiyoruz ve aynı şeyi
Mavi takım için de yapmak istiyoruz. `entry` API'sini kullanarak, kod Listing 8-24 gibi görünür.

<Listing number="8-24" caption="Using the `entry` method to only insert if the key does not already have a value">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

</Listing>

`Entry` üzerindeki `or_insert` yöntemi, ilgili `Entry` anahtarı varsa, bu anahtarın
değerine değiştirilebilir bir referans döndürmek üzere tanımlanmıştır. Anahtar yoksa,
parametreyi bu anahtarın yeni değeri olarak ekler ve yeni değere değiştirilebilir bir
referans döndürür. Bu teknik, mantığı kendimiz yazmaktan çok daha temizdir ve
ayrıca ödünç kontrolü ile daha uyumludur.

Listing 8-24'teki kodu çalıştırdığınızda `{“Yellow”: 50, “Blue”: 10}` çıktısı alınır.
`entry`'ye yapılan ilk çağrı, Yellow takımının henüz bir değeri olmadığı için,
Yellow takımının anahtarını `50` değeriyle ekler. `entry`'ye yapılan ikinci çağrı,
Blue takımının zaten `10` değerine sahip olduğu için hash haritasını değiştirmez.

#### Eski Değere Göre Bir Değeri Güncelleme

Hash haritalarının bir başka yaygın kullanım alanı, bir anahtarın değerini aramak ve ardından
eski değere göre güncellemektir. Örneğin, Listing 8-25, bir metinde her kelimenin kaç kez geçtiğini
saymak için kullanılan kodu gösterir. Anahtarlar olarak kelimeleri içeren bir hash haritası kullanıyoruz ve
o kelimeyi kaç kez gördüğümüzü takip etmek için değeri artırıyoruz. Bir kelimeyi
ilk kez görüyorsak, önce `0` değerini ekliyoruz.

<Listing number="8-25" caption="Counting occurrences of words using a hash map that stores words and counts">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

</Listing>

Bu kod `{“world”: 2, ‘hello’: 1, “wonderful”: 1}` yazdırır.
Aynı anahtar-değer çiftlerinin farklı bir sırayla yazdırıldığını görebilirsiniz: [“Hash Map'te Değerlere Erişme”][access]<!-- ignore --> bölümünden hatırlayacağınız gibi, hash map üzerinde yineleme
rastgele bir sırayla gerçekleşir.

`split_whitespace` yöntemi, `text` içindeki değerin boşluklarla ayrılmış alt dilimler üzerinde bir yineleyici döndürür.
`or_insert` yöntemi, belirtilen anahtarın değerine değiştirilebilir bir
referans (`&mut V`) döndürür. Burada, bu
değiştirilebilir referansı `count` değişkeninde saklıyoruz, bu nedenle bu değere atama yapmak için,
önce yıldız işareti (`*`) kullanarak `count` referansını kaldırmalıyız. Değiştirilebilir
referans, `for` döngüsünün sonunda kapsam dışı kalır, bu nedenle tüm bu
değişiklikler güvenlidir ve ödünç alma kuralları tarafından izin verilir.

### Karma İşlevleri

Varsayılan olarak, `HashMap`, karma tabloları içeren hizmet reddi (DoS) saldırılarına karşı direnç sağlayabilen
_SipHash_ adlı bir karma işlevi kullanır
[^siphash]<!-- ignore -->. Bu, mevcut en hızlı karma algoritması değildir,
 ancak performans düşüşüyle birlikte gelen daha iyi güvenlik için yapılan ödün
vermeye değer. Kodunuzu profilleyip varsayılan
hash fonksiyonunun amaçlarınız için çok yavaş olduğunu fark ederseniz, farklı bir hasher
belirleyerek başka bir fonksiyona geçebilirsiniz. Bir _hasher_,
`BuildHasher` özelliğini uygulayan bir türdür. Özellikler ve bunların nasıl uygulandığı hakkında
[Bölüm 10][özellikler]<!-- ignore -->'da konuşacağız. Kendi hash işleyicinizi sıfırdan
uygulamanız gerekmez; [crates.io](https://crates.io/)<!-- ignore -->
diğer Rust kullanıcıları tarafından paylaşılan ve birçok
yaygın hash algoritmasını uygulayan hash işleyicileri sağlayan kütüphaneler içerir.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Özet

Vektörler, diziler ve karma haritalar, verileri depolamak, erişmek ve değiştirmek
için programlarda gerekli olan çok sayıda işlevsellik sağlar. İşte şimdi çözebilecek
olmanız gereken bazı alıştırmalar:

1. Bir tamsayı listesi verildiğinde, bir vektör kullanarak medyanı (sıralandığında
   ortadaki değer) ve modu (en sık görülen değer) döndürün
   ; burada bir hash haritası yardımcı olacaktır) değerini döndürün.
1. Dizileri pig latin'e dönüştürün. Her kelimenin ilk ünsüz harfi kelimenin sonuna taşınır
   ve _ay_ eklenir, böylece _first_, _irst-fay_ olur. Ünlü harfle başlayan kelimelerin
   sonuna ise _hay_ eklenir (_apple_,
   _apple-hay_ olur). UTF-8 kodlamasıyla ilgili ayrıntıları unutmayın!
1. Hash haritası ve vektörler kullanarak, kullanıcının bir şirketin departmanına
   çalışan isimleri eklemesine olanak tanıyan bir metin arayüzü oluşturun; örneğin, "Sally'yi Mühendislik
   departmanına ekle“ veya ”Amir'i Satış departmanına ekle". Ardından, kullanıcının bir departmandaki tüm
   kişilerin veya şirketteki tüm kişilerin alfabetik olarak sıralanmış bir listesini
   almasına izin verin.

Standart kütüphane API belgeleri, vektörlerin, dizilerin ve
hash haritalarının bu alıştırmalar için yararlı olacak yöntemlerini açıklamaktadır!

İşlemlerin başarısız olabileceği daha karmaşık programlara giriyoruz, bu nedenle
hata işlemeyi tartışmak için mükemmel bir zaman. Bunu bir sonraki bölümde yapacağız!

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.md#yaşam-süreleri-ile-sarkan-referansları-önleme
[access]: #hash-haritaları-ve-sahiplik
[traits]: ch10-02-traits.md
