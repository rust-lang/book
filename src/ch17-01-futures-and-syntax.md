## Gelecekler (Futures) ve Async Söz Dizimi

Rust'ta eşzamanlı (asenkron) programlamanın temel unsurları _future_ (gelecek) kavramı ile Rust'ın `async` ve `await` anahtar kelimeleridir.

_Bir future (gelecek)_, şu anda hazır olmayan ama gelecekte bir noktada hazır olacak bir değerdir. (Bu kavram birçok programlama dilinde farklı isimlerle, örneğin _task_ veya _promise_ olarak da karşımıza çıkar.) Rust, farklı asenkron işlemlerin farklı veri yapılarıyla ama ortak bir arayüzle uygulanabilmesi için bir yapı taşı olarak `Future` trait'ini sağlar. Rust'ta future'lar, `Future` trait'ini uygulayan türlerdir. Her future, kendi ilerleme durumunu ve "hazır" olmanın ne anlama geldiğini kendi içinde tutar.

`async` anahtar kelimesini bloklara ve fonksiyonlara uygulayarak, bunların kesintiye uğrayıp daha sonra devam edebileceğini belirtirsiniz. Bir async blok veya async fonksiyon içinde, bir future'ın hazır olmasını _beklemek_ için `await` anahtar kelimesini kullanabilirsiniz. Yani, bir future'ı await etmek, onun hazır olmasını beklemek anlamına gelir. Bir async blok veya fonksiyon içinde future'ı await ettiğiniz her nokta, o kodun duraklatılıp daha sonra devam edebileceği potansiyel bir noktadır. Bir future'ın değerinin hazır olup olmadığını kontrol etme işlemine _polling_ (yoklama) denir.

Bazı diğer dillerde (örneğin C# ve JavaScript) de asenkron programlama için `async` ve `await` anahtar kelimeleri kullanılır. Bu dillere aşinaysanız, Rust'ın yaklaşımında ve söz diziminde önemli farklar olduğunu görebilirsiniz. Bunun iyi bir nedeni var; ilerleyen bölümlerde bunu göreceğiz!

Async Rust yazarken çoğu zaman `async` ve `await` anahtar kelimelerini kullanırız. Rust, bunları `Future` trait'ini kullanan eşdeğer koda derler; tıpkı `for` döngülerini `Iterator` trait'ini kullanan koda derlediği gibi. Ancak Rust, `Future` trait'ini sağladığı için, gerektiğinde kendi veri tipleriniz için de bunu uygulayabilirsiniz. Bu bölümde göreceğimiz birçok fonksiyon, kendi `Future` implementasyonlarına sahip türler döndürecek. Bölümün sonunda trait'in tanımına tekrar döneceğiz ve nasıl çalıştığını daha ayrıntılı inceleyeceğiz, ancak şimdilik bu kadar bilgiyle devam edebiliriz.

Tüm bunlar biraz soyut gelmiş olabilir, o yüzden ilk async programımızı yazalım: küçük bir web kazıyıcı (scraper). Komut satırından iki URL alacağız, ikisini de eşzamanlı olarak çekeceğiz ve hangisi önce biterse onun sonucunu döndüreceğiz. Bu örnekte epey yeni söz dizimi göreceksiniz ama endişelenmeyin—gereken her şeyi adım adım açıklayacağız.

## İlk Async Programımız

Bu bölümün odağını async öğrenmeye vermek ve ekosistemin parçalarıyla uğraşmamak için, `trpl` adında bir crate oluşturduk (`trpl`, "The Rust Programming Language"in kısaltmasıdır). Bu crate, ihtiyacınız olan tüm tipleri, trait'leri ve fonksiyonları, başta [`futures`][futures-crate]<!-- ignore --> ve [`tokio`][tokio]<!-- ignore --> olmak üzere yeniden dışa aktarır (re-export). `futures` crate'i, Rust'ta asenkron kod için resmi bir deneme alanıdır ve aslında `Future` trait'i ilk olarak burada tasarlanmıştır. Tokio ise günümüzde Rust'ta, özellikle web uygulamalarında en yaygın kullanılan async çalışma zamanıdır (runtime). Başka iyi çalışma zamanları da vardır ve bazıları sizin kullanım amacınıza daha uygun olabilir. `trpl` crate'i altında, arka planda `tokio` kullanıyoruz çünkü iyi test edilmiş ve yaygın olarak kullanılıyor.

Bazı durumlarda, `trpl` orijinal API'leri yeniden adlandırır veya sarmalar, böylece bu bölümde odaklanmanız gereken detaylara odaklanabilirsiniz. Crate'in ne yaptığını anlamak isterseniz, [kaynak koduna][crate-source]<!-- ignore --> göz atmanızı öneririz. Hangi crate'ten neyin dışa aktarıldığını görebilir ve crate'in ne yaptığına dair kapsamlı yorumlar bulabilirsiniz.

`hello-async` adında yeni bir binary proje oluşturun ve `trpl` crate'ini bağımlılık olarak ekleyin:

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

Artık `trpl`'in sağladığı çeşitli parçaları kullanarak ilk async programımızı yazabiliriz. Komut satırından iki web sayfası çeken, her birinden `<title>` elementini alan ve işlemi ilk bitiren sayfanın başlığını yazdıran küçük bir komut satırı aracı oluşturacağız.

### page_title Fonksiyonunu Tanımlamak

Bir sayfa URL'sini parametre olarak alan, ona istek yapan ve başlık (title) elementinin metnini döndüren bir fonksiyon yazarak başlayalım (Bkz. Liste 17-1).

<Listing number="17-1" file-name="src/main.rs" caption="Bir HTML sayfasından title elementini almak için async fonksiyon tanımlamak">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

Öncelikle, `page_title` adında bir fonksiyon tanımlıyor ve onu `async` anahtar kelimesiyle işaretliyoruz. Ardından, iletilen URL'yi çekmek için `trpl::get` fonksiyonunu kullanıyor ve yanıtı beklemek için `await` anahtar kelimesini ekliyoruz. Yanıtın metnini almak için `text` metodunu çağırıyor ve yine `await` ile bekliyoruz. Bu iki adım da asenkron. `get` fonksiyonu için, sunucunun yanıtının ilk kısmını (HTTP başlıkları, çerezler vs.) göndermesini beklememiz gerekir ve bu, yanıt gövdesinden ayrı olarak iletilebilir. Özellikle gövde çok büyükse, tamamının gelmesi zaman alabilir. Yanıtın _tamamının_ gelmesini beklememiz gerektiği için, `text` metodu da async'tir.

Her iki future'ı da açıkça await etmemiz gerekir, çünkü Rust'ta future'lar _tembeldir_: `await` anahtar kelimesini kullanmadığınız sürece hiçbir şey yapmazlar. (Hatta Rust, bir future'ı kullanmazsanız derleyici uyarısı gösterir.) Bu, 13. Bölümde iterator'lar ile ilgili [Bir Dizi Elemanı Iterator ile İşlemek][iterators-lazy]<!-- ignore --> kısmında gördüklerimizi hatırlatabilir. Iterator'lar, `next` metodunu çağırmadığınız sürece hiçbir şey yapmaz—bunu doğrudan veya `for` döngüleri ya da `map` gibi `next`i kullanan metodlarla dolaylı olarak yapabilirsiniz. Aynı şekilde, future'lar da açıkça istemedikçe çalışmaz. Bu tembellik, Rust'ın asenkron kodu gerçekten ihtiyaç duyulana kadar çalıştırmamasını sağlar.

> Not: Bu, önceki bölümde [Yeni Bir Thread Oluşturmak][thread-spawn]<!--ignore--> kısmında `thread::spawn` kullanırken gördüğümüz davranıştan farklıdır; orada başka bir thread'e verdiğimiz closure hemen çalışmaya başlıyordu. Ayrıca, birçok başka dilin asenkron yaklaşımından da farklıdır. Ancak Rust'ın performans garantilerini sağlayabilmesi için bu önemlidir; tıpkı iterator'larda olduğu gibi.

`response_text`'i aldıktan sonra, onu `Html::parse` ile bir `Html` tipine ayrıştırıyoruz. Artık elimizde ham bir string yerine, HTML ile daha zengin bir veri yapısı olarak çalışabileceğimiz bir tip var. Özellikle, bir CSS seçicisinin ilk eşleşmesini bulmak için `select_first` metodunu kullanabiliyoruz. "title" string'ini ilettiğimizde, dökümandaki ilk `<title>` elementini alıyoruz (varsa). Eşleşen bir element olmayabileceği için, `select_first` metodu `Option<ElementRef>` döndürür. Son olarak, `Option::map` metodunu kullanıyoruz; bu, `Option` içindeki eleman varsa onunla çalışmamıza, yoksa hiçbir şey yapmamamıza olanak tanır. (Burada bir `match` ifadesi de kullanılabilirdi ama `map` daha idiomatiktir.) `map`'e verdiğimiz fonksiyon gövdesinde, başlığın içeriğini almak için `inner_html` çağrılır ve bu bir `String` döndürür. Sonuç olarak elimizde bir `Option<String>` olur.

Dikkat edin, Rust'ta `await` anahtar kelimesi _beklediğiniz ifadenin sonuna_ gelir, başına değil. Yani, bu bir _son ek_ (postfix) anahtar kelimesidir. Eğer başka dillerde async kullandıysanız alışık olduğunuzdan farklı olabilir, ama Rust'ta bu, metod zincirlerini çok daha kullanışlı hale getirir. Sonuç olarak, `page_title` fonksiyonunun gövdesini, `trpl::get` ve `text` fonksiyon çağrılarını `await` ile aralarına koyarak zincirleyebiliriz (Bkz. Liste 17-2).

<Listing number="17-2" file-name="src/main.rs" caption="`await` anahtar kelimesiyle zincirleme yapmak">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

Böylece ilk async fonksiyonumuzu başarıyla yazmış olduk! `main` fonksiyonunda onu çağırmadan önce, yazdıklarımızı ve ne anlama geldiklerini biraz daha konuşalım.

Rust, `async` anahtar kelimesiyle işaretlenmiş bir blok gördüğünde, onu `Future` trait'ini uygulayan benzersiz, anonim bir veri tipine derler. Bir fonksiyon `async` olarak işaretlendiğinde ise, onu gövdesi bir async blok olan normal (async olmayan) bir fonksiyona dönüştürür. Bir async fonksiyonun dönüş tipi, derleyicinin o async blok için oluşturduğu anonim veri tipidir.

Yani, `async fn` yazmak, dönüş tipi _future_ olan bir fonksiyon yazmakla eşdeğerdir. Derleyici açısından, Liste 17-1'deki `async fn page_title` fonksiyonu, aşağıdaki gibi async olmayan bir fonksiyonla eşdeğerdir:

```rust
# extern crate trpl; // mdbook test için gerekli
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

Şimdi bu dönüştürülmüş versiyonun her bir parçasını inceleyelim:

- 10. Bölümde ["Parametre Olarak Trait Kullanmak"][impl-trait]<!-- ignore --> kısmında gördüğümüz `impl Trait` söz dizimini kullanır.
- Döndürülen trait, `Output` adında ilişkili bir tipe sahip bir `Future`'dır. Dikkat edin, `Output` tipi, orijinal `async fn` versiyonundaki dönüş tipiyle aynı olan `Option<String>`'dir.
- Orijinal fonksiyonun gövdesinde çağrılan tüm kod, bir `async move` bloğuna sarılmıştır. Blokların birer ifade olduğunu unutmayın. Bu tüm blok, fonksiyondan döndürülen ifadedir.
- Bu async blok, az önce açıkladığımız gibi `Option<String>` tipinde bir değer üretir. Bu değer, dönüş tipindeki `Output` ile eşleşir. Bu, daha önce gördüğünüz diğer bloklar gibidir.
- Yeni fonksiyon gövdesi, `url` parametresinin kullanımı nedeniyle bir `async move` bloğudur. (Bölümün ilerleyen kısımlarında `async` ile `async move` arasındaki farkı daha ayrıntılı konuşacağız.)

Artık `main` fonksiyonunda `page_title`'ı çağırabiliriz.

## Tek Bir Sayfanın Başlığını Belirlemek

Başlangıç olarak, sadece tek bir sayfanın başlığını alacağız. Liste 17-3'te, 12. Bölümde [Komut Satırı Argümanlarını Kabul Etmek][cli-args]<!-- ignore --> kısmında kullandığımız desenin aynısını izliyoruz. Ardından, ilk URL'yi `page_title`'a veriyor ve sonucu bekliyoruz. Future'ın ürettiği değer bir `Option<String>` olduğu için, sayfanın bir `<title>`'ı olup olmadığını kontrol etmek ve farklı mesajlar yazdırmak için bir `match` ifadesi kullanıyoruz.

<Listing number="17-3" file-name="src/main.rs" caption="Kullanıcıdan alınan argümanla `main` fonksiyonunda `page_title` fonksiyonunu çağırmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

Ne yazık ki, bu kod derlenmez. `await` anahtar kelimesini yalnızca async fonksiyonlarda veya bloklarda kullanabiliriz ve Rust, özel `main` fonksiyonunu `async` olarak işaretlememize izin vermez.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

`main` fonksiyonunun `async` olarak işaretlenememesinin nedeni, async kodun bir _çalışma zamanı_ (runtime) gerektirmesidir: Asenkron kodun yürütülmesiyle ilgili ayrıntıları yöneten bir Rust crate'i. Bir programın `main` fonksiyonu bir çalışma zamanı _başlatabilir_, ama kendisi bir çalışma zamanı _değildir_. (Bunun neden böyle olduğunu birazdan daha ayrıntılı göreceğiz.) Asenkron kod çalıştıran her Rust programında, en az bir yerde bir çalışma zamanı kurulur ve future'lar yürütülür.

Async destekleyen çoğu dil, çalışma zamanını gömülü olarak sunar, ancak Rust sunmaz. Bunun yerine, farklı kullanım senaryolarına uygun farklı trade-off'lar yapan birçok farklı async çalışma zamanı vardır. Örneğin, çok çekirdekli ve bol RAM'li bir web sunucusunun ihtiyaçları ile, tek çekirdekli, az RAM'li ve heap tahsisi olmayan bir mikrodenetleyicinin ihtiyaçları çok farklıdır. Bu çalışma zamanlarını sağlayan crate'ler genellikle dosya veya ağ I/O gibi yaygın işlevlerin asenkron versiyonlarını da sunar.

Bu bölümün geri kalanında, future'ı argüman olarak alıp tamamlanana kadar çalıştıran `trpl` crate'inden `run` fonksiyonunu kullanacağız. Arka planda, `run` fonksiyonunu çağırmak, future'ı çalıştırmak için bir çalışma zamanı kurar. Future tamamlandığında, `run` future'ın ürettiği değeri döndürür.

`page_title`'ın döndürdüğü future'ı doğrudan `run`'a verebiliriz ve tamamlandığında, dönen `Option<String>` üzerinde eşleşme yapabiliriz; tıpkı Liste 17-3'te denediğimiz gibi. Ancak, bu bölümdeki örneklerin çoğunda (ve gerçek dünyadaki çoğu async kodda) sadece tek bir async fonksiyon çağrısı yapmayacağımız için, bunun yerine bir `async` blok verip, `page_title` çağrısının sonucunu açıkça bekleyeceğiz (Bkz. Liste 17-4).

<Listing number="17-4" caption="`trpl::run` ile bir async bloğu await etmek" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook test does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

Bu kodu çalıştırdığımızda, başta beklediğimiz davranışı elde ederiz:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-04
cargo build # skip all the build noise
cargo run https://www.rust-lang.org
# copy the output here
-->

```console
$ cargo run -- https://www.rust-lang.org
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
The title for https://www.rust-lang.org was
            Rust Programming Language
```

Nihayet çalışan bir async kodumuz oldu! Ancak iki siteyi birbirine karşı yarıştırmadan önce, future'ların nasıl çalıştığına kısaca tekrar bakalım.

Her _await noktası_—yani kodda `await` anahtar kelimesinin kullanıldığı her yer—kontrolün çalışma zamanına devredildiği bir yerdir. Bunun çalışabilmesi için, Rust'ın async bloktaki durumu takip etmesi gerekir ki, çalışma zamanı başka bir işi başlatıp, ilk iş tekrar ilerletilmeye hazır olduğunda ona geri dönebilsin. Bu, sanki her await noktasında mevcut durumu saklamak için aşağıdaki gibi bir enum yazmışsınız gibi, görünmez bir durum makinesidir:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

Ancak, her bir durum arasında geçiş yapmak için kodu elle yazmak zahmetli ve hata yapmaya çok açık olurdu; özellikle de daha fazla işlevsellik ve daha fazla durum eklemeniz gerektiğinde. Neyse ki, Rust derleyicisi async kod için bu durum makinesi veri yapılarını otomatik olarak oluşturur ve yönetir. Normal sahiplik ve ödünç alma kuralları aynen geçerlidir ve derleyici bunları da kontrol eder, faydalı hata mesajları sağlar. Bölümün ilerleyen kısımlarında bunlardan bazılarını göreceğiz.

Sonuçta, bu durum makinesini bir şeyin çalıştırması gerekir ve bu şey bir çalışma zamanıdır (runtime). (Bu nedenle, çalışma zamanlarıyla ilgili araştırma yaparken _executor_ terimiyle karşılaşabilirsiniz: executor, çalışma zamanının async kodu çalıştırmaktan sorumlu parçasıdır.)

Artık, derleyicinin neden Liste 17-3'te `main` fonksiyonunu async yapmamıza izin vermediğini görebilirsiniz. Eğer `main` bir async fonksiyon olsaydı, onun döndürdüğü future'ın durum makinesini yönetecek başka bir şey gerekirdi, ama `main` programın başlangıç noktasıdır! Bunun yerine, `main` içinde `trpl::run` fonksiyonunu çağırarak bir çalışma zamanı kurduk ve async bloğun döndürdüğü future'ı tamamlanana kadar çalıştırdık.

> Not: Bazı çalışma zamanları, async bir `main` fonksiyonu yazabilmeniz için makrolar sağlar. Bu makrolar, `async fn main() { ... }` kodunu, bizim Liste 17-4'te elle yaptığımız gibi, bir future'ı tamamlanana kadar çalıştıran normal bir `fn main` fonksiyonuna dönüştürür.

Şimdi bu parçaları birleştirip, eşzamanlı kodu nasıl yazabileceğimize bakalım.

### İki URL'yi Birbirine Karşı Yarıştırmak

Liste 17-5'te, komut satırından verilen iki farklı URL ile `page_title` fonksiyonunu çağırıyor ve bunları yarıştırıyoruz.

<Listing number="17-5" caption="" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

Önce, kullanıcıdan alınan iki URL için `page_title` fonksiyonunu çağırıyoruz. Ortaya çıkan future'ları `title_fut_1` ve `title_fut_2` olarak saklıyoruz. Unutmayın, bunlar henüz hiçbir şey yapmaz, çünkü future'lar tembeldir ve henüz beklenmediler. Sonra, future'ları `trpl::race` fonksiyonuna veriyoruz; bu fonksiyon, verilen future'lardan hangisi önce biterse onun çıktısını döndüren bir değer döndürür.

> Not: `race` fonksiyonu, aslında gerçek dünyadaki Rust kodunda daha sık karşılaşacağınız daha genel bir fonksiyon olan `select` üzerine kuruludur. `select` fonksiyonu, `trpl::race`'in yapamadığı birçok şeyi yapabilir, ancak ek karmaşıklıklar da getirir. Şimdilik bu detaya girmiyoruz.

Her iki future da "kazanabilir", bu yüzden bir `Result` döndürmek mantıklı olmaz. Bunun yerine, `race` daha önce görmediğimiz bir tür döndürür: `trpl::Either`. `Either` tipi, bir bakıma `Result`'a benzer; iki durumu vardır. Ancak, `Result`'tan farklı olarak, burada başarı veya başarısızlık kavramı yoktur. Bunun yerine, "biri ya da diğeri" anlamına gelen `Left` ve `Right` kullanılır:

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

`race` fonksiyonu, ilk argüman olarak verilen future önce biterse onun çıktısıyla `Left`, ikinci argüman önce biterse onun çıktısıyla `Right` döndürür. Bu, fonksiyonu çağırırken argümanların sırasıyla eşleşir: ilk argüman solda, ikinci argüman sağda yer alır.

Ayrıca, `page_title` fonksiyonunu, iletilen URL'yi de döndürecek şekilde güncelliyoruz. Böylece, ilk dönen sayfanın `<title>`'ı yoksa bile, hangi URL'nin önce bittiğini anlamlı bir mesajla yazdırabiliriz. Bu bilgiyle, `println!` çıktımızı da hem hangi URL'nin önce bittiğini hem de o sayfanın `<title>`'ı varsa onu gösterecek şekilde güncelliyoruz.

Artık küçük, çalışan bir web kazıyıcıya sahipsiniz! Birkaç URL seçip komut satırı aracını çalıştırın. Bazı sitelerin sürekli olarak diğerlerinden daha hızlı olduğunu, bazen de hangi sitenin daha hızlı olduğunun değiştiğini görebilirsiniz. Daha da önemlisi, future'larla çalışmanın temellerini öğrendiniz; şimdi async ile neler yapabileceğimizi daha derinlemesine inceleyebiliriz.

[impl-trait]: ch10-02-traits.md#parametre-olarak-özellikler
[iterators-lazy]: ch13-02-iterators.md
[thread-spawn]: ch16-01-threads.md#spawn-ile-yeni-bir-thread-oluşturmak
[cli-args]: ch12-01-accepting-command-line-arguments.md

<!-- TODO: map source link version to version of Rust? -->

[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs
