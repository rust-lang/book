## Akışlar (Streams): Sıralı Future'lar

<!-- Eski başlıklar. Lütfen silmeyin, bağlantılar bozulabilir. -->

<a id="streams"></a>

Bu bölümde şimdiye kadar çoğunlukla tekil future'larla çalıştık. Büyük bir istisna olarak kullandığımız async kanal vardı. Bu bölümün başlarında ["Mesajlaşma"][17-02-messages]<!-- ignore --> kısmında async kanalın alıcı (receiver) tarafını nasıl kullandığımızı hatırlayın. Async `recv` metodu, zaman içinde bir dizi öğe üretir. Bu, _akış_ (stream) olarak bilinen çok daha genel bir desenin örneğidir.

13. bölümde, [Iterator Trait'i ve `next` Metodu][iterator-trait]<!-- ignore --> kısmında, bir dizi öğe ile çalışmıştık; ancak iterator'lar ile async kanal alıcısı arasında iki fark vardır. Birincisi zaman: iterator'lar eşzamanlıdır (senkron), kanal alıcısı ise asenkron. İkincisi ise API'dir. `Iterator` ile doğrudan çalışırken, senkron `next` metodunu çağırırız. Özellikle `trpl::Receiver` akışında ise, onun yerine asenkron `recv` metodunu çağırdık. Bunun dışında, bu API'ler oldukça benzerdir ve bu benzerlik tesadüf değildir. Bir akış, asenkron bir yineleme (iteration) gibidir. `trpl::Receiver` özel olarak mesajları beklerken, genel amaçlı akış API'si çok daha geniştir: `Iterator` gibi bir sonraki öğeyi sağlar, ama asenkron olarak.

Rust'ta iterator'lar ile akışlar arasındaki benzerlik sayesinde, herhangi bir iterator'dan bir akış oluşturabiliriz. Bir iterator'da olduğu gibi, bir akışla da `next` metodunu çağırıp çıktısını await ederek çalışabiliriz (Bkz. Liste 17-30).

<Listing number="17-30" caption="Bir iterator'dan akış oluşturup değerlerini yazdırmak" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-30/src/main.rs:stream}}
```

</Listing>

Bir sayı dizisiyle başlıyoruz, bunu bir iterator'a çevirip `map` ile tüm değerleri iki katına çıkarıyoruz. Sonra iterator'ı `trpl::stream_from_iter` fonksiyonu ile bir akışa dönüştürüyoruz. Son olarak, akıştaki öğeler geldikçe `while let` döngüsüyle üzerinden geçiyoruz.

Ne yazık ki, kodu çalıştırmaya çalıştığımızda derlenmez; bunun yerine, uygun bir `next` metodu olmadığına dair hata alırız:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-30
cargo build
copy only the error output
-->

```console
error[E0599]: no method named `next` found for struct `Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = note: the full type name has been written to 'file:///projects/async-await/target/debug/deps/async_await-575db3dd3197d257.long-type-14490787947592691573.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

Bu çıktının açıkladığı gibi, derleyici hatasının nedeni, `next` metodunu kullanabilmek için doğru trait'in scope'ta olması gerektiğidir. Şimdiye kadarki tartışmamıza bakarsak, bu trait'in `Stream` olmasını bekleyebilirsiniz, ama aslında `StreamExt`'tir. `Ext` (extension/kapsama) Rust topluluğunda bir trait'i başka bir trait ile genişletmek için yaygın bir desendir.

`Stream` ve `StreamExt` trait'lerini bölümün sonunda biraz daha ayrıntılı açıklayacağız, ama şimdilik bilmeniz gereken, `Stream` trait'inin aslında `Iterator` ve `Future` trait'lerini birleştiren düşük seviyeli bir arayüz tanımladığıdır. `StreamExt`, `Stream` üzerine daha yüksek seviyeli bir API sağlar; `next` metodu ve `Iterator` trait'inin sunduğu diğer yardımcı metodlar gibi. `Stream` ve `StreamExt` henüz Rust'ın standart kütüphanesinin parçası değildir, ama ekosistemdeki çoğu crate aynı tanımı kullanır.

Derleyici hatasını düzeltmek için, `trpl::StreamExt`'i scope'a ekleyen bir `use` satırı eklemeliyiz (Bkz. Liste 17-31).

<Listing number="17-31" caption="Bir iterator'ı akış olarak başarıyla kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-31/src/main.rs:all}}
```

</Listing>

Tüm bu parçalar bir araya geldiğinde, kod istediğimiz gibi çalışır! Ayrıca, artık `StreamExt` scope'ta olduğuna göre, tıpkı iterator'larda olduğu gibi tüm yardımcı metodlarını kullanabiliriz. Örneğin, Liste 17-32'de, `filter` metodunu kullanarak yalnızca üç ve beşin katlarını filtreliyoruz.

<Listing number="17-32" caption="Bir akışı `StreamExt::filter` metodu ile filtrelemek" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-32/src/main.rs:all}}
```

</Listing>

Tabii ki, bunu normal iterator'larla ve hiç async kullanmadan da yapabilirdik. Şimdi, akışlara özgü neler yapabileceğimize bakalım.

### Akışları Birleştirmek (Composing Streams)

Birçok kavram doğal olarak akış olarak temsil edilir: bir kuyruğa öğelerin gelmesi, tüm veri seti bilgisayarın belleğine sığmayacak kadar büyük olduğunda dosya sisteminden verilerin parça parça çekilmesi veya verilerin zamanla ağ üzerinden gelmesi gibi. Akışlar future olduğu için, onları diğer future türleriyle birlikte kullanabilir ve ilginç şekillerde birleştirebiliriz. Örneğin, çok fazla ağ çağrısı tetiklememek için olayları toplu işleyebilir, uzun süren işlemler dizisine zaman aşımı koyabilir veya gereksiz işleri önlemek için kullanıcı arayüzü olaylarını yavaşlatabiliriz (throttle).

Örneğin, bir WebSocket veya başka bir gerçek zamanlı iletişim protokolünden görebileceğimiz bir veri akışının yerine geçecek küçük bir mesaj akışı oluşturalım (Bkz. Liste 17-33).

<Listing number="17-33" caption="`rx` alıcısını bir `ReceiverStream` olarak kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-33/src/main.rs:all}}
```

</Listing>

Öncelikle, `get_messages` adında, dönüş tipi `impl Stream<Item = String>` olan bir fonksiyon oluşturuyoruz. Uygulamasında bir async kanal oluşturuyor, İngiliz alfabesinin ilk 10 harfi üzerinde döngü kuruyor ve bunları kanal üzerinden gönderiyoruz.

Ayrıca yeni bir tip kullanıyoruz: `ReceiverStream`. Bu tip, `trpl::channel`'dan gelen `rx` alıcısını, `next` metoduna sahip bir `Stream`'e dönüştürür. `main`'de ise, akıştaki tüm mesajları yazdırmak için bir `while let` döngüsü kullanıyoruz.

Kodu çalıştırdığımızda, tam da beklediğimiz sonuçları alırız:

<!-- Çıktı değişiklikleri önemli olmadığı için çıkartılmadı; değişiklikler büyük olasılıkla thread'lerin farklı çalışmasından kaynaklanıyor, derleyici değişikliklerinden değil -->

```text
Message: 'a'
Message: 'b'
Message: 'c'
Message: 'd'
Message: 'e'
Message: 'f'
Message: 'g'
Message: 'h'
Message: 'i'
Message: 'j'
```

Bunu normal `Receiver` API'siyle veya hatta normal `Iterator` API'siyle de yapabilirdik. Şimdi, akışlara özgü bir özellik ekleyelim: akıştaki her öğeye bir zaman aşımı (timeout) ekleyelim ve gönderdiğimiz öğelere gecikme ekleyelim (Bkz. Liste 17-34).

<Listing number="17-34" caption="Bir akıştaki öğelere zaman sınırı koymak için `StreamExt::timeout` metodunu kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-34/src/main.rs:timeout}}
```

</Listing>

Başlangıç olarak, akışa `timeout` metodunu ekliyoruz; bu metod `StreamExt` trait'inden gelir. Sonra, `while let` döngüsünün gövdesini güncelliyoruz, çünkü akış artık bir `Result` döndürüyor. `Ok` varyantı mesajın zamanında geldiğini, `Err` varyantı ise zaman aşımının dolduğunu gösterir. Sonucu `match` ile kontrol ediyoruz; mesajı başarıyla alırsak yazdırıyoruz, zaman aşımı olursa uyarı yazdırıyoruz. Son olarak, timeout uygulandıktan sonra mesajları pinliyoruz, çünkü timeout yardımcı fonksiyonu, poll edilebilmesi için pinlenmesi gereken bir akış üretir.

Ancak, mesajlar arasında gecikme olmadığı için, bu timeout programın davranışını değiştirmez. Şimdi, gönderdiğimiz mesajlara değişken gecikme ekleyelim (Bkz. Liste 17-35).

<Listing number="17-35" caption="`get_messages` fonksiyonunu async yapmadan, async gecikmeyle mesaj göndermek" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-35/src/main.rs:messages}}
```

</Listing>

`get_messages` fonksiyonunda, `messages` dizisiyle birlikte `enumerate` iterator metodunu kullanıyoruz, böylece gönderdiğimiz her öğenin indeksini de alabiliyoruz. Sonra, gerçek dünyadaki mesaj akışlarında görebileceğimiz farklı gecikmeleri simüle etmek için, çift indeksli öğelere 100 ms, tek indeksli öğelere 300 ms gecikme uyguluyoruz. Timeout'umuz 200 ms olduğu için, bu yarısı için etkili olmalı.

`get_messages` fonksiyonunda mesajlar arasında uyumak (sleep) için async kullanmamız gerekir. Ancak, `get_messages`'ı async fonksiyon yapamayız, çünkü o zaman dönüş tipi `Future<Output = Stream<Item = String>>` olurdu, oysa bizim istediğimiz `Stream<Item = String>>`. Çağıran, akışa erişmek için önce `get_messages`'ı await etmek zorunda kalırdı. Ama unutmayın: Bir future'daki her şey doğrusal olarak gerçekleşir; eşzamanlılık _future'lar arasında_ olur. `get_messages`'ı await etmek, tüm mesajların (ve aradaki gecikmelerin) gönderilmesini bekler, sonra alıcı akışı döndürürdü. Sonuç olarak, timeout işe yaramazdı; akışta gecikme olmaz, hepsi akış daha hazır olmadan gerçekleşirdi.

Bunun yerine, `get_messages`'ı normal bir fonksiyon olarak bırakıyoruz ve async `sleep` çağrılarını işlemek için bir görev (task) başlatıyoruz.

> Not: Bu şekilde `spawn_task` çağırmak, çalışma zamanımızı zaten kurduğumuz için çalışır; eğer kurmasaydık panic olurdu. Diğer implementasyonlar farklı tercihler yapar: yeni bir çalışma zamanı başlatıp panic'i önleyebilirler ama biraz ek yük getirirler, ya da çalışma zamanı referansı olmadan görev başlatmanın bir yolunu hiç sunmayabilirler. Kullandığınız çalışma zamanının hangi tercihi yaptığını bildiğinizden emin olun ve kodunuzu buna göre yazın!

Artık kodumuz çok daha ilginç bir sonuç veriyor. Her iki mesaj arasında bir `Problem: Elapsed(())` hatası.

<!-- Çıktı değişiklikleri önemli olmadığı için çıkartılmadı; değişiklikler büyük olasılıkla thread'lerin farklı çalışmasından kaynaklanıyor, derleyici değişikliklerinden değil -->

```text
Message: 'a'
Problem: Elapsed(())
Message: 'b'
Message: 'c'
Problem: Elapsed(())
Message: 'd'
Message: 'e'
Problem: Elapsed(())
Message: 'f'
Message: 'g'
Problem: Elapsed(())
Message: 'h'
Message: 'i'
Problem: Elapsed(())
Message: 'j'
```

Timeout, mesajların sonunda gelmesini engellemez. Hâlâ tüm orijinal mesajları alırız, çünkü kanalımız _sınırsız_ (unbounded): belleğe sığdığı sürece istediğimiz kadar mesaj tutabilir. Mesaj zamanında gelmezse, akış işleyicimiz bunu hesaba katar; ama akışı tekrar poll ettiğimizde, mesaj gelmiş olabilir.

Farklı davranışlar istiyorsanız, farklı kanal türleri veya genel olarak farklı akış türleri kullanabilirsiniz. Şimdi, zaman aralıklarından oluşan bir akışı, bu mesaj akışıyla birleştirerek bunu pratikte görelim.

### Akışları Birleştirmek (Merging Streams)

Öncelikle, her milisaniyede bir öğe üretecek başka bir akış oluşturalım. Basitlik için, bir gecikmeyle mesaj göndermek için `sleep` fonksiyonunu ve `get_messages`'ta kullandığımız kanal akışı oluşturma yaklaşımını kullanabiliriz. Farkı, bu sefer geçen aralık sayısını geri göndereceğiz; dönüş tipi `impl Stream<Item = u32>` olacak ve fonksiyonun adı `get_intervals` olacak (Bkz. Liste 17-36).

<Listing number="17-36" caption="Her milisaniyede bir sayıcı ile akış oluşturmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-36/src/main.rs:intervals}}
```

</Listing>

Görevde bir `count` tanımlayarak başlıyoruz. (Bunu görev dışında da tanımlayabilirdik, ama değişkenin kapsamını sınırlamak daha net.) Sonra sonsuz bir döngü oluşturuyoruz. Döngünün her yinelemesinde, asenkron olarak bir milisaniye uyuyor, sayacı artırıyor ve kanaldan gönderiyoruz. Tüm bunlar `spawn_task` ile başlatılan görevde olduğu için, sonsuz döngü de dahil olmak üzere her şey çalışma zamanı ile birlikte temizlenir.

Bu tür sonsuz döngüler, yalnızca tüm çalışma zamanı sona erdiğinde biter ve async Rust'ta oldukça yaygındır: birçok programın süresiz çalışması gerekir. Async ile, her yinelemede en az bir await noktası olduğu sürece, bu başka hiçbir şeyi engellemez.

Şimdi, ana fonksiyonumuzun async bloğunda, `messages` ve `intervals` akışlarını birleştirmeyi deneyebiliriz (Bkz. Liste 17-37).

<Listing number="17-37" caption="`messages` ve `intervals` akışlarını birleştirmeyi denemek" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-37/src/main.rs:main}}
```

</Listing>

Önce `get_intervals`'ı çağırıyoruz. Sonra, `merge` metodunu kullanarak `messages` ve `intervals` akışlarını birleştiriyoruz; bu, birden fazla akışı, kaynak akışlardan herhangi birinden öğe geldiği anda, belirli bir sıralama olmadan tek bir akışta birleştirir. Son olarak, birleşik akış üzerinde döngü kuruyoruz.

Bu noktada, ne `messages` ne de `intervals` pinlenmiş veya mutable olmak zorunda değil, çünkü ikisi de tek bir `merged` akışında birleştirilecek. Ancak, bu `merge` çağrısı derlenmez! (`while let` döngüsündeki `next` çağrısı da derlenmez, ama ona sonra döneceğiz.) Bunun nedeni, iki akışın farklı tiplere sahip olmasıdır. `messages` akışı, tipi `Timeout<impl Stream<Item = String>>` olan bir akıştır; burada `Timeout`, bir `timeout` çağrısı için `Stream`'i uygulayan tiptir. `intervals` akışı ise, tipi `impl Stream<Item = u32>` olan bir akıştır. Bu iki akışı birleştirmek için, birini diğerine uyarlamamız gerekir. `messages` zaten istediğimiz temel formda ve timeout hatalarını ele alması gerekiyor, bu yüzden `intervals` akışını dönüştüreceğiz (Bkz. Liste 17-38).

<!-- Bunu doğrudan test edemiyoruz, çünkü asla durmuyor. -->

<Listing number="17-38" caption="`intervals` akışının tipini `messages` akışının tipiyle hizalamak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-38/src/main.rs:main}}
```

</Listing>

Öncelikle, `intervals`'ı string'e dönüştürmek için `map` yardımcı metodunu kullanabiliriz. İkincisi, `messages`'daki `Timeout`'u eşleştirmemiz gerekir. Ancak, `intervals` için gerçekten bir timeout _istemediğimiz_ için, kullandığımız diğer sürelerden daha uzun bir timeout oluşturabiliriz. Burada, `Duration::from_secs(10)` ile 10 saniyelik bir timeout oluşturuyoruz. Son olarak, akışı mutable yapmamız ve pinlememiz gerekiyor, böylece `while let` döngüsündeki `next` çağrılarıyla akışta ilerleyebiliriz. Bu bizi _neredeyse_ istediğimiz yere getirir. Her şey tip kontrolünden geçer. Ancak, bunu çalıştırırsanız iki sorun olur. Birincisi, asla durmaz! Kapatmak için <span class="keystroke">ctrl-c</span> kullanmanız gerekir. İkincisi, İngiliz alfabesinden gelen mesajlar, interval sayaç mesajlarının arasında kaybolur:

<!-- Çıktı değişiklikleri önemli olmadığı için çıkartılmadı; değişiklikler büyük olasılıkla görevlerin farklı çalışmasından kaynaklanıyor, derleyici değişikliklerinden değil -->

```text
--snip--
Interval: 38
Interval: 39
Interval: 40
Message: 'a'
Interval: 41
Interval: 42
Interval: 43
--snip--
```

Liste 17-39, bu son iki sorunu çözmenin bir yolunu gösteriyor.

<Listing number="17-39" caption="Birleşik akışları yönetmek için `throttle` ve `take` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-39/src/main.rs:throttle}}
```

</Listing>

Öncelikle, `intervals` akışında `throttle` metodunu kullanıyoruz, böylece `messages` akışını boğmaz (overwhelm etmez). _Throttle_ etmek, bir fonksiyonun ne sıklıkta çağrılacağını—veya burada olduğu gibi, akışın ne sıklıkta poll edileceğini—sınırlamanın bir yoludur. Her 100 milisaniyede bir yeterli olur, çünkü mesajlarımız da yaklaşık bu sıklıkta geliyor.

Bir akıştan kabul edeceğimiz öğe sayısını sınırlamak için, `take` metodunu birleşik akışa uyguluyoruz; çünkü son çıktıyı sınırlamak istiyoruz, sadece bir akışı değil.

Artık programı çalıştırdığımızda, akıştan 20 öğe çektikten sonra durur ve interval'lar mesajları boğmaz. Ayrıca, `Interval: 100` veya `Interval: 200` gibi değerler görmeyiz; bunun yerine, `Interval: 1`, `Interval: 2` gibi değerler görürüz—her ne kadar kaynak akış her milisaniyede bir olay üretebilecek olsa da. Bunun nedeni, `throttle` çağrısının orijinal akışı saran yeni bir akış üretmesidir; böylece orijinal akış yalnızca throttle oranında poll edilir, kendi "doğal" oranında değil. Yani, göz ardı ettiğimiz bir sürü interval mesajı yok; aslında o interval mesajlarını hiç üretmiyoruz! Bu, Rust'ın future'larının doğasındaki "tembellik"tir; performans özelliklerimizi seçmemize olanak tanır.

<!-- Çıktı değişiklikleri önemli olmadığı için çıkartılmadı; değişiklikler büyük olasılıkla görevlerin farklı çalışmasından kaynaklanıyor, derleyici değişikliklerinden değil -->

```text
Interval: 1
Message: 'a'
Interval: 2
Interval: 3
Problem: Elapsed(())
Interval: 4
Message: 'b'
Interval: 5
Message: 'c'
Interval: 6
Interval: 7
Problem: Elapsed(())
Interval: 8
Message: 'd'
Interval: 9
Message: 'e'
Interval: 10
Interval: 11
Problem: Elapsed(())
Interval: 12
```

Son olarak ele almamız gereken bir şey daha var: hatalar! Bu kanal tabanlı akışların her ikisinde de, kanalın diğer ucu kapandığında `send` çağrıları başarısız olabilir—ve bu, akışı oluşturan future'ların çalışma zamanınca nasıl yürütüldüğüne bağlıdır. Şimdiye kadar, bu olasılığı `unwrap` çağırarak görmezden geldik; ama iyi davranan bir uygulamada, hatayı açıkça ele almalı, en azından döngüden çıkıp daha fazla mesaj göndermemeliyiz. Liste 17-40, basit bir hata stratejisi gösteriyor: sorunu yazdırıp döngüden `break` ile çıkmak.

<Listing number="17-40" caption="Hataları ele almak ve döngüleri kapatmak">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-40/src/main.rs:errors}}
```

</Listing>

Her zamanki gibi, bir mesaj gönderme hatasını nasıl ele alacağınız duruma göre değişir; sadece bir stratejiniz olduğundan emin olun.

Artık pratikte birçok async örneği gördük; şimdi bir adım geri çekilip, Rust'ın async'i çalıştırmak için kullandığı `Future`, `Stream` ve diğer temel trait'lerin ayrıntılarına bakalım.

[17-02-messages]: ch17-02-concurrency-with-async.html#message-passing
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
