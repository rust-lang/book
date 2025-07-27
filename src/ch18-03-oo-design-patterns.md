## Nesne Yönelimli Bir Tasarım Deseni Uygulamak

_State pattern_ (durum deseni), nesne yönelimli bir tasarım desenidir. Bu desenin özü, bir değerin dahili olarak sahip olabileceği bir dizi durumu tanımlamaktır. Durumlar, bir dizi _durum nesnesi_ ile temsil edilir ve değerin davranışı, mevcut durumuna göre değişir. Şimdi, bir alanında durumunu tutan bir blog yazısı struct'ı örneği üzerinden ilerleyeceğiz; bu alan, "taslak", "inceleme" veya "yayınlandı" durum nesnelerinden biri olacak.

Durum nesneleri bazı işlevleri paylaşır: Rust'ta elbette nesne ve kalıtım yerine struct ve trait kullanırız. Her durum nesnesi kendi davranışından ve başka bir duruma ne zaman geçeceğinden sorumludur. Durum nesnesini tutan değer ise, durumların farklı davranışları veya ne zaman geçiş yapılacağı hakkında hiçbir şey bilmez.

Durum desenini kullanmanın avantajı, programın iş gereksinimleri değiştiğinde, durumu tutan değerin kodunu veya onu kullanan kodu değiştirmemize gerek olmamasıdır. Sadece bir durum nesnesinin içindeki kodu güncelleyerek kuralları değiştirebilir veya yeni durum nesneleri ekleyebiliriz.

Önce, durum desenini daha geleneksel nesne yönelimli bir şekilde uygulayacağız; ardından Rust'a daha doğal gelen bir yaklaşımla tekrar ele alacağız. Şimdi, durum desenini kullanarak bir blog yazısı iş akışını adım adım uygulayalım.

Son işlevsellik şöyle olacak:

1. Bir blog yazısı boş bir taslak olarak başlar.
1. Taslak tamamlandığında, yazının incelenmesi istenir.
1. Yazı onaylandığında, yayınlanır.
1. Yalnızca yayınlanmış blog yazıları içerik döndürür; onaylanmamış yazılar yanlışlıkla yayınlanamaz.

Bir yazı üzerinde yapılan diğer değişiklikler etkisiz olmalıdır. Örneğin, bir taslak blog yazısını inceleme istemeden onaylamaya çalışırsak, yazı yayınlanmamış taslak olarak kalmalıdır.

### Geleneksel Nesne Yönelimli Bir Deneme

Aynı problemi çözmek için sonsuz sayıda kod yapısı oluşturulabilir ve her birinin farklı avantajları/dezavantajları vardır. Bu bölümdeki uygulama, Rust'ta yazılması mümkün olan ama Rust'ın bazı güçlü yönlerinden faydalanmayan, daha geleneksel nesne yönelimli bir stildir. Sonrasında, yine nesne yönelimli tasarım desenini kullanan ama nesne yönelimli programlamaya aşina olanlara daha az tanıdık gelebilecek, Rust'a daha uygun bir çözüm göstereceğiz. İki çözümü karşılaştırarak, Rust kodunu diğer dillerdeki kodlardan farklı tasarlamanın avantaj/dezavantajlarını göreceğiz.

18-11 numaralı listede, bu iş akışının kod hali gösteriliyor: Bu, `blog` adında bir kütüphane crate'inde uygulayacağımız API'nin örnek kullanımıdır. Henüz `blog` crate'ini uygulamadığımız için bu kod derlenmeyecek.

<Listing number="18-11" file-name="src/main.rs" caption="`blog` crate'imizin sahip olmasını istediğimiz davranışı gösteren kod">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

</Listing>

Kullanıcının `Post::new` ile yeni bir taslak blog yazısı oluşturmasına izin vermek istiyoruz. Blog yazısına metin eklenebilmesini istiyoruz. Eğer onaydan önce yazının içeriğini almaya çalışırsak, hiçbir metin almamalıyız; çünkü yazı hâlâ taslaktır. Kodda gösterim amaçlı olarak `assert_eq!` kullandık. Bunun için mükemmel bir birim testi, taslak bir blog yazısının `content` metodundan boş bir dize döndürdüğünü doğrulamak olurdu; ancak bu örnekte test yazmayacağız.

Sonraki adımda, yazının incelenmesini istemeyi etkinleştirmek ve inceleme beklerken `content`'in boş dize döndürmesini istiyoruz. Yazı onaylandığında ise yayınlanmalı; yani `content` çağrıldığında yazının metni döndürülmeli.

Dikkat edin, crate'ten etkileşime geçtiğimiz tek tip `Post` tipi. Bu tip, durum desenini kullanacak ve bir yazının olabileceği üç farklı durumu temsil eden bir değer tutacak: taslak, inceleme veya yayınlandı. Bir durumdan diğerine geçiş, `Post` tipi içinde dahili olarak yönetilecek. Durumlar, kütüphanemizin kullanıcılarının `Post` örneği üzerinde çağırdığı metotlara yanıt olarak değişecek, ancak kullanıcılar durum geçişlerini doğrudan yönetmek zorunda kalmayacak. Ayrıca, kullanıcılar yazının incelemeden önce yayınlanması gibi durumlarla ilgili hatalar yapamayacak.

#### `Post` Tanımlama ve Taslak Durumda Yeni Bir Örnek Oluşturma

Kütüphanenin uygulanmasına başlarken, öncelikle bazı içerikler tutan bir `Post` struct'ına ihtiyacımız olduğunu biliyoruz. Bu nedenle, bir `Post` örneği oluşturmak için bir `new` işlevi ile birlikte struct'ın tanımıyla başlayacağız. Ayrıca, tüm durum nesnelerinin sahip olması gereken davranışları tanımlayacak özel bir `State` trait'i oluşturacağız.

Ardından, `Post`, bir `Option<T>` içinde `Box<dyn State>` türünde bir trait nesnesi tutacak bir `state` adlı özel bir alanda durum nesnesini tutacak. `Option<T>` türünün neden gerekli olduğunu birazdan göreceksiniz.

<Listing number="18-12" file-name="src/lib.rs" caption="Yeni bir `Post` örneği oluşturan bir `new` işlevi, bir `State` trait'i ve bir `Draft` struct'ının tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

</Listing>

`State` trait'i, farklı gönderi durumları tarafından paylaşılan davranışları tanımlar. Durum nesneleri `Draft`, `PendingReview` ve `Published` olup, bunların tümü `State` trait'ini uygulayacaktır. Şu anda, trait'in herhangi bir yöntemi yok ve önce yalnızca `Draft` durumunu tanımlayacağız; çünkü bir gönderinin başlamak istediğimiz durumudur.

Yeni bir `Post` oluşturduğumuzda, `state` alanını bir `Some` değeri ile dolduruyoruz ve bu değer bir `Draft` struct'ına işaret ediyor. Bu, yeni bir `Post` örneği oluşturduğumuzda, onun bir taslak olarak başlamasını sağlıyor. `Post`'un `state` alanı özel olduğundan, başka bir durumda `Post` oluşturmanın bir yolu yoktur! `Post::new` işlevinde, `content` alanını yeni, boş bir `String` ile dolduruyoruz.

#### Gönderi İçeriğinin Metnini Saklama

18-11 numaralı listede, `add_text` adlı bir yöntemi çağırmak ve ona bir `&str` geçirerek bu metni blog yazısının içerik metni olarak eklemek istediğimizi gördük. Bunu, daha sonra `content` alanının verilerinin nasıl okunacağını kontrol eden bir yöntem olarak uyguluyoruz. `add_text` yöntemi oldukça basit, bu nedenle `impl Post` bloğuna 18-13 numaralı listede gösterildiği gibi uygulayalım.

<Listing number="18-13" file-name="src/lib.rs" caption="Bir gönderinin `content` alanına metin eklemek için `add_text` yönteminin uygulanması">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

</Listing>

`add_text` yöntemi, kendisi üzerinde `add_text` çağırdığımız `Post` örneğini değiştirdiğimiz için `self`'in bir referansını alır. Ardından, `content` içindeki `String` üzerinde `push_str` çağırır ve eklemek için `text` argümanını geçiririz. Bu davranış, gönderinin bulunduğu duruma bağlı değildir, bu nedenle durum deseninin bir parçası değildir. `add_text` yöntemi, `state` alanı ile etkileşime girmez, ancak desteklemek istediğimiz bir davranışın parçasıdır.

#### Taslak Bir Gönderinin İçeriğinin Boş Olmasını Sağlama

`add_text` çağrıldıktan ve gönderimize biraz içerik eklendikten sonra bile, `content` yönteminin bir boş dize dilimi döndürmesini istiyoruz; çünkü gönderi hâlâ taslak durumundadır, bu da 18-11 numaralı listenin 7. satırında gösterilmiştir. Şimdilik, bu gereksinimi karşılayacak en basit şeyle `content` yöntemini uygulayalım: her zaman boş bir dize dilimi döndürmek. Bunu daha sonra, bir gönderinin durumunu değiştirmenin yolunu uyguladığımızda değiştireceğiz, böylece yayınlanabilir. Şu anda, gönderiler yalnızca taslak durumunda olabilir, bu nedenle gönderi içeriği her zaman boş olmalıdır. 18-14 numaralı listede bu yer tutucu uygulamasını görebilirsiniz.

<Listing number="18-14" file-name="src/lib.rs" caption="Her zaman boş bir dize dilimi döndüren `content` yönteminin `Post` üzerindeki yer tutucu uygulamasının eklenmesi">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

</Listing>

Bu eklenmiş `content` yöntemi ile, 18-11 numaralı listedeki 7. satıra kadar her şey istenildiği gibi çalışıyor.

<!-- Eski başlıklar. Kaldırmayın yoksa bağlantılar bozulabilir. -->

<a id="requesting-a-review-of-the-post-changes-its-state"></a>

#### İnceleme İstemek Gönderinin Durumunu Değiştirir

Sonraki adımda, bir gönderinin incelemesini istemek için işlevsellik eklememiz gerekiyor; bu, durumunu `Draft`'tan `PendingReview`'a değiştirmelidir. Bu kod 18-15 numaralı listede gösterilmektedir.

<Listing number="18-15" file-name="src/lib.rs" caption="`Post` ve `State` trait'inde `request_review` yönteminin uygulanması">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

</Listing>

`Post`'a, kendisine değişken bir referans alacak şekilde bir `request_review` adlı genel bir yöntem veriyoruz. Ardından, `Post`'un mevcut durumunun içindeki `request_review` adlı dahili bir yöntemi çağırıyoruz ve bu ikinci `request_review` yöntemi mevcut durumu tüketip yeni bir durum döndürüyor.

`request_review` yöntemini `State` trait'ine ekliyoruz; trait'i uygulayan tüm türler artık `request_review` yöntemini uygulamak zorunda kalacak. İlk parametre olarak `self`, `&self` veya `&mut self` yerine `self: Box<Self>` kullanıyoruz. Bu sözdizimi, yöntemin yalnızca türü tutan bir `Box` üzerinde çağrıldığında geçerli olduğunu ifade eder. Bu sözdizimi, durumu sahiplenerek eski durumu geçersiz kılar, böylece `Post`'un durum değeri yeni bir duruma dönüşebilir.

Eski durumu tüketmek için, `request_review` yöntemi durum değerinin sahipliğini almalıdır. Bu, `Post`'un durum değerini hareket ettirmemize olanak tanır. Ardından, `state` değerini bu işlemin sonucuna ayarlayacağız.

`state`'i geçici olarak `None`'a ayarlamamızın nedeni, doğrudan `self.state = self.state.request_review();` gibi bir kodla ayarlamaktan ziyade, durum değerinin sahipliğini elde etmektir. Bu, `Post`'un eski `state` değerini kullanmasını engeller.

`Draft` üzerindeki `request_review` yöntemi, bir gönderinin inceleme beklediği durumu temsil eden yeni bir `PendingReview` struct'ısının yeni, kutulu bir örneğini döndüren bir yöntemdir. `PendingReview` struct'ı da `request_review` yöntemini uygular, ancak herhangi bir dönüşüm yapmaz. Bunun yerine, kendisini döndürür; çünkü bir gönderinin zaten `PendingReview` durumunda inceleme istemesi durumunda, `PendingReview` durumunda kalmalıdır.

Artık durum deseninin avantajlarını görmeye başlayabiliriz: `Post` üzerindeki `request_review` yöntemi, durum değeri ne olursa olsun aynıdır. Her durum kendi kurallarından sorumludur.

`content` yöntemini olduğu gibi bırakacağız; boş bir dize dilimi döndürmeye devam edecek. Artık `PendingReview` durumunda da bir `Post`'a sahip olabiliriz, ayrıca `Draft` durumunda da, ancak `PendingReview` durumundaki davranış aynı olmalıdır. 18-11 numaralı liste artık 10. satıra kadar çalışıyor!

<!-- Eski başlıklar. Kaldırmayın yoksa bağlantılar bozulabilir. -->

<a id="adding-the-approve-method-that-changes-the-behavior-of-content"></a>

#### `approve` Ekleme ile `content`'in Davranışını Değiştirme

`approve` yöntemi, `request_review` yöntemine benzer olacaktır: durumu, o durumun onaylandığında sahip olması gereken değere ayarlayacaktır. Bu, 18-16 numaralı listede gösterilmiştir.

<Listing number="18-16" file-name="src/lib.rs" caption="`Post` ve `State` trait'inde `approve` yönteminin uygulanması">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

</Listing>

`approve` yöntemini `State` trait'ine ekliyoruz ve `State`'i uygulayan yeni bir yapı olan `Published` durumunu ekliyoruz.

`PendingReview` üzerindeki `approve` yöntemi gibi, `Draft` üzerindeki `approve` çağrısı da etkisiz olacaktır; çünkü `approve`, kendisini döndürecektir. `PendingReview` üzerinde `approve` çağırdığımızda, bu yeni bir `Published` struct'ı örneği döndürür. `Published` struct'ı, `State` trait'ini uygular ve hem `request_review` hem de `approve` yöntemleri için kendisini döndürür; çünkü gönderi bu durumlarda `Published` durumunda kalmalıdır.

Artık `Post` üzerindeki `content` yöntemini güncellememiz gerekiyor. `content`'in döndürdüğü değerin `Post`'un mevcut durumuna bağlı olmasını istiyoruz, bu yüzden `Post`, `state`'inin üzerinde tanımlı bir `content` yöntemini devretmesini sağlayacağız. Bu, 18-17 numaralı listede gösterilmiştir.

<Listing number="18-17" file-name="src/lib.rs" caption="`Post` üzerindeki `content` yönteminin, `State` üzerindeki bir `content` yöntemini devretmek üzere güncellenmesi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src.lib.rs:here}}
```

</Listing>

Amacımız, tüm bu kuralları `State`'i uygulayan yapıların içinde tutmak olduğundan, `state` içindeki değere bir `content` yöntemi çağırıyoruz ve gönderi örneğini (yani `self`) bir argüman olarak geçiriyoruz. Ardından, `state` değerindeki `content` yöntemini kullanarak döndürülen değeri döndürüyoruz.

`Option` üzerindeki `as_ref` yöntemini çağırıyoruz çünkü `state` içindeki değere sahip olmak değil, ona referans almak istiyoruz. `state`, `Option<Box<dyn State>>` olduğundan, `as_ref` çağırdığımızda `Option<&Box<dyn State>>` döndürülür. Eğer `as_ref` çağırmazsak, fonksiyon parametresi olan `&self`'in hareket etmesine izin verilmediği için hata alırız.

Ardından, `unwrap` yöntemini çağırıyoruz; bu, `state`'in bu yöntemler tamamlandığında her zaman bir `Some` değeri içereceğini bildiğimiz için panik yapmayacaktır. Bu, 9. Bölümde, derleyicinin anlayamadığı durumlarda bile, belirli bir değerin (bu durumda `None`) asla olamayacağını bildiğimiz durumlar için geçerli olan bir durumdur.

Bu noktada, `&Box<dyn State>` üzerindeki `content`'i çağırdığımızda, `&` ve `Box` üzerindeki deref dönüştürmesi etkili olacaktır, bu nedenle `content` yöntemi nihayetinde `State` trait'ini uygulayan tür üzerinde çağrılacaktır. Bu nedenle, `State` trait tanımına `content` eklememiz gerekiyor ve işte burada, hangi duruma sahip olduğumuza bağlı olarak hangi içeriğin döndürüleceği ile ilgili mantığı koyacağız. 18-18 numaralı listede gösterilmiştir.

<Listing number="18-18" file-name="src/lib.rs" caption="`State` trait'ine `content` yönteminin eklenmesi">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src.lib.rs:here}}
```

</Listing>

`content` yöntemi için varsayılan bir uygulama ekliyoruz; bu, `content` yöntemini `Draft` ve `PendingReview` üzerinde uygulamamız gerekmediği anlamına gelir. `Published` struct'ı, `content` yöntemini geçersiz kılar ve `post.content` değerini döndürür. Ancak, `State` üzerindeki `content` yönteminin `Post`'un `content`'ini belirlemesi, `State` ve `Post`'un sorumlulukları arasındaki sınırları bulanıklaştırmaktadır.

Bu yöntemde, argüman olarak bir `post` referansı alıyoruz ve o `post`'un bir parçasına referans döndürüyoruz, bu nedenle döndürülen referansın ömrü, `post` argümanının ömrü ile ilişkilidir.

Ve işte bu kadar! Artık 18-11 numaralı listenin tamamı çalışıyor! Durum desenini, blog yazısı iş akışının kurallarıyla birlikte uyguladık. İlgili mantık, `Post` içinde dağınık olmak yerine durum nesnelerinde yer alıyor.

> ### Neden Bir Enum Değil?
>
> Enum kullanarak, farklı olası gönderi durumlarını varyantlar olarak tanımlamayı düşünmüş olabilirsiniz. Bu kesinlikle mümkün bir çözüm; deneyin ve hangisini tercih ettiğinizi görmek için son sonuçları karşılaştırın! Enum kullanmanın bir dezavantajı, enum'un değerini kontrol eden her yerde her olası varyantı ele almak için bir `match` ifadesine ihtiyaç duyulmasıdır. Bu, bu trait nesnesi çözümünden daha fazla tekrara yol açabilir.

#### Durum Deseninin Dezavantajları

Rust'ın, bir gönderinin her durumda sahip olması gereken farklı türdeki davranışları kapsüllemek için nesne yönelimli durum desenini uygulamakta yetenekli olduğunu gösterdik. `Post` üzerindeki yöntemler, çeşitli davranışlar hakkında hiçbir şey bilmez. Kodumuzu düzenleme şeklimizle, bir yayınlanmış gönderinin nasıl davranabileceğini bilmek için yalnızca bir yere bakmamız yeterlidir: `Published` struct'ında `State` trait'inin uygulanması.

Alternatif bir uygulama oluşturursak, durum desenini kullanmazsak, bunun yerine `Post` üzerindeki yöntemlerde veya gönderinin durumunu kontrol eden `main` kodunda her durumu kontrol eden `match` ifadeleri kullanabiliriz. Bu, bir gönderinin yayınlanmış durumda olmasının tüm sonuçlarını anlamak için birkaç yere bakmamız gerektiği anlamına gelir.

Durum desenini kullanarak yapılan uygulama, yeni işlevsellik eklemeyi kolaylaştırır. Durum desenini kullanan kodun bakımının ne kadar basit olduğunu görmek için, aşağıdaki önerilerden birkaçını deneyin:

- `PendingReview` durumundan durumu `Draft`'a geri döndüren bir `reject` yöntemi ekleyin.
- Durumun `Published`'a değişmesi için `approve` yöntemine iki kez çağrı yapılmasını gerektirin.
- Kullanıcıların yalnızca bir gönderi `Draft` durumundayken metin içeriği eklemesine izin verin. İpucu: durum nesnesinin, içeriğin neyin değişebileceğinden sorumlu olmasını ancak `Post`'u değiştirmekten sorumlu olmamasını sağlayın.

Durum deseninin bir dezavantajı, durumların durumlar arası geçişleri uyguladığından, bazı durumların birbirine bağlı olmasıdır. Eğer `PendingReview` ile `Published` arasında, örneğin `Scheduled` gibi başka bir durum eklersek, `PendingReview` kodunu `Scheduled`'a geçecek şekilde değiştirmemiz gerekecektir. Bu, başka bir tasarım desenine geçmekten daha az iş olacaktır.

Başka bir dezavantaj, bazı mantıkları çoğaltmış olmamızdır. Çoğaltmayı ortadan kaldırmak için, `State` trait'inde varsayılan uygulamalar yapmayı deneyebiliriz. Ancak bu işe yaramaz: `State` trait'ini nesne yönelimli bir şekilde kullandığımızda, trait tam olarak hangi somut `self`'in kullanılacağını bilmez, bu nedenle dönüş türü derleme zamanında bilinmez. (Bu, daha önce bahsedilen `dyn` uyumluluğu kurallarından biridir.)

Başka bir çoğaltma, `Post` üzerindeki `request_review` ve `approve` yöntemlerinin benzer uygulamalarıdır. Her iki yöntem de `Post`'un `state` alanı ile `Option::take` kullanır ve `state` bazı ise, sarılı değerin aynı yöntemini uygular ve `state` alanının yeni değerini ayarlar. Eğer `Post` üzerinde bu deseni takip eden birçok yöntem olsaydı, tekrarı ortadan kaldırmak için bir makro tanımlamayı düşünebilirdik (bkz. 20. Bölümdeki "Makrolar").

Durum desenini, nesne yönelimli diller için tanımlandığı gibi uygulayarak, Rust'ın güçlü yönlerinden tam olarak faydalanmıyoruz. Hatalı durumları ve geçişleri derleme zamanı hatalarına dönüştüren bazı değişiklikleri `blog` crate'inde görelim.

### Durumları ve Davranışları Türler Olarak Kodlama

Durum desenini yeniden düşünerek farklı bir avantaj/dezavantaj seti elde etmeyi göstereceğiz. Durumları ve geçişleri tamamen kapsüllemek yerine, türlere kodlayacağız. Sonuç olarak, Rust'ın tür kontrol sistemi, yalnızca yayınlanmış gönderilerin izin verildiği yerlerde taslak gönderilerin kullanılmaya çalışılmasını önleyecektir.

Şimdi, 18-11 numaralı listenin `main` fonksiyonunun ilk kısmını ele alalım:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:here}}
```

</Listing>

Hala `Post::new` ile taslak durumunda yeni gönderiler oluşturma ve gönderinin içeriğine metin ekleme yeteneğini sağlıyoruz. Ancak, taslak bir gönderinin içeriğini döndüren bir `content` yöntemine sahip olmasını sağlamaktan kaçınıyoruz. Bu nedenle, bir taslak gönderinin içeriğini almaya çalışırsak, bu yöntem mevcut değildir şeklinde bir derleme zamanı hatası alırız. Bu, taslak gönderi içeriğinin üretimde yanlışlıkla görüntülenmesini önleyecektir. 18-19 numaralı listede, bir `Post` struct'ının ve bir `DraftPost` struct'ının tanımı ile her biri üzerindeki yöntemleri görebilirsiniz.

<Listing number="18-19" file-name="src/lib.rs" caption="`content` yöntemine sahip bir `Post` ve `DraftPost` tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-19/src/lib.rs}}
```

</Listing>

Hem `Post` hem de `DraftPost` struct'larının özel bir `content` alanı vardır ve bu alan blog yazısı metnini saklar. Artık `state` alanına ihtiyacımız yok çünkü durum kodlamasını yapıların türlerine taşıyoruz. `Post` yapısı, yayınlanmış bir gönderiyi temsil edecek ve `content` yöntemine sahip olacak, bu yöntem de `content` değerini döndürecek.

Hala bir `Post::new` işlevine sahibiz, ancak bu işlev artık bir `Post` örneği döndürmek yerine bir `DraftPost` örneği döndürüyor. `content` özel olduğundan ve `Post` döndüren hiçbir işlev olmadığından, şu anda `Post` örneği oluşturmak mümkün değildir.

`DraftPost` yapısının bir `add_text` yöntemi vardır, böylece daha önce olduğu gibi `content`'e metin ekleyebiliriz, ancak dikkat edin ki `DraftPost` üzerinde tanımlı bir `content` yöntemi yoktur! Bu nedenle, şimdi gönderi içeriğini almaya çalışmak derleme zamanı hatasına yol açacaktır. Taslak gönderilerin yalnızca taslak durumunda kalmasını ve içeriklerinin görüntülenememesini sağlamak için bu türden yararlanıyoruz. Herhangi birinin bu kısıtlamaları aşmaya çalışması durumunda bir derleyici hatası ile sonuçlanacaktır.

<!-- Eski başlıklar. Kaldırmayın yoksa bağlantılar bozulabilir. -->

<a id="implementing-transitions-as-transformations-into-different-types"></a>

Peki, yayınlanmış bir gönderiyi nasıl alırız? Taslak bir gönderinin incelenip onaylanmadan yayınlanamayacağı kuralını zorunlu kılmak istiyoruz. İnceleme bekleyen bir gönderi, hâlâ içerik göstermemelidir. Bu kısıtlamaları, `PendingReviewPost` adlı başka bir yapı ekleyerek, `DraftPost` üzerinde `request_review` yöntemini tanımlayarak ve `PendingReviewPost` üzerinde bir `approve` yöntemi tanımlayarak uygulayalım. Bu, 18-20 numaralı listede gösterilmiştir.

<Listing number="18-20" file-name="src/lib.rs" caption="`DraftPost` üzerinde `request_review` çağrıldığında oluşturulan bir `PendingReviewPost` ve `PendingReviewPost`'u yayınlanmış bir `Post`'a dönüştüren bir `approve` yöntemi">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-20/src/lib.rs:here}}
```

</Listing>

`request_review` ve `approve` yöntemleri, `self`'in sahipliğini alır; böylece `DraftPost` ve `PendingReviewPost` örneklerini sırasıyla bir `PendingReviewPost` ve bir yayınlanmış `Post`'a dönüştürür. Bu şekilde, `request_review` çağrıldığında artık bekleyen `DraftPost` örneklerimiz olmaz ve aynı şekilde devam ederiz. `PendingReviewPost` yapısının üzerinde tanımlı bir `content` yöntemi yoktur, bu nedenle içeriğini okumaya çalışmak, `DraftPost` gibi derleme zamanı hatasına yol açar. Ancak, üzerinde `content` yöntemi tanımlı bir `Post` almak için tek yol, bir `PendingReviewPost` üzerindeki `approve` yöntemini çağırmaktır ve bir `PendingReviewPost` almak için tek yol, bir `DraftPost` üzerindeki `request_review` yöntemini çağırmaktır. Böylece, blog yazısı iş akışını tür sistemine kodlamış olduk.

Ancak, `main` üzerinde bazı küçük değişiklikler yapmamız gerekiyor. `request_review` ve `approve` yöntemleri, çağrıldıkları struct'ı değiştirmek yerine yeni örnekler döndürdüklerinden, döndürülen örnekleri kaydetmek için daha fazla `let post =` gölgeleme ataması eklememiz gerekiyor. Ayrıca, taslak ve inceleme bekleyen gönderilerin içeriklerinin boş dize olmaması gerektiği gibi, bu içeriklerin boş dize olmasını bekleyen `assert_eq!` ifadelerini de ekleyemeyiz. `main`'deki güncellenmiş kod 18-21 numaralı listede gösterilmiştir.

<Listing number="18-21" file-name="src/main.rs" caption="Blog yazısı iş akışının yeni uygulamasını kullanmak için `main`'deki değişiklikler">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-21/src/main.rs}}
```

</Listing>

`post`'u yeniden atamak zorunda olmamız, bu uygulamanın nesne yönelimli durum desenini tam olarak takip etmediği anlamına geliyor: durumlar arasındaki dönüşümler artık tamamen `Post` uygulaması içinde kapsüllenmemiştir. Ancak, kazancımız, geçersiz durumların artık tür sistemi ve derleme zamanında gerçekleşen tür kontrolü sayesinde imkansız hale gelmesidir! Bu, bazı hataların, örneğin, bir yayımlanmamış gönderinin içeriğinin görüntülenmesi gibi, üretime geçmeden önce keşfedileceğini garanti eder.

`main` fonksiyonundaki 18-21 numaralı listeden sonraki `blog` crate'ine önerilen görevleri deneyin; kodun bu versiyonundaki tasarım hakkında ne düşündüğünüzü görün. Bazı görevların bu tasarımda zaten tamamlanmış olabileceğini unutmayın.

Rust'ın nesne yönelimli tasarım desenlerini uygulamakta yetenekli olduğunu gördük, ancak durumun tür sistemine kodlanması gibi diğer desenler de Rust'ta mevcuttur. Bu desenlerin farklı avantaj ve dezavantajları vardır. Nesne yönelimli desenlere çok aşina olsanız bile, Rust'ın özelliklerinden yararlanmak için sorunu yeniden düşünmek, derleme zamanında bazı hataların önlenmesi gibi faydalar sağlayabilir. Nesne yönelimli desenler, Rust'ın sahiplik gibi bazı özellikleri nedeniyle her zaman en iyi çözüm olmayabilir.

## Özet

Bu bölümü okuduktan sonra Rust'ın nesne yönelimli bir dil olup olmadığını düşünseniz de, Rust'ta bazı nesne yönelimli özellikler elde etmek için trait nesnelerini kullanabileceğinizi artık biliyorsunuz. Dinamik dispatch, kodunuza biraz çalışma zamanı performansı karşılığında esneklik kazandırabilir. Bu esnekliği, kodunuzun bakımını kolaylaştırabilecek nesne yönelimli desenleri uygulamak için kullanabilirsiniz. Rust ayrıca, nesne yönelimli dillerde bulunmayan sahiplik gibi diğer özelliklere de sahiptir. Nesne yönelimli bir desen, Rust'ın güçlü yönlerinden yararlanmanın her zaman en iyi yolu olmayabilir, ancak mevcut bir seçenektir.

Şimdi, Rust'ın çok fazla esneklik sağlayan bir başka özelliği olan desenlere bakalım. Kitap boyunca kısaca onlara değindik ama henüz tam yeteneklerini görmedik. Haydi gidelim!
