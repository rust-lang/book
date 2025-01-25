## A Closer Look at the Traits for Async

<!-- Old headings. Do not remove or links may break. -->

<a id="digging-into-the-traits-for-async"></a>

পুরো chapter জুড়ে, আমরা বিভিন্ন উপায়ে `Future`, `Pin`, `Unpin`, `Stream`, এবং `StreamExt` trait ব্যবহার করেছি। তবে, এখন পর্যন্ত, আমরা কিভাবে সেগুলো কাজ করে বা কিভাবে সেগুলো একসাথে fit হয় সে বিষয়ে খুব বেশি detail এ যাইনি, যা আপনার day-to-day Rust work এর জন্য বেশিরভাগ সময়ে ঠিক আছে। তবে, মাঝে মাঝে, আপনি এমন পরিস্থিতির সম্মুখীন হবেন যেখানে এই detail গুলোর আরও কিছু বোঝার প্রয়োজন হবে। এই section এ, আমরা সেই scenario গুলোতে help করার জন্য যথেষ্ট discuss করব, still _really_ deep dive অন্য documentation এর জন্য রেখে।

<!-- Old headings. Do not remove or links may break. -->

<a id="future"></a>

### The `Future` Trait

চলুন `Future` trait কিভাবে কাজ করে তা ভালোভাবে দেখে শুরু করি। Rust কিভাবে define করে তা এখানে দেওয়া হলো:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

সেই trait definition এ অনেক নতুন type এবং কিছু syntax আছে যা আমরা আগে দেখিনি, তাই চলুন definition টি step by step আলোচনা করি।

প্রথমত, `Future` এর associated type `Output` বলে future কিসে resolve হয়। এটি `Iterator` trait এর associated type `Item` এর analogue। দ্বিতীয়ত, `Future` এর `poll` method ও আছে, যা তার `self` parameter এর জন্য একটি special `Pin` reference এবং একটি `Context` type এর mutable reference নেয়, এবং একটি `Poll<Self::Output>` return করে। আমরা `Pin` এবং `Context` নিয়ে একটু পরেই আলোচনা করব। আপাতত, চলুন method টি কি return করে সেটির উপর focus করি, `Poll` type:

```rust
enum Poll<T> {
    Ready(T),
    Pending,
}
```

এই `Poll` type টি `Option` এর মতো। এটির একটি variant আছে যার একটি value আছে, `Ready(T)`, এবং একটি আছে যেখানে কোনো value নেই, `Pending`। `Poll` এর মানে `Option` থেকে বেশ আলাদা! `Pending` variant indicate করে যে future এর এখনও কাজ করার বাকি আছে, তাই caller কে পরে আবার check করার প্রয়োজন। `Ready` variant indicate করে যে future তার কাজ শেষ করেছে এবং `T` value available আছে।

> Note: বেশিরভাগ future এর সাথে, caller এর future `Ready` return করার পর `poll` call করা উচিত না। অনেক future ready হওয়ার পর আবার poll করলে panic করবে। যে future গুলো poll করার জন্য safe সেগুলো তাদের documentation এ explicitly বলবে। এটি অনেকটা `Iterator::next` কিভাবে behave করে তার মতোই।

আপনি যখন `await` ব্যবহার করা code দেখেন, Rust under the hood এ `poll` call করে এমন code এ compile করে। আপনি যদি Listing 17-4 এ ফিরে দেখেন, যেখানে আমরা একটি single URL resolve হওয়ার পর page এর title print করেছিলাম, Rust এটিকে অনেকটা (যদিও exactly নয়) এমন code এ compile করে:

```rust,ignore
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
    Pending => {
        // But what goes here?
    }
}
```

যখন future এখনও `Pending` থাকে তখন আমাদের কি করা উচিত? আমাদের repeat করার জন্য কিছু উপায় দরকার, যতক্ষণ না future finally ready হয়। অন্যভাবে বললে, আমাদের একটি loop এর প্রয়োজন:

```rust,ignore
let mut page_title_fut = page_title(url);
loop {
    match page_title_fut.poll() {
        Ready(value) => match page_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
        Pending => {
            // continue
        }
    }
}
```

যদি Rust এটিকে exactly সেই code এ compile করত, তাহলে প্রত্যেক `await` blocking হতো—যা আমরা চাচ্ছিলাম তার exactly opposite! এর পরিবর্তে, Rust নিশ্চিত করে যে loop এমন কিছুর কাছে control handover করতে পারে যা অন্যান্য future এর উপর কাজ করার জন্য এই future এর কাজ pause করতে পারে এবং পরে আবার এটি check করতে পারে। আমরা যেমন দেখেছি, সেই জিনিসটি হলো একটি async runtime, এবং এই scheduling এবং coordination কাজ হলো এর main job গুলোর মধ্যে একটি।

এই chapter এর শুরুতে, আমরা `rx.recv` এ wait করা describe করেছিলাম। `recv` call একটি future return করে, এবং future await করলে সেটি poll হয়। আমরা note করেছিলাম যে runtime future টি `Some(message)` অথবা channel close হলে `None` দিয়ে ready না হওয়া পর্যন্ত pause করবে। `Future` trait এর এবং specifically `Future::poll` এর গভীর understanding এর সাথে, আমরা দেখতে পারি এটি কিভাবে কাজ করে। Runtime জানে যে future টি ready নয় যখন এটি `Poll::Pending` return করে। বিপরীতে, runtime জানে যে future টি _is_ ready এবং যখন `poll` `Poll::Ready(Some(message))` বা `Poll::Ready(None)` return করে তখন এটিকে advance করে।

Runtime কিভাবে করে তার exact detail এই বইয়ের scope এর বাইরে, কিন্তু মূল বিষয় হলো future এর basic mechanics দেখা: একটি runtime প্রতিটি future _poll_ করে যার জন্য এটি responsible, future ready না হলে এটিকে back to sleep করে দেয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="pinning-and-the-pin-and-unpin-traits"></a>

### The `Pin` and `Unpin` Traits

যখন আমরা Listing 17-16 এ pinning এর idea introduce করেছিলাম, তখন আমরা একটি খুব জটিল error message এর সম্মুখীন হয়েছিলাম। এখানে এর relevant অংশটি আবার দেওয়া হলো:

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
   |                                 ^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`, which is required by `Box<{async block@src/main.rs:10:23: 10:33}>: Future`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<{async block@src/main.rs:10:23: 10:33}>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `join_all`
```

এই error message আমাদের শুধু তাই বলে না যে আমাদের value pin করার প্রয়োজন বরং কেন pinning এর প্রয়োজন সেটিও বলে। `trpl::join_all` function `JoinAll` নামে একটি struct return করে। সেই struct টি একটি type `F` এর উপর generic, যা `Future` trait implement করার জন্য constrained। `await` দিয়ে directly future await করলে implicitly future pin হয়ে যায়। সেই কারণে, আমরা যেখানে future await করতে চাই সেখানে সব জায়গায় আমাদের `pin!` ব্যবহার করার প্রয়োজন হয় না।

তবে, আমরা এখানে directly future await করছি না। এর পরিবর্তে, আমরা `join_all` function এ future এর collection pass করে একটি নতুন future, `JoinAll`, construct করি। `join_all` এর signature require করে যে collection এর item গুলোর type যেনো `Future` trait implement করে, এবং `Box<T>` শুধুমাত্র তখনই `Future` implement করে যখন এটি wrap করা `T` একটি future হয় যা `Unpin` trait implement করে।

এগুলো absorb করার মতো অনেক কিছু! এটা ভালোভাবে বোঝার জন্য, চলুন আমরা `Future` trait আসলে কিভাবে কাজ করে সে বিষয়ে আরও গভীরে যাই, বিশেষ করে _pinning_ এর ব্যাপারে।

`Future` trait এর definition টি আবার দেখুন:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`cx` parameter এবং এর `Context` type হলো সেই key যার মাধ্যমে runtime actually জানে কখন কোনো future check করতে হবে, lazy থাকা সত্ত্বেও। আবারও, কিভাবে কাজ করে তার detail এই chapter এর scope এর বাইরে, এবং যখন আপনি custom `Future` implementation লিখেন তখন সাধারণত আপনার এটা নিয়ে চিন্তা করার প্রয়োজন। আমরা `self` এর type এর উপর focus করব, কারণ এই প্রথম আমরা এমন method দেখছি যেখানে `self` এর একটি type annotation আছে। `self` এর type annotation অন্য function parameter এর type annotation এর মতোই কাজ করে, তবে দুটি key difference আছে:

-   এটি Rust কে বলে যে method call করার জন্য `self` এর type কি হতে হবে।

-   এটা শুধু যেকোনো type হতে পারে না। এটা method যে type এর উপর implemented, সেই type, সেই type এর reference বা smart pointer, অথবা সেই type এর reference wrap করা `Pin` এর মধ্যে সীমাবদ্ধ।

আমরা [Chapter 18][ch-18]<!-- ignore --> এ এই syntax নিয়ে আরও দেখব। আপাতত, আমাদের শুধু এতটুকু জানলেই হবে যে আমরা যদি একটি future কে `Pending` বা `Ready(Output)` কিনা তা check করার জন্য poll করতে চাই, তাহলে আমাদের type এর `Pin` wrap করা mutable reference এর প্রয়োজন।

`Pin` pointer-like type যেমন `&`, `&mut`, `Box`, এবং `Rc` এর wrapper। (Technically, `Pin` সেই type গুলোর সাথে কাজ করে যা `Deref` বা `DerefMut` trait implement করে, কিন্তু এটি effectively pointer এর সাথে কাজ করার মতোই।) `Pin` নিজে কোনো pointer নয় এবং `Rc` এবং `Arc` reference counting এর সাথে যেমন করে তেমন কোনো behaviour ও এর নেই; এটা purely compiler এর জন্য একটি tool যা pointer usage এর উপর constraint enforce করতে পারে।

আমরা যে call এ `poll` ব্যবহার করে implemented `await` এর কথা মনে করলে আমরা সেই error message explain করতে পারি যা আমরা আগে দেখেছিলাম, কিন্তু সেটা ছিলো `Unpin` এর terms এ, `Pin` এর terms এ নয়। তাহলে কিভাবে `Pin` `Unpin` এর সাথে related, এবং কেন `Future` এর `poll` call করার জন্য `self` কে `Pin` type এ রাখার প্রয়োজন?

এই chapter এর আগে থেকে মনে করুন যে একটি future এর await point এর series একটি state machine এ compile হয়, এবং compiler নিশ্চিত করে যে সেই state machine safety এর চারপাশে Rust এর normal rule follow করে, borrowing এবং ownership সহ। সেটি কাজ করানোর জন্য, Rust দেখে যে একটি await point এবং পরবর্তী await point বা async block এর শেষ এর মধ্যে কি data এর প্রয়োজন। তারপর এটি compiled state machine এ corresponding variant তৈরি করে। প্রত্যেক variant source code এর সেই section এ ব্যবহার হওয়া data এর access পায়, সেটা সেই data এর ownership নিয়ে হোক বা mutable অথবা immutable reference নিয়ে হোক।

এখন পর্যন্ত সব ঠিক আছে: যদি আমরা কোনো async block এ ownership বা reference নিয়ে কিছু ভুল করি, তাহলে borrow checker আমাদের বলবে। যখন আমরা সেই block এর সাথে correspond করা future এর চারপাশে move করতে চাই—যেমন `join_all` এ pass করার জন্য `Vec` এ move করা—তখন জিনিসগুলো আরও জটিল হয়ে যায়।

যখন আমরা একটি future move করি—`join_all` এর সাথে iterator হিসেবে ব্যবহার করার জন্য কোনো data structure এ push করার মাধ্যমে বা কোনো function থেকে return করার মাধ্যমে—তার মানে হলো Rust আমাদের জন্য তৈরি করা state machine টি move করা। এবং Rust async block এর জন্য তৈরি করা future গুলোর বেশিরভাগ type এর বিপরীতে, যেকোনো variant এর field এ নিজেদের reference এর সাথে শেষ হতে পারে, যেমন Figure 17-4 এর simplified illustration এ দেখানো হয়েছে।

<figure>

<img alt="A single-column, three-row table representing a future, fut1, which has data values 0 and 1 in the first two rows and an arrow pointing from the third row back to the second row, representing an internal reference within the future." src="img/trpl17-04.svg" class="center" />

<figcaption>Figure 17-4: A self-referential data type.</figcaption>

</figure>

তবে, default ভাবে, যেকোনো object যার নিজের reference আছে, তা move করা unsafe, কারণ reference সবসময় সেই memory address কে point করে যেখানে তারা refer করে (Figure 17-5 দেখুন)। যদি আপনি data structure টি move করেন, তাহলে সেই internal reference গুলো old location কে point করতে থেকে যাবে। তবে, সেই memory location এখন invalid। একটি হলো, যখন আপনি data structure এ change করবেন তখন এর value update করা হবে না। অন্যটি—আরও গুরুত্বপূর্ণ—হলো কম্পিউটার এখন সেই memory অন্য কাজের জন্য reuse করতে পারবে! আপনি হয়তো পরে completely unrelated data read করতে পারেন।

<figure>

<img alt="Two tables, depicting two futures, fut1 and fut2, each of which has one column and three rows, representing the result of having moved a future out of fut1 into fut2. The first, fut1, is grayed out, with a question mark in each index, representing unknown memory. The second, fut2, has 0 and 1 in the first and second rows and an arrow pointing from its third row back to the second row of fut1, representing a pointer that is referencing the old location in memory of the future before it was moved." src="img/trpl17-05.svg" class="center" />

<figcaption>Figure 17-5: The unsafe result of moving a self-referential data type</figcaption>

</figure>

Theoretically, Rust compiler প্রত্যেক object move করার সময় এর সব reference update করার চেষ্টা করতে পারত, কিন্তু এর কারণে অনেক performance overhead যোগ হতে পারত, বিশেষ করে যদি reference এর পুরো web update করার প্রয়োজন হতো। এর পরিবর্তে আমরা যদি নিশ্চিত করতে পারি যে question এ থাকা data structure টি _memory তে move হবে না_, তাহলে আমাদের কোনো reference update করার প্রয়োজন হবে না। এটাই Rust এর borrow checker require করে: safe code এ, এটি active reference থাকা কোনো item move করা থেকে prevent করে।

`Pin` আমাদের সেই exact guarantee দেওয়ার জন্য তৈরি করা হয়েছে যার আমাদের প্রয়োজন। যখন আমরা কোনো value _pin_ করি `Pin` এ সেই value এর pointer wrap করার মাধ্যমে, তখন এটি আর move করতে পারে না। তাই, যদি আপনার কাছে `Pin<Box<SomeType>>` থাকে, তাহলে আপনি আসলে `SomeType` value pin করেন, `Box` pointer _নয়_। Figure 17-6 এই process illustrate করে।

<figure>

<img alt="Three boxes laid out side by side. The first is labeled “Pin”, the second “b1”, and the third “pinned”. Within “pinned” is a table labeled “fut”, with a single column; it represents a future with cells for each part of the data structure. Its first cell has the value “0”, its second cell has an arrow coming out of it and pointing to the fourth and final cell, which has the value “1” in it, and the third cell has dashed lines and an ellipsis to indicate there may be other parts to the data structure. All together, the “fut” table represents a future which is self-referential. An arrow leaves the box labeled “Pin”, goes through the box labeled “b1” and has terminates inside the “pinned” box at the “fut” table." src="img/trpl17-06.svg" class="center" />

<figcaption>Figure 17-6: Pinning a `Box` that points to a self-referential future type.</figcaption>

</figure>

আসলে, `Box` pointer এখনও freely move করতে পারে। মনে রাখবেন: আমরা নিশ্চিত করতে চাই যে data যা ultimately referenced হচ্ছে তা যেনো same জায়গায় থাকে। যদি একটি pointer move করে, _কিন্তু এটি point করা data যদি same জায়গায় থাকে_, যেমন Figure 17-7 এ, তাহলে কোনো potential problem নেই। Independent exercise হিসেবে, type গুলোর documentation এবং `std::pin` module দেখুন এবং বের করার চেষ্টা করুন কিভাবে আপনি `Box` wrap করা `Pin` দিয়ে এটা করতে পারেন। মূল বিষয় হলো self-referential type টি move করতে পারবে না, কারণ এটি এখনও pinned।

<figure>

<img alt="Four boxes laid out in three rough columns, identical to the previous diagram with a change to the second column. Now there are two boxes in the second column, labeled “b1” and “b2”, “b1” is grayed out, and the arrow from “Pin” goes through “b2” instead of “b1”, indicating that the pointer has moved from “b1” to “b2”, but the data in “pinned” has not moved." src="img/trpl17-07.svg" class="center" />

<figcaption>Figure 17-7: Moving a `Box` which points to a self-referential future type.</figcaption>

</figure>

তবে, বেশিরভাগ type move করার জন্য perfectly safe, এমনকি যদি তারা `Pin` pointer এর পিছনে থাকেও। যখন item এর internal reference থাকে তখন আমাদের শুধু pinning নিয়ে চিন্তা করার প্রয়োজন। Primitive value যেমন number এবং Boolean safe কারণ তাদের internal reference নেই, তাই তারা obviously safe। বেশিরভাগ type যা আপনি Rust এ normaly ব্যবহার করেন সেগুলোও safe। উদাহরণস্বরূপ, আপনি কোনো চিন্তা না করেই একটি `Vec` এর চারপাশে move করতে পারেন। শুধু এতটুকু যা আমরা দেখেছি তার উপর ভিত্তি করে, যদি আপনার কাছে `Pin<Vec<String>>` থাকে, তাহলে আপনাকে `Pin` দ্বারা provide করা safe কিন্তু restrictive API গুলোর মাধ্যমে সবকিছু করতে হতো, যদিও `Vec<String>` সবসময় move করার জন্য safe যদি এটির অন্য কোনো reference না থাকে। আমাদের compiler কে বলার জন্য একটি উপায় দরকার যে এই ধরনের ক্ষেত্রে item move করা ঠিক আছে—এবং সেখানেই `Unpin` কাজে আসে।

`Unpin` হলো একটি marker trait, Chapter 16 এ দেখা `Send` এবং `Sync` trait এর মতোই, এবং তাই এর নিজের কোনো functionality নেই। Marker trait শুধুমাত্র compiler কে বলতে exist করে যে given trait implement করা type টি একটি particular context এ ব্যবহার করা safe। `Unpin` compiler কে জানায় যে given type এর কোনো guarantee uphold করার প্রয়োজন _নেই_ যে value টি safely move করা যাবে কিনা।

<!--
  The inline `<code>` in the next block is to allow the inline `<em>` inside it,
  matching what NoStarch does style-wise, and emphasizing within the text here
  that it is something distinct from a normal type.
-->

`Send` এবং `Sync` এর মতোই, compiler automatic ভাবে সব type এর জন্য `Unpin` implement করে যেখানে এটি prove করতে পারে যে safe। একটি special case, আবারও `Send` এবং `Sync` এর মতোই, যেখানে একটি type এর জন্য `Unpin` implement করা _হয় না_। এর notation হলো <code>impl !Unpin for <em>SomeType</em></code>, যেখানে
<code><em>SomeType</em></code> হলো এমন type এর নাম যার safe থাকার জন্য সেই guarantee uphold করার প্রয়োজন যখন সেই type এর pointer `Pin` এ ব্যবহার করা হয়।

অন্যভাবে বলতে গেলে, `Pin` এবং `Unpin` এর মধ্যে relationship নিয়ে মনে রাখার মতো দুটি জিনিস আছে। প্রথমত, `Unpin` হলো "normal" case, এবং `!Unpin` হলো special case। দ্বিতীয়ত, কোনো type `Unpin` implement করে নাকি `!Unpin` সেটা _শুধুমাত্র_ তখনই matter করে যখন আপনি সেই type এ pinned pointer ব্যবহার করছেন যেমন <code>Pin<&mut
<em>SomeType</em>></code>।

সেটা concrete করার জন্য, একটি `String` নিয়ে ভাবুন: এটির একটি length আছে এবং Unicode character আছে যা এটি তৈরি করে। আমরা একটি `String` কে `Pin` এ wrap করতে পারি, যেমন Figure 17-8 এ দেখানো হয়েছে। তবে, `String` automatically `Unpin` implement করে, Rust এর বেশিরভাগ type এর মতো।

<figure>

<img alt="Concurrent work flow" src="img/trpl17-08.svg" class="center" />

<figcaption>Figure 17-8: Pinning a `String`; the dotted line indicates that the `String` implements the `Unpin` trait, and thus is not pinned.</figcaption>

</figure>

ফলস্বরূপ, আমরা এমন কাজ করতে পারি যা illegal হতো যদি `String` এর পরিবর্তে `!Unpin` implement করত, যেমন memory এর exact same location এ একটি string দিয়ে অন্য string replace করা, যেমন Figure 17-9 এ। এটা `Pin` contract violate করে না, কারণ `String` এর কোনো internal reference নেই যা এটিকে move করার জন্য unsafe করে! ঠিক এই কারণেই এটি `!Unpin` এর পরিবর্তে `Unpin` implement করে।

<figure>

<img alt="Concurrent work flow" src="img/trpl17-09.svg" class="center" />

<figcaption>Figure 17-9: Replacing the `String` with an entirely different `String` in memory.</figcaption>

</figure>

এখন আমরা Listing 17-17 থেকে সেই `join_all` call এর জন্য reported error গুলো বুঝতে যথেষ্ট জানি। আমরা originally async block দ্বারা তৈরি future গুলোকে `Vec<Box<dyn Future<Output = ()>>>` এ move করার চেষ্টা করেছিলাম, কিন্তু আমরা যেমন দেখেছি, সেই future গুলোর internal reference থাকতে পারে, তাই সেগুলো `Unpin` implement করে না। সেগুলোকে pin করার প্রয়োজন, এবং তারপর আমরা `Pin` type কে `Vec` এ pass করতে পারি, confident থেকে যে future এর underlying data _move_ হবে না।

`Pin` এবং `Unpin` mostly lower-level library তৈরি করার জন্য গুরুত্বপূর্ণ, বা যখন আপনি একটি runtime build করছেন, day-to-day Rust code এর জন্য নয়। তবে, আপনি যখন এই trait গুলো error message এ দেখেন, তখন আপনার code fix করার একটি ভালো idea হবে!

> Note: `Pin` এবং `Unpin` এর এই combination Rust এ complex type এর একটি class safely implement করা সম্ভব করে তোলে, যা self-referential হওয়ার কারণে challenging prove হতো। `Pin` require করা type গুলো আজ async Rust এ সবচেয়ে বেশি common, কিন্তু মাঝে মাঝে, আপনি সেগুলো অন্য context এও দেখতে পারেন।
>
> `Pin` এবং `Unpin` কিভাবে কাজ করে তার specifics, এবং তারা যে rule uphold করার জন্য required, তা `std::pin` এর API documentation এ extensively covered করা আছে, তাই আপনি যদি আরও জানতে আগ্রহী হন, তাহলে সেটা start করার জন্য ভালো জায়গা।
>
> আপনি যদি আরও detail এ under the hood কিভাবে কাজ করে তা বুঝতে চান, তাহলে [_Asynchronous Programming in Rust_][async-book] এর Chapter [2][under-the-hood] এবং [4][pinning] দেখুন।

### The `Stream` Trait

এখন যেহেতু আপনার `Future`, `Pin`, এবং `Unpin` trait এর উপর গভীর জ্ঞান হয়েছে, তাই আমরা `Stream` trait এর দিকে মনোযোগ দিতে পারি। আপনি chapter এর শুরুতে যেমন শিখেছেন, stream হলো asynchronous iterator এর মতো। তবে, `Iterator` এবং `Future` এর বিপরীতে, এই লেখার সময় পর্যন্ত standard library তে `Stream` এর কোনো definition নেই, কিন্তু `futures` crate থেকে একটি খুব common definition আছে যা পুরো ecosystem এ ব্যবহার করা হয়।

`Stream` trait কিভাবে একসাথে merge করতে পারে তা দেখার আগে `Iterator` এবং `Future` trait এর definition review করা যাক। `Iterator` থেকে, আমাদের কাছে একটি sequence এর idea আছে: এর `next` method একটি `Option<Self::Item>` provide করে। `Future` থেকে, আমাদের কাছে সময়ের সাথে readiness এর idea আছে: এর `poll` method একটি `Poll<Self::Output>` provide করে। সময়ের সাথে ready হওয়া item এর sequence represent করার জন্য, আমরা একটি `Stream` trait define করি যা এই feature গুলোকে একসাথে করে:

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

`Stream` trait একটি associated type `Item` define করে stream দ্বারা produce হওয়া item এর type এর জন্য। এটি `Iterator` এর মতো, যেখানে zero থেকে অনেক item থাকতে পারে, এবং `Future` এর বিপরীতে, যেখানে সবসময় একটি single `Output` থাকে, এমনকি যদি সেটি unit type `()` ও হয়।

`Stream` সেই item গুলো পাওয়ার জন্য একটি method define করে। আমরা এটাকে `poll_next` call করি, এটা clear করার জন্য যে এটি `Future::poll` এর মতোই poll করে এবং `Iterator::next` এর মতোই item এর sequence produce করে। এর return type `Poll` কে `Option` এর সাথে combine করে। Outer type টি `Poll`, কারণ future এর মতো এর readiness check করার প্রয়োজন। Inner type টি `Option`, কারণ iterator এর মতো আরও message আছে কিনা তা indicate করার প্রয়োজন।

এই definition এর মতো similar কিছু Rust এর standard library এর অংশ হিসেবে শেষ হতে পারে। এই মুহূর্তে, এটি বেশিরভাগ runtime এর toolkit এর অংশ, তাই আপনি এটির উপর নির্ভর করতে পারেন, এবং আমরা এরপর যা discuss করব তা সাধারণত apply হবে!

তবে, streaming নিয়ে section এ আমরা যে উদাহরণ দেখেছিলাম, সেখানে আমরা `poll_next` _বা_ `Stream` ব্যবহার করিনি, বরং `next` এবং `StreamExt` ব্যবহার করেছিলাম। আমরা অবশ্যই নিজেদের `Stream` state machine হাতে লিখে `poll_next` API এর মাধ্যমে directly কাজ করতে পারতাম, ঠিক যেমন আমরা future এর `poll` method এর মাধ্যমে directly কাজ করতে পারতাম। তবে, `await` ব্যবহার করা অনেক বেশি nicer, এবং `StreamExt` trait `next` method supply করে যাতে আমরা ঠিক সেটাই করতে পারি:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-stream-ext/src/lib.rs:here}}
```

<!--
TODO: update this if/when tokio/etc. update their MSRV and switch to using async functions
in traits, since the lack thereof is the reason they do not yet have this.
-->

> Note: আমরা chapter এর শুরুতে ব্যবহার করা actual definition টি এটার চেয়ে সামান্য different দেখায়, কারণ এটি Rust এর সেই version গুলো support করে যেগুলো trait এ async function ব্যবহার করা support করে না। ফলস্বরূপ, এটি দেখতে এমন:
>
> ```rust,ignore
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> সেই `Next` type টি একটি `struct` যা `Future` implement করে এবং আমাদের `Next<'_, Self>` দিয়ে `self` এর reference এর lifetime name করার সুযোগ দেয়, যাতে `await` এই method এর সাথে কাজ করতে পারে।

`StreamExt` trait হলো stream এর সাথে ব্যবহার করার জন্য available সব interesting method এর home। `StreamExt` automatic ভাবে প্রত্যেক type এর জন্য implemented হয় যা `Stream` implement করে, কিন্তু এই trait গুলো separately define করা হয় যাতে community foundational trait কে affect না করে convenience API নিয়ে iterate করতে পারে।

`trpl` crate এ ব্যবহৃত `StreamExt` এর version এ, trait শুধু `next` method define করে না বরং `Stream::poll_next` call করার detail সঠিকভাবে handle করে এমন `next` এর একটি default implementation supply করে। এর মানে হলো যখন আপনার নিজের streaming data type লেখার প্রয়োজন, তখন আপনাকে _শুধু_ `Stream` implement করতে হবে, এবং তারপর যে কেউ আপনার data type ব্যবহার করে automatic ভাবে `StreamExt` এবং এর method গুলো ব্যবহার করতে পারবে।

এই trait গুলোর lower-level details এর জন্য আমরা এতটুকুই discuss করব। শেষ করার জন্য, চলুন consider করি কিভাবে future (stream সহ), task, এবং thread একসাথে fit হয়!

[ch-18]: ch18-00-oop.html
[async-book]: https://rust-lang.github.io/async-book/
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[first-async]: ch17-01-futures-and-syntax.html#our-first-async-program
[any-number-futures]: ch17-03-more-futures.html#working-with-any-number-of-futures
