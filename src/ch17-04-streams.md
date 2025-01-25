## Streams: Futures in Sequence

<!-- Old headings. Do not remove or links may break. -->

<a id="streams"></a>

এই chapter এ আমরা এতক্ষণ mostly individual future নিয়েই আলোচনা করেছি। এর একটি বড় exception ছিল async channel যা আমরা ব্যবহার করেছিলাম। মনে করুন কিভাবে আমরা [“Message Passing”][17-02-messages]<!-- ignore --> section এ এই chapter এর শুরুতে async channel এর receiver ব্যবহার করেছিলাম। Async `recv` method সময়ের সাথে item এর একটি sequence তৈরি করে। এটি _stream_ নামে পরিচিত একটি general pattern এর instance।

আমরা Chapter 13 এ item এর একটি sequence দেখেছিলাম, যখন আমরা [The Iterator Trait and the `next` Method][iterator-trait]<!-- ignore --> section এ `Iterator` trait দেখেছিলাম, কিন্তু iterator এবং async channel receiver এর মধ্যে দুটি পার্থক্য আছে। প্রথম পার্থক্য হলো সময়: iterator synchronous, যেখানে channel receiver asynchronous। দ্বিতীয়টি হলো API। যখন আমরা সরাসরি `Iterator` নিয়ে কাজ করি, তখন আমরা এর synchronous `next` method call করি। Particular `trpl::Receiver` stream এর সাথে, আমরা এর পরিবর্তে asynchronous `recv` method call করেছিলাম। এছাড়া, এই API গুলো দেখতে অনেকটা একই রকম, এবং এই similarity কোনো কাকতালীয় ঘটনা নয়। একটি stream হলো iteration এর asynchronous form এর মতো। যেখানে `trpl::Receiver` specifically message receive করার জন্য wait করে, general-purpose stream API আরও বেশি broad: এটি `Iterator` এর মতো next item provide করে, কিন্তু asynchronously।

Rust এ iterator এবং stream এর মধ্যে similarity মানে হলো আমরা আসলে যেকোনো iterator থেকে একটি stream তৈরি করতে পারি। Iterator এর মতো, আমরা stream এর `next` method call করে এবং তারপর output await করে এর সাথে কাজ করতে পারি, যেমন Listing 17-30 এ দেখানো হয়েছে।

<Listing number="17-30" caption="একটি iterator থেকে stream তৈরি করা এবং এর value print করা" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-30/src/main.rs:stream}}
```

</Listing>

আমরা numbers এর একটি array দিয়ে শুরু করি, যা আমরা একটি iterator এ convert করি এবং তারপর সব value double করার জন্য `map` call করি। তারপর আমরা `trpl::stream_from_iter` function ব্যবহার করে iterator কে stream এ convert করি। এরপর, `while let` loop দিয়ে stream এ item গুলো আসলে সেগুলোর উপর loop করি।

দুর্ভাগ্যবশত, যখন আমরা code run করার চেষ্টা করি, তখন এটি compile হয় না, এর পরিবর্তে এটি report করে যে কোনো `next` method available নেই:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-30
cargo build
copy only the error output
-->

```console
error[E0599]: no method named `next` found for struct `Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = note: the full type name has been written to 'file:///projects/async_await/target/debug/deps/async_await-9de943556a6001b8.long-type-1281356139287206597.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

এই output টি explain করে, compiler error এর কারণ হলো `next` method ব্যবহার করার জন্য আমাদের scope এ সঠিক trait এর প্রয়োজন। আমাদের আলোচনা অনুযায়ী, আপনি হয়তো আশা করতে পারেন যে trait টি `Stream` হবে, কিন্তু আসলে এটি `StreamExt`। _Extension_ এর সংক্ষিপ্ত রূপ `Ext` হলো Rust community তে একটি সাধারণ pattern একটি trait কে অন্য trait দিয়ে extend করার জন্য।

আমরা chapter এর শেষে `Stream` এবং `StreamExt` trait নিয়ে আরও বিস্তারিত আলোচনা করব, তবে আপাতত আপনার শুধু জানার প্রয়োজন যে `Stream` trait এমন একটি low-level interface define করে যা effectively `Iterator` এবং `Future` trait কে combine করে। `StreamExt` `Stream` এর উপরে higher-level API এর set supply করে, `next` method সহ এবং `Iterator` trait দ্বারা provide করা utility method এর মতো অন্য utility method ও সরবরাহ করে। `Stream` এবং `StreamExt` এখনও Rust এর standard library এর অংশ নয়, তবে বেশিরভাগ ecosystem crate একই definition ব্যবহার করে।

Compiler error এর fix হলো Listing 17-31 এ দেখানো হিসাবে `trpl::StreamExt` এর জন্য একটি `use` statement যোগ করা।

<Listing number="17-31" caption="একটি iterator কে stream এর basis হিসেবে ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-31/src/main.rs:all}}
```

</Listing>

সব অংশ একসাথে করলে, এই code টি আমরা যেভাবে চেয়েছি সেভাবে কাজ করে! আরও কি, এখন যেহেতু আমাদের scope এ `StreamExt` আছে, তাই আমরা iterator এর মতো এর সব utility method ব্যবহার করতে পারি। উদাহরণস্বরূপ, Listing 17-32 এ, আমরা `filter` method ব্যবহার করে তিন এবং পাঁচের গুণিতক ছাড়া বাকি সবকিছু filter out করি।

<Listing number="17-32" caption="`StreamExt::filter` method দিয়ে একটি stream filter করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-32/src/main.rs:all}}
```

</Listing>

অবশ্যই, এটি খুব বেশি interesting নয়, কারণ আমরা normal iterator দিয়ে একই কাজ করতে পারতাম এবং কোনো async এর প্রয়োজনও হতো না। চলুন দেখি আমরা streams এর জন্য unique কি করতে পারি।

### Composing Streams

অনেক concept naturally stream হিসেবে represent করা হয়: queue এ available হওয়া item, computer এর জন্য full data set খুব বেশি বড় হলে file system থেকে incrementally pull করা data এর chunk, বা সময়ের সাথে network এর মাধ্যমে arrive হওয়া data। যেহেতু stream হলো future, তাই আমরা সেগুলোকে অন্য যেকোনো future এর সাথে ব্যবহার করতে পারি এবং interesting উপায়ে combine করতে পারি। উদাহরণস্বরূপ, আমরা খুব বেশি network call trigger করা avoid করার জন্য event batch up করতে পারি, long-running operation এর sequence এ timeout set করতে পারি, অথবা অপ্রয়োজনীয় কাজ করা avoid করার জন্য user interface event throttle করতে পারি।

চলুন Listing 17-33 এ দেখানো হিসাবে WebSocket বা অন্য কোনো real-time communication protocol থেকে আমরা যে data stream দেখতে পারি তার stand-in হিসেবে message এর একটি ছোট stream তৈরি করার মাধ্যমে শুরু করি।

<Listing number="17-33" caption="`rx` receiver কে `ReceiverStream` হিসেবে ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-33/src/main.rs:all}}
```

</Listing>

প্রথমে, আমরা `get_messages` নামে একটি function তৈরি করি যা `impl Stream<Item = String>` return করে। এর implementation এর জন্য, আমরা একটি async channel তৈরি করি, ইংরেজি alphabet এর প্রথম 10 টি letter এর উপর loop করি, এবং সেগুলোকে channel এর মাধ্যমে send করি।

আমরা একটি নতুন type ও ব্যবহার করি: `ReceiverStream`, যা `trpl::channel` থেকে `rx` receiver কে `next` method সহ একটি `Stream` এ convert করে। `main` এ ফিরে, আমরা stream থেকে সব message print করার জন্য `while let` loop ব্যবহার করি।

যখন আমরা এই code run করি, তখন আমরা ঠিক সেই result পাই যা আমরা আশা করি:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Message: 'a'
Message: 'b'
Message: 'c'
Message: 'd'
Message: 'e'
Message: 'f'
Message: 'g'
Message: 'h'
Message: 'i'
Message: 'j'
```

আবারও, আমরা regular `Receiver` API বা normal `Iterator` API দিয়েও এটা করতে পারতাম, তবে চলুন এমন feature যোগ করি যার জন্য stream এর প্রয়োজন: stream এ থাকা প্রত্যেক item এর উপর apply হয় এমন একটি timeout add করা, এবং আমরা emit করা item গুলোর উপর delay add করা, যা Listing 17-34 এ দেখানো হয়েছে।

<Listing number="17-34" caption="Stream এ item গুলোর উপর time limit set করার জন্য `StreamExt::timeout` method ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-34/src/main.rs:timeout}}
```

</Listing>

আমরা `timeout` method দিয়ে stream এ timeout add করে শুরু করি, যা `StreamExt` trait থেকে আসে। তারপর আমরা `while let` loop এর body update করি, কারণ stream এখন একটি `Result` return করে। `Ok` variant indicate করে যে message টি সময়ে এসেছে; `Err` variant indicate করে যে কোনো message আসার আগে timeout elapsed হয়েছে। আমরা সেই result এর উপর `match` করি এবং যখন আমরা successfully receive করি তখন message print করি অথবা timeout নিয়ে একটি notice print করি। সবশেষে, লক্ষ্য করুন যে timeout apply করার পর আমরা message pin করি, কারণ timeout helper এমন একটি stream তৈরি করে যা poll করার জন্য pin করার প্রয়োজন।

তবে, যেহেতু message গুলোর মধ্যে কোনো delay নেই, তাই এই timeout program এর behaviour change করে না। চলুন আমরা send করা message গুলোতে variable delay add করি, যা Listing 17-35 এ দেখানো হয়েছে।

<Listing number="17-35" caption="`get_messages` কে async function না বানিয়ে async delay দিয়ে `tx` এর মাধ্যমে message send করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-35/src/main.rs:messages}}
```

</Listing>

`get_messages` এ, আমরা `messages` array এর সাথে `enumerate` iterator method ব্যবহার করি যাতে আমরা item এর সাথে সাথে send করা প্রত্যেক item এর index ও পেতে পারি। তারপর আমরা even-index item এ 100-millisecond delay এবং odd-index item এ 300-millisecond delay apply করি real-world এ message stream থেকে আমরা যে ভিন্ন delay দেখতে পাই তা simulate করার জন্য। যেহেতু আমাদের timeout 200 milliseconds এর জন্য, তাই এটি message গুলোর অর্ধেকের উপর affect করবে।

Block না করে `get_messages` function এ message এর মাঝে sleep করার জন্য, আমাদের async ব্যবহার করার প্রয়োজন। তবে, আমরা `get_messages` কে async function বানাতে পারি না, কারণ তাহলে আমরা `Stream<Item = String>` এর পরিবর্তে `Future<Output = Stream<Item = String>>` return করতাম। Caller কে stream এর access পাওয়ার জন্য `get_messages` await করতে হতো। কিন্তু মনে রাখবেন: একটি নির্দিষ্ট future এ সবকিছু linearly ঘটে; concurrency future এর _মধ্যে_ ঘটে। `get_messages` await করলে এটি সব message send করতে হতো, প্রত্যেক message এর মাঝে sleep delay সহ, receiver stream return করার আগে। ফলস্বরূপ, timeout useless হতো। Stream এ কোনো delay থাকতো না; সেগুলো stream available হওয়ার আগেই ঘটতো।

এর পরিবর্তে, আমরা `get_messages` কে regular function হিসেবে রাখি যা stream return করে, এবং আমরা async `sleep` call handle করার জন্য একটি task spawn করি।

> Note: এভাবে `spawn_task` call করা কাজ করে কারণ আমরা ইতিমধ্যে আমাদের runtime set up করেছি; যদি না করতাম, তাহলে এটি panic করত। অন্যান্য implementation ভিন্ন tradeoff choose করে: তারা হয়তো একটি নতুন runtime spawn করতে পারে এবং panic avoid করতে পারে তবে কিছু extra overhead এ শেষ হতে পারে, অথবা তারা হয়তো runtime এর reference ছাড়া standalone ভাবে task spawn করার কোনো উপায় provide নাও করতে পারে। নিশ্চিত করুন যে আপনার runtime কোন tradeoff choose করেছে এবং সে অনুযায়ী code লিখুন!

এখন আমাদের code এর অনেক বেশি interesting result আছে। Message এর প্রত্যেক অন্য pair এর মাঝে একটি `Problem: Elapsed(())` error আছে।

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-35
cargo run
copy only the program output, *not* the compiler output
-->

```text
Message: 'a'
Problem: Elapsed(())
Message: 'b'
Message: 'c'
Problem: Elapsed(())
Message: 'd'
Message: 'e'
Problem: Elapsed(())
Message: 'f'
Message: 'g'
Problem: Elapsed(())
Message: 'h'
Message: 'i'
Problem: Elapsed(())
Message: 'j'
```

Timeout message গুলোকে শেষ পর্যন্ত arrive হওয়া থেকে prevent করে না। আমরা এখনও original message গুলো পাই, কারণ আমাদের channel _unbounded_: এটি memory তে fit করতে পারে এমন যত খুশি message hold করতে পারে। যদি timeout এর আগে message arrive না করে, তাহলে আমাদের stream handler সেটার হিসাব করবে, কিন্তু যখন এটি আবার stream poll করে, তখন হয়তো message arrive হয়ে গেছে।

অন্য ধরনের channel বা আরও general ভাবে অন্য ধরনের stream ব্যবহার করে প্রয়োজন হলে আপনি ভিন্ন behavior পেতে পারেন। চলুন time interval এর stream এর সাথে এই message stream combine করে practice এ দেখি।

[17-02-messages]: ch17-02-applying-concurrency-with-async.html#message-passing
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method

### Merging Streams

প্রথমত, চলুন অন্য একটি stream তৈরি করি, যা প্রত্যেক millisecond এ একটি item emit করবে যদি আমরা এটিকে সরাসরি run করতে দেই। Simplicity এর জন্য, আমরা delay তে একটি message send করার জন্য `sleep` function ব্যবহার করতে পারি এবং channel থেকে stream তৈরি করার `get_messages` এ ব্যবহার করা approach এর সাথে combine করতে পারি। পার্থক্য হলো এইবার, আমরা elapsed হওয়া interval এর count send করব, তাই return type `impl Stream<Item = u32>` হবে, এবং আমরা function টি `get_intervals` call করতে পারি (Listing 17-36 দেখুন)।

<Listing number="17-36" caption="একটি counter দিয়ে stream তৈরি করা যা প্রতি millisecond এ একবার emit করা হবে" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-36/src/main.rs:intervals}}
```

</Listing>

আমরা task এ একটি `count` define করে শুরু করি। (আমরা task এর বাইরেও define করতে পারতাম, তবে যেকোনো নির্দিষ্ট variable এর scope limit করা clear)। তারপর আমরা একটি infinite loop তৈরি করি। Loop এর প্রত্যেক iteration asynchronously এক millisecond এর জন্য sleep করে, count increment করে, এবং তারপর এটিকে channel এ send করে। যেহেতু এটি `spawn_task` দ্বারা তৈরি task এ wrap করা, তাই runtime এর সাথে এর সব কিছু—infinite loop সহ—clean up করা হবে।

এই ধরনের infinite loop, যা শুধুমাত্র পুরো runtime tear down হলেই শেষ হয়, async Rust এ বেশ common: অনেক program এর indefinitely run করতে থাকার প্রয়োজন। Async এর সাথে, এটি অন্য কিছু block করে না, যতক্ষণ loop এর through এ প্রত্যেক iteration এ কমপক্ষে একটি await point থাকে।

এখন, আমাদের main function এর async block এ ফিরে, আমরা Listing 17-37 এ দেখানো হিসাবে `messages` এবং `intervals` stream merge করার চেষ্টা করতে পারি।

<Listing number="17-37" caption="`messages` এবং `intervals` stream merge করার চেষ্টা করা" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-37/src/main.rs:main}}
```

</Listing>

আমরা `get_intervals` call করে শুরু করি। তারপর আমরা `merge` method দিয়ে `messages` এবং `intervals` stream merge করি, যা একাধিক stream কে একটি stream এ combine করে যা কোনো particular ordering impose না করে source stream থেকে item available হওয়ার সাথে সাথেই produce করে। সবশেষে, আমরা `messages` এর পরিবর্তে সেই combined stream এর উপর loop করি।

এই মুহূর্তে, `messages` এবং `intervals` কোনোটিরই pin বা mutable হওয়ার প্রয়োজন নেই, কারণ দুটোই single `merged` stream এ combine হবে। তবে, `merge` এর এই call compile হয় না! ( `while let` loop এ `next` call ও করে না, তবে আমরা সেটায় পরে আসব)। কারণ হলো দুটি stream এর type ভিন্ন। `messages` stream এর type হলো `Timeout<impl Stream<Item = String>>`, যেখানে `Timeout` হলো `timeout` call এর জন্য `Stream` implement করা type। `intervals` stream এর type হলো `impl Stream<Item = u32>`। এই দুটি stream merge করার জন্য, আমাদের একটিকে অন্যটির সাথে match করার জন্য transform করার প্রয়োজন। আমরা intervals stream rework করব, কারণ messages ইতিমধ্যে আমাদের desired basic format এ আছে এবং timeout error handle করতে হয় (Listing 17-38 দেখুন)।

<!-- We cannot directly test this one, because it never stops. -->

<Listing number="17-38" caption="`intervals` stream এর type কে `messages` stream এর type এর সাথে align করা" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-38/src/main.rs:main}}
```

</Listing>

প্রথমত, আমরা `map` helper method ব্যবহার করে `intervals` কে string এ transform করতে পারি। দ্বিতীয়ত, আমাদের `messages` থেকে `Timeout` match করার প্রয়োজন। যেহেতু আমরা আসলে `intervals` এর জন্য timeout _চাই না_, তাই আমরা এমন একটি timeout তৈরি করতে পারি যা আমাদের ব্যবহার করা অন্য duration এর চেয়ে বেশি long। এখানে, আমরা `Duration::from_secs(10)` দিয়ে 10-second timeout তৈরি করি। সবশেষে, আমাদের `stream` কে mutable বানানোর প্রয়োজন, যাতে `while let` loop এর `next` call stream এর মধ্যে iterate করতে পারে, এবং pin করি যাতে এটা safe ভাবে করা যায়। এটা আমাদের _প্রায়_ সেই জায়গায় নিয়ে যায় যেখানে আমাদের থাকার দরকার। সবকিছু type check করে। আপনি যদি এটা run করেন, তাহলে দুটি problem হবে। প্রথমত, এটা কখনো stop হবে না! <span class="keystroke">ctrl-c</span> দিয়ে আপনাকে এটা stop করতে হবে। দ্বিতীয়ত, ইংরেজি alphabet এর message গুলো সব interval counter message এর মাঝে চাপা পরে যাবে:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the tasks running differently rather than
changes in the compiler -->

```text
--snip--
Interval: 38
Interval: 39
Interval: 40
Message: 'a'
Interval: 41
Interval: 42
Interval: 43
--snip--
```

Listing 17-39 এই last দুটি problem solve করার একটি উপায় দেখায়।

<Listing number="17-39" caption="Merged stream manage করার জন্য `throttle` এবং `take` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-39/src/main.rs:throttle}}
```

</Listing>

প্রথমত, আমরা `intervals` stream এর উপর `throttle` method ব্যবহার করি যাতে এটি `messages` stream কে overwhelm না করে। _Throttling_ হলো এমন একটি উপায় যার মাধ্যমে কোনো function call হওয়ার rate limit করা যায়—অথবা, এই ক্ষেত্রে, stream কতবার poll হবে তা limit করা যায়। প্রতি 100 milliseconds এ একবার করলে ঠিক হবে, কারণ আমাদের message গুলো প্রায় ততটা সময়েই arrive করে।

Stream থেকে আমরা যে number এর item accept করব তা limit করার জন্য, আমরা `merged` stream এর উপর `take` method apply করি, কারণ আমরা শুধু একটি stream নয়, final output limit করতে চাই।

এখন যখন আমরা program run করি, তখন এটি stream থেকে 20টি item pull করার পর stop হয়, এবং interval message গুলো message গুলোকে overwhelm করে না। আমরা `Interval: 100` বা `Interval: 200` বা এই ধরনের কিছু পাই না, কিন্তু এর পরিবর্তে `Interval: 1`, `Interval: 2`, এবং এই ধরনের কিছু পাই—এমনকি যদিও আমাদের source stream এর প্রত্যেক millisecond এ event তৈরি করার ক্ষমতা আছে। কারণ হলো `throttle` call একটি নতুন stream তৈরি করে যা original stream কে wrap করে যাতে original stream throttle rate এ poll হয়, এর "native" rate এ নয়। আমাদের কাছে অনেক unhandled interval message নেই যা আমরা ignore করা choose করছি। এর পরিবর্তে, আমরা সেই interval message গুলো প্রথমে generate করি না! এটি Rust এর future এর inherent "laziness" যা আমাদের performance characteristics choose করার সুযোগ দেয়।

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-39
cargo run
copy and paste only the program output
-->

```text
Interval: 1
Message: 'a'
Interval: 2
Interval: 3
Problem: Elapsed(())
Interval: 4
Message: 'b'
Interval: 5
Message: 'c'
Interval: 6
Interval: 7
Problem: Elapsed(())
Interval: 8
Message: 'd'
Interval: 9
Message: 'e'
Interval: 10
Interval: 11
Problem: Elapsed(())
Interval: 12
```

আমাদের handle করার জন্য শেষ একটি জিনিস আছে: error! এই channel-based stream গুলোর সাথে, channel এর অন্য side close হয়ে গেলে `send` call fail করতে পারে—এবং এটি শুধু matter করে runtime কিভাবে stream তৈরি করা future execute করে। এতক্ষণ, আমরা `unwrap` call করে এই possibility ignore করেছি, কিন্তু একটি well-behaved app এ আমাদের error explicitly handle করা উচিত, কমপক্ষে loop end করে যাতে আমরা আর message send করার চেষ্টা না করি। Listing 17-40 একটি simple error strategy দেখায়: issue print করুন এবং তারপর loop থেকে `break` করুন।

<Listing number="17-40" caption="Error handle করা এবং loop shut down করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-40/src/main.rs:errors}}
```

</Listing>

যথারীতি, message send error handle করার সঠিক উপায় different হবে; শুধু নিশ্চিত করুন যে আপনার একটি strategy আছে।

এখন যেহেতু আমরা বাস্তবে অনেক async দেখেছি, তাই চলুন একটু back এ যাই এবং Rust async কে কাজ করানোর জন্য ব্যবহার করা `Future`, `Stream`, এবং অন্য key trait এর কিছু detail নিয়ে আলোচনা করি।

[17-02-messages]: ch17-02-applying-concurrency-with-async.html#message-passing
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
