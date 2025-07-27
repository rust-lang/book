## Paylaşılan Durumlu Eşzamanlılık (Shared-State Concurrency)

Mesajlaşma, eşzamanlılığı yönetmek için iyi bir yol olsa da tek yol değildir. Bir diğer yöntem, birden fazla thread'in aynı paylaşılan veriye erişmesidir. Go dili dokümantasyonundaki şu sloganı tekrar düşünün: “Belleği paylaşarak iletişim kurmayın.”

Peki, belleği paylaşarak iletişim kurmak nasıl olurdu? Ayrıca, mesajlaşma yanlılarının bellek paylaşımını kullanmamaya neden karşı çıktığını düşünelim.

Bir bakıma, herhangi bir programlama dilindeki kanallar tekil sahipliğe benzer; çünkü bir değeri bir kanaldan aktardıktan sonra, o değeri artık kullanmamalısınız. Paylaşılan bellekli eşzamanlılık ise çoklu sahipliğe benzer: birden fazla thread aynı bellek konumuna aynı anda erişebilir. 15. bölümde, akıllı işaretçilerin çoklu sahipliği mümkün kıldığını görmüştünüz; çoklu sahiplik, bu farklı sahiplerin yönetilmesi gerektiği için karmaşıklık katabilir. Rust'ın tip sistemi ve sahiplik kuralları, bu yönetimin doğru yapılmasına büyük ölçüde yardımcı olur. Örneğin, paylaşılan bellek için en yaygın eşzamanlılık ilkel araçlarından biri olan mutex'lere bakalım.

### Veriye Aynı Anda Sadece Bir Thread'in Erişebilmesi için Mutex Kullanmak

_Mutex_, _mutual exclusion_ (karşılıklı dışlama) ifadesinin kısaltmasıdır; yani bir mutex, herhangi bir anda yalnızca bir thread'in belirli bir veriye erişmesine izin verir. Bir thread, mutex içindeki veriye erişmek için önce mutex'in kilidini almak istediğini belirtmelidir. _Kilit_ (lock), mutex'in bir parçası olan ve şu anda veriye kimin özel erişimi olduğunu takip eden bir veri yapısıdır. Bu nedenle, mutex'in tuttuğu veriyi kilitleme sistemiyle _koruduğu_ (guarding) söylenir.

Mutex'ler, iki kuralı hatırlamanız gerektiği için zor kullanılır olarak bilinir:

1. Veriyi kullanmadan önce kilidi almaya çalışmalısınız.
2. Mutex'in koruduğu veriyi kullanmayı bitirdiğinizde, diğer thread'lerin kilidi alabilmesi için kilidi bırakmalısınız.

Gerçek hayattan bir benzetme olarak, bir konferansta tek bir mikrofonun olduğu bir panel tartışmasını düşünün. Bir panelist konuşmak istediğinde, mikrofonu kullanmak istediğini belirtmeli veya istemelidir. Mikrofonu aldığında, istediği kadar konuşabilir ve ardından mikrofonu konuşmak isteyen bir sonraki paneliste verir. Eğer bir panelist mikrofonu elden bırakmayı unutursa, kimse konuşamaz. Paylaşılan mikrofonun yönetimi yanlış yapılırsa, panel planlandığı gibi işlemez!

Mutex yönetimini doğru yapmak oldukça zor olabilir; bu yüzden birçok kişi kanalları tercih eder. Ancak Rust'ın tip sistemi ve sahiplik kuralları sayesinde, kilitleme ve kilit açmayı yanlış yapamazsınız.

#### `Mutex<T>` API'si

Bir mutex'in nasıl kullanılacağını göstermek için, önce tek thread'li bir bağlamda bir mutex kullanalım (16-12 numaralı liste).

<Listing number="16-12" file-name="src/main.rs" caption="Basitlik için tek thread'de `Mutex<T>` API'sini keşfetmek">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

</Listing>

Birçok türde olduğu gibi, `Mutex<T>`'yi `new` ilişkili fonksiyonuyla oluşturuyoruz. Mutex içindeki veriye erişmek için, `lock` metodunu kullanarak kilidi alırız. Bu çağrı, kilidi alma sırası bize gelene kadar mevcut thread'in çalışmasını durdurur.

Eğer başka bir thread kilidi tutarken panic yaparsa, `lock` çağrısı başarısız olur. Bu durumda, kimse kilidi alamaz; bu yüzden burada `unwrap` ile panic olmasını sağladık.

Kilit aldıktan sonra, dönen değeri (burada `num` olarak adlandırılmış) mutex içindeki veriye yönelik değiştirilebilir bir referans olarak kullanabiliriz. Tip sistemi, `m` içindeki değeri kullanmadan önce kilit almamızı garanti eder. `m`'nin tipi `Mutex<i32>`'dir, `i32` değil; bu yüzden içteki `i32`'ye erişmek için _mutlaka_ `lock` çağrısı yapmalıyız. Aksi halde tip sistemi içteki `i32`'ye erişmemize izin vermez.

`lock` çağrısı, `MutexGuard` adlı bir tür döndürür; bu, `LockResult` ile sarılmıştır ve biz burada `unwrap` ile açtık. `MutexGuard` türü, iç veriye işaret etmek için `Deref` uygular; ayrıca, bir `MutexGuard` scope dışına çıktığında kilidi otomatik olarak serbest bırakan bir `Drop` implementasyonuna sahiptir. Sonuç olarak, kilidi serbest bırakmayı unutup diğer thread'lerin mutex'i kullanmasını engelleme riskimiz yoktur; kilit otomatik olarak bırakılır.

Kilit bırakıldıktan sonra, mutex değerini yazdırabiliriz ve içteki `i32`'yi `6`'ya değiştirebildiğimizi görebiliriz.

#### Birden Fazla Thread Arasında `Mutex<T>` Paylaşmak

Şimdi, bir değeri birden fazla thread arasında `Mutex<T>` ile paylaşmayı deneyelim. 10 thread başlatıp, her birinin sayaç değerini 1 artırmasını sağlayacağız; sayaç 0'dan 10'a çıkacak. 16-13 numaralı örnek derleyici hatası verecek ve bu hatayı kullanarak `Mutex<T>`'yi nasıl doğru kullanacağımızı ve Rust'ın bize nasıl yardımcı olduğunu göreceğiz.

<Listing number="16-13" file-name="src/main.rs" caption="Her biri `Mutex<T>` ile korunan bir sayacı artıran 10 thread">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

</Listing>

Burada, 16-12'de olduğu gibi bir `i32`'yi `Mutex<T>` içinde tutan bir `counter` değişkeni oluşturuyoruz. Sonra, bir sayı aralığı üzerinde yineleyerek 10 thread oluşturuyoruz. Tüm thread'lere aynı closure'ı veriyoruz: counter'ı thread'e taşıyan, `Mutex<T>` üzerinde `lock` çağrısı ile kilidi alan ve ardından mutex içindeki değeri 1 artıran bir closure. Bir thread closure'ı çalıştırmayı bitirdiğinde, `num` scope dışına çıkar ve kilidi bırakır; böylece başka bir thread kilidi alabilir.

Ana thread'de, tüm join handle'ları topluyoruz. Sonra, 16-2'de olduğu gibi, her handle üzerinde `join` çağırarak tüm thread'lerin bitmesini sağlıyoruz. O noktada, ana thread kilidi alır ve programın sonucunu yazdırır.

Bu örneğin derlenmeyeceğini söylemiştik. Şimdi nedenini görelim!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

Hata mesajı, `counter` değerinin döngünün önceki yinelemesinde taşındığını söylüyor. Rust, kilit counter'ın sahipliğini birden fazla thread'e taşıyamayacağımızı belirtiyor. 15. bölümde tartıştığımız çoklu sahiplik yöntemiyle derleyici hatasını düzeltelim.

#### Çoklu Thread ile Çoklu Sahiplik

15. bölümde, bir değeri birden fazla sahipli yapmak için `Rc<T>` akıllı işaretçisini kullanmıştık. Burada da aynısını yapalım ve ne olacağını görelim. 16-14 numaralı listede, `Mutex<T>`'yi `Rc<T>` ile sarıp, sahipliği thread'e taşımadan önce `Rc<T>`'yi klonluyoruz.

<Listing number="16-14" file-name="src/main.rs" caption="Birden fazla thread'in `Mutex<T>`'ye sahip olmasını sağlamak için `Rc<T>` kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

</Listing>

Yine derliyoruz ve... farklı hatalar alıyoruz! Derleyici bize çok şey öğretiyor.

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

Hata mesajının önemli kısmı şudur: `` `Rc<Mutex<i32>>` cannot be sent between threads safely ``. Derleyici ayrıca nedenini de söylüyor: `` the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``. `Send`'i bir sonraki bölümde ele alacağız; bu trait, thread'lerle kullandığımız türlerin eşzamanlı durumlar için uygun olmasını sağlar.

Ne yazık ki, `Rc<T>` thread'ler arasında paylaşmak için güvenli değildir. `Rc<T>`, referans sayısını yönetirken her `clone` çağrısında sayıyı artırır ve her clone düşürüldüğünde azaltır. Ancak, sayının değişimini başka bir thread'in kesemeyeceğinden emin olmak için herhangi bir eşzamanlılık ilkel aracı kullanmaz. Bu, yanlış sayılara—ve dolayısıyla bellek sızıntılarına veya bir değerin işimiz bitmeden düşürülmesine—yol açabilir. İhtiyacımız olan şey, `Rc<T>` gibi bir tür, ancak referans sayısını thread-güvenli şekilde değiştiren bir tür.

#### `Arc<T>` ile Atomik Referans Sayımı

Neyse ki, `Arc<T>` tam da `Rc<T>` gibi, ancak eşzamanlı durumlarda güvenli olan bir türdür. Buradaki _a_, _atomic_ (atomik) anlamına gelir; yani _atomik referans sayımlı_ bir türdür. Atomikler, burada ayrıntılı olarak ele almayacağımız ek bir eşzamanlılık ilkel aracıdır; daha fazla bilgi için [`std::sync::atomic`][atomic]<!-- ignore --> standart kütüphane dokümantasyonuna bakabilirsiniz. Şimdilik, atomiklerin ilkel türler gibi çalıştığını, ancak thread'ler arasında paylaşmak için güvenli olduklarını bilmeniz yeterli.

O zaman neden tüm ilkel türler atomik değil ve neden standart kütüphane türleri varsayılan olarak `Arc<T>` kullanmıyor diye sorabilirsiniz. Bunun nedeni, thread güvenliğinin bir performans maliyeti getirmesidir ve bu maliyeti yalnızca gerçekten ihtiyaç duyduğunuzda ödemek istersiniz. Eğer yalnızca tek bir thread'de değerler üzerinde işlem yapıyorsanız, atomiklerin sağladığı garantileri uygulamak zorunda kalmazsanız kodunuz daha hızlı çalışır.

Örneğimize dönersek: `Arc<T>` ve `Rc<T>` aynı API'ye sahiptir, bu yüzden programımızı düzeltmek için sadece `use` satırını, `new` çağrısını ve `clone` çağrısını değiştiriyoruz. 16-15 numaralı kod nihayet derlenecek ve çalışacaktır.

<Listing number="16-15" file-name="src/main.rs" caption="Birden fazla thread arasında sahiplik paylaşmak için `Mutex<T>`'yi `Arc<T>` ile sarmak">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

</Listing>

Bu kod şunu yazdıracaktır:

```text
Result: 10
```

Başardık! 0'dan 10'a kadar saydık; bu çok etkileyici görünmeyebilir, ama bize `Mutex<T>` ve thread güvenliği hakkında çok şey öğretti. Bu programın yapısını, sadece sayaç artırmak yerine daha karmaşık işlemler yapmak için de kullanabilirsiniz. Bu stratejiyle, bir hesaplamayı bağımsız parçalara bölebilir, bu parçaları thread'lere dağıtabilir ve ardından her thread'in kendi parçasını sonuca eklemesi için bir `Mutex<T>` kullanabilirsiniz.

Eğer yalnızca basit sayısal işlemler yapıyorsanız, standart kütüphanenin [`std::sync::atomic` modülünde][atomic]<!-- ignore -->, `Mutex<T>`'den daha basit türler de vardır. Bu türler, ilkel türlere güvenli, eşzamanlı, atomik erişim sağlar. Bu örnekte, `Mutex<T>`'yi ilkel bir türle kullandık ki, esas olarak `Mutex<T>`'nin nasıl çalıştığına odaklanabilelim.

### `RefCell<T>`/`Rc<T>` ile `Mutex<T>`/`Arc<T>` Arasındaki Benzerlikler

Dikkat etmiş olabilirsiniz, `counter` değişkeni değiştirilemez (immutable) olmasına rağmen, içindeki değere değiştirilebilir bir referans alabiliyoruz; bu, `Mutex<T>`'nin, `Cell` ailesinde olduğu gibi içsel değiştirilebilirlik (interior mutability) sağladığı anlamına gelir. 15. bölümde, `Rc<T>` içinde içerikleri değiştirmek için `RefCell<T>` kullandığımız gibi, burada da `Arc<T>` içinde içerikleri değiştirmek için `Mutex<T>` kullanıyoruz.

Bir diğer önemli detay, Rust'ın `Mutex<T>` kullanırken tüm mantık hatalarından sizi koruyamamasıdır. 15. bölümde, `Rc<T>` kullanmanın referans döngüsü oluşturma riski taşıdığını ve bunun bellek sızıntısına yol açabileceğini hatırlayın. Benzer şekilde, `Mutex<T>` kullanmak da _deadlock_ (kilitlenme) riski taşır. Bu, bir işlem iki kaynağı kilitlemek istediğinde ve iki thread'in her biri bir kilidi aldığında, birbirlerini sonsuza kadar beklemelerine neden olur. Deadlock'larla ilgileniyorsanız, Rust'ta deadlock oluşturan bir program yazmayı deneyin; ardından herhangi bir dilde mutex'ler için deadlock önleme stratejilerini araştırıp Rust'ta uygulamaya çalışın. `Mutex<T>` ve `MutexGuard` için standart kütüphane API dokümantasyonu faydalı bilgiler sunar.

Bu bölümü, `Send` ve `Sync` trait'leri ile bunları özel türlerle nasıl kullanabileceğimizi anlatarak tamamlayacağız.

[atomic]: ../std/sync/atomic/index.html
