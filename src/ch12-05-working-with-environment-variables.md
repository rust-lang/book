## Ortam Değişkenleri ile Çalışma

minigrep' ikilisini ekstra bir özellik ekleyerek geliştireceğiz:
büyük/küçük harfe duyarlı olmayan arama için kullanıcının
ortam değişkeni aracılığıyla açabileceği bir seçenek. Bu özelliği bir komut satırı seçeneği haline getirebilir ve
kullanıcılarının bunu her istediklerinde girmelerini zorunlu kılabilirdik, ancak bunun yerine bunu bir
ortam değişkeni haline getirerek, kullanıcılarımızın ortam değişkenini bir kez
ayarlamalarına ve o terminal oturumunda tüm aramalarının büyük/küçük harfe duyarsız olmasına izin veriyoruz.

### Büyük/Küçük Harfe Duyarsız `search` Fonksiyonu için Başarısızlık Testi Yazma

İlk olarak `minigrep` kütüphanesine
yeni bir `search_case_insensitive` fonksiyonu ekliyoruz ve bu fonksiyon ortam değişkeni bir değere sahip olduğunda çağrılacak. TDD sürecini takip etmek için
adresine devam edeceğiz, bu nedenle ilk adım yine başarısız bir test yazmaktır.
Yeni `search_case_insensitive` fonksiyonu için yeni bir test ekleyeceğiz ve Liste 12-20'de gösterildiği gibi iki test arasındaki farkları
netleştirmek için eski testimizin adını
`one_result` yerine `case_sensitive` olarak değiştireceğiz.

<Listing number="12-20" file-name="src/lib.rs" caption="Adding a new failing test for the case-insensitive function we’re about to add">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

</Listing>

Eski testin “içeriğini” de düzenlediğimize dikkat edin. Büyük/küçük harfe duyarlı bir şekilde arama yaparken
`"duct"` sorgusuyla eşleşmemesi gereken büyük _D_ harfini kullanarak `"Duct tape."` metniyle birlikte
yeni bir satır ekledik. Eski testin
bu şekilde değiştirilmesi, halihazırda uyguladığımız büyük/küçük harfe duyarlı
arama işlevini yanlışlıkla bozmamamızı sağlamaya yardımcı olur. Bu test şimdi geçmelidir
ve biz büyük/küçük harfe duyarsız arama üzerinde çalışırken geçmeye devam etmelidir.

Büyük/küçük harfe duyarsız_ arama için yeni test, sorgu olarak `"rUsT"` kullanır. Eklemek üzere olduğumuz `search_case_insensitive` fonksiyonunun
adresinde, `"rUsT"`
sorgusu, büyük _R_ ile `"Rust:"` içeren satırla eşleşmeli ve her ikisi de sorgudan farklı harflere sahip olsa bile
`"Güven bana."` satırıyla eşleşmelidir. Bu
bizim başarısız testimizdir ve henüz
`search_case_insensitive` fonksiyonunu tanımlamadığımız için derlenemeyecektir. Testin derlendiğini ve başarısız olduğunu görmek için Liste 12-16'daki `search` işlevi için
yaptığımıza benzer şekilde, her zaman boş bir vektör döndüren bir iskelet
uygulaması eklemekten çekinmeyin.

### `search_case_insensitive` İşlevinin Uygulanması

Listing 12-21'de gösterilen `search_case_insensitive` fonksiyonu neredeyse
`search` fonksiyonu ile aynı olacaktır. Tek fark
`query` ve her `line` küçük harfle yazılacak böylece girdi argümanlarının büyük/küçük harf durumu ne olursa olsun
satırın sorguyu içerip içermediğini kontrol ettiğimizde aynı büyük/küçük harf durumunda olacaklar.

<Listing number="12-21" file-name="src/lib.rs" caption="Defining the `search_case_insensitive` function to lowercase the query and the line before comparing them">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

</Listing>

İlk olarak `query` dizesini küçük harfle yazıyoruz ve orijinal `query` dizesini gölgeleyerek
aynı isimli yeni bir değişkende saklıyoruz. Sorgu üzerinde `to_lowercase` çağrısı yapmak
gereklidir, böylece kullanıcının sorgusu `"rust"`, `"RUST"`,
`"Rust"` veya `"``rUsT``"` olsun, sorguya `"rust"`muş gibi davranacağız ve
büyük/küçük harfe duyarsız olacaktır. to_lowercase` temel Unicode'u işleyecek olsa da,
yüzde 100 doğru olmayacaktır. Gerçek bir uygulama yazıyor olsaydık,
adresinin burada biraz daha fazla iş yapmasını isterdik, ancak bu bölüm Unicode ile değil, ortam değişkenleri
ile ilgilidir, bu yüzden burada bırakacağız.

Sorgu`nun artık bir string dilimi yerine bir `String` olduğuna dikkat edin, çünkü
`to_lowercase` çağrısı mevcut verilere referans vermek yerine yeni veriler oluşturur. Örnek olarak,
sorgusunun `"rUsT"` olduğunu varsayalım: bu string diliminde kullanabileceğimiz küçük harfli bir
`u` veya `t` bulunmadığından,
`"rust"` içeren yeni bir `String` tahsis etmemiz gerekir. Şimdi `contains` yöntemine argüman olarak `query` ilettiğimizde,
bir ve işareti eklememiz gerekir, çünkü `contains` imzası
bir string dilimi alacak şekilde tanımlanmıştır.

Ardından, tüm
karakterlerini küçük harfle yazmak için her `satır`a `to_lowercase` çağrısı ekliyoruz. Artık `line` ve `query` karakterlerini küçük harfe dönüştürdüğümüze göre, sorgunun büyük/küçük harf durumu ne olursa olsun
eşleşmeleri bulacağız.

Bakalım bu uygulama testleri geçebilecek mi?

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

Harika! Geçtiler. Şimdi, yeni `search_case_insensitive` fonksiyonunu
`run` fonksiyonundan çağıralım. Öncelikle `Config`
yapısına büyük/küçük harfe duyarlı ve duyarsız arama arasında geçiş yapmak için bir yapılandırma seçeneği ekleyeceğiz. bu alanı eklemek derleyici hatalarına neden olacaktır çünkü bu alanı
henüz hiçbir yerde başlatmıyoruz:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:here}}
```

Boolean tutan `ignore_case` alanını ekledik. Daha sonra, `run`
fonksiyonunun `ignore_case` alanının değerini kontrol etmesine ve bunu kullanarak
`search` fonksiyonunu mu yoksa `search_case_insensitive`
fonksiyonunu mu çağıracağına karar vermesine ihtiyacımız var, Listing 12-22'de gösterildiği gibi. Bu yine de henüz derlenmeyecektir.

<Listing number="12-22" file-name="src/main.rs" caption="Calling either `search` or `search_case_insensitive` based on the value in `config.ignore_case`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:there}}
```

</Listing>

Son olarak, ortam değişkenini kontrol etmemiz gerekir. Ortam değişkenleri ile çalışan
fonksiyonları standart
kütüphanesindeki `env` modülünde bulunmaktadır ve bu modül _src/main.rs_ dosyasının en üstünde zaten kapsam dahilindedir. Liste 12-23'te gösterildiği gibi, `IGNORE_CASE` adlı bir ortam değişkeni için
herhangi bir değer ayarlanıp ayarlanmadığını kontrol etmek için `env` modülündeki
`var` işlevini kullanacağız.

<Listing number="12-23" file-name="src/main.rs" caption="Checking for any value in an environment variable named `IGNORE_CASE`">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/main.rs:here}}
```

</Listing>

Burada, `ignore_case` adında yeni bir değişken oluşturuyoruz. Değerini ayarlamak için
`env::var` fonksiyonunu çağırıyoruz ve ona `IGNORE_CASE` ortam
değişkeninin adını aktarıyoruz. env::var` fonksiyonu,
ortam değişkeni herhangi bir değere ayarlanmışsa, ortam değişkeninin değerini içeren
başarılı `Ok` değişkeni olacak bir `Result` döndürür. Ortam değişkeni ayarlanmamışsa `Err` değişkenini
döndürür.

Ortam
değişkeninin ayarlanıp ayarlanmadığını kontrol etmek için `Result` üzerinde `is_ok` yöntemini kullanıyoruz, bu da programın büyük/küçük harfe duyarlı olmayan bir arama yapması gerektiği anlamına gelir.
Eğer `IGNORE_CASE` ortam değişkeni herhangi bir şeye ayarlanmamışsa, `is_ok`
`false` değerini döndürür ve program büyük/küçük harfe duyarlı bir arama gerçekleştirir. Ortam değişkeninin değeri
umurumuzda değil, sadece ayarlı mı yoksa
ayarsız mı olduğu önemli, bu nedenle `unwrap`, `expect` veya `Result` üzerinde gördüğümüz diğer yöntemlerden herhangi birini
kullanmak yerine `is_ok` değerini kontrol ediyoruz.

Config` örneğine `ignore_case` değişkenindeki değeri aktarıyoruz, böylece
`run` fonksiyonu bu değeri okuyabilir ve Listing 12-22'de uyguladığımız gibi
`search_case_insensitive` veya `search` çağrısı yapıp yapmayacağına karar verebilir.

Hadi bir deneyelim! İlk olarak programımızı
ortam değişkeni ayarlanmadan ve `to` sorgusuyla çalıştıracağız, bu da
_to_ kelimesini tüm küçük harflerle içeren herhangi bir satırla eşleşmelidir:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

Görünüşe göre hala çalışıyor! Şimdi programı `IGNORE_CASE`
ayarını `1` olarak değiştirerek ama aynı _to_ sorgusuyla çalıştıralım:

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

PowerShell kullanıyorsanız, ortam değişkenini ayarlamanız ve
programını ayrı komutlar olarak çalıştırmanız gerekecektir:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

Bu, `IGNORE_CASE` öğesinin kabuk oturumunuzun geri kalanı boyunca kalıcı olmasını sağlayacaktır.
Bu ayar `Remove-Item` cmdlet`i ile kaldırılabilir:

```console
PS> Remove-Item Env:IGNORE_CASE
```

Büyük harfli olabilecek _to_ içeren satırlar almalıyız:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- to poem.txt
can't extract because of the environment variable
-->

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Mükemmel, _To_ içeren satırlarımız da var! Artık `minigrep` programımız bir ortam değişkeni tarafından kontrol edilen
büyük/küçük harfe duyarsız arama yapabilmektedir. Artık
komut satırı argümanları ya da ortam
değişkenleri kullanılarak ayarlanan seçeneklerin nasıl yönetileceğini biliyorsunuz.

Bazı programlar aynı
yapılandırması için argümanlara _ve_ ortam değişkenlerine izin verir. Bu durumlarda, programlar birinin ya da diğerinin
öncelikli olduğuna karar verir. Kendi başınıza başka bir alıştırma yapmak için,
büyük/küçük harf duyarlılığını bir komut satırı argümanı ya da bir ortam değişkeni aracılığıyla kontrol etmeyi deneyin. Program biri büyük/küçük harfe duyarlı diğeri
büyük/küçük harfi yok sayacak şekilde ayarlanmış olarak çalıştırılırsa
komut satırı argümanının mı yoksa ortam değişkeninin mi
öncelikli olacağına karar verin.

std::env` modülü,
ortam değişkenleri ile başa çıkmak için daha birçok yararlı özellik içerir: nelerin mevcut olduğunu görmek için belgelerine göz atın.
