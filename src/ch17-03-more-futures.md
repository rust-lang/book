## যেকোনো সংখ্যক ফিউচারের সাথে কাজ করা (Working with Any Number of Futures)

পূর্ববর্তী বিভাগে যখন আমরা দুটি ফিউচার থেকে তিনটি ফিউচার ব্যবহার করা শুরু করি, তখন আমাদের `join` ব্যবহার করা থেকে `join3` ব্যবহার করাতে পরিবর্তন করতে হয়েছিল। আমরা যতগুলি ফিউচার join করতে চাই তার সংখ্যা পরিবর্তন করার সময় প্রতিবার একটি ভিন্ন ফাংশন কল করা বিরক্তিকর হবে। আনন্দের বিষয়, আমাদের কাছে `join`-এর একটি ম্যাক্রো ফর্ম রয়েছে যাতে আমরা ইচ্ছামতো সংখ্যক আর্গুমেন্ট পাস করতে পারি। এটি নিজে থেকেই ফিউচারগুলির জন্য অপেক্ষা করার কাজটিও পরিচালনা করে। সুতরাং, আমরা Listing 17-13-এর কোডটিকে `join3`-এর পরিবর্তে `join!` ব্যবহার করে পুনরায় লিখতে পারি, যেমনটি Listing 17-14-তে রয়েছে।

<Listing number="17-14" caption="একাধিক ফিউচারের জন্য অপেক্ষা করতে `join!` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:here}}
```

</Listing>

এটি অবশ্যই `join` এবং `join3` এবং `join4` ইত্যাদির মধ্যে অদলবদল করার চেয়ে একটি উন্নতি! যাইহোক, এমনকি এই ম্যাক্রো ফর্মটিও তখনই কাজ করে যখন আমরা আগে থেকে ফিউচারের সংখ্যা জানি। বাস্তব-বিশ্বের Rust-এ, যদিও, ফিউচারগুলিকে একটি কালেকশনে পুশ করা এবং তারপরে তাদের মধ্যে কিছু বা সমস্ত ফিউচার সম্পূর্ণ হওয়ার জন্য অপেক্ষা করা একটি সাধারণ প্যাটার্ন।

কিছু কালেকশনের সমস্ত ফিউচার পরীক্ষা করার জন্য, আমাদের সেগুলির _সমস্ত_-এর উপর ইটারেট করতে হবে এবং join করতে হবে। `trpl::join_all` ফাংশনটি যেকোনো টাইপ গ্রহণ করে যা `Iterator` trait ইমপ্লিমেন্ট করে, যেটি আপনি [The Iterator Trait and the `next` Method][iterator-trait]<!-- ignore --> Chapter 13-এ শিখেছেন, তাই মনে হচ্ছে এটিই উপযুক্ত। আসুন আমাদের ফিউচারগুলিকে একটি ভেক্টরে রাখি এবং Listing 17-15-এ দেখানো মতো `join!`-কে `join_all` দিয়ে প্রতিস্থাপন করি।

<Listing  number="17-15" caption="একটি ভেক্টরে বেনামী ফিউচার সংরক্ষণ করা এবং `join_all` কল করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:here}}
```

</Listing>

দুর্ভাগ্যবশত, এই কোডটি কম্পাইল হয় না। পরিবর্তে, আমরা এই error টি পাই:

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
   |                                     ^^^^^^ expected `async` block, found a different `async` block
   |
   = note: expected `async` block `{async block@src/main.rs:10:23: 10:33}`
              found `async` block `{async block@src/main.rs:24:22: 24:27}`
   = note: no two async blocks, even if identical, have the same type
   = help: consider pinning your async block and casting it to a trait object
```

এটি বিস্ময়কর হতে পারে। সর্বোপরি, অ্যাসিঙ্ক্রোনাস ব্লকগুলির কোনওটিই কিছু রিটার্ন করে না, তাই প্রতিটি একটি `Future<Output = ()>` তৈরি করে। মনে রাখবেন যে `Future` হল একটি trait, এবং কম্পাইলার প্রতিটি অ্যাসিঙ্ক্রোনাস ব্লকের জন্য একটি অনন্য enum তৈরি করে। আপনি একটি `Vec`-এ দুটি ভিন্ন হাতে লেখা স্ট্রাক্ট রাখতে পারবেন না এবং কম্পাইলার দ্বারা তৈরি করা বিভিন্ন enums-এর ক্ষেত্রেও একই নিয়ম প্রযোজ্য।

এটি কাজ করার জন্য, আমাদের _trait অবজেক্ট_ ব্যবহার করতে হবে, ঠিক যেমনটি আমরা Chapter 12-এর [“Returning Errors from the run function”][dyn]<!-- ignore -->-এ করেছি। (আমরা Chapter 18-এ trait অবজেক্টগুলি বিস্তারিতভাবে কভার করব।) trait অবজেক্ট ব্যবহার করা আমাদের এই টাইপগুলি দ্বারা উৎপাদিত প্রতিটি বেনামী ফিউচারকে একই টাইপ হিসাবে বিবেচনা করতে দেয়, কারণ সেগুলি সবই `Future` trait ইমপ্লিমেন্ট করে।

> Note: Chapter 8-এর [Using an Enum to Store Multiple Values][enum-alt]<!-- ignore --> বিভাগে, আমরা একটি `Vec`-এ একাধিক টাইপ অন্তর্ভুক্ত করার আরেকটি উপায় নিয়ে আলোচনা করেছি: ভেক্টরে প্রদর্শিত হতে পারে এমন প্রতিটি টাইপকে উপস্থাপন করার জন্য একটি enum ব্যবহার করা। আমরা এখানে তা করতে পারি না। একটি কারণ হল, আমাদের কাছে বিভিন্ন টাইপের নাম দেওয়ার কোনও উপায় নেই, কারণ সেগুলি বেনামী। অন্যটির জন্য, আমরা যে কারণে একটি ভেক্টর এবং `join_all`-এর কাছে পৌঁছেছি তা হল ফিউচারের একটি ডায়নামিক কালেকশনের সাথে কাজ করতে সক্ষম হওয়া যেখানে আমরা শুধুমাত্র তাদের একই আউটপুট টাইপ আছে কিনা তা নিয়ে চিন্তা করি।

আমরা Listing 17-16-এ দেখানো মতো প্রতিটি ফিউচারকে `vec!`-এর মধ্যে `Box::new`-এ র‍্যাপ করে শুরু করি।

<Listing number="17-16" caption="একটি `Vec`-এ ফিউচারের টাইপগুলিকে সারিবদ্ধ করতে `Box::new` ব্যবহার করা" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

দুর্ভাগ্যবশত, এই কোডটি এখনও কম্পাইল হয় না। আসলে, আমরা দ্বিতীয় এবং তৃতীয় `Box::new` কলের জন্য আগেও একই বেসিক error পেয়েছি, পাশাপাশি `Unpin` trait উল্লেখ করে নতুন error-ও পেয়েছি। আমরা একটু পরেই `Unpin` error-এ ফিরে আসব। প্রথমে, আসুন `futures` variable-এর টাইপটি স্পষ্টভাবে annotate করে `Box::new` কলের টাইপ error গুলি ঠিক করি (Listing 17-17 দেখুন)।

<Listing number="17-17" caption="একটি explicit টাইপ ডিক্লারেশন ব্যবহার করে বাকি টাইপ মিসম্যাচ error গুলি ঠিক করা" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:here}}
```

</Listing>

এই টাইপ ডিক্লারেশনটি একটু জটিল, তাই আসুন এটি নিয়ে আলোচনা করি:

1.  ভেতরের টাইপটি হল ফিউচার নিজেই। আমরা স্পষ্টভাবে উল্লেখ করি যে ফিউচারের আউটপুট হল ইউনিট টাইপ `()`, `Future<Output = ()>` লিখে।
2.  তারপর আমরা এটিকে ডায়নামিক হিসাবে চিহ্নিত করতে `dyn` দিয়ে trait টিকে annotate করি।
3.  সম্পূর্ণ trait রেফারেন্সটি একটি `Box`-এর মধ্যে র‍্যাপ করা হয়েছে।
4.  অবশেষে, আমরা স্পষ্টভাবে বলি যে `futures` হল এই আইটেমগুলি ধারণকারী একটি `Vec`।

এটি ইতিমধ্যেই একটি বড় পার্থক্য তৈরি করেছে। এখন যখন আমরা কম্পাইলার চালাই, তখন আমরা শুধুমাত্র `Unpin` উল্লেখ করে error গুলি পাই। যদিও তাদের মধ্যে তিনটি রয়েছে, তাদের বিষয়বস্তু খুব একই রকম।

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
   --> file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/boxed.rs:252:12
    |
252 |     pub fn new(x: T) -> Self {
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
   --> file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/boxed.rs:252:12
    |
252 |     pub fn new(x: T) -> Self {
    |            ^^^

error[E0277]: `{async block@src/main.rs:10:23: 10:33}` cannot be unpinned
   --> src/main.rs:48:24
    |
48  |         trpl::join_all(futures).await;
    |         -------------- ^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`
    |         |
    |         required by a bound introduced by this call
    |
    = note: consider using the `pin!` macro
            consider using `Box::pin` if you need to access the pinned value outside of the current scope
    = note: required for `Box<{async block@src/main.rs:10:23: 10:33}>` to implement `Future`
note: required by a bound in `join_all`
   --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:105:14
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
   |         ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`
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

এটি _অনেক_ কিছু, তাই আসুন এটিকে আলাদা করি। মেসেজের প্রথম অংশটি আমাদের বলে যে প্রথম অ্যাসিঙ্ক্রোনাস ব্লক (`src/main.rs:8:23: 20:10`) `Unpin` trait ইমপ্লিমেন্ট করে না এবং এটি সমাধান করার জন্য `pin!` বা `Box::pin` ব্যবহার করার পরামর্শ দেয়। চ্যাপ্টারের পরে, আমরা `Pin` এবং `Unpin` সম্পর্কে আরও কয়েকটি বিশদ বিবরণে যাব। আপাতত, যদিও, আমরা আটকে যাওয়া থেকে বাঁচতে কম্পাইলারের পরামর্শ অনুসরণ করতে পারি। Listing 17-18-এ, আমরা প্রতিটি `Box`-এর জন্য `Pin` র‍্যাপ করে `futures`-এর জন্য টাইপ অ্যানোটেশন আপডেট করে শুরু করি। দ্বিতীয়ত, আমরা ফিউচারগুলিকে নিজেরাই পিন করতে `Box::pin` ব্যবহার করি।

<Listing number="17-18" caption="`Vec` টাইপ চেক করতে `Pin` এবং `Box::pin` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

যদি আমরা এটি কম্পাইল এবং রান করি, তাহলে আমরা অবশেষে সেই আউটপুটটি পাব যার জন্য আমরা আশা করেছিলাম:

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

উফ!

এখানে আরও কিছু বিষয় অন্বেষণ করার আছে। একটির জন্য, `Pin<Box<T>>` ব্যবহার করা `Box`-এর সাথে হিপে এই ফিউচারগুলি রাখার কারণে অল্প পরিমাণে ওভারহেড যুক্ত করে—এবং আমরা এটি শুধুমাত্র টাইপগুলিকে সারিবদ্ধ করার জন্য করছি। আমাদের আসলে হিপ অ্যালোকেশনের _প্রয়োজন_ নেই: এই ফিউচারগুলি এই বিশেষ ফাংশনের জন্য লোকাল। যেমনটি আগে উল্লেখ করা হয়েছে, `Pin` নিজেই একটি র‍্যাপার টাইপ, তাই আমরা `Vec`-এ একটি একক টাইপ থাকার সুবিধা পেতে পারি—যে মূল কারণে আমরা `Box`-এর কাছে পৌঁছেছিলাম—হিপ অ্যালোকেশন না করেই। আমরা প্রতিটি ফিউচারের সাথে সরাসরি `Pin` ব্যবহার করতে পারি, `std::pin::pin` ম্যাক্রো ব্যবহার করে।

যাইহোক, আমাদের এখনও পিন করা রেফারেন্সের টাইপ সম্পর্কে স্পষ্ট হতে হবে; অন্যথায়, Rust এখনও জানবে না যে এগুলিকে ডায়নামিক trait অবজেক্ট হিসাবে ব্যাখ্যা করতে হবে, যা আমাদের `Vec`-এ তাদের হতে হবে। তাই আমরা প্রতিটি ফিউচারকে সংজ্ঞায়িত করার সময় `pin!` করি এবং `futures`-কে ডায়নামিক ফিউচার টাইপের পিন করা মিউটেবল রেফারেন্স ধারণকারী একটি `Vec` হিসাবে সংজ্ঞায়িত করি, যেমনটি Listing 17-19-এ রয়েছে।

<Listing number="17-19" caption="অপ্রয়োজনীয় হিপ অ্যালোকেশন এড়াতে `pin!` ম্যাক্রোর সাথে সরাসরি `Pin` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:here}}
```

</Listing>

আমরা এই পর্যন্ত এসেছি এই সত্যটি উপেক্ষা করে যে আমাদের আলাদা `Output` টাইপ থাকতে পারে। উদাহরণস্বরূপ, Listing 17-20-এ, `a`-এর জন্য বেনামী ফিউচার `Future<Output = u32>` ইমপ্লিমেন্ট করে, `b`-এর জন্য বেনামী ফিউচার `Future<Output = &str>` ইমপ্লিমেন্ট করে এবং `c`-এর জন্য বেনামী ফিউচার `Future<Output = bool>` ইমপ্লিমেন্ট করে।

<Listing number="17-20" caption="ভিন্ন টাইপ সহ তিনটি ফিউচার" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:here}}
```

</Listing>

আমরা তাদের জন্য অপেক্ষা করতে `trpl::join!` ব্যবহার করতে পারি, কারণ এটি আমাদের একাধিক ফিউচার টাইপ পাস করতে দেয় এবং সেই টাইপগুলির একটি টাপল তৈরি করে। আমরা `trpl::join_all` ব্যবহার করতে _পারি না_, কারণ এটির জন্য পাস করা সমস্ত ফিউচারের একই টাইপ থাকতে হবে। মনে রাখবেন, সেই error-টিই আমাদের `Pin`-এর সাথে এই অ্যাডভেঞ্চারে শুরু করিয়েছে!

এটি একটি মৌলিক ট্রেডঅফ: আমরা হয় `join_all`-এর সাথে একটি ডায়নামিক সংখ্যক ফিউচারের সাথে ডিল করতে পারি, যতক্ষণ না তাদের সবার একই টাইপ থাকে, অথবা আমরা `join` ফাংশন বা `join!` ম্যাক্রোর সাথে একটি নির্দিষ্ট সংখ্যক ফিউচারের সাথে ডিল করতে পারি, এমনকি যদি তাদের আলাদা টাইপ থাকে। Rust-এ অন্য কোনও টাইপের সাথে কাজ করার সময় আমরা যে পরিস্থিতির মুখোমুখি হব এটি সেই একই পরিস্থিতি। ফিউচারগুলি বিশেষ নয়, যদিও তাদের সাথে কাজ করার জন্য আমাদের কাছে কিছু চমৎকার সিনট্যাক্স রয়েছে এবং এটি একটি ভাল জিনিস।

### রেসিং ফিউচার (Racing Futures)

যখন আমরা `join` পরিবারের ফাংশন এবং ম্যাক্রোগুলির সাথে ফিউচারগুলিকে “join” করি, তখন আমরা এগিয়ে যাওয়ার আগে তাদের _সমস্ত_-এর শেষ হওয়ার প্রয়োজন বোধ করি। কখনও কখনও, যদিও, এগিয়ে যাওয়ার আগে আমাদের একটি সেট থেকে _কিছু_ ফিউচার শেষ হওয়ার প্রয়োজন হয়—এক ধরনের ফিউচারকে একে অপরের বিরুদ্ধে রেস করানোর মতো।

Listing 17-21-এ, আমরা আবারও দুটি ফিউচার, `slow` এবং `fast`-কে একে অপরের বিরুদ্ধে চালানোর জন্য `trpl::race` ব্যবহার করি।

<Listing number="17-21" caption="যে ফিউচারটি প্রথমে শেষ হয় তার ফলাফল পেতে `race` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:here}}
```

</Listing>

প্রতিটি ফিউচার যখন চলতে শুরু করে তখন একটি মেসেজ প্রিন্ট করে, `sleep` কল করে এবং অপেক্ষা করে কিছু সময়ের জন্য বিরতি দেয় এবং তারপর এটি শেষ হলে আরেকটি মেসেজ প্রিন্ট করে। তারপর আমরা `slow` এবং `fast` উভয়কেই `trpl::race`-এ পাস করি এবং তাদের মধ্যে একটি শেষ হওয়ার জন্য অপেক্ষা করি। (এখানে ফলাফলটি খুব আশ্চর্যজনক নয়: `fast` জিতে।) [“Our First Async Program”][async-program]<!-- ignore -->-এ যখন আমরা `race` ব্যবহার করেছি তার বিপরীতে, আমরা এখানে এটি যে `Either` ইনস্ট্যান্স রিটার্ন করে তা উপেক্ষা করি, কারণ সমস্ত আকর্ষণীয় আচরণ অ্যাসিঙ্ক্রোনাস ব্লকের বডিতে ঘটে।

লক্ষ্য করুন যে আপনি যদি `race`-এ আর্গুমেন্টগুলির ক্রম উল্টে দেন, তাহলে `fast` ফিউচারটি সর্বদা প্রথমে শেষ হওয়া সত্ত্বেও “started” মেসেজগুলির ক্রম পরিবর্তন হয়। এর কারণ হল এই বিশেষ `race` ফাংশনের ইমপ্লিমেন্টেশনটি ফেয়ার নয়। এটি সর্বদা আর্গুমেন্ট হিসাবে পাস করা ফিউচারগুলিকে যে ক্রমে পাস করা হয়েছে সেই ক্রমে চালায়। অন্যান্য ইমপ্লিমেন্টেশনগুলি _ফেয়ার_ এবং র‍্যান্ডমভাবে কোন ফিউচারটি প্রথমে পোল করতে হবে তা বেছে নেবে। আমরা যে রেসের ইমপ্লিমেন্টেশন ব্যবহার করছি সেটি ফেয়ার হোক বা না হোক, অন্য টাস্ক শুরু হওয়ার আগে ফিউচারগুলির _একটি_ তার বডিতে প্রথম `await` পর্যন্ত চলবে।

[Our First Async Program][async-program]<!-- ignore --> থেকে মনে করুন যে প্রতিটি অ্যাওয়েট পয়েন্টে, Rust একটি রানটাইমকে টাস্কটি থামানোর এবং অন্যটিতে স্যুইচ করার সুযোগ দেয় যদি যে ফিউচারের জন্য অপেক্ষা করা হচ্ছে সেটি প্রস্তুত না হয়। এর বিপরীতটিও সত্য: Rust _শুধুমাত্র_ অ্যাসিঙ্ক্রোনাস ব্লকগুলিকে থামিয়ে দেয় এবং একটি অ্যাওয়েট পয়েন্টে একটি রানটাইমে কন্ট্রোল ফিরিয়ে দেয়। অ্যাওয়েট পয়েন্টগুলির মধ্যে সবকিছু সিঙ্ক্রোনাস।

এর মানে হল যে আপনি যদি কোনও অ্যাওয়েট পয়েন্ট ছাড়াই একটি অ্যাসিঙ্ক্রোনাস ব্লকে অনেকগুলি কাজ করেন, তাহলে সেই ফিউচারটি অন্য কোনও ফিউচারকে অগ্রগতি করতে বাধা দেবে। আপনি কখনও কখনও এটিকে একটি ফিউচার অন্য ফিউচারগুলিকে _স্টার্ভ_ করছে বলে উল্লেখ করতে পারেন। কিছু ক্ষেত্রে, এটি কোনও বড় বিষয় নাও হতে পারে। যাইহোক, আপনি যদি কোনও ব্যয়বহুল সেটআপ বা দীর্ঘ-চলমান কাজ করছেন, অথবা আপনার যদি এমন একটি ফিউচার থাকে যা অনির্দিষ্টকালের জন্য কোনও নির্দিষ্ট কাজ করতে থাকবে, তাহলে আপনাকে কখন এবং কোথায় রানটাইমে কন্ট্রোল ফিরিয়ে দিতে হবে সে সম্পর্কে ভাবতে হবে।

একইভাবে, যদি আপনার দীর্ঘ-চলমান ব্লকিং অপারেশন থাকে, তাহলে অ্যাসিঙ্ক্রোনাস প্রোগ্রামের বিভিন্ন অংশের একে অপরের সাথে সম্পর্কযুক্ত হওয়ার উপায় সরবরাহ করার জন্য একটি দরকারী টুল হতে পারে।

কিন্তু সেই ক্ষেত্রগুলিতে আপনি কীভাবে রানটাইমে কন্ট্রোল ফিরিয়ে _দেবেন_?

<!-- Old headings. Do not remove or links may break. -->

<a id="yielding"></a>

### রানটাইমে কন্ট্রোল প্রদান করা (Yielding Control to the Runtime)

আসুন একটি দীর্ঘ-চলমান অপারেশনের সিমুলেশন করি। Listing 17-22 একটি `slow` ফাংশন প্রবর্তন করে।

<Listing number="17-22" caption="ধীর অপারেশন সিমুলেট করতে `thread::sleep` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:slow}}
```

</Listing>

এই কোডটি `trpl::sleep`-এর পরিবর্তে `std::thread::sleep` ব্যবহার করে যাতে `slow` কল করলে বর্তমান থ্রেডটি কিছু মিলিসেকেন্ডের জন্য ব্লক হয়ে যায়। আমরা `slow` ব্যবহার করতে পারি বাস্তব-বিশ্বের অপারেশনগুলির জন্য যা দীর্ঘ-চলমান এবং ব্লকিং উভয়ই।

Listing 17-23-এ, আমরা এক জোড়া ফিউচারে এই ধরনের CPU-বাউন্ড কাজ করার অনুকরণ করতে `slow` ব্যবহার করি।

<Listing number="17-23" caption="ধীর অপারেশন সিমুলেট করতে `thread::sleep` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:slow-futures}}
```

</Listing>

শুরু করার জন্য, প্রতিটি ফিউচার শুধুমাত্র একগুচ্ছ ধীর অপারেশন করার _পরে_ রানটাইমে কন্ট্রোল ফিরিয়ে দেয়। আপনি যদি এই কোডটি চালান তবে আপনি এই আউটপুটটি দেখতে পাবেন:

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

আমাদের আগের উদাহরণের মতোই, `a` শেষ হওয়ার সাথে সাথেই `race` শেষ হয়ে যায়। দুটি ফিউচারের মধ্যে কোনও ইন্টারলিভিং নেই, যদিও। `a` ফিউচার `trpl::sleep` কলের জন্য অপেক্ষা করার আগ পর্যন্ত তার সমস্ত কাজ করে, তারপর `b` ফিউচার তার নিজের `trpl::sleep` কলের জন্য অপেক্ষা করার আগ পর্যন্ত তার সমস্ত কাজ করে এবং অবশেষে `a` ফিউচার সম্পূর্ণ হয়। তাদের ধীর টাস্কগুলির মধ্যে উভয় ফিউচারকে অগ্রগতি করার অনুমতি দেওয়ার জন্য, আমাদের অ্যাওয়েট পয়েন্টগুলির প্রয়োজন যাতে আমরা রানটাইমে কন্ট্রোল ফিরিয়ে দিতে পারি। এর মানে হল আমাদের এমন কিছুর প্রয়োজন যার জন্য আমরা অপেক্ষা করতে পারি!

আমরা Listing 17-23-এ এই ধরনের হ্যান্ডঅফ ঘটতে দেখতে পাচ্ছি: যদি আমরা `a` ফিউচারের শেষে `trpl::sleep` সরিয়ে দিই, তাহলে এটি `b` ফিউচার _মোটেই_ না চলেই সম্পূর্ণ হয়ে যাবে। আসুন Listing 17-24-এ দেখানো মতো অপারেশনগুলিকে অগ্রগতি বন্ধ করে দেওয়ার জন্য `sleep` ফাংশনটিকে একটি শুরুর পয়েন্ট হিসাবে ব্যবহার করার চেষ্টা করি।

<Listing number="17-24" caption="অপারেশনগুলিকে অগ্রগতি বন্ধ করার অনুমতি দিতে `sleep` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

Listing 17-24-এ, আমরা `slow`-এ প্রতিটি কলের মধ্যে অ্যাওয়েট পয়েন্ট সহ `trpl::sleep` কল যুক্ত করি। এখন দুটি ফিউচারের কাজ ইন্টারলিভড:

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

`a` ফিউচারটি `b`-তে কন্ট্রোল দেওয়ার আগে কিছুক্ষণ চলে, কারণ এটি `trpl::sleep` কল করার আগেই `slow` কল করে, কিন্তু তার পরে ফিউচারগুলি প্রতিবার তাদের মধ্যে একটি অ্যাওয়েট পয়েন্টে আঘাত করার সময় অদলবদল করে। এক্ষেত্রে, আমরা `slow`-এ প্রতিটি কলের পরে এটি করেছি, কিন্তু আমরা যে কোনও উপায়ে কাজটিকে ভেঙে দিতে পারি যা আমাদের কাছে সবচেয়ে বেশি অর্থবোধক।

আমরা এখানে সত্যিই _স্লিপ_ করতে চাই না: আমরা যতটা পারি তত দ্রুত অগ্রগতি করতে চাই। আমাদের শুধু রানটাইমে কন্ট্রোল ফিরিয়ে দিতে হবে। আমরা সরাসরি `yield_now` ফাংশন ব্যবহার করে তা করতে পারি। Listing 17-25-এ, আমরা সেই সমস্ত `sleep` কলগুলিকে `yield_now` দিয়ে প্রতিস্থাপন করি।

<Listing number="17-25" caption="অপারেশনগুলিকে অগ্রগতি বন্ধ করার অনুমতি দিতে `yield_now` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:yields}}
```

</Listing>

এই কোডটি প্রকৃত উদ্দেশ্য সম্পর্কে আরও স্পষ্ট এবং `sleep` ব্যবহার করার চেয়ে উল্লেখযোগ্যভাবে দ্রুত হতে পারে, কারণ `sleep` দ্বারা ব্যবহৃত টাইমারের মতো টাইমারগুলির প্রায়শই তারা কতটা দানাদার হতে পারে তার উপর সীমাবদ্ধতা থাকে। উদাহরণস্বরূপ, আমরা যে `sleep`-এর সংস্করণ ব্যবহার করছি, সেটি সর্বদা কমপক্ষে এক মিলিসেকেন্ডের জন্য স্লিপ করবে, এমনকি যদি আমরা এটিকে এক ন্যানোসেকেন্ডের `Duration` পাস করি। আবারও, আধুনিক কম্পিউটারগুলি _দ্রুত_: তারা এক মিলিসেকেন্ডে অনেক কিছু করতে পারে!

আপনি Listing 17-26-এ দেখানো একটির মতো একটি ছোট বেঞ্চমার্ক সেট আপ করে এটি নিজে দেখতে পারেন। (এটি পারফরম্যান্স পরীক্ষা করার জন্য বিশেষভাবে কঠোর উপায় নয়, তবে এখানে পার্থক্য দেখানোর জন্য এটি যথেষ্ট।)

<Listing number="17-26" caption="`sleep` এবং `yield_now`-এর পারফরম্যান্সের তুলনা করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-26/src/main.rs:here}}
```

</Listing>

এখানে, আমরা সমস্ত স্ট্যাটাস প্রিন্টিং এড়িয়ে যাই, `trpl::sleep`-এ একটি এক-ন্যানোসেকেন্ড `Duration` পাস করি এবং প্রতিটি ফিউচারকে নিজে থেকে চলতে দিই, ফিউচারগুলির মধ্যে কোনও স্যুইচিং ছাড়াই। তারপর আমরা 1,000 বার চালাই এবং দেখি `trpl::sleep` ব্যবহার করা ফিউচারটি `trpl::yield_now` ব্যবহার করা ফিউচারের তুলনায় কত সময় নেয়।

`yield_now` সহ সংস্করণটি _অনেক_ দ্রুত!

এর মানে হল যে অ্যাসিঙ্ক্রোনাস কম্পিউট-বাউন্ড টাস্কগুলির জন্যও দরকারী হতে পারে, আপনার প্রোগ্রাম অন্য কী করছে তার উপর নির্ভর করে, কারণ এটি প্রোগ্রামের বিভিন্ন অংশের মধ্যে সম্পর্কগুলিকে গঠন করার জন্য একটি দরকারী টুল সরবরাহ করে। এটি _কোঅপারেটিভ মাল্টিটাস্কিং_-এর একটি ফর্ম, যেখানে প্রতিটি ফিউচারের অ্যাওয়েট পয়েন্টের মাধ্যমে কখন এটি কন্ট্রোল হস্তান্তর করবে তা নির্ধারণ করার ক্ষমতা রয়েছে। তাই প্রতিটি ফিউচারেরও খুব বেশি সময় ধরে ব্লক করা এড়াতে দায়িত্ব রয়েছে। কিছু Rust-ভিত্তিক এমবেডেড অপারেটিং সিস্টেমে, এটিই _একমাত্র_ ধরনের মাল্টিটাস্কিং!

বাস্তব-বিশ্বের কোডে, আপনি সাধারণত প্রতিটি লাইনে ফাংশন কলের সাথে অ্যাওয়েট পয়েন্টগুলিকে অল্টারনেট করবেন না। যদিও এইভাবে কন্ট্রোল প্রদান করা তুলনামূলকভাবে সস্তা, এটি বিনামূল্যে নয়। অনেক ক্ষেত্রে, একটি কম্পিউট-বাউন্ড টাস্ককে ভেঙে ফেলার চেষ্টা করলে এটি উল্লেখযোগ্যভাবে ধীর হয়ে যেতে পারে, তাই কখনও কখনও একটি অপারেশনকে সংক্ষিপ্তভাবে ব্লক করতে দেওয়া _সামগ্রিক_ পারফরম্যান্সের জন্য আরও ভাল। আপনার কোডের আসল পারফরম্যান্সের বাধাগুলি কী তা দেখতে সর্বদা পরিমাপ করুন। অন্তর্নিহিত ডায়নামিকটি মনে রাখা গুরুত্বপূর্ণ, যদিও, আপনি যদি দেখেন যে আপনি কনকারেন্টলি ঘটবে বলে আশা করেছিলেন এমন অনেক কাজ সিরিয়ালে ঘটছে!

### আমাদের নিজস্ব অ্যাসিঙ্ক্রোনাস অ্যাবস্ট্রাকশন তৈরি করা (Building Our Own Async Abstractions)

আমরা ফিউচারগুলিকে একসাথে কম্পোজ করে নতুন প্যাটার্ন তৈরি করতে পারি। উদাহরণস্বরূপ, আমরা ইতিমধ্যেই আমাদের কাছে থাকা অ্যাসিঙ্ক্রোনাস বিল্ডিং ব্লকগুলির সাথে একটি `timeout` ফাংশন তৈরি করতে পারি। যখন আমরা শেষ করব, ফলাফলটি হবে আরেকটি বিল্ডিং ব্লক যা আমরা আরও অ্যাসিঙ্ক্রোনাস অ্যাবস্ট্রাকশন তৈরি করতে ব্যবহার করতে পারি।

Listing 17-27 দেখায় কিভাবে আমরা আশা করব এই `timeout` একটি স্লো ফিউচারের সাথে কাজ করবে।

<Listing number="17-27" caption="একটি সময়সীমা সহ একটি ধীর অপারেশন চালানোর জন্য আমাদের কল্পিত `timeout` ব্যবহার করা" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-27/src/main.rs:here}}
```

</Listing>

আসুন এটি ইমপ্লিমেন্ট করি! শুরু করার জন্য, আসুন `timeout`-এর জন্য API সম্পর্কে চিন্তা করি:

-   এটি নিজেই একটি অ্যাসিঙ্ক্রোনাস ফাংশন হওয়া দরকার যাতে আমরা এটির জন্য অপেক্ষা করতে পারি।
-   এর প্রথম প্যারামিটারটি চালানো উচিত একটি ফিউচার। আমরা এটিকে জেনেরিক করতে পারি যাতে এটি যেকোনো ফিউচারের সাথে কাজ করতে পারে।
-   এর দ্বিতীয় প্যারামিটারটি হবে অপেক্ষা করার সর্বোচ্চ সময়। যদি আমরা একটি `Duration` ব্যবহার করি, তাহলে এটিকে `trpl::sleep`-এ পাস করা সহজ হবে।
-   এটি একটি `Result` রিটার্ন করা উচিত। ফিউচার সফলভাবে সম্পন্ন হলে, `Result` হবে `Ok` ফিউচার দ্বারা উৎপাদিত মান সহ। যদি টাইমআউটটি প্রথমে শেষ হয়ে যায়, তাহলে `Result` হবে `Err` টাইমআউট যে সময়কালের জন্য অপেক্ষা করেছে তার সাথে।

Listing 17-28 এই ডিক্লারেশনটি দেখায়।

<!-- This is not tested because it intentionally does not compile. -->

<Listing number="17-28" caption="`timeout`-এর স্বাক্ষর সংজ্ঞায়িত করা" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-28/src/main.rs:declaration}}
```

</Listing>

এটি টাইপের জন্য আমাদের লক্ষ্যগুলি পূরণ করে। এখন আসুন _আচরণ_ সম্পর্কে চিন্তা করি যা আমাদের প্রয়োজন: আমরা যে ফিউচারটি পাস করেছি সেটিকে সময়কালের বিরুদ্ধে রেস করাতে চাই। আমরা সময়কাল থেকে একটি টাইমার ফিউচার তৈরি করতে `trpl::sleep` ব্যবহার করতে পারি এবং কলার যে ফিউচারটি পাস করে তার সাথে সেই টাইমারটি চালানোর জন্য `trpl::race` ব্যবহার করতে পারি।

আমরা এটাও জানি যে `race` ফেয়ার নয়, আর্গুমেন্টগুলিকে যে ক্রমে পাস করা হয়েছে সেই ক্রমে পোল করে। সুতরাং, আমরা `future_to_try`-কে `race`-এ প্রথমে পাস করি যাতে `max_time` খুব কম সময় হলেও এটি সম্পূর্ণ হওয়ার সুযোগ পায়। যদি `future_to_try` প্রথমে শেষ হয়, তাহলে `race` `future_to_try`-এর আউটপুট সহ `Left` রিটার্ন করবে। যদি `timer` প্রথমে শেষ হয়, তাহলে `race` টাইমারের `()` আউটপুট সহ `Right` রিটার্ন করবে।

Listing 17-29-এ, আমরা `trpl::race`-এর জন্য অপেক্ষা করার ফলাফলের উপর ম্যাচ করি।

<Listing number="17-29" caption="`race` এবং `sleep` সহ `timeout` সংজ্ঞায়িত করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-29/src/main.rs:implementation}}
```

</Listing>

যদি `future_to_try` সফল হয় এবং আমরা একটি `Left(output)` পাই, তাহলে আমরা `Ok(output)` রিটার্ন করি। যদি পরিবর্তে স্লিপ টাইমার শেষ হয়ে যায় এবং আমরা একটি `Right(())` পাই, তাহলে আমরা `_` দিয়ে `()` উপেক্ষা করি এবং পরিবর্তে `Err(max_time)` রিটার্ন করি।

এর সাথে, আমাদের কাছে দুটি অন্যান্য অ্যাসিঙ্ক্রোনাস হেল্পার থেকে তৈরি একটি কার্যকরী `timeout` রয়েছে। আমরা যদি আমাদের কোড চালাই, তাহলে এটি টাইমআউটের পরে ব্যর্থতার মোড প্রিন্ট করবে:

```text
Failed after 2 seconds
```

যেহেতু ফিউচারগুলি অন্যান্য ফিউচারের সাথে কম্পোজ করে, তাই আপনি ছোট অ্যাসিঙ্ক্রোনাস বিল্ডিং ব্লক ব্যবহার করে সত্যিই শক্তিশালী টুল তৈরি করতে পারেন। উদাহরণস্বরূপ, আপনি টাইমআউটগুলিকে রিট্রাই-এর সাথে একত্রিত করতে এই একই পদ্ধতি ব্যবহার করতে পারেন এবং পরিবর্তে নেটওয়ার্ক কলের মতো অপারেশনগুলির সাথে সেগুলি ব্যবহার করতে পারেন (চ্যাপ্টারের শুরু থেকে একটি উদাহরণ)।

বাস্তবে, আপনি সাধারণত সরাসরি `async` এবং `await`-এর সাথে কাজ করবেন এবং গৌণভাবে `join`, `join_all`, `race` ইত্যাদির মতো ফাংশন এবং ম্যাক্রোগুলির সাথে কাজ করবেন। সেই API-গুলির সাথে ফিউচার ব্যবহার করার জন্য আপনাকে কেবল মাঝে মাঝে `pin`-এর কাছে পৌঁছাতে হবে।

আমরা এখন একই সময়ে একাধিক ফিউচারের সাথে কাজ করার বিভিন্ন উপায় দেখেছি। এরপরে, আমরা দেখব কিভাবে আমরা _স্ট্রিম_-এর সাহায্যে সময়ের সাথে একটি সিকোয়েন্সে একাধিক ফিউচারের সাথে কাজ করতে পারি। এখানে আরও কয়েকটি বিষয় রয়েছে যা আপনি প্রথমে বিবেচনা করতে চাইতে পারেন:

-   আমরা কিছু গ্রুপের সমস্ত ফিউচার শেষ হওয়ার জন্য অপেক্ষা করতে `join_all`-এর সাথে একটি `Vec` ব্যবহার করেছি। পরিবর্তে একটি সিকোয়েন্সে ফিউচারের একটি গ্রুপ প্রসেস করতে আপনি কীভাবে একটি `Vec` ব্যবহার করতে পারেন? এটি করার ট্রেডঅফগুলি কী কী?

-   `futures` ক্রেট থেকে `futures::stream::FuturesUnordered` টাইপটি দেখুন। এটি ব্যবহার করা একটি `Vec` ব্যবহার করার থেকে কীভাবে আলাদা হবে? (চিন্তা করবেন না যে এটি ক্রেটের `stream` অংশ থেকে এসেছে; এটি ফিউচারের যেকোনো কালেকশনের সাথে ঠিকঠাক কাজ করে।)

[dyn]: ch12-03-improving-error-handling-and-modularity.html
[enum-alt]: ch12-03-improving-error-handling-and-modularity.html#returning-errors-from-the-run-function
[async-program]: ch17-01-futures-and-syntax.html#our-first-async-program
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
