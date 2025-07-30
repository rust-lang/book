## Referansları Yaşam Süreleri ile Doğrulama

Yaşam süreleri, zaten kullanmakta olduğumuz bir başka genel türdür. Bir türün istediğimiz davranışa sahip olmasını sağlamaktan ziyade
yaşam süreleri
referansların ihtiyaç duyduğumuz sürece geçerli olmasını sağlar.

Bölüm 4'teki [“Referanslar ve Ödünç Alma”][references-and-borrowing]<!-- ignore --> bölümünde tartışmadığımız bir ayrıntı
Rust'taki her referansın bir _lifetime_'a sahip olduğu ve bu referansın
geçerli olduğu kapsamdır. Çoğu zaman yaşam süreleri örtük ve çıkarımsaldır
tıpkı çoğu zaman tiplerin çıkarımsal olduğu gibi. Yalnızca birden fazla tür mümkün olduğunda
türlere açıklama eklememiz gerekir. Benzer bir şekilde, referansların yaşam süreleri birkaç
farklı şekilde ilişkili olabildiğinde
yaşam sürelerine açıklama eklememiz gerekir. Rust, çalışma zamanında kullanılan gerçek referansların
kesinlikle geçerli olmasını sağlamak için genel
yaşam süresi parametrelerini kullanarak ilişkilere açıklama eklememizi gerektirir.

Yaşam sürelerine açıklama eklemek diğer programlama dillerinin çoğunda
bulunan bir kavram bile değildir, bu nedenle bu durum size yabancı gelecektir. Bu bölümde yaşam sürelerini
bütünüyle ele almayacak olsak da, kavrama alışabilmeniz için
yaşam süresi sözdizimiyle karşılaşabileceğiniz yaygın yolları tartışacağız.

### Yaşam Süreleri ile Sarkan Referansları Önleme

Yaşam sürelerinin temel amacı, bir
programının başvurması gereken verilerden başka verilere başvurmasına neden olan _dangling references_'ı önlemektir.
Bir dış kapsamı ve bir iç
kapsamı olan Liste 10-16'daki programı düşünün.

<Listing number="10-16" caption="An attempt to use a reference whose value has gone out of scope">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

</Listing>

> Not: Liste 10-16, 10-17 ve 10-23'teki örnekler
> değişkenlerini onlara bir başlangıç değeri vermeden bildirir, böylece değişken adı dış
> kapsamında bulunur. İlk bakışta, bu durum Rust'ın
> null değerlere sahip olmaması ile çelişiyor gibi görünebilir. Ancak, bir değişkene değer vermeden önce kullanmaya çalışırsak,
> derleme zamanı hatası alırız, bu da Rust'ın gerçekten de
> null değerlere izin vermediğini gösterir.

Dış kapsam, başlangıç değeri olmayan `r` adında bir değişken bildirir ve
iç kapsam, başlangıç değeri `5` olan `x` adında bir değişken bildirir. İç kapsam
içinde, `r` değerini `x` değerine referans olarak ayarlamaya çalışırız. Ardından
iç kapsam sona erer ve `r` içindeki değeri yazdırmaya çalışırız. Bu kod
derlenmez, çünkü `r` değerinin başvurduğu değer
biz onu kullanmaya çalışmadan önce kapsam dışına çıkmıştır. İşte hata mesajı:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

Hata mesajı `x` değişkeninin “yeterince uzun yaşamadığını” söylüyor. Bunun
nedeni, iç kapsam 7. satırda sona erdiğinde `x` değişkeninin kapsam dışında kalacak olmasıdır.
Ancak `r` dış kapsam için hala geçerlidir; kapsamı daha geniş olduğu için
“daha uzun süre yaşadığını” söylüyoruz. Eğer Rust bu kodun çalışmasına izin verseydi, `r`
`x` kapsam dışına çıktığında deallocate edilen belleğe referans veriyor olacaktı ve
`r` ile yapmaya çalıştığımız hiçbir şey doğru çalışmayacaktı. Peki Rust
bu kodun geçersiz olduğunu nasıl belirler? Bir borç denetleyicisi kullanır.

### Ödünç Denetleyicisi

Rust derleyicisi
tüm borçlanmaların geçerli olup olmadığını belirlemek için kapsamları karşılaştıran bir _borrow checker_'a sahiptir. Liste 10-17, Liste
10-16 ile aynı kodu, ancak değişkenlerin yaşam sürelerini gösteren ek açıklamalarla birlikte gösterir.

<Listing number="10-17" caption="Annotations of the lifetimes of `r` and `x`, named `'a` and `'b`, respectively">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

</Listing>

Burada, `r`'nin yaşam süresini `'a` ve `x`'in yaşam süresini `'b` ile belirttik. Gördüğünüz gibi, içteki `'b` bloğu dıştaki `'a` yaşam süresi bloğundan çok daha küçük. Derleme zamanında Rust iki yaşam süresinin boyutunu karşılaştırır ve `r`'nin `'a` yaşam süresine sahip olduğunu, ancak `'b` yaşam süresine sahip bir belleği referans aldığını görür. Program reddedilir çünkü `'b`, `'a`'dan daha kısadır: referansın gösterdiği nesne, referansın kendisi kadar uzun yaşamaz.

Liste 10-18, kodu asılı referans içermeyecek şekilde düzelterek hatasız derlenmesini sağlıyor.

<Listing number="10-18" caption="A valid reference because the data has a longer lifetime than the reference">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

</Listing>

Burada, `x`'in ömrü `'b`'dir ve bu durumda `'a`'dan daha uzundur. Bu,
`r`'nin `x`'e referans verebileceği anlamına gelir, çünkü Rust, `x` geçerli olduğu sürece
`r`'deki referansın her zaman geçerli olacağını bilir.

Artık referansların ömürlerinin nerede olduğunu ve Rust'un referansların her zaman geçerli olmasını sağlamak için ömürleri nasıl analiz ettiğini bildiğinize göre, fonksiyonlar bağlamında parametrelerin ve dönüş değerlerinin genel ömürlerini inceleyelim.
### Fonksiyonlarda Genel Ömürler

İki string diliminin daha uzun olanını döndüren bir fonksiyon yazacağız. Bu

### İşlevlerde Genel Ömürler

İki dize diliminden daha uzun olanını döndüren bir işlev yazacağız. Bu
işlev iki dize dilimini alacak ve tek bir dize dilimi döndürecektir.
`longest` işlevini uyguladıktan sonra, Listing 10-19'daki kod
`The longest string is abcd` yazdırmalıdır.

<Listing number="10-19" file-name="src/main.rs" caption="A `main` function that calls the `longest` function to find the longer of two string slices">

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

</Listing>

İşlevin, dizgi yerine referans olan dizgi dilimlerini almasını istediğimizi unutmayın,
çünkü `longest` işlevinin parametrelerinin sahipliğini almasını istemiyoruz.
Listing 10-19'da kullandığımız parametrelerin neden istediğimiz parametreler olduğu hakkında daha fazla bilgi için Bölüm 4'teki [“Parametre Olarak Dize Dilimleri”][string-slices-as-parameters]<!-- ignore --> bölümüne bakın..

Listing 10-21'de gösterildiği gibi, `longest` işlevini parametreler olarak
dize dilimleri kullanacak şekilde yeniden yazabiliriz.

<Listing number="10-20" file-name="src/main.rs" caption="An implementation of the `longest` function that returns the longer of two string slices but does not yet compile">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

</Listing>

Bunun yerine, ömürlerle ilgili şu hata mesajını alıyoruz:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

Yardım metni, dönüş türünün genel bir ömür parametresine ihtiyaç duyduğunu ortaya koymaktadır.
Çünkü Rust, döndürülen referansın `x` mi yoksa `y` mi olduğunu ayırt edememektedir.
Aslında biz de bilmiyoruz, çünkü bu fonksiyonun gövdesindeki `if` bloğu
`x` referansını döndürürken, `else` bloğu `y` referansını döndürmektedir!
Bu fonksiyonu tanımlarken, bu fonksiyona aktarılacak somut değerleri bilmiyoruz.

Bu fonksiyonu tanımlarken, bu fonksiyona aktarılacak somut değerleri bilmiyoruz,
bu yüzden `if` durumunun mu yoksa `else` durumunun mu çalışacağını bilmiyoruz.
Ayrıca, aktarılacak referansların somut ömürlerini de bilmiyoruz, bu yüzden
Listing 10-17 ve 10-18'de yaptığımız gibi kapsamlara bakarak döndürdüğümüz referansın
her zaman geçerli olup olmayacağını belirleyemeyiz. Borç kontrolörü de bunu belirleyemez, çünkü
`x` ve `y`'nin ömürlerinin döndürülen değerin ömrüyle nasıl ilişkili olduğunu
bilmez. Bu hatayı düzeltmek için, aralarındaki ilişkiyi tanımlayan genel ömür parametreleri
ekleyeceğiz.een the references so the borrow checker can
perform its analysis.

### Ömür Boyu Anotasyon Sözdizimi

Ömür boyu anotasyonlar, referansların ömürlerini değiştirmez. Bunun yerine,
birden fazla referansın ömürleri arasındaki ilişkileri, ömürleri etkilemeden
tanımlar. İşlevler, imza genel bir tür parametresi belirlediğinde herhangi bir türü
kabul edebildiği gibi, genel bir ömür parametresi belirleyerek herhangi bir ömre sahip
referansları da kabul edebilir.

Ömür süresi açıklamalarının sözdizimi biraz sıra dışıdır: ömür süresi
parametrelerinin adları apostrof (`'`) ile başlamalıdır ve genellikle tümü küçük harflerle
yazılır ve genel türler gibi çok kısadır. Çoğu kişi ilk ömür süresi açıklaması için `'a`
adını kullanır. Ömür süresi parametresi açıklamalarını, açıklamayı referansın türünden ayırmak için
boşluk kullanarak referansın `&` işaretinden sonra yerleştiririz.

İşte bazı örnekler: ömür parametresi olmayan bir `i32` referansı,
`'a` adlı bir ömür parametresine sahip bir `i32` referansı ve yine `'a` ömrüne sahip bir `i32` referansı.
`'a` ömrüne sahip bir `i32` referansı.

```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning because the
annotations are meant to tell Rust how generic lifetime parameters of multiple
references relate to each other. Let’s examine how the lifetime annotations
relate to each other in the context of the `longest` function.

### İşlev İmzalarında Ömür Açıklamaları

İşlev imzalarında ömür açıklamaları kullanmak için, genel _lifetime_ parametrelerini, genel _type_ parametrelerinde yaptığımız gibi, işlev adı ile parametre listesi arasındaki köşeli parantezlerin içinde
beyan etmemiz gerekir.
İmzada, aşağıdaki kısıtlamayı ifade etmek istiyoruz: döndürülen

İmzanın aşağıdaki kısıtlamayı ifade etmesini istiyoruz: döndürülen
referans, her iki parametre de geçerli olduğu sürece geçerli olacaktır. Bu,
parametrelerin ömürleri ile döndürülen değer arasındaki ilişkidir. Ömrü `'a`
olarak adlandıracağız ve ardından Listing 10-21'de gösterildiği gibi her referansa
ekleyeceğiz.

<Listing number="10-21" file-name="src/main.rs" caption="The `longest` function definition specifying that all the references in the signature must have the same lifetime `'a`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

</Listing>

Bu kod, Listing 10-19'daki
`main` işleviyle birlikte kullandığımızda derlenmeli ve istediğimiz sonucu vermeli.

İşlev imzası artık Rust'a, bazı ömürler `'a` için işlevin
iki parametre aldığını ve her ikisinin de en az ömür `'a` kadar
uzun ömürlü string dilimleri olduğunu söyler. İşlev imzası ayrıca Rust'a, işlevden döndürülen string
diliminin en az ömür `'a` kadar uzun ömürlü olacağını söyler.
Pratikte bu, `longest` fonksiyonu tarafından döndürülen referansın ömrünün,
fonksiyon argümanları tarafından referanslanan değerlerin ömürlerinden daha kısa olanı
ile aynı olduğu anlamına gelir. Bu ilişkiler, Rust'un bu kodu analiz ederken
kullanmasını istediğimiz şeydir.

Unutmayın, bu fonksiyon imzasında ömür parametrelerini belirlediğimizde,
geçirilen veya döndürülen değerlerin ömürlerini değiştirmiyoruz. Aksine,
borç kontrolcüsünün bu kısıtlamalara uymayan değerleri reddetmesi gerektiğini
belirtmiş oluyoruz. `longest` işlevinin `x` ve `y`'nin tam olarak ne kadar süreyle
yaşayacağını bilmesi gerekmediğini, sadece bu imzayı karşılayacak bir kapsamın
`'a` yerine geçebileceğini unutmayın.

İşlevlerde ömürleri açıklama eklerken, açıklamalar işlev gövdesine değil, işlev
imzasına eklenir. Ömür açıklamaları, imza içindeki türler gibi, işlevin
sözleşmesinin bir parçası olur. İşlev imzalarının ömür sözleşmesini içermesi,
Rust derleyicisinin yaptığı analizin daha basit olabileceği anlamına gelir.
İşlevlerin Bir fonksiyonun açıklama şekli veya çağrılma şekliyle ilgili bir sorun varsa,
derleyici hataları kodumuzun ve kısıtlamaların daha kesin olarak hangi kısmına işaret
edebilir. Bunun yerine, Rust derleyicisi ömürlerin ilişkilerinin ne olması
istendiğine dair daha fazla çıkarımda bulunursa, derleyici sorunun nedeninden
çok uzak olan kodumuzun kullanımına işaret edebilir.

`longest`'a somut referanslar aktardığımızda, `'a`'nın yerine geçen somut ömür,
`x`'in kapsamının `y`'nin kapsamıyla çakışan kısmıdır.
Diğer bir deyişle, genel ömür `'a`, `x` ve `y`'nin ömürlerinden daha küçük olanına eşit olan somut ömrü alır. Geri dönen referansı aynı ömür parametresi `'a` ile
açıklamış olduğumuzdan, geri dönen referans da `x` ve `y` ömürlerinden
daha küçük olanının süresi boyunca geçerli olacaktır.

Ömür açıklamalarının, farklı somut ömürlere sahip referanslar geçirerek
`longest` işlevini nasıl kısıtladığını inceleyelim. Listing 10-22,
basit bir örnektir.

<Listing number="10-22" file-name="src/main.rs" caption="Using the `longest` function with references to `String` values that have different concrete lifetimes">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

</Listing>

Bu örnekte, `string1` dış kapsamın sonuna kadar geçerlidir, `string2`
iç kapsamın sonuna kadar geçerlidir ve `result` iç kapsamın sonuna kadar geçerli olan bir şeyi
referans alır. Bu kodu çalıştırdığınızda,
borç kontrolcüsünün onayladığını göreceksiniz; derlenecek ve `En uzun dize
uzun dizedir uzun` yazdırılacaktır.

Şimdi, `result` içindeki referansın ömrünün iki argümandan daha kısa olması gerektiğini
gösteren bir örnek deneyelim. `result` değişkeninin
bildirimini iç kapsamın dışına taşıyacağız, ancak `result` değişkenine değer atamayı
`string2` ile kapsamın içinde bırakacağız. Ardından, `result` kullanan `println!` ifadesini iç
kapsamın dışına, iç kapsamın sonrasına taşıyacağız. Listing 10-23'teki kod
derlenmeyecektir.

<Listing number="10-23" file-name="src/main.rs" caption="Attempting to use `result` after `string2` has gone out of scope">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

</Listing>

Bu kodu derlemeye çalıştığımızda şu hata mesajını alıyoruz:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

Hata, `println!` ifadesinde `result`'un geçerli olması için,
`string2`'nin dış kapsamın sonuna kadar geçerli olması gerektiğini gösteriyor. Rust bunu biliyor
çünkü fonksiyon parametrelerinin ve dönüş değerlerinin ömürlerini aynı ömür parametresi `'a` kullanarak
açıklamışız.

İnsanlar olarak, bu koda bakıp `string1`'in `string2`'den daha uzun olduğunu ve
bu nedenle `result`'un `string1`'e bir referans içereceğini görebiliriz.
`string1` henüz kapsam dışına çıkmadığı için, `string1`'e bir referans
`println!` ifadesi için hala geçerli olacaktır. Ancak, derleyici bu durumda
referansın geçerli olduğunu göremez. Rust'a, `longest` işlevinin döndürdüğü referansın ömrünün,
geçirilen referansların ömürlerinden daha kısa olanıyla aynı olduğunu söyledik.
Bu nedenle, ödünç alma denetleyicisi, Listing 10-23'teki kodun geçersiz bir referansa sahip olabileceği için
bu kodu reddeder.

`longest` işlevine aktarılan referansların değerlerini ve ömürlerini ve döndürülen referansın
nasıl kullanıldığını değiştiren daha fazla deney tasarlayın. Derlemeden önce deneylerinizin ödünç alma denetleyicisini geçip geçmeyeceğine dair
hipotezler oluşturun; ardından haklı olup olmadığınızı kontrol edin!

### Ömürler Açısından Düşünmek

Ömür parametrelerini belirtme şekliniz, fonksiyonunuzun ne yaptığına bağlıdır.
Örneğin,
`longest` fonksiyonunun uygulamasını, en uzun
dize dilimini değil, her zaman ilk parametreyi döndürecek şekilde değiştirirsek, `y` parametresinin ömrünü belirtmemiz gerekmez.
Aşağıdaki kod derlenecektir:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

</Listing>

Ömür parametrelerini belirtme şekli Parametre `x` ve dönüş türü için ömür parametresi `'a` belirledik, ancak
parametre `y` için belirlemedik, çünkü `y`'nin ömrü `x`'in ömrü veya
dönüş değeriyle hiçbir ilişkisi yoktur.

Bir işlevden referans döndürürken, dönüş türünün ömür parametresi, parametrelerden birinin ömür parametresiyle eşleşmelidir.
Döndürülen referans parametrelerden birine _atıfta bulunmuyorsa_, bu işlev içinde oluşturulan bir değere atıfta bulunmalıdır.
Ancak, bu değer işlevin sonunda kapsam dışı kalacağı için, bu bir sarkan referans olacaktır.
`longest` işlevinin derlenmeyen bu uygulama denemesini düşünün:
işlevinizin ne yaptığına bağlıdır.
Derlenemeyen `longest` işlevinin bu uygulama denemesini ele alalım:
işlevinizin ne yaptığına bağlıdır.
Örneğin, `longest` işlevinin uygulamasını en uzun
dize dilimi yerine her zaman ilk parametreyi döndürecek şekilde değiştirirsek,
`y` parametresi için bir ömür belirtmemiz gerekmez.
Aşağıdaki kod derlenecektir:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

</Listing>

Burada, dönüş türü için bir ömür parametresi `'a` belirtmiş olsak da,
bu uygulama derlenemeyecektir çünkü dönüş değerinin ömrü, parametrelerin ömrüyle
hiçbir şekilde ilişkili değildir. Aldığımız hata mesajı şöyledir:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

Sorun, `result`'un kapsam dışına çıkması ve `longest` işlevinin sonunda temizlenmesidir.
Ayrıca, işlevden `result`'a bir referans döndürmeye çalışıyoruz.
Sarkan referansı değiştirecek ömür parametreleri belirleyemeyiz ve Rust, sarkan
referans oluşturmamıza izin vermez.
Bu durumda, en iyi çözüm, referans yerine sahip olunan veri türünü döndürmektir. Bu durumda, en iyi çözüm referans yerine sahip olunan bir veri türü döndürmek
olur, böylece çağıran fonksiyon değeri temizlemekten
sorumlu olur.

Sonuç olarak, ömür sözdizimi çeşitli parametrelerin ömürlerini ve fonksiyonların
dönüş değerlerini birbirine bağlamakla ilgilidir. Bunlar birbirine bağlandığında, Rust
bellek güvenliği sağlayan işlemleri izin vermek ve sarkan işaretçiler oluşturacak veya
bellek güvenliğini ihlal edecek işlemleri engellemek için yeterli bilgiye sahip olur.

### Yapı Tanımlarında Ömür Açıklamaları

Şimdiye kadar tanımladığımız yapılar, sahip olunan türleri tutmaktadır. Yapıları,
referansları tutacak şekilde tanımlayabiliriz, ancak bu durumda yapının tanımındaki
her referansa bir ömür açıklaması eklememiz gerekir. Listing 10-24'te, bir string dilimini tutan
`ImportantExcerpt` adlı bir yapı bulunmaktadır.

<Listing number="10-24" file-name="src/main.rs" caption="A struct that holds a reference, requiring a lifetime annotation">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

</Listing>

Bu yapı, bir string dilimini tutan tek bir `part` alanı içerir ve bu bir
referanstır. Genel veri türlerinde olduğu gibi, genel ömür parametresinin adını
yapının adından sonra köşeli parantez içinde bildiririz, böylece yapı tanımının
gövdesinde ömür parametresini kullanabiliriz. Bu
açıklama, `ImportantExcerpt` örneğinin `part` alanında tuttuğu referanstan
daha uzun ömürlü olamayacağı anlamına gelir.

Buradaki `main` işlevi, `novel` değişkeninin sahip olduğu `String` nesnesinin
ilk cümlesine bir referans tutan `ImportantExcerpt` yapısının bir örneğini
oluşturur. `novel` içindeki veriler, `ImportantExcerpt` örneği oluşturulmadan
önce mevcuttur. Ayrıca, `novel`, `ImportantExcerpt` kapsam dışı kalana kadar
kapsam dışı kalmaz, bu nedenle `ImportantExcerpt` örneğindeki referans
geçerlidir.

### Ömür Boyu Elision

Her referansın bir ömrü olduğunu ve referans kullanan işlevler veya yapılar için ömür parametrelerini belirtmeniz gerektiğini öğrendiniz.
Ancak, Listing 4-9'da ve Listing 10-25'te tekrar gösterilen bir işlevimiz vardı ki, bu işlev ömür açıklamaları olmadan derlenmişti.

<Listing number="10-25" file-name="src/lib.rs" caption="A function we defined in Listing 4-9 that compiled without lifetime annotations, even though the parameter and return type are references">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

</Listing>

Bu fonksiyonun ömür süresi açıklamaları olmadan derlenmesinin nedeni tarihseldir:
Rust'un ilk sürümlerinde (1.0 öncesi), bu kod derlenemezdi çünkü
her referansın açık bir ömür süresi olması gerekiyordu. O zamanlar, fonksiyon
imzası şöyle yazılırdı:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Rust ekibi, çok sayıda Rust kodu yazdıktan sonra, Rust programcılarının
belirli durumlarda aynı ömür süresi açıklamalarını tekrar tekrar girdiklerini
fark etti. Bu durumlar öngörülebilirdi ve birkaç deterministik
modele uyuyordu. Geliştiriciler bu modelleri derleyicinin koduna programladılar, böylece
ödünç alma denetleyicisi bu durumlarda ömür sürelerini çıkarabilir ve
açık açıklamalara ihtiyaç duymazdı.

Rust tarihinin bu kısmı önemlidir, çünkü daha fazla
deterministik kalıp ortaya çıkıp derleyiciye eklenebilir. Gelecekte,
daha da az ömür süresi açıklaması gerekebilir.

Rust'un referans analizine programlanan kalıplara
_ömür elizyon kuralları_ denir. Bunlar, programcıların uyması gereken kurallar değildir; bunlar,
derleyicinin dikkate alacağı belirli durumlar kümesidir ve kodunuz
bu durumlara uyuyorsa, ömürleri açıkça yazmanıza gerek yoktur.

Elision kuralları tam bir çıkarım sağlamaz. Rust kuralları uyguladıktan sonra referansların ömürleri hakkında hala belirsizlik varsa,
derleyici kalan referansların ömürlerinin ne olması gerektiğini tahmin etmez.
Tahmin etmek yerine, derleyici size ömür açıklamaları ekleyerek çözebileceğiniz bir hata verir.
Bu, ömür açıklamalarının eksikliğini gidermek için bir uyarıdır.

İşlev veya yöntem parametrelerindeki ömürler _giriş ömürleri_ olarak adlandırılır ve
dönüş değerlerindeki ömürler _çıkış ömürleri_ olarak adlandırılır.

Derleyici, açık bir açıklama olmadığında referansların ömürlerini belirlemek için
üç kural kullanır. İlk kural giriş ömürleri için geçerlidir ve
ikinci ve üçüncü kurallar çıkış ömürleri için geçerlidir. Derleyici
üç kuralın sonuna geldiğinde ve hala ömürlerini belirleyemediği referanslar varsa,
derleyici bir hata ile durur.
Bu kurallar, `fn` tanımlarının yanı sıra `impl` blokları için de geçerlidir.

İlk kural, derleyicinin referans olan her parametreye bir ömür parametresi atamasıdır.
Diğer bir deyişle, tek parametreli bir fonksiyon Diğer bir deyişle, tek parametreli bir işlev
tek bir ömür parametresi alır: `fn foo<'a>(x: &'a i32)`; iki parametreli bir işlev
iki ayrı ömür parametresi alır: `fn foo<'a, 'b>(x: &'a i32,
y: &'b i32)`; ve benzeri.

İkinci kural, tam olarak bir giriş ömür parametresi varsa, bu
ömür tüm çıkış ömür parametrelerine atanır: `fn foo<'a>(x: &'a i32)
-> &'a i32`.

Üçüncü kural, birden fazla giriş ömrü parametresi varsa, ancak
bunlardan biri `&self` veya `&mut self` ise (çünkü bu bir yöntemdir),
`self` ömrü tüm çıkış ömrü parametrelerine atanır. Bu üçüncü kural,
daha az sembol gerektiğinden yöntemleri okumayı ve yazmayı çok daha kolay hale getirir.

Derleyici olduğumuzu varsayalım. Bu kuralları uygulayarak,
Listing 10-25'teki `first_word` işlevinin imzasında yer alan referansların
ömürlerini belirleyeceğiz. İmza, referanslarla ilişkili herhangi bir ömür olmadan başlar:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Then the compiler applies the first rule, which specifies that each parameter
gets its own lifetime. We’ll call it `'a` as usual, so now the signature is
this:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

İkinci kural, tam olarak bir giriş ömrü olduğu için geçerlidir. İkinci
kural, tek giriş parametresinin ömrünün çıkış ömrüne atanacağını belirtir,
bu nedenle imza artık şöyledir:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

Artık bu işlev imzasındaki tüm referansların ömürleri vardır ve
derleyici, programcının bu işlev imzasındaki ömürleri açıklamasına gerek kalmadan
analizine devam edebilir.

Başka bir örneğe bakalım, bu sefer Listing 10-20'de çalışmaya başladığımızda ömür parametresi olmayan
`longest` işlevini kullanarak:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

İlk kuralı uygulayalım: her parametre kendi ömrüne sahiptir. Bu sefer
bir yerine iki parametre var, bu yüzden iki ömür var:

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

İkinci kuralın geçerli olmadığını görebilirsiniz, çünkü birden fazla
giriş ömrü vardır. Üçüncü kural da geçerli değildir, çünkü `longest` bir
yöntem değil bir işlevdir, bu nedenle parametrelerin hiçbiri `self` değildir.
Üç kuralın hepsini inceledikten sonra, hala dönüş
türünün ömrünün ne olduğunu bulamadık. Bu nedenle,
Listing 10-20'deki kodu derlemeye çalışırken hata aldık: derleyici ömür elizyon kurallarını uyguladı, ancak yine de
imzadaki referansların tüm ömürlerini anlayamadı.

Üçüncü kural aslında sadece yöntem imzalarında geçerli olduğundan,
bu bağlamda ömürleri inceleyerek üçüncü kuralın neden yöntem imzalarında ömürleri
çok sık açıklamamız gerekmediğini göreceğiz.

### Yöntem Tanımlarında Ömür Açıklamaları

Ömürleri olan bir yapı üzerinde yöntemler uyguladığımızda, Listing 10-11'de gösterildiği gibi
genel tip parametreleriyle aynı sözdizimini kullanırız. Ömür parametrelerini nerede
bildirdiğimiz ve kullandığımız, bunların yapı alanlarıyla mı yoksa yöntem parametreleri ve
dönüş değerleriyle mi ilgili olduğuna bağlıdır.

Yapı alanları için ömür adları her zaman `impl` anahtar sözcüğünden sonra bildirilmelidir
anahtar kelimesinden sonra bildirilmeli ve ardından yapı adından sonra kullanılmalıdır, çünkü bu ömürler yapının türünün bir parçasıdır.


`impl` bloğu içindeki yöntem imzalarında, referanslar yapının alanlarındaki referansların ömrüyle bağlantılı olabilir veya bağımsız olabilir.

Ayrıca, ömür elizyon kuralları genellikle yöntem imzalarında ömür açıklamalarının gerekli olmamasını sağlar. Listing 10-24'te tanımladığımız
`ImportantExcerpt` adlı yapıyı kullanan bazı örneklere bakalım.

İlk olarak, tek parametresi `self`'e bir referans olan ve dönüş değeri hiçbir şeye referans olmayan bir `i32` olan `level` adlı bir yöntem kullanacağız:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

`impl`'den sonra ömür parametresi bildirimi ve tür adından sonra kullanımı
gerekli olmakla birlikte, ilk elizyon kuralı nedeniyle `self` referansının ömrünü
belirtmemiz gerekmez.

İşte üçüncü ömür elizyon kuralının uygulandığı bir örnek:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

İki giriş ömrü vardır, bu nedenle Rust ilk ömür elizyon kuralını uygular
ve hem `&self` hem de `announcement` için kendi ömürlerini verir. Ardından,
parametrelerden biri `&self` olduğu için, dönüş türü `&self` ömrünü alır
ve tüm ömürler hesaba katılmış olur.

### Statik Ömür

Tartışmamız gereken özel bir ömür olan `'static`, etkilenen referansın programın tüm süresi boyunca yaşayabileceğini belirtir.
Tüm
dize sabitleri `'static` ömre sahiptir ve bunu şu şekilde açıklayabiliriz:

```rust
let s: &'static str = "I have a static lifetime.";
```

Bu dizgenin metni, programın ikili dosyasında doğrudan saklanır ve
her zaman kullanılabilir durumdadır. Bu nedenle, tüm dizge sabitlerinin ömrü `'static`'tir.

Hata mesajlarında `'static` ömrünü kullanmanız için öneriler görebilirsiniz. Ancak
bir referans için ömür olarak `'static`'i belirtmeden önce, sahip olduğunuz referansın
programınızın tüm ömrü boyunca gerçekten var olup olmadığını ve bunu isteyip istemediğinizi
düşünün. Çoğu zaman, `'static` ömrü öneren bir hata mesajı,
sarkan bir referans oluşturmaya çalışmaktan veya kullanılabilir ömürlerin uyuşmamasından
kaynaklanır. Bu gibi durumlarda çözüm,
`'static` ömrünü belirtmek değil, bu sorunları gidermektir.

## Genel Tip Parametreleri, Özellik Sınırları ve Ömürlerin Bir Arada Kullanımı

Genel tip parametrelerini, özellik sınırlarını ve ömürleri tek bir işlevde belirtmenin
sözdizimini kısaca inceleyelim!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Bu, Listing 10-21'deki `longest` işlevidir ve iki dizgi diliminden daha uzun olanını döndürür.
Ancak şimdi, `where` yan tümcesinde belirtildiği gibi `Display` özelliğini uygulayan herhangi bir türle doldurulabilen
genel tür `T` adlı `ann` adlı ek bir parametreye sahiptir.
Bu ek parametre, `{}` kullanılarak yazdırılacaktır, bu nedenle `Display` özellik sınırlaması gereklidir. Bu ekstra parametre, `{}` kullanılarak yazdırılacaktır,
bu nedenle `Display` özelliği sınırı gereklidir. Ömürler bir tür genel olduğundan, ömür parametresi
`'a` ve genel tür parametresi `T`'nin bildirimleri, işlev adından sonra köşeli parantezlerin içindeki aynı listeye
gider.

## Özet

Bu bölümde çok şey öğrendik! Artık genel tip
parametreleri, özellikler ve özellik sınırları ile genel ömür parametreleri hakkında bilgi sahibi olduğunuza göre,
birçok farklı durumda çalışan, tekrarlamadan kod yazmaya hazırsınız.
Genel tip parametreleri, kodu farklı tiplere uygulamanızı sağlar. Özellikler ve
özellik sınırları, tipler genel olsa bile kodun ihtiyaç duyduğu davranışı sergileyeceklerini
garanti eder. Bu esnek kodun herhangi bir sarkan referansa sahip olmamasını sağlamak için
ömür süresi açıklamalarını nasıl kullanacağınızı öğrendiniz. Ve tüm bu
analizler derleme sırasında gerçekleşir, bu da çalışma zamanı performansını etkilemez!

İster inanın ister inanmayın, bu bölümde ele aldığımız konularda öğrenecek çok daha fazla şey var:
18. bölüm, özellikleri kullanmanın başka bir yolu olan özellik nesnelerini ele alıyor.
Özellikler. Ayrıca, yalnızca çok gelişmiş senaryolarda ihtiyaç duyacağınız ömür boyu açıklamaları içeren daha karmaşık senaryolar da vardır; bunlar için
[Rust Referansı][reference]'ı okumalısınız. Ancak şimdi, kodunuzun olması gerektiği gibi çalıştığından emin olmak için Rust'ta testler yazmayı öğreneceksiniz.

[references-and-borrowing]: ch04-02-references-and-borrowing.md#değiştirilebilir-referanslar
[string-slices-as-parameters]: ch04-03-slices.md#parametre-olarak-string-dilimleri
[reference]: ../reference/index.md
