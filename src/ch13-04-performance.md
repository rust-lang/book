## পারফরম্যান্স তুলনা: লুপ বনাম ইটারেটর

লুপ ব্যবহার করবেন নাকি ইটারেটর ব্যবহার করবেন তা নির্ধারণ করতে, আপনাকে জানতে হবে কোন ইমপ্লিমেন্টেশনটি দ্রুততর: `search` ফাংশনের explicit `for` লুপ সহ ভার্সন নাকি ইটারেটর সহ ভার্সন।

আমরা স্যার আর্থার কোনান ডয়েলের লেখা _The Adventures of Sherlock Holmes_-এর সম্পূর্ণ বিষয়বস্তু একটি `String`-এ লোড করে এবং contents-এর মধ্যে _the_ শব্দটি খুঁজে বের করে একটি বেঞ্চমার্ক চালিয়েছি। `For` লুপ ব্যবহার করে `search`-এর ভার্সন এবং ইটারেটর ব্যবহার করা ভার্সনের বেঞ্চমার্কের ফলাফল এখানে দেওয়া হল:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

দুটি implementation-এর performance প্রায় একই! আমরা এখানে বেঞ্চমার্ক কোড ব্যাখ্যা করব না, কারণ উদ্দেশ্য এটা প্রমাণ করা নয় যে দুটি ভার্সন equivalent, বরং এই দুটি implementation কীভাবে performance-এর দিক থেকে compare করে তার একটি general sense পাওয়া।

আরও comprehensive বেঞ্চমার্কের জন্য, আপনার `contents` হিসেবে বিভিন্ন আকারের various text, `query` হিসেবে ভিন্ন শব্দ এবং ভিন্ন দৈর্ঘ্যের শব্দ এবং অন্যান্য সমস্ত variation ব্যবহার করে পরীক্ষা করা উচিত। মূল বিষয়টি হল: ইটারেটরগুলো, যদিও একটি high-level abstraction, তবুও মোটামুটি একই কোডে compile হয় যেমনটি আপনি নিজে lower-level কোড লিখলে হত। ইটারেটর হল Rust-এর _zero-cost abstraction_-গুলোর মধ্যে একটি, যার অর্থ হল abstraction ব্যবহার করলে কোনো additional runtime overhead যুক্ত হয় না। এটি C++-এর original designer এবং implementor, Bjarne Stroustrup-এর "Foundations of C++" (2012)-এ _zero-overhead_-কে যেভাবে define করেছেন তার অনুরূপ:

> সাধারণভাবে, C++ implementation গুলো zero-overhead নীতি মেনে চলে: আপনি যা ব্যবহার করেন না, তার জন্য আপনাকে মূল্য দিতে হবে না। এবং আরও: আপনি যা ব্যবহার করেন, আপনি নিজে এর চেয়ে ভালো কোড করতে পারতেন না।

আরেকটি উদাহরণ হিসেবে, নিম্নলিখিত কোডটি একটি অডিও ডিকোডার থেকে নেওয়া হয়েছে। ডিকোডিং অ্যালগরিদম previous sample-গুলোর একটি linear function-এর উপর ভিত্তি করে future value গুলো estimate করতে linear prediction mathematical operation ব্যবহার করে। এই কোডটি scope-এর তিনটি variable-এর উপর কিছু math করার জন্য একটি ইটারেটর চেইন ব্যবহার করে: ডেটার একটি `buffer` স্লাইস, 12টি `coefficients`-এর একটি অ্যারে এবং `qlp_shift`-এ ডেটা shift করার একটি পরিমাণ। আমরা এই উদাহরণের মধ্যে variable গুলো declare করেছি কিন্তু সেগুলোকে কোনো value দিইনি; যদিও এই কোডটির context-এর বাইরে খুব বেশি অর্থ নেই, তবুও এটি একটি concise, real-world উদাহরণ যে কীভাবে Rust high-level idea গুলোকে low-level কোডে translate করে।

```rust,ignore
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

`Prediction`-এর value calculate করার জন্য, এই কোডটি `coefficients`-এর 12টি value-এর প্রতিটির উপর iterate করে এবং `zip` মেথড ব্যবহার করে coefficient value গুলোর সাথে `buffer`-এর previous 12টি value-এর pair তৈরি করে। তারপর, প্রতিটি pair-এর জন্য, আমরা value গুলোকে একসাথে গুণ করি, সমস্ত result যোগ করি এবং যোগফলের বিটগুলোকে `qlp_shift` বিট ডানদিকে shift করি।

অডিও ডিকোডারের মতো অ্যাপ্লিকেশনগুলোতে প্রায়শই calculation-গুলো performance-কে সবচেয়ে বেশি priority দেয়। এখানে, আমরা একটি ইটারেটর তৈরি করছি, দুটি অ্যাডাপ্টার ব্যবহার করছি এবং তারপর value টি consume করছি। এই Rust কোডটি কোন অ্যাসেম্বলি কোডে compile হবে? আচ্ছা, এই লেখার সময় পর্যন্ত, এটি সেই একই অ্যাসেম্বলিতে compile হয় যা আপনি নিজে লিখতেন। `Coefficients`-এর value-গুলোর উপর iteration-এর সাথে সম্পর্কিত কোনো লুপ নেই: Rust জানে যে 12টি iteration রয়েছে, তাই এটি লুপটিকে "আনরোল" করে। _Unrolling_ হল একটি অপ্টিমাইজেশন যা লুপ কন্ট্রোলিং কোডের overhead সরিয়ে দেয় এবং পরিবর্তে লুপের প্রতিটি পুনরাবৃত্তির জন্য repetitive কোড তৈরি করে।

সমস্ত coefficient গুলো register-এ store করা হয়, যার মানে value গুলো অ্যাক্সেস করা খুব দ্রুত। Runtime-এ অ্যারে অ্যাক্সেসে কোনো সীমা পরীক্ষা নেই। Rust যে সমস্ত অপ্টিমাইজেশন প্রয়োগ করতে সক্ষম সেগুলো resulting কোডকে অত্যন্ত efficient করে তোলে। এখন আপনি এটা জানেন, আপনি ভয় ছাড়াই ইটারেটর এবং ক্লোজার ব্যবহার করতে পারেন! সেগুলো কোডকে এমনভাবে দেখায় যেন এটি higher level-এর, কিন্তু এটি করার জন্য runtime performance penalty আরোপ করে না।

## সারসংক্ষেপ

ক্লোজার এবং ইটারেটর হল Rust-এর feature যা functional programming language-এর idea দ্বারা অনুপ্রাণিত। Low-level performance-এ high-level idea গুলোকে clearly express করার জন্য Rust-এর సామর্থ্যে এরা অবদান রাখে। ক্লোজার এবং ইটারেটরগুলোর implementation এমন যে runtime performance প্রভাবিত হয় না। এটি Rust-এর zero-cost abstraction provide করার প্রচেষ্টার লক্ষ্যের অংশ।

এখন যেহেতু আমরা আমাদের I/O প্রোজেক্টের expressiveness improve করেছি, আসুন `cargo`-এর আরও কিছু feature দেখি যা আমাদের প্রোজেক্টটিকে বিশ্বের সাথে share করতে সাহায্য করবে।
