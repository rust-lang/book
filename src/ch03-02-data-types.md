## Veri Türleri

Rust'taki her değer, Rust'a ne tür bir
veri belirtildiğini söyleyen ve böylece bu verilerle nasıl çalışacağını bilen belirli bir _data type_'a sahiptir. iki veri türü alt kümesine bakacağız: skaler ve bileşik.

Rust'ın _statically typed_ bir dil olduğunu unutmayın; bu da
derleme sırasında tüm değişkenlerin türlerini bilmesi gerektiği anlamına gelir. Derleyici genellikle
hangi türü kullanmak istediğimizi değere ve onu nasıl kullandığımıza bağlı olarak çıkarabilir. Birçok türün mümkün olduğudurumlarda
, örneğin


Bölüm 2'deki[“Tahmin ile Gizli
[SayınınKarşılaştırılması”][ comparing-the-guess-to-the-secret-number]<!-- ignore --> bölümünde`parse'
kullanarakbir `String'i sayısal bir türe dönüştürdüğümüzde


olduğu gibi, aşağıdaki gibi bir tür ek açıklaması eklemeliyiz:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Önceki kodda gösterilen `:u32` tür ek açıklamasını eklemezsek, Rust
aşağıdaki hatayı görüntüler, bu da derleyicinin hangi türü kullanmak istediğimizi bilmek için bizden daha fazla
bilgisine ihtiyaç duyduğu anlamına gelir:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Diğer veri türleri için farklı tür ek açıklamaları göreceksiniz.

### Skaler Tipler

Bir _scalar_ türü tek bir değeri temsil eder. Rust'ın dört temel skaler türü vardır:

tamsayılar, kayan noktalı sayılar, Boolean'lar ve karakterler. Bunları
diğer programlama dillerinden tanıyor olabilirsiniz. Rust'ta nasıl çalıştıklarına geçelim.

#### Tamsayı Türleri

Bir _integer_ kesirli bileşeni olmayan bir sayıdır. Bölüm 2'de bir tamsayı
türü olan `u32` türünü kullandık. Bu tip bildirimi, ilişkilendirildiği
değerinin 32 bit yer kaplayan işaretsiz bir tamsayı olması gerektiğini belirtir (işaretli tamsayı tipleri
`u` yerine `i` ile başlar). Tablo 3-1
adresinde Rust'taki yerleşik tamsayı türlerini göstermektedir. Bir tamsayı değerinin türünü
olarak bildirmek için bu varyantlardan herhangi birini kullanabiliriz.

<span class="caption">Table 3-1: Integer Types in Rust</span>

| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| architecture dependent | `isize` | `usize`  |

Her değişken işaretli veya işaretsiz olabilir ve açık bir boyuta sahiptir.
İşaretli_ ve işaretsiz_, sayının
negatif olmasının mümkün olup olmadığını, başka bir deyişle sayının yanında bir işaret olması gerekip gerekmediğini
(işaretli) veya sadece pozitif olup olmayacağını ve bu nedenle işaret olmadan
temsil edilip edilemeyeceğini (işaretsiz) ifade eder. Sayıları kağıda yazmak gibidir:
işareti önemli olduğunda, bir sayı artı işareti veya eksi işareti ile gösterilir; ancak
sayının pozitif olduğunu varsaymak güvenli olduğunda, işaret olmadan gösterilir.
İşaretli sayılar [two's complement][twos-complement]<!-- ignore kullanılarak saklanır
--> gösterimi.

Her işaretli değişken -(2<sup>n - 1</sup>) ile 2<sup>n -
1</sup> - 1 arasındaki sayıları saklayabilir, burada _n_ değişkenin kullandığı bit sayısıdır. Yani bir
`i8` -(2<sup>7</sup>) ila 2<sup>7</sup> - 1 arasındaki sayıları saklayabilir, bu da
-128 ila 127'ye eşittir. İşaretsiz değişkenler 0 ila 2<sup>n</sup> - 1 arasındaki sayıları saklayabilir,
böylece bir `u8` 0 ila 2<sup>8</sup> - 1 arasındaki sayıları saklayabilir, bu da 0 ila 255'e eşittir.

Ayrıca, `isize` ve `usize` tipleri programınızın üzerinde çalıştığı
bilgisayarının mimarisine bağlıdır: 64 bitlik bir mimarideyseniz 64 bit
32 bitlik bir mimarideyseniz 32 bit.

Tamsayı değişmezlerini Tablo 3-2'de gösterilen formlardan herhangi birinde yazabilirsiniz. Birden fazla sayısal tür olabilen sayı değişmezlerinin, türü belirtmek için `57u8` gibi bir tür sonekine
izin verdiğine
dikkat edin. Sayı değişmezleri ayrıca, sayının okunmasını kolaylaştırmak için
görsel ayırıcı olarak `_` kullanabilir; örneğin `1_000`,
, `1000` belirtmiş olmanızla aynı değere sahip olacaktır.

<span class="caption">Table 3-2: Integer Literals in Rust</span>

| Number literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

Peki hangi tamsayı türünü kullanacağınızı nasıl bileceksiniz? Emin değilseniz, Rust'ın
varsayılanları genellikle başlamak için iyi yerlerdir: tamsayı türleri varsayılan olarak `i32`dir.
isize` veya `usize` kullanacağınız birincil durum
bir çeşit koleksiyonu indekslerken olur.

> ##### Tamsayı Taşması
>
> Diyelim ki 0 ile
> 255 arasında değerler tutabilen `u8` tipinde bir değişkeniniz var. Değişkeni bu aralığın dışında bir değere değiştirmeye çalışırsanız, örneğin
> 256, _integer overflow_ meydana gelir ve bu da iki davranıştan birine neden olabilir.
> Hata ayıklama modunda derleme yaparken, Rust tamsayı taşması için kontroller içerir
> bu davranış meydana gelirse programınızın çalışma zamanında _panic_ yapmasına neden olur. Rust
> bir program bir hata ile çıktığında _panicking_ terimini kullanır;
> panikleri Bölüm
> 9'daki [“Unrecoverable Errors with
> `panic!`”][unrecoverable-errors-with-panic]<!-- ignore --> bölümünde daha derinlemesine tartışacağız.
>
> Sürüm modunda `--release` bayrağıyla derleme yaptığınızda, Rust
> _not_ paniğe neden olan tamsayı taşması kontrollerini içermez. Bunun yerine,
> taşma meydana gelirse, Rust _two's complement wrapping_ gerçekleştirir. Kısacası
> türün tutabileceği maksimum değerden büyük değerler
> türün tutabileceği minimum değerlere “sarılır”. Bir `u8` durumunda, 256 değeri
> 0 olur, 257 değeri 1 olur ve bu böyle devam eder. Program panik yapmaz, ancak
> değişkeni muhtemelen
> sahip olmasını beklediğinizden farklı bir değere sahip olacaktır. Tamsayı taşmasının sarma davranışına güvenmek bir hata olarak kabul edilir.
>
> Taşma olasılığını açıkça ele almak için, ilkel sayısal türler için standart kütüphane tarafından sağlanan yöntemlerin
> bu ailelerini kullanabilirsiniz:
>
> - `wrapping_add` gibi `wrapping_*` yöntemleriyle tüm modlarda sarın.
> - `checked_*` yöntemleriyle taşma varsa `None` değerini döndürür.
> -
> `taşan_*` yöntemleriyle taşma olup olmadığını belirten bir Boolean ve değer döndürür.
> - `saturating_*`
> yöntemleriyle değerin minimum veya maksimum değerlerinde doygunluk sağlayın.

#### Kayan Nokta Türleri

Rust ayrıca ondalık noktalı
sayıları olan _floating-point numbers_ için iki ilkel tipe sahiptir. Rust'ın kayan nokta tipleri `f32` ve `f64`tür,
bunlar sırasıyla 32 bit ve 64 bit boyutundadır. Varsayılan tür `f64`
çünkü modern CPU'larda `f32` ile kabaca aynı hızdadır ancak
daha fazla hassasiyete sahiptir. Tüm kayan nokta tipleri işaretlidir.

İşte kayan noktalı sayıları iş başında gösteren bir örnek:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Kayan noktalı sayılar IEEE-754 standardına göre temsil edilir.

#### Sayısal İşlemler

Rust, tüm sayı
türleri için beklediğiniz temel matematiksel işlemleri destekler: toplama, çıkarma, çarpma, bölme ve kalan. Tamsayı
bölme işlemi sıfıra doğru en yakın tamsayıya kadar keser. Aşağıdaki kod
adresinde her bir sayısal işlemi bir `let` deyiminde nasıl kullanacağınızı gösterir:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Bu ifadelerdeki her ifade bir matematiksel işleç kullanır ve
adresini tek bir değere değerlendirir ve bu değer daha sonra bir değişkene bağlanır. [Ek
B][appendix_b]<!-- ignore --> Rust
'un sağladığı tüm operatörlerin bir listesini içerir.

#### Boolean Türü

Diğer programlama dillerinin çoğunda olduğu gibi, Rust'ta da bir Boolean tipinin iki olası
değeri vardır:`

 `true` ve `false`. Booleanlar bir bayt boyutundadır. Rust'ta Boolean türü `bool` kullanılarak belirtilir. Örneğin:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Boolean değerlerini kullanmanın ana yolu, `if`
ifadesi gibi koşullu ifadelerdir. Rust'ta `if' ifadelerinin nasıl çalıştığını [“Control Flow”][control-flow]<!-- ignore --> bölümünde ele alacağız.

#### Karakter Türü

Rust'ın `char` türü, dilin en ilkel alfabetik türüdür. İşte
`char` değerlerini bildirmek için bazı örnekler:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Çift tırnak kullanan string
literallerinin aksine `char` literallerini tek tırnak ile belirttiğimize dikkat edin. Rust'ın `char` tipi dört bayt boyutundadır ve
bir Unicode skaler değerini temsil eder, yani
sadece ASCII'den çok daha fazlasını temsil edebilir. Aksanlı harfler; Çince, Japonca ve Korece karakterler; emoji;
ve sıfır genişlikli boşlukların tümü Rust'ta geçerli `char` değerleridir. Unicode skaler
değerleri `U+0000` ile `U+D7FF` ve `U+E000` ile `U+10FFFF` arasında değişir.
Ancak, “karakter” Unicode'da gerçek bir kavram değildir, bu nedenle “karakter ”in ne olduğuna dair insan
sezginiz
Rust'ta “karakter ”in ne olduğuyla eşleşmeyebilir. Bu konuyu Bölüm 8'de [“UTF-8 Kodlu Metni
Dizeleri ile Saklama”][strings]<!-- ignore --> bölümünde ayrıntılı olarak ele alacağız.

### Bileşik Tipler

Bileşik tipler_ birden fazla değeri tek bir tipte gruplayabilir. Rust'ın iki
ilkel bileşik türü vardır: tuple'lar ve diziler.

#### Tuple Türü

Bir _tuple_
çeşitli türlere sahip bir dizi değeri tek bir bileşik türde bir araya getirmenin genel bir yoludur. Tuple'ların sabit bir uzunluğu vardır: bir kez
bildirildikten sonra, boyutları büyüyemez veya küçülemez.

Virgülle ayrılmış bir değer listesini
parantezleri içine yazarak bir tuple oluştururuz. Tuple'daki her konumun bir türü vardır ve tuple'daki
farklı değerlerin türlerinin aynı olması gerekmez. Bu örnekte isteğe bağlı
tür ek açıklamalarını ekledik:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

Bir tuple
tek bir bileşik öğe olarak kabul edildiğinden `tup` değişkeni tüm tuple'a bağlanır. Bir tuple'dan tek tek değerleri almak için,
adresinde bir tuple değerini yıkmak için desen eşleştirmeyi kullanabiliriz:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Bu program önce bir tuple oluşturur ve bunu `tup` değişkenine bağlar. Daha sonra
, `tup` değişkenini alıp `x`, `y` ve `z` olmak üzere üç ayrı
değişkenine dönüştürmek için `let` ile bir kalıp kullanır. Buna _destructuring_ denir çünkü
tek tuple'ı üç parçaya böler. Son olarak, program
`y` değerini, yani `6.4` değerini yazdırır.

Ayrıca bir tuple elemanına doğrudan erişmek için bir nokta (`.`) ve ardından erişmek istediğimiz değerin indeksini
kullanabiliriz. Örneğin:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Bu program `x` tuple'ını oluşturur ve ardından
tuple`ının her bir elemanına kendi indislerini kullanarak erişir. Çoğu programlama dilinde olduğu gibi, bir tuple içindeki ilk
indeksi 0'dır.

Herhangi bir değeri olmayan tuple özel bir isme sahiptir, _unit_. Bu değer ve
karşılık gelen türünün her ikisi de `()` olarak yazılır ve boş bir değeri veya
boş bir dönüş türünü temsil eder. İfadeler
başka bir değer döndürmezlerse dolaylı olarak birim değeri döndürürler.

#### Dizi Türü

Birden fazla değerden oluşan bir koleksiyona sahip olmanın bir başka yolu da _array_ kullanmaktır. Bir tuple'ın
aksine, bir dizinin her elemanı aynı tipte olmalıdır. diğer bazı dillerdeki dizilerin aksine, Rust'taki dizilerin sabit bir uzunluğu vardır.

Bir dizideki değerleri virgülle ayrılmış bir liste olarak kare
parantezler içinde yazarız:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Diziler, verilerinizin yığın yerine
şimdiye kadar gördüğümüz diğer türlerde olduğu gibi yığın üzerinde ayrılmasını istediğinizde (
yığın ve yığın konusunu [Bölüm 4][stack-and-heap]<!-- ignore --> bölümünde daha ayrıntılı olarak ele alacağız) veya
her zaman sabit sayıda öğeye sahip olduğunuzdan emin olmak istediğinizde kullanışlıdır. Yine de bir dizi
vektör tipi kadar esnek değildir. Bir _vector_ standart kütüphane tarafından sağlanan
benzer bir koleksiyon türüdür ve içeriği heap üzerinde bulunduğu için
boyutunun büyümesine veya küçülmesine izin verilir. Bir dizi mi
yoksa bir vektör mü kullanmanız gerektiğinden emin değilseniz, büyük olasılıkla bir vektör kullanmalısınız. [Bölüm 8][vektörler]<!--
ignore --> vektörleri daha ayrıntılı olarak ele almaktadır.

Ancak, eleman sayısının değişmeyeceğini
değişmesi gerekmeyeceğini bildiğiniz durumlarda diziler daha kullanışlıdır. Örneğin, bir
programında ay adlarını kullanıyorsanız, muhtemelen bir vektör yerine bir dizi kullanırsınız çünkü
her zaman 12 eleman içereceğini bilirsiniz:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Bir dizinin türünü, köşeli parantez içinde her bir öğenin türünü,
noktalı virgül ve ardından dizideki öğe sayısını kullanarak yazarsınız:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Burada, `i32` her bir elemanın türüdür. Noktalı virgülden sonra gelen `5` sayısı
dizinin beş eleman içerdiğini gösterir.

Bir diziyi, burada gösterildiği gibi
başlangıç değerini ardından noktalı virgül ve ardından köşeli parantez içinde dizinin uzunluğunu
belirterek her bir öğe için aynı değeri içerecek şekilde de başlatabilirsiniz:

```rust
let a = [3; 5];
```

`a` adlı dizi, başlangıçta hepsi
`3` değerine ayarlanacak `5` eleman içerecektir. Bu, `let a = [3, 3, 3, 3];` yazmakla aynıdır, ancak
daha kısa bir şekilde.

##### Dizi Elemanlarına Erişim

Bir dizi, yığın üzerinde
ayrılabilen, bilinen, sabit boyutta tek bir bellek parçasıdır. Bir dizinin elemanlarına indeksleme kullanarak erişebilirsiniz,
bu şekilde:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

Bu örnekte, `first` adlı değişken `1` değerini alacaktır çünkü
dizideki `[0]` indeksindeki değerdir. `second` adlı değişken
adresinden dizideki `[1]` dizininden `2` değerini alacaktır.

##### Geçersiz Dizi Elemanı Erişimi

Bir dizinin
dizinin sonunu geçmiş bir elemanına erişmeye çalışırsanız ne olacağını görelim. Kullanıcıdan bir dizi indeksi almak için
Bölüm 2'deki tahmin oyununa benzer şekilde bu kodu çalıştırdığınızı varsayalım:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Bu kod başarıyla derlenir. Bu kodu `cargo run` kullanarak çalıştırırsanız ve
adresine `0`, `1`, `2`, `3` veya `4` girerseniz, program dizideki o dizine karşılık gelen
değerini yazdıracaktır. Bunun yerine dizinin
sonundan sonra `10` gibi bir sayı girerseniz, aşağıdaki gibi bir çıktı görürsünüz:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Program, indeksleme işleminde geçersiz bir
değerinin kullanıldığı noktada bir _runtime_ hatasıyla sonuçlandı. Program bir hata mesajı ile çıktı ve
son `println!` deyimini çalıştırmadı. İndeksleme kullanarak bir
öğesine erişmeye çalıştığınızda, Rust belirttiğiniz indeksin
dizi uzunluğundan daha az olup olmadığını kontrol eder. Eğer indeks uzunluktan büyük veya eşitse,
Rust panikleyecektir. Bu kontrol çalışma zamanında yapılmalıdır, özellikle bu durumda,
çünkü derleyici kullanıcının
kodu daha sonra çalıştırdığında hangi değeri gireceğini bilemez.

Bu, Rust'ın bellek güvenliği ilkelerinin iş başında olduğu bir örnektir. Birçok
düşük seviyeli dilde, bu tür bir kontrol yapılmaz ve
yanlış bir dizin sağladığınızda, geçersiz belleğe erişilebilir. Rust, bellek erişimine izin vermek yerine hemen çıkarak ve
devam ederek sizi bu
tür hatalara karşı korur. Bölüm 9'da Rust'ın hata işleme özelliği ve
adresinde nasıl panik yapmayan veya geçersiz bellek erişimine izin vermeyen okunabilir, güvenli kod yazabileceğiniz anlatılmaktadır.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.md#Tahmini-Gizli-Sayı-ile-Karşılaştırma
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.md#Kontrol-Akışı
[strings]: ch08-02-strings.md#UTF-8-Kodlu-Metni-Dizelerle-Saklama
[stack-and-heap]: ch04-01-what-is-ownership.md#Yığın-ve-Yığın
[vectors]: ch08-01-vectors.md
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.md
[appendix_b]: appendix-02-operators.md
