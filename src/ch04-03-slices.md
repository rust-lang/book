## Slice টাইপ

_Slices_ আপনাকে সম্পূর্ণ কালেকশনের পরিবর্তে [collection](ch08-00-common-collections.md) এর মধ্যে থাকা উপাদানগুলির একটি ধারাবাহিক সিকোয়েন্স reference করতে দেয়। একটি slice হল এক ধরনের reference, তাই এটির ownership নেই।

এখানে একটি ছোট প্রোগ্রামিং সমস্যা রয়েছে: একটি ফাংশন লিখুন যা স্পেস দ্বারা পৃথক করা শব্দগুলির একটি স্ট্রিং নেয় এবং সেই স্ট্রিংটিতে পাওয়া প্রথম শব্দটি ফেরত দেয়। যদি ফাংশনটি স্ট্রিংটিতে কোনো স্পেস না পায়, তাহলে পুরো স্ট্রিংটি একটি শব্দ হতে হবে, তাই পুরো স্ট্রিংটি ফেরত দেওয়া উচিত।

Slice ব্যবহার না করে আমরা এই ফাংশনের সিগনেচার কিভাবে লিখব তা দেখে নেওয়া যাক, যাতে slice যে সমস্যা সমাধান করবে তা বোঝা যায়:

```rust,ignore
fn first_word(s: &String) -> ?
```

`first_word` ফাংশনের একটি প্যারামিটার হিসেবে `&String` আছে। আমাদের ownership এর প্রয়োজন নেই, তাই এটি ঠিক আছে। (idiomatic Rust এ, ফাংশনগুলি তাদের আর্গুমেন্টের ownership নেয় না যতক্ষণ না তাদের প্রয়োজন হয়, এবং এর কারণগুলি আমরা যখন এগোতে থাকব তখন পরিষ্কার হয়ে যাবে!) কিন্তু আমাদের কী ফেরত দেওয়া উচিত? স্ট্রিং এর একটি অংশ সম্পর্কে কথা বলার মতো কোনো উপায় আমাদের কাছে নেই। তবে, আমরা শব্দের শেষ ইন্ডেক্স ফেরত দিতে পারি, যা একটি স্পেস দ্বারা চিহ্নিত করা হয়। Listing 4-7 এ দেখানো হিসাবে, আসুন চেষ্টা করি।

<Listing number="4-7" file-name="src/main.rs" caption="`first_word` ফাংশন যা `String` প্যারামিটারে একটি byte index মান ফেরত দেয়">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

</Listing>

যেহেতু আমাদের `String` এর প্রতিটি উপাদানের মধ্যে দিয়ে যেতে হবে এবং একটি মান স্পেস কিনা তা পরীক্ষা করতে হবে, তাই আমরা `as_bytes` পদ্ধতি ব্যবহার করে আমাদের `String` কে বাইটের একটি অ্যারেতে রূপান্তর করব।

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

এর পরে, আমরা `iter` পদ্ধতি ব্যবহার করে বাইটের অ্যারের উপর একটি iterator তৈরি করি:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

আমরা [Chapter 13][ch13]<!-- ignore --> এ iterator নিয়ে আরও বিস্তারিত আলোচনা করব। আপাতত, জেনে রাখুন যে `iter` হল একটি পদ্ধতি যা একটি কালেকশনের প্রতিটি উপাদান ফেরত দেয় এবং `enumerate` `iter` এর ফলাফল মোড়ানো এবং প্রতিটি উপাদানকে পরিবর্তে একটি tuple এর অংশ হিসাবে ফেরত দেয়। `enumerate` থেকে ফেরত দেওয়া tuple এর প্রথম উপাদানটি হল ইন্ডেক্স এবং দ্বিতীয় উপাদানটি হল উপাদানের একটি reference। এটি নিজে থেকে ইন্ডেক্স গণনা করার চেয়ে একটু বেশি সুবিধাজনক।

যেহেতু `enumerate` পদ্ধতি একটি tuple ফেরত দেয়, তাই আমরা সেই tuple কে destructure করতে pattern ব্যবহার করতে পারি। আমরা [Chapter 6][ch6]<!-- ignore --> এ pattern নিয়ে আরও আলোচনা করব। `for` লুপে, আমরা একটি pattern নির্দিষ্ট করি যেখানে tuple এর ইন্ডেক্সের জন্য `i` এবং tuple এর একক বাইটের জন্য `&item` রয়েছে। যেহেতু আমরা `.iter().enumerate()` থেকে উপাদানের একটি reference পাই, তাই আমরা pattern এ `&` ব্যবহার করি।

`for` লুপের ভিতরে, আমরা byte literal সিনট্যাক্স ব্যবহার করে স্পেস নির্দেশ করে এমন বাইটটি খুঁজি। যদি আমরা একটি স্পেস খুঁজে পাই, তবে আমরা অবস্থানটি ফেরত দিই। অন্যথায়, আমরা `s.len()` ব্যবহার করে স্ট্রিংটির দৈর্ঘ্য ফেরত দিই।

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

এখন আমাদের কাছে স্ট্রিং-এর প্রথম শব্দের শেষ ইন্ডেক্স বের করার একটি উপায় আছে, তবে একটি সমস্যা আছে। আমরা একটি `usize` ফেরত দিচ্ছি, তবে এটি শুধুমাত্র `&String` এর প্রেক্ষাপটে একটি অর্থপূর্ণ সংখ্যা। অন্য কথায়, যেহেতু এটি `String` থেকে একটি আলাদা মান, তাই ভবিষ্যতে এটি বৈধ থাকবে তার কোনো গ্যারান্টি নেই। Listing 4-8 এ প্রোগ্রামটি বিবেচনা করুন যা Listing 4-7 থেকে `first_word` ফাংশন ব্যবহার করে।

<Listing number="4-8" file-name="src/main.rs" caption="`first_word` ফাংশন কল করার ফলাফল স্টোর করা এবং তারপর `String` কন্টেন্ট পরিবর্তন করা">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

</Listing>

এই প্রোগ্রামটি কোনো ত্রুটি ছাড়াই কম্পাইল হয় এবং `s.clear()` কল করার পরেও যদি আমরা `word` ব্যবহার করি তাহলেও তাই করবে। যেহেতু `word` কোনোভাবেই `s` এর অবস্থার সাথে যুক্ত নয়, `word` এ এখনও `5` মান আছে। প্রথম শব্দটি বের করার চেষ্টা করার জন্য আমরা `5` মানটিকে ভেরিয়েবল `s` এর সাথে ব্যবহার করতে পারতাম, তবে এটি একটি বাগ হবে কারণ `word` এ `5` সংরক্ষণ করার পর থেকে `s` এর কন্টেন্ট পরিবর্তিত হয়েছে।

`word` এর ইন্ডেক্স `s` এর ডেটার সাথে সিঙ্ক না হওয়া নিয়ে চিন্তা করা ক্লান্তিকর এবং ত্রুটিপূর্ণ! যদি আমরা একটি `second_word` ফাংশন লিখি, তাহলে এই ইন্ডেক্সগুলি ম্যানেজ করা আরও কঠিন। এর সিগনেচারটি দেখতে এরকম হতে হবে:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

এখন আমরা একটি শুরুর _এবং_ শেষের ইন্ডেক্স ট্র্যাক করছি, এবং আমাদের কাছে আরও বেশি মান রয়েছে যা একটি নির্দিষ্ট অবস্থার ডেটা থেকে গণনা করা হয়েছে তবে সেই অবস্থার সাথে মোটেও আবদ্ধ নয়। আমাদের চারপাশে তিনটি সম্পর্কহীন ভেরিয়েবল ভাসছে যেগুলিকে সিঙ্ক রাখতে হবে।

সৌভাগ্যবশত, Rust এর এই সমস্যার সমাধান আছে: string slices।

### String Slices

একটি _string slice_ হল একটি `String` এর অংশের reference, এবং এটি দেখতে এইরকম:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

সম্পূর্ণ `String` এর reference এর পরিবর্তে, `hello` হল `String` এর একটি অংশের reference, যা অতিরিক্ত `[0..5]` বিটে নির্দিষ্ট করা হয়েছে। আমরা `[starting_index..ending_index]` উল্লেখ করে ব্র্যাকেটের মধ্যে একটি range ব্যবহার করে slice তৈরি করি, যেখানে `starting_index` হল slice এর প্রথম অবস্থান এবং `ending_index` হল slice এর শেষ অবস্থানের থেকে এক বেশি। অভ্যন্তরীণভাবে, slice ডেটা স্ট্রাকচার slice এর শুরুর অবস্থান এবং দৈর্ঘ্য স্টোর করে, যা `ending_index` মাইনাস `starting_index` এর সাথে মিলে যায়। সুতরাং, `let world = &s[6..11];` এর ক্ষেত্রে, `world` একটি slice হবে যাতে `s` এর 6 নম্বর ইন্ডেক্সের বাইটের একটি pointer থাকবে এবং যার দৈর্ঘ্যের মান `5` হবে।

Figure 4-7 একটি ডায়াগ্রামে এটি দেখায়।

<img alt="Three tables: a table representing the stack data of s, which points
to the byte at index 0 in a table of the string data &quot;hello world&quot; on
the heap. The third table rep-resents the stack data of the slice world, which
has a length value of 5 and points to byte 6 of the heap data table."
src="img/trpl04-07.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-7: `String` এর একটি অংশ নির্দেশ করে String Slice</span>

Rust এর `..` range সিনট্যাক্সের সাথে, আপনি যদি 0 ইন্ডেক্স থেকে শুরু করতে চান, তবে আপনি দুটি ডটের আগের মানটি বাদ দিতে পারেন। অন্য কথায়, এই দুটি সমান:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

একইভাবে, যদি আপনার slice এ `String` এর শেষ বাইট অন্তর্ভুক্ত থাকে, তবে আপনি trailing নম্বরটি বাদ দিতে পারেন। এর মানে হল এই দুটি সমান:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

আপনি পুরো স্ট্রিংটির একটি slice নিতে উভয় মানও বাদ দিতে পারেন। সুতরাং এই দুটি সমান:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> মনে রাখবেন: String slice range ইন্ডেক্সগুলি অবশ্যই বৈধ UTF-8 ক্যারেক্টার সীমানায় হতে হবে। আপনি যদি একটি মাল্টিবাইট ক্যারেক্টারের মাঝে একটি স্ট্রিং slice তৈরি করার চেষ্টা করেন, তবে আপনার প্রোগ্রামটি একটি ত্রুটি সহ শেষ হবে। স্ট্রিং slices প্রবর্তনের উদ্দেশ্যে, আমরা এই বিভাগে শুধুমাত্র ASCII ধরে নিচ্ছি; UTF-8 হ্যান্ডলিং এর আরও বিস্তারিত আলোচনা [“Storing UTF-8 Encoded Text with Strings”][strings]<!-- ignore --> বিভাগে Chapter 8 এ আছে।

এই সমস্ত তথ্য মনে রেখে, আসুন `first_word` কে একটি slice রিটার্ন করার জন্য পুনরায় লিখি। "string slice" বোঝাতে যে টাইপটি ব্যবহার করা হয়, তা `&str` হিসাবে লেখা হয়:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

</Listing>

স্পেসের প্রথম ঘটনাটি খুঁজে বের করে আমরা Listing 4-7 এর মতোই শব্দের শেষের ইন্ডেক্স পাই। যখন আমরা একটি স্পেস খুঁজে পাই, তখন আমরা স্ট্রিং এর শুরু এবং স্পেসের ইন্ডেক্সকে শুরুর এবং শেষের ইন্ডেক্স হিসাবে ব্যবহার করে একটি string slice রিটার্ন করি।

এখন যখন আমরা `first_word` কল করি, তখন আমরা একটি একক মান ফেরত পাই যা অন্তর্নিহিত ডেটার সাথে আবদ্ধ। মানটি slice এর শুরুর পয়েন্টের reference এবং slice এর উপাদানের সংখ্যা দিয়ে গঠিত।

একটি `second_word` ফাংশনের জন্যও একটি slice রিটার্ন করা কাজ করবে:

```rust,ignore
fn second_word(s: &String) -> &str {
```

আমাদের কাছে এখন একটি সরল API রয়েছে যা ভুল করা অনেক কঠিন, কারণ কম্পাইলার নিশ্চিত করবে যে `String` এর reference গুলি বৈধ থাকে। Listing 4-8 এর প্রোগ্রামের বাগটির কথা মনে আছে, যখন আমরা প্রথম শব্দের শেষের ইন্ডেক্স পেয়েছিলাম কিন্তু তারপরে স্ট্রিংটি খালি করেছিলাম তাই আমাদের ইন্ডেক্সটি অবৈধ ছিল? সেই কোডটি যুক্তিসঙ্গতভাবে ভুল ছিল কিন্তু কোনো তাৎক্ষণিক ত্রুটি দেখায়নি। যদি আমরা একটি খালি স্ট্রিং এর সাথে প্রথম শব্দের ইন্ডেক্স ব্যবহার করার চেষ্টা করতাম তাহলে সমস্যাগুলো পরে দেখা যেত। Slices এই বাগটিকে অসম্ভব করে তোলে এবং আমাদের কোডে সমস্যা আছে তা অনেক আগে জানতে দেয়। `first_word` এর slice সংস্করণ ব্যবহার করলে compile time এ একটি ত্রুটি আসবে:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

</Listing>

এখানে কম্পাইলার ত্রুটি রয়েছে:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

borrowing এর নিয়ম থেকে মনে রাখবেন যে যদি আমাদের কাছে কোনো কিছুর immutable reference থাকে, তবে আমরা একটি mutable reference ও নিতে পারি না। যেহেতু `clear` এর `String` কে truncate করার প্রয়োজন, তাই এর একটি mutable reference দরকার। `clear` এ কল করার পরে `println!` `word` এ reference ব্যবহার করে, তাই সেই সময়ে immutable reference টি এখনও সক্রিয় থাকতে হবে। Rust একই সময়ে `clear` এর mutable reference এবং `word` এর immutable reference কে বিদ্যমান থাকতে দেয় না এবং কম্পাইলেশন ব্যর্থ হয়। Rust শুধুমাত্র আমাদের API কে ব্যবহার করা সহজ করেনি, বরং compile time এ সম্পূর্ণ শ্রেণীর ত্রুটিও দূর করেছে!

<!-- Old heading. Do not remove or links may break. -->

<a id="string-literals-are-slices"></a>

#### String Literal Slices হিসাবে

মনে রাখবেন যে আমরা string literal গুলি বাইনারির ভিতরে স্টোর করা নিয়ে কথা বলেছি। এখন যেহেতু আমরা slices সম্পর্কে জানি, আমরা string literal গুলিকে সঠিকভাবে বুঝতে পারি:

```rust
let s = "Hello, world!";
```

এখানে `s` এর টাইপ হল `&str`: এটি বাইনারির সেই নির্দিষ্ট পয়েন্ট নির্দেশ করে এমন একটি slice। এই কারণেই string literal গুলি immutable; `&str` একটি immutable reference।

#### প্যারামিটার হিসাবে String Slices

literals এবং `String` ভ্যালুগুলির slice নিতে পারার বিষয়টি জেনে, `first_word` এর আরও একটি উন্নতি করা যায়, আর সেটি হল এর সিগনেচার:

```rust,ignore
fn first_word(s: &String) -> &str {
```

একজন অভিজ্ঞ Rustacean Listing 4-9 এ দেখানো সিগনেচারটি লিখতেন, কারণ এটি আমাদের `&String` ভ্যালু এবং `&str` ভ্যালু উভয় ক্ষেত্রেই একই ফাংশন ব্যবহার করতে দেয়।

<Listing number="4-9" caption="`s` প্যারামিটারের টাইপের জন্য একটি string slice ব্যবহার করে `first_word` ফাংশন উন্নত করা">

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

</Listing>

যদি আমাদের একটি string slice থাকে, তবে আমরা সেটি সরাসরি পাস করতে পারি। যদি আমাদের একটি `String` থাকে, তবে আমরা `String` এর একটি slice বা `String` এর reference পাস করতে পারি। এই নমনীয়তা _deref coercions_ এর সুবিধা নেয়, এমন একটি বৈশিষ্ট্য যা আমরা Chapter 15 এর [“Implicit Deref Coercions with Functions and Methods”][deref-coercions]<!--ignore--> বিভাগে আলোচনা করব।

`String` এর reference এর পরিবর্তে একটি string slice নিতে একটি ফাংশন সংজ্ঞায়িত করা আমাদের API কে আরও সাধারণ এবং কার্যকরী করে তোলে কোনো কার্যকারিতা না হারিয়ে:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

</Listing>

### অন্যান্য Slices

String slices, আপনি যেমন কল্পনা করতে পারেন, স্ট্রিংগুলির জন্য নির্দিষ্ট। কিন্তু আরও সাধারণ slice টাইপও রয়েছে। এই অ্যারেটি বিবেচনা করুন:

```rust
let a = [1, 2, 3, 4, 5];
```

ঠিক যেমন আমরা একটি স্ট্রিং এর অংশ উল্লেখ করতে চাই, তেমনি আমরা একটি অ্যারের অংশ উল্লেখ করতে চাইতে পারি। আমরা এটি এভাবে করব:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

এই slice এর টাইপ `&[i32]`। এটি string slice এর মতোই কাজ করে, প্রথম উপাদানের একটি reference এবং একটি দৈর্ঘ্য স্টোর করে। আপনি অন্যান্য সব ধরনের কালেকশনের জন্য এই ধরনের slice ব্যবহার করবেন। Chapter 8 এ ভেক্টর নিয়ে আলোচনা করার সময় আমরা এই কালেকশনগুলি নিয়ে বিস্তারিত আলোচনা করব।

## সারসংক্ষেপ

Ownership, borrowing, এবং slices এর ধারণাগুলি compile time এ Rust প্রোগ্রামগুলিতে মেমরি নিরাপত্তা নিশ্চিত করে। Rust ভাষা আপনাকে অন্যান্য সিস্টেম প্রোগ্রামিং ভাষার মতো আপনার মেমরি ব্যবহারের উপর নিয়ন্ত্রণ দেয়, তবে ডেটার owner যখন scope এর বাইরে চলে যায় তখন সেই ডেটা স্বয়ংক্রিয়ভাবে পরিষ্কার করে দেওয়ার অর্থ হল এই নিয়ন্ত্রণ পেতে আপনাকে অতিরিক্ত কোড লিখতে এবং ডিবাগ করতে হবে না।

Ownership Rust এর অন্যান্য অনেক অংশের কাজকে প্রভাবিত করে, তাই আমরা এই ধারণাগুলি নিয়ে বইটির বাকি অংশে আরও আলোচনা করব। আসুন Chapter 5 এ যাই এবং `struct` এ ডেটার অংশগুলিকে একসাথে গোষ্ঠীভুক্ত করা দেখি।

[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#patterns-that-bind-to-values
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[deref-coercions]: ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
