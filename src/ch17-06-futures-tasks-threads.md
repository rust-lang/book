## Hepsini Bir Araya Getirmek: Future'lar, Görevler ve Thread'ler

[16. Bölüm][ch16]<!-- ignore -->'de gördüğümüz gibi, thread'ler eşzamanlılık için bir yaklaşım sunar. Bu bölümde ise başka bir yaklaşım gördük: async ile future ve stream kullanmak. Hangi yöntemi ne zaman seçmeniz gerektiğini merak ediyorsanız, cevabı: duruma göre değişir! Hatta çoğu zaman seçim thread _veya_ async değil, thread _ve_ async olur.

Birçok işletim sistemi onlarca yıldır thread tabanlı eşzamanlılık modelleri sunuyor ve birçok programlama dili de bu nedenle thread desteği sağlıyor. Ancak bu modellerin de bazı dezavantajları var. Çoğu işletim sisteminde, her thread için hatırı sayılır miktarda bellek kullanılır ve başlatma/kapatma işlemlerinde ek maliyet oluşur. Ayrıca thread'ler, yalnızca işletim sisteminiz ve donanımınız destekliyorsa kullanılabilir. Ana akım masaüstü ve mobil bilgisayarların aksine, bazı gömülü sistemlerde hiç işletim sistemi yoktur; dolayısıyla thread de yoktur.

Async modeli ise farklı—ve nihayetinde tamamlayıcı—bir dizi avantaj ve dezavantaj sunar. Async modelde, eşzamanlı işlemler kendi thread'lerine ihtiyaç duymaz. Bunun yerine, görevler (task) üzerinde çalışabilirler; örneğin, stream bölümünde senkron bir fonksiyondan iş başlatmak için `trpl::spawn_task` kullandık. Görev, thread'e benzer; ancak işletim sistemi yerine kütüphane seviyesinde, yani çalışma zamanı (runtime) tarafından yönetilir.

Önceki bölümde, async bir kanal kullanarak ve senkron koddan çağrılabilen bir async görev başlatarak bir stream oluşturabileceğimizi gördük. Aynı şeyi bir thread ile de yapabiliriz. 17-40 numaralı listede `trpl::spawn_task` ve `trpl::sleep` kullandık. 17-41 numaralı listede ise bunların yerine standart kütüphanedeki `thread::spawn` ve `thread::sleep` API'lerini `get_intervals` fonksiyonunda kullandık.

<Listing number="17-41" caption="`get_intervals` fonksiyonu için async `trpl` API'leri yerine `std::thread` API'lerini kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-41/src/main.rs:threads}}
```

</Listing>

Bu kodu çalıştırırsanız, çıktısı 17-40 numaralı listeninkiyle aynıdır. Ayrıca, çağıran kod açısından ne kadar az şeyin değiştiğine dikkat edin. Dahası, fonksiyonlardan biri çalışma zamanında async bir görev başlatırken, diğeri işletim sistemi thread'i başlatıyor olsa da, ortaya çıkan stream'ler bu farklardan etkilenmez.

Benzerliklerine rağmen, bu iki yaklaşım çok farklı davranır; ancak bu basit örnekte bunu ölçmek zor olabilir. Modern bir kişisel bilgisayarda milyonlarca async görev başlatabiliriz. Bunu thread'lerle yapmaya kalkarsak, kelimenin tam anlamıyla belleğimiz tükenir!

Bu API'lerin bu kadar benzer olmasının bir nedeni var. Thread'ler, senkron işlemler kümesi için bir sınır oluşturur; eşzamanlılık thread'ler _arasında_ mümkündür. Görevler ise _asenkron_ işlemler kümesi için bir sınırdır; eşzamanlılık hem görevler _arasında_ hem de görevlerin _içinde_ mümkündür, çünkü bir görev gövdesinde farklı future'lar arasında geçiş yapabilir. Son olarak, future'lar Rust'ın en ince taneli eşzamanlılık birimidir ve her future başka future'lardan oluşan bir ağaç olabilir. Çalışma zamanı—özellikle executor'u—görevleri yönetir, görevler de future'ları yönetir. Bu açıdan görevler, işletim sistemi yerine çalışma zamanı tarafından yönetilen hafif thread'lere benzer, ancak ek yeteneklere sahiptir.

Bu, async görevlerin her zaman thread'lerden daha iyi olduğu anlamına gelmez (veya tersi). Thread'lerle eşzamanlılık, bazı açılardan `async` ile eşzamanlılıktan daha basit bir programlama modelidir. Bu, hem avantaj hem de dezavantaj olabilir. Thread'ler biraz "başlat ve unut" tarzındadır; future'a karşılık gelen bir yapıları yoktur, bu yüzden sadece tamamlanana kadar çalışırlar ve yalnızca işletim sistemi tarafından kesintiye uğratılabilirler. Yani, future'larda olduğu gibi _görev içi eşzamanlılık_ desteği yoktur. Rust'taki thread'lerin ayrıca iptal mekanizması da yoktur—bu bölümde açıkça ele almadık ama bir future sona erdiğinde durumunun düzgünce temizlendiğini gördük.

Bu sınırlamalar, thread'leri future'lardan daha zor birleştirilebilir (compose) hale getirir. Örneğin, bu bölümde oluşturduğumuz `timeout` ve `throttle` yardımcılarını thread'lerle yapmak çok daha zordur. Future'ların daha zengin veri yapıları olması, onları doğal olarak birleştirilebilir kılar; bunu da gördük.

Sonuç olarak, görevler future'lar üzerinde _ek_ kontrol sağlar ve onları nasıl ve nerede gruplayacağımızı seçmemize olanak tanır. Ve aslında, thread'ler ve görevler çoğu zaman birlikte çok iyi çalışır; çünkü görevler (en azından bazı çalışma zamanlarında) thread'ler arasında taşınabilir. Aslında, kullandığımız çalışma zamanı—`spawn_blocking` ve `spawn_task` fonksiyonları dahil—varsayılan olarak çoklu thread'lidir! Birçok çalışma zamanı, thread'ler arasındaki yükü dengelemek için _iş çalma_ (work stealing) yaklaşımını kullanır; böylece thread'lerin mevcut kullanımına göre görevler thread'ler arasında şeffafça taşınır ve sistemin genel performansı artar. Bu yaklaşım aslında thread _ve_ görev, dolayısıyla future gerektirir.

Hangi yöntemi ne zaman kullanacağınızı düşünürken şu kuralları göz önünde bulundurun:

- İş _çok paralelleştirilebiliyorsa_, yani her parçası ayrı ayrı işlenebilen büyük bir veri kümesi gibi, thread'ler daha iyi bir seçimdir.
- İş _çok eşzamanlıysa_, yani farklı aralık ve hızlarda gelen çok sayıda kaynaktan mesaj işlemek gibi, async daha iyi bir seçimdir.

Hem paralellik hem de eşzamanlılık gerekiyorsa, thread ve async arasında seçim yapmak zorunda değilsiniz. Her ikisini de özgürce birlikte kullanabilirsiniz; her biri en iyi olduğu rolü oynar. Örneğin, 17-42 numaralı listede, gerçek dünyadaki Rust kodunda bu tür bir karışımın oldukça yaygın bir örneği gösteriliyor.

<Listing number="17-42" caption="Bir thread'de bloklayıcı kodla mesaj gönderip, async blokta mesajları await etmek" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-42/src/main.rs:all}}
```

</Listing>

Önce bir async kanal oluşturuyoruz, ardından kanalın gönderici tarafının sahipliğini alan bir thread başlatıyoruz. Thread içinde, 1'den 10'a kadar sayıları gönderiyoruz ve her biri arasında bir saniye bekliyoruz. Son olarak, bu bölüm boyunca yaptığımız gibi, `trpl::run` ile bir async blok future'ı çalıştırıyoruz. O future'da, tıpkı diğer mesajlaşma örneklerinde olduğu gibi, bu mesajları await ediyoruz.

Bölümün başında ele aldığımız senaryoya dönersek, örneğin video kodlama işlemlerini özel bir thread'de (çünkü video kodlama işlemciye yük bindirir) çalıştırıp, bu işlemlerin tamamlandığını async bir kanal ile arayüze bildirdiğimizi hayal edin. Gerçek dünyada bu tür kombinasyonların sayısız örneği vardır.

## Özet

Bu kitapta eşzamanlılık konusunu son kez görmüyorsunuz. [21. Bölüm][ch21]'deki projede, burada tartışılan basit örneklerden daha gerçekçi bir durumda bu kavramlar uygulanacak ve thread ile görev tabanlı çözüm yolları doğrudan karşılaştırılacak.

Hangi yaklaşımı seçerseniz seçin, Rust size güvenli, hızlı ve eşzamanlı kod yazmak için gereken araçları sunar—ister yüksek verimli bir web sunucusu, ister gömülü bir işletim sistemi için olsun.

Sırada, Rust programlarınız büyüdükçe problemleri modellemenin ve çözümleri yapılandırmanın idiomatik yollarını konuşacağız. Ayrıca, Rust'ın idiomlarının nesne yönelimli programlamadan aşina olabileceğiniz yaklaşımlarla nasıl ilişkili olduğuna değineceğiz.

[ch16]: http://localhost:3000/ch16-00-concurrency.html
[combining-futures]: ch17-03-more-futures.html#building-our-own-async-abstractions
[streams]: ch17-04-streams.html#composing-streams
[ch21]: ch21-00-final-project-a-web-server.html
