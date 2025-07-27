## Async ile Eşzamanlılık Uygulamak

<!-- Eski başlıklar. Lütfen silmeyin, bağlantılar bozulabilir. -->

<a id="concurrency-with-async"></a>

Bu bölümde, 16. bölümde thread'lerle (iş parçacıklarıyla) ele aldığımız bazı eşzamanlılık (concurrency) problemlerine async yaklaşımını uygulayacağız. Temel fikirlerin çoğunu orada konuştuğumuz için, burada thread'ler ile future'lar arasındaki farklara odaklanacağız.

Birçok durumda, async ile eşzamanlılık için kullanılan API'ler, thread'lerle kullanılanlara oldukça benzer. Bazı durumlarda ise oldukça farklıdırlar. Thread'ler ve async arasındaki API'ler _benzer göründüğünde_ bile, genellikle farklı davranışlara ve neredeyse her zaman farklı performans özelliklerine sahiptirler.

<!-- Eski başlıklar. Lütfen silmeyin, bağlantılar bozulabilir. -->

<a id="counting"></a>

### `spawn_task` ile Yeni Bir Görev (Task) Oluşturmak

[Yeni Bir Thread Oluşturmak][thread-spawn]<!-- ignore --> başlığında ilk ele aldığımız işlem, iki ayrı thread'de sayma işlemi yapmaktı. Şimdi aynısını async ile yapalım. `trpl` crate'i, `thread::spawn` API'sine çok benzeyen bir `spawn_task` fonksiyonu ve `thread::sleep` API'sinin async versiyonu olan bir `sleep` fonksiyonu sağlar. Bunları birlikte kullanarak, sayma örneğini Liste 17-6'da gösterildiği gibi uygulayabiliriz.

<Listing number="17-6" caption="Ana görev başka bir şey yazdırırken yeni bir görev oluşturmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

Başlangıç noktası olarak, en üst düzey fonksiyonumuzun async olabilmesi için `main` fonksiyonumuzu `trpl::run` ile sarmalıyoruz.

> Not: Bu bölümün geri kalanında, her örnek tam olarak aynı şekilde `main` içinde `trpl::run` ile sarmalanacak, bu yüzden genellikle kodda göstermeyeceğiz. Kendi kodunuzda eklemeyi unutmayın!

Daha sonra, bu blok içinde her biri 500 milisaniye (yarım saniye) bekleyen birer `trpl::sleep` çağrısı içeren iki döngü yazıyoruz. Döngülerden birini `trpl::spawn_task` gövdesine, diğerini ise üst düzey bir `for` döngüsüne koyuyoruz. Ayrıca, `sleep` çağrılarının ardından birer `await` ekliyoruz.

Bu kod, thread tabanlı uygulamaya benzer şekilde çalışır—hatta kendi terminalinizde çalıştırdığınızda mesajların farklı sıralarda göründüğünü de görebilirsiniz:

<!-- Çıktı değişiklikleri önemli olmadığı için çıkartılmadı; değişiklikler büyük olasılıkla thread'lerin farklı çalışmasından kaynaklanıyor, derleyici değişikliklerinden değil -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

Bu sürüm, ana async bloktaki `for` döngüsü biter bitmez durur, çünkü `spawn_task` ile başlatılan görev, `main` fonksiyonu sona erdiğinde kapatılır. Görevin tamamen bitmesini istiyorsanız, ilk görevin tamamlanmasını beklemek için bir join handle kullanmanız gerekir. Thread'lerde, thread'in çalışması bitene kadar "bloklamak" için `join` metodunu kullanıyorduk. Liste 17-7'de, aynı şeyi yapmak için handle'ın kendisi bir future olduğu için `await` kullanabiliriz. `Output` tipi bir `Result` olduğu için, bekledikten sonra onu da `unwrap` ediyoruz.

<Listing number="17-7" caption="Bir görevi tamamlanana kadar çalıştırmak için join handle ile `await` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

Bu güncellenmiş sürüm, _her iki_ döngü de bitene kadar çalışır.

<!-- Çıktı değişiklikleri önemli olmadığı için çıkartılmadı; değişiklikler büyük olasılıkla thread'lerin farklı çalışmasından kaynaklanıyor, derleyici değişikliklerinden değil -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Şimdiye kadar, async ve thread'ler bize aynı temel sonuçları veriyor gibi görünüyor; sadece farklı bir söz dizimiyle: join handle'da `join` çağırmak yerine `await` kullanmak ve `sleep` çağrılarını beklemek.

Asıl büyük fark, bunu yapmak için başka bir işletim sistemi thread'i başlatmamıza gerek olmamasıdır. Hatta burada bir görev (task) başlatmamıza bile gerek yok. Çünkü async bloklar anonim future'lara derlenir, her döngüyü bir async bloğa koyup çalışma zamanının ikisini de tamamlanana kadar çalıştırmasını sağlayabiliriz; bunun için `trpl::join` fonksiyonunu kullanırız.

[Join Handle Kullanarak Tüm Thread'lerin Bitmesini Beklemek][join-handles]<!-- ignore --> bölümünde, `std::thread::spawn` çağrıldığında dönen `JoinHandle` tipinde `join` metodunun nasıl kullanılacağını göstermiştik. `trpl::join` fonksiyonu da benzer şekilde çalışır, ama future'lar için. İki future verdiğinizde, her ikisi de tamamlandığında çıktıları bir tuple olarak içeren yeni bir future üretir. Yani, Liste 17-8'de, hem `fut1` hem de `fut2` bitene kadar beklemek için `trpl::join` kullanıyoruz. `fut1` ve `fut2`'yi ayrı ayrı beklemiyoruz, onun yerine `trpl::join`'un ürettiği yeni future'ı bekliyoruz. Çıktıyı ise görmezden geliyoruz, çünkü sadece iki unit değeri içeren bir tuple.

<Listing number="17-8" caption="İki anonim future'ı beklemek için `trpl::join` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

Bunu çalıştırdığımızda, her iki future'ın da tamamlandığını görürüz:

<!-- Çıktı değişiklikleri önemli olmadığı için çıkartılmadı; değişiklikler büyük olasılıkla thread'lerin farklı çalışmasından kaynaklanıyor, derleyici değişikliklerinden değil -->

```text
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Artık her seferinde tam olarak aynı sırayı göreceksiniz; bu, thread'lerle gördüğümüzden çok farklı. Bunun nedeni, `trpl::join` fonksiyonunun _adil_ (fair) olmasıdır; yani her future'ı eşit sıklıkta kontrol eder, aralarında sırayla geçiş yapar ve biri hazırsa diğerinin öne geçmesine izin vermez. Thread'lerde, hangi thread'in ne kadar çalışacağına işletim sistemi karar verir. Async Rust'ta ise, hangi görevin kontrol edileceğine çalışma zamanı karar verir. (Gerçekte, ayrıntılar karmaşıklaşabilir çünkü bir async çalışma zamanı, eşzamanlılığı yönetmek için arka planda işletim sistemi thread'leri de kullanabilir; bu yüzden adilliği garanti etmek çalışma zamanı için daha fazla iş olabilir—ama yine de mümkündür!) Çalışma zamanlarının herhangi bir işlem için adillik garantisi vermesi gerekmez ve genellikle adillik isteyip istemediğinizi seçmenizi sağlayan farklı API'ler sunarlar.

Future'ları beklerken bu varyasyonları deneyin ve ne yaptıklarını gözlemleyin:

- Döngülerden birinin veya her ikisinin etrafındaki async bloğu kaldırın.
- Her async bloğu tanımladıktan hemen sonra bekleyin (await).
- Sadece ilk döngüyü bir async bloğa sarın ve ikinci döngünün gövdesinden sonra oluşan future'ı bekleyin.

Ek bir meydan okuma olarak, kodu çalıştırmadan önce her durumda çıktının ne olacağını tahmin etmeye çalışın!

<!-- Eski başlıklar. Lütfen silmeyin, bağlantılar bozulabilir. -->

<a id="message-passing"></a>

### İki Görevde Sayma İşlemini Mesajlaşma ile Yapmak

Future'lar arasında veri paylaşımı da tanıdık gelecek: yine mesajlaşma kullanacağız, ama bu kez tiplerin ve fonksiyonların async versiyonlarıyla. [Thread'ler Arasında Veri Aktarmak için Mesajlaşma Kullanmak][message-passing-threads]<!-- ignore --> bölümünde izlediğimizden biraz farklı bir yol izleyeceğiz, böylece thread tabanlı ve future tabanlı eşzamanlılık arasındaki bazı temel farkları gösterebileceğiz. Liste 17-9'da, ayrı bir görev başlatmadan—yani ayrı bir thread başlatmadan—sadece tek bir async blokla başlıyoruz.

<Listing number="17-9" caption="Async kanal oluşturmak ve iki ucunu `tx` ve `rx`'e atamak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

Burada, 16. bölümde thread'lerle kullandığımız çoklu üretici, tekli tüketici kanal API'sinin async versiyonu olan `trpl::channel`'ı kullanıyoruz. API'nin async versiyonu, thread tabanlı versiyondan çok az farklıdır: alıcı `rx` artık değiştirilebilir (mutable) ve `recv` metodu doğrudan değer döndürmek yerine beklememiz gereken bir future üretir. Artık göndericiden alıcıya mesaj gönderebiliriz. Dikkat edin, ayrı bir thread veya görev başlatmamıza gerek yok; sadece `rx.recv` çağrısını beklememiz yeterli.

`std::mpsc::channel`'daki eşzamanlı `Receiver::recv` metodu, bir mesaj alınana kadar bloklar. `trpl::Receiver::recv` metodu ise bloklamaz, çünkü async'tir. Bloklamak yerine, bir mesaj alınana veya kanalın gönderici tarafı kapanana kadar kontrolü çalışma zamanına bırakır. Buna karşılık, `send` çağrısını beklemiyoruz, çünkü bloklamaz. Bloklamasına gerek yoktur, çünkü gönderdiğimiz kanal sınırsızdır (unbounded).

> Not: Bu async kodun tamamı bir `trpl::run` çağrısındaki bir async blokta çalıştığı için, içindeki her şey bloklamadan çalışabilir. Ancak, _dışarıdaki_ kod, `run` fonksiyonu dönene kadar bloklanır. `trpl::run` fonksiyonunun amacı da budur: Hangi async kod kümesinde bloklanacağınızı ve böylece senkron ve asenkron kod arasında nerede geçiş yapacağınızı _siz_ seçersiniz. Çoğu async çalışma zamanında, `run` fonksiyonunun adı tam da bu yüzden `block_on`'dur.

Bu örnekte iki şeye dikkat edin. Birincisi, mesaj hemen ulaşacaktır. İkincisi, burada bir future kullansak da henüz eşzamanlılık yoktur. Listede olan her şey, future'lar hiç yokmuş gibi sırayla gerçekleşir.

Şimdi, her mesaj arasında uyuyarak (sleep) bir dizi mesaj göndererek ilk kısmı ele alalım; bu, Liste 17-10'da gösterilmiştir.

<!-- Bunu test edemiyoruz çünkü asla durmuyor! -->

<Listing number="17-10" caption="Async kanal üzerinden birden fazla mesaj gönderip almak ve her mesaj arasında `await` ile uyumak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

Mesajları göndermenin yanı sıra, onları da almamız gerekir. Bu durumda, kaç mesaj geleceğini bildiğimiz için, manuel olarak dört kez `rx.recv().await` çağırarak bunu yapabilirdik. Gerçek dünyada ise genellikle _bilinmeyen_ sayıda mesajı bekliyor olacağız, bu yüzden daha fazla mesaj kalmadığını anlayana kadar beklemeye devam etmemiz gerekir.

16-10'da, senkron bir kanaldan alınan tüm öğeleri işlemek için bir `for` döngüsü kullandık. Rust'ın henüz _asenkron_ bir dizi öğe üzerinde `for` döngüsü yazmanın bir yolu yok, bu yüzden daha önce görmediğimiz bir döngü kullanmamız gerekiyor: `while let` koşullu döngüsü. Bu, 6. bölümde [Kısa Kontrol Akışı: `if let` ve `let else`][if-let]<!-- ignore --> kısmında gördüğümüz `if let` yapısının döngü versiyonudur. Döngü, belirttiği desen değeriyle eşleştiği sürece çalışmaya devam eder.

`rx.recv` çağrısı bir future üretir ve onu bekleriz. Çalışma zamanı, future hazır olana kadar onu duraklatır. Bir mesaj geldiğinde, future her seferinde `Some(message)` olarak çözülür. Kanal kapandığında, _hiç_ mesaj gelmemiş olsa bile, future bunun yerine `None` döndürür ve artık beklememiz gerekmediğini belirtir.

`while let` döngüsü tüm bunları bir araya getirir. `rx.recv().await` çağrısının sonucu `Some(message)` ise, mesaja erişiriz ve döngü gövdesinde kullanabiliriz; tıpkı `if let` ile olduğu gibi. Sonuç `None` ise, döngü sona erer. Döngü her tamamlandığında, tekrar await noktasına gelir, böylece çalışma zamanı tekrar bir mesaj gelene kadar onu duraklatır.

Kod artık tüm mesajları başarıyla gönderip alıyor. Ne yazık ki, hâlâ birkaç sorun var. Birincisi, mesajlar yarım saniyelik aralıklarla gelmiyor; programı başlattıktan 2 saniye (2.000 ms) sonra hepsi birden geliyor. İkincisi, bu program asla çıkmıyor! Bunun yerine, sonsuza kadar yeni mesajları bekliyor. Kapatmak için <span class="keystroke">ctrl-c</span> kullanmanız gerekecek.

Önce, mesajların neden her birinin arasında gecikme olacak şekilde değil de, tüm gecikmeden sonra birden geldiğini inceleyelim. Bir async blokta, kodda `await` anahtar kelimesinin geçtiği sırada, program çalışırken de aynı sırada yürütülür.

Liste 17-10'da sadece bir async blok var, bu yüzden içindeki her şey doğrusal olarak çalışır. Hâlâ eşzamanlılık yok. Tüm `tx.send` çağrıları, aralarına serpiştirilmiş tüm `trpl::sleep` çağrıları ve bunların await noktalarıyla birlikte gerçekleşir. Ancak ondan sonra, `while let` döngüsü `recv` çağrılarındaki await noktalarından geçmeye başlar.

İstediğimiz davranışı elde etmek için, yani her mesaj arasında gecikme olması için, `tx` ve `rx` işlemlerini kendi async bloklarına koymamız gerekir; bu, Liste 17-11'de gösterilmiştir. Sonra çalışma zamanı, sayma örneğinde olduğu gibi, her birini ayrı ayrı çalıştırabilir; bunun için yine `trpl::join` kullanırız. Yine, ayrı future'ları değil, `trpl::join`'un ürettiği sonucu bekleriz. Eğer future'ları sırayla beklersek, yine doğrusal akışa dönmüş oluruz—ki tam olarak _istemediğimiz_ şey budur.

<!-- Bunu test edemiyoruz çünkü asla durmuyor! -->

<Listing number="17-11" caption="`send` ve `recv` işlemlerini kendi `async` bloklarına ayırmak ve bu blokların future'larını beklemek" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

Güncellenmiş kodda, mesajlar 500 milisaniyelik aralıklarla yazdırılır, hepsi birden değil.

Program hâlâ çıkmıyor, çünkü `while let` döngüsünün `trpl::join` ile etkileşiminden dolayı:

- `trpl::join`'dan dönen future, kendisine verilen _her iki_ future tamamlandığında tamamlanır.
- `tx` future'ı, `vals` içindeki son mesajı gönderdikten sonra uyuma işlemi bitince tamamlanır.
- `rx` future'ı, `while let` döngüsü sona erene kadar tamamlanmaz.
- `while let` döngüsü, `rx.recv` beklemesi `None` döndürdüğünde sona erer.
- `rx.recv` beklemesi, ancak kanalın diğer ucu kapandığında `None` döndürür.
- Kanal, ancak `rx.close` çağrılırsa veya gönderici tarafı (`tx`) düşerse kapanır.
- Hiçbir yerde `rx.close` çağırmıyoruz ve `tx`, en dıştaki async blok bitene kadar düşmez.
- Blok, `trpl::join` tamamlanana kadar bitmez; bu da bizi tekrar başa döndürür.

`rx`'i elle kapatmak için bir yerde `rx.close` çağırabiliriz, ama bu çok mantıklı olmaz. Rastgele bir mesaj sayısından sonra durmak programı kapatır, ama mesajları kaçırabiliriz. `tx`'in, fonksiyonun sonundan _önce_ düşmesini sağlayacak başka bir yol bulmamız gerekir.

Şu anda, mesajları gönderen async blok `tx`'i sadece ödünç alıyor, çünkü mesaj göndermek sahiplik gerektirmez; ama eğer `tx`'i o async bloğa taşıyabilseydik, o blok bittiğinde düşerdi. 13. bölümde [Referansları Yakalamak veya Sahipliği Taşımak][capture-or-move]<!-- ignore --> kısmında, closure'larda `move` anahtar kelimesinin nasıl kullanılacağını öğrenmiştiniz ve 16. bölümde [Thread'lerle `move` Closure Kullanmak][move-threads]<!-- ignore --> kısmında, thread'lerle çalışırken veriyi closure'lara taşımamız gerektiğini görmüştük. Aynı temel dinamikler async bloklar için de geçerlidir, bu yüzden `move` anahtar kelimesi async bloklarda da closure'larda olduğu gibi çalışır.

Liste 17-12'de, mesaj göndermek için kullandığımız bloğu `async`'ten `async move`'a çeviriyoruz. _Bu_ kodu çalıştırdığımızda, son mesaj gönderilip alındıktan sonra program düzgünce kapanır.

<Listing number="17-12" caption="Liste 17-11'deki kodun, tamamlandığında düzgünce kapanan revizyonu" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

Bu async kanal aynı zamanda çoklu üretici kanal olduğu için, birden fazla future'dan mesaj göndermek isterseniz `tx` üzerinde `clone` çağırabilirsiniz; bu, Liste 17-13'te gösterilmiştir.

<Listing number="17-13" caption="Async bloklarla çoklu üretici kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

Önce, ilk async bloğun dışında `tx`'i klonlayarak `tx1` oluşturuyoruz. `tx1`'i, tıpkı daha önce `tx` ile yaptığımız gibi, o bloğa taşıyoruz. Daha sonra, orijinal `tx`'i _yeni_ bir async bloğa taşıyoruz ve burada biraz daha yavaş bir gecikmeyle daha fazla mesaj gönderiyoruz. Bu yeni async bloğu, mesajları alan async bloktan sonra koyduk, ama önce de koyabilirdik. Önemli olan, future'ların hangi sırayla bekletildiği; hangi sırayla oluşturuldukları değil.

Mesaj gönderen her iki async blok da `async move` olmalı ki, hem `tx` hem de `tx1` o bloklar bittiğinde düşsün. Aksi takdirde, başta yaşadığımız sonsuz döngüye geri döneriz. Son olarak, ek future için `trpl::join` yerine `trpl::join3` kullanıyoruz.

Artık her iki gönderici future'dan gelen tüm mesajları görebiliyoruz ve future'lar mesaj gönderdikten sonra biraz farklı gecikmeler kullandığı için, mesajlar da o farklı aralıklarla alınıyor.

<!-- Çıktı değişiklikleri önemli olmadığı için çıkartılmadı; değişiklikler büyük olasılıkla thread'lerin farklı çalışmasından kaynaklanıyor, derleyici değişikliklerinden değil -->

```text
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

Bu iyi bir başlangıç, ama bizi sadece birkaç future ile sınırlar: `join` ile iki, `join3` ile üç. Şimdi daha fazla future ile nasıl çalışabileceğimize bakalım.

[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[join-handles]: ch16-01-threads.html#waiting-for-all-threads-to-finish-using-join-handles
[message-passing-threads]: ch16-02-message-passing.html
[if-let]: ch06-03-if-let.html
[capture-or-move]: ch13-01-closures.html#capturing-references-or-moving-ownership
[move-threads]: ch16-01-threads.html#using-move-closures-with-threads
