## আমাদের I/O প্রজেক্ট উন্নত করা

Iterators সম্পর্কে এই নতুন জ্ঞান দিয়ে, আমরা অধ্যায় 12-এর I/O প্রজেক্টে কোডটিকে আরও স্পষ্ট এবং সংক্ষিপ্ত করতে iterator ব্যবহার করে উন্নত করতে পারি। আসুন দেখি কিভাবে iterator আমাদের `Config::build` ফাংশন এবং `search` ফাংশনের বাস্তবায়নকে উন্নত করতে পারে।

### একটি Iterator ব্যবহার করে `clone` সরানো

Listing 12-6-এ, আমরা এমন কোড যোগ করেছি যা `String` মানগুলির একটি slice নেয় এবং slice-এর মধ্যে indexing করে এবং মানগুলির `clone` করে `Config` struct-এর একটি উদাহরণ তৈরি করে, যা `Config` struct-কে সেই মানগুলির মালিক হতে দেয়। Listing 13-17-এ, আমরা `Config::build` ফাংশনের বাস্তবায়ন পুনরুত্পাদন করেছি যেমনটি Listing 12-23-এ ছিল:

<Listing number="13-17" file-name="src/lib.rs" caption="Listing 12-23 থেকে `Config::build` ফাংশনের পুনরুৎপাদন">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```

</Listing>

সেই সময়, আমরা বলেছিলাম যে অদক্ষ `clone` কলগুলি নিয়ে চিন্তা না করতে কারণ আমরা ভবিষ্যতে সেগুলি সরিয়ে ফেলব। ভালো, সেই সময়টা এখন!

এখানে আমাদের `clone`-এর প্রয়োজন ছিল কারণ প্যারামিটার `args`-এ `String` উপাদান সহ আমাদের একটি slice রয়েছে, কিন্তু `build` ফাংশন `args`-এর মালিক নয়। `Config` উদাহরণের ownership ফেরত দিতে, আমাদের `Config`-এর `query` এবং `file_path` ফিল্ড থেকে মানগুলি clone করতে হয়েছিল যাতে `Config` উদাহরণ তার মানগুলির মালিক হতে পারে।

iterators সম্পর্কে আমাদের নতুন জ্ঞান দিয়ে, আমরা `build` ফাংশনকে একটি slice ধার করার পরিবর্তে এর আর্গুমেন্ট হিসাবে একটি iterator এর ownership নেওয়ার জন্য পরিবর্তন করতে পারি। আমরা slice-এর দৈর্ঘ্য পরীক্ষা করে নির্দিষ্ট স্থানে index করার পরিবর্তে iterator এর কার্যকারিতা ব্যবহার করব। এটি `Config::build` ফাংশনটি কী করছে তা স্পষ্ট করবে কারণ iterator মানগুলি অ্যাক্সেস করবে।

একবার `Config::build` iterator-এর ownership নিলে এবং ধার করা indexing অপারেশন ব্যবহার করা বন্ধ করলে, আমরা `clone` কল করে নতুন allocation করার পরিবর্তে iterator থেকে `String` মানগুলি `Config`-এ স্থানান্তরিত করতে পারি।

#### সরাসরি Return করা Iterator ব্যবহার করা

আপনার I/O প্রজেক্টের _src/main.rs_ ফাইলটি খুলুন, যা এইরকম হওয়া উচিত:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

আমরা প্রথমে `main` ফাংশনের শুরু পরিবর্তন করব যা আমাদের Listing 12-24-এ ছিল Listing 13-18 এর কোডে, যা এইবার একটি iterator ব্যবহার করে। আমরা `Config::build` আপডেট না করা পর্যন্ত এটি compile হবে না।

<Listing number="13-18" file-name="src/main.rs" caption="`Config::build`-এ `env::args`-এর return value পাস করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

`env::args` ফাংশন একটি iterator ফেরত দেয়! iterator মানগুলিকে একটি ভেক্টরে সংগ্রহ করে এবং তারপরে `Config::build`-এ একটি slice পাস করার পরিবর্তে, এখন আমরা `env::args` থেকে ফেরত দেওয়া iterator-এর ownership সরাসরি `Config::build`-এ পাস করছি।

পরবর্তীকালে, আমাদের `Config::build`-এর সংজ্ঞা আপডেট করতে হবে। আপনার I/O প্রজেক্টের _src/lib.rs_ ফাইলে, `Config::build`-এর সংজ্ঞাকে Listing 13-19 এর মতো দেখতে পরিবর্তন করা যাক। এটি এখনও compile হবে না কারণ আমাদের ফাংশন বডি আপডেট করতে হবে।

<Listing number="13-19" file-name="src/lib.rs" caption="একটি iterator আশা করার জন্য `Config::build`-এর সংজ্ঞা আপডেট করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs:here}}
```

</Listing>

`env::args` ফাংশনের জন্য স্ট্যান্ডার্ড লাইব্রেরি ডকুমেন্টেশন দেখায় যে এটি যে iterator ফেরত দেয় তার টাইপ হল `std::env::Args`, এবং সেই টাইপটি `Iterator` trait implement করে এবং `String` মান ফেরত দেয়।

আমরা `Config::build` ফাংশনের সংজ্ঞা আপডেট করেছি যাতে প্যারামিটার `args`-এর `&[String]` এর পরিবর্তে trait বাউন্ড `impl Iterator<Item = String>` সহ একটি জেনেরিক টাইপ থাকে। অধ্যায় 10-এর [“Traits as Parameters”][impl-trait]<!-- ignore --> বিভাগে আমরা আলোচনা করা `impl Trait` সিনট্যাক্সের এই ব্যবহারের অর্থ হল `args` এমন যেকোনো টাইপ হতে পারে যা `Iterator` trait implement করে এবং `String` আইটেম ফেরত দেয়।

যেহেতু আমরা `args`-এর ownership নিচ্ছি এবং এর উপর iterate করে `args` পরিবর্তন করব, তাই এটিকে mutable করার জন্য আমরা `args` প্যারামিটারের স্পেসিফিকেশনে `mut` কীওয়ার্ড যোগ করতে পারি।

#### Indexing এর পরিবর্তে `Iterator` Trait মেথড ব্যবহার করা

পরবর্তীকালে, আমরা `Config::build`-এর বডি ঠিক করব। যেহেতু `args` `Iterator` trait implement করে, তাই আমরা জানি যে আমরা এর উপর `next` মেথড কল করতে পারি! Listing 13-20, `next` মেথড ব্যবহার করার জন্য Listing 12-23 থেকে কোড আপডেট করে:

<Listing number="13-20" file-name="src/lib.rs" caption="iterator মেথড ব্যবহার করার জন্য `Config::build`-এর বডি পরিবর্তন করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs:here}}
```

</Listing>

মনে রাখবেন যে `env::args`-এর return value-এর প্রথম মানটি প্রোগ্রামের নাম। আমরা সেটাকে উপেক্ষা করতে চাই এবং পরবর্তী মান পেতে চাই, তাই প্রথমে আমরা `next` কল করি এবং return value দিয়ে কিছুই করি না। দ্বিতীয়ত, আমরা `Config`-এর `query` ফিল্ডে রাখতে চাই এমন মান পেতে `next` কল করি। যদি `next` একটি `Some` ফেরত দেয়, তাহলে আমরা মানটি বের করার জন্য একটি `match` ব্যবহার করি। যদি এটি `None` ফেরত দেয়, এর মানে পর্যাপ্ত আর্গুমেন্ট দেওয়া হয়নি এবং আমরা `Err` মান দিয়ে তাড়াতাড়ি ফেরত দিই। আমরা `file_path` মানের জন্যও একই কাজ করি।

### Iterator Adapter দিয়ে কোড আরও স্পষ্ট করা

আমরা আমাদের I/O প্রজেক্টের `search` ফাংশনেও iterator-এর সুবিধা নিতে পারি, যা Listing 13-21-এ Listing 12-19-এর মতো পুনরুত্পাদন করা হয়েছে:

<Listing number="13-21" file-name="src/lib.rs" caption="Listing 12-19 থেকে `search` ফাংশনের বাস্তবায়ন">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

আমরা iterator adapter মেথড ব্যবহার করে এই কোডটিকে আরও সংক্ষিপ্ত উপায়ে লিখতে পারি। এটি আমাদের mutable ইন্টারমিডিয়েট `results` ভেক্টর এড়াতেও দেয়। Functional প্রোগ্রামিং স্টাইল কোডকে আরও স্পষ্ট করার জন্য mutable state এর পরিমাণ কমাতে পছন্দ করে। mutable state সরিয়ে ভবিষ্যতে সমান্তরালভাবে অনুসন্ধান করার জন্য একটি উন্নতি করতে সক্ষম হতে পারে, কারণ `results` ভেক্টরের concurrent অ্যাক্সেস পরিচালনা করতে হবে না। Listing 13-22 এই পরিবর্তনটি দেখায়:

<Listing number="13-22" file-name="src/lib.rs" caption="`search` ফাংশনের বাস্তবায়নে iterator adapter মেথড ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

মনে রাখবেন যে `search` ফাংশনের উদ্দেশ্য হল `contents`-এর সেই সমস্ত লাইনগুলি ফেরত দেওয়া যেখানে `query` রয়েছে। Listing 13-16-এর `filter` উদাহরণের মতো, এই কোডটি `filter` adapter ব্যবহার করে শুধুমাত্র সেই লাইনগুলি রাখতে যেখানে `line.contains(query)` `true` ফেরত দেয়। তারপরে আমরা `collect` দিয়ে মিলে যাওয়া লাইনগুলিকে অন্য একটি ভেক্টরে সংগ্রহ করি। অনেক সহজ! `search_case_insensitive` ফাংশনে iterator মেথড ব্যবহার করার জন্য একই পরিবর্তন করতে দ্বিধা বোধ করবেন না।

### লুপ বা Iterators এর মধ্যে নির্বাচন করা

পরবর্তী যৌক্তিক প্রশ্ন হল আপনার নিজের কোডে কোন স্টাইলটি বেছে নেওয়া উচিত এবং কেন: Listing 13-21-এর আসল বাস্তবায়ন নাকি Listing 13-22-এ iterator ব্যবহার করা সংস্করণ। বেশিরভাগ Rust প্রোগ্রামার iterator স্টাইল ব্যবহার করতে পছন্দ করেন। প্রথমে এটি আয়ত্ত করা একটু কঠিন, কিন্তু একবার আপনি বিভিন্ন iterator adapter এবং তারা কী করে সে সম্পর্কে ধারণা পেলে, iterator বোঝা সহজ হতে পারে। লুপ এবং নতুন ভেক্টর তৈরির বিভিন্ন বিট নিয়ে কাজ করার পরিবর্তে, কোডটি লুপের উচ্চ-স্তরের উদ্দেশ্যের উপর দৃষ্টি নিবদ্ধ করে। এটি কিছু সাধারণ কোডকে abstract করে, তাই এই কোডের জন্য অনন্য ধারণাগুলি দেখা সহজ, যেমন iterator-এর প্রতিটি উপাদানকে যে ফিল্টারিং শর্তটি পাস করতে হবে।

কিন্তু দুটি বাস্তবায়ন কি সত্যিই সমতুল্য? স্বজ্ঞাত ধারণা হতে পারে যে আরও নিম্ন-স্তরের লুপটি দ্রুত হবে। আসুন কর্মক্ষমতা নিয়ে কথা বলি।

[impl-trait]: ch10-02-traits.html#traits-as-parameters
