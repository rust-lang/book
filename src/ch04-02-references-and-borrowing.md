## রেফারেন্স এবং Borrowing

Listing 4-5 এ Tuple কোডের সমস্যা হল যে `calculate_length` এ কল করার পরে `String` ব্যবহার করতে পারার জন্য আমাদের কলিং ফাংশনে `String` টি ফেরত দিতে হয়, কারণ `String` টি `calculate_length` এ move করা হয়েছিল। পরিবর্তে, আমরা `String` ভ্যালুর একটি reference দিতে পারি। একটি _reference_ একটি pointer এর মতো, কারণ এটি একটি ঠিকানা যা অনুসরণ করে সেই ঠিকানায় স্টোর করা ডেটা অ্যাক্সেস করতে পারি; সেই ডেটা অন্য কোনো ভেরিয়েবলের মালিকানাধীন। Pointer এর বিপরীতে, একটি reference নিশ্চিত করে যে সেই reference এর জীবনকালে এটি একটি নির্দিষ্ট ধরণের বৈধ মান নির্দেশ করবে।

এখানে একটি `calculate_length` ফাংশন কিভাবে সংজ্ঞায়িত এবং ব্যবহার করবেন, যেখানে ভ্যালুর মালিকানা নেওয়ার পরিবর্তে প্যারামিটার হিসাবে একটি অবজেক্টের reference থাকবে:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

</Listing>

প্রথমে, লক্ষ্য করুন যে ভেরিয়েবল ডিক্লারেশনে এবং ফাংশনের return ভ্যালুতে tuple কোড নেই। দ্বিতীয়ত, মনে রাখবেন যে আমরা `calculate_length` এ `&s1` পাস করি এবং এর সংজ্ঞায় আমরা `String` এর পরিবর্তে `&String` নিই। এই ampersand গুলি _reference_ প্রতিনিধিত্ব করে, এবং তারা আপনাকে এর মালিকানা না নিয়ে কিছু মান উল্লেখ করার অনুমতি দেয়। Figure 4-6 এই ধারণাটি চিত্রিত করে।

<img alt="Three tables: the table for s contains only a pointer to the table
for s1. The table for s1 contains the stack data for s1 and points to the
string data on the heap." src="img/trpl04-06.svg" class="center" />

<span class="caption">Figure 4-6: `String s1` কে নির্দেশ করে এমন `&String s` এর একটি ডায়াগ্রাম</span>

> মনে রাখবেন: `&` ব্যবহার করে referencing এর বিপরীত হল _dereferencing_, যা dereference অপারেটর `*` দিয়ে সম্পন্ন করা হয়। আমরা Chapter 8 এ dereference অপারেটরের কিছু ব্যবহার দেখব এবং Chapter 15 এ dereferencing এর বিবরণ নিয়ে আলোচনা করব।

আসুন এখানে ফাংশন কলের দিকে আরও ঘনিষ্ঠভাবে দেখে নিই:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

`&s1` সিনট্যাক্স আমাদের একটি reference তৈরি করতে দেয় যা `s1` এর মানকে _নির্দেশ করে_ কিন্তু এর মালিক নয়। যেহেতু reference এটির মালিক নয়, তাই যখন reference ব্যবহার করা বন্ধ হয়ে যায় তখন এটি যে মানকে নির্দেশ করে তা drop হবে না।

একইভাবে, ফাংশনের সিগনেচার `&` ব্যবহার করে নির্দেশ করে যে প্যারামিটার `s` এর টাইপ হল একটি reference। আসুন কিছু ব্যাখ্যাপূর্ণ অ্যানোটেশন যোগ করি:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

যে scope এ ভেরিয়েবল `s` বৈধ, তা যেকোনো ফাংশন প্যারামিটারের scope এর মতোই, কিন্তু reference দ্বারা নির্দেশিত মানটি drop হয় না যখন `s` ব্যবহার করা বন্ধ হয়ে যায়, কারণ `s` এর ownership নেই। যখন ফাংশনে প্যারামিটার হিসাবে আসল মানের পরিবর্তে reference থাকে, তখন ownership ফেরত দেওয়ার জন্য আমাদের মানগুলি ফেরত দেওয়ার প্রয়োজন হবে না, কারণ আমাদের কাছে কখনই ownership ছিল না।

একটি reference তৈরি করার কাজটিকে আমরা _borrowing_ বলি। বাস্তব জীবনের মতো, যদি কোনো ব্যক্তির কিছু থাকে, তবে আপনি তার কাছ থেকে ধার নিতে পারেন। আপনার কাজ শেষ হয়ে গেলে, আপনাকে এটি ফেরত দিতে হবে। আপনি এটির মালিক নন।

সুতরাং, যদি আমরা ধার করা কিছু পরিবর্তন করার চেষ্টা করি তবে কী হবে? Listing 4-6 এর কোডটি চেষ্টা করুন। স্পয়লার অ্যালার্ট: এটি কাজ করে না!

<Listing number="4-6" file-name="src/main.rs" caption="একটি ধার করা মান পরিবর্তন করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

</Listing>

এখানে ত্রুটিটি রয়েছে:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

যেহেতু ভেরিয়েবলগুলি ডিফল্টরূপে immutable, তাই reference গুলিও তাই। আমরা এমন কিছু পরিবর্তন করার অনুমতি নেই যার একটি reference আমাদের কাছে আছে।

### Mutable Reference

আমরা Listing 4-6 এর কোডটি ঠিক করতে পারি যাতে একটি ধার করা মান পরিবর্তন করার অনুমতি দেওয়া যায়, শুধুমাত্র কয়েকটি ছোট পরিবর্তনের মাধ্যমে, যা পরিবর্তে একটি _mutable reference_ ব্যবহার করে:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

</Listing>

প্রথমে আমরা `s` কে `mut` এ পরিবর্তন করি। তারপর আমরা `&mut s` দিয়ে একটি mutable reference তৈরি করি যেখানে আমরা `change` ফাংশন কল করি এবং ফাংশনের সিগনেচার আপডেট করে `some_string: &mut String` দিয়ে একটি mutable reference গ্রহণ করি। এটি খুব স্পষ্ট করে তোলে যে `change` ফাংশনটি ধার করা মান পরিবর্তন করবে।

Mutable reference এর একটি বড় সীমাবদ্ধতা রয়েছে: যদি আপনার কাছে কোনো মানের mutable reference থাকে, তবে সেই মানের অন্য কোনো reference থাকতে পারবে না। `s` এর দুটি mutable reference তৈরি করার চেষ্টা করলে এই কোডটি ব্যর্থ হবে:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

</Listing>

এখানে ত্রুটিটি রয়েছে:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

এই ত্রুটিটি বলছে যে এই কোডটি অবৈধ কারণ আমরা একই সময়ে একাধিকবার `s` কে mutable হিসাবে borrow করতে পারি না। প্রথম mutable borrow টি `r1` এ আছে এবং `println!` এ ব্যবহার না করা পর্যন্ত স্থায়ী হতে হবে, তবে সেই mutable reference তৈরি এবং এর ব্যবহারের মধ্যে, আমরা `r2` এ আরেকটি mutable reference তৈরি করার চেষ্টা করেছি যা `r1` এর মতো একই ডেটা borrow করে।

একই সময়ে একই ডেটার একাধিক mutable reference প্রতিরোধ করার সীমাবদ্ধতা পরিবর্তনের অনুমতি দেয় তবে খুব নিয়ন্ত্রিত উপায়ে। এটি এমন কিছু যা নতুন Rustacean দের সাথে সংগ্রাম করতে হয় কারণ বেশিরভাগ ভাষা আপনাকে যখনই চান পরিবর্তন করার অনুমতি দেয়। এই সীমাবদ্ধতা রাখার সুবিধা হল Rust compile time এ ডেটা রেস প্রতিরোধ করতে পারে। একটি _data race_ একটি race condition এর মতো এবং ঘটে যখন এই তিনটি আচরণ ঘটে:

- দুই বা ততোধিক pointer একই সময়ে একই ডেটা অ্যাক্সেস করে।
- pointer গুলোর মধ্যে অন্তত একটি ডেটাতে লেখার জন্য ব্যবহার করা হচ্ছে।
- ডেটাতে অ্যাক্সেস সিঙ্ক্রোনাইজ করার জন্য কোনো প্রক্রিয়া ব্যবহার করা হচ্ছে না।

Data race অনির্ধারিত আচরণের কারণ হয় এবং রানটাইমে সেগুলি খুঁজে বের করার চেষ্টা করার সময় নির্ণয় এবং ঠিক করা কঠিন হতে পারে; Rust ডেটা রেস সহ কোড কম্পাইল করতে অস্বীকার করে এই সমস্যা প্রতিরোধ করে!

সবসময়, আমরা একটি নতুন scope তৈরি করতে curly bracket ব্যবহার করতে পারি, একাধিক mutable reference এর অনুমতি দেওয়ার জন্য, তবে _একযোগে_ নয়:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Rust mutable এবং immutable reference একত্রিত করার জন্য একই ধরনের নিয়ম প্রয়োগ করে। এই কোডটি একটি ত্রুটির ফলস্বরূপ:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

এখানে ত্রুটিটি রয়েছে:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

উফ! একই মানের জন্য immutable reference থাকা অবস্থায় আমরা একটি mutable reference রাখতে পারি _না_।

একটি immutable reference এর ব্যবহারকারীরা আশা করেন না যে মানটি তাদের চোখের সামনে হঠাৎ করে পরিবর্তন হয়ে যাবে! তবে, একাধিক immutable reference অনুমোদিত কারণ যে ডেটা পড়ছে তার কারও কাছেই অন্য কারও ডেটা পড়াকে প্রভাবিত করার ক্ষমতা নেই।

মনে রাখবেন যে একটি reference এর scope শুরু হয় যেখানে এটি প্রবর্তিত হয় এবং সেই reference টি শেষবার ব্যবহার না করা পর্যন্ত চলতে থাকে। উদাহরণস্বরূপ, এই কোডটি কম্পাইল হবে কারণ immutable reference গুলির শেষ ব্যবহার `println!` এ, mutable reference প্রবর্তিত হওয়ার আগে:

```rust,edition2021
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

Immutable reference `r1` এবং `r2` এর scope `println!` এর পরে শেষ হয় যেখানে সেগুলি শেষবার ব্যবহার করা হয়েছে, যা mutable reference `r3` তৈরি হওয়ার আগে। এই scope গুলো overlap করে না, তাই এই কোডটি অনুমোদিত: কম্পাইলার বলতে পারে যে scope এর শেষ হওয়ার আগে reference টি আর ব্যবহার করা হচ্ছে না।

এমনকি যদি borrowing এর ত্রুটিগুলি মাঝে মাঝে হতাশাজনক হয়, তবে মনে রাখবেন যে Rust কম্পাইলার সম্ভাব্য বাগটিকে প্রথম দিকে (রানটাইমের পরিবর্তে compile time এ) নির্দেশ করে এবং আপনাকে ঠিক কোথায় সমস্যা তা দেখাচ্ছে। তাহলে আপনার ডেটা কেন আপনি যা ভেবেছিলেন তেমন নয় তা খুঁজে বের করতে হবে না।

### Dangling Reference

Pointer সহ ভাষাগুলিতে, ভুলভাবে একটি _dangling pointer_ তৈরি করা সহজ—একটি pointer যা মেমরির এমন একটি লোকেশন নির্দেশ করে যা অন্য কাউকে দেওয়া হতে পারে—সেই মেমরির একটি pointer রেখে সেই মেমরি free করার মাধ্যমে। বিপরীতে, Rust এ, কম্পাইলার গ্যারান্টি দেয় যে reference গুলি কখনই dangling reference হবে না: যদি আপনার কাছে কিছু ডেটার reference থাকে, তবে কম্পাইলার নিশ্চিত করবে যে ডেটার reference এর আগে ডেটা scope এর বাইরে যাবে না।

আসুন একটি dangling reference তৈরি করার চেষ্টা করি, Rust compile time ত্রুটির মাধ্যমে কীভাবে সেগুলি প্রতিরোধ করে তা দেখার জন্য:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

</Listing>

এখানে ত্রুটিটি রয়েছে:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

এই ত্রুটি বার্তাটি এমন একটি বৈশিষ্ট্য নির্দেশ করে যা আমরা এখনও আলোচনা করিনি: lifetimes। আমরা Chapter 10 এ lifetimes নিয়ে বিস্তারিত আলোচনা করব। তবে, আপনি যদি lifetimes সম্পর্কে অংশগুলি উপেক্ষা করেন, তবে বার্তাটিতে এই কোডটি কেন সমস্যা তার মূল কারণ রয়েছে:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

আসুন আমাদের `dangle` কোডের প্রতিটি পর্যায়ে কী ঘটছে তা আরও ঘনিষ্ঠভাবে দেখে নিই:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

</Listing>

যেহেতু `s` `dangle` এর ভিতরে তৈরি করা হয়েছে, যখন `dangle` এর কোড শেষ হয়ে যায়, তখন `s` ডিঅ্যালোকেট হয়ে যাবে। কিন্তু আমরা এটির একটি reference ফেরত দেওয়ার চেষ্টা করেছি। এর মানে হল এই reference একটি অবৈধ `String` কে নির্দেশ করবে। এটা ভালো না! Rust আমাদের এটা করতে দেবে না।

এখানে সমাধান হল সরাসরি `String` ফেরত দেওয়া:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

এটি কোনো সমস্যা ছাড়াই কাজ করে। Ownership move করা হয়েছে, এবং কিছুই ডিঅ্যালোকেট করা হয়নি।

### Reference এর নিয়ম

আসুন আমরা reference সম্পর্কে যা আলোচনা করেছি তার সারসংক্ষেপ করি:

- যে কোনো সময়ে, আপনার কাছে _হয়_ একটি mutable reference _অথবা_ যেকোনো সংখ্যক immutable reference থাকতে পারে।
- Reference সবসময় বৈধ হতে হবে।

এর পরে, আমরা অন্য ধরনের reference দেখব: slices।
