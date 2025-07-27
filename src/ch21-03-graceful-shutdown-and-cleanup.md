## Zarif Kapatma ve Temizlik

21-20 Listesi'ndeki kod, amaçladığımız gibi bir iş parçacığı havuzu kullanarak isteklere eşzamanlı olarak yanıt veriyor. `workers`, `id` ve `thread` alanlarını doğrudan kullanmadığımız için bazı uyarılar alıyoruz; bu da aslında hiçbir şeyi temizlemediğimizi hatırlatıyor. Ana iş parçacığını durdurmak için daha az zarif olan <kbd>ctrl</kbd>-<kbd>c</kbd> yöntemini kullandığımızda, diğer tüm iş parçacıkları da hemen durdurulur; hatta bir isteği işlerken bile.

Şimdi, havuzdaki her iş parçacığı üzerinde `join` çağrısı yapmak için `Drop` trait'ini uygulayacağız; böylece iş parçacıkları çalıştıkları istekleri bitirebilecek ve ardından kapanacak. Sonra, iş parçacıklarına yeni istek kabul etmeyi bırakmaları ve kapanmaları gerektiğini bildiren bir yol ekleyeceğiz. Bu kodu çalışırken görmek için, sunucumuzu yalnızca iki isteği kabul edecek ve ardından iş parçacığı havuzunu zarifçe kapatacak şekilde değiştireceğiz.

Dikkat edilmesi gereken bir şey: Buradaki hiçbir şey, closure'ların çalıştırılmasını yöneten kodu etkilemez; yani burada yaptığımız her şey, bir async çalışma zamanı için iş parçacığı havuzu kullansaydık da aynı olurdu.

### ThreadPool Üzerinde `Drop` Trait'ini Uygulamak

Thread havuzumuz üzerinde `Drop` uygulamakla başlayalım. Havuz bırakıldığında, iş parçacıklarımızın tümü, yaptıkları işleri bitirmek için bir araya gelmelidir. 21-22 Listesi, `Drop` uygulamasına yönelik ilk denemeyi göstermektedir; bu kod henüz tam olarak çalışmayacaktır.

<Listing number="21-22" file-name="src/lib.rs" caption="Thread havuzu kapsam dışına çıktığında her iş parçacığını birleştirme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

Öncelikle, her bir iş parçacığı havuzu `workers` üzerinden döngü yapıyoruz. Bunu yapmak için `&mut` kullanıyoruz çünkü `self` değişken bir referans ve ayrıca `worker` değişkenini de değiştirmemiz gerekiyor. Her bir işçi için, bu belirli `Worker` örneğinin kapanmakta olduğunu belirten bir mesaj yazdırıyoruz ve ardından `Worker` örneğinin iş parçacığına `join` çağrısı yapıyoruz. `join` çağrısı başarısız olursa, Rust'ın panik yapması ve kontrolsüz bir kapanışa girmesi için `unwrap` kullanıyoruz.

Bu kodu derlediğimizde aldığımız hata şudur:

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

Hata, `join` çağrısı yapamayacağımızı çünkü her bir `worker` üzerinde yalnızca değişken bir ödünç alma işlemi gerçekleştirdiğimizi ve `join`'in argümanının sahipliğini aldığını söylüyor. Bu sorunu çözmek için, `join`'in iş parçacığını tüketebilmesi için `worker`'ın `thread` alanından iş parçacığını çıkarmamız gerekiyor. Bunu, 18-15 Listesi'nde yaptığımız aynı yaklaşımı benimseyerek yapabiliriz. `Worker`, bir `Option<thread::JoinHandle<()>>` tutuyorsa, `Option` üzerindeki `take` yöntemini çağırarak değeri `Some` varyantından çıkarabilir ve yerine `None` varyantını bırakabiliriz. Diğer bir deyişle, çalışan bir `Worker`, `thread` içinde bir `Some` varyantına sahip olurdu ve bir `Worker`'ı temizlemek istediğimizde, `Some`'ı `None` ile değiştirerek `Worker`'ın çalışacak bir iş parçacığına sahip olmamasını sağlardık.

Ancak, bu durum yalnızca `Worker`'ı bırakırken ortaya çıkacaktır. Bunun karşılığında, `worker.thread`'e eriştiğimiz her yerde bir `Option<thread::JoinHandle<()>>` ile başa çıkmak zorunda kalacağız. İdiomatik Rust, `Option`'ı oldukça fazla kullanır, ancak bir şeyi `Option` içinde sarmalamanız gerektiğinde ve bunun bir çözüm olarak ortaya çıktığında, alternatif yaklaşımlar aramak iyi bir fikirdir. Bu, kodunuzu daha temiz ve daha az hata yapma olasılığıyla yazmanıza yardımcı olabilir.

Bu durumda, daha iyi bir alternatif vardır: `Vec::drain` yöntemi. Bu yöntem, hangi öğelerin `Vec`'den çıkarılacağını belirtmek için bir aralık parametresi kabul eder ve bu öğelerin bir iteratörünü döndürür. `..` aralık sözdizimini geçirmek, `Vec`'den _herhangi birini_ çıkarmak anlamına gelir.

Bu nedenle, `ThreadPool`'un `drop` uygulamasını şu şekilde güncellememiz gerekiyor:

<Listing file-name="src/lib.rs">

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

Bu, derleyici hatasını çözer ve kodumuzda başka herhangi bir değişiklik gerektirmez.

### İş Parçacıklarına İş Beklemeyi Bırakmalarını Bildirmek

Yaptığımız tüm değişikliklerle birlikte, kodumuz uyarı vermeden derleniyor. Ancak, kötü haber şu ki, bu kod henüz istediğimiz gibi çalışmıyor. Anahtar, `Worker` örneklerinin içinde çalışan closure'larda: şu anda `join` çağırıyoruz, ancak bu, iş parçacıklarını sonsuz döngüde iş beklerken kapatmayacaktır. Mevcut `drop` uygulamamızla `ThreadPool`'u bırakmaya çalıştığımızda, ana iş parçacığı ilk iş parçacığının bitmesini beklerken sonsuz döngüye girecektir.

Bu sorunu çözmek için, önce `ThreadPool`'un `drop` uygulamasında bir değişiklik yapmamız ve ardından `Worker` döngüsünde bir değişiklik yapmamız gerekecek.

Öncelikle, `ThreadPool`'un `drop` uygulamasını, iş parçacıklarının bitmesini beklemeden önce `sender`'ı açıkça bırakacak şekilde değiştireceğiz. 21-23 Listesi, `ThreadPool`'a `sender`'ı açıkça bırakacak şekilde yapılan değişiklikleri göstermektedir. İş parçacığı ile birlikte burada, `ThreadPool`'dan `Option::take` ile `sender`'ı çıkarabilmek için bir `Option` kullanmamız gerekiyor.

<Listing number="21-23" file-name="src/lib.rs" caption="Worker iş parçacıklarını birleştirmeden önce `sender`'ı açıkça bırakma">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

`sender`'ı bırakmak, kanalı kapatır; bu da daha fazla mesaj gönderilmeyeceğini gösterir. Bu olduğunda, `Worker` örneklerinin sonsuz döngülerinde yaptıkları tüm `recv` çağrıları bir hata döndürecektir. 21-24 Listesi'nde, `Worker` döngüsünü bu durumda döngüden zarifçe çıkacak şekilde değiştiriyoruz; bu, iş parçacıkları `ThreadPool`'un `drop` uygulaması `join` çağrısı yaptığında biteceği anlamına gelir.

<Listing number="21-24" file-name="src/lib.rs" caption="recv bir hata döndürdüğünde döngüden açıkça çıkma">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

Bu kodu çalıştırırken görmek için, `main` fonksiyonumuzu yalnızca iki isteği kabul edecek ve ardından sunucuyu zarifçe kapatacak şekilde değiştirelim; bu, 21-25 Listesi'nde gösterilmiştir.

<Listing number="21-25" file-name="src/main.rs" caption="İki isteği işledikten sonra sunucuyu kapatma">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

Gerçek bir web sunucusunun yalnızca iki isteği işledikten sonra kapanmasını istemezsiniz. Bu kod, zarif kapanış ve temizlik işleminin çalıştığını göstermek içindir.

`take` yöntemi, `Iterator` trait'inde tanımlanmıştır ve yine de en fazla iki öğe ile sınırlıdır. `ThreadPool`, `main`'in sonunda kapsam dışı kalacak ve `drop` uygulaması çalışacaktır.

Sunucuyu `cargo run` ile başlatın ve üç istek gönderin. Üçüncü isteğin hata vermesi gerekir ve terminalinizde benzer bir çıktı görmelisiniz:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

İş parçacıklarının kimlikleri ve mesajlarının sıralaması farklı olabilir. Bu kodun, mesajlardan nasıl çalıştığını görebiliriz: `Worker` örnekleri 0 ve 3 ilk iki isteği aldı. Sunucu, ikinci bağlantıdan sonra yeni bağlantılar kabul etmeyi bıraktı ve `ThreadPool` üzerindeki `Drop` uygulaması, `Worker` 3 bile işine başlamadan önce çalışmaya başladı. `sender`'ın bırakılması, tüm `Worker` örneklerini devre dışı bırakır ve kapanmalarını söyler. `Worker` örnekleri her biri, devre dışı kaldıklarında bir mesaj yazdırır ve ardından iş parçacığı havuzu, her `Worker` iş parçacığını bitirmek için `join` çağrısı yapar.

Bu belirli yürütmenin ilginç bir yönüne dikkat edin: `ThreadPool`, `sender`'ı devre dışı bıraktı ve hiçbir `Worker` bir hata almadığı sürece, `Worker` 0'ı birleştirmeye çalıştık. `Worker` 0, `recv`'den bir hata almadığı için ana iş parçacığı, `Worker` 0'ın bitmesini beklerken engellendi. Bu arada, `Worker` 3 bir iş aldı ve ardından tüm iş parçacıkları bir hata aldı. `Worker` 0 bittiğinde, ana iş parçacığı diğer `Worker` örneklerinin bitmesini bekledi. O noktada, hepsi döngülerinden çıkmış ve durmuştu.

Tebrikler! Artık projemizi tamamladık; eşzamanlı olarak yanıt veren bir iş parçacığı havuzuna sahip temel bir web sunucumuz var. Sunucunun zarif bir şekilde kapanmasını sağlıyoruz ve bu da havuzdaki tüm iş parçacıklarını temizliyor.

Referans için tam kod burada:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

Burada daha fazla şey yapabiliriz! Bu projeyi geliştirmeye devam etmek istiyorsanız, işte bazı fikirler:

- `ThreadPool` ve genel yöntemleri hakkında daha fazla belge ekleyin.
- Kitaplığın işlevselliğini test edin.
- `unwrap` çağrılarını daha sağlam hata işleme ile değiştirin.
- `ThreadPool`'u web istekleri dışında başka bir görevi yerine getirmek için kullanın.
- [crates.io](https://crates.io/) adresinde bir iş parçacığı havuzu kütüphanesi bulun ve bunun yerine bu kütüphaneyi kullanarak benzer bir web sunucusu uygulayın. Ardından, uyguladığınız iş parçacığı havuzunun API'sini ve sağlamlığını karşılaştırın.

## Özet

Aferin! Kitabın sonuna geldiniz! Sizi Rust'ın bu turuna katıldığınız için teşekkür etmek istiyoruz. Artık kendi Rust projelerinizi uygulamaya ve diğer insanların projelerine yardımcı olmaya hazırsınız. Unutmayın ki, Rust yolculuğunuzda karşılaştığınız her türlü zorlukta size yardımcı olmaktan mutluluk duyacak diğer Rustaceans topluluğu var.
