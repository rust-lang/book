## Working with Any Number of Futures

যখন আমরা আগের section এ দুটি future থেকে তিনটি future এ switch করেছিলাম, তখন আমাদের `join` ব্যবহার করার পরিবর্তে `join3` ব্যবহার করতে হয়েছিল। যখনই আমরা join করতে চাই এমন future এর number change করি, তখনই ভিন্ন function call করাটা বিরক্তিকর। সৌভাগ্যবশত, আমাদের `join` এর একটি macro form আছে যেখানে আমরা arbitrary number এর argument pass করতে পারি। এটি future await করাও handle করে। তাই, আমরা Listing 17-13 এর code কে `join3` এর পরিবর্তে `join!` ব্যবহার করে rewrite করতে পারি, যেমন Listing 17-14 এ করা হয়েছে।

<Listing number="17-14" caption="একাধিক future এর জন্য wait করার জন্য `join!` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:here}}
```

</Listing>

`join` এবং `join3` এবং `join4` এর মধ্যে swap করার চেয়ে এটি অবশ্যই একটি improvement! তবে, এই macro form টিও শুধুমাত্র তখনই কাজ করে যখন আমরা future এর number আগে থেকে জানি। কিন্তু real-world Rust এ, একটি collection এ future push করা এবং তারপর তাদের কিছু বা সব future এর complete হওয়ার জন্য wait করা একটি common pattern।

কোনো collection এ সব future check করার জন্য, আমাদের সেগুলোর উপর iterate করে _সবগুলোর_ উপর join করতে হবে। `trpl::join_all` function `Iterator` trait implement করা যেকোনো type accept করে, যা আপনি Chapter 13 এর [The Iterator Trait and the `next` Method][iterator-trait]<!-- ignore --> এ শিখেছেন, তাই এটি ঠিক যেন ticket এর মতো। চলুন আমাদের future গুলোকে একটি vector এ রাখি এবং Listing 17-15 এ দেখানো হিসাবে `join!` কে `join_all` দিয়ে replace করার চেষ্টা করি।

<Listing  number="17-15" caption="একটি vector এ anonymous future store করা এবং `join_all` call করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:here}}
```

</Listing>

দুর্ভাগ্যবশত, এই code compile হয় না। এর পরিবর্তে, আমরা এই error টি পাই:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-15/
cargo build
copy just the compiler error
-->

```text
error[E0308]: mismatched types
  --> src/main.rs:45:37
   |
10 |         let tx1_fut = async move {
   |                       ---------- the expected `async` block
...
24 |         let rx_fut = async {
   |                      ----- the found `async` block
...
45 |         let futures = vec![tx1_fut, rx_fut, tx_fut];
   |                                     ^^^^^^ expected `async` block, found a 
different `async` block
   |
   = note: expected `async` block `{async block@src/main.rs:10:23: 10:33}`
              found `async` block `{async block@src/main.rs:24:22: 24:27}`
   = note: no two async blocks, even if identical, have the same type
   = help: consider pinning your async block and casting it to a trait object
```

এটি হয়তো surprising মনে হতে পারে। সব async block কোনো কিছু return করে না, তাই প্রত্যেকটি `Future<Output = ()>` তৈরি করে। মনে রাখবেন যে `Future` একটি trait, এবং compiler প্রত্যেক async block এর জন্য একটি unique enum তৈরি করে। আপনি `Vec` এ দুটি ভিন্ন হাতে লেখা struct রাখতে পারবেন না, এবং একই rule compiler দ্বারা generate হওয়া ভিন্ন enum এর জন্য apply হয়।

এটি কাজ করার জন্য, আমাদের trait object ব্যবহার করতে হবে, ঠিক যেমন Chapter 12 এর [“Returning Errors from the run function”][dyn]<!-- ignore --> এ করেছিলাম। (আমরা Chapter 18 এ trait object নিয়ে বিস্তারিত আলোচনা করব)। Trait object ব্যবহার করে আমরা এই type গুলো দ্বারা তৈরি হওয়া anonymous future গুলোর প্রত্যেকটিকে একই type হিসেবে treat করতে পারি, কারণ এদের সবাই `Future` trait implement করে।

> Note: Chapter 8 এর section [Using an Enum to Store Multiple Values][enum-alt]<!-- ignore --> এ, আমরা `Vec` এ multiple type include করার অন্য একটি উপায় নিয়ে আলোচনা করেছিলাম: vector এ appear হতে পারে এমন প্রত্যেক type কে represent করার জন্য enum ব্যবহার করা। তবে, আমরা এখানে তা করতে পারি না। প্রথমত, আমাদের কাছে ভিন্ন type গুলোর নাম দেওয়ার কোনো উপায় নেই, কারণ সেগুলো anonymous। দ্বিতীয়ত, আমরা vector এবং `join_all` এর কাছে এই জন্য পৌঁছেছিলাম যাতে আমরা future এর dynamic collection এর সাথে কাজ করতে পারি যেখানে আমরা শুধু care করি যে তাদের output type একই আছে।

আমরা Listing 17-16 এ দেখানো হিসাবে `vec!` এ প্রত্যেক future কে `Box::new` এ wrap করে শুরু করি।

<Listing number="17-16" caption="`Vec` এ future গুলোর type align করার জন্য `Box::new` ব্যবহার করা" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

দুর্ভাগ্যবশত, এই code এখনও compile হয় না। আসলে, আমরা দ্বিতীয় এবং তৃতীয় `Box::new` call এর জন্য আগের মতোই একই basic error পাই, এবং `Unpin` trait refer করে নতুন error পাই। আমরা `Unpin` error এ একটু পরেই ফিরে আসব। প্রথমে, `futures` variable এর type explicitly annotate করে `Box::new` call এর type error ঠিক করি (Listing 17-17 দেখুন)।

<Listing number="17-17" caption="Explicit type declaration ব্যবহার করে type mismatch error এর বাকি অংশ ঠিক করা" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:here}}
```

</Listing>

এই type declaration টি একটু জটিল, তাই চলুন এটা নিয়ে আলোচনা করি:

1.  সবচেয়ে ভিতরের type হলো future নিজেই। আমরা explicitly note করি যে future এর output হলো unit type `()` `Future<Output = ()>` লিখে।
2.  তারপর আমরা trait কে `dyn` দিয়ে annotate করি dynamic mark করার জন্য।
3.  পুরো trait reference টি একটি `Box` এ wrap করা।
4.  অবশেষে, আমরা explicitly declare করি যে `futures` হলো এই item গুলো ধারণ করা একটি `Vec`।

এতেই অনেক difference তৈরি হয়েছে। এখন যখন আমরা compiler run করি, তখন আমরা শুধুমাত্র `Unpin` mention করা error গুলো পাই। যদিও তাদের তিনটি আছে, তাদের content গুলো প্রায় একই।

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-16
cargo build
# copy *only* the errors
# fix the paths
-->

```text
error[E0308]: mismatched types
   --> src/main.rs:46:46
    |
10  |         let tx1_fut = async move {
    |                       ---------- the expected `async` block
...
24  |         let rx_fut = async {
    |                      ----- the found `async` block
...
46  |             vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
    |                                     -------- ^^^^^^ expected `async` block, found a different `async` block
    |                                     |
    |                                     arguments to this function are incorrect
    |
    = note: expected `async` block `{async block@src/main.rs:10:23: 10:33}`
               found `async` block `{async block@src/main.rs:24:22: 24:27}`
    = note: no two async blocks, even if identical, have the same type
    = help: consider pinning your async block and casting it to a trait object
note: associated function defined here
   --> file:///home/.rustup/toolchains/1.82/lib/rustlib/src/rust/library/alloc/src/boxed.rs:255:12
    |
255 |     pub fn new(x: T) -> Self {
    |            ^^^

error[E0308]: mismatched types
   --> src/main.rs:46:64
    |
10  |         let tx1_fut = async move {
    |                       ---------- the expected `async` block
...
30  |         let tx_fut = async move {
    |                      ---------- the found `async` block
...
46  |             vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
    |                                                       -------- ^^^^^^ expected `async` block, found a different `async` block
    |                                                       |
    |                                                       arguments to this function are incorrect
    |
    = note: expected `async` block `{async block@src/main.rs:10:23: 10:33}`
               found `async` block `{async block@src/main.rs:30:22: 30:32}`
    = note: no two async blocks, even if identical, have the same type
    = help: consider pinning your async block and casting it to a trait object
note: associated function defined here
   --> file:///home/.rustup/toolchains/1.82/lib/rustlib/src/rust/library/alloc/src/boxed.rs:255:12
    |
255 |     pub fn new(x: T) -> Self {
    |            ^^^

error[E0277]: `{async block@src/main.rs:10:23: 10:33}` cannot be unpinned
   --> src/main.rs:48:24
    |
48  |         trpl::join_all(futures).await;
    |         -------------- ^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`, which is required by `Box<{async block@src/main.rs:10:23: 10:33}>: Future`
    |         |
    |         required by a bound introduced by this call
    |
    = note: consider using the `pin!` macro
            consider using `Box::pin` if you need to access the pinned value outside of the current scope
    = note: required for `Box<{async block@src/main.rs:10:23: 10:33}>` to implement `Future`
note: required by a bound in `join_all`
   --> file:///home/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:105:14
    |
102 | pub fn join_all<I>(iter: I) -> JoinAll<I::Item>
    |        -------- required by a bound in this function
...
105 |     I::Item: Future,
    |              ^^^^^^ required by this bound in `join_all`

error[E0277]: `{async block@src/main.rs:10:23: 10:33}` cannot be unpinned
  --> src/main.rs:48:9
   |
48 |         trpl::join_all(futures).await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`, which is required by `Box<{async block@src/main.rs:10:23: 10:33}>: Future`
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
   |        ^^^^^^ required by this bound in `JoinAll`

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
   |        ^^^^^^ required by this bound in `JoinAll`
```

এটা digest করার মতো _অনেক_ কিছু, তাই চলুন এটা আলাদা করি। message এর প্রথম অংশ আমাদের জানায় যে প্রথম async block (`src/main.rs:8:23: 20:10`) `Unpin` trait implement করে না এবং এটি resolve করার জন্য `pin!` বা `Box::pin` ব্যবহার করার পরামর্শ দেয়। Chapter এর পরে, আমরা `Pin` এবং `Unpin` নিয়ে আরও কিছু detail এ আলোচনা করব। আপাতত, আমরা শুধু compiler এর advice follow করে stuck হওয়া থেকে বাঁচতে পারি। Listing 17-18 এ, আমরা `futures` এর type annotation update করে শুরু করি, প্রত্যেক `Box` কে `Pin` দিয়ে wrap করে। দ্বিতীয়ত, আমরা future গুলোকে pin করার জন্য `Box::pin` ব্যবহার করি।

<Listing number="17-18" caption="`Vec` এর type check করানোর জন্য `Pin` এবং `Box::pin` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

যদি আমরা compile এবং run করি, তাহলে আমরা অবশেষে সেই output পাই যা আমরা আশা করেছিলাম:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
received 'hi'
received 'more'
received 'from'
received 'messages'
received 'the'
received 'for'
received 'future'
received 'you'
```

Phew!

এখানে explore করার মতো আরও কিছু বিষয় আছে। একটি হলো, `Pin<Box<T>>` ব্যবহার করে `Box` দিয়ে heap এ future গুলো রাখার জন্য অল্প overhead যোগ করে—এবং আমরা শুধু type গুলোকে align করার জন্য এটা করছি। আসলে আমাদের heap allocation এর দরকার নেই: এই future গুলো এই particular function এর local। আগে উল্লেখ করা হয়েছে, `Pin` নিজেই একটি wrapper type, তাই আমরা `Vec` এ single type রাখার সুবিধা পেতে পারি—`Box` ব্যবহার করার original reason—heap allocation না করে। আমরা `std::pin::pin` macro ব্যবহার করে প্রত্যেক future এর সাথে সরাসরি `Pin` ব্যবহার করতে পারি।

তবে, আমাদের pinned reference এর type explicitly বলতে হবে; অন্যথায়, Rust এখনও জানবে না যে এগুলোকে dynamic trait object হিসেবে interpret করতে হবে, যা আমাদের `Vec` এ প্রয়োজন। তাই আমরা `pin!` macro ব্যবহার করে প্রত্যেক future pin করি, এবং `futures` কে dynamic future type এ pinned mutable reference ধারণ করা `Vec` হিসেবে define করি, যা Listing 17-19 এ দেখানো হয়েছে।

<Listing number="17-19" caption="অপ্রয়োজনীয় heap allocation avoid করার জন্য `pin!` macro দিয়ে সরাসরি `Pin` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:here}}
```

</Listing>

আমরা এতদূর পর্যন্ত এসেছি এই fact ignore করে যে আমাদের ভিন্ন `Output` type থাকতে পারে। উদাহরণস্বরূপ, Listing 17-20 এ, `a` এর anonymous future `Future<Output = u32>` implement করে, `b` এর anonymous future `Future<Output = &str>` implement করে, এবং `c` এর anonymous future `Future<Output = bool>` implement করে।

<Listing number="17-20" caption="ভিন্ন type এর তিনটি future" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:here}}
```

</Listing>

আমরা `trpl::join!` ব্যবহার করে await করতে পারি, কারণ এটি আমাদের multiple future type pass করতে দেয় এবং সেই type গুলোর tuple তৈরি করে। আমরা `trpl::join_all` ব্যবহার _করতে পারি না_, কারণ এর জন্য pass করা সব future এর type একই হতে হয়। মনে রাখবেন, সেই error এর কারণে `Pin` এর সাথে এই adventure এ আমাদের শুরু হয়েছিল!

এটি একটি fundamental tradeoff: আমরা হয় `join_all` দিয়ে dynamic number এর future handle করতে পারি, যতক্ষণ না তাদের সবগুলোর type একই থাকে, অথবা আমরা `join` function বা `join!` macro দিয়ে set number এর future handle করতে পারি, এমনকি যদি তাদের ভিন্ন type ও থাকে। এটি same scenario যা আমরা Rust এ অন্য যেকোনো type এর সাথে কাজ করার সময় face করি। Future বিশেষ কিছু নয়, যদিও আমাদের সাথে কাজ করার জন্য কিছু ভালো syntax আছে, এবং এটা ভালো জিনিস।

### Racing Futures

যখন আমরা `join` family এর function এবং macro দিয়ে future "join" করি, তখন আমাদের move on করার আগে তাদের _সবগুলোকে_ শেষ করার প্রয়োজন। তবে, মাঝে মাঝে, আমাদের move on করার আগে set থেকে _কিছু_ future এর finish হওয়ার প্রয়োজন—এক future এর সাথে অন্য future race করানোর মতো।

Listing 17-21 এ, আমরা আবারও `trpl::race` ব্যবহার করি দুটি future, `slow` এবং `fast` কে একে অপরের সাথে race করানোর জন্য।

<Listing number="17-21" caption="যে future আগে শেষ হয় তার result পাওয়ার জন্য `race` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:here}}
```

</Listing>

প্রত্যেক future run করা শুরু করার সময় একটি message print করে, `sleep` call করে এবং await করে কিছু সময়ের জন্য pause নেয়, এবং তারপর শেষ হওয়ার সময় অন্য message print করে। তারপর আমরা `slow` এবং `fast` দুটোকেই `trpl::race` এ pass করি এবং তাদের একটি শেষ হওয়ার জন্য wait করি। (এখানের outcome খুব বেশি surprising নয়: `fast` win করে)। [“Our First Async Program”][async-program]<!-- ignore --> এ যখন আমরা `race` ব্যবহার করেছিলাম, তখন আমরা এখানে return করা `Either` instance কে ignore করি, কারণ async block এর body তে interesting behaviour গুলো ঘটে।

লক্ষ্য করুন যে আপনি যদি `race` এর argument এর order flip করেন, তাহলে "started" message এর order change হয়ে যায়, যদিও `fast` future সবসময় আগে complete হয়। কারণ হলো এই particular `race` function এর implementation fair নয়। এটি সবসময় argument হিসেবে pass করা future গুলোকে সেই order এ run করে যে order এ pass করা হয়েছে। অন্য implementation গুলো fair এবং random ভাবে choose করবে কোন future কে আগে poll করবে। তবে আমরা যে race implementation ব্যবহার করি সেটা fair হোক বা না হোক, _একটি_ future তার body তে first `await` পর্যন্ত run করবে তারপর অন্য task শুরু হতে পারবে।

[Our First Async Program][async-program]<!-- ignore --> থেকে মনে করুন যে প্রত্যেক await point এ, Rust runtime কে task pause করার এবং অন্য task এ switch করার সুযোগ দেয় যদি await করা future ready না থাকে। এর inverse ও সত্য: Rust _শুধুমাত্র_ await point এ async block pause করে এবং runtime এ control back করে। Await point এর মধ্যে সবকিছু synchronous।

এর মানে হলো আপনি যদি await point ছাড়া async block এ অনেক কাজ করেন, তাহলে সেই future অন্য future কে progress করা থেকে block করবে। আপনি মাঝে মাঝে শুনতে পারেন যে একটি future অন্য future কে _starve_ করাচ্ছে। কিছু ক্ষেত্রে, এটি খুব বেশি problem তৈরি নাও করতে পারে। তবে, যদি আপনি expensive setup বা long-running কোনো কাজ করেন, বা যদি আপনার এমন কোনো future থাকে যা অনির্দিষ্টকালের জন্য কোনো particular task করতে থাকবে, তাহলে কখন এবং কোথায় runtime এ control back করতে হবে তা নিয়ে চিন্তা করতে হবে।

একইভাবে, যদি আপনার long-running blocking operation থাকে, তাহলে async program এর বিভিন্ন অংশকে একে অপরের সাথে relate করার উপায় provide করার জন্য একটি useful tool হতে পারে।

কিন্তু এই ক্ষেত্রে আপনি _কিভাবে_ runtime এ control back করবেন?

<!-- Old headings. Do not remove or links may break. -->

<a id="yielding"></a>

### Yielding Control to the Runtime

চলুন একটি long-running operation simulate করি। Listing 17-22 একটি `slow` function introduce করে।

<Listing number="17-22" caption="Slow operation simulate করার জন্য `thread::sleep` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:slow}}
```

</Listing>

এই code `trpl::sleep` এর পরিবর্তে `std::thread::sleep` ব্যবহার করে যাতে `slow` call করলে কিছু milliseconds এর জন্য current thread block হয়ে যায়। আমরা `slow` কে এমন real-world operation represent করার জন্য ব্যবহার করতে পারি যা long-running এবং blocking দুটোই।

Listing 17-23 এ, আমরা একজোড়া future এ এই ধরনের CPU-bound কাজ emulate করার জন্য `slow` ব্যবহার করি।

<Listing number="17-23" caption="Slow operation simulate করার জন্য `thread::sleep` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:slow-futures}}
```

</Listing>

শুরুতে, প্রত্যেক future slow operation এর bunch perform করার _পরে_ control runtime এ back করে। আপনি যদি এই code run করেন, তাহলে আপনি এই output দেখতে পাবেন:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-23/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

আমাদের আগের উদাহরণটির মতো, `race` ও `a` এর কাজ শেষ হওয়ার সাথে সাথেই শেষ হয়ে যায়। তবে, দুটি future এর মধ্যে কোনো interleaving নেই। `a` future `trpl::sleep` call await না হওয়া পর্যন্ত তার সব কাজ করে, তারপর `b` future তার নিজের `trpl::sleep` call await না হওয়া পর্যন্ত তার সব কাজ করে, এবং সবশেষে `a` future complete হয়। দুটি future কে slow task এর মাঝে progress করার সুযোগ দেওয়ার জন্য, আমাদের await point দরকার যাতে আমরা control runtime এ back করতে পারি। তার মানে আমাদের এমন কিছুর দরকার যা আমরা await করতে পারি!

আমরা Listing 17-23 এ এই ধরনের handoff ঘটতে দেখতে পারি: যদি আমরা `a` future এর শেষে `trpl::sleep` remove করি, তাহলে এটি `b` future _একদমই_ run না করে complete হয়ে যেত। চলুন `sleep` function কে operation switch off করার জন্য starting point হিসেবে ব্যবহার করার চেষ্টা করি, যা Listing 17-24 এ দেখানো হয়েছে।

<Listing number="17-24" caption="Operation switch off করার জন্য progress তৈরি করার সুযোগ দেওয়ার জন্য `sleep` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

Listing 17-24 এ, আমরা `slow` এর প্রত্যেক call এর মধ্যে await point দিয়ে `trpl::sleep` call যোগ করি। এখন দুটি future এর কাজ interleaved:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-24
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

`a` future `slow` call করার পর `trpl::sleep` call করার আগে কিছুক্ষণের জন্য run হয়, কিন্তু তারপর যখনই একটি await point hit করে, তখনই future গুলো swap হয়। এই ক্ষেত্রে, আমরা `slow` এর প্রত্যেক call এর পর এটা করেছি, কিন্তু আমরা আমাদের জন্য সবচেয়ে বেশি কাজের মনে হয় এমন যেকোনো উপায়ে কাজ break করতে পারি।

তবে, আমরা এখানে আসলে _sleep_ করতে চাই না: আমরা যত দ্রুত সম্ভব progress করতে চাই। আমাদের শুধু runtime এ control back করার প্রয়োজন। আমরা `yield_now` function ব্যবহার করে সরাসরি তা করতে পারি। Listing 17-25 এ, আমরা সব `sleep` call কে `yield_now` দিয়ে replace করি।

<Listing number="17-25" caption="Operation switch off করার জন্য progress তৈরি করার সুযোগ দেওয়ার জন্য `yield_now` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:yields}}
```

</Listing>

এই code টি actual intent এর ব্যাপারে clear এবং `sleep` ব্যবহারের চেয়ে significantly fast হতে পারে, কারণ `sleep` দ্বারা ব্যবহৃত timer গুলোর মতো timer এর granular limit থাকতে পারে। উদাহরণস্বরূপ, আমরা যে `sleep` version ব্যবহার করছি, সেটি সবসময় কমপক্ষে এক millisecond এর জন্য sleep করবে, এমনকি যদি আমরা এটিকে এক nanosecond এর `Duration` ও pass করি। আবারও, আধুনিক কম্পিউটার _fast_: তারা এক millisecond এ অনেক কাজ করতে পারে!

আপনি নিজে একটি ছোট benchmark set up করে তা দেখতে পারেন, যেমন Listing 17-26 এ দেখানো হয়েছে। (Performance test করার জন্য এটি বিশেষ rigorous উপায় নয়, তবে এখানে difference দেখানোর জন্য যথেষ্ট।)

<Listing number="17-26" caption="`sleep` এবং `yield_now` এর performance compare করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-26/src/main.rs:here}}
```

</Listing>

এখানে, আমরা সব status printing skip করি, `trpl::sleep` এ এক nanosecond `Duration` pass করি, এবং প্রত্যেক future কে নিজে run করতে দেই, future গুলোর মধ্যে কোনো switching ছাড়া। তারপর আমরা 1,000 iteration এর জন্য run করি এবং দেখি `trpl::sleep` ব্যবহার করা future এর `trpl::yield_now` ব্যবহার করা future এর তুলনায় কত সময় লাগে।

`yield_now` ব্যবহার করা version _অনেক_ বেশি fast!

এর মানে হলো async compute-bound task এর জন্যও useful হতে পারে, আপনার program আর কি কি করছে তার উপর নির্ভর করে, কারণ এটি program এর বিভিন্ন অংশের মধ্যে relationship structure করার জন্য একটি useful tool provide করে। এটি _cooperative multitasking_ এর একটি form, যেখানে প্রত্যেক future এর await point এর মাধ্যমে control handover করার সময় determine করার power থাকে। তাই প্রত্যেক future এর ও দায়িত্ব আছে খুব বেশি সময় block করা avoid করার। Rust-based embedded operating system এ, এটিই হলো multitasking এর _একমাত্র_ উপায়!

Real-world code এ, আপনি সাধারণত প্রত্যেক single line এ await point এর সাথে function call alternate করবেন না। যদিও এভাবে control yield করা relatively inexpensive, এটি free নয়। বেশিরভাগ ক্ষেত্রে, compute-bound task break করার চেষ্টা করলে তা significantly slow হয়ে যেতে পারে, তাই মাঝে মাঝে কোনো operation কে সংক্ষেপে block করতে দেওয়া _overall_ performance এর জন্য ভালো। আপনার code এর actual performance bottleneck কি তা দেখার জন্য সবসময় measure করুন। Underlying dynamic মনে রাখা গুরুত্বপূর্ণ, যদি আপনি serial এ অনেক কাজ হতে দেখেন যা আপনি concurrently হওয়ার আশা করেছিলেন!

### Building Our Own Async Abstractions

আমরা নতুন pattern তৈরি করার জন্য future গুলোকে একসাথে compose ও করতে পারি। উদাহরণস্বরূপ, আমরা async building block দিয়ে `timeout` function তৈরি করতে পারি যা আমাদের কাছে আগে থেকেই আছে। যখন আমরা শেষ করব, result এমন একটি building block হবে যা ব্যবহার করে আমরা আরও async abstraction তৈরি করতে পারব।

Listing 17-27 দেখায় কিভাবে আমরা এই `timeout` slow future এর সাথে কাজ করবে বলে আশা করি।

<Listing number="17-27" caption="Time limit দিয়ে slow operation run করার জন্য আমাদের imaginary `timeout` ব্যবহার করা" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-27/src/main.rs:here}}
```

</Listing>

চলুন এটি implement করি! শুরু করার জন্য, `timeout` এর API নিয়ে ভাবি:

-   আমাদের await করার জন্য এটি নিজে একটি async function হতে হবে।
-   এর প্রথম parameter run করার জন্য একটি future হওয়া উচিত। আমরা এটিকে generic করতে পারি যাতে এটি যেকোনো future এর সাথে কাজ করতে পারে।
-   এর দ্বিতীয় parameter wait করার maximum time হবে। যদি আমরা `Duration` ব্যবহার করি, তাহলে `trpl::sleep` এ pass করা সহজ হবে।
-   এটির `Result` return করা উচিত। যদি future successfully complete হয়, তাহলে `Result` টি future দ্বারা তৈরি value দিয়ে `Ok` হবে। যদি timeout আগে elapsed হয়ে যায়, তাহলে `Result` টি timeout যত duration এর জন্য wait করেছে সেই duration দিয়ে `Err` হবে।

Listing 17-28 এই declaration দেখায়।

<!-- This is not tested because it intentionally does not compile. -->

<Listing number="17-28" caption="`timeout` এর signature define করা" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-28/src/main.rs:declaration}}
```

</Listing>

এটি type এর জন্য আমাদের goal satisfy করে। এখন চলুন আমাদের প্রয়োজনীয় _behaviour_ নিয়ে ভাবি: আমরা pass করা future কে duration এর সাথে race করাতে চাই। আমরা duration থেকে timer future তৈরি করার জন্য `trpl::sleep` ব্যবহার করতে পারি, এবং caller pass করা future এর সাথে সেই timer run করার জন্য `trpl::race` ব্যবহার করতে পারি।

আমরা জানি যে `race` fair নয়, argument pass করার order এ poll করে। তাই, আমরা `future_to_try` কে `race` এ প্রথমে pass করি যাতে `max_time` যদি খুব short duration ও হয় তবুও এটির complete হওয়ার সুযোগ থাকে। যদি `future_to_try` আগে শেষ হয়, তাহলে `race` `future_to_try` থেকে output দিয়ে `Left` return করবে। যদি `timer` আগে শেষ হয়, তাহলে `race` timer এর `()` output দিয়ে `Right` return করবে।

Listing 17-29 এ, আমরা `trpl::race` await করার result এর উপর match করি।

<Listing number="17-29" caption="`race` এবং `sleep` দিয়ে `timeout` define করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-29/src/main.rs:implementation}}
```

</Listing>

যদি `future_to_try` succeed হয় এবং আমরা `Left(output)` পাই, তাহলে আমরা `Ok(output)` return করি। যদি এর পরিবর্তে sleep timer elapsed হয় এবং আমরা `Right(())` পাই, তাহলে আমরা `()` কে `_` দিয়ে ignore করি এবং এর পরিবর্তে `Err(max_time)` return করি।

এর মাধ্যমে, আমরা দুটি async helper দিয়ে তৈরি একটি working `timeout` পেলাম। যদি আমরা আমাদের code run করি, তাহলে এটি timeout এর পরে failure mode print করবে:

```text
Failed after 2 seconds
```

যেহেতু future অন্য future এর সাথে compose হয়, তাই আপনি ছোট async building block ব্যবহার করে powerful tool তৈরি করতে পারেন। উদাহরণস্বরূপ, আপনি retry এর সাথে timeout combine করার জন্য একই approach ব্যবহার করতে পারেন, এবং সেগুলো network call এর মতো operation এর সাথে ব্যবহার করতে পারেন (chapter এর শুরু থেকে একটি উদাহরণ)।

বাস্তবে, আপনি সাধারণত সরাসরি `async` এবং `await` এর সাথে কাজ করবেন, এবং দ্বিতীয়ত `join`, `join_all`, `race`, ইত্যাদির মতো function এবং macro এর সাথে কাজ করবেন। এই API গুলোর সাথে future ব্যবহার করার জন্য মাঝে মাঝে আপনার `pin` এর কাছে reach করার প্রয়োজন হবে।

আমরা এখন একই সময়ে multiple future এর সাথে কাজ করার অনেক উপায় দেখেছি। এরপর, আমরা দেখব কিভাবে আমরা _stream_ ব্যবহার করে সময়ের সাথে multiple future এর sequence এ কাজ করতে পারি। এখানে আরও কিছু জিনিস আছে যা আপনি প্রথমে consider করতে চাইতে পারেন:

-   আমরা কোনো group এর সব future এর finish হওয়ার জন্য wait করার জন্য `join_all` এর সাথে `Vec` ব্যবহার করেছিলাম। কিভাবে আপনি sequence এ future এর group process করার জন্য `Vec` ব্যবহার করতে পারেন? সেটা করার Tradeoff কি?

-   `futures` crate থেকে `futures::stream::FuturesUnordered` type টি দেখুন। এটিকে ব্যবহার করা `Vec` ব্যবহার করার থেকে কিভাবে different হবে? (Crate এর `stream` part থেকে এসেছে এই fact নিয়ে চিন্তা করবেন না; এটি যেকোনো future এর collection এর সাথে ঠিকঠাক কাজ করে)।

[dyn]: ch12-03-improving-error-handling-and-modularity.html
[enum-alt]: ch12-03-improving-error-handling-and-modularity.html#returning-errors-from-the-run-function
[async-program]: ch17-01-futures-and-syntax.html#our-first-async-program
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
