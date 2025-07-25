## Sahiplik Nedir?

_Ownership_ bir Rust programının belleği nasıl yönettiğini düzenleyen kurallar bütünüdür.
Tüm programlar çalışırken bilgisayarın belleğini nasıl kullanacaklarını yönetmek zorundadır.
Bazı diller, artık kullanılmayan çöpleri düzenli olarak arayan çöp toplama özelliğine sahiptir.
Program çalışırken bellek; diğer dillerde, programcı açıkça
belleği tahsis eder ve serbest bırakır. Rust üçüncü bir yaklaşım kullanır: bellek yönetilir
derleyicinin kontrol ettiği bir dizi kurala sahip bir sahiplik sistemi aracılığıyla. Eğer
kurallardan herhangi biri ihlal edilirse, program derlenmeyecektir. Özelliklerin hiçbiri
sahiplik kavramı programınızı çalışırken yavaşlatacaktır.

Sahiplik birçok programcı için yeni bir kavram olduğundan, biraz zaman alır
alışmak için. İyi haber şu ki, Rust konusunda ne kadar deneyimli olursanız
ve mülkiyet sisteminin kurallarını ne kadar iyi bilirseniz, doğal olarak
güvenli ve verimli kod geliştirin. Devam edin!

Sahipliği anladığınızda, aşağıdakileri anlamak için sağlam bir temele sahip olacaksınız
Rust'ı benzersiz kılan özellikler. Bu bölümde, şu yollarla sahiplik öğreneceksiniz
çok yaygın bir veri yapısına odaklanan bazı örnekler üzerinde çalışacağız:
dizeler.

> ### Yığın ve Yığın
>
> Birçok programlama dili, yığın ve yığıt hakkında düşünmenizi gerektirmez.
> heap çok sık kullanılır. Ancak Rust gibi bir sistem programlama dilinde, bir
> değerin yığında veya heap'te olması dilin nasıl davrandığını ve nedenini etkiler
> belirli kararlar vermeniz gerekir. Mülkiyetin bölümleri şu şekilde açıklanacaktır
> Bu bölümün ilerleyen kısımlarında yığın ve heap ile ilgili olarak, burada kısa bir
> açıklama hazırlık aşamasında.
>
> Hem yığın hem de heap, kodunuzun kullanabileceği bellek parçalarıdır
> çalışma zamanında, ancak farklı şekillerde yapılandırılmışlardır. Yığın şunları depolar
> değerleri aldığı sırayla alır ve tersi sıradaki değerleri kaldırır.
> sırası. Bu _son giren ilk çıkar_ olarak adlandırılır. Bir yığın düşünün
> tabaklar: daha fazla tabak eklediğinizde, onları yığının üstüne koyarsınız ve
> Bir plakaya ihtiyacınız olduğunda, üstten bir tane alırsınız. Plaka ekleme veya çıkarma
> orta veya alt da işe yaramaz! Veri eklemeye _pushing denir
> yığının üzerine_ ve veriyi kaldırmaya _yığın dışına_ atma denir. Tümü
Yığın üzerinde depolanan > veri bilinen, sabit bir boyuta sahip olmalıdır. Bilinmeyen veriler
> derleme zamanındaki boyut veya değişebilecek bir boyut heap üzerinde saklanmalıdır
> yerine.
>
> Yığın daha az düzenlidir: yığına veri koyduğunuzda, bir
> belirli miktarda alan. Bellek ayırıcı yığın içinde boş bir yer bulur
> yeterince büyükse, kullanımda olduğunu işaretler ve bir _pointer_ döndürür.
> o konumun adresidir. Bu işleme _allocating adı verilir.
> heap_ ve bazen sadece _allocating_ olarak kısaltılır (değerleri
> yığın ayırma olarak kabul edilmez). Çünkü heap işaretçisi bir
> bilinen, sabit boyutta, işaretçiyi yığında saklayabilirsiniz, ancak istediğiniz zaman
> gerçek veri, işaretçiyi takip etmelisiniz. Bir yerde oturduğunuzu düşünün
> restoran. Girdiğinizde, grubunuzdaki kişi sayısını belirtiyorsunuz ve
> Ev sahibi herkese uygun boş bir masa bulur ve sizi oraya götürür. Eğer
> Grubunuzdan biri geç gelirse, nerede oturduğunuzu sorabilir
> seni bulmak.
>
> Yığına itme işlemi, heap üzerinde ayırma işleminden daha hızlıdır çünkü
> ayırıcı hiçbir zaman yeni verileri depolamak için bir yer aramak zorunda değildir; bu konum
> her zaman yığının en üstünde yer alır. Karşılaştırmalı olarak, yığın üzerinde alan ayırma
> daha fazla çalışma gerektirir çünkü ayırıcı önce yeterince büyük bir alan bulmalıdır
> verileri tutmak ve ardından bir sonraki işleme hazırlanmak için defter tutma işlemini gerçekleştirmek için
> Tahsis.
>
> Yığın içindeki verilere erişmek genellikle veri tabanındaki verilere erişmekten daha yavaştır
> yığın çünkü oraya ulaşmak için bir işaretçiyi takip etmeniz gerekir. Çağdaş
> işlemciler bellekte daha az dolaştıklarında daha hızlıdırlar. Devam edersek
> Bir benzetme yapmak gerekirse, bir restoranda birçok masadan sipariş alan bir sunucuyu düşünün.
> Diğer masalara geçmeden önce bir masadaki tüm siparişleri almak en verimli yöntemdir.
> bir sonraki masa. A masasından bir sipariş alınıyor, sonra B masasından bir sipariş alınıyor,
> sonra tekrar A'dan bir tane ve sonra tekrar B'den bir tane çok daha yavaş bir
> işlem. Aynı şekilde, bir işlemci genellikle aşağıdaki durumlarda işini daha iyi yapabilir
> diğer verilere yakın olan veriler üzerinde (yığında olduğu gibi) çalışmak yerine
> daha uzakta (yığın üzerinde olabileceği gibi).
>
> Kodunuz bir işlevi çağırdığında, işleve aktarılan değerler
> (potansiyel olarak heap üzerindeki verilere işaretçiler dahil) ve işlevin
> yerel değişkenler yığına itilir. İşlev sona erdiğinde, bu
> değerler yığından çıkarılır.
>
> Kodun hangi bölümlerinin yığın üzerinde hangi verileri kullandığını takip etmek,
> Yığındaki yinelenen veri miktarını en aza indirmek ve kullanılmayan verileri temizlemek
> alanın tükenmemesi için yığın üzerindeki veriler, sahiplikle ilgili tüm sorunlardır
> adresler. Sahipliği bir kez anladıktan sonra, aşağıdakiler hakkında düşünmenize gerek kalmayacaktır
> yığın ve heap çok sık kullanılır, ancak sahipliğin temel amacının
> yığın verisini yönetmek, neden bu şekilde çalıştığını açıklamaya yardımcı olabilir.

### Mülkiyet Kuralları

İlk olarak, mülkiyet kurallarına bir göz atalım. Bu kuralları aklınızda bulundurun
bunları gösteren örnekler üzerinde çalışın:

- Rust'ta her değerin bir _sahibi_ vardır.
- Aynı anda yalnızca bir sahip olabilir.
- Sahibi kapsam dışına çıktığında, değer düşürülür.

### Değişken Kapsamı

Artık temel Rust sözdizimini geçtiğimize göre, tüm `fn main() {`
kodunu örneklerde bulabilirsiniz, bu nedenle eğer takip ediyorsanız aşağıdakileri eklediğinizden emin olun
örneklerini manuel olarak bir `main` fonksiyonu içine yerleştiriyoruz. Sonuç olarak, örneklerimiz bir
biraz daha özlü, daha ziyade gerçek detaylara odaklanmamızı sağlıyor
şablon kodu.

Sahipliğin ilk örneği olarak, bazı değişkenlerin _kapsamına_ bakacağız. A
kapsam, bir öğenin geçerli olduğu bir program içindeki aralıktır. Almak
aşağıdaki değişken:

```rust
let s = "hello";
```

s` değişkeni, dizenin değerinin şu olduğu bir dize değişmezini ifade eder
programımızın metnine kodlanmıştır. Değişken şu noktadan itibaren geçerlidir
geçerli _scope_'un sonuna kadar bildirilir. Liste 4-1'de bir
değişkeninin nerede geçerli olacağını açıklayan yorumlar içeren program.

<Listing number="4-1" caption="A variable and the scope in which it is valid">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

</Listing>

Başka bir deyişle, burada zaman içinde iki önemli nokta vardır:

- s` kapsam içine girdiğinde geçerlidir.
- Kapsam dışına çıkana kadar geçerli kalır.

Bu noktada, kapsamlar ve değişkenlerin geçerli olduğu zamanlar arasındaki ilişki
diğer programlama dillerindekine benzer. Şimdi bunun üzerine inşa edeceğiz
'String' türünü tanıtarak anlamak.

### `String` Türü

Sahiplik kurallarını göstermek için daha karmaşık bir veri türüne ihtiyacımız var
“Veri Türleri”][data-types]<!-- ignore --> bölümünde ele aldıklarımızdan daha fazla
Bölüm 3. Daha önce ele alınan türler bilinen bir boyuttadır, depolanabilir
yığına yerleştirilir ve kapsamları sona erdiğinde yığından çıkarılır ve
yeni, bağımsız bir örnek oluşturmak için hızlı ve önemsiz bir şekilde kopyalanır.
kod parçasının aynı değeri farklı bir kapsamda kullanması gerekir. Ama biz istiyoruz ki
yığın üzerinde depolanan verilere bakın ve Rust'ın ne zaman ne yapacağını nasıl bildiğini keşfedin.
Bu verileri temizlemek için `String` türü harika bir örnektir.

Biz `String`'in sahiplikle ilgili kısımlarına odaklanacağız. Bunlar
tarafından sağlanıp sağlanmadığına bakılmaksızın diğer karmaşık veri türleri için de geçerlidir.
standart kütüphane veya sizin tarafınızdan oluşturulur. String`i daha derinlemesine tartışacağız
[Bölüm 8] [ch8]<!-- görmezden gelin -->.

Bir dize değerinin sabit olarak kodlandığı dize değişmezlerini zaten gördük.
Program. Dize değişmezleri kullanışlıdır, ancak her program için uygun değildir
Metin kullanmak isteyebileceğimiz durumlar. Bunun bir nedeni
değişmez. Bir diğeri ise her string değerinin yazarken bilinemeyeceğidir.
Kodumuz: Örneğin, kullanıcı girdisini almak ve saklamak istersek ne olur? İçin
Bu durumlarda, Rust ikinci bir string tipine sahiptir, `String`. Bu tür yönetir
veri heap üzerinde ayrılmıştır ve bu nedenle aşağıdaki miktarda metin depolayabilir
derleme zamanında bizim için bilinmemektedir. Bir stringten `String` oluşturabilirsiniz
literalini `from` fonksiyonunu kullanarak aşağıdaki gibi kullanın:

```rust
let s = String::from("hello");
```

Çift iki nokta üst üste `::` işleci, bu özel `from` ad alanını oluşturmamızı sağlar
gibi bir isim kullanmak yerine `String` türü altında fonksiyon
`string_from`. Bu sözdizimini ["Yöntem
Bölüm 5'in [Syntax"][method-syntax]<!-- ignore --> bölümü ve
modüllerle ad aralığı oluşturma hakkında ["Bir Öğeye Başvurma Yolları
Bölüm 7'de [Modül Ağacı"][paths-module-tree]<!-- ignore -->.
This kind of string _can_ be mutated:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

Peki, buradaki fark nedir? Neden `String' mutasyona uğratılabilirken, değişmezler
yapamaz mı? Aradaki fark, bu iki türün bellekle nasıl ilgilendiğidir.

### Bellek ve Tahsis

Bir dizge değişmezi söz konusu olduğunda, içeriği derleme zamanında biliriz, bu nedenle
metni doğrudan son çalıştırılabilir dosyaya sabit kodlanır. Bu yüzden string
değişmezleri hızlı ve verimlidir. Ancak bu özellikler yalnızca dizeden gelir
değişmezliktir. Ne yazık ki, bir bellek bloğunu
boyutu derleme zamanında bilinmeyen her metin parçası için ikili
boyutu program çalıştırılırken değişebilir.

Değişebilir, büyüyebilir bir metin parçasını desteklemek için `String` türü ile,
heap üzerinde derleme zamanında bilinmeyen bir miktar bellek ayırmamız gerekir,
içeriği tutmak için. Bu şu anlama gelir:

- Bellek, çalışma zamanında bellek ayırıcıdan talep edilmelidir.
- İşimiz bittiğinde bu belleği ayırıcıya geri döndürmenin bir yoluna ihtiyacımız var
  bizim `String`imiz.

Bu ilk kısım bizim tarafımızdan yapılır: `String::from` çağırdığımızda, onun uygulaması
ihtiyaç duyduğu belleği talep eder. Bu, programlamada hemen hemen evrenseldir
diller.

Ancak ikinci kısım farklıdır. Çöp toplayıcıya sahip dillerde
(GC)_, GC kullanılmayan belleği takip eder ve temizler
ve bunun hakkında düşünmemize gerek yok. Çoğu dilde GC olmadan,
hafızanın artık kullanılmadığını tespit etmek bizim sorumluluğumuzdur ve
tıpkı onu talep etmek için yaptığımız gibi, onu açıkça serbest bırakmak için kodu çağırın. Bunu yapmak
tarihsel olarak zor bir programlama problemi olmuştur. Eğer unutursak,
hafızayı boşa harcamış oluruz. Eğer bunu çok erken yaparsak, geçersiz bir değişkenimiz olur. Eğer
bunu iki kez yaparsak, bu da bir hatadır. Tam olarak bir `allocate` ile
tam olarak bir `free`.

Rust farklı bir yol izler: bellek otomatik olarak
sahibi olan değişken kapsam dışına çıkar. İşte kapsam örneğimizin bir versiyonu
Listeleme 4-1'den bir string literal yerine bir `String` kullanarak:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

`String`imizin ihtiyaç duyduğu belleği geri verebileceğimiz doğal bir nokta vardır
tahsis ediciye: `s` kapsam dışına çıktığında. Bir değişken kapsam dışına çıktığında
kapsamında, Rust bizim için özel bir fonksiyon çağırır. Bu fonksiyonun adı
[`drop`][drop]<!-- ignore -->, ve `String` yazarının koyabileceği yer
belleği geri döndürmek için kod. Rust kapanışta otomatik olarak `drop` çağırır
küme parantezi.

> Not: C++'da, bir öğenin sonundaki kaynakları ayırma modeli
> Yaşam süresi bazen _Resource Acquisition Is Initialization (RAII)_ olarak adlandırılır.
> Eğer RAII kullandıysanız Rust'taki `drop` fonksiyonu size tanıdık gelecektir
> kalıpları.

Bu modelin Rust kodunun yazılma şekli üzerinde derin bir etkisi vardır. Bu görünebilir
Şu anda basit, ancak kodun davranışı daha fazla durumda beklenmedik olabilir
birden fazla değişkenin verileri kullanmasını istediğimizde karmaşık durumlar
heap üzerinde ayırdık. Şimdi bu durumlardan bazılarını inceleyelim.


<!-- Old heading. Do not remove or links may break. -->

<a id="ways-variables-and-data-interact-move"></a>

#### Variables and Data Interacting with Move

Rust'ta birden fazla değişken aynı veri ile farklı şekillerde etkileşime girebilir.
Liste 4-2'de bir tamsayı kullanan bir örneğe bakalım.

<Listing number="4-2" caption="Assigning the integer value of variable `x` to `y`">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

</Listing>

Bunun ne yaptığını muhtemelen tahmin edebiliriz: "`5` değerini `x` değerine bağla; sonra
'deki değerin bir kopyasını alın ve `y`'ye bağlayın." Şimdi iki değişkenimiz var, `x`
ve `y`, ve her ikisi de `5`e eşittir. Gerçekten de olan budur, çünkü tamsayılar
bilinen, sabit bir boyuta sahip basit değerlerdir ve bu iki `5` değeri
yığın üzerine.

Şimdi `String` versiyonuna bakalım:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

Bu çok benzer görünüyor, bu yüzden çalışma şeklinin aşağıdaki gibi olacağını varsayabiliriz
aynıdır: yani, ikinci satır `s1` içindeki değerin bir kopyasını oluşturur ve
's2'ye. Ancak olan tam olarak bu değildir.

Şekil `4-1`e bakarak `String`e ne olduğunu görebilirsiniz.
kapsar. Bir `String` solda gösterilen üç parçadan oluşur: bir işaretçi
dizenin içeriğini, bir uzunluğu ve bir kapasiteyi tutan bellek.
Bu veri grubu yığın üzerinde saklanır. Sağ tarafta ise bellek
içeriği tutan yığın.

<img alt="Two tables: the first table contains the representation of s1 on the
stack, consisting of its length (5), capacity (5), and a pointer to the first
value in the second table. The second table contains the representation of the
string data on the heap, byte by byte." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />

<span class="caption">Şekil 4-1: `s1`</span>'e bağlı `"hello"` değerini tutan bir `String`
'un bellekteki gösterimi

Uzunluk, `String`in içeriğinin
şu anda bayt cinsinden ne kadar bellek kullandığını gösterir. Kapasite,
`String`in tahsis ediciden aldığı toplam bellek miktarıdır, bayt cinsinden. Uzunluk ve
kapasitesi arasındaki fark önemlidir, ancak bu bağlamda önemli değildir, bu nedenle şimdilik
kapasitesini göz ardı etmekte bir sakınca yoktur.

s1`i `s2`ye atadığımızda, `String` verisi kopyalanır, yani
işaretçisini, uzunluğu ve yığında bulunan kapasiteyi kopyalarız. İşaretçinin işaret ettiği heap üzerindeki
verisini kopyalamayız. Başka bir deyişle, bellekteki veri
gösterimi Şekil 4-2'deki gibi görünür.

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap."
src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Şekil 4-2: `s1`</span> işaretçisinin, uzunluğunun ve kapasitesinin bir kopyasına sahip olan `s2`
değişkeninin bellekteki gösterimi

Bu gösterim Şekil 4-3'teki gibi görünmez, eğer Rust bunun yerine heap verilerini de kopyalasaydı bellek
böyle görünürdü. Eğer Rust bunu yapsaydı,
`s2 = s1` işlemi
heap üzerindeki veri büyükse çalışma zamanı performansı açısından çok pahalı olabilirdi.

<img alt="Four tables: two tables representing the stack data for s1 and s2,
and each points to its own copy of string data on the heap."
src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Şekil 4-3: Rust yığın verilerini de kopyaladıysa `s2 = s1`in
ne yapabileceğine dair başka bir olasılık</span>

Daha önce, bir değişken kapsam dışına çıktığında, Rust'ın otomatik olarak
`drop` fonksiyonunu çağırdığını ve bu değişken için yığın belleği temizlediğini söylemiştik. Ancak
Şekil 4-2, her iki veri işaretçisinin de aynı konuma işaret ettiğini göstermektedir. Bu bir
sorunudur: `s2` ve `s1` kapsam dışına çıktığında, her ikisi de
aynı belleği boşaltmaya çalışacaktır. Bu _double free_ hatası olarak bilinir ve daha önce bahsettiğimiz bellek
güvenliği hatalarından biridir. Belleğin iki kez serbest bırakılması
belleğin bozulmasına yol açabilir ve bu da potansiyel olarak güvenlik açıklarına neden olabilir.

Bellek güvenliğini sağlamak için, `let s2 = s1;` satırından sonra, Rust `s1`i artık geçerli olmayan
olarak kabul eder. Bu nedenle, `s1`
kapsam dışına çıktığında Rust'ın herhangi bir şeyi serbest bırakmasına gerek yoktur. s2`
oluşturulduktan sonra `s1` kullanmaya çalıştığınızda ne olacağını kontrol edin; çalışmayacaktır:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

Bunun gibi bir hata alırsınız çünkü Rust, aşağıdakileri kullanmanızı engeller
geçersiz referans:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

ile çalışırken _shallow copy_ ve _deep copy_ terimlerini duyduysanız
diğer dillerde, işaretçi, uzunluk ve kapasite kopyalama kavramı
verileri kopyalamadan muhtemelen yüzeysel bir kopyalama yapmak gibi görünmektedir. Ama
çünkü Rust aynı zamanda ilk değişkeni geçersiz kılar, bunun yerine
sığ kopyalama, _move_ olarak bilinir. Bu örnekte, `s1`
`s2` içine _taşındı_. Yani, gerçekte ne olduğu Şekil 4-4'te gösterilmektedir.

<img alt="Three tables: tables s1 and s2 representing those strings on the
stack, respectively, and both pointing to the same string data on the heap.
Table s1 is grayed out be-cause s1 is no longer valid; only s2 can be used to
access the heap data." src="img/trpl04-04.svg" class="center" style="width:
50%;" />

<span class="caption">Şekil 4-4: `s1`
geçersiz kılındıktan sonra bellekteki gösterim</span>

Bu sorunumuzu çözer! Sadece `s2` geçerli olduğunda, kapsam dışına çıktığında
tek başına belleği boşaltacak ve işimiz bitecek.

Buna ek olarak, bununla ima edilen bir tasarım seçimi var: Rust asla
verilerinizin “derin” kopyalarını otomatik olarak oluşturmayacaktır. Bu nedenle, herhangi bir _otomatik_
kopyalamanın çalışma zamanı performansı açısından ucuz olduğu varsayılabilir.

#### Kapsam ve Atama

Bunun tersi, kapsam belirleme, sahiplik ve
belleğin `drop` işlevi aracılığıyla serbest bırakılması arasındaki ilişki için de geçerlidir. Mevcut bir değişkene tamamen
yeni bir değer atadığınızda, Rust `drop` fonksiyonunu çağıracak ve orijinal
değerinin belleğini hemen serbest bırakacaktır. Örneğin bu kodu düşünün:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04b-replacement-drop/src/main.rs:here}}
```

Başlangıçta bir `s` değişkeni bildiririz ve bunu değeri olan bir `String`e bağlarız
`“merhaba”`. Ardından hemen `“ahoy”` değerine sahip yeni bir `String` oluşturuyoruz ve
bunu `s` ye atayın. Bu noktada, hiçbir şey `s` üzerindeki orijinal değere atıfta bulunmaz.
yığın hiç.

<img alt="One table s representing the string value on the stack, pointing to
the second piece of string data (ahoy) on the heap, with the original string
data (hello) grayed out because it cannot be accessed anymore."
src="img/trpl04-05.svg"
class="center"
style="width: 50%;"
/>

<span class="caption">Şekil 4-5: Başlangıçtaki
değeri tamamen değiştirildikten sonra bellekteki gösterim</span>.

Böylece orijinal string hemen kapsam dışına çıkar. Rust bunun üzerinde `drop`
fonksiyonunu çalıştıracak ve bellek hemen serbest bırakılacaktır. Sonunda
değerini yazdırdığımızda, `“ahoy, world!”` olacaktır.

<!-- Old heading. Do not remove or links may break. -->

<a id="ways-variables-and-data-interact-clone"></a>

#### Klon ile Etkileşime Giren Değişkenler ve Veriler

Eğer `String`in heap verisini derinlemesine kopyalamak istiyorsak, sadece
yığın verisi için `clone` adı verilen yaygın bir yöntem kullanabiliriz. Yöntemi tartışacağız
sözdizimi Bölüm 5'te açıklanmıştır, ancak yöntemler birçok ülkede ortak bir özellik olduğu için
programlama dilleri, muhtemelen bunları daha önce görmüşsünüzdür.

İşte `clone` yönteminin iş başında olduğu bir örnek:


```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

Bu gayet iyi çalışır ve Şekil 4-3'te gösterilen davranışı açıkça üretir,
yığın verisinin _kopyalandığı_ yer.

Bir `clone` çağrısı gördüğünüzde, bazı keyfi kodların kopyalandığını bilirsiniz.
yürütülür ve bu kod pahalı olabilir. Bu görsel bir gösterge.
farklı bir şey oluyor.

#### Yalnızca Yığın Veriler: Kopyalama

Henüz bahsetmediğimiz bir başka sorun daha var. Bu kod
tamsayıları -bir kısmı Liste 4-2'de gösterilmiştir- çalışır ve geçerlidir:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

Ancak bu kod az önce öğrendiklerimizle çelişiyor gibi görünüyor:
`clone`, ancak `x` hala geçerlidir ve `y` içine taşınmamıştır.

Bunun nedeni, derleme sırasında bilinen bir boyuta sahip olan tamsayılar gibi türlerin
zaman tamamen yığında saklanır, bu nedenle gerçek değerlerin kopyaları hızlı
yapmak için. Bu da `x'in, ‘x’ olmasını engellemek istememiz için hiçbir neden olmadığı anlamına gelir.
değişkenini oluşturduktan sonra geçerlidir. Başka bir deyişle, hiçbir fark yok
Burada derin ve sığ kopyalama arasında bir fark yoktur, bu nedenle `clone` çağrısı hiçbir şey yapmaz
normal sığ kopyalamadan farklıdır ve bunu dışarıda bırakabiliriz.

Rust, `Copy` özelliği adı verilen özel bir ek açıklamaya sahiptir.
tamsayılar gibi yığında depolanan türler (tamsayılar hakkında daha fazla konuşacağız)
traits in [Chapter 10][traits]<!-- ignore -->). Eğer bir tip `Copy`
özelliğini kullanan değişkenler hareket etmez, bunun yerine önemsiz bir şekilde kopyalanır,
başka bir değişkene atandıktan sonra da geçerli olmasını sağlar.

Rust, tip veya herhangi bir parçası `Copy` ile açıklama yapmamıza izin vermez,
`Drop` özelliğini uygulamıştır. Tipin özel bir şeye ihtiyacı varsa
değer kapsam dışına çıktığında ve bu türe `Copy` ek açıklamasını eklediğimizde,
derleme zamanı hatası alırız. Copy` ek açıklamasının nasıl ekleneceği hakkında bilgi edinmek için
özelliği uygulamak için türünüze, bkz ["Türetilebilir
[Traits"][derivable-traits]<!-- ignore --> Ek C'de.

Peki, hangi türler `Copy` özelliğini uygular? Şunlar için belgeleri kontrol edebilirsiniz
ancak genel bir kural olarak, herhangi bir basit skaler grup
değerleri `Copy` uygulayabilir ve tahsis gerektiren veya bazı
kaynak biçimi `Copy` uygulayabilir. İşte bu türlerden bazıları
`Copy` uygulamasını gerçekleştirir:

- u32` gibi tüm tamsayı türleri.
- `True` ve `false` değerlerine sahip Boolean türü, `bool`.
- f64` gibi tüm kayan nokta türleri.
- Karakter türü, `char`.
- Tuple'lar, yalnızca `Copy` uygulayan türleri içeriyorlarsa. Örneğin,
  `(i32, i32)` `Copy` fonksiyonunu uygular, ancak `(i32, String)` fonksiyonunu uygulamaz.

### Sahiplik ve İşlevler

Bir fonksiyona bir değer aktarmanın mekaniği, aşağıdaki durumlarla benzerdir
bir değişkene değer atamak. Bir değişkeni bir işleve aktarmak, değişkeni hareket ettirir veya
kopyalar, tıpkı atamanın yaptığı gibi. Liste 4-3'te bazı ek açıklamalar içeren bir örnek bulunmaktadır
değişkenlerin kapsam içine girdiği ve kapsam dışına çıktığı yerleri gösterir.

<Listing number="4-3" file-name="src/main.rs" caption="Functions with ownership and scope annotated">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

</Listing>

Eğer `takes_ownership` çağrısından sonra `s` kullanmaya çalışsaydık, Rust bir
derleme zamanı hatası. Bu statik kontroller bizi hatalardan korur. Eklemeyi deneyin
s` ve `x` kullanan `main` kodunu nerede kullanabileceğinizi ve nerede kullanabileceğinizi görmek için
sahiplik kuralları bunu yapmanızı engeller.

### Dönüş Değerleri ve Kapsamı

Dönen değerler de sahiplik aktarımı yapabilir. Liste 4-4 bir örnek göstermektedir
ile benzer ek açıklamalar içeren bir değer döndüren işlev
4-3.

<Listing number="4-4" file-name="src/main.rs" caption="Transferring ownership of return values">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

</Listing>

Bir değişkenin sahipliği her seferinde aynı kalıbı izler: bir
değerini başka bir değişkene taşır. üzerinde veri içeren bir değişken olduğunda
heap kapsam dışına çıktığında, değer sahiplik olmadıkça `drop` tarafından temizlenecektir
verinin başka bir değişkene taşındığını gösterir.

Bu işe yarasa da, sahiplik almak ve ardından her
fonksiyonu biraz sıkıcıdır. Ya bir fonksiyonun bir değer kullanmasına izin vermek istiyorsak ama
sahiplenmiyor mu? İçeri girdiğimiz her şeyin aynı zamanda
tekrar kullanmak istediğimizde, ortaya çıkan herhangi bir veriye ek olarak geri aktarılacaktır.
fonksiyonun gövdesinden de döndürmek isteyebiliriz.

Rust, Liste 4-5'te gösterildiği gibi bir tuple kullanarak birden fazla değer döndürmemize izin verir.

<Listing number="4-5" file-name="src/main.rs" caption="Returning ownership of parameters">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

</Listing>

Ancak bu, olması gereken bir konsept için çok fazla tören ve çok fazla iş
yaygındır. Şanslıyız ki Rust'ta bir değeri
mülkiyetin aktarılması, _referanslar_ olarak adlandırılır.

[data-types]: ch03-02-data-types.html#data-types
[ch8]: ch08-02-strings.html
[traits]: ch10-02-traits.html
[derivable-traits]: appendix-03-derivable-traits.html
[method-syntax]: ch05-03-method-syntax.html#method-syntax
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[drop]: ../std/ops/trait.Drop.html#tymethod.drop
