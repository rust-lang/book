## Tek İş Parçacıklı Bir Web Sunucusu Oluşturmak

Öncelikle, tek iş parçacıklı bir web sunucusunu çalıştırarak başlayacağız. Başlamadan önce, web sunucuları oluştururken yer alan protokollere hızlıca bir göz atalım. Bu protokollerin ayrıntıları bu kitabın kapsamı dışında, ancak kısa bir özet ihtiyacınız olan bilgiyi verecektir.

Web sunucularında yer alan iki ana protokol _Hiper Metin Aktarım Protokolü_ (_HTTP_) ve _Aktarım Kontrol Protokolü_ (_TCP_)'dir. Her iki protokol de _istek-yanıt_ protokolüdür; yani bir _istemci_ istek başlatır ve bir _sunucu_ bu istekleri dinler ve istemciye bir yanıt sağlar. Bu istek ve yanıtların içeriği protokoller tarafından tanımlanır.

TCP, bilgilerin bir sunucudan diğerine nasıl iletileceğinin ayrıntılarını tanımlayan alt seviye bir protokoldür, ancak bu bilgilerin ne olduğunu belirtmez. HTTP ise, istek ve yanıtların içeriğini tanımlayarak TCP'nin üzerine inşa edilmiştir. Teknik olarak HTTP'nin başka protokollerle de kullanılması mümkündür, ancak çoğu durumda HTTP verilerini TCP üzerinden gönderir. Biz de TCP ve HTTP istek ve yanıtlarının ham baytlarıyla çalışacağız.

### TCP Bağlantısını Dinlemek

Web sunucumuzun bir TCP bağlantısını dinlemesi gerekir, bu yüzden önce bu kısmı ele alacağız. Standart kütüphane, bunu yapmamıza olanak tanıyan bir `std::net` modülü sunar. Her zamanki gibi yeni bir proje oluşturalım:

```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```

Şimdi, _src/main.rs_ dosyasına 21-1 Listesi'ndeki kodu girin. Bu kod, `127.0.0.1:7878` yerel adresinde gelen TCP akışlarını dinleyecek. Bir akış geldiğinde, `Bağlantı kuruldu!` mesajını yazdıracak.

<Listing number="21-1" file-name="src/main.rs" caption="Gelen akışları dinlemek ve bir akış aldığımızda mesaj yazdırmak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-01/src/main.rs}}
```

</Listing>

`TcpListener` kullanarak, `127.0.0.1:7878` adresinde TCP bağlantılarını dinleyebiliriz. Adresteki iki nokta öncesi kısım bilgisayarınızı (her bilgisayarda aynıdır ve yazarların bilgisayarına özel değildir), `7878` ise portu temsil eder. Bu portu iki nedenle seçtik: HTTP genellikle bu portta kabul edilmez, bu nedenle sunucumuzun makinenizde çalışan başka bir web sunucusuyla çakışma olasılığı düşüktür ve 7878, telefonda _rust_ olarak yazılır.

Bu senaryoda `bind` fonksiyonu, `new` fonksiyonu gibi çalışır ve yeni bir `TcpListener` örneği döndürür. Fonksiyonun adı "bind"'dir çünkü ağ programlamasında bir portu dinlemek için bağlanmaya "porta bağlanmak" denir.

`bind` fonksiyonu bir `Result<T, E>` döndürür, yani bağlanmanın başarısız olabileceğini gösterir. Örneğin, 80 numaralı porta bağlanmak için yönetici ayrıcalıkları gerekir (yönetici olmayanlar yalnızca 1023'ten büyük portlarda dinleme yapabilir), bu nedenle yönetici olmadan 80 numaralı porta bağlanmaya çalışırsak bağlanma gerçekleşmez. Ayrıca, programımızın iki örneğini çalıştırırsak ve iki program aynı portu dinlerse de bağlanma gerçekleşmez. Temel bir sunucu yazdığımız için bu tür hatalarla ilgilenmeyeceğiz; bunun yerine, hata olursa programı durdurmak için `unwrap` kullanıyoruz.

`TcpListener` üzerindeki `incoming` metodu, bize bir dizi akış (daha spesifik olarak, `TcpStream` türünde akışlar) veren bir yineleyici döndürür. Tek bir _akış_, istemci ile sunucu arasında açık bir bağlantıyı temsil eder. Bir _bağlantı_, istemcinin sunucuya bağlandığı, sunucunun bir yanıt ürettiği ve bağlantının kapatıldığı tam istek-yanıt sürecinin adıdır. Bu nedenle, istemcinin ne gönderdiğini görmek için `TcpStream`'den okuyacak ve yanıtımızı istemciye göndermek için akışa yazacağız. Genel olarak, bu `for` döngüsü her bağlantıyı sırayla işleyecek ve bizim ele almamız için bir dizi akış üretecek.

Şimdilik, akışı ele alma işlemimiz, akışta herhangi bir hata olursa programı sonlandırmak için `unwrap` çağırmaktan ibaret; hata yoksa program bir mesaj yazdırır. Başarılı durum için daha fazla işlevsellik ekleyeceğiz.

TCP bağlantısını dinlemeyi başardık! Şimdi, bir tarayıcıdan gelen isteği okuyalım ve bu isteğe yanıt verelim.

### İsteği Okuma

Tarayıcıdan isteği okuma işlevselliğini uygulayalım! Bağlantıyı alıp bu bağlantıyla bazı eylemler gerçekleştirme kaygısını ayırmak için, istekleri dinleyip yanıt verecek yeni bir `handle_connection` işlevi başlatacağız. Bu yeni işlevde, TCP akışından veri okuyacağız ve bu verileri ekrana yazdıracağız. Kodumuzu 21-2 Listesi'ndeki gibi değiştirelim.

<Listing number="21-2" file-name="src/main.rs" caption="TcpStream'den okumak ve verileri yazdırmak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-02/src/main.rs}}
```

</Listing>

`std::io::prelude` ve `std::io::BufReader`'ı kapsamımıza alarak, akıştan okuma ve akışa yazma işlemlerini gerçekleştirmemizi sağlayan özellikler ve türler elde ediyoruz. `main` işlevindeki `for` döngüsünde, artık bağlantı kuruldu mesajı yazdırmak yerine, yeni oluşturduğumuz `handle_connection` işlevini çağırıyor ve `stream`'i ona iletiyoruz.

`handle_connection` işlevinde, `stream`'in referansını saran yeni bir `BufReader` örneği oluşturuyoruz. `BufReader`, bizim için `std::io::Read` özelliği yöntemlerine yapılan çağrıları yöneterek tamponlama ekler.

Tarayıcıdan sunucumuza gelen isteğin satırlarını toplamak için `http_request` adında bir değişken oluşturuyoruz. Bu satırları bir vektörde toplamak istediğimizi belirtmek için `Vec<_>` türü ekliyoruz.

`BufReader`, `std::io::BufRead` özelliğini uygular; bu özellik `lines` yöntemini sağlar. `lines` yöntemi, verileri her gördüğünde bir yeni satır baytı ile ayırarak bir `Result<String, std::io::Error>` yineleyicisi döndürür. Her `String`'i elde etmek için her `Result`'ı eşleştirip `unwrap`lıyoruz. `Result`, veriler geçerli UTF-8 değilse veya akıştan okuma sırasında bir sorun oluşursa hata verebilir. Yine, üretim amaçlı bir program bu hataları daha zarif bir şekilde ele almalıdır, ancak biz basitlik açısından hata durumunda programı durdurmayı seçiyoruz.

Tarayıcı, bir HTTP isteğinin sonunu iki ardışık yeni satır karakteri göndererek belirtir, bu nedenle akıştan bir isteği almak için, boş bir satır alana kadar satırları toplarız. Satırları vektöre topladıktan sonra, sunucumuza gelen verileri görebilmek için bunları güzel bir şekilde biçimlendirilmiş olarak yazdırıyoruz.

Bu kodu deneyelim! Programı başlatın ve tekrar bir web tarayıcısında istek yapın. Tarayıcıda hala bir hata sayfası göreceğiz, ancak terminaldeki program çıktımız artık buna benzer görünmelidir:

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

Tarayıcınıza bağlı olarak, biraz farklı bir çıktı alabilirsiniz. Artık istek verilerini yazdırdığımıza göre, bir tarayıcı isteğinden gelen birden fazla bağlantıyı görmenin nedenini, isteğin ilk satırındaki `GET` sonrası yola bakarak görebiliriz. Tekrar eden bağlantıların hepsi _/_'yi talep ediyorsa, tarayıcının sunucumuzdan yanıt alamadığı için _/_'yi tekrar tekrar almaya çalıştığını biliyoruzdur.

Bu istek verilerine daha yakından bakalım, böylece tarayıcının programımızdan ne talep ettiğini anlayalım.

### HTTP İsteğine Yakından Bakış

HTTP, metin tabanlı bir protokoldür ve bir istek şu formatı alır:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

İlk satır, istemcinin ne talep ettiğine dair bilgi içeren _istek satırı_'dır. İstek satırının ilk kısmı, istemcinin bu isteği nasıl yaptığını tanımlayan `GET` veya `POST` gibi bir _metot_'u gösterir. İstemcimiz bir `GET` isteği kullandı, bu da bilgilere erişim talep ettiği anlamına geliyor.

İstek satırının bir sonraki kısmı _/_, istemcinin talep ettiği _üniform kaynak tanımlayıcısı_ _(URI)_'dır: bir URI, neredeyse ama tam olarak, bir _üniform kaynak konumlayıcı_ _(URL)_'dır. URI'ler ve URL'ler arasındaki farklar, bu bölümdeki amacımız için önemli değildir, ancak HTTP spesifikasyonu terim olarak URI kullanır, bu yüzden burada sadece zihnimizde _URL_ ile _URI_ arasında bir değiş tokuş yapabiliriz.

Son olarak, istemcinin kullandığı HTTP sürümünü gösterir ve ardından istek satırı bir CRLF dizisi ile biter. (CRLF, _carriage return_ ve _line feed_ anlamına gelir, bu terimler daktilo günlerinden kalmadır!) CRLF dizisi ayrıca `\r\n` olarak da yazılabilir; burada `\r` bir taşıyıcı dönüş ve `\n` bir satır beslemesidir. _CRLF dizisi_, istek satırını geri kalan istek verilerinden ayırır. CRLF yazdırıldığında, yeni bir satırın başlamasını görürüz, `\r\n`'yi değil.

Şu ana kadar programımızı çalıştırarak elde ettiğimiz istek satırı verilerine bakarak, `GET` metodunun, _/_ istek URI'sinin ve `HTTP/1.1` sürümünün kullanıldığını görüyoruz. İstek satırından sonra, `Host:` ile başlayan kalan satırlar başlıklardır. `GET` isteklerinin gövdesi yoktur.

Farklı bir tarayıcıdan istek yapmayı veya _127.0.0.1:7878/test_ gibi farklı bir adresi istemeyi deneyin, istek verisinin nasıl değiştiğini gözlemleyin.

Artık tarayıcının bizden ne istediğini bildiğimize göre, biraz veri gönderelim!

### Bir Yanıt Yazmak

İstemci isteğine yanıt olarak veri göndermeyi uygulayacağız. Yanıtlar şu biçimdedir:

```text
HTTP-Sürümü Durum-Kodu Açıklama-İfadesi CRLF
başlıklar CRLF
mesaj-gövdesi
```

İlk satır, yanıtta kullanılan HTTP sürümünü, isteğin sonucunu özetleyen sayısal bir durum kodunu ve durum kodunun metinsel açıklamasını içeren _durum satırı_dır. CRLF dizisinden sonra başlıklar, bir başka CRLF dizisi ve yanıtın gövdesi gelir.

Aşağıda, HTTP 1.1 sürümünü kullanan, 200 durum koduna ve OK açıklama ifadesine sahip, başlıksız ve gövdesiz bir örnek yanıt verilmiştir:

```text
HTTP/1.1 200 OK\r\n\r\n
```

200 durum kodu, standart başarı yanıtıdır. Bu metin, küçük bir başarılı HTTP yanıtıdır. Bunu, başarılı bir isteğe yanıt olarak akışa yazalım! `handle_connection` fonksiyonunda, istek verisini yazdıran `println!`'ı kaldırın ve yerine 21-3 Listesi'ndeki kodu ekleyin.

<Listing number="21-3" file-name="src/main.rs" caption="Akışa küçük bir başarılı HTTP yanıtı yazmak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-03/src/main.rs:here}}
```

</Listing>

İlk yeni satır, başarı mesajının verisini tutan `response` değişkenini tanımlar. Ardından, `response` üzerindeki `as_bytes` fonksiyonunu çağırarak dize verisini baytlara dönüştürürüz. `stream` üzerindeki `write_all` metodu bir `&[u8]` alır ve bu baytları doğrudan bağlantı üzerinden gönderir. `write_all` işlemi başarısız olabileceğinden, hata durumunda yine `unwrap` kullanıyoruz. Gerçek bir uygulamada burada hata yönetimi eklenmelidir.

Bu değişikliklerle kodumuzu çalıştırıp bir istek yapalım. Artık terminalde herhangi bir veri yazdırmıyoruz, bu yüzden Cargo'dan başka bir çıktı görmeyeceğiz. _127.0.0.1:7878_ adresini bir web tarayıcısında yüklediğinizde, bir hata yerine boş bir sayfa görmelisiniz. Artık bir HTTP isteği alıp yanıt göndermeyi elle kodladınız!

### Gerçek HTML Döndürmek

Boş bir sayfa yerine daha fazlasını döndürmek için işlevsellik ekleyelim. Proje dizininizin kökünde, _src_ klasörünün dışında yeni bir _hello.html_ dosyası oluşturun. İstediğiniz herhangi bir HTML'yi girebilirsiniz; 21-4 Listesi'nde bir örnek gösterilmiştir.

<Listing number="21-4" file-name="hello.html" caption="Yanıtta döndürülecek örnek bir HTML dosyası">

```html
{{#include ../listings/ch21-web-server/listing-21-05/hello.html}}
```

</Listing>

Bu, bir başlık ve biraz metin içeren minimal bir HTML5 belgesidir. Sunucuya bir istek geldiğinde bunu döndürmek için, 21-5 Listesi'nde gösterildiği gibi `handle_connection` fonksiyonunu değiştirerek HTML dosyasını okuyup yanıtın gövdesine ekleyecek ve göndereceğiz.

<Listing number="21-5" file-name="src/main.rs" caption="Yanıt gövdesi olarak *hello.html* içeriğini göndermek">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-05/src/main.rs:here}}
```

</Listing>

`use` satırına standart kütüphanenin dosya sistemi modülünü (`fs`) ekledik. Bir dosyanın içeriğini bir dizeye okuma kodu tanıdık gelmeli; 12-4 Listesi'nde dosya içeriğini okurken de kullanmıştık.

Sonra, dosya içeriğini başarı yanıtının gövdesi olarak eklemek için `format!` kullanıyoruz. Geçerli bir HTTP yanıtı sağlamak için, yanıt gövdemizin boyutına (bu durumda `hello.html`'in boyutuna) ayarlanan `Content-Length` başlığını ekliyoruz.

Bu kodu `cargo run` ile çalıştırın ve tarayıcınızda _127.0.0.1:7878_ adresini yükleyin; HTML'nizin görüntülendiğini görmelisiniz!

Şu anda, `http_request`'teki istek verisini yok sayıyor ve HTML dosyasının içeriğini koşulsuz olarak döndürüyoruz. Yani, tarayıcınızda _127.0.0.1:7878/something-else_ gibi başka bir istek yapsanız bile aynı HTML yanıtını alırsınız. Şu anda sunucumuz çok sınırlı ve çoğu web sunucusunun yaptığı gibi davranmıyor. Yanıtlarımızı isteğe göre özelleştirmek ve yalnızca _/_'ye düzgün bir istek geldiğinde HTML dosyasını döndürmek istiyoruz.

### İsteği Doğrulama ve Seçici Yanıt Verme

Şu anda, web sunucumuz istemci ne isterse istesin dosyadaki HTML'yi döndürüyor. Tarayıcının _/_ isteyip istemediğini kontrol edecek ve başka bir şey isterse hata döndürecek işlevsellik ekleyelim. Bunun için, 21-6 Listesi'nde gösterildiği gibi `handle_connection` fonksiyonunu değiştirmemiz gerekiyor. Bu yeni kod, alınan isteğin içeriğini _/_ isteğiyle bildiğimiz istekle karşılaştırır ve farklı istekleri farklı şekilde ele almak için `if` ve `else` blokları ekler.

<Listing number="21-6" file-name="src/main.rs" caption="*/ * isteklerini diğer isteklerden farklı şekilde ele almak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-06/src/main.rs:here}}
```

</Listing>

Yalnızca HTTP isteğinin ilk satırına bakacağımız için, isteğin tamamını bir vektöre okumak yerine, yineleyiciden ilk öğeyi almak için `next` çağırıyoruz. İlk `unwrap`, `Option`'ı ele alır ve yineleyicide hiç öğe yoksa programı durdurur. İkinci `unwrap`, `Result`'ı ele alır ve 21-2 Listesi'nde `map` ile eklenen `unwrap` ile aynı etkiye sahiptir.

Sonra, `request_line`'ı, _/_ yoluna yapılan bir GET isteğinin istek satırıyla eşit olup olmadığını kontrol ediyoruz. Eğer eşitse, `if` bloğu HTML dosyamızın içeriğini döndürür.

Eğer `request_line`, _/_ yoluna yapılan GET isteğiyle eşleşmiyorsa, başka bir istek aldık demektir. Birazdan `else` bloğuna, diğer tüm isteklere yanıt olarak dönecek kodu ekleyeceğiz.

Şimdi bu kodu çalıştırın ve _127.0.0.1:7878_ isteği yapın; _hello.html_ içeriğini almalısınız. Başka bir istek yaparsanız, örneğin _127.0.0.1:7878/something-else_, 21-1 ve 21-2 Listeleri'ndeki kodları çalıştırırken gördüğünüz gibi bir bağlantı hatası alırsınız.

Şimdi, 21-7 Listesi'ndeki kodu `else` bloğuna ekleyerek, istenen içerik bulunamadığında 404 durum kodu ve bir hata sayfası döndürelim. Ayrıca, yanıt olarak kullanıcıya gösterilecek bir HTML de döndüreceğiz.

<Listing number="21-7" file-name="src/main.rs" caption="*/ * dışında bir şey istenirse 404 durum kodu ve hata sayfası ile yanıt verme">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-07/src/main.rs:here}}
```

</Listing>

Burada, yanıtımızda 404 durum kodu ve `NOT FOUND` açıklama ifadesi olan bir durum satırı var. Yanıtın gövdesi olarak _404.html_ dosyasındaki HTML dönecek. Hata sayfası için _hello.html_ dosyasının yanında bir _404.html_ dosyası oluşturmanız gerekecek; yine istediğiniz herhangi bir HTML'yi kullanabilir veya 21-8 Listesi'ndeki örnek HTML'yi kullanabilirsiniz.

<Listing number="21-8" file-name="404.html" caption="404 yanıtı ile gönderilecek sayfa için örnek içerik">

```html
{{#include ../listings/ch21-web-server/listing-21-07/404.html}}
```

</Listing>

Bu değişikliklerle sunucunuzu tekrar çalıştırın. _127.0.0.1:7878_ isteği _hello.html_ içeriğini döndürmeli, _127.0.0.1:7878/foo_ gibi başka bir istek ise _404.html_'den hata HTML'sini döndürmelidir.

### Biraz Refaktörizasyon

Şu anda, `if` ve `else` bloklarında çok fazla tekrar var: ikisi de dosya okuyor ve dosya içeriğini akışa yazıyor. Tek fark, durum satırı ve dosya adı. Kodu daha derli toplu yapmak için, bu farkları ayrı `if` ve `else` satırlarında değişkenlere atayalım; ardından bu değişkenleri dosya okuma ve yanıt yazma kodunda koşulsuz kullanalım. 21-9 Listesi'nde, iki durum arasındaki farkı gösteren kodu görebilirsiniz.

<Listing number="21-9" file-name="src/main.rs" caption="İki durum arasındaki farkı yalnızca değişken atamasıyla ayıran refaktörize kod">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-09/src/main.rs:here}}
```

</Listing>

Artık `if` ve `else` blokları yalnızca durum satırı ve dosya adı için uygun değerleri bir tuple olarak döndürüyor; ardından bu iki değeri, 19. Bölümde tartışıldığı gibi, `let` ifadesinde desen kullanarak `status_line` ve `filename` değişkenlerine atıyoruz.

Daha önce tekrarlanan kod artık `if` ve `else` bloklarının dışında ve `status_line` ile `filename` değişkenlerini kullanıyor. Bu, iki durum arasındaki farkı görmeyi kolaylaştırıyor ve dosya okuma ve yanıt yazma işlemini değiştirmek istersek kodu yalnızca bir yerde güncellememizi sağlıyor. 21-9 Listesi'ndeki kodun davranışı, 21-7 Listesi'ndeki kodla aynı olacaktır.

Harika! Artık yaklaşık 40 satırlık Rust koduyla, bir isteğe içerik sayfası dönen ve diğer tüm isteklere 404 yanıtı veren basit bir web sunucumuz var.

Şu anda sunucumuz tek bir iş parçacığında çalışıyor, yani aynı anda yalnızca bir isteğe hizmet edebiliyor. Şimdi, bazı yavaş istekleri simüle ederek bunun nasıl bir sorun olabileceğine bakalım. Ardından, sunucumuzu aynı anda birden fazla isteği işleyebilecek şekilde düzelteceğiz.
