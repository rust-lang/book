## `Box<T>` ব্যবহার করে Heap-এর ডেটার দিকে পয়েন্ট করা

সবচেয়ে straightforward স্মার্ট পয়েন্টার হল একটি _box_, যার টাইপ লেখা হয় `Box<T>`। Box গুলো আপনাকে stack-এর পরিবর্তে heap-এ ডেটা store করার অনুমতি দেয়। Stack-এ যা অবশিষ্ট থাকে তা হল heap ডেটার পয়েন্টার। Stack এবং heap-এর মধ্যে পার্থক্য পর্যালোচনা করতে Chapter 4 দেখুন।

Box-গুলোর পারফরম্যান্স ওভারহেড নেই, stack-এর পরিবর্তে heap-এ তাদের ডেটা store করা ছাড়া। কিন্তু সেগুলোর অনেক extra capabilities-ও নেই। আপনি সেগুলোকে প্রায়শই এই পরিস্থিতিতে ব্যবহার করবেন:

- যখন আপনার কাছে এমন একটি টাইপ থাকে যার আকার compile time-এ জানা যায় না এবং আপনি সেই টাইপের একটি value এমন একটি context-এ ব্যবহার করতে চান যার জন্য একটি exact আকারের প্রয়োজন
- যখন আপনার কাছে প্রচুর পরিমাণে ডেটা থাকে এবং আপনি ownership transfer করতে চান কিন্তু নিশ্চিত করতে চান যে ডেটা copy করা হবে না
- যখন আপনি একটি value-র owner হতে চান এবং আপনি শুধুমাত্র এটি একটি particular trait implement করে এমন একটি টাইপ কিনা তা নিয়ে চিন্তা করেন, specific টাইপের কিনা তা নয়

আমরা প্রথম পরিস্থিতিটি ["বক্স সহ পুনরাবৃত্তিমূলক প্রকারগুলিকে সক্ষম করা"](#enabling-recursive-types-with-boxes) বিভাগে প্রদর্শন করব। দ্বিতীয় ক্ষেত্রে, প্রচুর পরিমাণে ডেটার ownership transfer করতে দীর্ঘ সময় লাগতে পারে কারণ ডেটা stack-এর চারপাশে copy করা হয়। এই পরিস্থিতিতে পারফরম্যান্স improve করার জন্য, আমরা box-এ heap-এর উপর প্রচুর পরিমাণে ডেটা store করতে পারি। তারপর, stack-এর চারপাশে শুধুমাত্র অল্প পরিমাণ পয়েন্টার ডেটা copy করা হয়, যেখানে এটি যে ডেটা refer করে তা heap-এর একটি স্থানে থাকে। তৃতীয় ক্ষেত্রটি _trait object_ নামে পরিচিত, এবং Chapter 18-এ একটি সম্পূর্ণ বিভাগ, ["ভিন্ন প্রকারের মানের জন্য অনুমতি দেয় এমন Trait অবজেক্ট ব্যবহার করা,"][trait-objects] শুধুমাত্র সেই বিষয়ে আলোচনা করা হয়েছে। তাই আপনি এখানে যা শিখবেন তা Chapter 18-এ আবার প্রয়োগ করবেন!

### Heap-এ ডেটা Store করার জন্য একটি `Box<T>` ব্যবহার করা

আমরা `Box<T>`-এর জন্য heap storage use case নিয়ে আলোচনা করার আগে, আমরা syntax এবং `Box<T>`-এর মধ্যে stored value-গুলোর সাথে কীভাবে ইন্টারঅ্যাক্ট করতে হয় তা দেখব।

Listing 15-1 দেখানো হয়েছে কিভাবে heap-এ একটি `i32` value store করতে একটি box ব্যবহার করতে হয়:

<Listing number="15-1" file-name="src/main.rs" caption="একটি box ব্যবহার করে heap-এ একটি `i32` ভ্যালু store করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

</Listing>

আমরা `b` variable-টিকে একটি `Box`-এর value হিসেবে define করি যা `5` value-টির দিকে point করে, যেটি heap-এ allocate করা হয়েছে। এই প্রোগ্রামটি `b = 5` প্রিন্ট করবে; এই ক্ষেত্রে, আমরা box-এর ডেটা অ্যাক্সেস করতে পারি একইভাবে যেভাবে আমরা করতাম যদি এই ডেটা stack-এ থাকত। যেকোনো owned value-এর মতোই, যখন একটি box scope-এর বাইরে চলে যায়, যেমন `b` `main`-এর শেষে করে, তখন এটিকে deallocate করা হবে। Deallocation টি box (স্ট্যাকে সংরক্ষিত) এবং এটি যে ডেটার দিকে point করে (heap-এ সংরক্ষিত) উভয়ের জন্যই ঘটে।

Heap-এ একটি single value রাখা খুব useful নয়, তাই আপনি এভাবে প্রায়শই নিজে থেকে box ব্যবহার করবেন না। Stack-এ একটি single `i32`-এর মতো value থাকা, যেখানে সেগুলো default ভাবে store করা হয়, বেশিরভাগ পরিস্থিতিতে বেশি উপযুক্ত। আসুন এমন একটি ক্ষেত্র দেখি যেখানে box গুলো আমাদের এমন type define করার অনুমতি দেয় যেগুলো আমাদের কাছে box না থাকলে define করার অনুমতি থাকত না।

### Box-এর সাহায্যে Recursive Type গুলো Enable করা

একটি _recursive type_-এর value-র অংশ হিসেবে একই type-এর অন্য value থাকতে পারে। Recursive type গুলো একটি সমস্যা তৈরি করে কারণ, compile time-এ, Rust-কে জানতে হবে একটি type কতটুকু জায়গা নেয়। যাইহোক, recursive type-এর value-গুলোর nesting তাত্ত্বিকভাবে অসীমভাবে চলতে পারে, তাই Rust জানতে পারে না value-টির জন্য কতটুকু জায়গা প্রয়োজন। যেহেতু box-গুলোর একটি known আকার রয়েছে, তাই আমরা recursive type definition-এ একটি box insert করে recursive type গুলোকে enable করতে পারি।

Recursive type-এর একটি উদাহরণ হিসেবে, আসুন _cons list_ explore করি। এটি functional programming language-গুলোতে commonly পাওয়া একটি ডেটা টাইপ। আমরা যে cons list type টি define করব সেটি recursion ছাড়া straightforward; অতএব, আমরা যে উদাহরণের সাথে কাজ করব তার concept গুলো useful হবে যে কোনো সময় আপনি recursive type-এর সাথে জড়িত আরও complex পরিস্থিতিতে পড়লে।

#### Cons List সম্পর্কে আরও তথ্য

একটি _cons list_ হল একটি ডেটা স্ট্রাকচার যা Lisp প্রোগ্রামিং ভাষা এবং এর উপভাষাগুলো থেকে এসেছে এবং এটি nested pair দিয়ে তৈরি, এবং এটি Lisp-এর linked list-এর সংস্করণ। এর নামটি Lisp-এর `cons` ফাংশন (সংক্ষেপে "construct ফাংশন") থেকে এসেছে যা তার দুটি আর্গুমেন্ট থেকে একটি new pair তৈরি করে। একটি value এবং অন্য একটি pair নিয়ে গঠিত একটি pair-এ `cons` কল করে, আমরা recursive pair দিয়ে তৈরি cons list তৈরি করতে পারি।

উদাহরণস্বরূপ, এখানে 1, 2, 3 তালিকা ধারণকারী একটি cons list-এর একটি pseudocode উপস্থাপনা রয়েছে যেখানে প্রতিটি pair বন্ধনীতে রয়েছে:

```text
(1, (2, (3, Nil)))
```

একটি cons list-এর প্রতিটি item-এ দুটি element রয়েছে: current item-এর value এবং next item। List-এর শেষ item-টিতে শুধুমাত্র `Nil` নামক একটি value রয়েছে যেখানে কোনো next item নেই। একটি cons list recursively `cons` ফাংশন কল করে তৈরি করা হয়। Recursion-এর base case বোঝানোর জন্য canonical নামটি হল `Nil`। মনে রাখবেন যে এটি Chapter 6-এ আলোচিত "null" বা "nil" concept-এর মতো নয়, যেটি একটি invalid বা অনুপস্থিত value।

Cons list Rust-এ commonly ব্যবহৃত ডেটা স্ট্রাকচার নয়। বেশিরভাগ সময় যখন আপনার Rust-এ item-গুলোর একটি list থাকে, তখন `Vec<T>` ব্যবহার করা একটি ভাল পছন্দ। অন্যান্য, আরও complex recursive data type গুলো বিভিন্ন পরিস্থিতিতে _useful_, কিন্তু এই chapter-এ cons list দিয়ে শুরু করে, আমরা explore করতে পারি কীভাবে box গুলো আমাদের খুব বেশি বিভ্রান্তি ছাড়াই একটি recursive data type define করতে দেয়।

Listing 15-2 একটি cons list-এর জন্য একটি enum definition ধারণ করে। মনে রাখবেন যে এই কোডটি এখনও compile হবে না কারণ `List` type-টির একটি known আকার নেই, যা আমরা প্রদর্শন করব।

<Listing number="15-2" file-name="src/main.rs" caption="`i32` value-গুলোর একটি cons list ডেটা স্ট্রাকচারকে represent করার জন্য একটি enum define করার প্রথম প্রচেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

</Listing>

> দ্রষ্টব্য: আমরা এই উদাহরণের উদ্দেশ্যে শুধুমাত্র `i32` value ধারণ করে এমন একটি cons list implement করছি। আমরা এটিকে জেনেরিক ব্যবহার করে implement করতে পারতাম, যেমনটি আমরা Chapter 10-এ আলোচনা করেছি, একটি cons list type define করতে যা যেকোনো type-এর value store করতে পারে।

`1, 2, 3` তালিকা store করার জন্য `List` type ব্যবহার করা Listing 15-3-এর কোডের মতো হবে:

<Listing number="15-3" file-name="src/main.rs" caption="`1, 2, 3` তালিকা store করার জন্য `List` enum ব্যবহার করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

</Listing>

প্রথম `Cons` value-টিতে `1` এবং আরেকটি `List` value রয়েছে। এই `List` value-টি হল আরেকটি `Cons` value যাতে `2` এবং আরেকটি `List` value রয়েছে। এই `List` value-টি আরও একটি `Cons` value যাতে `3` এবং একটি `List` value রয়েছে, যেটি অবশেষে `Nil`, non-recursive variant যা list-এর শেষ নির্দেশ করে।

যদি আমরা Listing 15-3-এর কোড compile করার চেষ্টা করি, তাহলে আমরা Listing 15-4-এ দেখানো error টি পাব:

<Listing number="15-4" file-name="output.txt" caption="একটি recursive enum define করার চেষ্টা করার সময় আমরা যে error পাই">

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

</Listing>

Error টি দেখায় যে এই type-টির "অসীম আকার" রয়েছে। কারণ হল যে আমরা `List`-কে এমন একটি variant দিয়ে define করেছি যেটি recursive: এটি সরাসরি নিজের আরেকটি value ধারণ করে। ফলস্বরূপ, Rust বুঝতে পারে না যে একটি `List` value store করার জন্য তার কতটুকু জায়গা প্রয়োজন। আসুন ভেঙে দেখি কেন আমরা এই error টি পাই। প্রথমে, আমরা দেখব কিভাবে Rust decide করে যে এটি একটি non-recursive type-এর value store করার জন্য কতটুকু জায়গা প্রয়োজন।

#### একটি Non-Recursive Type-এর আকার গণনা করা

Chapter 6-এ enum definition নিয়ে আলোচনা করার সময় আমরা Listing 6-2-তে define করা `Message` enum-টি স্মরণ করি:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

একটি `Message` value-এর জন্য কতটা জায়গা allocate করতে হবে তা নির্ধারণ করতে, Rust প্রতিটি variant-এর মধ্যে দিয়ে যায় এটা দেখতে যে কোন variant-টির সবচেয়ে বেশি জায়গা প্রয়োজন। Rust দেখে যে `Message::Quit`-এর কোনো জায়গার প্রয়োজন নেই, `Message::Move`-এর দুটি `i32` value store করার জন্য যথেষ্ট জায়গা প্রয়োজন, ইত্যাদি। যেহেতু শুধুমাত্র একটি variant ব্যবহার করা হবে, তাই একটি `Message` value-এর জন্য সবচেয়ে বেশি যে জায়গার প্রয়োজন হবে তা হল এর largest variant store করার জন্য যে জায়গা লাগবে।

Listing 15-2-এর `List` enum-এর মতো recursive type-এর জন্য Rust কতটা জায়গা প্রয়োজন তা নির্ধারণ করার চেষ্টা করলে কী ঘটে তার সাথে এটি contrast করুন। Compiler `Cons` variant দেখে শুরু করে, যেটিতে type `i32`-এর একটি value এবং type `List`-এর একটি value রয়েছে। অতএব, `Cons`-এর একটি `i32`-এর আকারের সমান amount জায়গা এবং একটি `List`-এর আকারের প্রয়োজন। `List` type-টির জন্য কতটা মেমরির প্রয়োজন তা বের করতে, compiler variant গুলো দেখে, `Cons` variant দিয়ে শুরু করে। `Cons` variant-এ type `i32`-এর একটি value এবং type `List`-এর একটি value রয়েছে এবং এই প্রক্রিয়াটি অনির্দিষ্টকালের জন্য চলতে থাকে, যেমনটি Figure 15-1-এ দেখানো হয়েছে।

<img alt="একটি অসীম Cons তালিকা" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 15-1: অসীম `Cons` ভেরিয়েন্ট নিয়ে গঠিত একটি অসীম `List`</span>

#### একটি পরিচিত আকারের Recursive Type পেতে `Box<T>` ব্যবহার করা

যেহেতু Rust recursively define করা type-গুলোর জন্য কতটা জায়গা allocate করতে হবে তা বের করতে পারে না, তাই compiler এই সহায়ক পরামর্শ সহ একটি error দেয়:

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

এই পরামর্শে, "indirection"-এর অর্থ হল সরাসরি একটি value store করার পরিবর্তে, আমাদের data structure পরিবর্তন করে value-টিকে পরোক্ষভাবে store করা উচিত value-টির একটি pointer store করে।

যেহেতু একটি `Box<T>` হল একটি পয়েন্টার, তাই Rust সব সময় জানে একটি `Box<T>`-এর জন্য কতটা জায়গা প্রয়োজন: একটি পয়েন্টারের আকার এটি যে ডেটার দিকে point করছে তার পরিমাণের উপর ভিত্তি করে পরিবর্তিত হয় না। এর মানে হল আমরা সরাসরি অন্য একটি `List` value-এর পরিবর্তে `Cons` variant-এর ভিতরে একটি `Box<T>` রাখতে পারি। `Box<T>` পরবর্তী `List` value-টির দিকে point করবে যা `Cons` variant-এর ভিতরে থাকার পরিবর্তে heap-এ থাকবে। ধারণাগতভাবে, আমাদের এখনও একটি list রয়েছে, যা অন্যান্য list ধারণকারী list দিয়ে তৈরি, কিন্তু এই implementation টি এখন item গুলোকে একে অপরের ভিতরে রাখার পরিবর্তে একে অপরের পাশে রাখার মতো।

আমরা Listing 15-2-তে `List` enum-এর definition এবং Listing 15-3-এ `List`-এর usage পরিবর্তন করে Listing 15-5-এর কোডে পরিবর্তন করতে পারি, যেটি compile হবে:

<Listing number="15-5" file-name="src/main.rs" caption="একটি known আকার পাওয়ার জন্য `Box<T>` ব্যবহার করে `List`-এর Definition">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

</Listing>

`Cons` variant-টির একটি `i32`-এর আকার এবং box-এর পয়েন্টার ডেটা store করার জায়গার প্রয়োজন। `Nil` variant কোনো value store করে না, তাই এটির `Cons` variant-এর চেয়ে কম জায়গার প্রয়োজন। আমরা এখন জানি যে কোনো `List` value একটি `i32`-এর আকার এবং একটি box-এর পয়েন্টার ডেটার আকার নেবে। একটি box ব্যবহার করে, আমরা অসীম, recursive chain ভেঙে দিয়েছি, তাই compiler একটি `List` value store করার জন্য প্রয়োজনীয় আকার বের করতে পারে। Figure 15-2 দেখায় এখন `Cons` variant-টি কেমন দেখাচ্ছে।

<img alt="একটি সসীম Cons তালিকা" src="img/trpl15-02.svg" class="center" />

<span class="caption">Figure 15-2: একটি `List` যা অসীম আকারের নয় কারণ `Cons` একটি `Box` ধারণ করে</span>

Box গুলো শুধুমাত্র indirection এবং heap allocation provide করে; সেগুলোর অন্য কোনো special capabilities নেই, যেমনটি আমরা অন্য স্মার্ট পয়েন্টার টাইপগুলোর সাথে দেখব। সেগুলোর এই special capability গুলোর কারণে হওয়া পারফরম্যান্স ওভারহেডও নেই, তাই cons list-এর মতো ক্ষেত্রগুলোতে সেগুলো useful হতে পারে যেখানে indirection হল একমাত্র feature যা আমাদের প্রয়োজন। আমরা Chapter 18-এ box-গুলোর আরও use case দেখব।

`Box<T>` type টি একটি স্মার্ট পয়েন্টার কারণ এটি `Deref` trait implement করে, যা `Box<T>` value গুলোকে reference-এর মতো treat করার অনুমতি দেয়। যখন একটি `Box<T>` value scope-এর বাইরে চলে যায়, তখন box যে heap ডেটার দিকে point করছে সেটিও clean up করা হয় `Drop` trait implementation-এর কারণে। এই দুটি trait আমরা এই chapter-এর বাকি অংশে আলোচনা করব এমন অন্যান্য স্মার্ট পয়েন্টার টাইপগুলোর দ্বারা provide করা functionality-এর জন্য আরও গুরুত্বপূর্ণ হবে। আসুন এই দুটি trait আরও বিশদভাবে explore করি।

[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
