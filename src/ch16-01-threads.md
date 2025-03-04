## থ্রেড ব্যবহার করে একই সাথে কোড চালানো

বেশিরভাগ current অপারেটিং সিস্টেমে, একটি executed প্রোগ্রামের কোড একটি _প্রসেস_-এ run হয় এবং অপারেটিং সিস্টেম একসাথে multiple প্রসেস manage করবে। একটি প্রোগ্রামের মধ্যে, আপনার independent অংশগুলোও থাকতে পারে যেগুলো simultaneously চলে। এই independent অংশগুলো run করে এমন feature গুলোকে _থ্রেড_ বলা হয়। উদাহরণস্বরূপ, একটি ওয়েব সার্ভারে multiple থ্রেড থাকতে পারে যাতে এটি একই সময়ে একাধিক অনুরোধে respond করতে পারে।

আপনার প্রোগ্রামের computation-কে multiple থ্রেডে বিভক্ত করে একই সময়ে multiple task চালানো পারফরম্যান্স improve করতে পারে, তবে এটি complexity-ও বাড়িয়ে দেয়। যেহেতু থ্রেডগুলো simultaneously চলতে পারে, তাই বিভিন্ন থ্রেডে আপনার কোডের অংশগুলো কোন ক্রমে চলবে সে সম্পর্কে কোনো inherent গ্যারান্টি নেই। এটি সমস্যার দিকে নিয়ে যেতে পারে, যেমন:

-   রেস কন্ডিশন, যেখানে থ্রেডগুলো একটি অসঙ্গত ক্রমে ডেটা বা রিসোর্স অ্যাক্সেস করছে
-   ডেডলক, যেখানে দুটি থ্রেড একে অপরের জন্য অপেক্ষা করছে, উভয় থ্রেডকে continue করা থেকে বিরত রাখছে
-   বাগ যেগুলো শুধুমাত্র certain পরিস্থিতিতে ঘটে এবং reliably reproduce এবং ঠিক করা কঠিন

Rust থ্রেড ব্যবহারের negative effect গুলো প্রশমিত করার চেষ্টা করে, কিন্তু একটি multithreaded context-এ প্রোগ্রামিং করার জন্য এখনও careful thought প্রয়োজন এবং এর জন্য একটি কোড স্ট্রাকচার প্রয়োজন যা single thread-এ চলা প্রোগ্রামগুলোর থেকে আলাদা।

প্রোগ্রামিং ল্যাঙ্গুয়েজগুলো কয়েকটি different উপায়ে থ্রেড implement করে এবং অনেক অপারেটিং সিস্টেম একটি API provide করে যা ল্যাঙ্গুয়েজ new থ্রেড তৈরি করার জন্য কল করতে পারে। Rust standard library থ্রেড ইমপ্লিমেন্টেশনের একটি _1:1_ মডেল ব্যবহার করে, যেখানে একটি প্রোগ্রাম প্রতি language থ্রেডের জন্য একটি অপারেটিং সিস্টেম থ্রেড ব্যবহার করে। এমন কিছু crate রয়েছে যেগুলো থ্রেডিংয়ের অন্যান্য মডেল implement করে যা 1:1 মডেলের সাথে different trade-off করে। (Rust-এর async সিস্টেম, যা আমরা அடுத்த chapter-এ দেখব, concurrency-র জন্য আরেকটি অ্যাপ্রোচ প্রদান করে।)

### `spawn`-এর সাহায্যে একটি New Thread তৈরি করা

একটি new thread তৈরি করতে, আমরা `thread::spawn` ফাংশনটি কল করি এবং এটিকে একটি ক্লোজার (আমরা Chapter 13-এ ক্লোজার নিয়ে আলোচনা করেছি) pass করি যেখানে new thread-এ আমরা যে কোডটি চালাতে চাই তা থাকে। Listing 16-1-এর উদাহরণটি একটি main thread থেকে কিছু text এবং একটি new thread থেকে অন্য text প্রিন্ট করে:

<Listing number="16-1" file-name="src/main.rs" caption="Main thread-এ অন্য কিছু প্রিন্ট করার সময় একটি new thread-এ কিছু প্রিন্ট করা">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

লক্ষ্য করুন যে যখন একটি Rust প্রোগ্রামের main thread complete হয়, তখন সমস্ত spawned thread গুলো বন্ধ হয়ে যায়, সেগুলো running শেষ করুক বা না করুক। এই প্রোগ্রাম থেকে আউটপুট প্রতিবার একটু ভিন্ন হতে পারে, তবে এটি নিম্নলিখিতগুলোর মতো দেখাবে:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

`Thread::sleep`-এর কলগুলো একটি থ্রেডকে অল্প সময়ের জন্য তার execution stop করতে বাধ্য করে, অন্য একটি থ্রেডকে চলতে দেয়। থ্রেডগুলো সম্ভবত পালাক্রমে চলবে, তবে এটির গ্যারান্টি নেই: এটি নির্ভর করে আপনার অপারেটিং সিস্টেম কীভাবে থ্রেডগুলোকে schedule করে তার উপর। এই run-এ, main thread টি প্রথমে প্রিন্ট করেছে, যদিও spawned thread থেকে প্রিন্ট স্টেটমেন্টটি কোডে প্রথমে appear করে। এবং যদিও আমরা spawned thread-টিকে `i` `9` না হওয়া পর্যন্ত প্রিন্ট করতে বলেছিলাম, main thread বন্ধ হওয়ার আগে এটি শুধুমাত্র `5`-এ পৌঁছেছে।

যদি আপনি এই কোডটি চালান এবং শুধুমাত্র main thread থেকে আউটপুট দেখেন, অথবা কোনো overlap না দেখেন, তাহলে range-গুলোতে সংখ্যা বাড়ানোর চেষ্টা করুন যাতে অপারেটিং সিস্টেমের থ্রেডগুলোর মধ্যে switch করার আরও সুযোগ তৈরি হয়।

### `join` Handle ব্যবহার করে সমস্ত Thread শেষ হওয়ার জন্য অপেক্ষা করা

Listing 16-1-এর কোডটি শুধুমাত্র main thread শেষ হওয়ার কারণে বেশিরভাগ সময় spawned thread-টিকে prematurely থামিয়ে দেয় না, কিন্তু যেহেতু থ্রেডগুলো কোন ক্রমে run করে সে সম্পর্কে কোনো গ্যারান্টি নেই, তাই আমরা এটাও গ্যারান্টি দিতে পারি না যে spawned thread টি আদৌ run করতে পারবে!

আমরা `thread::spawn`-এর return value একটি variable-এ save করে spawned thread-টি না চলা বা prematurely শেষ হওয়ার সমস্যাটি সমাধান করতে পারি। `Thread::spawn`-এর return type হল `JoinHandle`। একটি `JoinHandle` হল একটি owned value যা, যখন আমরা এটিতে `join` method কল করি, তখন এর থ্রেড শেষ হওয়ার জন্য অপেক্ষা করবে। Listing 16-2 দেখায় কিভাবে Listing 16-1-এ তৈরি করা থ্রেডের `JoinHandle` ব্যবহার করতে হয় এবং `main` exit করার আগে spawned thread টি শেষ হয়েছে তা নিশ্চিত করতে `join` কল করতে হয়:

<Listing number="16-2" file-name="src/main.rs" caption="থ্রেডটি সম্পূর্ণ run হয়েছে তা নিশ্চিত করতে `thread::spawn` থেকে একটি `JoinHandle` সংরক্ষণ করা">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

হ্যান্ডেলে `join` কল করা currently running থ্রেডটিকে ব্লক করে যতক্ষণ না হ্যান্ডেল দ্বারা represented থ্রেডটি terminate হয়। একটি থ্রেডকে _ব্লক_ করার অর্থ হল সেই থ্রেডটিকে কাজ করা বা exit করা থেকে বিরত রাখা। যেহেতু আমরা main thread-এর `for` লুপের পরে `join`-এর কলটি রেখেছি, তাই Listing 16-2 চালালে এইরকম আউটপুট produce হওয়া উচিত:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

দুটি থ্রেড alternating ভাবে চলতে থাকে, কিন্তু main thread টি `handle.join()`-এ কলের কারণে অপেক্ষা করে এবং spawned thread শেষ না হওয়া পর্যন্ত শেষ হয় না।

কিন্তু আসুন দেখি কি হয় যখন আমরা পরিবর্তে `main`-এ `for` লুপের আগে `handle.join()` move করি, এইভাবে:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

Main thread টি spawned thread শেষ হওয়ার জন্য অপেক্ষা করবে এবং তারপর তার `for` লুপ চালাবে, তাই আউটপুটটি আর interleaved হবে না, যেমনটি এখানে দেখানো হয়েছে:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

ছোট details, যেমন `join` কোথায় কল করা হয়েছে, তা আপনার থ্রেডগুলো একই সময়ে run করবে কিনা তা প্রভাবিত করতে পারে।

### থ্রেডগুলোর সাথে `move` ক্লোজার ব্যবহার করা

আমরা প্রায়শই `thread::spawn`-এ pass করা closure-গুলোর সাথে `move` keyword ব্যবহার করব কারণ closure টি তখন environment থেকে ব্যবহৃত value গুলোর ownership নেবে, এইভাবে সেই value গুলোর ownership এক থ্রেড থেকে অন্য থ্রেডে transfer করবে। Chapter 13-এ [“Reference Capture করা বা Ownership Move করা”][capture]-এ, আমরা closure-এর context-এ `move` নিয়ে আলোচনা করেছি। এখন, আমরা `move` এবং `thread::spawn`-এর মধ্যে interaction-এর উপর বেশি concentrate করব।

Listing 16-1-এ লক্ষ্য করুন যে আমরা `thread::spawn`-এ যে closure টি pass করি সেটি কোনো argument নেয় না: আমরা spawned thread-এর কোডে main thread থেকে কোনো ডেটা ব্যবহার করছি না। Spawned thread-এ main thread থেকে ডেটা ব্যবহার করার জন্য, spawned thread-এর closure-এর প্রয়োজনীয় value গুলো capture করতে হবে। Listing 16-3 main thread-এ একটি vector তৈরি করার এবং অন্য thread-এ এটি ব্যবহার করার একটি প্রচেষ্টা দেখায়। যাইহোক, এটি এখনও কাজ করবে না, যেমনটি আপনি একটু পরেই দেখতে পাবেন।

<Listing number="16-3" file-name="src/main.rs" caption="অন্য থ্রেডে main thread দ্বারা তৈরি একটি vector ব্যবহার করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

Closure টি `v` ব্যবহার করে, তাই এটি `v` ক্যাপচার করবে এবং এটিকে closure-এর environment-এর অংশ করে তুলবে। যেহেতু `thread::spawn` এই closure-টিকে একটি new thread-এ চালায়, তাই আমাদের সেই new thread-এর মধ্যে `v` অ্যাক্সেস করতে সক্ষম হওয়া উচিত। কিন্তু যখন আমরা এই উদাহরণটি compile করি, তখন আমরা নিম্নলিখিত error টি পাই:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust _infer_ করে কিভাবে `v` ক্যাপচার করতে হবে, এবং যেহেতু `println!`-এর শুধুমাত্র `v`-এর একটি reference প্রয়োজন, তাই closure টি `v` borrow করার চেষ্টা করে। যাইহোক, একটি সমস্যা আছে: Rust বলতে পারে না spawned thread টি কতক্ষণ চলবে, তাই এটি জানে না যে `v`-এর reference সব সময় valid থাকবে কিনা।

Listing 16-4 এমন একটি scenario provide করে যেখানে `v`-এর একটি reference থাকার সম্ভাবনা বেশি যা valid হবে না:

<Listing number="16-4" file-name="src/main.rs" caption="একটি থ্রেড যেখানে একটি ক্লোজার রয়েছে যা `v`-এর একটি reference ক্যাপচার করার চেষ্টা করে এমন একটি main thread থেকে যা `v` ড্রপ করে">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

যদি Rust আমাদের এই কোডটি চালানোর অনুমতি দিত, তাহলে spawned thread-টি immediately background-এ চলে যাওয়ার সম্ভাবনা থাকত, আদৌ না চলে। Spawned thread-টির ভিতরে `v`-এর একটি reference রয়েছে, কিন্তু main thread অবিলম্বে `v` ড্রপ করে, Chapter 15-এ আলোচনা করা `drop` ফাংশনটি ব্যবহার করে। তারপর, যখন spawned thread execute করা শুরু করে, তখন `v` আর valid থাকে না, তাই এটির একটি reference-ও invalid। ওহ না!

Listing 16-3-এর compiler error ঠিক করতে, আমরা error message-এর পরামর্শ ব্যবহার করতে পারি:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

Closure-এর আগে `move` keyword যোগ করে, আমরা closure-টিকে এটি যে value গুলো ব্যবহার করছে সেগুলোর ownership নিতে বাধ্য করি, Rust-কে infer করার অনুমতি দেওয়ার পরিবর্তে যে এটির value গুলো borrow করা উচিত। Listing 16-5-এ দেখানো Listing 16-3-এর modification টি compile হবে এবং আমরা যেভাবে চাই সেভাবে চলবে:

<Listing number="16-5" file-name="src/main.rs" caption="`move` keyword ব্যবহার করে একটি closure-কে এটি যে value গুলো ব্যবহার করে সেগুলোর ownership নিতে বাধ্য করা">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

আমরা Listing 16-4-এর কোডটি ঠিক করার জন্য একই কাজ করার চেষ্টা করতে প্রলুব্ধ হতে পারি যেখানে main thread একটি `move` closure ব্যবহার করে `drop` কল করেছে। যাইহোক, এই fix কাজ করবে না কারণ Listing 16-4 যা করার চেষ্টা করছে তা একটি ভিন্ন কারণে অনুমোদিত নয়। যদি আমরা closure-এ `move` যোগ করি, তাহলে আমরা `v`-কে closure-এর environment-এ move করব এবং আমরা main thread-এ এটিতে আর `drop` কল করতে পারব না। পরিবর্তে আমরা এই compiler error টি পাব:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Rust-এর ownership rule গুলো আবার আমাদের বাঁচিয়েছে! আমরা Listing 16-3-এর কোড থেকে একটি error পেয়েছি কারণ Rust রক্ষণশীল ছিল এবং thread-এর জন্য শুধুমাত্র `v` ধার করছিল, যার অর্থ main thread তাত্ত্বিকভাবে spawned thread-এর reference-কে invalidate করতে পারত। Rust-কে `v`-এর ownership spawned thread-এ move করতে বলে, আমরা Rust-কে গ্যারান্টি দিচ্ছি যে main thread আর `v` ব্যবহার করবে না। যদি আমরা Listing 16-4-কে একইভাবে পরিবর্তন করি, তাহলে আমরা ownership rule গুলো লঙ্ঘন করছি যখন আমরা main thread-এ `v` ব্যবহার করার চেষ্টা করি। `Move` keyword Rust-এর borrowing-এর রক্ষণশীল default-কে override করে; এটি আমাদের ownership rule গুলো লঙ্ঘন করতে দেয় না।

থ্রেড এবং থ্রেড API-এর একটি basic understanding-এর সাথে, আসুন দেখি আমরা থ্রেড দিয়ে কী _করতে পারি_।

[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
