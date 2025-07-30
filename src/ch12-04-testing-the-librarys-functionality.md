## Test Güdümlü Geliştirme ile Kütüphanenin İşlevselliğini Geliştirme

Artık _src/lib.rs_ içinde `main`
işlevinden ayrı bir arama mantığımız olduğuna göre,
kodumuzun temel işlevleri için testler yazmak çok daha kolay. Fonksiyonları çeşitli argümanlarla doğrudan çağırabilir ve ikili dosyamızı komut satırından çağırmak zorunda kalmadan dönen
değerlerini kontrol edebiliriz.

Bu bölümde, aşağıdaki adımlarla test güdümlü geliştirme (TDD) sürecini
kullanarak `minigrep` programına arama mantığını ekleyeceğiz:

1. Başarısız olan bir test yazın ve
 beklediğiniz nedenle başarısız olduğundan emin olmak için çalıştırın.
2. Yeni testin geçmesini sağlayacak kadar kod yazın ya da değiştirin.
3. Yeni eklediğiniz veya değiştirdiğiniz kodu yeniden düzenleyin ve testlerin
 geçmeye devam ettiğinden emin olun.
4. Adım 1'den itibaren tekrarlayın!

TDD, yazılım yazmanın birçok yolundan yalnızca biri olsa da, kodun
tasarımını yönlendirmeye yardımcı olabilir. Testin geçmesini sağlayan kodu yazmadan önce testi yazmak
süreç boyunca yüksek test kapsamının korunmasına yardımcı olur.

Dosya içeriğinde
sorgu dizesini arayacak ve sorguyla eşleşen
satırlarının bir listesini üretecek işlevselliğin uygulanmasını test edeceğiz. Bu işlevi
`search` adlı bir fonksiyona ekleyeceğiz.

### Başarısız Bir Test Yazmak

_src/lib.rs_ dosyasına,
[Bölüm 11][ch11-anatomy]<!-- ignore --> dosyasında yaptığımız gibi bir test fonksiyonuna sahip bir `tests` modülü ekleyeceğiz. Test fonksiyonu, `search` fonksiyonunun sahip olmasını istediğimiz
davranışını belirtir: bir sorgu ve aranacak
metnini alır ve metinden yalnızca sorguyu
içeren satırları döndürür. Liste 12-15 bu testi göstermektedir.

<Listing number="12-15" file-name="src/lib.rs" caption="Creating a failing test for the `search` function for the functionality we wish we had">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

</Listing>

Bu test `"duct"` dizesini arar. Aradığımız metin, yalnızca biri `"duct"` içeren üç
satırdır (
açılış çift tırnak işaretinden sonraki ters eğik çizginin Rust'a bu dize değişmezinin içeriğinin başına
yeni satır karakteri koymamasını söylediğine dikkat edin). `search` fonksiyonundan dönen değerin sadece beklediğimiz satırı içerdiğini iddia ediyoruz.

Eğer bu testi çalıştırırsak, şu anda başarısız olacaktır çünkü `unimplemented!` makrosu
“not implemented” mesajıyla paniğe kapılır. TDD ilkelerine uygun olarak,
Listing 12-16'da gösterildiği gibi, `search` fonksiyonunu her zaman bir
boş vektör döndürecek şekilde tanımlayarak, fonksiyonu çağırırken testin paniklememesi için
yeterli kodu ekleyerek küçük bir adım atacağız. Ardından test derlenmeli ve boş bir vektör `"safe,
fast, productive."` satırını içeren bir vektörle eşleşmediği için
başarısız olmalıdır.

<Listing number="12-16" file-name="src/lib.rs" caption="Defining just enough of the `search` function so calling it won’t panic">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

</Listing>

Şimdi neden `search` öğesinin
imzasında açık bir `'a` yaşam süresi tanımlamamız ve bu yaşam süresini `contents` argümanıyla ve
dönüş değeriyle kullanmamız gerektiğini tartışalım. Bölüm 10][ch10-lifetimes]<!-- ignore -->'da
yaşam süresi parametrelerinin hangi argüman yaşam süresinin dönüş değerinin
yaşam süresine bağlı olduğunu belirttiğini hatırlayın. Bu durumda, döndürülen
vektörünün,
`contents` argümanının dilimlerine (`query` argümanı yerine) referans veren dize dilimleri içermesi gerektiğini belirtiyoruz.

Başka bir deyişle, Rust'a
`search` fonksiyonu tarafından döndürülen verilerin
`contents` argümanında `search` fonksiyonuna aktarılan veriler kadar yaşayacağını söylüyoruz. Bu çok önemli! Bir dilim tarafından referans verilen verinin geçerli olabilmesi için
geçerli olması gerekir; eğer derleyici
string dilimlerini `contents` yerine `query` yaptığımızı varsayarsa, güvenlik kontrolünü
yanlış yapacaktır.

Yaşam süresi ek açıklamalarını unutur ve bu fonksiyonu derlemeye çalışırsak,
bu hatayı alırız:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust, çıktı için iki parametreden hangisine ihtiyacımız olduğunu bilemez, bu nedenle
adresine açıkça söylememiz gerekir. Yardım metninin tüm parametreler ve çıktı türü için aynı
lifetime parametresini belirtmeyi önerdiğine dikkat edin, ki bu
yanlıştır! Contents` parametresi
metnimizin tamamını içerdiğinden ve bu metnin eşleşen kısımlarını döndürmek istediğimizden,
lifetime sözdizimini kullanarak dönüş değerine bağlanması gereken tek parametrenin
olduğunu biliyoruz.

Diğer programlama dilleri, imzada
değerlerini döndürmek için argümanları bağlamanızı gerektirmez, ancak bu uygulama zamanla daha kolay hale gelecektir. Bu örneği
Bölüm 10'daki [“Validating Referenceswith Lifetimes”][validating-references-with-lifetimes]<!-- ignore -->
bölümündeki örneklerle karşılaştırmak isteyebilirsiniz.

### Testi Geçmek İçin Kod Yazma

Şu anda, her zaman boş bir vektör döndürdüğümüz için testimiz başarısız oluyor. Bunu
düzeltmek ve `search` uygulamak için programımızın aşağıdaki adımları izlemesi gerekir:

1. İçeriğin her satırını yineleyin.
2. Satırın sorgu dizemizi içerip içermediğini kontrol edin.
3. Eğer içeriyorsa, döndürdüğümüz değerler listesine ekleyin.
4. Eğer içermiyorsa, hiçbir şey yapmayın.
5. Eşleşen sonuçların listesini döndürün.

Satırlar arasında yineleme ile başlayarak her adımda çalışalım.

#### `lines` Yöntemi ile Satırlar Arasında Yineleme

Rust, dizelerin satır satır yinelenmesini işlemek için yararlı bir yönteme sahiptir,
uygun bir şekilde `lines` olarak adlandırılır ve Liste 12-17'de gösterildiği gibi çalışır. Unutmayın ki
bu henüz derlenmeyecektir.

<Listing number="12-17" file-name="src/lib.rs" caption="Iterating through each line in `contents`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

</Listing>

lines` yöntemi bir yineleyici döndürür. Yineleyiciler hakkında
[Bölüm 13][ch13-iterators]<!-- ignore --> adresinde derinlemesine konuşacağız, ancak bir yineleyici kullanmanın bu yolunu
adresinde gördüğünüzü hatırlayın [Listing 3-5][ch3-iter]<!-- ignore -->, burada bir koleksiyondaki her öğe üzerinde bazı kodlar çalıştırmak için bir yineleyici ile bir
`for` döngüsü kullandık.

#### Sorgu için Her Satırı Arama

Daha sonra, geçerli satırın sorgu dizemizi içerip içermediğini kontrol edeceğiz.
Neyse ki, dizelerin bunu
bizim için yapan `contains` adlı yararlı bir yöntemi vardır! Liste 12-18'de gösterildiği gibi `search` fonksiyonuna `contains` metoduna bir çağrı ekleyin. Bunun henüz derlenmeyeceğini unutmayın.

<Listing number="12-18" file-name="src/lib.rs" caption="Adding functionality to see whether the line contains the string in `query`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

</Listing>

Şu anda işlevsellik oluşturuyoruz. Kodun derlenmesini sağlamak için

 imzasında belirttiğimiz gibi gövdeden bir değer döndürmemiz gerekir.

#### Eşleşen Satırları Saklama

Bu fonksiyonu tamamlamak için,
adresinin döndürmesini istediğimiz eşleşen satırları saklamanın bir yoluna ihtiyacımız var. Bunun için `for` döngüsünden önce değişebilir bir vektör oluşturabilir ve
adresinden `push` yöntemini çağırarak vektörde bir `line` saklayabiliriz. for` döngüsünden sonra,
Liste 12-19'da gösterildiği gibi vektörü döndürürüz.

<Listing number="12-19" file-name="src/lib.rs" caption="Storing the lines that match so we can return them">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

</Listing>

Şimdi `search` fonksiyonu sadece `query`,
içeren satırları döndürmeli ve testimiz geçmelidir. Testi çalıştıralım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Testimiz geçti, yani çalıştığını biliyoruz!

Bu noktada,
adresine geçen testlerin aynı işlevselliği sürdürmesini sağlarken arama işlevinin
uygulamasını yeniden düzenleme fırsatlarını değerlendirebiliriz. Arama fonksiyonundaki kod çok kötü değil,
ancak yineleyicilerin bazı yararlı özelliklerinden yararlanmıyor. Bu örneğe
[Bölüm 13][ch13-iterators]<!-- ignore -->'da geri döneceğiz, burada
yineleyicileri ayrıntılı olarak inceleyeceğiz ve nasıl geliştireceğimize bakacağız.

Şimdi tüm program çalışmalıdır! İlk olarak
adresinin Emily Dickinson şiirinden tam olarak bir satır döndürmesi gereken bir kelimeyle deneyelim: _frog_.

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

Harika! Şimdi _body_ gibi birden fazla satırla eşleşecek bir kelime deneyelim:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

Ve son olarak, şiirin hiçbir yerinde olmayan bir
kelimesini aradığımızda, örneğin _monomorphization_ gibi, herhangi bir satır almadığımızdan emin olalım:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

Mükemmel! Klasik bir aracın kendi mini versiyonunu oluşturduk ve uygulamaların nasıl yapılandırılacağı hakkında
çok şey öğrendik. Ayrıca dosya girişi
ve çıkışı, yaşam süreleri, test etme ve komut satırı ayrıştırma hakkında da biraz bilgi edindik.

Bu projeyi tamamlamak için,
ortam değişkenleri ile nasıl çalışılacağını ve standart hataya nasıl yazdırılacağını kısaca göstereceğiz, her ikisi de komut satırı programları yazarken
yararlıdır.

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.md#referansları-yaşam-süreleri-ile-doğrulama
[ch11-anatomy]: ch11-01-writing-tests.md#test-i̇şlevinin-yapısı
[ch10-lifetimes]: ch10-03-lifetime-syntax.md
[ch3-iter]: ch03-05-control-flow.md#for-ile-bir-koleksiyonda-döngü-oluşturma
[ch13-iterators]: ch13-02-iterators.md
