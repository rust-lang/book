## UTF-8 Kodlu Metni Dizelerle Saklama

Bölüm 4'te dizelerden bahsetmiştik, ancak şimdi onlara daha derinlemesine bakacağız.
Yeni Rustace'ciler genellikle şu üç nedenden dolayı dizgiler konusunda takılıp kalırlar
nedenler: Rust'ın olası hataları açığa çıkarma eğilimi, dizgilerin daha
birçok programcının itibar ettiğinden daha karmaşık bir veri yapısıdır ve
UTF-8. Bu faktörler bir araya geldiğinde zor görünebilir.
diğer programlama dillerinden gelmektedir.

Dizeleri koleksiyonlar bağlamında tartışıyoruz çünkü dizeler
bayt koleksiyonu olarak uygulanır, ayrıca yararlı yöntemler sağlamak için bazı yöntemler
bu baytlar metin olarak yorumlandığında işlevsellik. Bu bölümde, şunları yapacağız
gibi her koleksiyon türünün sahip olduğu `String` üzerindeki işlemler hakkında konuşun.
oluşturma, güncelleme ve okuma. Ayrıca `String`'in hangi yollarla kullanıldığını da tartışacağız.
diğer koleksiyonlardan farklıdır, yani bir `String`e indekslemenin nasıl olduğu
İnsanların ve bilgisayarların yorumlama biçimleri arasındaki farklar nedeniyle karmaşık
`String` veri.

### String Nedir?

İlk olarak _string_ terimi ile ne kastettiğimizi tanımlayacağız. Rust'ta yalnızca bir dize vardır
türü, çekirdek dilde genellikle görülen string dilimi `str`dir
'in ödünç alınmış hali olan `&str`. Bölüm 4'te _string dilimleri_ hakkında konuştuk,
bunlar başka bir yerde depolanan UTF-8 kodlu dize verilerine referanslardır. Dize
Örneğin, değişmezler programın ikili dosyasında saklanır ve bu nedenle
string dilimleri.

yerine Rust'ın standart kütüphanesi tarafından sağlanan `String` türü
çekirdek dile kodlanmış, büyütülebilir, değiştirilebilir, sahipli, UTF-8 kodlu bir
string türü. Rustacean'lar Rust'ta “string ”lerden bahsettiklerinde şu şekilde olabilirler
sadece birine değil, `String` veya string slice `&str` türlerine atıfta bulunur
bu türlerin. Bu bölüm büyük ölçüde `String` ile ilgili olsa da, her iki tür de
Rust'ın standart kütüphanesinde yoğun olarak kullanılır ve hem `String` hem de string dilimleri
UTF-8 kodludur.

### Yeni String Oluşturma

`Vec<T>` ile kullanılabilen işlemlerin çoğu `String` ile de kullanılabilir
çünkü `String` aslında bir vektör etrafında bir sarmalayıcı olarak uygulanmaktadır
bazı ekstra garantiler, kısıtlamalar ve yeteneklere sahip baytlardan oluşur. Bir örnek
`Vec<T>` ve `String` ile aynı şekilde çalışan bir fonksiyonun `new`
işlevini kullanarak bir örnek oluşturun, Listing 8-11'de gösterilmiştir.


<Listing number="8-11" caption="Creating a new, empty `String`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```
</Listing>

Bu satır `s` adında yeni, boş bir dize oluşturur ve bu dizeye şunları yükleyebiliriz
veri. Çoğunlukla, başlamak istediğimiz bazı başlangıç verilerine sahip oluruz
string. Bunun için, herhangi bir tür üzerinde kullanılabilen `to_string` yöntemini kullanırız
dizge değişmezlerinin yaptığı gibi `Display` özelliğini uygular. Liste 8-12 şunları gösterir
iki örnek.

<Listing number="8-12" caption="Using the `to_string` method to create a `String` from a string literal">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

</Listing>

Bu kod `initial contents` içeren bir string oluşturur.

Bir stringten `String` oluşturmak için `String::from` fonksiyonunu da kullanabiliriz
gerçek. Liste 8-13'teki kod, Liste 8-12'deki koda eşdeğerdir
to_string` kullanan

<Listing number="8-13" caption="Using the `String::from` function to create a `String` from a string literal">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

</Listing>

Stringler çok fazla şey için kullanıldığından, birçok farklı jenerik
Dizeler için API'ler, bize birçok seçenek sunar. Bazıları görünebilir
gereksizdir, ancak hepsinin bir yeri vardır! Bu durumda, `String::from` ve
to_string` aynı şeyi yapar, bu nedenle hangisini seçeceğiniz bir stil meselesidir ve
okunabilirlik.

Dizelerin UTF-8 kodlu olduğunu unutmayın, bu nedenle düzgün kodlanmış herhangi bir
Liste 8-14'te gösterildiği gibi, içlerindeki veriler.

<Listing number="8-14" caption="Storing greetings in different languages in strings">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

</Listing>

Bunların hepsi geçerli `String` değerleridir.

### Bir Dizeyi Güncelleme

Bir `String` boyut olarak büyüyebilir ve içeriği değişebilir, tıpkı içeriği gibi
içine daha fazla veri iterseniz, bir `Vec<T>`. Ek olarak, rahatça şunları yapabilirsiniz
`String` değerlerini birleştirmek için `+` operatörünü veya `format!` makrosunu kullanın.

#### `push_str` ve `push` ile bir Dizeye Ekleme

Bir dize dilimi eklemek için `push_str` yöntemini kullanarak bir `String`i büyütebiliriz,
Liste 8-15'te gösterildiği gibi.


<Listing number="8-15" caption="Appending a string slice to a `String` using the `push_str` method">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

</Listing>

Bu iki satırdan sonra, `s` `foobar` içerecektir. push_str` yöntemi bir
dize diliminin sahipliğini almak istemediğimiz için
parametresini kullanabiliriz. Örneğin, Listing 8-16'daki kodda, aşağıdaki parametreyi kullanabilmek istiyoruz
s2` içeriğini `s1`e ekledikten sonra.

<Listing number="8-16" caption="Using a string slice after appending its contents to a `String`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

</Listing>

Eğer `push_str` metodu `s2` metodunun sahipliğini alsaydı, şunları yazdıramazdık
son satırdaki değeri. Ancak, bu kod beklediğimiz gibi çalışıyor!

Push` metodu parametre olarak tek bir karakter alır ve onu
`String`. Liste 8-17 `push` kullanarak bir `String`e _l_ harfini ekler
yöntem.

<Listing number="8-17" caption="Adding one character to a `String` value using `push`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

</Listing>

Sonuç olarak, `s` `lol` içerecektir.

#### `+` Operatörü veya `format!` Makrosu ile Birleştirme

Genellikle, mevcut iki dizeyi birleştirmek istersiniz. Bunu yapmanın bir yolu
Liste 8-18'de gösterildiği gibi `+` operatörü.

<Listing number="8-18" caption="Using the `+` operator to combine two `String` values into a new `String` value">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

</Listing>

s3` dizesi `Hello, world!` içerecektir. s1`in artık olmamasının nedeni
eklendikten sonra geçerlidir ve `s2` referansını kullanmamızın nedeni şudur
operatörünü kullandığımızda çağrılan metodun imzası ile aynıdır.
`+` operatörü, imzası aşağıdaki gibi görünen `add` metodunu kullanır
Bu:

```rust,ignore
fn add(self, s: &str) -> String {
```

Standart kütüphanede, jenerikler kullanılarak tanımlanmış `add` ve ilişkili
türleri. Burada, somut türler ile yer değiştirdik, bu da
bu yöntemi `String` değerleri ile çağırın. Jenerikleri Bölüm 10'da tartışacağız.
Bu imza bize, aşağıdaki zorluğu anlamak için ihtiyacımız olan ipuçlarını verir
operatörünün bitleri.

İlk olarak, `s2` bir `&` içerir, bu da ikinci bir _referans_ eklediğimiz anlamına gelir
dizesini ilk dizeye ekler. Bunun nedeni `add` komutundaki `s` parametresidir
fonksiyonu: bir `String`e sadece bir `&str` ekleyebiliriz; iki `String` ekleyemeyiz
değerlerini bir araya getirir. Ancak bekleyin - `&s2`nin türü `&str` değil `&String`dir.
'e ikinci parametre olarak belirtilir. Peki Liste 8-18 neden derleniyor?

`add` çağrısında `&s2` kullanabilmemizin nedeni, derleyicinin
`&String` argümanını bir `&str`ye _coerce_ edebilir. `add` komutunu çağırdığımızda
metodunda, Rust burada `&s2`yi `&s2[..]`ye dönüştüren bir _deref coercion_ kullanır.
Deref zorlamasını Bölüm 15'te daha derinlemesine tartışacağız. Çünkü `add` şunları yapar
`s` parametresinin sahipliğini almazsa, `s2` hala geçerli bir `String` olacaktır
bu işlemden sonra.

İkinci olarak, imzada `add` işleminin `self` işleminin sahipliğini aldığını görebiliriz
çünkü `self` bir `&` ye sahip değildir. Bu, `Liste 8-18`deki `s1`in şöyle olacağı anlamına gelir
`add` çağrısına taşınmıştır ve bundan sonra artık geçerli olmayacaktır. Yani, her ne kadar
`let s3 = s1 + &s2;` her iki dizeyi de kopyalayacak ve yeni bir tane oluşturacak gibi görünüyor,
bu ifade aslında sahibini
Birden fazla dizeyi birleştirmemiz gerekirse, `+` işlecinin davranışı
hantal olur:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

Bu noktada, `s` `tic-tac-toe` olacaktır. Tüm `+` ve `"` ile
karakterleri varsa, neler olup bittiğini görmek zordur. Dizeleri birleştirmek için
daha karmaşık yollar yerine `format!` makrosunu kullanabiliriz:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Bu kod ayrıca `s` değerini `tic-tac-toe` olarak ayarlar. `format!` makrosu şu şekilde çalışır
`println!`, ancak çıktıyı ekrana yazdırmak yerine, bir
içeriğiyle birlikte `String`. Kodun `format!` kullanan sürümü çok
daha kolay okunur ve `format!` makrosu tarafından oluşturulan kod referansları kullanır
böylece bu çağrı hiçbir parametresinin sahipliğini almaz.

### Dizelere İndeksleme

Diğer birçok programlama dilinde, bir karakterdeki tek tek karakterlere erişmek
dizesini dizine göre referans vererek kullanmak geçerli ve yaygın bir işlemdir. Ancak,
Rust'ta indeksleme sözdizimini kullanarak bir `String`in parçalarına erişmeye çalışırsanız
bir hata alırsınız. Liste 8-19'daki geçersiz kodu düşünün.


<Listing number="8-19" caption="Attempting to use indexing syntax with a String">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

</Listing>

This code will result in the following error:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

Hata ve not hikayeyi anlatıyor: Rust dizeleri indekslemeyi desteklemez. Fakat
neden olmasın? Bu soruyu yanıtlamak için, Rust'ın dizeleri nasıl sakladığını tartışmamız gerekir
Hafıza.

#### İç Temsil

Bir `String`, bir `Vec<u8>` üzerinde bir sarmalayıcıdır. Şimdi bazı düzgünlerimize bakalım
Liste 8-14'teki UTF-8 kodlu örnek dizeler. İlk olarak, bu:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

Bu durumda, `len` `4` olacaktır, bu da dizeyi depolayan vektör anlamına gelir
“Hola”` 4 bayt uzunluğundadır. Bu harflerin her biri şu şekilde kodlandığında bir bayt alır
UTF-8. Ancak aşağıdaki satır sizi şaşırtabilir (bu dizenin
büyük Kiril harfi _Ze_ ile başlar, 3 rakamı ile değil):

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

İpin ne kadar uzun olduğu sorulsaydı 12 diyebilirdiniz. Aslında, Rust'ın
cevap 24'tür: bu, “Здравствуйте” kelimesini kodlamak için gereken bayt sayısıdır.
UTF-8, çünkü bu dizideki her Unicode skaler değeri 2 bayt
depolama. Bu nedenle, dizenin baytlarına yönelik bir dizin her zaman aşağıdakilerle ilişkili olmayacaktır
geçerli bir Unicode skaler değerine dönüştürür. Göstermek için şu geçersiz Rust'ı düşünün
Kod:

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

Zaten `cevap`ın ilk harf olan `З` olmayacağını biliyorsunuz. Kodlandığında
UTF-8'de `З`nin ilk baytı `208` ve ikincisi `151`dir, bu nedenle
aslında `cevap`ın `208` olması gerekir, ancak `208` geçerli bir karakter değildir
kendi başına. `208` döndürmek muhtemelen bir kullanıcının sorduğunda isteyeceği şey değildir
Bu dizenin ilk harfi için; ancak, Rust'ın bu dizeye ilişkin tek verisi
bayt dizini 0'da bulunur. Kullanıcılar genellikle bayt değerinin döndürülmesini istemez, hatta
eğer dize sadece Latin harfleri içeriyorsa: eğer `&“hi”[0]` geçerli bir kod olsaydı
bayt değerini döndürdüğünde, `h` değil `104` değerini döndürecektir.

O halde cevap, beklenmedik bir değer döndürmekten kaçınmak ve
Hemen keşfedilemeyebilecek hatalar, Rust bu kodu derlemez
ve geliştirme sürecinin başlarında yanlış anlamaları önler.

#### Baytlar ve Skaler Değerler ve Grapheme Kümeleri! Aman Tanrım!

UTF-8 ile ilgili bir başka nokta da aslında üç ilgili yol olduğudur
dizelere Rust'ın bakış açısından bakın: baytlar, skaler değerler ve grafem olarak
kümelerdir (bizim _harfler_ dediğimiz şeye en yakın şey).

Devanagari alfabesiyle yazılmış Hintçe “नमस्ते” kelimesine bakarsak
aşağıdaki gibi görünen `u8` değerlerinden oluşan bir vektör olarak saklanır:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

Bu 18 bayttır ve bilgisayarlar bu verileri nihai olarak bu şekilde depolar. Eğer bakarsak
Unicode skaler değerler olarak, ki Rust'ın `char` tipi de budur, bu
baytlar böyle görünür:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

cBurada altı `char` değeri vardır, ancak dördüncü ve altıncı harf değildir:
kendi başlarına bir anlam ifade etmeyen aksan işaretleridir. Son olarak, eğer bakarsak
onları grapheme kümeleri olarak, bir kişinin dört harf olarak adlandıracağı şeyi elde ederiz
Hintçe kelimeyi oluşturan:

```text
["न", "म", "स्", "ते"]
```

Rust, bilgisayarların ham dize verilerini yorumlamak için farklı yollar sağlar
depolayın, böylece her program ihtiyaç duyduğu yorumu seçebilir, ne olursa olsun
verinin hangi insan dilinde olduğu.

Rust'ın bir `String`i indekslememize izin vermemesinin son bir nedeni
karakteri, indeksleme işlemlerinin her zaman sabit zaman almasının beklenmesidir
(O(1)). Ancak bir `String` ile bu performansı garanti etmek mümkün değildir,
Çünkü Rust, içeriği baştan sona gözden geçirmek zorunda kalacaktı.
kaç tane geçerli karakter olduğunu belirlemek için dizin.

### Dizeleri Dilimleme

Bir dizeye indeksleme yapmak genellikle kötü bir fikirdir çünkü
dize indeksleme işleminin dönüş türü şu şekilde olmalıdır: bir bayt değeri, bir
karakteri, bir grapheme kümesi veya bir dize dilimi. Gerçekten kullanmanız gerekiyorsa
dizgi dilimleri oluşturmak için indeksler, bu nedenle Rust sizden daha spesifik olmanızı ister.

Tek bir sayı ile `[]` kullanarak indeksleme yapmak yerine, bir sayı ile `[]` kullanabilirsiniz.
belirli baytları içeren bir dize dilimi oluşturmak için aralık:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Burada `s`, dizenin ilk dört baytını içeren bir `&str` olacaktır.
Daha önce, bu karakterlerin her birinin iki bayt olduğundan bahsetmiştik, yani
`s`, `Зд` olacaktır.

Bir karakterin baytlarının sadece bir kısmını aşağıdaki gibi bir şeyle kesmeye çalışsaydık
`&hello[0..1]`, Rust çalışma zamanında geçersiz bir
dizinine bir vektör içinde erişildi:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

Aralıklarla dize dilimleri oluştururken dikkatli olmalısınız, çünkü
bu yüzden programınızı çökertebilir.

### Dizeler Üzerinde Yineleme Yöntemleri

String parçaları üzerinde işlem yapmanın en iyi yolu
karakter veya bayt istiyorsunuz. Tek tek Unicode skaler değerleri için
`chars` yöntemi. “Зд” üzerinde `chars` çağrıldığında, iki değer ayrılır ve döndürülür
türünde `char` yazabilir ve her bir öğeye erişmek için sonuç üzerinde yineleme yapabilirsiniz:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Bu kod aşağıdakileri yazdıracaktır:

```text
З
д
```

Alternatif olarak, `bytes` yöntemi her bir ham baytı döndürür, bu da
alanınız için uygun:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Bu kod, bu dizeyi oluşturan dört baytı yazdıracaktır:

```text
208
151
208
180
```

Ancak geçerli Unicode skaler değerlerinin daha fazla değerden oluşabileceğini unutmayın
bir bayttan fazla.

Devanagari yazısında olduğu gibi, dizelerden grafem kümeleri elde etmek
karmaşıktır, bu nedenle bu işlevsellik standart kütüphane tarafından sağlanmamaktadır. Kasalar [crates.io](https://crates.io/)<!-- ignore --> adresinde mevcuttur, eğer bu
İhtiyacınız olan işlevsellik.

### Dizeler O Kadar Basit Değil

Özetlemek gerekirse, dizgiler karmaşıktır. Farklı programlama dilleri
Bu karmaşıklığın programcıya nasıl sunulacağı konusunda farklı seçimler. Pas
`String` verilerinin doğru işlenmesini varsayılan davranış haline getirmeyi seçmiştir
Bu da programcıların tüm Rust programları için daha fazla düşünmesi gerektiği anlamına gelir.
UTF-8 verilerini önden işleme. Bu değiş tokuş, daha fazla karmaşıklığı ortaya çıkarır
Diğer programlama dillerinde görülenden daha fazla dizgi, ancak sizi engeller
ASCII olmayan karakterleri içeren hataları daha sonra ele almak zorunda kalmamak için
geliştirme yaşam döngüsü.

İyi haber şu ki, standart kütüphane birçok işlevsellik sunuyor
Bu karmaşık durumların üstesinden gelmeye yardımcı olmak için `String` ve `&str` türlerini
Doğru şekilde. Aşağıdaki gibi yararlı yöntemler için belgelere göz attığınızdan emin olun
Bir dize içinde arama yapmak için `contains` ve bir dizenin parçalarını değiştirmek için `replace`
dizgesini başka bir dizgeyle eşleyebiliriz.

Biraz daha az karmaşık bir şeye geçelim: hash haritaları!
