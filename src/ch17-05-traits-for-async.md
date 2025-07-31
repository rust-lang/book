## A Closer Look at Traits for Async

<!-- Old headings. Please do not delete, links may break. -->

<a id="digging-into-the-traits-for-async"></a>

Throughout this chapter, we've used the `Future`, `Pin`, `Unpin`, `Stream`, and `StreamExt` traits in various ways. So far, we haven't delved much into how they work or fit together; for most of your daily Rust work, that's fine. But sometimes, you'll encounter situations where you need to understand a bit more detail. In this section, we'll go deep enough to help in those scenarios; for a _really_ in-depth look, we'll leave that to other documentation.

<!-- Old headings. Please do not delete, links may break. -->

<a id="future"></a>

### `Gelecek` Özelliği

İlk olarak, `Future` özelliğinin nasıl çalıştığına daha yakından bakalım. Rust bunu şu şekilde tanımlar:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Bu özellik tanımı bir dizi yeni tip ve daha önce görmediğimiz bazı sözdizimi içeriyor, bu yüzden parça parça inceleyelim.

İlk olarak, `Future` ile ilişkili `Output` tipi geleceğin neye çözümleneceğini belirtir. Bu, `Iterator` özelliğinin `Item` ilişkili tipine benzer. İkinci olarak, `Future` ayrıca bir `Pin` referansı ve bir `Context` için değiştirilebilir bir referans alan ve bir `Poll<Self::Output>` döndüren özel bir `poll` yöntemine sahiptir.` Pin` ve `Context` konularına birazdan değineceğiz. Şimdilik, yöntem tarafından döndürülen `Poll` türüne odaklanalım:

```rust
enum Poll<T> {
    Ready(T),
    Pending,
}
```

Bu `Poll` tipi bir nevi `Option` gibidir. Değer içeren bir `Ready(T)` çeşidi ve değer içermeyen bir `Pending` çeşidi vardır. Ancak `Poll`, `Option`dan oldukça farklı bir anlama gelir! Beklemede değişkeni geleceğin hala yapacak işleri olduğunu ve çağıranın daha sonra tekrar kontrol etmesi gerektiğini belirtir.` Ready` değişkeni ise geleceğin işinin bittiğini ve `T` değerinin hazır olduğunu ifade eder.

> Not: Çoğu future için, future `Ready` değerini döndürdükten sonra `poll` seçeneğini tekrar çağırmamalısınız. Birçok gelecek, hazır olduktan sonra tekrar sorgulanırsa panikleyecektir. Tekrar sorgulanabilen future'lar bunu dokümantasyonlarında açıkça belirtecektir. Bu `Iterator::next` davranışına benzer.

Kodunuzda `await` kullandığınızda, Rust bunu perde arkasında `poll` çağıran bir kod olarak derler. 17-4'te çözümlendiğinde bir URL'nin başlığını yazdırdığımız örneğe bakarsanız, Rust bunu kabaca (tam olarak olmasa da) aşağıdaki gibi derler:

```rust,ignore
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
    Pending => {
        // But what happens here?
    }
}
```

Eğer gelecek hala `Bekliyor` ise, ne yapmalıyız? Tekrar denemek için bir yola ihtiyacımız var - bu yüzden bir döngüye ihtiyacımız var:

```rust,ignore
let mut page_title_fut = page_title(url);
loop {
    match page_title_fut.poll() {
        Ready(value) => match page_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
        Pending => {
            // keep going
        }
    }
}
```

Ancak Rust bunu tam olarak bu şekilde derleseydi, her `await' noktası bloke edici olurdu - istediğimizin tam tersi! Bunun yerine Rust, döngünün kontrolü başka bir gelecek üzerinde çalışabilecek başka bir şeye vermesine ve daha sonra bunu tekrar kontrol etmek için geri gelmesine izin verir. Gördüğümüz gibi, bu şey bir asenkron çalışma zamanıdır ve bu zamanlama ve koordinasyon onun ana işlerinden biridir.

Bölümün başlarında `rx.recv` çağrısını beklemekten bahsetmiştik. `Recv` çağrısı bir gelecek döndürür ve geleceğin onu yoklamasını bekler. Çalışma zamanı, hazır olana kadar geleceği askıya alır ve kanal kapandığında ya `Some(message)` ya da `None` döndürür. Şimdi `Future` özelliğini ve özellikle `Future::poll` özelliğini daha iyi anladığımıza göre, bunun nasıl çalıştığını görebiliriz. Çalışma zamanı, `poll` `Poll::Pending` döndürdüğünde geleceğin hazır olmadığını bilir. Tersine, `poll` `Poll::Ready(Some(message))` veya `Poll::Ready(None)` döndürdüğünde, gelecek hazırdır ve çalışma zamanı devam edebilir.

Bir çalışma zamanının bunu tam olarak nasıl yaptığı bu kitabın kapsamı dışındadır, ancak geleceklerin temel mekaniğini görmek önemlidir: çalışma zamanı sorumlu olduğu her geleceği _poll_ eder ve gelecek henüz hazır değilse, onu tekrar uykuya yatırır.
<!-- Old headings. Please do not delete, links may break. -->

<a id="pinning-and-the-pin-and-unpin-traits"></a>

### `Pin` ve `Unpin` Özellikleri

Liste 17-16'da sabitleme kavramını tanıttığımızda, oldukça karmaşık bir hata mesajıyla karşılaştık. İşte ilgili kısım tekrar:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-16
cargo build
copy *only* the final `error` block from the errors
-->

```text
error[E0277]: `{async block@src/main.rs:10:23: 10:33}` cannot be unpinned
  --> src/main.rs:48:33
   |
48 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<{async block@src/main.rs:10:23: 10:33}>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`
```

Bu hata mesajı bize sadece değerleri sabitlememiz gerektiğini değil, aynı zamanda sabitlemenin neden gerekli olduğunu da söyler. `trpl::join_all` fonksiyonu `JoinAll` adında bir struct döndürür. Bu struct, `Future` özelliğini uygulaması gereken bir `F` tipi üzerinde geneldir. Bir geleceği doğrudan beklemek onu dolaylı olarak sabitler, böylece bir geleceği beklemek istediğiniz her yerde `pin!` kullanmanıza gerek kalmaz.

Ancak burada, bir geleceği doğrudan beklemiyoruz. Bunun yerine, `JoinAll` adında yeni bir gelecek oluşturan `join_all`'a bir gelecek koleksiyonu veriyoruz. `Join_all`un imzası, koleksiyondaki öğelerin `Future` özelliğini uygulamasını gerektirir ve `Box<T>` yalnızca sarılmış `T` geleceği `Unpin` özelliğini uygularsa `Future` özelliğini uygular.

Bu çok fazla bilgi demek! Gerçekten anlamak için, `Future` özelliğinin _pinning_ kısmının nasıl çalıştığına biraz daha yakından bakalım.

Future` özelliğinin tanımına tekrar bakın:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`cx` parametresi ve onun `Context` tipi, bir çalışma zamanının herhangi bir geleceği ne zaman kontrol edeceğini nasıl bildiğinin anahtarıdır. Bunun nasıl çalıştığının ayrıntıları bu bölümün kapsamı dışındadır ve genellikle yalnızca özel bir `Future` uygulaması yazarken önemlidir. Bunun yerine, bir metodun ilk kez `self` için bir tip ek açıklaması aldığı sözdizimine odaklanalım. self` için tip ek açıklaması diğer fonksiyon parametresi tip ek açıklamaları gibi çalışır, ancak iki önemli fark vardır:

- Rust`a metodun çağrılabilmesi için `self` türünün ne olması gerektiğini söyler.
- Herhangi bir tür olamaz. Yalnızca yöntemin uygulandığı tür, bu türe bir referans, akıllı bir işaretçi veya bu türe bir referansı saran bir `Pin` olabilir.

Bu sözdiziminin daha fazlasını [Bölüm 18][ch-18]<!-- ignore --> bölümünde göreceksiniz. Şimdilik bilmeniz gereken şey, bir geleceğin `Pending` ya da `Ready(Output)` olup olmadığını kontrol etmek için, `Pin` içine sarılmış tipe değişken bir referansa ihtiyacınız olduğudur.

`Pin`, `&`, `&mut`, `Box` ve `Rc` gibi işaretçi benzeri tipler için bir sarmalayıcıdır. (Teknik olarak, `Pin` `Deref` veya `DerefMut` özelliklerini uygulayan türlerle çalışır, ancak pratikte bu işaretçiler anlamına gelir). `Pin`in kendisi bir işaretçi değildir ve `Rc` veya `Arc` gibi kendi davranışına sahip değildir; sadece derleyicinin işaretçi kullanımına kısıtlamalar getirmesine yardımcı olan bir araçtır.

`await`in perde arkasında `poll` çağrılarıyla uygulandığını hatırlamak, daha önce gördüğümüz hata mesajını açıklamaya yardımcı olur; ancak bu hata `Pin` ile değil `Unpin` ile ilgiliydi. Peki, nasıl

`await`in perde arkasında `poll` çağrılarıyla uygulandığını hatırlamak, daha önce gördüğümüz hata mesajını açıklamaya yardımcı olur; ancak bu hata `Pin` ile değil `Unpin` ile ilgiliydi. Peki, `Pin` tam olarak `Unpin` ile nasıl ilişkilidir ve `Future`un `poll` çağrısı neden `self`in `Pin` tipinde olmasını gerektirir?

Bu bölümün başlarında, bir gelecekteki bekleme noktalarının bir durum makinesine derlendiğini ve derleyicinin bu durum makinesinin Rust'ın olağan güvenlik kurallarına (sahiplik, ödünç alma) uymasını sağladığını söylemiştik. Bunu yapmak için Rust, bir await noktası ile bir sonraki veya asenkron bloğun sonu arasında hangi verilere ihtiyaç duyulduğuna bakar. Ardından, derlenmiş durum makinesinde karşılık gelen bir varyant oluşturur. Her varyant, kaynak kodun o noktasında ihtiyaç duyulan verilere ya sahiplik alarak ya da değişken veya değişmez olarak ödünç alarak erişir.

Buraya kadar her şey yolunda: Bir asenkron blokta sahiplik veya referanslarla ilgili bir hata yaparsanız, ödünç alma denetleyicisi bunu yakalayacaktır. Ancak geleceğin kendisini taşımak istediğinizde - örneğin, bir `Vec` içine koyup `join_all` kullanarak veya bir fonksiyondan döndürerek - işler zorlaşır.

Bir geleceği taşımak -ister bir veri yapısına koymak ister bir fonksiyondan döndürmek olsun- aslında Rust'ın sizin için oluşturduğu durum makinesini taşımak anlamına gelir. Ve Rust'taki çoğu türün aksine, async bloklardan derlenen future'lar herhangi bir varyantın alanlarında kendilerine referanslar içerebilir (bkz. Şekil 17-4).

<figure>

<img alt="A single-column, three-row table representing a future called fut1. The first two rows have values 0 and 1, and the third row has an arrow pointing to the second row, showing the future referencing itself." src="img/trpl17-04.svg" class="center" />

<figcaption>Figure 17-4: A self-referential data type.</figcaption>

</figure>

Ancak, referanslar her zaman başvurdukları gerçek bellek adresini gösterdiğinden, kendi kendine referans veren herhangi bir nesnenin taşınması varsayılan olarak güvenli değildir (bkz. Şekil 17-5). Veri yapısını taşırsanız ve dahili referanslar hala eski konumu gösteriyorsa, o bellek artık geçersizdir. Bu konumda yapılan değişiklikler yeni veri yapısını etkilemez ve daha da önemlisi, bilgisayar bu belleği başka bir şey için kullanabilir! Daha sonra tamamen ilgisiz verileri okumak zorunda kalabilirsiniz.

<figure>

<img alt="Two tables showing two futures, fut1 and fut2. fut1 is gray with question marks in each index (unknown memory). fut2 has 0 and 1, and the third row has an arrow pointing to the second row of fut1, showing a reference to the old location before moving." src="img/trpl17-05.svg" class="center" />

<figcaption>Şekil 17-5: Öz referanslı bir veri türünü taşımanın güvenli olmayan sonucu</figcaption>

</figure>

Teorik olarak, Rust derleyicisi bir nesne taşındığında tüm referansları güncellemeye çalışabilir, ancak bu, özellikle bir referans ağı varsa, büyük bir performans darbesi olacaktır. Bunun yerine, ilgili veri yapısının bellekte _taşınmadığından_ emin olursak, referansları güncellememize gerek kalmaz. Rust'ın ödünç alma denetleyicisi tam olarak bunu gerektirir: güvenli kodda, aktif bir referansı olan herhangi bir öğe taşınamaz.

`Pin` bize tam olarak ihtiyacımız olan garantiyi verir. Bir değere işaretçiyi `Pin` içine sararak, değeri _pin_ yaparız, böylece taşınamaz. Yani, eğer bir `Pin<Box<SomeType>>` varsa, aslında _Box_ işaretçisini değil, `SomeType` değerini sabitlemiş olursunuz. Şekil 17-6 bunu göstermektedir.


<figure>

<img alt="Three boxes side by side: the first is 'Pin', the second is 'b1', the third is 'pinned'. The 'pinned' box contains a table called 'fut', with cells for each part of the future. The first cell is 0, the second cell has an arrow to the fourth cell, the third cell has ... The arrow from 'Pin' goes through 'b1' and 'pinned'." src="img/trpl17-06.svg" class="center" />

<figcaption>Şekil 17-6: Kendinden referanslı bir gelecek türüne işaret eden bir Kutunun sabitlenmesi</figcaption>

</figure>

Aslında, Kutu işaretçisinin kendisi hala serbestçe hareket ettirilebilir. Önemli olan, işaret ettiği verinin aynı yerde kalmasıdır. İşaretçi hareket ettiği ancak işaret ettiği veri hareket etmediği sürece (bkz. Şekil 17-7), sorun yoktur. (Kendi başınıza, bir `Box`ı bir `Pin`e sararak bunu nasıl yapabileceğinizi görmek için tiplerin ve `std::pin` modülünün belgelerine bakmayı deneyin). Kilit nokta, öz referanslı tipin kendisinin taşınamayacağıdır, çünkü hala sabitlenmiştir.

<figure>

<img alt="Four boxes in three columns; same as the previous diagram, but the second column has two boxes: 'b1' and 'b2'. 'b1' is gray, the arrow now goes from 'b2'; the pointer has moved from 'b1' to 'b2', but the data in 'pinned' hasn't moved." src="img/trpl17-07.svg" class="center" />

<figcaption>Şekil 17-7: Kendinden referanslı bir gelecek türüne işaret eden bir Kutunun taşınması</figcaption>

</figure>

Bununla birlikte, çoğu tür bir pine sarılmış olsalar bile taşınmaları tamamen güvenlidir. Yalnızca dahili referansları olan tipler için pinleme hakkında düşünmemiz gerekir. Sayılar ve booleanlar gibi ilkel değerler güvenlidir çünkü iç referansları yoktur. Rust'ta kullandığınız çoğu türün de iç referansları yoktur. Örneğin, bir `Vec`i istediğiniz gibi taşıyabilirsiniz. Eğer bir `Pin<Vec<String>>` olsaydı, `Pin` tarafından sağlanan güvenli ancak kısıtlayıcı API'yi kullanmanız gerekirdi, ancak başka referanslar yoksa bir `Vec<String>` taşımak her zaman güvenlidir. Derleyiciye bu durumlarda öğeleri taşımanın sorun olmadığını söylemenin bir yoluna ihtiyacımız var - işte `Unpin` burada devreye giriyor.

`Unpin`, Bölüm 16'da gördüğümüz `Send` ve `Sync` özellikleri gibi bir işaretleyici özelliktir ve kendi başına hiçbir şey yapmaz. İşaretleyici özellikler yalnızca derleyiciye bir türün belirli bir bağlamda kullanılmasının güvenli olduğunu söylemek için vardır. Unpin` derleyiciye söz konusu tipin taşınmasında bir sakınca olmadığını söyler.

<!--
  The inline `<code>` in the block below is just to emphasize the `<em>` inside, per NoStarch style.
-->

Tıpkı `Send` ve `Sync` gibi, derleyici güvenli olduğunu kanıtlayabildiği tüm tipler için otomatik olarak `Unpin` uygular. Özel durum, `Send` ve `Sync`de olduğu gibi, bir türün `Unpin`i _uygulamadığı_ durumdur. This is shown as <code>impl !Unpin for <em>SomeType</em></code>; here, <code><em>SomeType</em></code> is a type that needs to provide these guarantees to be safe when used with a pointer.

Bu nedenle, `Pin` ve `Unpin` arasındaki ilişki hakkında iki şeyi aklınızda bulundurun.İlk olarak, `Unpin` “normal” durumdur ve `!Unpin` özel durumdur. İkinci olarak, bir türün `Unpin` veya `!Unpin` uygulayıp uygulamadığı yalnızca o türe `Pin` ile sarılmış bir işaretçi kullandığınızda önemlidir, like <code>Pin<&mut <em>SomeType</em>></code>.

Bunu somutlaştırmak için bir `String` düşünün: bir uzunluğu ve karakterleri vardır. Bir `String`i bir `Pin` içine sarabiliriz (bkz. Şekil 17-8). Ancak `String`, Rust`taki çoğu tür gibi otomatik olarak `Unpin`i uygular.

<figure>

<img alt="Concurrent workflow" src="img/trpl17-08.svg" class="center" />

<figcaption>Şekil 17-8: Bir `String`in sabitlenmesi; kesikli çizgi `String`in `Unpin`i uyguladığını ve bu nedenle aslında sabitlenmediğini gösterir.</figcaption>

</figure>

Sonuç olarak, `String` `!Unpin` uygulasaydı yasak olacak şeyleri yapabiliriz, örneğin bellekteki bir dizeyi tamamen farklı bir dizeyle değiştirmek gibi (bkz. Şekil 17-9). Bu `Pin` sözleşmesini ihlal etmez, çünkü `String` dizginin taşınmasını güvensiz kılacak iç referanslara sahip değildir! İşte tam da bu yüzden `!Unpin` değil `Unpin` uygulanmaktadır.

<figure>

<img alt="Concurrent workflow" src="img/trpl17-09.svg" class="center" />

<figcaption>Figure 17-9: Replacing a `String` in memory with a completely different `String`.</figcaption>

</figure>

Şimdi, 17-17'deki `join_all` çağrısı için bildirilen hataları anlamak için yeterince bilgimiz var. Async bloklardan üretilen futures'ları bir `Vec<Box<dyn Future<Output = ()>>` içine taşımaya çalıştık, ancak gördüğümüz gibi, bu futures'lar dahili referanslara sahip olabilir, bu nedenle `Unpin` uygulamasını gerçekleştirmezler. Bunları pinlememiz ve ardından `Pin` türünü `Vec` içine koymamız gerekir, böylece futures'lardaki temel veriler _taşınmaz_.

Pin` ve `Unpin` çoğunlukla düşük seviyeli kütüphaneler veya bir çalışma zamanı oluştururken önemlidir; genellikle günlük Rust kodunda bunlara ihtiyacınız yoktur. Ancak bu özellikleri hata mesajlarında gördüğünüzde, kodunuzu nasıl düzelteceğiniz konusunda artık daha iyi bir fikriniz olacak!

> Not: `Pin` ve `Unpin` kombinasyonu, aksi takdirde kendi kendine referanslı oldukları için zor olabilecek karmaşık tiplerin Rust'ta güvenli bir şekilde uygulanmasını mümkün kılar. Pinning gerektiren tipler günümüzde en çok async Rust'ta yaygındır, ancak bunları başka bağlamlarda da görebilirsiniz.
>
> `Pin` ve `Unpin`in nasıl çalıştığının ayrıntıları ve uymaları gereken kurallar `std::pin` API belgelerinde ayrıntılı olarak açıklanmıştır; daha fazla bilgi edinmek istiyorsanız oradan başlayın.
>
> Daha derin bir anlayış için [_Asynchronous Programming in Rust_][async-book] kitabının [2.][under-the-hood] ve [4.][pinning] bölümlerine bakın.

### `Stream` Özelliği

Artık `Future`, `Pin` ve `Unpin` özelliklerini daha iyi anladığınıza göre, `Stream` özelliğine odaklanalım. Bölümün başlarında öğrendiğiniz gibi, akışlar asenkron iteratörler gibidir. Ancak, `Iterator` ve `Future` özelliklerinin aksine, standart kütüphanede `Stream` özelliğinin bir tanımı yoktur; ancak `futures` crate'inde yaygın olarak kullanılan bir tanım mevcuttur.

Şimdi `Iterator` ve `Future` özelliklerinin tanımlarını gözden geçirelim, ardından bir `Stream` özelliğinin bunları nasıl birleştirebileceğini görelim.` Iterator`dan bir dizi fikrini alırız: `next` metodu bir `Option<Self::Item>` döndürür.` Future`dan, zaman içinde hazır olma fikrini elde ederiz: `poll` yöntemi bir `Poll<Self::Output>` döndürür. Zaman içinde hazır hale gelen bir dizi öğeyi temsil etmek için, bu özellikleri birleştiren bir `Stream` özelliği tanımlarız:


```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

`Stream` özelliği, akışın ürettiği öğelerin türünü belirten `Item` adlı ilişkili bir tür tanımlar. Bu `Iterator` tipine benzer, çünkü sıfır ya da daha fazla öğe olabilir; `Future` tipinde, `()` birim tipi olsa bile her zaman tek bir `Output` vardır.

`Stream` ayrıca bu öğeleri almak için bir yöntem tanımlar. Hem `Future::poll` gibi yoklama yaptığını hem de `Iterator::next` gibi bir dizi ürettiğini vurgulamak için buna `poll_next` adını veriyoruz. Dönüş tipi `Poll` ve `Option`ı birleştirir. Dış tip `Poll`, çünkü bir gelecek gibi hazır olup olmadığını kontrol etmemiz gerekiyor. İç tip `Option`dır, çünkü bir iterator gibi daha fazla mesaj olup olmadığını belirtmemiz gerekir.

Buna çok benzer bir şey muhtemelen Rust'ın standart kütüphanesinin bir parçası olacak. Şimdilik, çoğu çalışma zamanı araç setinde mevcuttur, bu yüzden güvenle kullanabilirsiniz ve bundan sonra söylediklerimiz genellikle geçerli olacaktır!

Akışlar bölümünde gördüğümüz örnekte, doğrudan `poll_next` veya `Stream` kullanmadık; bunun yerine `next` ve `StreamExt` kullandık. Elbette, kendi akış durum makinelerimizi yazabilir ve doğrudan `poll_next` API'si ile çalışabilirdik, tıpkı futures için doğrudan `poll` yöntemi ile çalışabileceğimiz gibi. Ancak `await` kullanmak çok daha güzeldir ve `StreamExt` özelliği `next` yöntemini sağlar, böylece bunu yapabiliriz:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-stream-ext/src/lib.rs:here}}
```

<!--
TODO: When crates like tokio update MSRV and support async functions in traits, update this section.
-->

> Not: Bölümün başlarında kullandığımız gerçek tanım biraz farklı görünüyor, çünkü trait'lerde async fonksiyonlarını desteklemeyen Rust sürümlerini de destekliyor. Sonuç olarak, şu şekilde görünür:
>
> ```rust,ignore
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> Bu `Next` tipi `Future` uygulayan bir `struct`tur ve `Next<'_, Self>` içindeki yaşam süresi referansın yaşam süresini adlandırmamıza izin verir, böylece bu yöntemle `await` kullanabiliriz.

`StreamExt` özelliği de akışlarla kullanabileceğiniz tüm ilginç yöntemlere ev sahipliği yapar. StreamExt` özelliği `Stream` özelliğini uygulayan her tür için otomatik olarak uygulanır; ancak bu özellikler, topluluğun çekirdek özelliği etkilemeden yardımcı API'ler geliştirebilmesi için ayrı olarak tanımlanmıştır.

trpl` crate`inde kullanılan `StreamExt` sürümünde, özellik yalnızca `next` yöntemini tanımlamakla kalmaz, aynı zamanda `Stream::poll_next` yöntemini doğru şekilde çağıran varsayılan bir `next` uygulaması da sağlar. Yani, kendi stream veri tipinizi yazsanız bile, sadece `Stream`i uygulamanız yeterlidir ve tipinizi kullanan herkes otomatik olarak `StreamExt` ve metotlarını kullanabilir.

Bu özelliklerin düşük seviyeli detayları hakkında söyleyeceklerimiz bu kadar. Son olarak, vadeli işlemlerin (akışlar dahil), görevlerin ve iş parçacıklarının birbirine nasıl uyduğunu düşünelim!

[ch-18]: ch18-00-oop.md
[async-book]: https://rust-lang.github.io/async-book/
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[first-async]: ch17-01-futures-and-syntax.md#i̇lk-async-programımız
[any-number-futures]: ch17-03-more-futures.md#herhangi-bir-sayıda-future-ile-çalışmak
