## Nesne Yönelimli Dillerin Özellikleri

Bir dilin nesne yönelimli sayılması için hangi özelliklere sahip olması gerektiği konusunda programlama topluluğunda bir fikir birliği yoktur. Rust, OOP dahil olmak üzere birçok programlama paradigmasından etkilenmiştir; örneğin, 13. bölümde fonksiyonel programlamadan gelen özellikleri inceledik. Tartışmasız olarak, OOP dilleri bazı ortak özellikleri paylaşır: nesneler, kapsülleme ve kalıtım. Şimdi bu özelliklerin her birinin ne anlama geldiğine ve Rust'ın bunları destekleyip desteklemediğine bakalım.

### Nesneler Veri ve Davranış İçerir

Erich Gamma, Richard Helm, Ralph Johnson ve John Vlissides tarafından yazılan _Design Patterns: Elements of Reusable Object-Oriented Software_ (Addison-Wesley, 1994), yaygın adıyla _Gang of Four_ kitabı, nesne yönelimli tasarım desenlerinin bir kataloğudur. OOP'yi şöyle tanımlar:

> Nesne yönelimli programlar nesnelerden oluşur. Bir **nesne**, hem veriyi hem de bu veriler üzerinde çalışan prosedürleri bir arada paketler. Bu prosedürlere genellikle **metot** veya **işlem** denir.

Bu tanıma göre, Rust nesne yönelimlidir: struct ve enum'lar veri içerir ve `impl` blokları bu struct ve enum'lara metotlar sağlar. Struct ve enum'lar metotlara sahip olsalar da _nesne_ olarak adlandırılmasalar bile, Gang of Four'un nesne tanımına göre aynı işlevselliği sunarlar.

### Uygulama Detaylarını Gizleyen Kapsülleme

OOP ile yaygın olarak ilişkilendirilen bir diğer kavram da _kapsülleme_ (encapsulation)'dır. Kapsülleme, bir nesnenin uygulama detaylarının, o nesneyi kullanan kod tarafından erişilemez olması anlamına gelir. Yani, bir nesneyle etkileşime geçmenin tek yolu onun herkese açık (public) API'sidir; nesneyi kullanan kod, nesnenin iç yapısına erişip veriyi veya davranışı doğrudan değiştirememelidir. Bu sayede, programcı nesnenin içini değiştirebilir ve yeniden düzenleyebilir, kullanan kodda değişiklik yapmasına gerek kalmaz.

Kapsüllemeyi nasıl kontrol edeceğimizi 7. bölümde tartıştık: Kodumuzda hangi modül, tip, fonksiyon ve metotların public olacağına `pub` anahtar kelimesiyle karar verebiliriz; varsayılan olarak her şey gizlidir. Örneğin, içinde `i32` değerlerinden oluşan bir vektör tutan bir `AveragedCollection` struct'ı tanımlayabiliriz. Struct ayrıca, vektördeki değerlerin ortalamasını tutan bir alan da içerebilir; böylece ortalama, her ihtiyaç duyulduğunda tekrar hesaplanmak zorunda kalmaz. Yani, `AveragedCollection` bizim için hesaplanan ortalamayı önbelleğe alır. 18-1 numaralı listede `AveragedCollection` struct'ının tanımı yer alıyor.

<Listing number="18-1" file-name="src/lib.rs" caption="Bir tamsayı listesini ve listedeki öğelerin ortalamasını tutan `AveragedCollection` struct'ı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-01/src/lib.rs}}
```

</Listing>

Struct `pub` olarak işaretlenmiştir, böylece diğer kodlar onu kullanabilir; ancak struct içindeki alanlar gizli kalır. Bu, burada önemlidir çünkü listeye bir değer eklendiğinde veya çıkarıldığında ortalamanın da güncellenmesini sağlamak isteriz. Bunu, struct üzerinde `add`, `remove` ve `average` metotlarını uygulayarak yaparız; 18-2 numaralı listede gösterildiği gibi.

<Listing number="18-2" file-name="src/lib.rs" caption="`AveragedCollection` üzerinde `add`, `remove` ve `average` herkese açık metotlarının implementasyonları">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-02/src/lib.rs:here}}
```

</Listing>

`add`, `remove` ve `average` herkese açık metotları, `AveragedCollection` örneğindeki verilere erişmenin veya onları değiştirmenin tek yoludur. `add` metodu ile `list`'e bir öğe eklendiğinde veya `remove` ile çıkarıldığında, her iki implementasyon da `average` alanını güncelleyen özel `update_average` metodunu çağırır.

`list` ve `average` alanlarını gizli bırakıyoruz; böylece dışarıdan kodun doğrudan `list`'e öğe eklemesi veya çıkarması mümkün olmaz; aksi halde, `average` alanı, `list` değiştiğinde güncel olmayabilir. `average` metodu ise, dışarıdan kodun ortalamayı okumasına izin verir ama değiştirmesine izin vermez.

Uygulama detaylarını gizleme konusunda bir dilin nesne yönelimli sayılması için kapsülleme gerektiği düşünülüyorsa, Rust bu gerekliliği karşılar. Kodun farklı kısımları için `pub` veya başka bir şey kullanma seçeneği, uygulama detaylarının kapsüllenmesini sağlar.

### Kalıtımın Bir Tür Sistemi Olarak ve Kod Paylaşımı Olarak Kullanımı

_Kalıtım_, bir nesnenin başka bir nesnenin tanımından öğeleri miras alabilmesi mekanizmasıdır; böylece ebeveyn nesnenin veri ve davranışlarına, bunları yeniden tanımlamak zorunda kalmadan sahip olunur.

Eğer bir dilin nesne yönelimli olması için kalıtıma sahip olması gerekiyorsa, o zaman Rust böyle bir dil değildir. Bir struct'ın ebeveyn struct'ın alanlarını ve metot implementasyonlarını miras alacak şekilde tanımlanması mümkün değildir; bunun için bir makro kullanmak gerekir.

Ancak, eğer programlama araçlarınızdan biri olarak kalıtıma alışkınsanız, Rust'ta kalıtımın yerini tutacak diğer çözümleri kullanabilirsiniz; bu, kalıtımı kullanma nedeninize bağlıdır.

Kalıtımı iki ana nedenle tercih edersiniz. Birincisi, kodun yeniden kullanımı içindir: bir tür için belirli bir davranış uygulayabilir ve kalıtım, bu implementasyonu farklı bir tür için yeniden kullanmanızı sağlar. Bunu, `Summary` trait'inin `summarize` metodunun varsayılan bir implementasyonunu eklediğimiz 10-14 numaralı listede gördüğünüz gibi, Rust kodunda sınırlı bir şekilde yapabilirsiniz. `Summary` trait'ini implement eden her tür, ek bir koda ihtiyaç duymadan `summarize` metoduna sahip olacaktır. Bu, bir üst sınıfın bir metot implementasyonuna sahip olması ve onu miras alan bir alt sınıfın da metot implementasyonuna sahip olmasıyla benzerdir. Ayrıca, `summarize` metodunun varsayılan implementasyonunu, `Summary` trait'ini implement ederken geçersiz kılabiliriz; bu da, bir alt sınıfın üst sınıfından miras aldığı bir metot implementasyonunu geçersiz kılmasıyla benzerdir.

Kalıtımın diğer bir kullanım amacı ise tür sistemine ilişkindir: bir alt türün, üst türle aynı yerlerde kullanılabilmesini sağlamaktır. Bu aynı zamanda _polimorfizm_ olarak da adlandırılır; belirli özellikleri paylaşan birden fazla nesnenin, çalışma zamanında birbirinin yerine geçebilmesi anlamına gelir.

> ### Polimorfizm
>
> Polimorfizm, birçok kişi için kalıtımla eşanlamlıdır. Ancak, aslında birden fazla türdeki verilerle çalışabilen kodları ifade eden daha genel bir kavramdır. Kalıtımda, bu türler genellikle alt sınıflardır.
>
> Rust bunun yerine, farklı olası türler üzerinde soyutlama yapmak için generics kullanır ve bu türlerin ne sağlaması gerektiği konusunda kısıtlamalar getirmek için trait bounds kullanır. Bu bazen _sınırlı parametrik polimorfizm_ olarak adlandırılır.

Rust, kalıtım sunmamayı tercih ederek farklı bir dizi ödünleşim (tradeoff) seçmiştir. Kalıtım, genellikle gereğinden fazla kod paylaşma riski taşır. Alt sınıflar her zaman üst sınıfın tüm özelliklerini paylaşmamalıdır; ancak kalıtım ile paylaşırlar. Bu, bir programın tasarımını daha az esnek hale getirebilir. Ayrıca, bazı diller yalnızca _tekil kalıtım_ (single inheritance) izni verir; bu da bir alt sınıfın yalnızca bir üst sınıftan miras alabileceği anlamına gelir ve bu da program tasarımının esnekliğini daha da kısıtlar.

Tüm bu nedenlerden dolayı, Rust, polimorfizmi sağlamak için kalıtım yerine trait objeleri kullanma yaklaşımını benimsemiştir. Şimdi trait objelerinin nasıl çalıştığına bakalım.
