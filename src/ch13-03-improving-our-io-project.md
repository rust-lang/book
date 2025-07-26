## G/Ç Projemizi Geliştirmek

Yineleyiciler hakkındaki bu yeni bilgilerimizle, 12. Bölümdeki G/Ç projesini yineleyiciler kullanarak kodun bazı kısımlarını daha açık ve daha öz hale getirebiliriz. Yineleyicilerin `Config::build` fonksiyonumuzu ve `search` fonksiyonumuzu nasıl iyileştirebileceğine bakalım.

### Bir Yineleyici Kullanarak `clone`'u Kaldırmak

12-6 numaralı listede, bir `String` dilimini alıp, dilimden indeksleme ve klonlama yoluyla `Config` yapısının bir örneğini oluşturan kod eklemiştik. Bu, `Config` yapısının bu değerlerin sahipliğini almasını sağlıyordu. 13-17 numaralı listede, 12-23 numaralı listedeki `Config::build` fonksiyonunun uygulamasını yeniden ürettik.

<Listing number="13-17" file-name="src/main.rs" caption="12-23 numaralı listeden `Config::build` fonksiyonunun yeniden üretimi">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/main.rs:ch13}}
```

</Listing>

O zamanlar, verimsiz `clone` çağrıları hakkında endişelenmememizi, bunları ileride kaldıracağımızı söylemiştik. İşte o zaman şimdi!

Burada `clone`'a ihtiyacımız vardı çünkü parametre olarak aldığımız `args` dilimi `String` öğeleri içeriyor, fakat `build` fonksiyonu `args`'ın sahipliğini almıyor. Bir `Config` örneğinin sahipliğini döndürmek için, `Config`'in `query` ve `file_path` alanlarındaki değerleri klonlamamız gerekiyordu ki, `Config` örneği bu değerlerin sahipliğini alabilsin.

Yineleyiciler hakkındaki yeni bilgilerimizle, `build` fonksiyonunu bir dilimi ödünç almak yerine argüman olarak bir yineleyicinin sahipliğini alacak şekilde değiştirebiliriz. Dilimin uzunluğunu kontrol eden ve belirli konumlara indeksleme yapan kod yerine yineleyici işlevselliğini kullanacağız. Bu, `Config::build` fonksiyonunun ne yaptığını daha açık hale getirecek çünkü değerleri yineleyiciyle erişeceğiz.

`Config::build` yineleyicinin sahipliğini alıp, ödünç alan indeksleme işlemlerini bırakınca, `String` değerlerini yineleyiciden `Config`'e taşıyabiliriz ve böylece `clone` çağırıp yeni bir tahsis yapmamıza gerek kalmaz.

#### Döndürülen Yineleyiciyi Doğrudan Kullanmak

G/Ç projenizin _src/main.rs_ dosyasını açın; bu dosya şu şekilde görünmeli:

<span class="filename">Dosya Adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

Önce, 12-24 numaralı listede olan `main` fonksiyonunun başlangıcını, bu kez bir yineleyici kullanan 13-18 numaralı listedeki kodla değiştireceğiz. Bunu yapınca, `Config::build`'ı da güncellemeden derlenmeyecektir.

<Listing number="13-18" file-name="src/main.rs" caption="`env::args`'ın döndürdüğü değeri `Config::build`'a geçirmek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

`env::args` fonksiyonu bir yineleyici döndürür! Yineleyici değerlerini bir vektöre toplayıp sonra `Config::build`'a bir dilim olarak geçirmek yerine, artık `env::args`'ın döndürdüğü yineleyicinin sahipliğini doğrudan `Config::build`'a geçiriyoruz.

Şimdi, `Config::build` fonksiyonunun tanımını güncellememiz gerekiyor. `Config::build`'ın imzasını 13-19 numaralı listedeki gibi değiştirelim. Bu hâlâ derlenmeyecek, çünkü fonksiyon gövdesini de güncellememiz gerekecek.

<Listing number="13-19" file-name="src/main.rs" caption="`Config::build`'ın imzasını bir yineleyici bekleyecek şekilde güncellemek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/main.rs:here}}
```

</Listing>

Standart kütüphane dokümantasyonunda, `env::args` fonksiyonunun döndürdüğü yineleyicinin türünün `std::env::Args` olduğu ve bu türün `Iterator` özelliğini uyguladığı, ayrıca `String` değerler döndürdüğü belirtiliyor.

`Config::build` fonksiyonunun imzasını, `args` parametresi `&[String]` yerine `impl Iterator<Item = String>` trait sınırına sahip genel bir tür olacak şekilde güncelledik. 10. Bölümdeki ["Parametre Olarak Trait'ler"](ch10-02-traits.html#traits-as-parameters) bölümünde tartıştığımız `impl Trait` sözdizimini burada kullanmamız, `args`'ın `Iterator` özelliğini uygulayan ve `String` döndüren herhangi bir tür olabileceği anlamına gelir.

`args`'ın sahipliğini alıp, üzerinde yineleme yapacağımız için, parametre tanımında `mut` anahtar kelimesini de ekleyebiliriz.

#### İndeksleme Yerine `Iterator` Özelliği Metotlarını Kullanmak

Şimdi, `Config::build` fonksiyonunun gövdesini düzeltelim. `args` `Iterator` özelliğini uyguladığı için, üzerinde `next` metodunu çağırabileceğimizi biliyoruz! 13-20 numaralı listede, 12-23 numaralı listedeki kodun `next` metodu ile güncellenmiş halini görebilirsiniz.

<Listing number="13-20" file-name="src/main.rs" caption="`Config::build` gövdesini yineleyici metotlarıyla değiştirmek">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/main.rs:here}}
```

</Listing>

`env::args`'ın döndürdüğü ilk değerin programın adı olduğunu unutmayın. Bunu atlamak ve bir sonraki değere ulaşmak için önce `next` çağırıp dönen değeri kullanmıyoruz. Sonra, `Config`'in `query` alanına koymak istediğimiz değeri almak için tekrar `next` çağırıyoruz. Eğer `next` `Some` döndürürse, değeri `match` ile çıkarıyoruz. Eğer `None` dönerse, yeterli argüman verilmemiş demektir ve erken bir `Err` değeri ile dönüyoruz. Aynı işlemi `file_path` değeri için de yapıyoruz.

### Yineleyici Adaptörleriyle Kodu Daha Açık Hale Getirmek

G/Ç projemizdeki `search` fonksiyonunda da yineleyicilerden faydalanabiliriz; bu fonksiyonun 12-19 numaralı listedeki halini burada 13-21 numaralı listede yeniden ürettik.

<Listing number="13-21" file-name="src/lib.rs" caption="12-19 numaralı listeden `search` fonksiyonunun uygulaması">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

Bu kodu, yineleyici adaptör metotlarını kullanarak daha öz bir şekilde yazabiliriz. Böyle yapmak, arada değiştirilebilir bir `results` vektörüne ihtiyaç duymamızı da ortadan kaldırır. Fonksiyonel programlama tarzı, kodun daha açık olması için değiştirilebilir durumu en aza indirmeyi tercih eder. Değiştirilebilir durumu kaldırmak, gelecekte aramayı paralel yapmak gibi bir geliştirmeyi de kolaylaştırabilir, çünkü `results` vektörüne eşzamanlı erişimi yönetmemiz gerekmez. 13-22 numaralı listede bu değişikliği görebilirsiniz.

<Listing number="13-22" file-name="src/lib.rs" caption="`search` fonksiyonunun uygulamasında yineleyici adaptör metotlarını kullanmak">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

`search` fonksiyonunun amacı, `contents` içindeki `query`'yi içeren tüm satırları döndürmektir. 13-16 numaralı listedeki `filter` örneğine benzer şekilde, bu kod da yalnızca `line.contains(query)` ifadesi `true` döndürdüğü satırları tutmak için `filter` adaptörünü kullanır. Ardından, eşleşen satırları `collect` ile başka bir vektörde toplarız. Çok daha basit! Aynı değişikliği `search_case_insensitive` fonksiyonunda da yapmaktan çekinmeyin.

Daha da ileri bir geliştirme olarak, `collect` çağrısını kaldırıp dönüş türünü `impl Iterator<Item = &'a str>` olarak değiştirerek `search` fonksiyonundan bir yineleyici döndürebilirsiniz; böylece fonksiyon bir yineleyici adaptörü olur. Testleri de güncellemeniz gerekeceğini unutmayın! `minigrep` aracınızı bu değişiklikten önce ve sonra büyük bir dosyada arama yapmak için çalıştırarak davranıştaki farkı gözlemleyin. Bu değişiklikten önce, program tüm sonuçları toplamadıkça hiçbir sonuç yazdırmaz; ancak değişiklikten sonra, `run` fonksiyonundaki `for` döngüsü, yineleyicinin tembelliğinden faydalanarak her eşleşen satırı buldukça yazdırır.

<!-- Eski başlık. Kaldırmayın, bağlantılar bozulabilir. -->

<a id="choosing-between-loops-or-iterators"></a>

### Döngüler ve Yineleyiciler Arasında Seçim Yapmak

Bir sonraki mantıklı soru, kendi kodunuzda hangi tarzı seçmeniz gerektiği ve nedenidir: 13-21 numaralı listedeki orijinal uygulama mı, yoksa 13-22 numaralı listedeki yineleyici kullanan sürüm mü (tüm sonuçları döndürmeden önce topladığımızı varsayarsak)? Çoğu Rust programcısı yineleyici tarzını kullanmayı tercih eder. Başta alışmak biraz zor olabilir, ancak çeşitli yineleyici adaptörlerine ve ne yaptıklarına alışınca, yineleyiciler daha anlaşılır olabilir. Döngü ve yeni vektörler oluşturma gibi ayrıntılarla uğraşmak yerine, kod döngünün yüksek seviyeli amacına odaklanır. Bu, sıradan kodun bir kısmını soyutlayarak, yineleyicideki her öğenin geçmesi gereken filtreleme koşulu gibi bu koda özgü kavramları daha net görmenizi sağlar.

Peki, iki uygulama gerçekten eşdeğer mi? Sezgisel olarak, daha düşük seviyeli döngünün daha hızlı olacağı düşünülebilir. Şimdi performanstan bahsedelim.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
