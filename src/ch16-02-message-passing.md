## থ্রেডগুলোর মধ্যে ডেটা ট্রান্সফার করতে Message Passing ব্যবহার করা

নিরাপদ concurrency নিশ্চিত করার জন্য একটি ক্রমবর্ধমান জনপ্রিয় অ্যাপ্রোচ হল _মেসেজ পাসিং_, যেখানে থ্রেড বা অ্যাক্টররা একে অপরের কাছে ডেটাযুক্ত মেসেজ পাঠিয়ে communicate করে। [Go ল্যাঙ্গুয়েজ ডকুমেন্টেশন](https://golang.org/doc/effective_go.html#concurrency) থেকে একটি স্লোগানে এই ধারণাটি হল: “মেমরি শেয়ার করে communicate করবেন না; পরিবর্তে, communicate করে মেমরি শেয়ার করুন।”

মেসেজ-সেন্ডিং concurrency সম্পন্ন করার জন্য, Rust-এর standard library চ্যানেলগুলোর একটি ইমপ্লিমেন্টেশন provide করে। একটি _চ্যানেল_ হল একটি general প্রোগ্রামিং কনসেপ্ট যার মাধ্যমে ডেটা এক থ্রেড থেকে অন্য থ্রেডে পাঠানো হয়।

আপনি প্রোগ্রামিং-এ একটি চ্যানেলকে জলের একটি দিকনির্দেশক চ্যানেলের মতো কল্পনা করতে পারেন, যেমন একটি স্রোত বা একটি নদী। আপনি যদি একটি রাবার হাঁসের মতো কিছু নদীতে রাখেন তবে এটি জলপথের শেষ পর্যন্ত downstream-এ চলে যাবে।

একটি চ্যানেলের দুটি অংশ রয়েছে: একটি ট্রান্সমিটার এবং একটি রিসিভার। ট্রান্সমিটার অংশটি হল upstream location যেখানে আপনি নদীতে রাবার হাঁস রাখেন এবং রিসিভার অংশটি হল যেখানে রাবার হাঁসটি downstream-এ শেষ হয়। আপনার কোডের একটি অংশ ডেটা সহ ট্রান্সমিটারে মেথড কল করে যা আপনি পাঠাতে চান এবং অন্য অংশটি আগত মেসেজগুলোর জন্য রিসিভিং প্রান্তটি চেক করে। একটি চ্যানেলকে _বন্ধ_ বলা হয় যদি ট্রান্সমিটার বা রিসিভার অংশের যেকোনো একটি ড্রপ করা হয়।

এখানে, আমরা একটি প্রোগ্রাম তৈরি করব যেখানে value generate করতে এবং সেগুলোকে একটি চ্যানেলের নিচে পাঠানোর জন্য একটি থ্রেড থাকবে এবং অন্য একটি থ্রেড value গুলো receive করবে এবং সেগুলো প্রিন্ট করবে। ফিচারটি বোঝানোর জন্য আমরা একটি চ্যানেল ব্যবহার করে থ্রেডগুলোর মধ্যে simple value পাঠাব। একবার আপনি technique-টির সাথে পরিচিত হয়ে গেলে, আপনি যেকোনো থ্রেডের মধ্যে communicate করার জন্য চ্যানেলগুলো ব্যবহার করতে পারেন, যেমন একটি চ্যাট সিস্টেম বা এমন একটি সিস্টেম যেখানে অনেক থ্রেড একটি calculation-এর অংশগুলো সম্পাদন করে এবং অংশগুলো একটি থ্রেডে পাঠায় যা result গুলোকে একত্রিত করে।

প্রথমে, Listing 16-6-এ, আমরা একটি চ্যানেল তৈরি করব কিন্তু এটি দিয়ে কিছু করব না। মনে রাখবেন যে এটি এখনও compile হবে না কারণ Rust বলতে পারে না আমরা চ্যানেলের মাধ্যমে কী ধরনের value পাঠাতে চাই।

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

<span class="caption">Listing 16-6: একটি চ্যানেল তৈরি করা এবং দুটি অংশকে `tx` এবং `rx`-এ assign করা</span>

আমরা `mpsc::channel` ফাংশন ব্যবহার করে একটি new চ্যানেল তৈরি করি; `mpsc` মানে _multiple producer, single consumer_। সংক্ষেপে, Rust-এর standard library যেভাবে চ্যানেলগুলো implement করে তার মানে হল একটি চ্যানেলের একাধিক _সেন্ডিং_ প্রান্ত থাকতে পারে যেগুলো value produce করে কিন্তু শুধুমাত্র একটি _রিসিভিং_ প্রান্ত থাকতে পারে যা সেই value গুলোকে consume করে। কল্পনা করুন multiple stream একসাথে একটি বড় নদীতে প্রবাহিত হচ্ছে: যেকোনো stream-এর নিচে পাঠানো সবকিছু শেষে একটি নদীতে শেষ হবে। আমরা আপাতত একটি single producer দিয়ে শুরু করব, কিন্তু যখন আমরা এই উদাহরণটি কাজ করাব তখন আমরা multiple producer যোগ করব।

`Mpsc::channel` ফাংশনটি একটি tuple রিটার্ন করে, যার প্রথম element টি হল sending end—ট্রান্সমিটার—এবং দ্বিতীয় element টি হল receiving end—রিসিভার। `Tx` এবং `rx` abbreviation গুলো traditionally অনেক ক্ষেত্রে যথাক্রমে _ট্রান্সমিটার_ এবং _রিসিভার_-এর জন্য ব্যবহৃত হয়, তাই আমরা আমাদের variable গুলোর নাম সেই অনুযায়ী রাখি প্রতিটি প্রান্ত নির্দেশ করার জন্য। আমরা একটি `let` স্টেটমেন্ট ব্যবহার করছি একটি প্যাটার্নের সাথে যা tuple গুলোকে destructure করে; আমরা Chapter 19-এ `let` স্টেটমেন্ট এবং destructuring-এ প্যাটার্নের ব্যবহার নিয়ে আলোচনা করব। আপাতত, জেনে রাখুন যে এইভাবে একটি `let` স্টেটমেন্ট ব্যবহার করা `mpsc::channel` দ্বারা returned tuple-এর অংশগুলো extract করার একটি সুবিধাজনক উপায়।

আসুন ট্রান্সমিটিং প্রান্তটিকে একটি spawned thread-এ move করি এবং এটিকে একটি string পাঠাতে দিই যাতে spawned thread টি main thread-এর সাথে communicate করে, যেমনটি Listing 16-7-এ দেখানো হয়েছে। এটি নদীতে upstream-এ একটি রাবার হাঁস রাখার বা এক থ্রেড থেকে অন্য থ্রেডে একটি চ্যাট মেসেজ পাঠানোর মতো।

<Listing number="16-7" file-name="src/main.rs" caption="`tx`-কে একটি spawned thread-এ move করা এবং “hi” পাঠানো">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

</Listing>

আবারও, আমরা একটি new thread তৈরি করতে `thread::spawn` ব্যবহার করছি এবং তারপর `tx`-কে closure-এ move করতে `move` ব্যবহার করছি যাতে spawned thread-টি `tx`-এর owner হয়। Spawned thread-টির চ্যানেলের মাধ্যমে message পাঠাতে সক্ষম হওয়ার জন্য transmitter-এর owner হওয়া প্রয়োজন।

ট্রান্সমিটারের একটি `send` method রয়েছে যা আমরা যে value টি পাঠাতে চাই সেটি নেয়। `Send` method টি একটি `Result<T, E>` টাইপ রিটার্ন করে, তাই যদি রিসিভারটি ইতিমধ্যেই ড্রপ করা হয়ে থাকে এবং একটি value পাঠানোর কোনো জায়গা না থাকে, তাহলে send operation টি একটি error রিটার্ন করবে। এই উদাহরণে, আমরা error-এর ক্ষেত্রে panic করার জন্য `unwrap` কল করছি। কিন্তু একটি real application-এ, আমরা এটিকে সঠিকভাবে হ্যান্ডেল করব: proper error handling-এর জন্য strategy গুলো পর্যালোচনা করতে Chapter 9-এ ফিরে যান।

Listing 16-8-এ, আমরা main thread-এ রিসিভার থেকে value টি পাব। এটি নদীর শেষে জল থেকে রাবার হাঁস retrieve করা বা একটি চ্যাট মেসেজ receive করার মতো।

<Listing number="16-8" file-name="src/main.rs" caption="Main thread-এ “hi” value টি receive করা এবং এটি প্রিন্ট করা">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

</Listing>

রিসিভারের দুটি useful method রয়েছে: `recv` এবং `try_recv`। আমরা `recv` ব্যবহার করছি, _receive_-এর সংক্ষিপ্ত, যেটি main thread-এর execution-কে ব্লক করবে এবং চ্যানেলের নিচে একটি value পাঠানো না হওয়া পর্যন্ত অপেক্ষা করবে। একবার একটি value পাঠানো হলে, `recv` এটিকে একটি `Result<T, E>`-তে রিটার্ন করবে। যখন ট্রান্সমিটার বন্ধ হয়ে যায়, তখন `recv` একটি error রিটার্ন করবে এটা বোঝাতে যে আর কোনো value আসবে না।

`Try_recv` method টি ব্লক করে না, কিন্তু পরিবর্তে অবিলম্বে একটি `Result<T, E>` রিটার্ন করবে: যদি একটি message available থাকে তাহলে একটি `Ok` value যাতে message টি থাকবে এবং যদি এই মুহূর্তে কোনো message না থাকে তাহলে একটি `Err` value। `Try_recv` ব্যবহার করা useful যদি এই থ্রেডের message-এর জন্য অপেক্ষা করার সময় অন্য কাজ করার থাকে: আমরা একটি লুপ লিখতে পারি যা প্রতি কিছুক্ষণ অন্তর `try_recv` কল করে, যদি একটি message available থাকে তাহলে সেটিকে handle করে এবং অন্যথায় কিছুক্ষণ অন্য কাজ করে আবার check করে।

আমরা এই উদাহরণে সরলতার জন্য `recv` ব্যবহার করেছি; main thread-এর message-এর জন্য অপেক্ষা করা ছাড়া অন্য কোনো কাজ করার নেই, তাই main thread-কে ব্লক করা উপযুক্ত।

যখন আমরা Listing 16-8-এর কোডটি চালাই, তখন আমরা main thread থেকে প্রিন্ট করা value টি দেখতে পাব:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
```

দারুণ!

### চ্যানেল এবং Ownership Transference

Ownership rule গুলো message পাঠানোর ক্ষেত্রে একটি গুরুত্বপূর্ণ ভূমিকা পালন করে কারণ সেগুলো আপনাকে নিরাপদ, concurrent কোড লিখতে সাহায্য করে। Concurrent প্রোগ্রামিং-এ error প্রতিরোধ করা হল আপনার Rust প্রোগ্রামগুলো জুড়ে ownership সম্পর্কে চিন্তা করার সুবিধা। আসুন একটি experiment করি এটা দেখাতে যে কীভাবে চ্যানেল এবং ownership সমস্যা প্রতিরোধ করতে একসাথে কাজ করে: আমরা spawned thread-এ একটি `val` value ব্যবহার করার চেষ্টা করব _পরে_ যখন আমরা এটিকে চ্যানেলের নিচে পাঠিয়েছি। Listing 16-9-এর কোডটি compile করার চেষ্টা করুন এটা দেখতে যে কেন এই কোডটির অনুমতি নেই:

<Listing number="16-9" file-name="src/main.rs" caption="চ্যানেলের নিচে পাঠানোর পরে `val` ব্যবহার করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

</Listing>

এখানে, আমরা `tx.send`-এর মাধ্যমে চ্যানেলের নিচে পাঠানোর পরে `val` প্রিন্ট করার চেষ্টা করি। এটির অনুমতি দেওয়া একটি খারাপ ধারণা হবে: একবার value টি অন্য থ্রেডে পাঠানো হলে, সেই থ্রেডটি value টি আবার ব্যবহার করার চেষ্টা করার আগে এটিকে modify বা ড্রপ করতে পারে। সম্ভাব্যভাবে, অন্য থ্রেডের modification গুলো অসঙ্গত বা অস্তিত্বহীন ডেটার কারণে error বা unexpected result-এর কারণ হতে পারে। যাইহোক, আমরা যদি Listing 16-9-এর কোড compile করার চেষ্টা করি তাহলে Rust আমাদের একটি error দেয়:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

আমাদের concurrency-জনিত ভুল একটি compile time error ঘটিয়েছে। `Send` ফাংশনটি তার parameter-এর ownership নেয় এবং যখন value টি move করা হয়, তখন receiver এটির ownership নেয়। এটি আমাদের পাঠানো value টি accidental ভাবে আবার ব্যবহার করা থেকে বিরত রাখে; ownership system পরীক্ষা করে যে সবকিছু ঠিক আছে।

### একাধিক Value পাঠানো এবং Receiver-এর অপেক্ষা দেখা

Listing 16-8-এর কোডটি compile এবং run হয়েছিল, কিন্তু এটি আমাদের স্পষ্টভাবে দেখায়নি যে দুটি আলাদা থ্রেড চ্যানেলের মাধ্যমে একে অপরের সাথে কথা বলছে। Listing 16-10-এ আমরা কিছু modification করেছি যা প্রমাণ করবে যে Listing 16-8-এর কোডটি concurrently চলছে: spawned thread টি এখন multiple message পাঠাবে এবং প্রতিটি message-এর মধ্যে এক সেকেন্ডের জন্য pause করবে।

<Listing number="16-10" file-name="src/main.rs" caption="একাধিক message পাঠানো এবং প্রতিটির মধ্যে pause করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

</Listing>

এইবার, spawned thread-টিতে string-গুলোর একটি vector রয়েছে যা আমরা main thread-এ পাঠাতে চাই। আমরা সেগুলোর উপর iterate করি, প্রতিটি individually পাঠাই এবং প্রতিটি পাঠানোর মধ্যে `thread::sleep` ফাংশনটিকে `Duration` value 1 সেকেন্ড দিয়ে কল করে pause করি।

Main thread-এ, আমরা আর explicit ভাবে `recv` ফাংশনটি কল করছি না: পরিবর্তে, আমরা `rx`-কে একটি iterator হিসেবে treat করছি। প্রতিটি value receive করার জন্য, আমরা এটিকে প্রিন্ট করছি। যখন চ্যানেলটি বন্ধ হয়ে যায়, তখন iteration শেষ হয়ে যাবে।

Listing 16-10-এর কোডটি চালানোর সময়, আপনি প্রতিটি লাইনের মধ্যে 1-সেকেন্ড pause সহ নিম্নলিখিত আউটপুট দেখতে পাবেন:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: from
Got: the
Got: thread
```

যেহেতু main thread-এর `for` লুপে আমাদের কোনো কোড নেই যা pause বা delay করে, তাই আমরা বলতে পারি যে main thread টি spawned thread থেকে value receive করার জন্য অপেক্ষা করছে।

### ট্রান্সমিটারকে ক্লোন করে একাধিক Producer তৈরি করা

আগে আমরা উল্লেখ করেছি যে `mpsc` হল _multiple producer, single consumer_-এর জন্য একটি acronym। আসুন `mpsc` ব্যবহার করি এবং Listing 16-10-এর কোড expand করি multiple thread তৈরি করতে যেগুলো সবই একই receiver-এ value পাঠায়। আমরা ট্রান্সমিটারকে ক্লোন করে এটি করতে পারি, যেমনটি Listing 16-11-তে দেখানো হয়েছে:

<Listing number="16-11" file-name="src/main.rs" caption="একাধিক producer থেকে একাধিক message পাঠানো">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

</Listing>

এইবার, আমরা প্রথম spawned thread তৈরি করার আগে, আমরা ট্রান্সমিটারে `clone` কল করি। এটি আমাদের একটি new transmitter দেবে যা আমরা প্রথম spawned thread-এ pass করতে পারি। আমরা original transmitter-টিকে একটি second spawned thread-এ pass করি। এটি আমাদের দুটি থ্রেড দেয়, প্রতিটি এক রিসিভারে different message পাঠায়।

যখন আপনি কোডটি চালান, তখন আপনার আউটপুট এইরকম হওয়া উচিত:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

আপনি হয়তো value গুলো অন্য ক্রমে দেখতে পারেন, আপনার সিস্টেমের উপর নির্ভর করে। এটিই concurrency-কে interesting এবং সেইসাথে কঠিন করে তোলে। আপনি যদি `thread::sleep` নিয়ে experiment করেন, এটিকে different thread-এ বিভিন্ন value দেন, তাহলে প্রতিটি run আরও nondeterministic হবে এবং প্রতিবার different আউটপুট তৈরি করবে।

এখন আমরা দেখেছি কিভাবে চ্যানেলগুলো কাজ করে, আসুন concurrency-র একটি different method দেখি।
