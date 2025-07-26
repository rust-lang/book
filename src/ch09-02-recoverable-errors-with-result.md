## `Result` ile Kurtarılabilir Hatalar

Çoğu hata, programın tamamen durmasını gerektirecek kadar ciddi değildir.
Bazen bir işlev başarısız olduğunda,
adresini kolayca yorumlayabileceğiniz ve yanıt verebileceğiniz bir nedeni vardır. Örneğin, bir dosyayı açmaya çalışırsanız ve bu işlem dosya mevcut olmadığı için
başarısız olursa,
işlemi sonlandırmak yerine dosyayı oluşturmak isteyebilirsiniz.

Bölüm 2'deki [“Potansiyel Hatayı `Result ile İşleme`”][handle_failure]<!--
ignore --> kısmından `Result` enum'unun aşağıdaki gibi iki
varyantına, `Ok` ve `Err`, sahip olarak tanımlandığını hatırlayın:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` ve `E` genel tip parametreleridir: genel tipleri Bölüm 10'da daha ayrıntılı olarak
tartışacağız. Şu anda bilmeniz gereken şey, `T` parametresinin
adresinde `Ok`
değişkeninde bir başarı durumunda döndürülecek değerin türünü temsil ettiği ve `E` parametresinin de
adresinde `Err` değişkeninde bir başarısızlık durumunda döndürülecek hatanın türünü temsil ettiğidir. `Result` türü bu genel
parametrelerine sahip olduğundan,
döndürmek istediğimiz başarı değeri ile hata değerinin farklı olabileceği birçok farklı durumda
içinde `Result` türünü ve üzerinde tanımlanan fonksiyonları kullanabiliriz.

Bir `Result` değeri döndüren bir fonksiyon çağıralım çünkü fonksiyon
başarısız olabilir. Listing 9-3`te bir dosya açmaya çalışıyoruz.

<Listing number="9-3" file-name="src/main.rs" caption="Opening a file">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

</Listing>

`File::open`ın dönüş tipi bir `Result<T, E>`dir. `T`
genel parametresi,
başarı değerinin türü olan `std::fs::File` ile `File::open` uygulaması tarafından doldurulmuştur, bu da bir dosya tanıtıcısıdır. Hata değerinin
adresinde kullanılan `E` tipi `std::io::Error`dur. Bu dönüş türü,
`File::open` çağrısının başarılı olabileceği ve okuyabileceğimiz veya
yazabileceğimiz bir dosya tanıtıcısı döndürebileceği anlamına gelir. Fonksiyon çağrısı başarısız da olabilir: örneğin, dosya
mevcut olmayabilir veya dosyaya erişim iznimiz olmayabilir. `File::open`
fonksiyonunun bize başarılı ya da başarısız olduğunu söyleyecek ve
aynı zamanda bize dosya tanıtıcısını ya da hata bilgisini verecek bir yolu olması gerekir. Bu
bilgisi tam olarak `Result` enumunun ilettiği şeydir.

`File::open`ın başarılı olduğu durumda,
`greeting_file_result` değişkenindeki değer, bir dosya tanıtıcısı içeren bir `Ok` örneği olacaktır.
Başarısız olduğu durumda, `greeting_file_result` değişkenindeki değer,
adresinde meydana gelen hata türü hakkında daha fazla bilgi içeren bir
`Err` örneği olacaktır.

`File::open`ın döndürdüğü değere
bağlı olarak farklı eylemler gerçekleştirmek için Listing 9-3'teki koda ekleme yapmamız gerekir. Liste 9-4,
Bölüm 6'da tartıştığımız temel bir araç olan `match` ifadesini kullanarak
`Result` ile başa çıkmanın bir yolunu göstermektedir.

<Listing number="9-4" file-name="src/main.rs" caption="Using a `match` expression to handle the `Result` variants that might be returned">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

</Listing>

Tıpkı `Option` enum'u gibi, `Result` enum'u ve varyantlarının da
prelude tarafından kapsam içine alındığını, dolayısıyla `match` kollarındaki `Ok` ve `Err` varyantlarından önce `Result::`
belirtmemize gerek olmadığını unutmayın.

Sonuç `Ok` olduğunda, bu kod iç `dosya` değerini
`Ok` varyantından döndürür ve daha sonra bu dosya tanıtıcısı değerini
`greeting_file` değişkenine atarız. Eşleşmeden sonra, dosya tanıtıcısını okuma veya
yazma için kullanabiliriz.

Eşleşmenin diğer kolu,
`File::open` adresinden bir `Err` değeri aldığımız durumu ele alır. Bu örnekte, `panic!` makrosunu çağırmayı seçtik. Eğer
geçerli dizinimizde _hello.txt_ adında bir dosya yoksa ve bu
kodunu çalıştırırsak, `panic!` makrosundan aşağıdaki çıktıyı görürüz:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Her zamanki gibi, bu çıktı bize tam olarak neyin yanlış gittiğini söyler.

### Farklı Hatalarda Eşleştirme

Listing 9-4'teki kod, `File::open` neden başarısız olursa olsun `panic!`
Ancak, farklı başarısızlık nedenleri için farklı eylemler gerçekleştirmek istiyoruz. Eğer
`File::open` dosya mevcut olmadığı için başarısız olduysa,
dosyasını oluşturmak ve yeni dosyanın tanıtıcısını döndürmek istiyoruz. Eğer `File::open` başka bir
nedenden dolayı başarısız olduysa - örneğin, dosyayı açma iznimiz olmadığı için - yine de
kodun Listing 9-4'te olduğu gibi `panic!` Bunun için,
Liste 9-5'te gösterilen bir iç `match` ifadesi ekleriz.

<Listing number="9-5" file-name="src/main.rs" caption="Handling different kinds of errors in different ways">

<!-- bu testi göz ardı edin çünkü aksi takdirde hello.txt dosyasını oluşturur ve bu da diğer
testlerinin başarısız olmasına neden olur haha -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

</Listing>

`File::open`ın `Err` değişkeni içinde döndürdüğü değerin türü, standart kütüphane tarafından sağlanan bir struct olan
`io::Error`dur. Bu struct
, bir `io::ErrorKind` değeri elde etmek için çağırabileceğimiz bir `kind` yöntemine sahiptir. `io::ErrorKind` enumu standart kütüphane tarafından sağlanır ve bir `io`
işleminden kaynaklanabilecek farklı hata türlerini temsil eden
varyantlarına sahiptir. Kullanmak istediğimiz değişken,
açmaya çalıştığımız dosyanın henüz mevcut olmadığını gösteren `ErrorKind::NotFound`dur. Bu yüzden
`greeting_file_result` ile eşleşiyoruz, ancak `error.kind()` ile de bir iç eşleşmemiz var.

İç eşleşmede kontrol etmek istediğimiz koşul, `error.kind()` tarafından
döndürülen değerin `ErrorKind` enumunun `NotFound` varyantı olup olmadığıdır. Eğer öyleyse
dosyayı `File::create` ile oluşturmaya çalışırız. Ancak, `File::create`
da başarısız olabileceğinden, iç `match` ifadesinde ikinci bir kola ihtiyacımız var. dosyası oluşturulamadığında, farklı bir hata mesajı yazdırılır. dış `match` ifadesinin ikinci kolu aynı kalır, böylece program
eksik dosya hatası dışında herhangi bir hatada panik yapar.

> #### `Result<T, E>` ile `match` Kullanmanın Alternatifleri
>
> Bu çok fazla `match``! match` ifadesi çok kullanışlıdır ama aynı zamanda çok
> ilkeldir. Bölüm 13'te, `Result<T, E>` üzerinde tanımlanan birçok yöntemle birlikte
> kullanılan closure'lar hakkında bilgi edineceksiniz. Bu yöntemler, kodunuzdaki `Result<T, E>` değerlerini işlerken `match` kullanmaktan daha
> özlü olabilir.
>
> Örneğin, Listing
> 9-5'te gösterilen aynı mantığı bu kez closure'ları ve `unwrap_or_else` yöntemini kullanarak yazmanın başka bir yolu:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {error:?}");
>             })
>         } else {
>             panic!("Problem opening the file: {error:?}");
>         }
>     });
> }
> ```
>
> Bu kod Liste 9-5 ile aynı davranışa sahip olmasına rağmen,
> herhangi bir `match` ifadesi içermez ve okunması daha temizdir. Bölüm 13'ü okuduktan sonra
> bu örneğe geri dönün ve
> standart kütüphane belgelerinde `unwrap_or_else` yöntemine bakın. Bu yöntemlerden çok daha fazlası, hatalarla uğraşırken büyük
> iç içe `match` ifadelerini temizleyebilir.

#### Hata Paniği için Kısayollar: `unwrap` ve `expect`

`Match` kullanımı yeterince iyi çalışır, ancak biraz ayrıntılı olabilir ve her zaman
iyi bir amaç iletmez. Sonuç`<T, E>` türünün çeşitli, daha spesifik görevleri yerine getirmek için üzerinde tanımlanmış
birçok yardımcı yöntemi vardır. `unwrap` yöntemi,
Liste 9-4'te yazdığımız `match` ifadesi gibi uygulanan bir
kısayol yöntemidir. Eğer `Result` değeri `Ok` varyantı ise, `unwrap` yöntemi
adresine `Ok` içindeki değeri döndürür. Eğer `Result` değeri `Err` ise, `unwrap`
bizim için `panic!` makrosunu çağıracaktır. İşte `unwrap` uygulamasının bir örneği:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

</Listing>

Bu kodu _hello.txt_ dosyası olmadan çalıştırırsak,
adresinde `unwrap` yönteminin yaptığı `panic!` çağrısından kaynaklanan bir hata mesajı görürüz:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Benzer şekilde, `expect` yöntemi de `panic!` hata mesajını seçmemizi sağlar.
`Unwrap` yerine `expect` kullanmak ve iyi hata mesajları sağlamak, niyetinizi
adresine iletebilir ve bir paniğin kaynağını izlemeyi kolaylaştırabilir. `expect` sözdizimi aşağıdaki gibidir:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

</Listing>

`expect`i `unwrap` ile aynı şekilde kullanırız: dosya tanıtıcısını döndürmek veya
adresini `panic!` makrosuna çağırmak için. `Expect` tarafından `panic!`
çağrısında kullanılan hata mesajı, `unwrap`ın kullandığı varsayılan
`panic!` mesajı yerine, `expect`e aktardığımız parametre olacaktır. İşte böyle görünüyor:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Üretim kalitesindeki kodlarda, çoğu Rustacean
`unwrap` yerine `expect` seçeneğini seçer ve işlemin neden her zaman
başarılı olmasının beklendiği hakkında daha fazla bağlam verir. Bu şekilde, varsayımlarınızın yanlış olduğu kanıtlanırsa, hata ayıklamada kullanabileceğiniz daha fazla
bilgiye sahip olursunuz.

### Hataların Yayılması

Bir fonksiyonun uygulaması başarısız olabilecek bir şeyi çağırdığında,
hatayı fonksiyonun kendi içinde ele almak yerine, ne yapacağına karar verebilmesi için hatayı
çağıran koda döndürebilirsiniz. Buna hatayı _propagating_
denir ve çağıran koda daha fazla kontrol sağlar; burada hatanın nasıl ele alınması gerektiğini belirleyen daha fazla
bilgi veya mantık, kodunuzun bağlamında mevcut olan
bilgiden daha fazla olabilir.

Örneğin, Liste 9-6'da bir dosyadan kullanıcı adı okuyan bir fonksiyon gösterilmektedir. Eğer
dosya mevcut değilse veya okunamıyorsa, bu fonksiyon bu hataları
fonksiyonu çağıran koda döndürecektir.

<Listing number="9-6" file-name="src/main.rs" caption="A function that returns errors to the calling code using `match`">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

</Listing>

Bu fonksiyon çok daha kısa bir şekilde yazılabilir, ancak hata işlemeyi keşfetmek için
ile başlayacağız; sonunda,
daha kısa yolu göstereceğiz. Önce
fonksiyonunun dönüş tipine bakalım: `Result<String, io::Error>`. Bu, fonksiyonun `Result<T, E>` tipinde bir
değeri döndürdüğü anlamına gelir; burada genel parametre `T`
somut tip `String` ile doldurulmuştur ve genel tip `E`
somut tip `io::Error` ile doldurulmuştur.

Bu fonksiyon sorunsuz bir şekilde başarılı olursa, bu
fonksiyonunu çağıran kod, bir `String` tutan bir `Ok` değeri alacaktır -
bu fonksiyonun dosyadan okuduğu `kullanıcı adı`. Bu işlev herhangi bir sorunla karşılaşırsa,
çağıran kod, sorunların ne olduğu hakkında daha fazla bilgi içeren bir `io::Error`
örneğini tutan bir `Err` değeri alacaktır. Bu fonksiyonun geri dönüş tipi olarak
`io::Error` seçtik çünkü
bu fonksiyonun gövdesinde çağırdığımız ve başarısız olabilecek her iki işlemden de dönen hata değerinin
tipi budur: `File::open` fonksiyonu ve
`read_to_string` metodu.

Fonksiyonun gövdesi `File::open` fonksiyonunu çağırarak başlar. Ardından
adresinde `Result` değerini Liste 9-4'teki `match`e benzer bir `match` ile ele alıyoruz.
Eğer `File::open` başarılı olursa, `file`
kalıp değişkenindeki dosya tanıtıcısı `kullanıcı_adı_dosya` değişkenindeki değer olur ve
fonksiyonu devam eder. `Error` durumunda, `panic!` çağrısı yapmak yerine, fonksiyondan tamamen erken dönmek için `return`
anahtar sözcüğünü kullanırız ve `File::open` fonksiyonundan gelen
hata değerini, şimdi `e` kalıp değişkeninde,
bu fonksiyonun hata değeri olarak çağıran koda geri aktarırız.

Yani, eğer `kullanıcıadı_dosyası` içinde bir dosya tanıtıcımız varsa, fonksiyon daha sonra `kullanıcıadı` değişkeni içinde bir
yeni `String` oluşturur ve dosyanın içeriğini
`kullanıcıadı` içine okumak için `kullanıcıadı_dosyası` içindeki dosya tanıtıcısını
üzerinde `read_to_string` metodunu çağırır. `read_to_string` yöntemi de bir `Result` döndürür, çünkü
`File::open` başarılı olsa bile başarısız olabilir. Bu yüzden
bu `Sonuç`u işlemek için başka bir `eşleşme`ye ihtiyacımız var: eğer `read_to_string` başarılı olursa, o zaman fonksiyonumuz
başarılı olmuştur ve şimdi `username`
içinde bulunan dosyadaki kullanıcı adını bir `Ok` ile sarılmış olarak döndürürüz. Eğer `read_to_string` başarısız olursa, `File::open` fonksiyonunun
dönüş değerini işleyen `match` fonksiyonunda hata değerini döndürdüğümüz gibi
şeklinde hata değerini döndürürüz. Ancak, bu fonksiyondaki son ifade olduğu için
`return` ifadesini açıkça söylememize gerek yoktur.

Bu kodu çağıran kod daha sonra bir kullanıcı adı içeren bir `Ok` değeri
veya bir `io::Error` içeren bir `Err` değeri almayı ele alacaktır. Bu değerlerle ne yapılacağına karar vermek
çağıran koda bağlıdır. Çağıran
kodu bir `Err` değeri alırsa, `panic!` çağırabilir ve programı çökertebilir,
varsayılan kullanıcı adını kullanabilir veya kullanıcı adını bir dosyadan başka bir yerden arayabilir, örneğin
. Çağıran kodun aslında
ne yapmaya çalıştığı hakkında yeterli bilgiye sahip değiliz, bu nedenle tüm başarı veya hata bilgilerini
uygun şekilde ele alması için yukarı doğru yayıyoruz.

Bu hata yayma modeli Rust'ta o kadar yaygındır ki, Rust bunu kolaylaştırmak için
soru işareti operatörünü `?` sağlar.

#### Hataları Yaymak için Bir Kısayol: ? Operatörü

Liste 9-7, Liste 9-6'daki ile
aynı işlevselliğe sahip bir `read_username_from_file` uygulamasını göstermektedir, ancak bu uygulama `?`
operatörünü kullanmaktadır.

<Listing number="9-7" file-name="src/main.rs" caption="A function that returns errors to the calling code using the `?` operator">

<!-- Burada kasıtlı olarak rustdoc_include kullanmıyoruz;
dosyasındaki `main` fonksiyonu panik yapıyor. Okuyucu denemesi amacıyla dahil etmek istiyoruz, ancak
rustdoc testi amacıyla dahil etmek istemiyoruz-->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

</Listing>

Bir `Result` değerinden sonra yerleştirilen `?`, Listing
9-6'da `Result` değerlerini işlemek için tanımladığımız `match` ifadeleriyle hemen hemen aynı şekilde
çalışmak üzere tanımlanmıştır. Eğer `Result` değeri bir `Ok` ise, `Ok` içindeki değer
bu ifadeden döndürülür ve program devam eder. Eğer
değeri bir `Err` ise, sanki
`return` anahtar sözcüğünü kullanmışız gibi tüm fonksiyondan `Err` değeri döndürülür, böylece hata değeri çağıran
koduna yayılır.

Listing 9-6'daki `match` ifadesinin
yaptığı ile `?` operatörünün yaptığı arasında bir fark vardır:
tarafından `?` operatörü çağrılan hata değerleri,
standart kütüphanesindeki `From` özelliğinde tanımlanan ve değerleri bir türden diğerine dönüştürmek için kullanılan `from` fonksiyonundan geçer.
? operatörü `from` fonksiyonunu çağırdığında, alınan hata tipi
geçerli
fonksiyonunun dönüş tipinde tanımlanan hata tipine dönüştürülür. Bu, bir işlev tek bir hata türü döndürdüğünde
bir işlevin başarısız olabileceği tüm yolları temsil etmek için kullanışlıdır; bazı bölümler birçok farklı
nedenden dolayı başarısız olsa bile.

Örneğin, Listing
9-7 adresindeki `read_username_from_file` işlevini, tanımladığımız `OurError` adlı özel bir hata türünü döndürecek şekilde değiştirebiliriz. Ayrıca
bir `io::Error`dan bir
`OurError` örneği oluşturmak için `impl From<io::Error> for OurError` tanımlarsak,
`read_username_from_file` gövdesindeki `?` operatör çağrıları `from` fonksiyonunu çağıracak ve
fonksiyona daha fazla kod eklemek zorunda kalmadan hata tiplerini dönüştürecektir.

Listing 9-7 bağlamında, `File::open` çağrısının sonundaki `?`
bir `Ok` içindeki değeri `username_file` değişkenine döndürecektir. Eğer
bir hata oluşursa, `?` operatörü tüm fonksiyondan erken dönecek ve
çağıran koda herhangi bir `Err` değeri verecektir. Aynı şey `read_to_string` çağrısının
sonundaki `?` için de geçerlidir.

? operatörü çok sayıda `boilerplate`i ortadan kaldırır ve bu fonksiyonun
uygulamasını daha basit hale getirir. Hatta Listing 9-8'de gösterildiği gibi, `?`'dan hemen sonra
yöntem çağrılarını zincirleyerek bu kodu daha da kısaltabiliriz.

<Listing number="9-8" file-name="src/main.rs" caption="Chaining method calls after the `?` operator">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

</Listing>

Yeni `String`in `username` içinde oluşturulmasını
işlevinin başına taşıdık; bu kısım değişmedi. `username_file` değişkenini oluşturmak yerine, `read_to_string` çağrısını doğrudan
`File::open("hello.txt")?` sonucuna zincirledik. `read_to_string` çağrısının sonunda hala bir `?` var ve hem `File::open` hem de `read_to_string` başarılı olduğunda
hatalarını döndürmek yerine hala `username`
içeren bir `Ok` değeri döndürüyoruz. İşlevsellik yine Liste 9-6 ve Liste 9-7'deki ile aynıdır;
bu sadece yazmanın farklı ve daha ergonomik bir yoludur.

Liste 9-9, `fs::read_to_string` kullanarak bunu daha da kısaltmanın bir yolunu göstermektedir.

<Listeleme number="9-9" file-name="src/main.rs" caption="Dosyayı açıp okumak yerine `fs::read_to_string` kullanmak">

<!-- Burada kasıtlı olarak rustdoc_include kullanmıyoruz;
dosyasındaki `main` fonksiyonu panik yapıyor. Okuyucu denemesi amacıyla dahil etmek istiyoruz, ancak
rustdoc testi amacıyla dahil etmek istemiyoruz. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

</Listing>

Bir dosyayı bir dizgiye okumak oldukça yaygın bir işlemdir, bu nedenle standart
kütüphanesi
dosyasını açan, yeni bir `String` oluşturan, dosyanın içeriğini okuyan,
içeriğini bu `String` içine koyan ve geri döndüren kullanışlı `fs::read_to_string` işlevini sağlar. Tabii ki, `fs::read_to_string`
kullanmak bize tüm hata işlemlerini açıklama fırsatı vermez, bu yüzden önce uzun yoldan
yaptık.

#### `?` Operatörünün Kullanılabileceği Yerler

? operatörü sadece dönüş tipi
`?` operatörünün kullanıldığı değerle uyumlu olan fonksiyonlarda kullanılabilir. Bunun nedeni, `?` operatörünün
Liste 9-6'da tanımladığımız `match` ifadesiyle aynı şekilde
fonksiyondan bir değerin erken dönüşünü gerçekleştirmek için tanımlanmış olmasıdır. Listing 9-6'da,
`match` bir `Result` değeri kullanıyordu ve erken dönüş kolu bir
`Err(e)` değeri döndürüyordu. Fonksiyonun geri dönüş tipinin bir `Result` olması gerekir ki
bu `return` ile uyumlu olsun.

Listing 9-10'da, `?` operatörünü
bir `main` fonksiyonunda, `?` operatörünü kullandığımız değerin
tipiyle uyumsuz bir geri dönüş tipiyle kullanırsak alacağımız hataya bakalım.

<Listing number="9-10" file-name="src/main.rs" caption="Attempting to use the `?` in the `main` function that returns `()` won’t compile.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

</Listing>

Bu kod, başarısız olabilecek bir dosyayı açar. ? operatörü `File::open` tarafından döndürülen `Result`
değerini takip eder, ancak bu `main` fonksiyonu `Result` değil,
`()` dönüş türüne sahiptir. Bu kodu derlediğimizde, aşağıdaki hata
mesajını alırız:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

Bu hata, `?` operatörünü yalnızca `Result`, `Option` veya `FromResidual` trait'ini uygulayan bir tür döndüren fonksiyonlarda kullanabileceğimizi belirtiyor.

Hatayı düzeltmek için iki seçeneğiniz var. İlk seçenek, fonksiyonunuzun dönüş türünü, `?` operatörünü kullandığınız değerle uyumlu hale getirmek (eğer bunu engelleyen bir kısıtınız yoksa). İkinci seçenek ise, `Result<T, E>` değerini uygun şekilde ele almak için bir `match` ifadesi veya `Result<T, E>` metodlarından birini kullanmak.

Hata mesajında ayrıca, `?` operatörünün `Option<T>` değerleriyle de kullanılabileceği belirtiliyor. `Result` üzerinde `?` kullanımında olduğu gibi, `Option` üzerinde `?` kullanabilmek için fonksiyonunuzun bir `Option` döndürmesi gerekiyor. `Option<T>` üzerinde `?` operatörünün davranışı, `Result<T, E>` üzerindeki davranışına benzer: eğer değer `None` ise, fonksiyon o noktada erken sonlanır ve `None` döndürülür. Eğer değer `Some` ise, `Some` içindeki değer ifadenin sonucu olur ve fonksiyon devam eder. Örnek 9-11, verilen metindeki ilk satırın son karakterini bulan bir fonksiyon örneği içeriyor.

<Listing number="9-11" caption="Using the `?` operator on an `Option<T>` value">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

</Listing>

Bu fonksiyon `Option<char>` döndürür çünkü orada bir karakter olabilir, ancak karakter olmaması da mümkündür. Bu kod, `text` adlı string dilimini alır ve üzerinde `lines` metodunu çağırır. Bu metod, stringdeki satırlar üzerinde bir iterator döndürür. Fonksiyon ilk satırı incelemek istediği için, iteratör üzerinde `next` çağrısı yaparak ilk değeri alır. Eğer `text` boş bir stringse, `next` çağrısı `None` döndürecektir. Bu durumda `?` operatörü devreye girerek fonksiyonun erken sonlanmasını ve `last_char_of_first_line`'dan `None` döndürülmesini sağlar. Eğer `text` boş değilse, `next`, `text`'in ilk satırını içeren bir `Some` değeri döndürür.  

`?` operatörü string dilimini çıkarır ve bu string dilimi üzerinde `chars` metodunu çağırarak karakterlerinin iteratorünü alabiliriz. İlk satırın son karakteriyle ilgilendiğimiz için, iteratördeki son öğeyi almak üzere `last` metodunu çağırırız. Bu bir `Option` döndürür çünkü ilk satır boş olabilir (örneğin, `"\nhi"` gibi bir stringde olduğu gibi). Ancak eğer ilk satırda bir karakter varsa, `Some` varyantı içinde döndürülür. Ortadaki `?` operatörü, bu mantığı özlü bir şekilde ifade etmemizi sağlar ve fonksiyonu tek satırda yazmamıza olanak tanır. Eğer `Option` üzerinde `?` kullanamasaydık, bu mantığı daha fazla metod çağrısı veya `match` ifadesiyle gerçekleştirmemiz gerekirdi.  

Dikkat edin, `Result` döndüren bir fonksiyonda `?` operatörünü `Result` üzerinde kullanabilirsiniz. Benzer şekilde, `Option` döndüren bir fonksiyonda `?` operatörünü `Option` üzerinde kullanabilirsiniz. Ancak bu ikisini karıştıramazsınız. `?` operatörü otomatik olarak bir `Result`'ı `Option`'a veya tam tersine dönüştürmez. Bu gibi durumlarda, `Result` üzerindeki `ok` metodu veya `Option` üzerindeki `ok_or` metodu gibi yöntemlerle açıkça dönüşüm yapabilirsiniz.  

Şu ana kadar kullandığımız tüm `main` fonksiyonları `()` döndürüyordu. `main` fonksiyonu özeldir çünkü bir çalıştırılabilir programın giriş ve çıkış noktasıdır. Programın beklenen şekilde davranması için dönüş türünde bazı kısıtlamalar vardır.  

Neyse ki, `main` aynı zamanda `Result<(), E>` de döndürebilir. Örnek 9-12'de, Örnek 9-10'daki kod yer alıyor ancak `main`'in dönüş türü `Result<(), Box<dyn Error>>` olarak değiştirilmiş ve sonuna `Ok(())` eklenmiştir. Bu sayede kod artık derlenebilir.

<Listing number="9-12" file-name="src/main.rs" caption="Changing `main` to return `Result<(), E>` allows the use of the `?` operator on `Result` values.">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

</Listing>

Box<dyn Error> türü bir özellik nesnesidir (trait object) ve bunu [“Farklı Türlerdeki Değerlere İzin Veren Özellik Nesnelerini Kullanma”][trait-objects] bölümünde (Bölüm 18) ele alacağız. Şimdilik, Box<dyn Error>'ı "herhangi bir tür hata" olarak okuyabilirsiniz. Hata türü Box<dyn Error> olan bir main fonksiyonunda Result değeri üzerinde ? kullanılmasına izin verilir, çünkü bu herhangi bir Err değerinin erken döndürülmesini sağlar. Bu main fonksiyonunun gövdesi yalnızca std::io::Error türündeki hataları döndürse bile, Box<dyn Error> belirtmek, main'in gövdesine başka hatalar döndüren kodlar eklendiğinde bile bu imzanın doğru kalmasını sağlar.

Bir main fonksiyonu Result<(), E> döndürdüğünde, eğer main Ok(()) döndürürse çalıştırılabilir program 0 değeriyle, bir Err değeri döndürürse sıfırdan farklı bir değerle sonlanır. C dilinde yazılan çalıştırılabilir dosyalar çıkış yaparken tamsayılar döndürür: başarıyla çıkan programlar 0 döndürürken, hata veren programlar 0 dışında bir tamsayı döndürür. Rust da bu geleneğe uyum sağlamak için çalıştırılabilir dosyalardan tamsayılar döndürür.

main fonksiyonu, [std::process::Termination][termination] özelliğini uygulayan herhangi bir türü döndürebilir. Bu özellik, bir ExitCode döndüren report adlı bir fonksiyon içerir. Kendi türleriniz için Termination özelliğini uygulama hakkında daha fazla bilgi edinmek için standart kütüphane dokümantasyonuna başvurabilirsiniz.

Artık panic! çağırmanın veya Result döndürmenin detaylarını ele aldığımıza göre, hangi durumlarda hangisinin uygun olduğuna nasıl karar vereceğimiz konusuna geri dönebiliriz.

[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result
[trait-objects]: ch18-02-trait-objects.md#using-trait-objects-that-**allow**-for-values-of-different-types
[termination]: ../std/process/trait.Termination.md
