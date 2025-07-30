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

Just like `Send` and `Sync`, the compiler automatically implements `Unpin` for all types it can prove are safe. The special case, as with `Send` and `Sync`, is when a type does _not_ implement `Unpin`. This is shown as <code>impl !Unpin for <em>SomeType</em></code>; here, <code><em>SomeType</em></code> is a type that needs to provide these guarantees to be safe when used with a pointer.

So, keep two things in mind about the relationship between `Pin` and `Unpin`. First, `Unpin` is the "normal" case, and `!Unpin` is the special case. Second, whether a type implements `Unpin` or `!Unpin` only matters when you use a pointer wrapped in `Pin` to that type, like <code>Pin<&mut <em>SomeType</em>></code>.

To make this concrete, consider a `String`: it has a length and characters. We can wrap a `String` in a `Pin` (see Figure 17-8). But `String` automatically implements `Unpin`, as do most types in Rust.

<figure>

<img alt="Concurrent workflow" src="img/trpl17-08.svg" class="center" />

<figcaption>Figure 17-8: Pinning a `String`; the dashed line shows that `String` implements `Unpin` and thus is not actually pinned.</figcaption>

</figure>

As a result, we can do things that would be forbidden if `String` implemented `!Unpin`, like replacing a string in memory with a completely different string (see Figure 17-9). This doesn't violate the `Pin` contract, because `String` doesn't have internal references that would make moving it unsafe! That's exactly why it implements `Unpin`, not `!Unpin`.

<figure>

<img alt="Concurrent workflow" src="img/trpl17-09.svg" class="center" />

<figcaption>Figure 17-9: Replacing a `String` in memory with a completely different `String`.</figcaption>

</figure>

Now, we know enough to understand the errors reported for the `join_all` call in 17-17. We tried to move futures produced from async blocks into a `Vec<Box<dyn Future<Output = ()>>>`, but as we've seen, these futures may have internal references, so they don't implement `Unpin`. We need to pin them, and then put the `Pin` type into the `Vec`, so the underlying data in the futures is _not moved_.

`Pin` and `Unpin` mostly matter when building low-level libraries or a runtime; you usually don't need them in everyday Rust code. But when you see these traits in error messages, you'll now have a better idea of how to fix your code!

> Note: This combination of `Pin` and `Unpin` makes it possible to safely implement complex types in Rust that would otherwise be difficult because they're self-referential. Types that require pinning are most common in async Rust today, but you may see them in other contexts as well.
>
> The details of how `Pin` and `Unpin` work and the rules they must follow are explained in detail in the `std::pin` API documentation; start there if you want to learn more.
>
> For an even deeper understanding, see Chapters [2.][under-the-hood] and [4.][pinning] of the [_Asynchronous Programming in Rust_][async-book] book.

### The `Stream` Trait

Now that you have a better understanding of the `Future`, `Pin`, and `Unpin` traits, let's focus on the `Stream` trait. As you learned earlier in the chapter, streams are like asynchronous iterators. However, unlike `Iterator` and `Future`, there is currently no definition of `Stream` in the standard library; but a widely used definition exists in the `futures` crate.

Let's review the definitions of the `Iterator` and `Future` traits, then see how a `Stream` trait can combine them. From `Iterator`, we get the idea of a sequence: the `next` method returns an `Option<Self::Item>`. From `Future`, we get the idea of readiness over time: the `poll` method returns a `Poll<Self::Output>`. To represent a sequence of items that become ready over time, we define a `Stream` trait that combines these features:

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

The `Stream` trait defines an associated type called `Item`, which specifies the type of items the stream produces. This is similar to `Iterator`, since there can be zero or more items; in `Future`, there's always a single `Output`, even if it's the unit type `()`.

`Stream` also defines a method to get these items. We call it `poll_next`, to emphasize that it both polls like `Future::poll` and produces a sequence like `Iterator::next`. The return type combines `Poll` and `Option`. The outer type is `Poll`, because we need to check readiness like a future. The inner type is `Option`, because we need to indicate whether there are more messages, like an iterator.

Something very similar to this will likely become part of Rust's standard library. For now, it's available in most runtime toolkits, so you can use it with confidence, and what we say next will generally apply!

In the example we saw in the streams section, we didn't use `poll_next` or `Stream` directly; instead, we used `next` and `StreamExt`. Of course, we could have written our own stream state machines and worked directly with the `poll_next` API, just as we could work directly with the `poll` method for futures. But using `await` is much nicer, and the `StreamExt` trait provides the `next` method so we can do just that:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-stream-ext/src/lib.rs:here}}
```

<!--
TODO: When crates like tokio update MSRV and support async functions in traits, update this section.
-->

> Note: The actual definition we used earlier in the chapter looks a bit different, because it also supports Rust versions that don't support async functions in traits. As a result, it looks like this:
>
> ```rust,ignore
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> This `Next` type is a `struct` that implements `Future`, and the lifetime in `Next<'_, Self>` allows us to name the reference's lifetime, so we can use `await` with this method.

The `StreamExt` trait is also home to all the interesting methods you can use with streams. `StreamExt` is automatically implemented for any type that implements `Stream`; but these traits are defined separately so the community can develop helper APIs without affecting the core trait.

In the version of `StreamExt` used in the `trpl` crate, the trait not only defines the `next` method, but also provides a default implementation of `next` that correctly calls `Stream::poll_next`. So, even if you write your own stream data type, you only need to implement `Stream`, and anyone using your type can automatically use `StreamExt` and its methods.

That's all we have to say about the low-level details of these traits. Finally, let's think about how futures (including streams), tasks, and threads fit together!

[ch-18]: ch18-00-oop.html
[async-book]: https://rust-lang.github.io/async-book/
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[first-async]: ch17-01-futures-and-syntax.html#our-first-async-program
[any-number-futures]: ch17-03-more-futures.html#working-with-any-number-of-futures
