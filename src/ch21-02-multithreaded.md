## Tek İş Parçacıklı Sunucumuzu Çok İş Parçacıklı Sunucuya Dönüştürmek

Şu anda, sunucu her isteği sırayla işler; yani, ilk istek işlenmeden ikinci bağlantı işlenmez. Sunucu daha fazla istek aldıkça, bu seri yürütme giderek daha az verimli olur. Sunucu, işlenmesi uzun süren bir istek alırsa, sonraki istekler hızlıca işlenebilecek olsalar bile, uzun isteğin bitmesini beklemek zorunda kalır. Bunu düzeltmemiz gerekecek, ancak önce sorunu uygulamada görelim.

### Mevcut Sunucu Uygulamasında Yavaş Bir İsteği Simüle Etmek

Yavaş işlenen bir isteğin mevcut sunucu uygulamamızda diğer istekleri nasıl etkileyebileceğine bakalım. 21-10 Listesi, _/sleep_ isteğini simüle edilmiş yavaş bir yanıtla işler; sunucu yanıt vermeden önce beş saniye uyuyacaktır.

<Listing number="21-10" file-name="src/main.rs" caption="5 saniye uyuyarak yavaş bir isteği simüle etmek">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

</Listing>

Artık üç durumumuz olduğu için `if` yerine `match` kullandık. String literal değerlerle desen eşleştirmek için `request_line`'ın bir dilimi üzerinde açıkça eşleşme yapmamız gerekir; `match`, eşitlik yöntemi gibi otomatik referanslama ve dereferanslama yapmaz.

İlk kol, 21-9 Listesi'ndeki `if` bloğuyla aynıdır. İkinci kol, _/sleep_ isteğiyle eşleşir. Bu istek alındığında, sunucu başarılı HTML sayfasını döndürmeden önce beş saniye uyuyacaktır. Üçüncü kol ise 21-9 Listesi'ndeki `else` bloğuyla aynıdır.

Sunucumuzun ne kadar ilkel olduğunu görebilirsiniz: Gerçek kütüphaneler, birden fazla isteği tanımayı çok daha az ayrıntılı şekilde ele alır!

Sunucuyu `cargo run` ile başlatın. Ardından iki tarayıcı penceresi açın: biri için _http://127.0.0.1:7878/_, diğeri için _http://127.0.0.1:7878/sleep_. Daha önce olduğu gibi _/_ URI'sini birkaç kez girerseniz, hızlıca yanıt verdiğini görürsünüz. Ancak _/sleep_ girdikten sonra _/_'yi yüklerseniz, _/_ sayfasının yüklenmeden önce `sleep`'in tam beş saniye uyumasını beklediğini göreceksiniz.

Yavaş bir isteğin arkasında isteklerin birikmesini önlemek için kullanabileceğimiz birçok teknik var; bunlar arasında 17. Bölümde yaptığımız gibi async kullanmak da var. Bizim uygulayacağımız ise bir iş parçacığı havuzu (thread pool).

### İş Parçacığı Havuzuyla Verimi Artırmak

_Bir iş parçacığı havuzu_, görevleri bekleyen ve işlemeye hazır bir grup başlatılmış iş parçacığıdır. Program yeni bir görev aldığında, havuzdaki iş parçacıklarından birine bu görevi atar ve o iş parçacığı görevi işler. Havuzdaki kalan iş parçacıkları, ilk iş parçacığı görevi işlerken gelen diğer görevleri işlemek için hazırdır. İlk iş parçacığı görevini bitirdiğinde, havuzdaki boşta olan iş parçacıklarına geri döner ve yeni bir görevi işlemeye hazır olur. Bir iş parçacığı havuzu, bağlantıları eşzamanlı olarak işlemenizi sağlar ve sunucunuzun verimini artırır.

Havuzdaki iş parçacığı sayısını küçük bir sayıyla sınırlandıracağız; böylece DoS saldırılarına karşı korunmuş oluruz. Eğer programımız her istek için yeni bir iş parçacığı oluştursaydı, sunucumuza 10 milyon istek yapan biri tüm kaynaklarımızı tüketip isteklerin işlenmesini durma noktasına getirebilirdi.

Bu nedenle, sınırsız iş parçacığı başlatmak yerine, havuzda sabit sayıda iş parçacığı bekleyecek. Gelen istekler işlenmek üzere havuza gönderilecek. Havuz, gelen isteklerin bir kuyruğunu tutacak. Havuzdaki her iş parçacığı, bu kuyruktan bir istek alacak, isteği işleyecek ve ardından kuyruktan bir başka istek isteyecek. Bu tasarımla, aynı anda *N* isteği işleyebiliriz; burada *N*, iş parçacığı sayısıdır. Her iş parçacığı uzun süren bir isteğe yanıt veriyorsa, sonraki istekler yine kuyrukta birikebilir, ancak bu noktaya ulaşmadan önce işleyebileceğimiz uzun süren istek sayısını artırmış oluruz.

Bu teknik, bir web sunucusunun verimini artırmak için kullanabileceğiniz birçok yoldan sadece biridir. Araştırabileceğiniz diğer seçenekler arasında fork/join modeli, tek iş parçacıklı async I/O modeli ve çok iş parçacıklı async I/O modeli vardır. Bu konu ilginizi çekiyorsa, diğer çözümler hakkında daha fazla okuyabilir ve uygulamaya çalışabilirsiniz; Rust gibi düşük seviyeli bir dilde bunların hepsi mümkündür.

Bu seçeneklere geçmeden önce, bir iş parçacığı havuzu kullanmanın nasıl görüneceğinden bahsedelim. Kod tasarlamaya çalışırken, önce istemci arayüzünü yazmak tasarımınıza rehberlik edebilir. Kodun API'sini, çağırmak istediğiniz şekilde yapılandırın; ardından işlevselliği bu yapının içinde uygulayın, işlevselliği uygulayıp sonra genel API'yi tasarlamak yerine.

12. Bölümdeki projede test güdümlü geliştirme kullandığımız gibi, burada da derleyici güdümlü geliştirme kullanacağız. İstediğimiz fonksiyonları çağıran kodu yazacağız ve ardından kodun çalışması için neyi değiştirmemiz gerektiğini belirlemek için derleyiciden gelen hatalara bakacağız. Ancak bunu yapmadan önce, başlangıç noktası olarak kullanmayacağımız tekniği inceleyeceğiz.

<!-- Eski başlıklar. Silmeyin, bağlantılar bozulabilir. -->

<a id="code-structure-if-we-could-spawn-a-thread-for-each-request"></a>

#### Her İstek İçin Bir İş Parçacığı Oluşturmak

Öncelikle, her bağlantı için yeni bir iş parçacığı oluştursaydık kodumuzun nasıl görüneceğini inceleyelim. Daha önce de belirttiğimiz gibi, sınırsız sayıda iş parçacığı başlatma sorunları nedeniyle bu nihai planımız değil, ancak önce çalışan çok iş parçacıklı bir sunucu elde etmek için bir başlangıç noktasıdır. Ardından, iş parçacığı havuzunu bir iyileştirme olarak ekleyeceğiz ve iki çözümü karşılaştırmak daha kolay olacak. 21-11 Listesi, `main` fonksiyonunda her akış için yeni bir iş parçacığı başlatmak için yapılacak değişiklikleri gösteriyor.

<Listing number="21-11" file-name="src/main.rs" caption="Her akış için yeni bir iş parçacığı başlatmak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

</Listing>

16. Bölümde öğrendiğiniz gibi, `thread::spawn` yeni bir iş parçacığı oluşturur ve ardından closure içindeki kodu yeni iş parçacığında çalıştırır. Bu kodu çalıştırıp tarayıcınızda _/sleep_ ve ardından iki sekmede _/_ yüklerseniz, _/_ isteklerinin _/sleep_ bitmesini beklemeden yanıtlandığını görürsünüz. Ancak, daha önce de belirttiğimiz gibi, bu sonunda sistemi aşırı yükler çünkü her istek için yeni iş parçacıkları başlatırsınız.

Ayrıca 17. Bölümden hatırlayacağınız üzere, bu tam da async ve await'in parladığı bir durumdur! İş parçacığı havuzunu oluştururken bunu aklınızda bulundurun ve async ile nelerin farklı veya aynı olacağını düşünün.

<!-- Eski başlıklar. Silmeyin, bağlantılar bozulabilir. -->

<a id="creating-a-similar-interface-for-a-finite-number-of-threads"></a>

#### Sınırlı Sayıda İş Parçacığı İçin Benzer Bir Arayüz Oluşturmak

İş parçacığı havuzumuzun, iş parçacıklarından havuza geçişin API'yi kullanan kodda büyük değişiklik gerektirmeyeceği şekilde benzer ve tanıdık bir şekilde çalışmasını istiyoruz. 21-12 Listesi, `thread::spawn` yerine kullanmak istediğimiz `ThreadPool` yapısı için varsayımsal arayüzü gösteriyor.

<Listing number="21-12" file-name="src/main.rs" caption="İdeal `ThreadPool` arayüzümüz">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

</Listing>

`ThreadPool::new` ile yapılandırılabilir sayıda iş parçacığı olan yeni bir iş parçacığı havuzu oluşturuyoruz, bu örnekte dört. Ardından, `for` döngüsünde, `pool.execute` tıpkı `thread::spawn` gibi bir closure alıyor ve havuzun her akış için çalıştırmasını sağlıyor. `pool.execute`'u, closure'ı alıp havuzdaki bir iş parçacığına çalıştırması için verecek şekilde uygulamamız gerekiyor. Bu kod henüz derlenmeyecek, ancak derleyicinin bize nasıl düzeltileceği konusunda yol göstermesini sağlayacağız.

<!-- Eski başlıklar. Silmeyin, bağlantılar bozulabilir. -->

<a id="building-the-threadpool-struct-using-compiler-driven-development"></a>

#### Derleyici Güdümlü Geliştirme ile `ThreadPool` Yapısını Oluşturmak

21-12 Listesi'ndeki değişiklikleri _src/main.rs_ dosyanıza yapın ve ardından geliştirmemizi derleyici hatalarıyla yönlendirmek için `cargo check` çıktısını kullanın. İlk aldığımız hata şudur:

```console
{{#include ../listings/ch21-web-server/listing-21-12/output.txt}}
```

Harika! Bu hata, bir `ThreadPool` türüne veya modülüne ihtiyacımız olduğunu söylüyor, şimdi bunu oluşturacağız. `ThreadPool` uygulamamız, web sunucumuzun yaptığı işten bağımsız olacak. Bu yüzden, `hello` paketini, `ThreadPool` uygulamamızı tutacak şekilde ikili paket yerine bir kütüphane paketine çevireceğiz. Kütüphane paketine geçtikten sonra, ayrı iş parçacığı havuzu kütüphanesini yalnızca web istekleri için değil, iş parçacığı havuzu kullanmak istediğimiz her iş için kullanabiliriz.

Şimdilik en basit `ThreadPool` tanımını içeren _src/lib.rs_ dosyasını oluşturun:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

</Listing>

Ardından, _main.rs_ dosyanızı, kütüphane paketinden `ThreadPool`'u kapsamınıza alacak şekilde düzenleyin:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

</Listing>

Bu kod hâlâ çalışmayacak, ancak bir sonraki düzeltmemiz gereken hatayı görmek için tekrar kontrol edelim:

```console
{{#include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

Bu hata, sıradaki adımda `ThreadPool` için `new` adlı ilişkili bir fonksiyon oluşturmamız gerektiğini gösteriyor. Ayrıca, `new`'nun bir parametre alması ve `ThreadPool` örneği döndürmesi gerektiğini biliyoruz. Bu özelliklere sahip en basit `new` fonksiyonunu uygulayalım:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

</Listing>

`size` parametresi için `usize` türünü seçtik çünkü negatif iş parçacığı sayısının bir anlamı yok. Ayrıca, bu `4`'ü iş parçacıklarından oluşan bir koleksiyonun eleman sayısı olarak kullanacağımızı biliyoruz; bu da 3. Bölümde ["Tamsayı Türleri"][integer-types]<!-- ignore --> kısmında tartışıldığı gibi `usize` türünün amacıdır.

Kodu tekrar kontrol edelim:

```console
{{#include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

Şimdi hata, `ThreadPool` üzerinde bir `execute` metodu olmadığı için oluşuyor. ["Sınırlı Sayıda İş Parçacığı İçin Benzer Bir Arayüz Oluşturmak"](#sınırlı-sayıda-i̇ş-parçacığı-i̇çin-benzer-bir-arayüz-oluşturmak)<!-- ignore --> kısmında, iş parçacığı havuzumuzun `thread::spawn`'a benzer bir arayüze sahip olması gerektiğine karar vermiştik. Ayrıca, `execute` fonksiyonunu, aldığı closure'ı havuzdaki boşta olan bir iş parçacığına çalıştırması için verecek şekilde uygulayacağız.

`ThreadPool` üzerinde, parametre olarak bir closure alacak şekilde `execute` metodunu tanımlayacağız. 13. Bölümde ["Kapatıcıdan Taşınan Değerler ve `Fn` Trait'leri"] [fn-traits]<!-- ignore --> kısmında closure'ları üç farklı trait ile parametre olarak alabileceğimizi öğrenmiştik: `Fn`, `FnMut` ve `FnOnce`. Burada hangi tür closure kullanacağımıza karar vermemiz gerekiyor. Sonunda, `execute`'da aldığımız argümanı `spawn`'a aktaracağımız için, standart kütüphanedeki `thread::spawn` uygulamasının parametresinde hangi trait sınırlarının olduğunu inceleyebiliriz. Belgeler bize şunu gösteriyor:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

Burada ilgilendiğimiz tür parametresi `F`'dir; `T` tür parametresi dönüş değeriyle ilgilidir ve bizi ilgilendirmiyor. `spawn`'un, `F` üzerinde trait sınırı olarak `FnOnce` kullandığını görebiliyoruz. Muhtemelen bizim de istediğimiz budur, çünkü sonunda `execute`'da aldığımız closure'ı `spawn`'a aktaracağız. Ayrıca, bir isteği çalıştıracak iş parçacığı yalnızca o isteğin closure'ını bir kez çalıştıracağı için, `FnOnce`'ın uygun trait olduğunu daha da net görebiliriz.

`F` tür parametresi ayrıca `Send` trait sınırına ve `'static` ömür sınırına sahip; bunlar da bizim durumumuzda faydalı: Closure'ı bir iş parçacığından diğerine aktarmak için `Send`'e, iş parçacığının ne kadar süreceğini bilmediğimiz için de `'static`'e ihtiyacımız var. Şimdi, bu sınırlarla birlikte, `ThreadPool` üzerinde generic bir `F` parametresi alan bir `execute` metodu oluşturalım:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

</Listing>

Hâlâ `FnOnce`'tan sonra `()` kullanıyoruz çünkü bu `FnOnce`, parametre almayan ve birim türü `()` döndüren bir closure'ı temsil ediyor. Fonksiyon tanımlarında olduğu gibi, dönüş türü imzadan çıkarılabilir, ancak parametremiz olmasa bile parantezleri yazmamız gerekir.

Yine, bu `execute` metodunun en basit uygulamasıdır: hiçbir şey yapmaz, ancak amacımız kodun derlenmesini sağlamak. Tekrar kontrol edelim:

```console
{{#include ../listings/ch21-web-server/no-listing-03-define-execute/output.txt}}
```

Derleniyor! Ancak, `cargo run` yapıp tarayıcıda bir istek yaparsanız, bölümün başında gördüğümüz hataları tarayıcıda göreceksiniz. Kütüphanemiz, `execute`'a geçirilen closure'ı henüz çağırmıyor!

> Not: Haskell ve Rust gibi katı derleyicilere sahip diller için duyabileceğiniz bir söz vardır: "Kod derleniyorsa, çalışır." Ancak bu söz evrensel olarak doğru değildir. Projemiz derleniyor, ancak hiçbir şey yapmıyor! Gerçek, tamamlanmış bir proje inşa ediyorsak, kodun derlenip _istediğimiz davranışı_ gösterdiğinden emin olmak için birim testleri yazmaya başlamak iyi bir fikir olurdu.

Düşünün: Burada bir closure yerine bir _future_ çalıştıracak olsaydık ne farklı olurdu?
