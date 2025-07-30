## Herhangi Bir Sayıda Future ile Çalışmak

Önceki bölümde iki future'dan üç future'a geçtiğimizde, `join` yerine `join3` kullanmamız gerekti. Her seferinde birleştirmek (join) istediğimiz future sayısı değiştiğinde farklı bir fonksiyon çağırmak zorunda kalmak can sıkıcı olurdu. Neyse ki, istediğimiz kadar argüman verebileceğimiz bir `join` makro formu var. Bu makro, future'ları kendi içinde bekler (await eder). Böylece, Liste 17-13'teki kodu `join3` yerine `join!` ile yeniden yazabiliriz (Bkz. Liste 17-14).

<Listing number="17-14" caption="Birden fazla future'ı beklemek için `join!` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:here}}
```

</Listing>

Bu, `join`, `join3`, `join4` ve benzeri fonksiyonlar arasında geçiş yapmaktan kesinlikle daha iyi! Ancak, bu makro formu bile yalnızca future sayısını önceden bildiğimizde çalışır. Gerçek dünyadaki Rust kodlarında ise, future'ları bir koleksiyona ekleyip, bunların bazılarını veya hepsini tamamlanana kadar beklemek yaygın bir desendir.

Bir koleksiyondaki tüm future'ları kontrol etmek için, hepsini yinelememiz ve _hepsini_ birleştirmemiz gerekir. `trpl::join_all` fonksiyonu, 13. bölümde [Iterator Trait'i ve `next` Metodu][iterator-trait]<!-- ignore --> kısmında öğrendiğiniz gibi, `Iterator` trait'ini uygulayan herhangi bir türü kabul eder; bu tam aradığımız şey gibi görünüyor. Future'larımızı bir vektöre koyup, `join!` yerine `join_all` ile değiştirmeyi deneyelim (Bkz. Liste 17-15).

<Listing  number="17-15" caption="Anonim future'ları bir vektörde saklayıp `join_all` çağırmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:here}}
```

</Listing>

Ne yazık ki, bu kod derlenmez. Bunun yerine şu hatayı alırız:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-15/
cargo build
copy just the compiler error
-->

```text
error[E0308]: mismatched types
  --> src/main.rs:45:37
   |
10 |         let tx1_fut = async move {
   |                       ---------- the expected `async` block
...
24 |         let rx_fut = async {
   |                      ----- the found `async` block
...
45 |         let futures = vec![tx1_fut, rx_fut, tx_fut];
   |                                     ^^^^^^ expected `async` block, found a different `async` block
   |
   = note: expected `async` block `{async block@src/main.rs:10:23: 10:33}`
              found `async` block `{async block@src/main.rs:24:22: 24:27}`
   = note: no two async blocks, even if identical, have the same type
   = help: consider pinning your async block and casting it to a trait object
```

Bu şaşırtıcı olabilir. Sonuçta, async bloklarının hiçbiri bir değer döndürmüyor, yani her biri `Future<Output = ()>` üretiyor. Ancak unutmayın ki, `Future` bir trait'tir ve derleyici her async blok için benzersiz bir enum oluşturur. El yazısı iki farklı struct'ı bir `Vec`'e koyamazsınız; aynı kural derleyicinin oluşturduğu farklı enum'lar için de geçerlidir.

Bunu çalıştırmak için, tıpkı 12. bölümde [run fonksiyonundan Hataları Döndürmek][dyn]<!-- ignore --> kısmında yaptığımız gibi _trait nesneleri_ (trait objects) kullanmamız gerekir. (Trait nesnelerini ayrıntılı olarak 18. bölümde işleyeceğiz.) Trait nesneleri, bu anonim future'ların hepsini aynı tipmiş gibi ele almamıza olanak tanır, çünkü hepsi `Future` trait'ini uygular.

> Not: 8. bölümde [Birden Fazla Tipi Bir Vektörde Saklamak için Enum Kullanmak][enum-alt]<!-- ignore --> kısmında, bir `Vec`'e birden fazla tip koymanın başka bir yolunu tartışmıştık: vektörde görünebilecek her tipi temsil eden bir enum kullanmak. Burada bunu yapamayız. Birincisi, farklı tipleri adlandıramayız çünkü anonimler. İkincisi, başta vektör ve `join_all` kullanmamızın nedeni, sadece aynı çıktı tipine sahip dinamik bir future koleksiyonuyla çalışmak istememizdi.

`vec!` içindeki her future'ı `Box::new` ile sarmalayarak başlıyoruz (Bkz. Liste 17-16).

<Listing number="17-16" caption="Future'ların tiplerini hizalamak için `Box::new` kullanmak" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

Ne yazık ki, bu kod da derlenmez. Aslında, hem ikinci hem de üçüncü `Box::new` çağrısı için öncekiyle aynı temel hatayı ve ayrıca `Unpin` trait'iyle ilgili yeni hatalar alırız. Önce `Box::new` çağrılarındaki tip hatalarını, `futures` değişkeninin tipini açıkça belirterek düzeltelim (Bkz. Liste 17-17).

<Listing number="17-17" caption="Açık tip bildirimiyle kalan tip uyuşmazlığı hatalarını düzeltmek" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:here}}
```

</Listing>

Bu tip bildirimi biraz karmaşık, o yüzden adım adım inceleyelim:

1. En içteki tip, future'ın kendisidir. Future'ın çıktısının birim tipi `()` olduğunu açıkça belirtmek için `Future<Output = ()>` yazıyoruz.
2. Sonra trait'i `dyn` ile dinamik olarak işaretliyoruz.
3. Tüm trait referansını bir `Box` ile sarıyoruz.
4. Son olarak, `futures`'ın bu öğeleri içeren bir `Vec` olduğunu açıkça belirtiyoruz.

Bu zaten büyük bir fark yarattı. Şimdi derleyiciyi çalıştırdığımızda, yalnızca `Unpin` ile ilgili hatalar alıyoruz. Üç tane olmasına rağmen, içerikleri çok benzer.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-17
cargo build
# copy *only* the errors
# fix the paths
-->

```text
error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
   --> src/main.rs:49:24
    |
49  |         trpl::join_all(futures).await;
    |         -------------- ^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
    |         |
    |         required by a bound introduced by this call
    |
    = note: consider using the `pin!` macro
            consider using `Box::pin` if you need to access the pinned value outside of the current scope
    = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `join_all`
   --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:105:14
    |
102 | pub fn join_all<I>(iter: I) -> JoinAll<I::Item>
    |        -------- required by a bound in this function
...
105 |     I::Item: Future,
    |              ^^^^^^ required by this bound in `join_all`

error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
  --> src/main.rs:49:9
   |
49 |         trpl::join_all(futures).await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
  --> src/main.rs:49:33
   |
49 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `async_await` (bin "async_await") due to 3 previous errors
```

Bu oldukça fazla, o yüzden parçalara ayıralım. Mesajın ilk kısmı, ilk async bloğun (`src/main.rs:8:23: 20:10`) `Unpin` trait'ini uygulamadığını ve çözüm için `pin!` veya `Box::pin` kullanmamızı öneriyor. Bölümün ilerleyen kısımlarında `Pin` ve `Unpin` hakkında daha fazla ayrıntıya gireceğiz. Şimdilik, derleyicinin önerisini izleyerek takıldığımız yerden kurtulabiliriz. Liste 17-18'de, önce `std::pin`'den `Pin`'i içe aktarıyoruz. Sonra, `futures` için tip bildirimini, her `Box`'ı saran bir `Pin` ile güncelliyoruz. Son olarak, future'ların kendisini pinlemek için `Box::pin` kullanıyoruz.

<Listing number="17-18" caption="`Vec` tipini doğrulamak için `Pin` ve `Box::pin` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

Bunu derleyip çalıştırırsak, sonunda beklediğimiz çıktıyı alırız:

<!-- Çıktı değişiklikleri önemli olmadığı için çıkartılmadı; değişiklikler büyük olasılıkla thread'lerin farklı çalışmasından kaynaklanıyor, derleyici değişikliklerinden değil -->

```text
received 'hi'
received 'more'
received 'from'
received 'messages'
received 'the'
received 'for'
received 'future'
received 'you'
```

Oh be!

Burada biraz daha keşfedilecek şey var. Örneğin, `Pin<Box<T>>` kullanmak, future'ları heap'te sakladığımız için küçük bir ek yük getirir—ve bunu sadece tiplerin uyumlu olması için yapıyoruz. Aslında heap tahsisine _ihtiyacımız yok_; bu future'lar sadece bu fonksiyona özgü. Daha önce de belirtildiği gibi, `Pin` başlı başına bir sarmalayıcı tiptir, bu yüzden `Vec`'te tek tip elde etmenin orijinal nedeni olan `Box`'a gerek kalmadan doğrudan `Pin` kullanabiliriz. Her future'ı doğrudan `std::pin::pin` makrosu ile pinleyebiliriz.

Ancak, pinlenmiş referansın tipini açıkça belirtmemiz gerekir; aksi takdirde Rust, bunları `Vec`'te dinamik trait nesneleri olarak yorumlamaz. Bu yüzden, `std::pin`'den `pin`'i de içe aktarıyoruz. Sonra, future'ları tanımlarken her birine `pin!` uygulayabilir ve `futures`'ı dinamik future tipinin pinlenmiş mutable referanslarını içeren bir `Vec` olarak tanımlayabiliriz (Bkz. Liste 17-19).

<Listing number="17-19" caption="Gereksiz heap tahsisinden kaçınmak için doğrudan `pin!` makrosu ile `Pin` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:here}}
```

</Listing>

Buraya kadar, farklı `Output` tiplerine sahip olabileceğimizi göz ardı ettik. Örneğin, Liste 17-20'de, `a` için anonim future `Future<Output = u32>`, `b` için anonim future `Future<Output = &str>`, `c` için anonim future ise `Future<Output = bool>` uygular.

<Listing number="17-20" caption="Farklı tiplerde üç future" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:here}}
```

</Listing>

Bunları beklemek için `trpl::join!` kullanabiliriz, çünkü bu makro birden fazla future tipine izin verir ve bu tiplerin bir tuple'ını üretir. `trpl::join_all` kullanamayız, çünkü bu fonksiyon verilen tüm future'ların aynı tipe sahip olmasını gerektirir. Unutmayın, bizi `Pin` macerasına başlatan hata buydu!

Bu temel bir denge: Eğer future'ların hepsi aynı tipe sahipse, dinamik sayıda future ile `join_all` kullanabiliriz; ya da sabit sayıda future ile, tipleri farklı olsa bile, `join` fonksiyonları veya `join!` makrosunu kullanabiliriz. Bu, Rust'ta başka tiplerle çalışırken de karşılaştığımız bir durumdur. Future'lar özel değildir, sadece onlarla çalışmak için güzel bir söz dizimi sunulmuştur ve bu iyi bir şeydir.

### Future'ları Yarıştırmak

`join` ailesindeki fonksiyon ve makrolarla future'ları "birleştirdiğimizde", _hepsinin_ bitmesini bekleriz. Bazen ise, bir grup future'dan sadece _birinin_ bitmesini beklememiz gerekir—yani future'ları birbirine karşı yarıştırmak gibi.

Liste 17-21'de, yine iki future olan `slow` ve `fast`'i birbirine karşı çalıştırmak için `trpl::race` kullanıyoruz.

<Listing number="17-21" caption="Hangi future önce biterse onun sonucunu almak için `race` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:here}}
```

</Listing>

Her future, çalışmaya başladığında bir mesaj yazdırır, `sleep` çağırıp bekler ve ardından bittiğinde başka bir mesaj yazdırır. Sonra, her ikisini de `trpl::race`'e verip, hangisi önce biterse onu bekleriz. (Buradaki sonuç şaşırtıcı değil: `fast` kazanır.) Daha önce [İlk Async Programımız][async-program]<!-- ignore --> kısmında `race` kullandığımızda, dönen `Either` örneğini kullanmıştık; burada ise, tüm ilginç davranışlar async blokların gövdesinde olduğu için onu görmezden geliyoruz.

Dikkat edin, `race`'e verilen argümanların sırasını değiştirirseniz, "started" mesajlarının sırası da değişir, ama `fast` future her zaman önce tamamlanır. Bunun nedeni, bu `race` fonksiyonunun adil olmamasıdır. Her zaman argümanlar hangi sırada verilmişse, o sırada future'ları çalıştırır. Diğer bazı implementasyonlar _adildir_ ve hangi future'ın önce kontrol edileceğini rastgele seçer. Hangi implementasyonu kullanırsak kullanalım, _bir_ future, gövdesindeki ilk `await`'e kadar çalışır, sonra başka bir görev başlatılabilir.

[İlk Async Programımız][async-program]<!-- ignore --> kısmında gördüğümüz gibi, her await noktasında Rust, çalışma zamanına görevi duraklatma ve başka bir göreve geçme şansı verir. Tersi de doğrudur: Rust, async blokları _yalnızca_ bir await noktasında duraklatır ve kontrolü çalışma zamanına bırakır. Await noktaları arasındaki her şey eşzamanlıdır.

Bu, bir async blokta await noktası olmadan çok iş yaparsanız, o future'ın diğer future'ların ilerlemesini engelleyeceği anlamına gelir. Bazen buna bir future'ın diğer future'ları _aç bırakması_ (starving) denir. Bazı durumlarda bu önemli olmayabilir. Ancak, pahalı bir kurulum veya uzun süren bir iş yapıyorsanız ya da bir future sürekli bir görevi sonsuza kadar yapacaksa, kontrolü ne zaman ve nerede çalışma zamanına bırakacağınızı düşünmeniz gerekir.

Aynı şekilde, uzun süren bloklayıcı işlemleriniz varsa, async farklı program bölümlerinin birbiriyle ilişkilenmesi için faydalı bir araç olabilir.

Peki, bu durumlarda kontrolü çalışma zamanına _nasıl_ bırakırız?

<!-- Eski başlıklar. Lütfen silmeyin, bağlantılar bozulabilir. -->

<a id="yielding"></a>

### Kontrolü Çalışma Zamanına Bırakmak (Yielding)

Şimdi uzun süren bir işlemi simüle edelim. Liste 17-22'de bir `slow` fonksiyonu tanıtıyoruz.

<Listing number="17-22" caption="Yavaş işlemleri simüle etmek için `thread::sleep` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:slow}}
```

</Listing>

Bu kod, `trpl::sleep` yerine `std::thread::sleep` kullanır, böylece `slow` çağrıldığında mevcut thread'i belirli bir milisaniye boyunca bloklar. Gerçek dünyadaki hem uzun süren hem de bloklayan işlemler için `slow`'u kullanabiliriz.

Liste 17-23'te, bu tür CPU yoğun iş yapan iki future'da `slow` fonksiyonunu kullanıyoruz.

<Listing number="17-23" caption="Yavaş işlemleri simüle etmek için `thread::sleep` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:slow-futures}}
```

</Listing>

Başlangıçta, her future, bir dizi yavaş işlemi _tamamladıktan sonra_ kontrolü çalışma zamanına bırakır. Bu kodu çalıştırırsanız, şu çıktıyı görürsünüz:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-23/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

Daha önceki örnekte olduğu gibi, `race` yine `a` tamamlanır tamamlanmaz biter. Ancak iki future arasında bir iç içe geçme (interleaving) yoktur. `a` future'ı, gövdesindeki ilk `trpl::sleep` çağrısına kadar tüm işini yapar, sonra `b` future'ı kendi `trpl::sleep` çağrısına kadar tüm işini yapar ve sonunda `a` future'ı tamamlanır. Her iki future'ın yavaş işleri arasında da ilerleme olmasını istiyorsak, await noktalarına ihtiyacımız var; böylece kontrolü çalışma zamanına bırakabiliriz. Bunun için await edebileceğimiz bir şeye ihtiyacımız var!

Bu tür bir geçişin Liste 17-23'te zaten olduğunu görebiliyoruz: Eğer `a` future'ındaki `trpl::sleep`'i kaldırırsak, `a` tamamen biter ve `b` future'ı _hiç_ çalışmaz. İşlemlerin sırayla ilerlemesini sağlamak için, Liste 17-24'te gösterildiği gibi, her `slow` çağrısından sonra `sleep` fonksiyonunu await edelim.

<Listing number="17-24" caption="İşlemlerin sırayla ilerlemesini sağlamak için `sleep` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

Liste 17-24'te, her `slow` çağrısından sonra await noktası olan `trpl::sleep` çağrıları ekliyoruz. Artık iki future'ın işleri iç içe geçiyor:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-24
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

`a` future'ı yine de bir süre çalışır, çünkü ilk `trpl::sleep`'ten önce `slow` çağrılır; ama ondan sonra future'lar her await noktasında sırayla çalışır. Burada her `slow` çağrısından sonra await noktası ekledik, ama işleri istediğimiz şekilde bölebiliriz.

Burada aslında _uyumak_ istemiyoruz; mümkün olduğunca hızlı ilerlemek istiyoruz. Sadece kontrolü çalışma zamanına bırakmamız gerekiyor. Bunu doğrudan `yield_now` fonksiyonunu kullanarak yapabiliriz. Liste 17-25'te, tüm bu `sleep` çağrılarını `yield_now` ile değiştiriyoruz.

<Listing number="17-25" caption="İşlemlerin sırayla ilerlemesini sağlamak için `yield_now` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:yields}}
```

</Listing>

Bu kod, gerçek amacı daha net gösterir ve `sleep`'e göre çok daha hızlı olabilir, çünkü zamanlayıcılar (örneğin `sleep`'in kullandığı) genellikle ne kadar hassas olabilecekleri konusunda sınırlamalara sahiptir. Örneğin, kullandığımız `sleep` sürümü, bir nanosaniyelik `Duration` versek bile en az bir milisaniye uyur. Modern bilgisayarlar _çok hızlıdır_: bir milisaniyede çok iş yapabilirler!

Bunu kendiniz de görebilirsiniz; Liste 17-26'da olduğu gibi küçük bir karşılaştırma yapabilirsiniz. (Bu, performans testi için çok titiz bir yol değildir, ama burada farkı göstermek için yeterlidir.)

<Listing number="17-26" caption="`sleep` ve `yield_now` performansını karşılaştırmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-26/src/main.rs:here}}
```

</Listing>

Burada, tüm durum yazdırmalarını atlıyoruz, bir nanosaniyelik `Duration` ile `trpl::sleep`'i çağırıyoruz ve her future'ı tek başına çalıştırıyoruz, future'lar arasında geçiş yok. Sonra 1.000 yineleme için çalıştırıyoruz ve `trpl::sleep` kullanan future'ın, `trpl::yield_now` kullanan future'a göre ne kadar sürdüğüne bakıyoruz.

`yield_now` ile olan sürüm _çok_ daha hızlı!

Bu, async'in, programınızda başka neler yaptığına bağlı olarak, hesaplama ağırlıklı (compute-bound) görevler için bile faydalı olabileceği anlamına gelir; çünkü programın farklı bölümleri arasındaki ilişkileri yapılandırmak için faydalı bir araç sunar. Bu, _kooperatif çoklu görev_ (cooperative multitasking) biçimidir; her future, await noktaları aracılığıyla kontrolü ne zaman devredeceğine kendisi karar verir. Bu nedenle, her future'ın çok uzun süre bloklamamaya dikkat etme sorumluluğu da vardır. Bazı Rust tabanlı gömülü işletim sistemlerinde, bu _tek_ çoklu görev türüdür!

Gerçek dünyadaki kodlarda, elbette her satırda await noktasıyla fonksiyon çağrısı arasında geçiş yapmazsınız. Bu şekilde kontrolü devretmek nispeten ucuzdur, ama bedava değildir. Birçok durumda, hesaplama ağırlıklı bir görevi bölmeye çalışmak onu önemli ölçüde yavaşlatabilir; bu yüzden bazen _genel_ performans için bir işlemin kısa süreliğine bloklanmasına izin vermek daha iyidir. Kodunuzun gerçek performans darboğazlarının nerede olduğunu her zaman ölçün. Ancak, eğer beklediğinizden daha fazla işin ardışık (seri) gerçekleştiğini görüyorsanız, bu dinamiği aklınızda tutmak önemlidir!

### Kendi Async Soyutlamalarımızı Oluşturmak

Future'ları birleştirerek yeni desenler de oluşturabiliriz. Örneğin, elimizdeki async yapı taşlarıyla bir `timeout` fonksiyonu inşa edebiliriz. Sonuçta, ortaya çıkan şey başka async soyutlamaları oluşturmak için kullanabileceğimiz bir yapı taşı olur.

Liste 17-27, bu `timeout` fonksiyonunun yavaş bir future ile nasıl çalışmasını beklediğimizi gösteriyor.

<Listing number="17-27" caption="Yavaş bir işlemi zaman sınırıyla çalıştırmak için hayalimizdeki `timeout` fonksiyonunu kullanmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-27/src/main.rs:here}}
```

</Listing>

Haydi bunu uygulayalım! Öncelikle, `timeout` için API'yi düşünelim:

- Kendisini await edebilmemiz için async bir fonksiyon olmalı.
- İlk parametresi çalıştırılacak bir future olmalı. Bunu generic yaparsak, herhangi bir future ile çalışabilir.
- İkinci parametresi bekleme süresi olacak. `Duration` kullanırsak, bunu `trpl::sleep`'e kolayca iletebiliriz.
- Bir `Result` döndürmeli. Eğer future başarıyla tamamlarsa, `Result`'ın `Ok` kısmında future'ın ürettiği değer olur. Eğer zaman aşımı önce dolarsa, `Result`'ın `Err` kısmında beklenen süre olur.

Liste 17-28 bu bildirimi gösteriyor.

<!-- Bu kasıtlı olarak derlenmediği için test edilmiyor. -->

<Listing number="17-28" caption="`timeout` fonksiyonunun imzasını tanımlamak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-28/src/main.rs:declaration}}
```

</Listing>

Tip hedeflerimizi karşıladık. Şimdi _davranış_ olarak neye ihtiyacımız var: Geçilen future'ı süreye karşı yarıştırmak istiyoruz. Süreden bir zamanlayıcı future oluşturmak için `trpl::sleep` kullanabilir ve bunu, çağıranın verdiği future ile birlikte `trpl::race` ile çalıştırabiliriz.

Ayrıca, `race`'in adil olmadığını, argümanları verildiği sırada kontrol ettiğini biliyoruz. Bu yüzden, `future_to_try`'ı önce `race`'e veriyoruz ki, çok kısa bir süre olsa bile tamamlanma şansı olsun. Eğer `future_to_try` önce biterse, `race` bize `Left` ile future'ın çıktısını döndürür. Eğer zamanlayıcı önce biterse, `race` bize `Right` ile zamanlayıcının çıktısı olan `()`'yı döndürür.

Liste 17-29'da, `trpl::race`'i await ettikten sonra sonucu match ediyoruz.

<Listing number="17-29" caption="`race` ve `sleep` ile `timeout` tanımlamak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-29/src/main.rs:implementation}}
```

</Listing>

Eğer `future_to_try` başarılı olursa ve `Left(output)` alırsak, `Ok(output)` döndürürüz. Eğer sleep zamanlayıcısı önce dolarsa ve `Right(())` alırsak, `()`'ı `_` ile görmezden gelir ve `Err(max_time)` döndürürüz.

Böylece, iki async yardımcıdan oluşan çalışan bir `timeout`'umuz oldu. Kodu çalıştırırsak, zaman aşımı sonrası başarısızlık modunu yazdırır:

```text
Failed after 2 seconds
```

Future'lar diğer future'larla birleşebildiği için, küçük async yapı taşlarıyla gerçekten güçlü araçlar inşa edebilirsiniz. Örneğin, bu aynı yaklaşımı kullanarak zaman aşımını tekrar denemelerle birleştirebilir ve bunları da ağ çağrıları gibi işlemlerle (bölümün başındaki örneklerden biri) kullanabilirsiniz.

Pratikte, genellikle doğrudan `async` ve `await` ile, ikincil olarak da `join`, `join_all`, `race` gibi fonksiyon ve makrolarla çalışırsınız. Sadece bu API'lerle future'ları kullanmak için ara sıra `pin`'e ihtiyaç duyarsınız.

Artık aynı anda birden fazla future ile çalışmanın çeşitli yollarını gördük. Sırada, _stream_ (akış) ile zaman içinde birden fazla future ile nasıl çalışabileceğimize bakacağız. Ancak önce şunları düşünmek isteyebilirsiniz:

- Bir gruptaki tüm future'ların bitmesini beklemek için `Vec` ile `join_all` kullandık. Peki, bir grup future'ı sırayla işlemek için `Vec`'i nasıl kullanabilirsiniz? Bunu yapmanın avantajları ve dezavantajları nelerdir?

- `futures` crate'inden `futures::stream::FuturesUnordered` tipine göz atın. Bunu bir `Vec` kullanmaktan ne farkı olurdu? (Bunun crate'in `stream` kısmında olmasına takılmayın; herhangi bir future koleksiyonuyla gayet iyi çalışır.)

[dyn]: ch12-03-improving-error-handling-and-modularity.md
[enum-alt]: ch08-01-vectors.md#enum-kullanarak-birden-fazla-türü-depolama
[async-program]: ch17-01-futures-and-syntax.md#i̇lk-async-programımız
[iterator-trait]: ch13-02-iterators.md#iterator-özelliği-ve-next-metodu
