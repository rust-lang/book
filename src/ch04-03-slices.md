## Dilim Tipi

_Slices_, bir öğedeki bitişik bir öğe dizisine başvurmanıza olanak tanır
[collection](ch08-00-common-collections.md)<!-- ignore -->. Dilim bir türdür
referansına sahip değildir, bu nedenle sahipliğe sahip değildir.

İşte küçük bir programlama problemi: bir dizi alan bir fonksiyon yazın
sözcükleri boşluklarla ayırır ve bu dizede bulduğu ilk sözcüğü döndürür.
İşlev dizede bir boşluk bulamazsa, tüm dize
bir kelime, bu nedenle tüm dize döndürülmelidir.

> Not: Dize dilimlerini tanıtmak amacıyla, ASCII
> UTF-8 kullanımına ilişkin daha kapsamlı bir tartışma bu bölümde yer almaktadır.
> [“Storing UTF-8 Encoded Text with Strings”][strings]<!-- ignore --> bölüm
>Bölüm 8'in içinde. 

Bu fonksiyonun imzasını kullanmadan nasıl yazacağımızı inceleyelim
dilimleri, dilimlerin çözeceği sorunu anlamak için:

```rust,ignore
fn first_word(s: &String) -> ?
```

first_word` fonksiyonunun `&String` tipinde bir parametresi vardır. İhtiyacımız yok
sahiplik, bu yüzden sorun yok. (Rust deyiminde, fonksiyonlar sahiplik almaz
Gerekmedikçe argümanlarını kullanmayacaklardır ve bunun nedenleri
devam ettikçe netleşecek). Ama neyi iade etmeliyiz? Gerçekten bir yolumuz yok
bir dizenin *bir kısmı* hakkında konuşmak için. Bununla birlikte, sonun indeksini döndürebiliriz
sözcüğün boşluk ile gösterilen kısmı. Liste 4-7'de gösterildiği gibi bunu deneyelim.

<Listing number="4-7" file-name="src/main.rs" caption="The `first_word` function that returns a byte index value into the `String` parameter">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

</Listing>

Çünkü `String` öğesini öğe öğe incelememiz ve
değerinin bir boşluk olduğunu varsayarsak, `String` değerimizi bir bayt dizisine dönüştürmek için
as_bytes` yöntemi.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

Daha sonra, `iter` yöntemini kullanarak bayt dizisi üzerinde bir yineleyici oluşturuyoruz:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

Yineleyicileri [Bölüm 13][ch13]<!-- ignore --> bölümünde daha ayrıntılı olarak ele alacağız.
Şimdilik, `iter`in bir koleksiyondaki her bir öğeyi döndüren bir yöntem olduğunu bilin
ve `enumerate` öğesinin `iter` öğesinin sonucunu sardığını ve her bir öğeyi
yerine bir tuple'ın parçası. 'den döndürülen tuple'ın ilk elemanı
`enumerate` indeks, ikinci eleman ise elemana bir referanstır.
Bu, indeksi kendimiz hesaplamaktan biraz daha kullanışlıdır.

Çünkü `enumerate` metodu bir tuple döndürür, kalıpları şu şekilde kullanabiliriz
bu tuple'ı yok eder. Kalıpları [Bölüm]'de daha fazla tartışacağız.
6][ch6]<!-- ignore -->. `for` döngüsünde, `i` olan bir kalıp belirtiriz
tuple içindeki indeks için ve `&item` tuple içindeki tek bayt için.
Elemana `.iter().enumerate()` işlevinden bir referans aldığımız için
`&` olarak tanımlıyoruz.

`for` döngüsünün içinde, boşluğu temsil eden baytı şu şekilde ararız
byte literal sözdizimini kullanarak. Eğer bir boşluk bulursak, pozisyonu döndürürüz.
Aksi takdirde, `s.len()` kullanarak dizenin uzunluğunu döndürürüz.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

Artık ilk kelimenin sonunun indeksini bulmak için bir yolumuz var.
string'i döndürüyoruz, ancak bir sorun var. Kendi başına bir `usize` döndürüyoruz, ancak bu
sadece `&String` bağlamında anlamlı bir sayıdır. Başka bir deyişle,
'String'den ayrı bir değer olduğu için, bunun bir garantisi yoktur.
gelecekte de geçerli olacaktır. Liste 4-8'deki programı düşünün
Listing 4-7'deki `first_word` fonksiyonunu kullanır.

<Listing number="4-8" file-name="src/main.rs" caption="Storing the result from calling the `first_word` function and then changing the `String` contents">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

</Listing>

Bu program herhangi bir hata olmadan derlenir ve `word` kullansaydık da öyle olurdu
`s.clear()` çağrısından sonra. Çünkü `word` `s`nin durumuna bağlı değildir
hiç değilse, `word` hala `5` değerini içerir. Bu değeri `5` ile kullanabiliriz
ilk kelimeyi çıkarmaya çalışmak için `s` değişkenini kullanabilir, ancak bu bir hata olacaktır
çünkü `5`i `word`e kaydettiğimizden beri `s`nin içeriği değişti.

`word` içindeki indeksin `word` içindeki verilerle senkronize olmaması konusunda endişelenmek
`s` sıkıcı ve hataya meyillidir! Bu endeksleri yönetmek aşağıdaki durumlarda daha da kırılgandır
bir `second_word` fonksiyonu yazıyoruz. İmzası şu şekilde görünmelidir:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Şimdi bir başlangıç _ve_ bir bitiş dizinini izliyoruz ve daha da fazlasına sahibiz
Belirli bir durumdaki verilerden hesaplanan ancak belirli bir duruma bağlı olmayan değerler
bu durum hiç yok. Etrafta dolaşan üç ilgisiz değişkenimiz var
senkronize tutulmalıdır.

Neyse ki Rust'ın bu soruna bir çözümü var: string dilimleri.

### Dize Dilimleri

Bir _string slice_, bir _string slice_ öğesinin bitişik bir dizisine referanstır.
`String`, ve şöyle görünür:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

Tüm `String`e bir referans yerine, `hello` bir `String`e referanstır.
ekstra `[0..5]` bitinde belirtilen `String` bölümü. Dilimler oluşturuyoruz
'`[başlangıç_indeksi..bitiş_indeksi]` belirterek parantez içinde bir aralık kullanarak,
burada _`starting_index`_ dilimdeki ilk konum ve _`ending_index`_
dilimdeki son konumdan bir fazladır. Dahili olarak, dilim verileri
yapısı başlangıç konumunu ve dilimin uzunluğunu saklar.
_`ending_index`_ eksi _`starting_index`_ değerine karşılık gelir. Yani, `let' durumunda
world = &s[6..11];`, `world` bir işaretçi içeren bir dilim olacaktır.
uzunluk değeri `5` olan `s` dizininin 6. byte'ıdır.

Şekil 4-7 bunu bir diyagramda göstermektedir.

<img alt="Three tables: a table representing the stack data of s, which points
to the byte at index 0 in a table of the string data &quot;hello world&quot; on
the heap. The third table rep-resents the stack data of the slice world, which
has a length value of 5 and points to byte 6 of the heap data table."
src="img/trpl04-07.svg" class="center" style="width: 50%;" />

<span class="caption">Şekil 4-7:
`String`</span>'in bir kısmına atıfta bulunan String dilimi

Rust'ın `..` aralık sözdizimiyle, 0 dizininden başlamak istiyorsanız,
adresini iki noktadan önceki değere bırakabilirsiniz. Başka bir deyişle, bunlar eşittir:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

Aynı şekilde, diliminiz `String`in son baytını içeriyorsa
sondaki sayıyı düşürebilir. Bu, bunların eşit olduğu anlamına gelir:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

Tüm dizenin bir dilimini almak için her iki değeri de bırakabilirsiniz. Yani bunlar
eşittir:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> Not: Dize dilimi aralık indisleri geçerli UTF-8 karakterlerinde bulunmalıdır
> sınırlar. Bir dize diliminin ortasında bir dize dilimi oluşturmaya çalışırsanız
> çok baytlı karakter döndürürse, programınız bir hata ile çıkacaktır.

Tüm bu bilgileri aklımızda tutarak, `first_word` öğesini bir
slice. “Dize dilimi” anlamına gelen tür `&str` olarak yazılır:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

</Listing>

Sözcüğün sonu için indeksi, Liste 4-7'de yaptığımız gibi, şu şekilde elde ederiz
ilk boşluk oluşumunu arar. Bir boşluk bulduğumuzda, bir
olarak dizenin başlangıcını ve boşluğun dizinini kullanarak dize dilimi
başlangıç ve bitiş indisleri.

Şimdi `first_word` dediğimizde, başlangıç ve bitiş indislerine bağlı tek bir değer alırız.
temel veri. Değer, verinin başlangıç noktasına yapılan bir referanstan oluşur.
dilim ve dilimdeki eleman sayısı.

Bir dilim döndürmek `second_word` işlevi için de işe yarayacaktır:


```rust,ignore
fn second_word(s: &String) -> &str {
```

Artık karıştırması çok daha zor olan basit bir API'ye sahibiz çünkü
derleyici `String`e yapılan referansların geçerli kalmasını sağlayacaktır. Unutmayın
Liste 4-8'deki programdaki hata, indeksi dizinin sonuna getirdiğimizde
ilk kelimeyi girdik ama sonra dizeyi temizledik, böylece dizinimiz geçersiz mi oldu? Bu kod
mantıksal olarak yanlıştı ancak herhangi bir ani hata göstermiyordu. Sorunlar
ilk kelime dizinini boş bir şekilde kullanmaya devam edersek daha sonra ortaya çıkar
string. Dilimler bu hatayı imkansız hale getirir ve bize bir sorunumuz olduğunu bildirir
kodumuz çok daha erken. İlk_kelime`nin dilim versiyonunu kullanmak bir
derleme zamanı hatası:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

</Listing>

İşte derleyici hatası:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

Ödünç alma kurallarından hatırlayın, eğer değişmez bir referansımız varsa
bir şey varsa, değişebilir bir referans da alamayız. Çünkü `clear` için
`String`i kesmek için, değiştirilebilir bir referans alması gerekir. 'println!`
çağrısından sonra `clear` `word` içindeki referansı kullanır, bu nedenle değişmez
referansı o noktada hala aktif olmalıdır. Rust mutable'a izin vermez
referansının ve `word` içindeki değişmez referansın `clear` içinde var olmasını engeller.
aynı zamanda ve derleme başarısız olur. Rust sadece API'mizin kullanımını kolaylaştırmakla kalmadı,
ama aynı zamanda derleme zamanındaki bütün bir hata sınıfını da ortadan kaldırdı!

<!-- Old heading. Do not remove or links may break. -->

<a id="string-literals-are-slices"></a>

#### Dilim Olarak Dize Harfleri

Dize değişmezlerinin ikilinin içinde saklandığından bahsettiğimizi hatırlayın. Şimdi
Dilimler hakkında bildiklerimizle, dize değişmezlerini düzgün bir şekilde anlayabiliriz:

```rust
let s = "Hello, world!";
```

Buradaki `s`nin türü `&str`dir:
ikilisinin belirli bir noktasına işaret eden bir dilimdir. String değişmezlerinin değişmez olmasının nedeni de budur; `&str` bir
değişmez referansıdır.

#### Parametre Olarak String Dilimleri

Değişmezlerin ve `String` değerlerinin dilimlerini alabileceğinizi bilmek bizi
adresinde `first_word` üzerinde bir iyileştirme daha yapmaya yönlendirir ve bu da onun imzasıdır:

```rust,ignore
fn first_word(s: &String) -> &str {
```

Daha fazla bölgede bir Rustacean bunun yerine Liste 4-9'da
programı imzayı yazardı çünkü bu imza aynı fonksiyon hem `&String` değerleri
hem de `&str` değerleri üzerinde kullanmamıza izin verirdi.      

<Listing number="4-9" caption="Improving the `first_word` function by using a string slice for the type of the `s` parameter">

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

</Listing>

Eğer bir string dilimimiz varsa, bunu doğrudan aktarabiliriz. Elimizde bir `String` varsa,
`String`in bir dilimini veya `String`e bir referansı geçirebiliriz. Bu
esnekliği, Bölüm 15'in
[“Implicit Deref Coercions with Functions and Methods”][deref-coercions]<!--ignore--> bölümünde ele alacağımız bir özellik olan _deref zorlamalarından_ yararlanır.

Bir `String`
referansı yerine bir string dilimi almak için bir fonksiyon tanımlamak, API'mizi herhangi bir işlevsellik kaybı olmadan daha genel ve kullanışlı hale getirir:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

</Listing>

### Diğer Dilimler

Dize dilimleri, tahmin edebileceğiniz gibi, dizelere özgüdür. Ancak
daha genel bir dilim türü de vardır. Bu diziyi düşünün:
```rust
let a = [1, 2, 3, 4, 5];
```

Tıpkı bir dizinin bir kısmına atıfta bulunmak isteyebileceğimiz gibi, bir dizinin
kısmına da atıfta bulunmak isteyebiliriz. Bunu şu şekilde yaparız:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

Bu dilim `&[i32]` türündedir. String dilimleri ile aynı şekilde çalışır,
ilk elemana bir referans ve bir uzunluk depolar. Bu tür bir
dilimini diğer tüm koleksiyonlar için kullanacaksınız. Bu koleksiyonları Bölüm 8'de vektörlerden bahsederken
ayrıntılı olarak tartışacağız.

## Özet

Sahiplik, ödünç alma ve dilimler kavramları, Rust
programlarında derleme zamanında bellek güvenliğini sağlar. Rust dili, diğer sistem programlama dilleriyle aynı şekilde bellek
kullanımınız üzerinde kontrol sağlar, ancak
veri sahibinin kapsam dışına çıktığında bu verileri otomatik olarak temizlemesini sağlamak
bu kontrolü elde etmek için fazladan kod yazmanız ve hata ayıklamanız gerekmediği anlamına gelir.

Sahiplik, Rust'ın diğer birçok bölümünün nasıl çalıştığını etkiler, bu nedenle kitabın geri kalanında
bu kavramlar hakkında daha fazla konuşacağız. Bölüm 5'e geçelim ve veri parçalarını bir `struct' içinde gruplamaya bakalım.

[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#patterns-that-bind-to-values
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[deref-coercions]: ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
