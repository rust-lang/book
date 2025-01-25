## Iterators এর মাধ্যমে আইটেমের সিরিজ প্রক্রিয়া করা

Iterator প্যাটার্ন আপনাকে একটি সিকোয়েন্সের আইটেমের উপর একটি কাজ করে যেতে দেয়। একটি iterator প্রতিটি আইটেমের উপর পুনরাবৃত্তি করার যুক্তি এবং কখন সিকোয়েন্সটি শেষ হয়েছে তা নির্ধারণ করার জন্য দায়ী। যখন আপনি iterator ব্যবহার করেন, তখন আপনাকে সেই যুক্তিটি নিজে থেকে পুনরায় প্রয়োগ করতে হয় না।

Rust এ, iterator _lazy_ হয়, মানে যতক্ষণ না আপনি iterator ব্যবহার করার জন্য consume করেন, ততক্ষণ পর্যন্ত এর কোনো প্রভাব নেই। উদাহরণস্বরূপ, Listing 13-10 এর কোড `Vec<T>` এ সংজ্ঞায়িত `iter` মেথড কল করে ভেক্টর `v1`-এর আইটেমগুলোর উপর একটি iterator তৈরি করে। এই কোডটি নিজে থেকে কোনো কাজ করে না।

<Listing number="13-10" file-name="src/main.rs" caption="একটি iterator তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

iterator `v1_iter` ভেরিয়েবলে সংরক্ষিত হয়। একবার আমরা একটি iterator তৈরি করার পরে, আমরা এটিকে বিভিন্ন উপায়ে ব্যবহার করতে পারি। অধ্যায় 3-এর Listing 3-5-এ, আমরা একটি `for` লুপ ব্যবহার করে একটি অ্যারের প্রতিটি আইটেমের উপর কিছু কোড execute করার জন্য iterate করেছিলাম। পর্দার আড়ালে এটি implicit ভাবে একটি iterator তৈরি করে এবং পরে consume করে, কিন্তু এখন পর্যন্ত এটি কিভাবে কাজ করে তা আমরা আলোচনা করিনি।

Listing 13-11-এর উদাহরণে, আমরা `for` লুপে iterator-এর ব্যবহার থেকে iterator তৈরি করাকে আলাদা করি। যখন `for` লুপ `v1_iter`-এর iterator ব্যবহার করে কল করা হয়, তখন iterator-এর প্রতিটি উপাদান লুপের একটি পুনরাবৃত্তিতে ব্যবহৃত হয়, যা প্রতিটি মান প্রিন্ট করে।

<Listing number="13-11" file-name="src/main.rs" caption="একটি `for` লুপে একটি iterator ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

যেসব ভাষায় তাদের স্ট্যান্ডার্ড লাইব্রেরি দ্বারা প্রদত্ত iterator নেই, আপনি সম্ভবত index 0 থেকে একটি ভেরিয়েবল শুরু করে, সেই ভেরিয়েবলটি ভেক্টরের মধ্যে একটি মান পাওয়ার জন্য index করতে এবং একটি লুপে ভেরিয়েবলের মান বাড়িয়ে ভেক্টরের মোট আইটেমের সংখ্যা না পৌঁছানো পর্যন্ত একই কার্যকারিতা লিখতেন।

Iterators আপনার জন্য সেই সমস্ত যুক্তি পরিচালনা করে, পুনরাবৃত্তিমূলক কোড কমিয়ে দেয় যা আপনি সম্ভাব্যভাবে ভুল করতে পারেন। Iterators আপনাকে ভেক্টরের মতো index করা যায় এমন ডেটা স্ট্রাকচার ছাড়াও বিভিন্ন ধরনের সিকোয়েন্সের সাথে একই যুক্তি ব্যবহার করার আরও বেশি সুবিধা দেয়। আসুন পরীক্ষা করি কিভাবে iterators তা করে।

### `Iterator` Trait এবং `next` মেথড

সমস্ত iterator `Iterator` নামের একটি trait implement করে যা স্ট্যান্ডার্ড লাইব্রেরিতে সংজ্ঞায়িত করা হয়েছে। trait এর সংজ্ঞাটি দেখতে এইরকম:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // default implementations সহ মেথডগুলো বাদ দেওয়া হয়েছে
}
```

লক্ষ্য করুন এই সংজ্ঞাটি কিছু নতুন সিনট্যাক্স ব্যবহার করে: `type Item` এবং `Self::Item`, যা এই trait এর সাথে একটি _associated type_ সংজ্ঞায়িত করছে। আমরা অধ্যায় 20 এ associated type নিয়ে বিস্তারিত আলোচনা করব। আপাতত, আপনার যা জানা দরকার তা হল এই কোডটি বলছে যে `Iterator` trait implement করতে হলে আপনাকে একটি `Item` টাইপও সংজ্ঞায়িত করতে হবে এবং এই `Item` টাইপটি `next` মেথডের রিটার্ন টাইপে ব্যবহৃত হয়। অন্য কথায়, `Item` টাইপটি iterator থেকে ফেরত দেওয়া টাইপ হবে।

`Iterator` trait implementer-দের শুধুমাত্র একটি মেথড সংজ্ঞায়িত করতে বলে: `next` মেথড, যা `Some`-এ মোড়ানো একটি iterator-এর একটি আইটেম ফেরত দেয় এবং যখন পুনরাবৃত্তি শেষ হয়ে যায়, তখন `None` ফেরত দেয়।

আমরা সরাসরি iterator-এ `next` মেথড কল করতে পারি; Listing 13-12 ভেক্টর থেকে তৈরি iterator-এ `next`-এর বারবার কল করা থেকে কোন মানগুলি ফেরত দেওয়া হয় তা দেখায়।

<Listing number="13-12" file-name="src/lib.rs" caption="একটি iterator-এ `next` মেথড কল করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

লক্ষ্য করুন যে আমাদের `v1_iter`-কে mutable করতে হয়েছে: iterator-এ `next` মেথড কল করলে অভ্যন্তরীণ state পরিবর্তন হয় যা iterator সিকোয়েন্সে কোথায় আছে তা track করতে ব্যবহার করে। অন্য কথায়, এই কোডটি iterator _consume_ করে বা ব্যবহার করে। `next`-এর প্রতিটি কল iterator থেকে একটি আইটেম consume করে। যখন আমরা `for` লুপ ব্যবহার করি তখন আমাদের `v1_iter`-কে mutable করার প্রয়োজন ছিল না কারণ লুপ `v1_iter`-এর ownership গ্রহণ করে এবং পর্দার আড়ালে এটিকে mutable করে তোলে।

এছাড়াও মনে রাখবেন যে `next`-এ কল করা থেকে আমরা যে মানগুলি পাই তা ভেক্টরের মানগুলির immutable reference। `iter` মেথড immutable রেফারেন্সের উপর একটি iterator তৈরি করে। যদি আমরা এমন একটি iterator তৈরি করতে চাই যা `v1`-এর ownership নেয় এবং owned মান ফেরত দেয়, তাহলে আমরা `iter`-এর পরিবর্তে `into_iter` কল করতে পারি। একইভাবে, যদি আমরা mutable রেফারেন্সের উপর iterate করতে চাই, তাহলে আমরা `iter`-এর পরিবর্তে `iter_mut` কল করতে পারি।

### যে মেথডগুলো Iterator Consume করে

`Iterator` trait-এর স্ট্যান্ডার্ড লাইব্রেরি দ্বারা প্রদত্ত ডিফল্ট বাস্তবায়ন সহ বিভিন্ন মেথড রয়েছে; `Iterator` trait-এর জন্য স্ট্যান্ডার্ড লাইব্রেরি API ডকুমেন্টেশনে দেখে আপনি এই মেথডগুলি সম্পর্কে জানতে পারেন। এই মেথডগুলির মধ্যে কিছু তাদের সংজ্ঞায় `next` মেথড কল করে, সেই কারণে `Iterator` trait implement করার সময় আপনাকে `next` মেথড implement করতে হয়।

যে মেথডগুলি `next` কল করে সেগুলোকে _consuming adapter_ বলা হয়, কারণ এগুলি কল করলে iterator ব্যবহার হয়ে যায়। একটি উদাহরণ হল `sum` মেথড, যা iterator-এর ownership নেয় এবং বারবার `next` কল করে আইটেমগুলোর মাধ্যমে iterate করে, যার ফলে iterator consume হয়ে যায়। এটি iterate করার সাথে সাথে, এটি প্রতিটি আইটেমকে চলমান টোটালে যোগ করে এবং পুনরাবৃত্তি সম্পূর্ণ হলে মোট রিটার্ন করে। Listing 13-13-এ `sum` মেথডের ব্যবহারের একটি test রয়েছে:

<Listing number="13-13" file-name="src/lib.rs" caption="iterator-এর সমস্ত আইটেমের মোট পেতে `sum` মেথড কল করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

`sum` কল করার পরে `v1_iter` ব্যবহার করার অনুমতি নেই কারণ `sum` iterator-এর ownership নেয়।

### যে মেথডগুলো অন্যান্য Iterator তৈরি করে

_Iterator adapter_ হল `Iterator` trait-এ সংজ্ঞায়িত মেথড যা iterator consume করে না। পরিবর্তে, তারা মূল iterator-এর কিছু দিক পরিবর্তন করে বিভিন্ন iterator তৈরি করে।

Listing 13-14 iterator adapter মেথড `map` কল করার একটি উদাহরণ দেখায়, যা আইটেমগুলোর মাধ্যমে iterate করার সময় প্রতিটি আইটেমের উপর কল করার জন্য একটি closure নেয়। `map` মেথড একটি নতুন iterator ফেরত দেয় যা পরিবর্তিত আইটেম তৈরি করে। এখানে closure একটি নতুন iterator তৈরি করে যেখানে ভেক্টরের প্রতিটি আইটেম 1 দ্বারা বৃদ্ধি করা হবে:

<Listing number="13-14" file-name="src/main.rs" caption="একটি নতুন iterator তৈরি করার জন্য iterator adapter `map` কল করা">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

তবে, এই কোডটি একটি warning তৈরি করে:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

Listing 13-14 এর কোড কিছুই করে না; আমরা যে closure নির্দিষ্ট করেছি তা কখনও কল করা হয় না। warning টি আমাদের মনে করিয়ে দেয় কেন: iterator adapter lazy এবং এখানে আমাদের iterator consume করতে হবে।

এই warning ঠিক করতে এবং iterator consume করতে, আমরা `collect` মেথড ব্যবহার করব, যা আমরা Listing 12-1-এ `env::args` এর সাথে অধ্যায় 12-এ ব্যবহার করেছি। এই মেথডটি iterator consume করে এবং ফলাফলের মান একটি collection ডেটা টাইপে সংগ্রহ করে।

Listing 13-15-এ, আমরা `map`-এ কল করা থেকে ফেরত দেওয়া iterator-এর উপর iterate করার ফলাফল একটি ভেক্টরে সংগ্রহ করি। এই ভেক্টরটিতে 1 দ্বারা বৃদ্ধি করা মূল ভেক্টরের প্রতিটি আইটেম থাকবে।

<Listing number="13-15" file-name="src/main.rs" caption="একটি নতুন iterator তৈরি করতে `map` মেথড কল করা এবং তারপরে নতুন iterator consume করতে এবং একটি ভেক্টর তৈরি করতে `collect` মেথড কল করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

যেহেতু `map` একটি closure নেয়, তাই আমরা প্রতিটি আইটেমের উপর আমাদের ইচ্ছামত যেকোনো অপারেশন নির্দিষ্ট করতে পারি। এটি একটি চমৎকার উদাহরণ যে কিভাবে closure আপনাকে `Iterator` trait দ্বারা প্রদত্ত পুনরাবৃত্তি আচরণ পুনরায় ব্যবহার করার সময় কিছু আচরণ কাস্টমাইজ করতে দেয়।

আপনি একটি পঠনযোগ্য উপায়ে জটিল কাজগুলি করার জন্য iterator adapter-এ একাধিক কল chain করতে পারেন। তবে যেহেতু সমস্ত iterator lazy, তাই iterator adapter-এ কল করা থেকে ফলাফল পেতে আপনাকে consuming adapter মেথডগুলির মধ্যে একটি কল করতে হবে।

### Closure ব্যবহার করা যা তাদের Environment ক্যাপচার করে

অনেক iterator adapter আর্গুমেন্ট হিসাবে closure নেয়, এবং সাধারণত আমরা iterator adapter-এর আর্গুমেন্ট হিসাবে যে closure গুলি নির্দিষ্ট করব তা হবে closure যা তাদের environment ক্যাপচার করে।

এই উদাহরণের জন্য, আমরা `filter` মেথড ব্যবহার করব যা একটি closure নেয়। closure iterator থেকে একটি আইটেম পায় এবং একটি `bool` ফেরত দেয়। যদি closure `true` ফেরত দেয়, তাহলে মানটি `filter` দ্বারা তৈরি করা পুনরাবৃত্তিতে অন্তর্ভুক্ত করা হবে। যদি closure `false` ফেরত দেয়, তাহলে মানটি অন্তর্ভুক্ত করা হবে না।

Listing 13-16-এ, আমরা `Shoe` struct দৃষ্টান্তগুলির একটি collection-এর উপর iterate করার জন্য `filter`-এর সাথে একটি closure ব্যবহার করি যা তার environment থেকে `shoe_size` ভেরিয়েবল ক্যাপচার করে। এটি শুধুমাত্র সেই জুতাগুলি ফেরত দেবে যা নির্দিষ্ট আকারের।

<Listing number="13-16" file-name="src/lib.rs" caption="`shoe_size` ক্যাপচার করে এমন একটি closure সহ `filter` মেথড ব্যবহার করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

`shoes_in_size` ফাংশনটি প্যারামিটার হিসাবে জুতার একটি ভেক্টর এবং একটি জুতার আকার নেয়। এটি শুধুমাত্র নির্দিষ্ট আকারের জুতা ধারণকারী একটি ভেক্টর ফেরত দেয়।

`shoes_in_size`-এর বডিতে, আমরা একটি iterator তৈরি করতে `into_iter` কল করি যা ভেক্টরের ownership নেয়। তারপর আমরা সেই iterator কে একটি নতুন iterator এ রূপান্তর করতে `filter` কল করি যাতে closure `true` ফেরত দিলে শুধুমাত্র সেই উপাদানগুলি থাকে।

closure environment থেকে `shoe_size` প্যারামিটার ক্যাপচার করে এবং প্রতিটি জুতার আকারের সাথে মানটি তুলনা করে, শুধুমাত্র নির্দিষ্ট আকারের জুতা রাখে। অবশেষে, `collect` কল করা ফাংশন দ্বারা ফেরত দেওয়া একটি ভেক্টরে রূপান্তরিত iterator দ্বারা ফেরত দেওয়া মানগুলি সংগ্রহ করে।

test টি দেখায় যে যখন আমরা `shoes_in_size` কল করি, তখন আমরা শুধুমাত্র সেই জুতাগুলি ফেরত পাই যেগুলির আকার আমরা নির্দিষ্ট করেছি সেই মানের সমান।
