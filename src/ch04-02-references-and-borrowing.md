## Referanslar ve Borçlanma

Listing 4-5'teki tuple koduyla ilgili sorun şu ki
`String`i çağıran fonksiyona aktarırız, böylece `String`i
'a yapılan `calculate_length` çağrısı, çünkü `String`
calculate_length`. Bunun yerine, `String` değerine bir referans sağlayabiliriz.
Bir _referans_, erişmek için takip edebileceğimiz bir adres olması açısından bir işaretçi gibidir
Bu adreste depolanan veri; bu verinin sahibi başka bir değişkendir.
Bir işaretçinin aksine, bir referansın geçerli bir değere işaret etmesi garanti edilir.
o referansın ömrü boyunca belirli bir tür.

Bir `calculate_length` fonksiyonunu nasıl tanımlayacağınız ve kullanacağınız aşağıda açıklanmıştır
değerin sahipliğini almak yerine bir nesneye parametre olarak başvurur:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

</Listing>

İlk olarak, değişken bildirimindeki tüm tuple kodunun ve
fonksiyonunun dönüş değeri kaybolur. İkinci olarak, `&s1` ifadesini
`calculate_length` ve tanımında, `&String` yerine `&String` alıyoruz
String`. Bu ve işaretleri _referansları_ temsil eder ve referans vermenizi sağlar
sahipliğini almadan bir değere dönüştürür. Şekil 4-6 bu kavramı tasvir etmektedir.

<img alt="Three tables: the table for s contains only a pointer to the table
for s1. The table for s1 contains the stack data for s1 and points to the
string data on the heap." src="img/trpl04-06.svg" class="center" />

<span class="caption">Şekil 4-6: `String'e işaret eden `&String s` diyagramı
s1`</span>

> Not: `&` kullanarak referans vermenin tersi _dereferanslama_dır, yani
> dereferans operatörü `*` ile gerçekleştirilir. Bazı kullanımlarını göreceğiz
> dereferans operatörünü Bölüm 8'de inceleyecek ve dereferanslamanın ayrıntılarını
> Bölüm 15.

Buradaki fonksiyon çağrısına daha yakından bakalım:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

& s1` sözdizimi, `s1` değerine _refere_ eden bir referans oluşturmamızı sağlar
ancak ona sahip değildir. Referans ona sahip olmadığı için, işaret ettiği değer
referansın kullanımı sona erdiğinde düşmeyecektir.

Aynı şekilde, işlevin imzasında da `&` kullanılarak işlevin türünün
parametresi `s` bir referanstır. Bazı açıklayıcı ek açıklamalar ekleyelim:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

` s` değişkeninin geçerli olduğu kapsam, herhangi bir işlevle aynıdır
parametresinin kapsamına dahil edilir, ancak referansın işaret ettiği değer düşürülmez
` s` kullanılmayı bıraktığında, çünkü `s` sahipliğe sahip değildir. Fonksiyonlar ne zaman
gerçek değerler yerine referansları parametre olarak alırsak
mülkiyeti geri vermek için değerleri iade edin, çünkü hiçbir zaman
sahiplik.

Bir referans oluşturma eylemine _borrowing_ diyoruz. Gerçek hayatta olduğu gibi, eğer bir
Bir kişi bir şeye sahipse, ondan ödünç alabilirsiniz. İşiniz bittiğinde
geri vermek için. Ona sahip değilsiniz.

Peki, ödünç aldığımız bir şeyi değiştirmeye çalışırsak ne olur? İçindeki kodu deneyin
Liste 4-6. Spoiler uyarısı: çalışmıyor!

<Listing number="4-6" file-name="src/main.rs" caption="Attempting to modify a borrowed value">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

</Listing>

İşte hata:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

Değişkenlerin varsayılan olarak değişmez olması gibi, referanslar da değişmezdir. Biz değiliz
referansımız olan bir şeyi değiştirmemize izin verilir.

### Değiştirilebilir Referanslar

Ödünç alınan bir değeri değiştirmemize izin vermek için Listing 4-6'daki kodu düzeltebiliriz
bunun yerine _mutable reference_ kullanan birkaç küçük değişiklikle:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

</Listing>

Önce `s` yi `mut` olarak değiştiriyoruz. Daha sonra `&mut` ile değiştirilebilir bir referans oluştururuz
`s` fonksiyonunu çağırıyoruz ve fonksiyon imzasını şu şekilde güncelliyoruz
değişebilir bir referansı `some_string: &mut String` ile kabul eder. Bu onu çok yapar
'change' fonksiyonunun ödünç aldığı değeri mutasyona uğratacağı açıktır.

Mutable referansların büyük bir kısıtlaması vardır: eğer mutable bir referansınız varsa
bir değere sahipseniz, bu değere başka hiçbir referansınız olamaz. Bu kod
'a iki değiştirilebilir referans oluşturma girişimleri başarısız olacaktır:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

</Listing>

İşte hata:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

Bu hata bu kodun geçersiz olduğunu söylüyor çünkü `s` yi
bir seferde birden fazla değiştirilebilir. İlk değiştirilebilir ödünç `r1` içindedir ve
'de kullanılana kadar sürer, ancak bunun oluşturulması arasında
değiştirilebilir referans ve kullanımı, başka bir değiştirilebilir referans oluşturmaya çalıştık
`r1` ile aynı veriyi ödünç alan `r2` içinde.

Aynı veriye birden fazla değiştirilebilir referansı engelleyen kısıtlama
Aynı zamanda mutasyona da izin verir ama çok kontrollü bir şekilde. Bu bir şey
yeni Rustace'cilerin zorlandığı bir konu çünkü çoğu dil mutasyona izin veriyor
ne zaman isterseniz. Bu kısıtlamaya sahip olmanın yararı, Rust'ın
derleme zamanında veri yarışlarını önler. Bir _veri yarışı_ bir yarışa benzer
koşuludur ve bu üç davranış gerçekleştiğinde meydana gelir:

- İki veya daha fazla işaretçi aynı veriye aynı anda erişiyor.
- İşaretçilerden en az biri veriye yazmak için kullanılıyor.
- Verilere erişimi senkronize etmek için kullanılan bir mekanizma yoktur.

Veri yarışları tanımlanmamış davranışlara neden olur ve teşhis edilmesi ve düzeltilmesi zor olabilir
Rust, çalışma zamanında onları bulmaya çalıştığınızda bu sorunu şu şekilde önler
veri yarışları ile kod derlemeyi reddetme!

Her zaman olduğu gibi, yeni bir kapsam oluşturmak için küme parantezlerini kullanabiliriz, böylece
çoklu değiştirilebilir referanslar, sadece _eşzamanlı_ olanlar değil:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Rust, mutable ve immutable referansları birleştirmek için benzer bir kural uygular.
Bu kod bir hata ile sonuçlanır:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

İşte hata:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

Vay canına! Ayrıca, değişmez bir referansımız varken değişebilir bir referansa sahip olamayız
aynı değere.

Değişmez bir referansı kullananlar, değerin aniden değişmesini beklemezler
onların altından! Ancak, birden fazla değişmez referansa izin verilir çünkü
Sadece verileri okuyan bir kişi, başkalarının verilerini etkileme yeteneğine sahiptir.
verilerin okunması.

Bir referansın kapsamının tanıtıldığı yerden başladığını ve devam ettiğini unutmayın
referansın son kullanıldığı zamana kadar. Örneğin, bu kod
derlenemez çünkü değişmez referansların son kullanımı `println!
değiştirilebilir referans tanıtılmadan önce:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

Değişmez referanslar `r1` ve `r2`nin kapsamları en son kullanıldıkları `println!`
son kullanıldıkları yerde, yani `r3` değişken referansından önce
oluşturuldu. Bu kapsamlar çakışmaz, bu nedenle bu koda izin verilir: derleyici
'nin sonundan önceki bir noktada referansın artık kullanılmadığını söyler.
kapsamı.

Ödünç alma hataları zaman zaman sinir bozucu olsa da, unutmayın ki
Rust derleyicisinin potansiyel bir hatayı erkenden işaret etmesi (derleme zamanında değil
çalışma zamanından daha fazla) ve size sorunun tam olarak nerede olduğunu gösterir. O zaman yapmazsın
verilerinizin neden düşündüğünüz gibi olmadığını bulmak zorundasınız.

### Sarkan Referanslar

İşaretçi içeren dillerde, yanlışlıkla bir _dangling
pointer_-bellekte daha önce bulunmuş olabilecek bir konuma referans veren bir pointer
bir işaretçiyi korurken bir miktar belleği serbest bırakarak başka birine verilir.
bellek. Buna karşın Rust'ta derleyici, referansların
asla sarkan referanslar olmamalıdır: eğer bir veriye referansınız varsa
derleyici, verinin kapsam dışına çıkmamasını sağlayacaktır.
referansının yaptığı gibi.

Rust'ın bunları nasıl önlediğini görmek için sarkan bir referans oluşturmayı deneyelim
derleme zamanı hatası:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

</Listing>

İşte hata:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

Bu hata mesajı henüz ele almadığımız bir özelliğe atıfta bulunuyor: yaşam süreleri. Biz yapacağız
yaşam sürelerini Bölüm 10'da ayrıntılı olarak ele alacağız. Ancak, bu bölümleri göz ardı ederseniz
ömürleri hakkında, mesaj bu kodun neden bir sorun olduğunun anahtarını içeriyor:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

Her bir aşamamızda tam olarak neler olduğuna daha yakından bakalım
`dangle` kodu:
<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

</Listing>

Çünkü `s`, `dangle` kodu tamamlandığında `dangle` içinde oluşturulur,
`s` ayrılmış olacak. Ama biz ona bir referans döndürmeye çalıştık. Bunun anlamı
bu referans geçersiz bir `String`e işaret ediyor olacaktır. Bu hiç iyi değil! Pas
bunu yapmamıza izin vermez.

Buradaki çözüm `String`i doğrudan döndürmektir:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

Bu sorunsuz bir şekilde işler. Mülkiyet taşındı ve hiçbir şey
deallocated.

### Referans Kuralları

Referanslar hakkında konuştuklarımızı özetleyelim:

- Herhangi bir zamanda, bir _değişebilir_ referansa _veya_ herhangi bir
  değişmez referans sayısı.
- Referanslar her zaman geçerli olmalıdır.

Daha sonra, farklı bir referans türüne bakacağız: dilimler.