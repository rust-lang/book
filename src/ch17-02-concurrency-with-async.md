## Applying Concurrency with Async

<!-- Old headings. Do not remove or links may break. -->

<a id="concurrency-with-async"></a>

এই বিভাগে, আমরা Chapter 16 এ thread দিয়ে সমাধান করা কিছু concurrency challenge এর জন্য async ব্যবহার করব। যেহেতু আমরা সেখানে অনেক গুরুত্বপূর্ণ ধারণা নিয়ে আলোচনা করেছি, তাই এই বিভাগে আমরা thread এবং future এর মধ্যে পার্থক্যগুলোর উপর focus করব।

অনেক ক্ষেত্রে, async ব্যবহার করে concurrency এর সাথে কাজ করার API গুলো thread ব্যবহারের API গুলোর মতোই। অন্য ক্ষেত্রে, সেগুলো বেশ আলাদা হয়ে যায়। এমনকি যখন thread এবং async এর মধ্যে API গুলো দেখতে একই রকম _মনে হয়_, তখনও তাদের behaviour ভিন্ন হয়—এবং তাদের performance characteristics প্রায় সবসময়ই ভিন্ন হয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="counting"></a>

### Creating a New Task with `spawn_task`

[Creating a New Thread with Spawn][thread-spawn]<!-- ignore --> এ আমরা যে প্রথম operation নিয়ে কাজ করেছিলাম তা হলো দুটি আলাদা thread এ count করা। চলুন async ব্যবহার করে একই কাজ করি। `trpl` crate `thread::spawn` API এর মতোই দেখতে `spawn_task` function এবং `thread::sleep` API এর async version `sleep` function supply করে। Listing 17-6 এ দেখানো হিসাবে, এই দুটি একসাথে ব্যবহার করে আমরা counting example implement করতে পারি।

<Listing number="17-6" caption="একটি main task অন্য কিছু print করার সময় অন্য কিছু print করার জন্য নতুন task তৈরি করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

আমাদের starting point হিসেবে, আমরা `trpl::run` দিয়ে আমাদের `main` function set up করি যাতে আমাদের top-level function async হতে পারে।

> Note: এই chapter এ এখন থেকে, প্রত্যেক উদাহরণে `main` এ `trpl::run` দিয়ে exactly একই wrapping code থাকবে, তাই আমরা `main` এর মতো এটি skip করব। আপনার code এ এটি include করতে ভুলবেন না!

তারপর আমরা সেই block এর মধ্যে দুটি loop লিখি, প্রত্যেকটিতে একটি `trpl::sleep` call আছে, যা পরবর্তী message send করার আগে আধা সেকেন্ড (500 milliseconds) অপেক্ষা করে। আমরা একটি loop `trpl::spawn_task` এর body তে এবং অন্যটি top-level `for` loop এ রাখি। আমরা `sleep` call এর পরে `await` ও যোগ করি।

এই code টি thread-based implementation এর মতোই behave করে—এই fact সহ যে আপনি যখন এটি run করেন তখন আপনার terminal এ message গুলো ভিন্ন order এ দেখতে পারেন:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

এই version টি main async block এর body তে `for` loop শেষ হওয়ার সাথে সাথেই stop হয়ে যায়, কারণ `spawn_task` দ্বারা spawn হওয়া task টি `main` function শেষ হওয়ার সাথে সাথে shut down হয়ে যায়। যদি আপনি চান যে এটি task এর completion পর্যন্ত run হোক, তাহলে প্রথম task টি complete হওয়ার জন্য wait করতে join handle ব্যবহার করতে হবে। Thread এর সাথে, আমরা thread run হওয়া শেষ না হওয়া পর্যন্ত “block” করার জন্য `join` method ব্যবহার করতাম। Listing 17-7 এ, আমরা একই কাজ করার জন্য `await` ব্যবহার করতে পারি, কারণ task handle নিজেই একটি future। এর `Output` type হলো একটি `Result`, তাই আমরা await করার পর এটিকে unwrap ও করি।

<Listing number="17-7" caption="একটি task কে completion পর্যন্ত run করার জন্য join handle এর সাথে `await` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

এই updated version টি _দুটি_ loop শেষ হওয়া পর্যন্ত run হয়।

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

এখন পর্যন্ত, এটা async এবং thread এর মতো একই basic outcome দেয়, শুধু ভিন্ন syntax ব্যবহার করে: join handle এর উপর `join` call করার পরিবর্তে `await` ব্যবহার করা, এবং `sleep` call await করা।

বড় পার্থক্য হলো এই কাজ করার জন্য আমাদের অন্য কোনো operating system thread spawn করার প্রয়োজন হয়নি। আসলে, এখানে আমাদের task spawn করার ও দরকার নেই। যেহেতু async block গুলো anonymous future এ compile হয়, তাই আমরা প্রতিটি loop কে async block এ রাখতে পারি এবং `trpl::join` function ব্যবহার করে runtime দিয়ে সেগুলোকে completion পর্যন্ত run করাতে পারি।

[Waiting for All Threads to Finishing Using `join` Handles][join-handles]<!-- ignore --> section এ, আমরা দেখিয়েছি কিভাবে `std::thread::spawn` call করার সময় return হওয়া `JoinHandle` type এ `join` method ব্যবহার করতে হয়। `trpl::join` function ও একই রকম, কিন্তু future এর জন্য। যখন আপনি এটিকে দুটি future দেন, তখন এটি একটি single নতুন future তৈরি করে যার output হলো আপনি pass করা প্রত্যেক future এর output এর tuple, একবার যখন তারা _দুজনেই_ complete হয়। সুতরাং, Listing 17-8 এ, আমরা `trpl::join` ব্যবহার করি `fut1` এবং `fut2` উভয়ের শেষ হওয়ার জন্য wait করার জন্য। আমরা `fut1` এবং `fut2` await _করি না_, বরং `trpl::join` দ্বারা তৈরি হওয়া নতুন future await করি। আমরা output ignore করি, কারণ এটি শুধুমাত্র দুটি unit value এর একটি tuple।

<Listing number="17-8" caption="দুটি anonymous future await করার জন্য `trpl::join` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

যখন আমরা এটি run করি, তখন আমরা দেখি দুটি future completion পর্যন্ত run হয়:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

এখন, আপনি সবসময় exact একই order দেখবেন, যা thread এর সাথে আমরা যা দেখেছিলাম তার থেকে অনেক আলাদা। কারণ হলো `trpl::join` function _fair_, মানে এটি প্রত্যেক future কে সমানভাবে check করে, তাদের মধ্যে alternate করে, এবং অন্যটি ready থাকলে একটিকে এগিয়ে যেতে দেয় না। Thread এর সাথে, operating system decide করে কোন thread কে check করবে এবং কতক্ষণ run করতে দেবে। Async Rust এর সাথে, runtime decide করে কোন task কে check করবে। (বাস্তবে, detail গুলো জটিল হয়ে যায় কারণ একটি async runtime under the hood এ concurrency manage করার অংশ হিসেবে operating system thread ব্যবহার করতে পারে, তাই fairness guarantee করা runtime এর জন্য আরও বেশি কাজ হতে পারে—তবে এটি এখনও সম্ভব!) Runtime কে কোনো নির্দিষ্ট operation এর জন্য fairness guarantee করতে হয় না, এবং তারা প্রায়ই fairness চান কিনা তা choose করার জন্য ভিন্ন API offer করে।

Future await করার এই variation গুলোর কিছু try করুন এবং দেখুন সেগুলো কী করে:

- Loop এর চারপাশে থাকা async block হয় remove করুন অথবা দুটোর থেকেই remove করুন।
- প্রত্যেক async block define করার সাথে সাথে await করুন।
- শুধুমাত্র প্রথম loop কে async block এ wrap করুন, এবং দ্বিতীয় loop এর body এর পরে resulting future await করুন।

Extra challenge হিসেবে, code run করার _আগে_ প্রত্যেক ক্ষেত্রে output কি হবে তা বের করতে পারেন কিনা দেখুন!

<!-- Old headings. Do not remove or links may break. -->

<a id="message-passing"></a>

### Counting Up on Two Tasks Using Message Passing

Future এর মধ্যে data share করাও পরিচিত হবে: আমরা আবারও message passing ব্যবহার করব, তবে এবার type এবং function গুলোর async version ব্যবহার করে। Thread-based এবং future-based concurrency এর মধ্যে কিছু গুরুত্বপূর্ণ পার্থক্য illustrate করার জন্য আমরা [Using Message Passing to Transfer Data Between Threads][message-passing-threads]<!-- ignore --> এর চেয়ে একটু ভিন্ন path নেব। Listing 17-9 এ, আমরা শুধুমাত্র একটি single async block দিয়ে শুরু করব—আলাদা thread spawn করার মতো কোনো আলাদা task spawn _করব না_।

<Listing number="17-9" caption="একটি async channel তৈরি করা এবং দুটি অংশ `tx` এবং `rx` assign করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

এখানে, আমরা `trpl::channel` ব্যবহার করি, যা Chapter 16 এ thread এর সাথে ব্যবহার করা multiple-producer, single-consumer channel API এর একটি async version। API এর async version টি thread-based version থেকে একটু আলাদা: এটি immutable receiver `rx` এর পরিবর্তে mutable receiver ব্যবহার করে এবং এর `recv` method সরাসরি value তৈরি করার পরিবর্তে এমন একটি future তৈরি করে যা আমাদের await করতে হবে। এখন আমরা sender থেকে receiver এ message send করতে পারি। লক্ষ্য করুন যে আমাদের আলাদা thread বা task spawn করতে হচ্ছে না; আমাদের শুধু `rx.recv` call await করতে হচ্ছে।

`std::mpsc::channel` এ synchronous `Receiver::recv` method একটি message না পাওয়া পর্যন্ত block করে। `trpl::Receiver::recv` method তা করে না, কারণ এটি async। Block করার পরিবর্তে, এটি runtime এ control back করে দেয় যতক্ষণ না message receive হয় অথবা channel এর send side close হয়ে যায়। বিপরীতে, আমরা `send` call await করি না, কারণ এটি block করে না। এটির প্রয়োজন নেই, কারণ আমরা যে channel এ send করছি সেটি unbounded।

> Note: যেহেতু এই সব async code একটি `trpl::run` call এর async block এ run হয়, তাই এর ভিতরের সবকিছু blocking avoid করতে পারে। তবে, এর _বাইরের_ code `run` function return করার জন্য block হবে। `trpl::run` function এর মূল বিষয় এটাই: এটি আপনাকে _choose_ করতে দেয় async code এর কিছু set এ কোথায় block করতে হবে, এবং sync এবং async code এর মধ্যে কোথায় transition করতে হবে। বেশিরভাগ async runtime এ, `run` কে আসলে ঠিক এই কারণে `block_on` বলা হয়।

এই উদাহরণে দুটি বিষয় লক্ষ্য করুন। প্রথমত, message সাথে সাথেই আসবে। দ্বিতীয়ত, যদিও আমরা এখানে future ব্যবহার করি, এখনও কোনো concurrency নেই। Listing এ সবকিছু sequentially হয়, ঠিক যেমন future involve না থাকলে হতো।

Listing 17-10 এ দেখানো হিসাবে, চলুন message এর একটি series send করে এবং তাদের মাঝে sleep করে প্রথম অংশটি address করি।

<!-- We cannot test this one because it never stops! -->

<Listing number="17-10" caption="async channel এ একাধিক message send এবং receive করা এবং প্রত্যেক message এর মধ্যে `await` দিয়ে sleep করা" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

Message send করা ছাড়াও, আমাদের সেগুলো receive করতে হবে। এই ক্ষেত্রে, যেহেতু আমরা জানি কতগুলো message আসছে, তাই আমরা manually চারবার `rx.recv().await` call করে তা করতে পারতাম। তবে, real world এ, আমরা সাধারণত কিছু _অজানা_ number এর message এর জন্য wait করব, তাই আমাদের wait করতে থাকতে হবে যতক্ষণ না আমরা determine করতে পারি যে আর কোনো message নেই।

Listing 16-10 এ, আমরা synchronous channel থেকে receive করা সব item process করার জন্য একটি `for` loop ব্যবহার করেছিলাম। তবে, Rust এ এখনও _asynchronous_ series এর item এর উপর `for` loop লেখার কোনো উপায় নেই, তাই আমাদের এমন একটি loop ব্যবহার করতে হবে যা আমরা আগে দেখিনি: `while let` conditional loop। এটি loop version এর `if let` construct যা আমরা section [Concise Control Flow with `if let` and `let else`][if-let]<!-- ignore --> এ দেখেছি। Loop টি ততক্ষণ execute হতে থাকবে যতক্ষণ না এটি specified pattern value এর সাথে match করতে থাকে।

`rx.recv` call একটি future তৈরি করে, যা আমরা await করি। Runtime future টি ready না হওয়া পর্যন্ত pause করবে। একবার message আসলে, future টি `Some(message)` তে resolve হবে যতবার message আসবে ততবার। যখন channel close হয়ে যায়, তখন _কোনো_ message এসেছে কিনা তা নির্বিশেষে, future টি `None` তে resolve হবে indicate করার জন্য যে আর কোনো value নেই এবং তাই আমাদের polling stop করা উচিত—মানে, await করা stop করা উচিত।

`while let` loop এই সব একসাথে করে। `rx.recv().await` call করার result যদি `Some(message)` হয়, তাহলে আমরা message এর access পাই এবং loop body তে এটি ব্যবহার করতে পারি, ঠিক যেমন আমরা `if let` এর সাথে করতে পারতাম। যদি result `None` হয়, তাহলে loop টি শেষ হয়ে যায়। যখনই loop complete হয়, তখনই এটি await point এ hit করে, তাই runtime এটি আবার pause করে যতক্ষণ না অন্য কোনো message আসে।

Code টি এখন সফলভাবে সব message send এবং receive করে। দুর্ভাগ্যবশত, এখনও কিছু সমস্যা আছে। প্রথমত, message গুলো আধা-সেকেন্ড বিরতিতে আসে না। প্রোগ্রাম শুরু করার 2 সেকেন্ড (2,000 milliseconds) পরে তারা একসাথে আসে। দ্বিতীয়ত, এই program টি কখনো exit ও হয় না! এর পরিবর্তে, এটি নতুন message এর জন্য সবসময় wait করতে থাকে। <span class="keystroke">ctrl-c</span> ব্যবহার করে আপনাকে এটি shut down করতে হবে।

আসুন প্রথমে পরীক্ষা করি কেন message গুলো প্রত্যেকটির মধ্যে delay না দিয়ে পুরো delay এর পর একসাথে আসে। একটি নির্দিষ্ট async block এর মধ্যে, code এ `await` keyword গুলো যে order এ appear করে, সেই order এই program run করার সময় execute ও হয়।

Listing 17-10 এ শুধুমাত্র একটি async block আছে, তাই এর ভিতরের সবকিছু linearly run হয়। এখনও কোনো concurrency নেই। সব `tx.send` call, `trpl::sleep` call এবং এর associated await point এর সাথে interspersed হয়। তারপর `while let` loop `recv` call এ কোনো `await` point এর মধ্য দিয়ে যায়।

আমরা যে behaviour চাই, যেখানে sleep delay প্রত্যেক message এর মাঝে হবে, তার জন্য আমাদের `tx` এবং `rx` operation গুলোকে তাদের নিজস্ব async block এ রাখতে হবে, যা Listing 17-11 এ দেখানো হয়েছে। তারপর runtime `trpl::join` ব্যবহার করে সেগুলোর প্রত্যেকটিকে আলাদাভাবে execute করতে পারে, ঠিক যেমন counting example এ করা হয়েছে। আবারও, আমরা `trpl::join` call করার result await করি, individual future গুলো নয়। যদি আমরা individual future গুলো sequentially await করতাম, তাহলে আমরা sequential flow এ ফিরে যেতাম—যা আমরা করতে _চাইছি_ না।

<!-- We cannot test this one because it never stops! -->

<Listing number="17-11" caption="`send` এবং `recv` কে তাদের নিজস্ব `async` block এ আলাদা করা এবং সেই block গুলোর future await করা" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

Listing 17-11 এ updated code এর সাথে, message গুলো 2 সেকেন্ড পর rush করে আসার পরিবর্তে 500-millisecond বিরতিতে print হয়।

তবে, program টি এখনও exit হয় না, কারণ `while let` loop `trpl::join` এর সাথে interact করে:

- `trpl::join` থেকে return হওয়া future শুধুমাত্র তখনই complete হয় যখন এর মধ্যে pass করা _দুটি_ future complete হয়ে যায়।
- `tx` future `vals` এ শেষ message send করার পর sleep করা শেষ করলে complete হয়।
- `rx` future যতক্ষণ না `while let` loop শেষ হয় ততক্ষণ পর্যন্ত complete হবে না।
- `while let` loop ততক্ষণ পর্যন্ত শেষ হবে না যতক্ষণ না `rx.recv` await করা `None` তৈরি করে।
- `rx.recv` await করা শুধুমাত্র তখনই `None` return করবে যখন channel এর অন্য end close হয়ে যায়।
- Channel শুধুমাত্র তখনই close হবে যদি আমরা `rx.close` call করি অথবা sender side, `tx`, drop হয়ে যায়।
- আমরা কোথাও `rx.close` call করি না, এবং `tx` outermost async block `trpl::run` এ pass করা শেষ না হওয়া পর্যন্ত drop হবে না।
- Block টি শেষ হতে পারে না কারণ এটি `trpl::join` complete হওয়ার জন্য block করা আছে, যা আমাদের এই list এর শুরুতে নিয়ে যায়।

আমরা manually কোথাও `rx.close` call করে close করতে পারতাম, কিন্তু এর কোনো মানে হয় না। কিছু arbitrary number এর message handle করার পর stop করলে program shut down হয়ে যেত, তবে আমরা message miss করতে পারতাম। `tx` function এর শেষ হওয়ার _আগে_ drop হয়েছে তা নিশ্চিত করার জন্য আমাদের অন্য কোনো উপায় দরকার।

এখন, যে async block এ আমরা message send করি সেটি শুধু `tx` borrow করে কারণ message send করার জন্য ownership এর প্রয়োজন নেই, কিন্তু যদি আমরা `tx` কে সেই async block এ move করতে পারতাম, তাহলে সেই block শেষ হওয়ার সাথে সাথেই এটি drop হয়ে যেত। Chapter 13 এর section [Capturing References or Moving Ownership][capture-or-move]<!-- ignore --> এ, আপনি শিখেছিলেন কিভাবে closure এর সাথে `move` keyword ব্যবহার করতে হয়, এবং Chapter 16 এর section [Using `move` Closures with Threads][move-threads]<!-- ignore --> এ আলোচনা করা হয়েছে, thread এর সাথে কাজ করার সময় closure এর মধ্যে data move করার প্রয়োজন হয়। একই basic dynamic async block এর জন্য ও প্রযোজ্য, তাই `move` keyword closure এর মতো async block এর সাথেও কাজ করে।

Listing 17-12 এ, আমরা message send করার জন্য ব্যবহৃত block কে `async` থেকে `async move` তে পরিবর্তন করি। যখন আমরা code এর _এই_ version run করি, তখন এটি শেষ message send এবং receive করার পর gracefully shut down হয়ে যায়।

<Listing number="17-12" caption="Listing 17-11 এর code এর একটি revision যা complete হলে সঠিকভাবে shut down হয়" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

এই async channel ও multiple-producer channel, তাই আমরা যদি একাধিক future থেকে message send করতে চাই তাহলে `tx` এ `clone` call করতে পারি, যা Listing 17-13 এ দেখানো হয়েছে।

<Listing number="17-13" caption="async block দিয়ে multiple producer ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

প্রথমে, আমরা `tx` clone করি, প্রথম async block এর বাইরে `tx1` তৈরি করি। আমরা `tx` এর মতো `tx1` কে সেই block এ move করি। তারপর, পরে, আমরা original `tx` কে একটি _নতুন_ async block এ move করি, যেখানে আমরা আরও message send করি একটু slower delay দিয়ে। আমরা এই নতুন async block টি message receive করার async block এর পরে রাখি, কিন্তু এটি এর আগেও যেতে পারত। মূল বিষয় হলো future গুলো যে order এ await করা হয়েছে, সেগুলো তৈরি করার order নয়।

Message send করার জন্য async block গুলো `async move` block হতে হবে যাতে `tx` এবং `tx1` উভয়ই সেই block গুলো শেষ হলে drop হয়। অন্যথায়, আমরা সেই infinite loop এ ফিরে যাব যা দিয়ে আমরা শুরু করেছিলাম। অবশেষে, আমরা additional future handle করার জন্য `trpl::join` থেকে `trpl::join3` তে switch করি।

এখন আমরা দুটি sending future থেকে সব message দেখতে পাচ্ছি, এবং যেহেতু sending future গুলো send করার পর সামান্য ভিন্ন delay ব্যবহার করে, তাই message গুলো সেই ভিন্ন বিরতিতে receive ও হয়।

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

এটা ভালো শুরু, কিন্তু এটি আমাদের শুধু handful future এর মধ্যে সীমাবদ্ধ করে: `join` দিয়ে দুটি, অথবা `join3` দিয়ে তিনটি। চলুন দেখি কিভাবে আমরা আরও বেশি future নিয়ে কাজ করতে পারি।

[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[join-handles]: ch16-01-threads.html#waiting-for-all-threads-to-finish-using-join-handles
[message-passing-threads]: ch16-02-message-passing.html
[if-let]: ch06-03-if-let.html
[capture-or-move]: ch13-01-closures.html#capturing-references-or-moving-ownership
[move-threads]: ch16-01-threads.html#using-move-closures-with-threads
