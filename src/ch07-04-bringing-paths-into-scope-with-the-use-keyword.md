## `use` কীওয়ার্ড দিয়ে Scope এ Paths আনা

ফাংশন কল করার জন্য path লিখতে বাধ্য হওয়া অসুবিধাজনক এবং পুনরাবৃত্তিমূলক মনে হতে পারে। Listing 7-7 এ, `add_to_waitlist` ফাংশনের জন্য absolute বা relative path বেছে নিই না কেন, যখনই আমরা `add_to_waitlist` কল করতে চেয়েছি তখনই আমাদের `front_of_house` এবং `hosting` ও নির্দিষ্ট করতে হয়েছিল। সৌভাগ্যবশত, এই প্রক্রিয়াটিকে সহজ করার একটি উপায় আছে: আমরা একবার `use` কীওয়ার্ড দিয়ে একটি path এর শর্টকাট তৈরি করতে পারি এবং তারপর scope এর অন্য সব জায়গায় ছোট নাম ব্যবহার করতে পারি।

Listing 7-11 এ, আমরা `eat_at_restaurant` ফাংশনের scope এ `crate::front_of_house::hosting` মডিউল নিয়ে আসি যাতে `eat_at_restaurant` এ `add_to_waitlist` ফাংশন কল করার জন্য আমাদের শুধু `hosting::add_to_waitlist` নির্দিষ্ট করতে হয়।

<Listing number="7-11" file-name="src/lib.rs" caption="`use` দিয়ে একটি মডিউলকে scope এ নিয়ে আসা">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

scope এ `use` এবং একটি path যোগ করা ফাইল সিস্টেমে একটি সিম্বলিক লিঙ্ক তৈরি করার অনুরূপ। crate root এ `use crate::front_of_house::hosting` যোগ করার মাধ্যমে, `hosting` এখন সেই scope এর মধ্যে একটি বৈধ নাম, ঠিক যেন `hosting` মডিউলটি crate root এ সংজ্ঞায়িত করা হয়েছে। `use` দিয়ে scope এ আনা paths গুলো অন্য যেকোনো paths এর মতো গোপনীয়তাও পরীক্ষা করে।

মনে রাখবেন যে `use` শুধুমাত্র সেই নির্দিষ্ট scope এর জন্য শর্টকাট তৈরি করে যেখানে `use` ঘটে। Listing 7-12 `eat_at_restaurant` ফাংশনটিকে `customer` নামের একটি নতুন চাইল্ড মডিউলে move করে, যা তখন `use` স্টেটমেন্টের থেকে একটি ভিন্ন scope, তাই ফাংশন বডি কম্পাইল হবে না।

<Listing number="7-12" file-name="src/lib.rs" caption="একটি `use` স্টেটমেন্ট শুধুমাত্র সেই scope এই প্রযোজ্য যেখানে এটি আছে">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

কম্পাইলার ত্রুটি দেখায় যে শর্টকাটটি `customer` মডিউলের মধ্যে আর প্রযোজ্য নয়:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

লক্ষ্য করুন যে একটি সতর্কতাও রয়েছে যে `use` তার scope এ আর ব্যবহার করা হচ্ছে না! এই সমস্যাটি সমাধান করতে, `use` টিকে `customer` মডিউলের ভিতরে move করুন বা চাইল্ড `customer` মডিউলের ভিতরে `super::hosting` দিয়ে প্যারেন্ট মডিউলে শর্টকাটটি উল্লেখ করুন।

### Idiomatic `use` Paths তৈরি করা

Listing 7-11 এ, আপনি হয়তো ভেবেছিলেন কেন আমরা `use
crate::front_of_house::hosting` নির্দিষ্ট করেছি এবং তারপর `eat_at_restaurant` এ `hosting::add_to_waitlist` কল করেছি, `add_to_waitlist` ফাংশনটিকে scope এ আনার জন্য `use` path টিকে শেষ পর্যন্ত নির্দিষ্ট করার পরিবর্তে, Listing 7-13 এর মতো একই ফলাফল অর্জনের জন্য।

<Listing number="7-13" file-name="src/lib.rs" caption="`use` দিয়ে scope এ `add_to_waitlist` ফাংশন আনা, যা অ-idiomatic">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

যদিও Listing 7-11 এবং Listing 7-13 উভয়ই একই কাজ করে, Listing 7-11 হল `use` দিয়ে একটি ফাংশনকে scope এ আনার idiomatic উপায়। `use` দিয়ে ফাংশনের প্যারেন্ট মডিউলটিকে scope এ আনার অর্থ হল ফাংশন কল করার সময় আমাদের প্যারেন্ট মডিউলটি নির্দিষ্ট করতে হবে। ফাংশন কল করার সময় প্যারেন্ট মডিউলটি নির্দিষ্ট করা স্পষ্ট করে তোলে যে ফাংশনটি স্থানীয়ভাবে সংজ্ঞায়িত নয়, তবে পুরো path এর পুনরাবৃত্তি কমিয়ে দেয়। Listing 7-13 এর কোডটি অস্পষ্ট যে `add_to_waitlist` কোথায় সংজ্ঞায়িত করা হয়েছে।

অন্যদিকে, `use` দিয়ে structs, enums এবং অন্যান্য আইটেম আনার সময়, সম্পূর্ণ path টি নির্দিষ্ট করা idiomatic। Listing 7-14 একটি বাইনারি crate এর scope এ standard library এর `HashMap` struct আনার idiomatic উপায় দেখায়।

<Listing number="7-14" file-name="src/main.rs" caption="একটি idiomatic উপায়ে scope এ `HashMap` আনা">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

এই idiomatic এর পিছনে কোনো জোরালো কারণ নেই: এটি শুধু এমন একটি নিয়ম যা তৈরি হয়েছে এবং লোকেরা এভাবেই Rust কোড পড়তে এবং লিখতে অভ্যস্ত হয়ে গেছে।

এই idiomatic এর ব্যতিক্রম হল যদি আমরা `use` স্টেটমেন্ট দিয়ে একই নামের দুটি আইটেম scope এ আনি, কারণ Rust এর অনুমতি দেয় না। Listing 7-15 দেখায় কিভাবে দুটি `Result` প্রকারকে scope এ আনতে হয় যাদের একই নাম আছে কিন্তু ভিন্ন প্যারেন্ট মডিউল আছে এবং কিভাবে সেগুলিকে উল্লেখ করতে হয়।

<Listing number="7-15" file-name="src/lib.rs" caption="একই scope এ একই নামের দুটি প্রকার আনার জন্য তাদের প্যারেন্ট মডিউল ব্যবহার করা প্রয়োজন।">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

আপনি দেখতে পাচ্ছেন, প্যারেন্ট মডিউল ব্যবহার করে দুটি `Result` প্রকারকে আলাদা করা যায়। যদি আমরা পরিবর্তে `use std::fmt::Result` এবং `use std::io::Result` নির্দিষ্ট করি, তাহলে আমাদের একই scope এ দুটি `Result` প্রকার থাকবে এবং Rust জানবে না যে আমরা যখন `Result` ব্যবহার করব তখন আমরা কোনটি বুঝিয়েছি।

### `as` কীওয়ার্ড দিয়ে নতুন নাম প্রদান করা

`use` দিয়ে একই scope এ একই নামের দুটি প্রকার আনার সমস্যার আরেকটি সমাধান আছে: path এর পরে, আমরা `as` এবং টাইপের জন্য একটি নতুন স্থানীয় নাম বা _alias_ নির্দিষ্ট করতে পারি। Listing 7-16, `as` ব্যবহার করে দুটি `Result` প্রকারের একটির নাম পরিবর্তন করে Listing 7-15 এর কোড লেখার আরেকটি উপায় দেখায়।

<Listing number="7-16" file-name="src/lib.rs" caption="`as` কীওয়ার্ড দিয়ে scope এ আনা হলে একটি টাইপের নাম পরিবর্তন করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

দ্বিতীয় `use` স্টেটমেন্টে, আমরা `std::io::Result` টাইপের জন্য নতুন নাম `IoResult` বেছে নিয়েছি, যা `std::fmt` থেকে আসা `Result` এর সাথে দ্বন্দ্ব করবে না, যা আমরা scope এও এনেছি। Listing 7-15 এবং Listing 7-16 কে idiomatic হিসাবে বিবেচনা করা হয়, তাই পছন্দটি আপনার উপর নির্ভর করে!

### `pub use` দিয়ে নাম পুনরায় রপ্তানি করা

যখন আমরা `use` কীওয়ার্ড দিয়ে একটি name কে scope এ আনি, তখন নতুন scope এ উপলব্ধ নাম private থাকে। আমাদের কোড কল করে এমন কোডকে সেই নামটিকে উল্লেখ করার জন্য সক্ষম করতে, যেন এটি সেই কোডের scope এ সংজ্ঞায়িত করা হয়েছে, আমরা `pub` এবং `use` একত্রিত করতে পারি। এই কৌশলটিকে _re-exporting_ বলা হয় কারণ আমরা একটি আইটেমকে scope এ আনছি কিন্তু সেই আইটেমটিকে অন্যদের তাদের scope এ আনার জন্য উপলব্ধ করছি।

Listing 7-17, মূল মডিউলে `use` এর সাথে Listing 7-11 এর কোড দেখায় যা `pub use` এ পরিবর্তিত হয়েছে।

<Listing number="7-17" file-name="src/lib.rs" caption="`pub use` দিয়ে একটি নতুন scope থেকে ব্যবহারের জন্য যেকোনো কোডের জন্য একটি নাম উপলব্ধ করা">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

এই পরিবর্তনের আগে, বাইরের কোডকে `restaurant::front_of_house::hosting::add_to_waitlist()` path ব্যবহার করে `add_to_waitlist` ফাংশনটি কল করতে হতো, যার জন্য `front_of_house` মডিউলটিকেও `pub` হিসাবে চিহ্নিত করার প্রয়োজন হতো। এখন যেহেতু এই `pub use` মূল মডিউল থেকে `hosting` মডিউলটিকে পুনরায় রপ্তানি করেছে, বাইরের কোডটি `restaurant::hosting::add_to_waitlist()` path ব্যবহার করতে পারে।

re-exporting কার্যকর যখন আপনার কোডের অভ্যন্তরীণ কাঠামোটি আপনার কোড কল করা প্রোগ্রামাররা ডোমেইন সম্পর্কে যেভাবে চিন্তা করে তার থেকে আলাদা হয়। উদাহরণস্বরূপ, এই রেস্তোরাঁ রূপকটিতে, যারা রেস্তোরাঁ চালায় তারা "front of house" এবং "back of house" সম্পর্কে চিন্তা করে। কিন্তু একটি রেস্তোরাঁয় আসা গ্রাহকরা সম্ভবত রেস্তোরাঁর অংশগুলি সম্পর্কে সেভাবে চিন্তা করবে না। `pub use` এর সাহায্যে, আমরা একটি কাঠামো দিয়ে আমাদের কোড লিখতে পারি তবে একটি ভিন্ন কাঠামো প্রকাশ করতে পারি। এটি করার ফলে আমাদের লাইব্রেরিটি লাইব্রেরিতে কাজ করা প্রোগ্রামার এবং লাইব্রেরি কল করা প্রোগ্রামার উভয়ের জন্যই সুসংগঠিত হয়। আমরা `pub use` এর আরও একটি উদাহরণ দেখব এবং এটি কিভাবে Chapter 14 এর [“Exporting a Convenient Public API with `pub use`”][ch14-pub-use]<!-- ignore --> বিভাগে আপনার crate এর ডকুমেন্টেশনকে প্রভাবিত করে।

### External Packages ব্যবহার করা

Chapter 2 এ, আমরা একটি অনুমান করার গেম প্রজেক্ট প্রোগ্রাম করেছি যা এলোমেলো সংখ্যা পাওয়ার জন্য `rand` নামের একটি বহিরাগত প্যাকেজ ব্যবহার করেছে। আমাদের প্রজেক্টে `rand` ব্যবহার করতে, আমরা _Cargo.toml_ এ এই লাইনটি যোগ করেছি:

<!-- `rand` এর ব্যবহৃত সংস্করণ আপডেট করার সময়, এই ফাইলগুলিতে ব্যবহৃত `rand` এর সংস্করণও আপডেট করুন যাতে সেগুলি সব মিলে যায়:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

_Cargo.toml_ এ `rand` কে নির্ভরতা হিসাবে যোগ করা Cargo কে [crates.io](https://crates.io/) থেকে `rand` প্যাকেজ এবং যেকোনো নির্ভরতা ডাউনলোড করতে এবং আমাদের প্রজেক্টের জন্য `rand` উপলব্ধ করতে বলে।

তারপর, আমাদের প্যাকেজের scope এ `rand` সংজ্ঞা আনার জন্য, আমরা crate এর নাম, `rand` দিয়ে শুরু করে একটি `use` লাইন যোগ করেছি এবং scope এ আনতে চাই এমন আইটেমগুলি তালিকাভুক্ত করেছি। মনে রাখবেন যে Chapter 2 এর [“Generating a Random Number”][rand]<!-- ignore --> বিভাগে, আমরা `Rng` trait টিকে scope এ এনেছি এবং `rand::thread_rng` ফাংশন কল করেছি:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Rust কমিউনিটির সদস্যরা [crates.io](https://crates.io/) এ অনেক প্যাকেজ উপলব্ধ করেছেন এবং সেগুলির যেকোনোটিকে আপনার প্যাকেজে আনার জন্য একই ধাপগুলি অনুসরণ করতে হবে: আপনার প্যাকেজের _Cargo.toml_ ফাইলে সেগুলি তালিকাভুক্ত করা এবং তাদের crates থেকে scope এ আইটেমগুলি আনতে `use` ব্যবহার করা।

মনে রাখবেন যে standard `std` লাইব্রেরিও একটি crate যা আমাদের প্যাকেজের জন্য বহিরাগত। যেহেতু standard library Rust ভাষার সাথে পাঠানো হয়, তাই `std` অন্তর্ভুক্ত করার জন্য আমাদের _Cargo.toml_ পরিবর্তন করার প্রয়োজন নেই। তবে আমাদের সেখান থেকে আইটেমগুলি আমাদের প্যাকেজের scope এ আনার জন্য `use` দিয়ে উল্লেখ করতে হবে। উদাহরণস্বরূপ, `HashMap` এর সাথে আমরা এই লাইনটি ব্যবহার করব:

```rust
use std::collections::HashMap;
```

এটি `std` দিয়ে শুরু হওয়া একটি absolute path, যা standard library crate এর নাম।

### বড় `use` তালিকা পরিষ্কার করতে নেস্টেড Paths ব্যবহার করা

যদি আমরা একই crate বা একই মডিউলে সংজ্ঞায়িত একাধিক আইটেম ব্যবহার করি, তবে প্রতিটি আইটেমকে তার নিজের লাইনে তালিকাভুক্ত করলে আমাদের ফাইলগুলিতে অনেক উল্লম্ব স্থান নিতে পারে। উদাহরণস্বরূপ, Listing 2-4 এ অনুমান করার গেমটিতে আমাদের থাকা এই দুটি `use` স্টেটমেন্ট `std` থেকে আইটেমগুলিকে scope এ নিয়ে আসে:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

পরিবর্তে, আমরা একটি লাইনে একই আইটেমগুলিকে scope এ আনতে নেস্টেড paths ব্যবহার করতে পারি। আমরা path এর সাধারণ অংশটি নির্দিষ্ট করে এটি করি, তারপরে দুটি কোলন এবং তারপরে paths এর সেই অংশগুলির একটি তালিকা সহ কার্লি ব্র্যাকেট যা আলাদা, যেমন Listing 7-18 এ দেখানো হয়েছে।

<Listing number="7-18" file-name="src/main.rs" caption="একই উপসর্গ সহ একাধিক আইটেম scope এ আনতে একটি নেস্টেড path নির্দিষ্ট করা">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

বড় প্রোগ্রামগুলিতে, নেস্টেড paths ব্যবহার করে একই crate বা মডিউল থেকে scope এ অনেকগুলি আইটেম আনা প্রয়োজনীয় `use` স্টেটমেন্টের সংখ্যা অনেক কমিয়ে দিতে পারে!

আমরা একটি path এর যেকোনো স্তরে একটি নেস্টেড path ব্যবহার করতে পারি, যা দুটি `use` স্টেটমেন্ট একত্রিত করার সময় কার্যকর হয় যা একটি subpath শেয়ার করে। উদাহরণস্বরূপ, Listing 7-19 দুটি `use` স্টেটমেন্ট দেখায়: একটি যা `std::io` কে scope এ নিয়ে আসে এবং অন্যটি যা `std::io::Write` কে scope এ নিয়ে আসে।

<Listing number="7-19" file-name="src/lib.rs" caption="দুটি `use` স্টেটমেন্ট যেখানে একটি অন্যটির subpath">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

এই দুটি path এর সাধারণ অংশ হল `std::io` এবং এটি হল সম্পূর্ণ প্রথম path। এই দুটি path কে একটি `use` স্টেটমেন্টে মার্জ করতে, আমরা নেস্টেড path এ `self` ব্যবহার করতে পারি, যেমন Listing 7-20 এ দেখানো হয়েছে।

<Listing number="7-20" file-name="src/lib.rs" caption="Listing 7-19 এর paths কে একটি `use` স্টেটমেন্টে একত্রিত করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

এই লাইনটি `std::io` এবং `std::io::Write` কে scope এ নিয়ে আসে।

### গ্লোব অপারেটর

যদি আমরা একটি path এ সংজ্ঞায়িত _সমস্ত_ public আইটেমকে scope এ আনতে চাই, তবে আমরা সেই path টিকে `*` গ্লোব অপারেটর দিয়ে অনুসরণ করতে পারি:

```rust
use std::collections::*;
```

এই `use` স্টেটমেন্টটি `std::collections` এ সংজ্ঞায়িত সমস্ত public আইটেমকে বর্তমান scope এ নিয়ে আসে। গ্লোব অপারেটর ব্যবহার করার সময় সতর্ক থাকুন! গ্লোব আপনার scope এ কোন নামগুলি আছে এবং আপনার প্রোগ্রামে ব্যবহৃত একটি নাম কোথায় সংজ্ঞায়িত করা হয়েছে তা বলা কঠিন করে তুলতে পারে।

গ্লোব অপারেটরটি প্রায়শই পরীক্ষার সময় ব্যবহৃত হয় `tests` মডিউলে পরীক্ষার অধীনে থাকা সবকিছু আনার জন্য; আমরা Chapter 11 এর [“How to Write Tests”][writing-tests]<!-- ignore --> বিভাগে এই বিষয়ে কথা বলব। গ্লোব অপারেটরটি কখনও কখনও prelude pattern এর অংশ হিসাবেও ব্যবহৃত হয়: সেই pattern সম্পর্কে আরও তথ্যের জন্য [standard library ডকুমেন্টেশন](../std/prelude/index.html#other-preludes)<!-- ignore --> দেখুন।

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
