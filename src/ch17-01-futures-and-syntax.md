## Futures and the Async Syntax

Rust এ asynchronous programming এর মূল উপাদান হলো _futures_ এবং Rust এর `async` এবং `await` keyword।

একটি _future_ হলো এমন একটি value যা এখন ready নাও হতে পারে কিন্তু ভবিষ্যতে কোনো এক সময় ready হবে। (একই concept অনেক ভাষায় দেখা যায়, কখনও _task_ বা _promise_ এর মতো অন্য নামে)। Rust একটি building block হিসেবে `Future` trait প্রদান করে যাতে বিভিন্ন async operation বিভিন্ন data structure দিয়ে implement করা যায় কিন্তু common interface থাকে। Rust এ, future হলো সেই type যা `Future` trait implement করে। প্রত্যেক future তার progress এবং "ready" বলতে কী বোঝায় সে সম্পর্কে নিজস্ব তথ্য রাখে।

আপনি block এবং function এ `async` keyword ব্যবহার করতে পারেন এটা উল্লেখ করার জন্য যে এগুলো interrupt এবং resume করা যেতে পারে। Async block বা async function এর মধ্যে, আপনি `await` keyword ব্যবহার করে একটি _future await করতে পারেন_ (অর্থাৎ, এটি ready হওয়ার জন্য অপেক্ষা করতে পারেন)। কোনো async block বা function এর মধ্যে আপনি যেখানে একটি future await করেন, সেটি সেই async block বা function pause এবং resume করার জন্য potential spot। Future এর value এখনও available আছে কিনা তা দেখার জন্য future এর সাথে check করার process কে _polling_ বলা হয়।

C# এবং JavaScript এর মতো কিছু অন্য language ও async programming এর জন্য `async` এবং `await` keyword ব্যবহার করে। আপনি যদি সেই language গুলোর সাথে পরিচিত হন, তাহলে আপনি Rust কিভাবে কাজ করে তার কিছু গুরুত্বপূর্ণ পার্থক্য দেখতে পাবেন, যার মধ্যে syntax handle করার পদ্ধতিও রয়েছে। এর ভালো কারণ আছে, যা আমরা দেখব!

Async Rust লেখার সময়, আমরা বেশিরভাগ সময় `async` এবং `await` keyword ব্যবহার করি। Rust `Future` trait ব্যবহার করে এদের equivalent code এ compile করে, অনেকটা `Iterator` trait ব্যবহার করে `for` loop কে equivalent code এ compile করার মতো। তবে, যেহেতু Rust `Future` trait প্রদান করে, তাই আপনি যখন প্রয়োজন হয় তখন নিজের data type এর জন্য ও implement করতে পারেন। এই chapter এ আমরা যে function গুলো দেখব তার অনেকগুলোই `Future` এর নিজস্ব implementation সহ type return করে। আমরা chapter এর শেষে trait এর definition এ ফিরে যাব এবং এটি কিভাবে কাজ করে তার আরও গভীরে যাব, কিন্তু আপাতত এতটুকু জানলেই আমাদের কাজ চালিয়ে যাওয়ার জন্য যথেষ্ট।

এগুলো সবই কিছুটা abstract মনে হতে পারে, তাই চলুন আমাদের প্রথম async program লিখি: একটি ছোট web scraper। আমরা command line থেকে দুটি URL pass করব, concurrent ভাবে দুটি fetch করব, এবং যেটার কাজ আগে শেষ হবে তার result return করব। এই উদাহরণে বেশ কিছু নতুন syntax থাকবে, কিন্তু চিন্তা করবেন না—আমরা প্রয়োজনীয় সবকিছু explain করব।

## Our First Async Program

এই chapter এর focus async শেখার উপর রাখার জন্য, ecosystem এর বিভিন্ন অংশ নিয়ে কাজ করার পরিবর্তে, আমরা `trpl` crate (`trpl` মানে "The Rust Programming Language") তৈরি করেছি। এটি মূলত [`futures`][futures-crate]<!-- ignore --> এবং [`tokio`][tokio]<!-- ignore --> crate থেকে আপনার প্রয়োজনীয় সব type, trait, এবং function re-export করে। `futures` crate হলো async code এর জন্য Rust experimentation এর official home, এবং এখানেই মূলত `Future` trait design করা হয়েছিল। Tokio হলো বর্তমানে Rust এ সবচেয়ে বেশি ব্যবহৃত async runtime, বিশেষ করে web application এর জন্য। আরও অনেক ভালো runtime আছে, এবং সেগুলো হয়তো আপনার উদ্দেশ্যে আরও বেশি উপযুক্ত হতে পারে। আমরা `trpl` এর under the hood এ `tokio` crate ব্যবহার করি, কারণ এটি ভালোভাবে test করা এবং ব্যাপকভাবে ব্যবহৃত।

কিছু ক্ষেত্রে, `trpl` আপনাকে এই chapter এর সাথে সম্পর্কিত detail এর উপর focus রাখার জন্য original API গুলোর rename বা wrap ও করে। আপনি যদি crate টি কি করে তা বুঝতে চান, তাহলে আমরা আপনাকে [its source code][crate-source]<!-- ignore --> দেখতে encourage করব। আপনি দেখতে পারবেন যে কোন crate থেকে প্রতিটি re-export এসেছে, এবং crate টি কি করে তা explain করার জন্য আমরা প্রচুর comment রেখেছি।

`hello-async` নামে একটি নতুন binary project তৈরি করুন এবং `trpl` crate কে dependency হিসেবে যোগ করুন:

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

এখন আমরা আমাদের প্রথম async program লেখার জন্য `trpl` দ্বারা দেওয়া বিভিন্ন অংশ ব্যবহার করতে পারি। আমরা একটি ছোট command line tool তৈরি করব যা দুটি web page fetch করবে, প্রত্যেকটি থেকে `<title>` element pull করবে, এবং যে page এর পুরো process আগে শেষ হবে তার title print করবে।

### Defining the page_title Function

আসুন প্রথমে এমন একটি function লিখি যা parameter হিসেবে একটি page URL নেয়, সেটিতে request করে, এবং title element এর text return করে (Listing 17-1 দেখুন)।

<Listing number="17-1" file-name="src/main.rs" caption="একটি HTML page থেকে title element পাওয়ার জন্য async function define করা">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

প্রথমে, আমরা `page_title` নামে একটি function define করি এবং এটিকে `async` keyword দিয়ে mark করি। তারপর আমরা pass করা URL fetch করার জন্য `trpl::get` function ব্যবহার করি এবং response await করার জন্য `await` keyword যোগ করি। Response এর text পাওয়ার জন্য, আমরা এর `text` method call করি এবং আবারও `await` keyword দিয়ে await করি। এই দুটি step asynchronous। `get` function এর জন্য, server কে তার response এর প্রথম অংশ, যার মধ্যে HTTP header, cookie, ইত্যাদি থাকবে, send করার জন্য আমাদের অপেক্ষা করতে হবে, এবং এটি response body থেকে আলাদাভাবে deliver করা যেতে পারে। বিশেষ করে body অনেক বড় হলে, এর সবটা আসতে কিছু সময় লাগতে পারে। যেহেতু response এর _পুরোটা_ আসার জন্য আমাদের অপেক্ষা করতে হবে, তাই `text` method ও async।

আমাদের এই দুটি future কে explicitly await করতে হবে, কারণ Rust এ future গুলো _lazy_: `await` keyword দিয়ে না বলা পর্যন্ত এগুলো কিছুই করে না। (আসলে, Rust compiler warning দেখাবে যদি আপনি future ব্যবহার না করেন।) এটি আপনাকে Chapter 13 এর section [Processing a Series of Items With Iterators][iterators-lazy]<!-- ignore --> এ iterator নিয়ে করা আলোচনা মনে করিয়ে দিতে পারে। Iterator গুলো কিছুই করে না যতক্ষণ না আপনি তাদের `next` method call করেন—সরাসরি অথবা `for` loop বা `map` এর মতো method ব্যবহার করে যেগুলো under the hood এ `next` ব্যবহার করে। একইভাবে, future গুলোও explicitly না বলা পর্যন্ত কিছুই করে না। এই laziness Rust কে async code প্রয়োজন না হওয়া পর্যন্ত run করা এড়াতে সাহায্য করে।

> Note: এটি আগের chapter এ [Creating a New Thread with spawn][thread-spawn]<!--ignore--> এ `thread::spawn` ব্যবহার করার সময় আমরা যে behaviour দেখেছিলাম তার থেকে আলাদা, যেখানে অন্য thread এ pass করা closure টি সাথে সাথে run হতে শুরু করেছিল। এটি অন্যান্য language এর async approach থেকেও আলাদা। কিন্তু এটি Rust এর জন্য গুরুত্বপূর্ণ, এবং আমরা পরে দেখব কেন।

একবার `response_text` পেলে, আমরা `Html::parse` ব্যবহার করে এটিকে `Html` type এর instance এ parse করতে পারি। এখন আমাদের কাছে raw string এর পরিবর্তে এমন একটি data type আছে যা ব্যবহার করে আমরা HTML কে আরও সমৃদ্ধ data structure হিসেবে কাজ করতে পারি। বিশেষ করে, আমরা একটি নির্দিষ্ট CSS selector এর প্রথম instance খুঁজে বের করার জন্য `select_first` method ব্যবহার করতে পারি। `"title"` string pass করার মাধ্যমে, আমরা document এ প্রথম `<title>` element পাব, যদি থাকে। যেহেতু কোনো matching element নাও থাকতে পারে, তাই `select_first` একটি `Option<ElementRef>` return করে। অবশেষে, আমরা `Option::map` method ব্যবহার করি, যা আমাদের `Option` এ item থাকলে তার সাথে কাজ করার সুযোগ দেয়, এবং না থাকলে কিছুই করে না। (আমরা এখানে `match` expression ও ব্যবহার করতে পারতাম, কিন্তু `map` বেশি idiomatic)। `map` এ আমরা যে function টি supply করি, তার body তে `title_element` এর content পাওয়ার জন্য `inner_html` call করি, যা একটি `String`। সবশেষে, আমাদের কাছে একটি `Option<String>` থাকে।

লক্ষ্য করুন যে Rust এর `await` keyword আপনি await করছেন এমন expression এর _পরে_ বসে, আগে নয়। অর্থাৎ, এটি একটি _postfix_ keyword। আপনি যদি অন্য language এ `async` ব্যবহার করে থাকেন, তাহলে এটি আপনার অভ্যাসের থেকে ভিন্ন হতে পারে, কিন্তু Rust এ এটি method এর chain এর সাথে কাজ করা অনেক সহজ করে দেয়। ফলস্বরূপ, আমরা `page_url_for` এর body পরিবর্তন করে `trpl::get` এবং `text` function call গুলোকে `await` এর মাধ্যমে chain করতে পারি, যা Listing 17-2 এ দেখানো হয়েছে।

<Listing number="17-2" file-name="src/main.rs" caption="`await` keyword দিয়ে chaining করা">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

এর মাধ্যমে, আমরা সফলভাবে আমাদের প্রথম async function লিখেছি! `main` এ call করার জন্য কিছু code যোগ করার আগে, আমরা যা লিখেছি এবং এর মানে কী তা নিয়ে আরও একটু আলোচনা করি।

যখন Rust `async` keyword দিয়ে mark করা কোনো block দেখে, তখন এটি `Future` trait implement করা একটি unique, anonymous data type এ compile করে। যখন Rust `async` দিয়ে mark করা কোনো function দেখে, তখন এটি একটি non-async function এ compile করে যার body একটি async block। Async function এর return type হলো সেই anonymous data type এর type যা compiler সেই async block এর জন্য তৈরি করে।

সুতরাং, `async fn` লেখা return type এর _future_ return করা একটি function লেখার equivalent। Compiler এর কাছে, Listing 17-1 এ `async fn page_title` এর মতো একটি function definition একটি non-async function এর equivalent যা এভাবে define করা হয়েছে:

```rust
# extern crate trpl; // required for mdbook test
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> + '_ {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

আসুন transformed version এর প্রত্যেকটি অংশ আলোচনা করি:

- এটি `impl Trait` syntax ব্যবহার করে যা আমরা Chapter 10 এর [“Traits as Parameters”][impl-trait]<!-- ignore --> section এ আলোচনা করেছিলাম।
- Return করা trait টি একটি `Future` যার associated type হলো `Output`। লক্ষ্য করুন যে `Output` type হলো `Option<String>`, যা `page_title` এর `async fn` version এর original return type এর মতোই।
- Original function এর body তে call করা সব code একটি `async move` block এ wrap করা আছে। মনে রাখবেন block গুলো expression। এই পুরো block টি হলো function থেকে return করা expression।
- এই async block টি একটি `Option<String>` type এর value তৈরি করে, যা উপরে বর্ণনা করা হয়েছে। সেই value return type এর `Output` type এর সাথে match করে। এটি আপনি আগে দেখা অন্যান্য block এর মতোই।
- Function এর নতুন body টি `async move` block, কারণ এটি কিভাবে `url` parameter ব্যবহার করে। (আমরা এই chapter এ `async` বনাম `async move` নিয়ে আরও অনেক আলোচনা করব।)
- Function এর নতুন version এর output type এ এমন একটি lifetime আছে যা আমরা আগে দেখিনি: `'_`। যেহেতু function টি এমন একটি future return করে যা reference কে refer করে—এই ক্ষেত্রে, `url` parameter থেকে আসা reference—আমাদের Rust কে বলতে হবে যে আমরা সেই reference কে include করতে চাই। এখানে আমাদের lifetime এর নাম দিতে হবে না, কারণ Rust জানে যে এখানে শুধুমাত্র একটি reference জড়িত থাকতে পারে, তবে আমাদের explicitly বলতে হবে যে resulting future সেই lifetime দ্বারা bound।

এখন আমরা `main` এ `page_title` call করতে পারি।

## Determining a Single Page’s Title

শুরু করার জন্য, আমরা শুধু একটি single page এর title পাব। Listing 17-3 এ, আমরা Chapter 12 এর [Accepting Command Line Arguments][cli-args]<!-- ignore --> section এ command line argument পাওয়ার জন্য যে pattern ব্যবহার করেছিলাম সেটি follow করি। তারপর আমরা প্রথম URL `page_title` এ pass করি এবং result await করি। যেহেতু future দ্বারা তৈরি value একটি `Option<String>`, তাই page এ `<title>` আছে কিনা তা হিসাব করার জন্য আমরা বিভিন্ন message print করতে একটি `match` expression ব্যবহার করি।

<Listing number="17-3" file-name="src/main.rs" caption="ব্যবহারকারী দ্বারা দেওয়া argument দিয়ে `main` থেকে `page_title` function call করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

দুর্ভাগ্যবশত, এই code compile হয় না। `await` keyword ব্যবহার করার একমাত্র জায়গা হলো async function বা block, এবং Rust বিশেষ `main` function কে `async` হিসেবে mark করতে দেবে না।

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

`main` function কে `async` হিসেবে mark করা যায় না কারণ async code এর একটি _runtime_ এর প্রয়োজন: একটি Rust crate যা asynchronous code execute করার detail manage করে। একটি program এর `main` function একটি runtime _initialize_ করতে পারে, কিন্তু এটি নিজে runtime _নয়_। (আমরা পরে দেখব কেন এমন হয়।) প্রত্যেক Rust program যা async code execute করে, সেখানে কমপক্ষে একটি জায়গা থাকে যেখানে runtime set up করা হয় এবং future গুলো execute করা হয়।

বেশিরভাগ language যা async support করে, তারা runtime bundle করে, কিন্তু Rust করে না। এর পরিবর্তে, অনেক ভিন্ন async runtime available আছে, যার প্রত্যেকটি তার target use case এর জন্য উপযুক্ত ভিন্ন trade off করে। উদাহরণস্বরূপ, অনেক CPU core এবং প্রচুর RAM সহ একটি high-throughput web server এর single core, অল্প RAM, এবং কোনো heap allocation ability ছাড়া একটি microcontroller এর থেকে অনেক ভিন্ন প্রয়োজন। এই runtime গুলো প্রদান করা crate গুলো file বা network I/O এর মতো common functionality এর async version ও supply করে।

এখানে, এবং এই chapter এর বাকি অংশে, আমরা `trpl` crate থেকে `run` function ব্যবহার করব, যা argument হিসেবে future নেয় এবং এটিকে completion পর্যন্ত run করে। Under the hood এ, `run` call করা একটি runtime set up করে যা pass করা future run করার জন্য ব্যবহার করা হয়। একবার future complete হয়ে গেলে, `run` future দ্বারা তৈরি করা value return করে।

আমরা `page_title` দ্বারা return করা future সরাসরি `run` এ pass করতে পারতাম, এবং একবার এটি complete হয়ে গেলে, আমরা resulting `Option<String>` এর উপর match করতে পারতাম, যেমনটা আমরা Listing 17-3 এ করার চেষ্টা করেছিলাম। তবে, এই chapter এর বেশিরভাগ উদাহরণ (এবং real world এর বেশিরভাগ async code) এর জন্য, আমরা শুধু একটি async function call এর চেয়ে আরও বেশি কিছু করব, তাই এর পরিবর্তে আমরা একটি `async` block pass করব এবং Listing 17-4 এ দেখানো অনুযায়ী `page_title` call এর result explicitly await করব।

<Listing number="17-4" caption="`trpl::run` দিয়ে একটি async block await করা" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook test does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

যখন আমরা এই code run করি, তখন আমরা প্রথমে যা আশা করেছিলাম তেমন behavior পাই:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-04
cargo build # skip all the build noise
cargo run https://www.rust-lang.org
# copy the output here
-->

```console
$ cargo run -- https://www.rust-lang.org
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
The title for https://www.rust-lang.org was
            Rust Programming Language
```

অবশেষে, আমরা কিছু working async code পেলাম! কিন্তু দুটি site এর মধ্যে race এর code যোগ করার আগে, চলুন future কিভাবে কাজ করে সেদিকে মনোযোগ দেই।

প্রত্যেকটি _await point_—অর্থাৎ, code এ যেখানে `await` keyword ব্যবহার করা হয়েছে—সেখানে এমন একটি place represent করে যেখানে control runtime এ ফিরিয়ে দেওয়া হয়। সেটা কাজ করার জন্য, Rust কে async block এর সাথে জড়িত state এর track রাখতে হবে যাতে runtime অন্য কোনো কাজ শুরু করতে পারে এবং তারপর যখন প্রথমটি আবার চেষ্টা করার জন্য ready হবে তখন ফিরে আসতে পারে। এটি একটি invisible state machine, যেন আপনি প্রতিটি await point এ current state save করার জন্য এই enum টির মতো লিখেছেন:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

তবে, প্রত্যেক state এর মধ্যে transition করার code হাতে লেখা ক্লান্তিকর এবং error-prone হবে, বিশেষ করে যখন আপনাকে পরে code এ আরও functionality এবং state যোগ করতে হবে। সৌভাগ্যবশত, Rust compiler automatic ভাবে async code এর জন্য state machine data structure তৈরি এবং manage করে। Data structure এর চারপাশে borrowing এবং ownership এর normal rule গুলো এখনও প্রযোজ্য, এবং compiler ও আমাদের জন্য এগুলো check করে এবং কাজের error message দেয়। আমরা chapter এর পরে এগুলোর কিছু নিয়ে কাজ করব।

অবশেষে, কোনো কিছুকে এই state machine execute করতে হবে, এবং সেটি হলো একটি runtime। (এজন্য runtime নিয়ে investigate করার সময় আপনি হয়তো _executor_ এর reference দেখতে পারেন: executor হলো runtime এর সেই অংশ যা async code execute করার জন্য responsible)।

এখন আপনি বুঝতে পারছেন কেন compiler Listing 17-3 এ `main` কে async function বানাতে বাধা দিয়েছিল। যদি `main` একটি async function হতো, তাহলে `main` যে future return করত তার state machine manage করার জন্য অন্য কিছুর প্রয়োজন হতো, কিন্তু `main` হলো program এর starting point! এর পরিবর্তে, আমরা `main` এ `trpl::run` function call করেছি একটি runtime set up করার জন্য এবং `async` block দ্বারা return করা future কে `Ready` return করা পর্যন্ত run করার জন্য।

> Note: কিছু runtime macro প্রদান করে যাতে আপনি async `main` function লিখতে পারেন। সেই macro গুলো `async fn main() { ... }` কে একটি normal `fn main` হিসেবে rewrite করে, যা Listing 17-5 এ আমরা manually যা করেছি সেটিই করে: একটি function call করে যা completion পর্যন্ত future run করে যেভাবে `trpl::run` করে।

এখন চলুন এই অংশগুলোকে একসাথে করি এবং দেখি কিভাবে আমরা concurrent code লিখতে পারি।

### Racing Our Two URLs Against Each Other

Listing 17-5 এ, আমরা command line থেকে pass করা দুটি ভিন্ন URL দিয়ে `page_title` call করি এবং তাদের race করাই।

<Listing number="17-5" caption="" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

আমরা user দ্বারা দেওয়া প্রত্যেক URL এর জন্য `page_title` call করার মাধ্যমে শুরু করি। আমরা resulting future গুলোকে `title_fut_1` এবং `title_fut_2` হিসেবে save করি। মনে রাখবেন, এগুলো এখনও কিছু করে না, কারণ future গুলো lazy এবং আমরা এখনও await করিনি। তারপর আমরা future গুলোকে `trpl::race` এ pass করি, যা return করে যে pass করা future গুলোর মধ্যে কোনটি আগে শেষ হয়েছে।

> Note: Under the hood, `race` একটি আরও general function, `select` এর উপর built করা, যা আপনি real-world Rust code এ আরও বেশি দেখতে পাবেন। একটি `select` function অনেক কাজ করতে পারে যা `trpl::race` function পারে না, তবে এর কিছু additional complexity ও আছে যা আমরা এখন skip করতে পারি।

যেকোনো future legitimately "win" করতে পারে, তাই `Result` return করাটা যুক্তিযুক্ত নয়। এর পরিবর্তে, `race` একটি type return করে যা আমরা আগে দেখিনি, `trpl::Either`। `Either` type টি কিছুটা `Result` এর মতো কারণ এর দুটি case আছে। তবে, `Result` এর বিপরীতে, `Either` এ success বা failure এর কোনো ধারণা নেই। এর পরিবর্তে, এটি "এক বা অন্য" indicate করার জন্য `Left` এবং `Right` ব্যবহার করে:

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

`race` function প্রথম argument win করলে সেই future এর output দিয়ে `Left` return করে, এবং যদি দ্বিতীয় future argument win করে তাহলে _সেটার_ output দিয়ে `Right` return করে। এটি function call করার সময় argument গুলো যে order এ থাকে তার সাথে match করে: প্রথম argument দ্বিতীয় argument এর বাম দিকে থাকে।

আমরা pass করা URL return করার জন্য `page_title` ও update করি। এভাবে, যদি প্রথম return করা page এর `<title>` না থাকে, তাহলে আমরা meaningful message print করতে পারব। সেই information available থাকার সাথে, আমরা `println!` output update করে শেষ করি যাতে indicate করা যায় কোন URL আগে শেষ হয়েছে এবং সেই URL এর web page এর `<title>` কি, যদি থাকে।

আপনি এখন একটি ছোট working web scraper তৈরি করেছেন! কিছু URL select করুন এবং command line tool টি run করুন। আপনি হয়তো আবিষ্কার করবেন যে কিছু site নিয়মিতভাবে অন্যগুলোর চেয়ে দ্রুত, আবার কিছু ক্ষেত্রে run to run এর উপর নির্ভর করে fast site টি পরিবর্তন হয়। এর চেয়েও গুরুত্বপূর্ণ, আপনি future এর সাথে কাজ করার basics শিখেছেন, তাই এখন আমরা async দিয়ে আরও কি কি করা যেতে পারে তা নিয়ে আলোচনা করতে পারি।

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[iterators-lazy]: ch13-02-iterators.html
[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[cli-args]: ch12-01-accepting-command-line-arguments.html

<!-- TODO: map source link version to version of Rust? -->

[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs
