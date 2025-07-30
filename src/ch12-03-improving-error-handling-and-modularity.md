## Modülerliği ve Hata İşlemeyi İyileştirmek için Yeniden Yapılandırma

Programımızı iyileştirmek için, programın yapısı ve potansiyel hataları nasıl
işlediği ile ilgili dört sorunu çözeceğiz. İlk olarak, `main`
fonksiyonumuz artık iki görevi yerine getiriyor: argümanları ayrıştırıyor ve dosyaları okuyor. Programımız büyüdükçe, `main`
fonksiyonunun yerine getirdiği ayrı görevlerin sayısı da
artacak. Bir işlevin sorumlulukları arttıkça, onu anlamak, test etmek ve bir parçasını bozmadan değiştirmek
daha zor hale gelir.
En iyisi, işlevleri ayırarak her işlevin tek bir görevden sorumlu olmasını
sağlamaktır.

Bu sorun, ikinci sorunla da bağlantılıdır: `query` ve `file_path`
programımızın yapılandırma değişkenleri olsa da, `contents` gibi değişkenler
programın mantığını gerçekleştirmek için kullanılır. `main` ne kadar uzarsa, o kadar çok değişkeni
kapsama alanına almamız gerekir; kapsama alanında ne kadar çok değişken varsa, her birinin amacını takip etmek o kadar zor
olur. Yapılandırma değişkenlerini tek bir yapı altında gruplandırarak amaçlarını netleştirmek en iyisidir.

Üçüncü sorun, dosya okunamadığında hata mesajı yazdırmak için `expect` komutunu kullanmış olmamızdır, ancak hata mesajı sadece `Dosya okunabilmeliydi` yazdırmaktadır.
Dosya okunamaması birçok nedenden kaynaklanabilir: örneğin, dosya eksik olabilir veya dosyayı açma iznimiz olmayabilir.
Şu anda, durum ne olursa olsun, her şey için aynı hata mesajını yazdırıyoruz, bu da kullanıcıya hiçbir bilgi vermiyor!
Dördüncü olarak, bir hatayı işlemek için `expect` komutunu kullanıyoruz ve kullanıcı hata mesajını görmeden önce komutun sonuna kadar ilerliyoruz.
Şu anda, durum ne olursa olsun, her şey için aynı hata mesajını yazdırıyoruz,
bu da kullanıcıya hiçbir bilgi vermiyor!

Dördüncü olarak, bir hatayı işlemek için `expect` kullanıyoruz ve kullanıcı programımızı
yeterli argüman belirtmeden çalıştırırsa, Rust'tan sorunu açıkça açıklamayan bir `index out of bounds` hatası
alır. En iyisi, tüm
hata işleme kodunun tek bir yerde olmasıdır, böylece gelecekteki bakımcılar, hata işleme mantığının değiştirilmesi gerektiğinde
kodu tek bir yerden inceleyebilirler. Tüm
hata işleme kodunun tek bir yerde olması, son kullanıcılar için anlamlı mesajlar
yazdırdığımızdan da emin olmamızı sağlar.

Projemizi yeniden düzenleyerek bu dört sorunu ele alalım.

### İkili Projeler için Sorunların Ayrılması

Birden fazla görevin sorumluluğunu `main` işlevine atama konusundaki
organizasyonel sorun, birçok ikili projede yaygın olarak görülür. Sonuç olarak, birçok Rust
programcısı, `main` işlevi büyümeye başladığında ikili programın ayrı sorunlarını
ayırmanın yararlı olduğunu düşünür. Bu süreç aşağıdaki adımları içerir:
- Programınızı _main.rs_ dosyası ve _lib.rs_ dosyası olarak ikiye ayırın ve

- Programınızı _main.rs_ dosyası ve _lib.rs_ dosyası olarak bölün ve
  programınızın mantığını _lib.rs_ dosyasına taşıyın.
- Komut satırı ayrıştırma mantığınız küçük olduğu sürece,
  `main` işlevinde kalabilir.
- Komut satırı ayrıştırma mantığı karmaşıklaşmaya başladığında, onu
  `main` işlevinden diğer işlevlere veya türlere ayırın.

Bu işlemden sonra `main` işlevinde kalan sorumluluklar
aşağıdakilerle sınırlı olmalıdır:

- Komut satırı ayrıştırma mantığını argüman değerleriyle çağırmak
- Diğer yapılandırmaları ayarlamak
- _lib.rs_ içindeki `run` işlevini çağırmak
- `run` bir hata döndürdüğünde hatayı işlemek

Bu model, endişeleri ayırmakla ilgilidir: _main.rs_ programı çalıştırmayı
yönetir ve _lib.rs_ eldeki görevin tüm mantığını yönetir. `main` işlevini
doğrudan test edemeyeceğiniz için, bu yapı, programı `main` işlevinden
çıkararak tüm mantığını test etmenizi sağlar. `main` işlevinde kalan kod,
okuyarak doğruluğunu kontrol edebilecek kadar küçük olacaktır. Bu süreci takip ederek programımızı yeniden düzenleyelim.

#### Argüman Ayrıştırıcısını Çıkarma

Argümanları ayrıştırma işlevini, `main` işlevinin çağıracağı bir işlev olarak
ayrıştıracağız. Listing 12-5, _src/main.rs_ dosyasında tanımlayacağımız yeni `parse_config` işlevini çağıran `main` işlevinin yeni başlangıcını göstermektedir.

<Listing number="12-5" file-name="src/main.rs" caption="Extracting a `parse_config` function from `main`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

Hala komut satırı argümanlarını bir vektöre topluyoruz, ancak
`main` fonksiyonu içinde 1. indekste bulunan argüman değerini `query`
değişkenine ve 2. indekste bulunan argüman değerini `file_path`
değişkenine atamak yerine, tüm vektörü `parse_config` fonksiyonuna aktarıyoruz.
`parse_config` işlevi, hangi argümanın hangi değişkene gideceğini belirleyen mantığı barındırır ve
değerleri `main` işlevine geri aktarır. Hala `main` işlevinde `query` ve `file_path`
değişkenlerini oluşturuyoruz, ancak `main` artık komut satırı argümanları ve değişkenlerin
nasıl eşleştiğini belirleme sorumluluğuna sahip değil.
Bu yeniden düzenleme, küçük programımız için aşırıya kaçmış gibi görünebilir, ancak

Bu yeniden çalışma, küçük programımız için aşırı gibi görünebilir, ancak biz küçük, aşamalı adımlarla yeniden düzenleme yapıyoruz.
Bu değişikliği yaptıktan sonra, programı tekrar çalıştırarak
argüman ayrıştırmanın hala çalıştığını doğrulayın. Sorunlar ortaya çıktığında bunların nedenini belirlemek için ilerlemenizi
sık sık kontrol etmek iyidir.

#### Yapılandırma Değerlerini Gruplama

`parse_config` işlevini daha da iyileştirmek için küçük bir adım daha atabiliriz.
Şu anda bir tuple döndürüyoruz, ancak hemen ardından bu tuple'ı tekrar tek tek parçalara ayırıyoruz.
Bu, belki de henüz doğru soyutlamaya sahip olmadığımızın bir işaretidir.

İyileştirme için alan olduğunu gösteren bir başka gösterge de `parse_config`
işlevinin `config` kısmıdır. Bu, döndürdüğümüz iki değerin birbiriyle ilişkili
olduğunu ve her ikisinin de tek bir yapılandırma değerinin parçası olduğunu
ima eder. Şu anda bu anlamı, iki değeri bir tuple içinde gruplandırmak
dışında veri yapısında aktarmıyoruz; bunun yerine iki değeri tek bir yapıya
koyacağız ve her bir yapı alanına anlamlı bir ad vereceğiz. Böylece, bu kodun gelecekteki
bakımcıları, farklı değerlerin birbirleriyle nasıl ilişkili olduğunu ve amaçlarının
ne olduğunu daha kolay anlayabilecekler.

Listing 12-6, `parse_config` işlevindeki iyileştirmeleri göstermektedir.

<Listing number="12-6" file-name="src/main.rs" caption="Refactoring `parse_config` to return an instance of a `Config` struct">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

`query` ve `file_path` adlı alanlara sahip olacak şekilde tanımlanmış `Config` adlı bir yapı ekledik.
`parse_config` işlevinin imzası artık bir `Config` değeri döndürdüğünü
belirtir. `parse_config` işlevinin gövdesinde, eskiden `args` içindeki `String` değerlerine
atıfta bulunan string dilimleri döndürdüğümüz yerde, artık `Config`'i
sahip olunan `String` değerlerini içerecek şekilde tanımlıyoruz. `main` içindeki `args` değişkeni,
argüman değerlerinin sahibidir ve `parse_config` işlevinin bunları yalnızca ödünç almasına izin verir.
Bu, `Config`, `args` içindeki değerlerin sahipliğini almaya çalışırsa,
Rust'un ödünç alma kurallarını ihlal edeceğimiz anlamına gelir.

`String` verilerini yönetmenin birkaç yolu vardır; en kolay,
ancak biraz verimsiz olan yol, değerler üzerinde `clone` yöntemini çağırmaktır.
Bu, `Config` örneğinin sahip olacağı verilerin tam bir kopyasını oluşturur, bu da
dize verilerine bir referans depolamaktan daha fazla zaman ve bellek gerektirir.
Ancak, verileri klonlamak kodumuzu çok basit hale getirir, çünkü
referansların ömürlerini yönetmemiz gerekmez; bu durumda,
basitlik kazanmak için biraz performanstan vazgeçmek değerli bir ödünleşmedir.

> ### `clone` Kullanmanın Avantajları ve Dezavantajları
>
> Birçok Rustacean, çalışma zamanı maliyeti nedeniyle sahiplik sorunlarını gidermek için `clone` kullanmaktan kaçınma eğilimindedir.
> [Bölüm 13][ch13]<!-- ignore -->, bu tür durumlarda daha verimli
> yöntemleri nasıl kullanacağınızı öğreneceksiniz.
> Ancak şimdilik, ilerlemeyi sürdürmek için birkaç
> dizeyi kopyalamak sorun değildir, çünkü bu kopyaları yalnızca
> bir kez yapacaksınız ve dosya yolunuz ve sorgu dizeniz çok küçüktür. İlk denemenizde kodu aşırı optimize etmeye çalışmaktansa,
> biraz verimsiz olsa da çalışan bir programa sahip olmak daha iyidir.
> Rust konusunda daha deneyimli hale geldikçe, en verimli çözümle başlamak
> daha kolay hale gelecektir, ancak şimdilik `clone` işlevini çağırmak
> tamamen kabul edilebilir bir durumdur.

`main`'i güncelledik, böylece `parse_config` tarafından döndürülen `Config` örneğini
`config` adlı bir değişkene yerleştiriyor ve daha önce ayrı `query` ve `file_path` değişkenlerini kullanan kodu güncelledik, böylece artık
`Config` yapısının alanlarını kullanıyor.
Şimdi kodumuz, `query` ve `file_path`'in ilişkili olduğunu ve

Artık kodumuz, `query` ve `file_path`'in birbiriyle ilişkili olduğunu ve
amaçlarının programın nasıl çalışacağını yapılandırmak olduğunu daha açık bir şekilde ifade ediyor. Bu değerleri kullanan herhangi bir kod,
bu değerleri `config` örneğinde, amaçlarına göre adlandırılmış alanlarda
bulabilir.

#### `Config` için bir Yapıcı Oluşturma

Şimdiye kadar, komut satırı argümanlarını ayrıştırmaktan sorumlu mantığı
`main` işlevinden ayırdık ve `parse_config` işlevine yerleştirdik. Bu,
`query` ve `file_path` değerlerinin birbiriyle ilişkili olduğunu ve bu
ilişkinin kodumuzda ifade edilmesi gerektiğini anlamamıza yardımcı oldu. Daha sonra, `query` ve `file_path`'in ilgili amacını adlandırmak ve
`parse_config` işlevinden değerlerin adlarını yapı alanı adları olarak döndürebilmek için
bir `Config` yapısı ekledik.

Artık `parse_config` işlevinin amacı bir `Config`
örneği oluşturmak olduğuna göre, `parse_config` işlevini düz bir işlevden, `Config`
yapısı ile ilişkili `new` adlı bir işlev haline getirebiliriz. Bu değişikliği yapmak,
kodu daha doğal hale getirecektir. `String` gibi standart kütüphanedeki türlerin örneklerini
`String::new` işlevini çağırarak oluşturabiliriz. Benzer şekilde,
`parse_config` işlevini `Config` ile ilişkili bir `new` işlevine dönüştürerek,
`Config::new` işlevini çağırarak `Config` örnekleri oluşturabiliriz. Listing 12-7
yapmamız gereken değişiklikleri göstermektedir.

<Listing number="12-7" file-name="src/main.rs" caption="Changing `parse_config` into `Config::new`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

`parse_config`'i çağırdığımız `main`'i, bunun yerine
`Config::new`'i çağırmak üzere güncelledik. `parse_config`'in adını `new` olarak değiştirdik ve bunu
`new` işlevini `Config` ile ilişkilendiren bir `impl` bloğu içine taşıdık. Çalıştığından emin olmak için
bu kodu tekrar derlemeyi deneyin.

### Hata İşlemeyi Düzeltme

Şimdi hata işlemeyi düzeltmeye çalışacağız. Hatırlayın, `args` vektöründeki
1 veya 2 indeksindeki değerlere erişmeye çalışmak, vektör üçten az öğe içeriyorsa
programın paniklemesine neden olur. Programı herhangi bir argüman olmadan çalıştırmayı deneyin;
şöyle görünecektir:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

`index out of bounds: the len is 1 but the index is 1` satırı, programcılar için
hazırlanmış bir hata mesajıdır. Bu mesaj, son kullanıcıların ne yapmaları gerektiğini
anlamalarına yardımcı olmaz. Şimdi bunu düzeltelim.

#### Hata Mesajını İyileştirme

Listing 12-8'de, `new` işlevine, indeks 1 ve indeks 2'ye erişmeden önce dilimin
yeterince uzun olup olmadığını doğrulayan bir kontrol ekliyoruz. Dilim yeterince uzun değilse,
program panik yapar ve daha iyi bir hata mesajı görüntüler.

<Listing number="12-8" file-name="src/main.rs" caption="Adding a check for the number of arguments">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

Bu kod, [Listing 9-13'te yazdığımız `Guess::new` işlevine benzer][ch9-custom-types]<!-- ignore -->, burada
`value` argümanı geçerli değer aralığının dışındaysa `panic!` çağrısı yapıyoruz. Burada değer aralığını kontrol etmek yerine,
`args` uzunluğunun en az
`3` olduğundan emin oluyoruz ve fonksiyonun geri kalanı bu
koşulun sağlandığını varsayarak çalışabilir. Eğer `args` üçten az öğe içeriyorsa, bu koşul
`true` olur ve programı hemen sonlandırmak için `panic!` makrosunu çağırırız.

`new` içindeki bu birkaç satırlık ek kodla, programı herhangi bir
argüman olmadan tekrar çalıştırarak hatanın şimdi nasıl göründüğüne bakalım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

Bu çıktı daha iyidir: artık makul bir hata mesajımız var. Ancak,
kullanıcılarımıza vermek istemediğimiz gereksiz bilgiler de var. Belki de
Listing 9-13'te kullandığımız teknik burada kullanmak için en iyisi değildir:
`panic!` çağrısı, kullanım sorunundan çok programlama sorunu için daha uygundur,
[Bölüm 9'da tartışıldığı gibi][ch9-error-guidelines]<!-- ignore -->. Bunun yerine,
Bölüm 9'da öğrendiğiniz diğer tekniği kullanacağız—[`Result` döndürmek][ch9-result]<!-- ignore --> bu, başarıyı veya birerror.

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### `panic!` Çağırmak Yerine `Result` Döndürmek

Bunun yerine, başarılı durumda bir `Config` örneği içeren ve hata durumunda sorunu açıklayan bir `Result` değeri döndürebiliriz. Ayrıca, birçok programcı `new` işlevlerinin asla başarısız olmayacağını beklediği için işlev adını `new`'den `build`'e değiştireceğiz.

`Config::build` işlevinin `main` ile iletişim kurduğunda, bir sorun olduğunu belirtmek için `Result` türünü kullanabiliriz. Ardından `main`'i değiştirerek bir `Err` varyantını daha açıklayıcı bir `Result` türüne dönüştürebiliriz. `Config::build`, `main` ile
iletişim kurarken, bir sorun olduğunu belirtmek için `Result` türünü kullanabiliriz.
Ardından, `main`'i değiştirerek `Err` varyantını, `panic!` çağrısının neden olduğu
`thread ‘main’` ve `RUST_BACKTRACE` ile ilgili metinler olmadan, kullanıcılarımız için
daha pratik bir hataya dönüştürebiliriz.

Listing 12-9, şu anda `Config::build` olarak adlandırdığımız fonksiyonun dönüş değerinde
ve `Result` döndürmek için gerekli olan fonksiyon gövdesinde yapmamız gereken değişiklikleri
göstermektedir. `main`'i de güncelleyene kadar bunun derlenmeyeceğini
unutmayın; bunu bir sonraki listede yapacağız.

<Listing number="12-9" file-name="src/main.rs" caption="Returning a `Result` from `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

`build` işlevimiz, başarılı durumda bir `Config` örneği içeren bir `Result` ve
hata durumunda bir string literal döndürür. Hata değerlerimiz her zaman
`'static` ömrü olan string literalleri olacaktır.

İşlevin gövdesinde iki değişiklik yaptık: kullanıcı yeterli argüman geçmediğinde `panic!`
çağırmak yerine, artık bir `Err` değeri döndürüyoruz ve
`Config` dönüş değerini bir `Ok` içine sardık. Bu değişiklikler,
işlevin yeni tür imzasına uymasını sağlar.

`Config::build` işlevinden bir `Err` değeri döndürmek, `main` işlevinin
`build` işlevinden döndürülen `Result` değerini işlemesine ve hata durumunda
işlemi daha temiz bir şekilde sonlandırmasına olanak tanır.

<!-- Old headings. Do not remove or links may break. -->

<a id="calling-confignew-and-handling-errors"></a>

#### `Config::build`'u çağırma ve hataları işleme

Hata durumunu işlemek ve kullanıcı dostu bir mesaj yazdırmak için,
`Config::build` tarafından döndürülen `Result`'u işlemek üzere
`main`'i Listing 12-10'da gösterildiği gibi güncellememiz gerekir. Ayrıca, sıfır olmayan bir hata koduyla komut satırı
aracından çıkma sorumluluğunu `panic!`'den alıp bunun yerine
elle uygulayacağız. Sıfır olmayan bir çıkış durumu, programımızı çağıran sürece
programın bir hata durumuyla çıktığını bildirmek için kullanılan bir kuraldır.

<Listing number="12-10" file-name="src/main.rs" caption="Exiting with an error code if building a `Config` fails">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

Bu listede, henüz ayrıntılı olarak ele almadığımız bir yöntem kullandık:
`unwrap_or_else`, standart kütüphane tarafından `Result<T, E>` üzerinde tanımlanmıştır.
`unwrap_or_else` kullanarak, bazı özel, `panic!` olmayan hata
işleme tanımlayabiliriz. `Result` bir `Ok` değeri ise, bu yöntemin davranışı
`unwrap` ile benzerdir: `Ok`'un sardığı iç değeri döndürür. Ancak,
değer bir `Err` değeri ise, bu yöntem _closure_ içindeki kodu çağırır; bu,
tanımladığımız ve `unwrap_or_else`'ye argüman olarak geçirdiğimiz anonim bir işlevdir.
Kapanışları [Bölüm 13][ch13]<!-- ignore -->. Daha ayrıntılı olarak ele alacağız.
Şu anda, `unwrap_or_else`'nin `Err`'nin iç değerini geçireceğini bilmeniz yeterlidir.
Bu durumda, bu değer, Listing 12-9'da eklediğimiz statik dize `“not enough arguments”`
, dikey borular arasında görünen `err` argümanındaki kapatmamıza aktarır. Kapatmadaki kod,
çalıştığında `err` değerini kullanabilir.
Standart kütüphaneden `process`'i kapsam içine almak için yeni bir `use` satırı ekledik.

Standart kütüphaneden `process` işlevini kapsam içine almak için yeni bir `use` satırı ekledik.
Hata durumunda çalıştırılacak kapatma içindeki kod sadece iki satırdan oluşuyor:
`err` değerini yazdırıyoruz ve ardından `process::exit` işlevini çağırıyoruz.
`process::exit` işlevi programı hemen durdurur ve çıkış durum kodu olarak
geçirilen sayıyı döndürür. Bu, Listing 12-8'de kullandığımız
`panic!` tabanlı işleme benzer, ancak artık tüm
ekstra çıktıları almıyoruz. Deneyelim:,

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

Harika! Bu çıktı kullanıcılarımız için çok daha dostça.

<!-- Old headings. Do not remove or links may break. -->

<a id="extracting-logic-from-main"></a>

### `main` Fonksiyonundan Mantık Çıkarma

Yapılandırma ayrıştırmasını yeniden düzenlemeyi bitirdiğimize göre, şimdi programın mantığına
dönelim. [İkili Projeler için Endişelerin Ayrılması](#i̇kili-projeler-için-sorunların-ayrılması)<!-- ignore --> bölümünde belirttiğimiz gibi,
adresinden `run` adında bir fonksiyon çıkaracağız ve bu fonksiyon şu anda
`main` fonksiyonunda bulunan ve yapılandırmayı ayarlamak ya da
hatalarını ele almakla ilgili olmayan tüm mantığı tutacak. İşimiz bittiğinde, `main` fonksiyonu kısa ve
adresini kontrol ederek doğrulaması kolay olacak ve diğer tüm mantık için testler yazabileceğiz.

Liste 12-11, bir `run`
işlevinin çıkarılmasının küçük, artımlı iyileştirmesini göstermektedir.


<Listing number="12-11" file-name="src/main.rs" caption="Extracting a `run` function containing the rest of the program logic">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

Şimdi `run` fonksiyonu,
adresinden dosyayı okumaya başlayarak `main` fonksiyonundan kalan tüm mantığı içerir. Run` fonksiyonu,
argümanı olarak `Config` örneğini alır.

#### `run` Fonksiyonundan Hata Döndürme

Kalan program mantığının `run` fonksiyonuna ayrılmasıyla, Listing 12-9'daki `Config::build` fonksiyonunda yaptığımız gibi,
hata işlemeyi geliştirebiliriz.
Programın `expect` çağrısı yaparak paniklemesine izin vermek yerine, `run`
fonksiyonu bir şeyler ters gittiğinde bir `Result<T, E>` döndürecektir. Bu,
hataları ele alma mantığını
kullanıcı dostu bir şekilde `main` içinde daha da birleştirmemize izin verecektir. Liste 12-12,
imzasında ve `run` gövdesinde yapmamız gereken değişiklikleri göstermektedir.



<Listing number="12-12" file-name="src/main.rs" caption="Changing the `run` function to return `Result`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

Burada üç önemli değişiklik yaptık. İlk olarak,
`run` fonksiyonunun dönüş tipini `Result<(), Box<dyn Error>>` olarak değiştirdik. Bu fonksiyon daha önce
birim türünü, `()` döndürüyordu ve bunu
`Ok` durumunda döndürülen değer olarak tutuyoruz.

Hata türü için _trait nesnesi_ `Box<dyn Error>` kullandık (ve
adresinde `std::error::Error` ifadesini en üstte bir `use` ifadesiyle kapsama aldık).
Trait nesnelerini [Bölüm 18][ch18]<!-- ignore -->'de ele alacağız. Şimdilik, sadece
`Box<dyn Error>` fonksiyonunun
`Error` özelliğini uygulayan bir tip döndüreceği anlamına geldiğini bilin, ancak dönüş değerinin hangi tip
olacağını belirtmek zorunda değiliz. Bu bize
farklı hata durumlarında farklı tiplerde olabilecek hata değerleri döndürme esnekliği sağlar. dyn` anahtar sözcüğü _dynamic_ için kısa bir
adresidir.

İkinci olarak,
adresinde [Bölüm 9][ch9-question-mark]<!-- ignore --> bölümünde bahsettiğimiz gibi, `?` operatörü lehine `expect` çağrısını kaldırdık. Bir hatada
`panic!` yerine, `?`, çağıranın işlemesi için
geçerli işlevden hata değerini döndürecektir.

Üçüncü olarak, `run` fonksiyonu artık başarı durumunda bir `Ok` değeri döndürmektedir.
İmzada `run` fonksiyonunun başarı tipini `()` olarak bildirdik,
bu da birim tip değerini `Ok` değerine sarmamız gerektiği anlamına geliyor. Bu
`Ok(())` sözdizimi ilk başta biraz garip görünebilir, ancak `()` değerini bu şekilde kullanmak
`run` fonksiyonunu yalnızca yan etkileri
için çağırdığımızı belirtmenin deyimsel yoludur; ihtiyacımız olan bir değer döndürmez.

Bu kodu çalıştırdığınızda derlenecek ancak bir uyarı görüntülenecektir:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust bize kodumuzun `Result` değerini göz ardı ettiğini ve
adresindeki `Result` değerinin bir hata oluştuğunu gösterebileceğini söylüyor. Ancak
bir hata olup olmadığını kontrol etmiyoruz ve derleyici bize
muhtemelen burada bazı hata işleme kodlarına sahip olmamız gerektiğini hatırlatıyor! Şimdi bu sorunu düzeltelim.

#### `main` içinde `run`dan Dönen Hataların İşlenmesi

Hataları kontrol edeceğiz ve Liste 12-10'da `Config::build` ile
kullandığımıza benzer bir teknik kullanarak ele alacağız, ancak küçük bir farkla

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

run` fonksiyonunun
`Err` değeri döndürüp döndürmediğini kontrol etmek ve döndürürse `process::exit(1)` fonksiyonunu çağırmak için `unwrap_or_else` yerine `if let` kullanıyoruz. `run` fonksiyonu,
`Config::build` fonksiyonunun `Config` örneğini döndürdüğü gibi `unwrap` yapmak istediğimiz bir değer döndürmez. Çünkü `run` fonksiyonu
adresinde `()` değerini döndürdüğünden, biz sadece bir hatayı tespit etmekle ilgileniyoruz, bu yüzden
 adresinde `unwrap_or_else` fonksiyonunun sadece `()` değerini döndürmesine gerek yok.

if let` ve `unwrap_or_else` fonksiyonlarının gövdeleri
her iki durumda da aynıdır: hatayı yazdırır ve çıkarız.

### Kodu Kütüphane Kasasına Bölme

minigrep` projemiz şu ana kadar iyi görünüyor! Şimdi
_src/main.rs_ dosyasını böleceğiz ve _src/lib.rs_ dosyasına bazı kodlar koyacağız. Bu şekilde,
kodu test edebilir ve daha az sorumluluğu olan bir _src/main.rs_ dosyasına sahip olabiliriz.

Metni aramaktan sorumlu kodu _src/main.rs_ yerine _src/lib.rs_ dosyasında tanımlayalım, böylece biz (veya
`minigrep` kütüphanemizi kullanan herhangi biri) arama fonksiyonunu
`minigrep` ikilimizden daha fazla bağlamdan çağırabilelim.

İlk olarak, _src/lib.rs_ dosyasındaki `search` fonksiyon imzasını
Liste 12-13'de gösterildiği gibi, `unimplemented!` makrosunu çağıran bir gövde ile tanımlayalım. Uygulamayı doldurduğumuzda imzayı
daha ayrıntılı olarak açıklayacağız.

<Listing number="12-13" file-name="src/lib.rs" caption="Defining the `search` function in  *src/lib.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

</Listing>

İşlev tanımında `pub` anahtar sözcüğünü kullanarak `search`
işlevini kütüphane sandığımızın genel API'sinin bir parçası olarak belirledik. Artık
adresinden binary crate'imizden kullanabileceğimiz ve test edebileceğimiz bir kütüphane crate'imiz var!

Şimdi _src/lib.rs_ içinde tanımlanan kodu _src/main.rs_ içindeki
binary crate'in kapsamına getirmemiz ve Listing 12-14'te gösterildiği gibi çağırmamız gerekiyor.

<Listing number="12-14" file-name="src/main.rs" caption="Using the `minigrep` library crate’s `search` function in *src/main.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

adresindeki `search` fonksiyonunu kütüphane crate'inden ikili crate'in kapsamına getirmek için bir `use minigrep::search` satırı ekliyoruz. Ardından, `run` fonksiyonunda,
dosyanın içeriğini yazdırmak yerine, `search`
fonksiyonunu çağırıyoruz ve `config.query` değerini ve `contents` değerini argüman olarak iletiyoruz. Daha sonra
`run`,
sorguyla eşleşen `search` fonksiyonundan dönen her satırı yazdırmak için bir `for` döngüsü kullanacaktır. Bu aynı zamanda
adresindeki sorguyu ve dosya yolunu görüntüleyen `main` fonksiyonundaki `println!` çağrılarını kaldırmak için iyi bir zamandır, böylece
programımız yalnızca arama sonuçlarını yazdırır (herhangi bir hata oluşmazsa).

Arama fonksiyonunun tüm sonuçları bir vektörde toplayacağını unutmayın
herhangi bir yazdırma gerçekleşmeden önce döner. Bu uygulama büyük dosyaları ararken sonuçları
görüntülemek için yavaş olabilir çünkü sonuçlar bulundukları anda
olarak yazdırılmaz; bunu yineleyiciler kullanarak düzeltmenin olası bir yolunu
Bölüm 13'te tartışacağız.

Vay be! Bu çok fazla işti, ancak kendimizi
geleceğinde başarı için hazırladık. Artık hataları ele almak çok daha kolay ve kodu daha
modüler hale getirdik. Bundan sonra neredeyse tüm işlerimiz _src/lib.rs_ içinde yapılacak.

Bu yeni modülerlikten yararlanarak
eski kodla zor olan ancak yeni kodla kolay olan bir şey yapalım:
bazı testler yazacağız!

[ch13]: ch13-00-functional-features.md
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.md#doğrulama-için-özel-tipler-oluşturma
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.md#derleyiciden-daha-fazla-bilgiye-sahip-olduğunuz-durumlar
[ch9-result]: ch09-02-recoverable-errors-with-result.md
[ch18]: ch18-00-oop.md
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.md#hataları-yaymak-için-bir-kısayol--operatörü
