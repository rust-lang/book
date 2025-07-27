## Thread'ler Arasında Veri Aktarmak için Mesajlaşma Kullanmak

Güvenli eşzamanlılığı sağlamak için giderek daha popüler hale gelen yaklaşımlardan biri _mesajlaşma_ (message passing)'dır; burada thread'ler veya aktörler, veri içeren mesajlar göndererek birbirleriyle iletişim kurar. [Go dili dokümantasyonundaki](https://golang.org/doc/effective_go.html#concurrency) şu sloganda olduğu gibi: “Belleği paylaşarak iletişim kurmayın; iletişim kurarak belleği paylaşın.”

Mesaj göndermeli eşzamanlılığı sağlamak için, Rust'ın standart kütüphanesi kanal (channel) implementasyonu sunar. _Kanal_ (channel), bir thread'den diğerine veri gönderilen genel bir programlama kavramıdır.

Programlamada bir kanalı, yönlü bir su kanalı gibi düşünebilirsiniz; örneğin bir dere veya nehir. Bir lastik ördek gibi bir şeyi nehre bırakırsanız, suyun akışıyla aşağıya doğru yol alır ve nehrin sonunda ulaşır.

Bir kanalın iki yarısı vardır: bir gönderici (transmitter) ve bir alıcı (receiver). Gönderici, lastik ördeği nehre bıraktığınız yukarı akıştaki noktadır; alıcı ise ördeğin aşağıda ulaştığı noktadır. Kodunuzun bir bölümü, göndermek istediğiniz verilerle gönderici üzerinde metotlar çağırır; diğer bir bölüm ise gelen mesajlar için alıcıyı kontrol eder. Gönderici veya alıcıdan biri düşürülürse, kanal _kapanmış_ (closed) olur.

Burada, bir thread'in değerler üretip bunları bir kanaldan göndereceği ve diğer thread'in bu değerleri alıp yazdıracağı bir program oluşturacağız. Özelliği göstermek için thread'ler arasında basit değerler göndereceğiz. Bu tekniğe alıştıktan sonra, thread'lerin birbirleriyle iletişim kurması gereken her türlü durumda (örneğin bir sohbet sistemi veya birçok thread'in bir hesaplamanın parçalarını yapıp, sonuçları birleştiren bir thread'e göndermesi gibi) kanalları kullanabilirsiniz.

Öncelikle, 16-6 numaralı listede bir kanal oluşturacağız ama henüz bir şey yapmayacağız. Dikkat edin, bu kod henüz derlenmeyecek; çünkü Rust, kanal üzerinden hangi türde değer göndermek istediğimizi anlayamıyor.

<Listing number="16-6" file-name="src/main.rs" caption="Bir kanal oluşturup iki yarısını `tx` ve `rx`'e atamak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

</Listing>

Yeni bir kanal oluşturmak için `mpsc::channel` fonksiyonunu kullanıyoruz; `mpsc`, _multiple producer, single consumer_ (çoklu üretici, tekli tüketici) anlamına gelir. Kısaca, Rust'ın standart kütüphanesindeki kanal implementasyonu, bir kanalın birden fazla _gönderici_ ucu olabileceği, ancak yalnızca bir _alıcı_ ucu olabileceği anlamına gelir. Birden fazla dere birleşip tek bir büyük nehir oluşturuyormuş gibi: herhangi bir dereden gönderilen her şey, sonunda tek bir nehirde toplanır. Şimdilik tek bir üreticiyle başlayacağız, ancak örnek çalışınca birden fazla üretici ekleyeceğiz.

`mpsc::channel` fonksiyonu bir tuple döndürür; ilk eleman gönderici (transmitter), ikinci eleman ise alıcıdır (receiver). `tx` ve `rx` kısaltmaları, birçok alanda _transmitter_ ve _receiver_ için geleneksel olarak kullanılır; bu yüzden değişkenlerimizi bu şekilde adlandırıyoruz. `let` ifadesinde tuple'ı parçalayan bir desen (pattern) kullanıyoruz; 19. bölümde desenlerin ve destructuring'in detaylarını göreceğiz. Şimdilik, bu şekilde bir `let` ifadesiyle, `mpsc::channel`'dan dönen tuple'ın parçalarını kolayca alabileceğimizi bilin.

Şimdi, gönderici ucunu (tx) bir spawned thread'e taşıyıp, bir string göndermesini sağlayalım; böylece spawned thread ana thread ile iletişim kuracak (16-7 numaralı liste). Bu, yukarı akışa bir lastik ördek bırakmak veya bir thread'den diğerine sohbet mesajı göndermek gibidir.

<Listing number="16-7" file-name="src/main.rs" caption='`tx`'i spawned thread'e taşıyıp, "hi" göndermek'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

</Listing>

Yine, yeni bir thread oluşturmak için `thread::spawn` kullanıyoruz ve ardından `move` ile `tx`'i closure'a taşıyoruz, böylece spawned thread `tx`'in sahibi oluyor. Mesaj gönderebilmek için spawned thread'in göndericiye sahip olması gerekir.

Göndericinin, göndermek istediğimiz değeri alan bir `send` metodu vardır. `send` metodu, `Result<T, E>` türünde bir değer döndürür; eğer alıcı daha önce düşürülmüşse ve gönderilecek bir yer yoksa, gönderme işlemi hata döndürür. Bu örnekte, hata durumunda panic yapmak için `unwrap` çağırıyoruz. Gerçek bir uygulamada ise uygun şekilde ele almak gerekir; 9. bölüme dönerek hata yönetimi stratejilerini gözden geçirebilirsiniz.

16-8 numaralı listede, ana thread'de alıcıdan değeri alacağız. Bu, nehrin sonunda lastik ördeği almak veya bir sohbet mesajı almak gibidir.

<Listing number="16-8" file-name="src/main.rs" caption='Ana thread'de "hi" değerini alıp yazdırmak'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

</Listing>

Alıcının iki kullanışlı metodu vardır: `recv` ve `try_recv`. Burada, ana thread'in çalışmasını durdurup bir değer gelene kadar beklemesini sağlayan `recv` (receive) metodunu kullanıyoruz. Bir değer gönderildiğinde, `recv` onu bir `Result<T, E>` olarak döndürür. Gönderici kapatıldığında, `recv` artık değer gelmeyeceğini bildiren bir hata döndürür.

`try_recv` metodu ise bloklamaz; hemen bir `Result<T, E>` döndürür: bir mesaj varsa `Ok`, yoksa `Err` döner. Eğer bu thread'in mesaj beklerken başka işleri de varsa, `try_recv` kullanmak faydalı olur: belirli aralıklarla `try_recv` çağıran, mesaj varsa işleyen, yoksa başka işler yapan bir döngü yazabilirsiniz.

Bu örnekte, ana thread'in başka işi olmadığı için basitlik adına `recv` kullandık; ana thread'i bloklamak uygundur.

16-8 numaralı kodu çalıştırdığımızda, ana thread'den şu çıktıyı görürüz:

```text
Got: hi
```

Harika!

### Kanallar ve Sahiplik Aktarımı

Sahiplik kuralları, mesaj göndermede önemli bir rol oynar; çünkü güvenli, eşzamanlı kod yazmanıza yardımcı olur. Eşzamanlı programlamada hataları önlemek, Rust programlarınızda sahipliği düşünmenin avantajıdır. Kanallar ve sahipliğin birlikte nasıl çalıştığını göstermek için bir deney yapalım: Bir değeri (`val`) kanaldan gönderdikten _sonra_ spawned thread'de kullanmaya çalışalım. 16-9 numaralı kodu derlemeye çalışırsanız neden izin verilmediğini göreceksiniz.

<Listing number="16-9" file-name="src/main.rs" caption="Bir değeri kanaldan gönderdikten sonra kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

</Listing>

Burada, `val`'i `tx.send` ile kanaldan gönderdikten sonra yazdırmaya çalışıyoruz. Buna izin vermek kötü bir fikir olurdu: Değer başka bir thread'e gönderildikten sonra, o thread değeri değiştirebilir veya düşürebilir; biz tekrar kullanmaya çalıştığımızda veri tutarsız veya yok olabilir. Ancak, 16-9'u derlemeye çalışırsak Rust hata verir:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

Eşzamanlılık hatamız derleme zamanı hatasına yol açtı. `send` fonksiyonu parametresinin sahipliğini alır ve değer taşındığında alıcı onun sahibi olur. Bu, gönderdikten sonra değeri yanlışlıkla tekrar kullanmamızı engeller; sahiplik sistemi her şeyin yolunda olduğunu kontrol eder.

### Birden Fazla Değer Göndermek ve Alıcının Beklediğini Görmek

16-8 numaralı kod derlendi ve çalıştı, ancak iki ayrı thread'in kanal üzerinden konuştuğunu açıkça göstermedi.

16-10 numaralı listede, kodun gerçekten eşzamanlı çalıştığını kanıtlayacak bazı değişiklikler yaptık: spawned thread artık birden fazla mesaj gönderecek ve her mesaj arasında bir saniye bekleyecek.

<Listing number="16-10" file-name="src/main.rs" caption="Birden fazla mesaj göndermek ve her biri arasında beklemek">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

</Listing>

Bu sefer, spawned thread'de ana thread'e göndermek istediğimiz string'lerden oluşan bir vektör var. Bunların üzerinde döngüyle ilerliyor, her birini tek tek gönderiyor ve her gönderimden sonra `thread::sleep` ile bir saniye bekliyor.

Ana thread'de ise artık `recv` fonksiyonunu açıkça çağırmıyoruz; bunun yerine, `rx`'i bir iterator olarak kullanıyoruz. Her alınan değer için onu yazdırıyoruz. Kanal kapandığında, iterasyon sona eriyor.

16-10 numaralı kodu çalıştırdığınızda, her satır arasında bir saniye bekleyerek şu çıktıyı görmelisiniz:

```text
Got: hi
Got: from
Got: the
Got: thread
```

Ana thread'deki `for` döngüsünde herhangi bir bekleme veya gecikme kodu olmadığı için, ana thread'in spawned thread'den değer almak için beklediğini görebiliyoruz.

### Göndericiyi Klonlayarak Birden Fazla Üretici Oluşturmak

Daha önce, `mpsc`'nin _multiple producer, single consumer_ (çoklu üretici, tekli tüketici) anlamına geldiğinden bahsetmiştik. Şimdi, 16-10 numaralı kodu genişletip, aynı alıcıya değer gönderen birden fazla thread oluşturalım. Bunu, göndericiyi klonlayarak yapabiliriz (16-11 numaralı liste).

<Listing number="16-11" file-name="src/main.rs" caption="Birden fazla üreticiden birden fazla mesaj göndermek">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

</Listing>

Bu sefer, ilk spawned thread'i oluşturmadan önce göndericiyi klonluyoruz. Böylece, ilk spawned thread'e verebileceğimiz yeni bir gönderici elde ediyoruz. Orijinal göndericiyi ise ikinci spawned thread'e veriyoruz. Böylece, her biri farklı mesajlar gönderen iki thread'imiz ve tek bir alıcımız oluyor.

Kodu çalıştırdığınızda, çıktınız şöyle bir şey olmalı:

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

Sisteminizin durumuna göre değerleri farklı bir sırada görebilirsiniz. Bu, eşzamanlılığı hem ilginç hem de zor yapan şeydir. Farklı thread'lerde `thread::sleep`'e çeşitli değerler verirseniz, her çalıştırmada daha da öngörülemez ve farklı çıktılar elde edersiniz.

Artık kanalların nasıl çalıştığını gördüğümüze göre, farklı bir eşzamanlılık yöntemine bakalım.
