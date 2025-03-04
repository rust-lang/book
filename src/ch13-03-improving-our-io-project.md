## আমাদের I/O প্রোজেক্টকে উন্নত করা

Iterator সম্পর্কে এই নতুন জ্ঞান দিয়ে, আমরা Chapter 12-এর I/O প্রোজেক্টকে improve করতে পারি iterator ব্যবহার করে code-এর জায়গাগুলোকে আরও clear এবং concise করতে। আসুন দেখি কীভাবে iterator গুলো `Config::build` ফাংশন এবং `search` ফাংশনের আমাদের implementation-কে improve করতে পারে।

### একটি `clone` সরানো Iterator ব্যবহার করে

Listing 12-6-এ, আমরা code যোগ করেছিলাম যেটি `String` value-গুলোর একটি slice নিত এবং slice-এ index করে এবং value গুলোকে clone করে `Config` struct-এর একটি instance create করত, `Config` struct-কে সেই value-গুলোর owner হওয়ার অনুমতি দিত। Listing 13-17-এ, আমরা `Config::build` ফাংশনের implementation-টিকে পুনরায় লিখেছি যেমনটি Listing 12-23-এ ছিল:

<Listing number="13-17" file-name="src/lib.rs" caption="Listing 12-23 থেকে `Config::build` ফাংশনের পুনরাবৃত্তি">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```

</Listing>

তখন, আমরা বলেছিলাম inefficient `clone` কলগুলো নিয়ে চিন্তা না করতে কারণ আমরা ভবিষ্যতে সেগুলোকে সরিয়ে দেব। এখন সেই সময়!

আমাদের এখানে `clone`-এর প্রয়োজন ছিল কারণ parameter `args`-এ `String` element-সহ একটি slice রয়েছে, কিন্তু `build` ফাংশনটি `args`-এর owner নয়। একটি `Config` instance-এর ownership return করার জন্য, আমাদের `Config`-এর `query` এবং `file_path` field থেকে value গুলোকে clone করতে হয়েছিল যাতে `Config` instance টি তার value-গুলোর owner হতে পারে।

Iterator সম্পর্কে আমাদের নতুন জ্ঞান দিয়ে, আমরা `build` ফাংশনটিকে পরিবর্তন করতে পারি একটি slice borrow করার পরিবর্তে argument হিসেবে একটি iterator-এর ownership নেওয়ার জন্য। আমরা slice-এর length check করা এবং specific location-গুলোতে index করার code-এর পরিবর্তে iterator functionality ব্যবহার করব। এটি `Config::build` ফাংশনটি কী করছে তা স্পষ্ট করবে কারণ iterator টি value গুলো access করবে।

একবার `Config::build` iterator-এর ownership নেওয়ার পরে এবং borrow করা indexing operation গুলো ব্যবহার করা বন্ধ করে দিলে, আমরা `clone` কল না করে এবং একটি new allocation তৈরি না করে iterator থেকে `String` value গুলোকে `Config`-এ move করতে পারি।

#### Returned Iterator সরাসরি ব্যবহার করা

আপনার I/O প্রোজেক্টের _src/main.rs_ ফাইলটি খুলুন, যেটি এইরকম হওয়া উচিত:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

আমরা প্রথমে `main` ফাংশনের শুরু পরিবর্তন করব যা Listing 12-24-এ ছিল Listing 13-18-এর code-এ, যেটি এবার একটি iterator ব্যবহার করে। আমরা যতক্ষণ `Config::build` update না করি ততক্ষণ এটি compile হবে না।

<Listing number="13-18" file-name="src/main.rs" caption="`Config::build`-এ `env::args`-এর return value পাস করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

`Env::args` ফাংশন একটি iterator return করে! Iterator value গুলোকে একটি vector-এ collect করে তারপর `Config::build`-এ একটি slice pass করার পরিবর্তে, এখন আমরা `env::args` থেকে returned iterator-এর ownership সরাসরি `Config::build`-এ pass করছি।

এরপরে, আমাদের `Config::build`-এর definition update করতে হবে। আপনার I/O প্রোজেক্টের _src/lib.rs_ ফাইলে, আসুন `Config::build`-এর signature পরিবর্তন করে Listing 13-19-এর মতো করি। এটি এখনও compile হবে না কারণ আমাদের function body update করতে হবে।

<Listing number="13-19" file-name="src/lib.rs" caption="একটি iterator আশা করার জন্য `Config::build`-এর signature আপডেট করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs:here}}
```

</Listing>

`Env::args` ফাংশনের জন্য standard library documentation দেখায় যে এটি যে iterator return করে তার type হল `std::env::Args`, এবং সেই type টি `Iterator` trait implement করে এবং `String` value return করে।

আমরা `Config::build` ফাংশনের signature আপডেট করেছি যাতে parameter `args`-এর একটি generic type থাকে trait bound `impl Iterator<Item = String>` সহ, `&[String]`-এর পরিবর্তে। Chapter 10-এর [“প্যারামিটার হিসেবে Traits”][impl-trait] বিভাগে আলোচনা করা `impl Trait` syntax-এর এই ব্যবহারটির অর্থ হল `args` যেকোনো type হতে পারে যেটি `Iterator` trait implement করে এবং `String` item return করে।

যেহেতু আমরা `args`-এর ownership নিচ্ছি এবং এটিকে iterate করে `args` কে mutate করব, তাই আমরা এটিকে mutable করতে `args` parameter-এর specification-এ `mut` keyword যোগ করতে পারি।

#### Indexing-এর পরিবর্তে `Iterator` Trait Method ব্যবহার করা

এরপরে, আমরা `Config::build`-এর body ঠিক করব। যেহেতু `args`, `Iterator` trait implement করে, তাই আমরা জানি যে আমরা এটিতে `next` method কল করতে পারি! Listing 13-20, Listing 12-23 থেকে code update করে `next` method ব্যবহার করার জন্য:

<Listing number="13-20" file-name="src/lib.rs" caption="Iterator method ব্যবহার করতে `Config::build`-এর body পরিবর্তন করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs:here}}
```

</Listing>

মনে রাখবেন যে `env::args`-এর return value-এর প্রথম value টি হল প্রোগ্রামের নাম। আমরা সেটিকে ignore করতে চাই এবং পরবর্তী value-তে যেতে চাই, তাই প্রথমে আমরা `next` কল করি এবং return value-এর সাথে কিছুই করি না। দ্বিতীয়ত, আমরা `Config`-এর `query` field-এ যে value টি রাখতে চাই সেটি পেতে `next` কল করি। যদি `next` একটি `Some` return করে, তাহলে আমরা value টি extract করতে একটি `match` ব্যবহার করি। যদি এটি `None` return করে, তাহলে এর অর্থ হল পর্যাপ্ত argument দেওয়া হয়নি এবং আমরা একটি `Err` value দিয়ে তাড়াতাড়ি return করি। আমরা `file_path` value-এর জন্য একই কাজ করি।

### Iterator Adapter দিয়ে Code আরও Clear করা

আমরা আমাদের I/O প্রোজেক্টের `search` ফাংশনেও iterator-গুলোর সুবিধা নিতে পারি, যেটি Listing 13-21-এ পুনরায় লেখা হয়েছে যেমনটি Listing 12-19-এ ছিল:

<Listing number="13-21" file-name="src/lib.rs" caption="Listing 12-19 থেকে `search` ফাংশনের ইমপ্লিমেন্টেশন">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

আমরা iterator adapter method ব্যবহার করে এই code-টিকে আরও concise উপায়ে লিখতে পারি। এটি আমাদের একটি mutable intermediate `results` vector থাকাও এড়াতে দেয়। Functional programming style code-কে আরও clear করতে mutable state-এর পরিমাণ minimize করা prefer করে। Mutable state সরিয়ে দেওয়া future-এ searching-কে parallel-এ ঘটানোর enhancement enable করতে পারে, কারণ আমাদের `results` vector-এ concurrent access manage করতে হবে না। Listing 13-22 এই পরিবর্তনটি দেখায়:

<Listing number="13-22" file-name="src/lib.rs" caption="`search` ফাংশনের ইমপ্লিমেন্টেশনে iterator অ্যাডাপ্টার মেথড ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

মনে রাখবেন যে `search` ফাংশনের উদ্দেশ্য হল `contents`-এর সমস্ত line return করা যেগুলোতে `query` রয়েছে। Listing 13-16-এর `filter` উদাহরণের মতোই, এই code টি শুধুমাত্র সেই line গুলো রাখতে `filter` adapter ব্যবহার করে যেগুলোর জন্য `line.contains(query)` `true` return করে। তারপর আমরা matching line গুলোকে `collect`-এর সাহায্যে অন্য একটি vector-এ collect করি। অনেক সহজ! `Search_case_insensitive` ফাংশনেও iterator method ব্যবহার করতে একই পরিবর্তন করতে পারেন।

### Loop বা Iterator-এর মধ্যে বেছে নেওয়া

পরবর্তী logical প্রশ্ন হল আপনার নিজের code-এ আপনার কোন style বেছে নেওয়া উচিত এবং কেন: Listing 13-21-এর original implementation নাকি Listing 13-22-এ iterator ব্যবহার করা version। বেশিরভাগ Rust programmer iterator style ব্যবহার করা prefer করেন। এটি প্রথমে আয়ত্ত করা একটু কঠিন, কিন্তু একবার আপনি বিভিন্ন iterator adapter এবং সেগুলো কী করে সে সম্পর্কে ধারণা পেলে, iterator গুলো বুঝতে সহজ হতে পারে। Looping-এর বিভিন্ন অংশ নিয়ে ঘাঁটাঘাঁটি করার এবং new vector তৈরি করার পরিবর্তে, code loop-এর high-level objective-এর উপর focus করে। এটি কিছু commonplace code-কে abstract করে যাতে এই code-এর unique concept গুলো দেখা সহজ হয়, যেমন iterator-এর প্রতিটি element-কে যে filtering condition টি pass করতে হবে।

কিন্তু দুটি implementation কি truly equivalent? স্বজ্ঞাত অনুমান হতে পারে যে আরও low-level loop টি দ্রুততর হবে। আসুন performance নিয়ে কথা বলি।

[impl-trait]: ch10-02-traits.html#traits-as-parameters
