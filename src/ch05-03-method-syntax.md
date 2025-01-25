## Method সিনট্যাক্স

_Methods_ ফাংশনের মতোই: আমরা `fn` কীওয়ার্ড এবং একটি নাম দিয়ে সেগুলি ঘোষণা করি, তাদের প্যারামিটার এবং একটি রিটার্ন ভ্যালু থাকতে পারে এবং সেগুলিতে কিছু কোড থাকে যা অন্য কোথাও থেকে পদ্ধতিটি কল করা হলে রান হয়। ফাংশনগুলির বিপরীতে, methods একটি struct এর (অথবা একটি enum বা একটি trait object, যা আমরা [Chapter 6][enums]<!-- ignore --> এবং [Chapter 17][trait-objects]<!-- ignore --> এ আলোচনা করব) প্রেক্ষাপটের মধ্যে সংজ্ঞায়িত করা হয় এবং তাদের প্রথম প্যারামিটার সর্বদা `self` হয়, যা struct এর সেই instance কে প্রতিনিধিত্ব করে যার উপর পদ্ধতিটি কল করা হচ্ছে।

### Methods সংজ্ঞায়িত করা

আসুন `area` ফাংশনটি পরিবর্তন করি, যেখানে একটি প্যারামিটার হিসাবে `Rectangle` instance থাকে এবং পরিবর্তে `Rectangle` struct এ সংজ্ঞায়িত একটি `area` method তৈরি করি, যেমন Listing 5-13 এ দেখানো হয়েছে।

<Listing number="5-13" file-name="src/main.rs" caption="`Rectangle` struct এ একটি `area` method সংজ্ঞায়িত করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

`Rectangle` এর প্রেক্ষাপটের মধ্যে ফাংশনটি সংজ্ঞায়িত করতে, আমরা `Rectangle` এর জন্য একটি `impl` (implementation) ব্লক শুরু করি। এই `impl` ব্লকের মধ্যে সবকিছু `Rectangle` টাইপের সাথে যুক্ত হবে। তারপর আমরা `impl` কার্লি ব্র্যাকেটের মধ্যে `area` ফাংশনটিকে move করি এবং সিগনেচারে এবং বডির ভিতরে সর্বত্র প্রথম (এবং এই ক্ষেত্রে, একমাত্র) প্যারামিটারটিকে `self` এ পরিবর্তন করি। `main`-এ, যেখানে আমরা `area` ফাংশন কল করেছি এবং `rect1` কে আর্গুমেন্ট হিসাবে পাস করেছি, সেখানে আমরা পরিবর্তে আমাদের `Rectangle` instance এ `area` method কল করতে _method syntax_ ব্যবহার করতে পারি। Method syntax একটি instance এর পরে যায়: আমরা একটি ডট যোগ করি এবং তারপরে method এর নাম, বন্ধনী এবং যেকোনো আর্গুমেন্ট দিই।

`area` এর সিগনেচারে, আমরা `rectangle: &Rectangle` এর পরিবর্তে `&self` ব্যবহার করি। `&self` আসলে `self: &Self` এর সংক্ষিপ্ত রূপ। একটি `impl` ব্লকের মধ্যে, `Self` টাইপটি সেই টাইপের জন্য একটি alias যার জন্য `impl` ব্লকটি তৈরি করা হয়েছে। Methods এর প্রথম প্যারামিটারের জন্য `Self` টাইপের `self` নামের একটি প্যারামিটার থাকতে হবে, তাই Rust আপনাকে প্রথম প্যারামিটারের স্থানে শুধুমাত্র `self` নামটি দিয়ে এটিকে সংক্ষেপ করতে দেয়। মনে রাখবেন যে এই method টি `Self` instance কে borrow করে, তা নির্দেশ করার জন্য আমাদের এখনও `self` শর্টহ্যান্ডের সামনে `&` ব্যবহার করতে হবে, ঠিক যেমন আমরা `rectangle: &Rectangle` এ করেছিলাম। Methods `self` এর ownership নিতে পারে, `self` কে immutable ভাবে borrow করতে পারে, যেমনটা আমরা এখানে করেছি, অথবা `self` কে mutable ভাবে borrow করতে পারে, ঠিক যেমন তারা অন্য কোনো প্যারামিটার করতে পারে।

ফাংশন সংস্করণে আমরা `&Rectangle` ব্যবহার করার একই কারণে আমরা এখানে `&self` বেছে নিয়েছি: আমরা ownership নিতে চাই না এবং আমরা শুধু struct এ ডেটা পড়তে চাই, এতে লিখতে চাই না। যদি আমরা method এর অংশ হিসাবে method টি কল করা instance পরিবর্তন করতে চাই, তবে আমরা প্রথম প্যারামিটার হিসাবে `&mut self` ব্যবহার করব। প্রথম প্যারামিটার হিসাবে শুধুমাত্র `self` ব্যবহার করে instance এর ownership নেওয়া একটি method থাকা বিরল; এই কৌশলটি সাধারণত ব্যবহার করা হয় যখন method টি `self` কে অন্য কিছুতে রূপান্তরিত করে এবং আপনি রূপান্তরের পরে কলারকে মূল instance ব্যবহার করা থেকে আটকাতে চান।

ফাংশনের পরিবর্তে methods ব্যবহার করার মূল কারণ, method syntax প্রদান করা এবং প্রতিটি method এর সিগনেচারে `self` এর টাইপ পুনরাবৃত্তি করতে না হওয়ার পাশাপাশি, সংগঠনের জন্য। আমরা একটি টাইপের instance দিয়ে করতে পারি এমন সমস্ত কিছু একটি `impl` ব্লকে রেখেছি, পরিবর্তে আমাদের কোডের ভবিষ্যতের ব্যবহারকারীদের লাইব্রেরিতে `Rectangle` এর ক্ষমতা বিভিন্ন জায়গায় খুঁজে বের করতে বাধ্য করার চেয়ে।

মনে রাখবেন যে আমরা একটি method কে struct এর ফিল্ডগুলির একটির মতোই নাম দিতে পছন্দ করতে পারি। উদাহরণস্বরূপ, আমরা `Rectangle` এ একটি method সংজ্ঞায়িত করতে পারি যার নামও `width`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

এখানে, আমরা `width` method টিকে এমনভাবে তৈরি করছি যাতে instance এর `width` ফিল্ডের মান `0` এর চেয়ে বেশি হলে `true` এবং মান `0` হলে `false` ফেরত দেয়: আমরা যেকোনো উদ্দেশ্যে একই নামের একটি method এর মধ্যে একটি ফিল্ড ব্যবহার করতে পারি। `main`-এ, যখন আমরা `rect1.width` এর পরে বন্ধনী ব্যবহার করি, তখন Rust জানে যে আমাদের `width` method বোঝানো হয়েছে। যখন আমরা বন্ধনী ব্যবহার করি না, তখন Rust জানে যে আমাদের `width` ফিল্ড বোঝানো হয়েছে।

প্রায়শই, কিন্তু সবসময় নয়, যখন আমরা একটি method কে একটি ফিল্ডের মতোই নাম দিই, তখন আমরা চাই যে এটি শুধুমাত্র ফিল্ডের মান ফেরত দিক এবং অন্য কিছু না করুক। এই ধরনের method গুলিকে _getters_ বলা হয় এবং Rust struct ফিল্ডগুলির জন্য স্বয়ংক্রিয়ভাবে সেগুলি প্রয়োগ করে না যেমন কিছু অন্যান্য ভাষা করে। Getters কার্যকর কারণ আপনি ফিল্ডটিকে private করতে পারেন কিন্তু method টিকে public করতে পারেন এবং এইভাবে টাইপের public API এর অংশ হিসাবে সেই ফিল্ডে শুধুমাত্র-পঠনযোগ্য অ্যাক্সেস সক্ষম করতে পারেন। public এবং private কী এবং কিভাবে একটি ফিল্ড বা method কে public বা private হিসাবে মনোনীত করতে হয় তা আমরা [Chapter 7][public]<!-- ignore --> এ আলোচনা করব।

> ### `->` অপারেটর কোথায়?
>
> C এবং C++ এ, methods কল করার জন্য দুটি ভিন্ন অপারেটর ব্যবহার করা হয়: আপনি `.` ব্যবহার করেন যদি আপনি সরাসরি অবজেক্টে একটি method কল করেন এবং `->` ব্যবহার করেন যদি আপনি অবজেক্টের pointer এ method কল করেন এবং প্রথমে pointer টিকে dereference করতে হয়। অন্য কথায়, যদি `object` একটি pointer হয়, তবে `object->something()` হল `(*object).something()` এর অনুরূপ।
>
> Rust এ `->` অপারেটরের সমতুল্য কিছু নেই; পরিবর্তে, Rust এ _automatic referencing and dereferencing_ নামক একটি বৈশিষ্ট্য রয়েছে। Methods কল করা Rust এর কয়েকটি জায়গার মধ্যে একটি যেখানে এই আচরণ রয়েছে।
>
> এখানে এটি কিভাবে কাজ করে: যখন আপনি `object.something()` দিয়ে একটি method কল করেন, তখন Rust স্বয়ংক্রিয়ভাবে `&`, `&mut`, বা `*` যোগ করে যাতে `object` method এর সিগনেচারের সাথে মিলে যায়। অন্য কথায়, নিম্নলিখিতগুলি একই:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> প্রথমটি দেখতে অনেক পরিষ্কার। এই স্বয়ংক্রিয় referencing আচরণ কাজ করে কারণ methods এর একটি স্পষ্ট রিসিভার আছে—`self` এর প্রকার। একটি method এর রিসিভার এবং নাম দেওয়া হলে, Rust নিশ্চিতভাবে জানতে পারে যে method টি পড়ছে (`&self`), পরিবর্তন করছে (`&mut self`), অথবা ব্যবহার করছে (`self`)। Rust method রিসিভারদের জন্য borrowing কে অন্তর্নিহিত করে তোলে, এই বিষয়টি বাস্তবে ownership কে ergonomic করার একটি বড় অংশ।

### একাধিক প্যারামিটার সহ Methods

আসুন `Rectangle` struct এ দ্বিতীয় একটি method প্রয়োগ করে methods ব্যবহার করার অনুশীলন করি। এবার আমরা চাই `Rectangle` এর একটি instance অন্য একটি `Rectangle` এর instance নিক এবং `true` ফেরত দিক যদি দ্বিতীয় `Rectangle` টি সম্পূর্ণভাবে `self` (প্রথম `Rectangle`) এর মধ্যে ফিট হতে পারে; অন্যথায়, এটি `false` ফেরত দেবে। অর্থাৎ, একবার আমরা `can_hold` method সংজ্ঞায়িত করার পরে, আমরা Listing 5-14 এ দেখানো প্রোগ্রামটি লিখতে সক্ষম হতে চাই।

<Listing number="5-14" file-name="src/main.rs" caption="এখনও লেখা হয়নি এমন `can_hold` method ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

প্রত্যাশিত আউটপুটটি দেখতে এইরকম হবে কারণ `rect2` এর উভয় ডাইমেনশন `rect1` এর ডাইমেনশনগুলির চেয়ে ছোট, কিন্তু `rect3` হল `rect1` এর চেয়ে চওড়া:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

আমরা জানি যে আমরা একটি method সংজ্ঞায়িত করতে চাই, তাই এটি `impl Rectangle` ব্লকের মধ্যে থাকবে। Method এর নাম হবে `can_hold` এবং এটি প্যারামিটার হিসাবে অন্য `Rectangle` এর একটি immutable borrow নেবে। আমরা method কল করা কোডটি দেখে প্যারামিটারের প্রকার কী হবে তা বলতে পারি: `rect1.can_hold(&rect2)` `&rect2` পাস করে, যা `rect2` এর একটি immutable borrow, `Rectangle` এর একটি instance। এটি অর্থপূর্ণ কারণ আমাদের `rect2` কে শুধুমাত্র পড়তে হবে (লেখার পরিবর্তে, যার মানে আমাদের একটি mutable borrow প্রয়োজন হবে) এবং আমরা চাই `main` `rect2` এর ownership ধরে রাখুক যাতে আমরা `can_hold` method কল করার পরে এটিকে আবার ব্যবহার করতে পারি। `can_hold` এর রিটার্ন ভ্যালু একটি বুলিয়ান হবে এবং বাস্তবায়নটি পরীক্ষা করবে যে `self` এর প্রস্থ এবং উচ্চতা অন্য `Rectangle` এর প্রস্থ এবং উচ্চতার চেয়ে বেশি কিনা। আসুন Listing 5-13 থেকে `impl` ব্লকে নতুন `can_hold` method যোগ করি, যা Listing 5-15 এ দেখানো হয়েছে।

<Listing number="5-15" file-name="src/main.rs" caption="`Rectangle` এ `can_hold` method প্রয়োগ করা যা প্যারামিটার হিসাবে অন্য `Rectangle` instance নেয়">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

যখন আমরা Listing 5-14 এর `main` ফাংশন দিয়ে এই কোডটি চালাব, তখন আমরা আমাদের কাঙ্ক্ষিত আউটপুট পাব। Methods একাধিক প্যারামিটার নিতে পারে যা আমরা `self` প্যারামিটারের পরে সিগনেচারে যোগ করি এবং সেই প্যারামিটারগুলি ফাংশনের প্যারামিটারগুলির মতোই কাজ করে।

### Associated Functions

একটি `impl` ব্লকের মধ্যে সংজ্ঞায়িত সমস্ত ফাংশনকে _associated functions_ বলা হয়, কারণ সেগুলি `impl` এর পরে নামের টাইপের সাথে যুক্ত। আমরা associated functions সংজ্ঞায়িত করতে পারি যার প্রথম প্যারামিটার হিসাবে `self` নেই (এবং তাই সেগুলি methods নয়) কারণ তাদের কাজ করার জন্য টাইপের instance এর প্রয়োজন হয় না। আমরা ইতিমধ্যেই এইরকম একটি ফাংশন ব্যবহার করেছি: `String` টাইপে সংজ্ঞায়িত `String::from` ফাংশন।

যে associated functions গুলো methods নয়, সেগুলি প্রায়শই কনস্ট্রাক্টরদের জন্য ব্যবহৃত হয় যা struct এর একটি নতুন instance ফেরত দেবে। এগুলিকে প্রায়ই `new` বলা হয়, তবে `new` কোনো বিশেষ নাম নয় এবং ভাষার মধ্যে তৈরি করা হয়নি। উদাহরণস্বরূপ, আমরা একটি associated function প্রদান করতে পছন্দ করতে পারি যার নাম `square`, যেখানে একটি ডাইমেনশন প্যারামিটার থাকবে এবং সেটিকেই প্রস্থ এবং উচ্চতা উভয় হিসাবে ব্যবহার করা হবে, এইভাবে একই মান দুবার উল্লেখ করার পরিবর্তে একটি বর্গক্ষেত্র `Rectangle` তৈরি করা সহজ হবে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

রিটার্ন টাইপে এবং ফাংশনের বডিতে থাকা `Self` কীওয়ার্ডগুলি সেই টাইপের alias যা `impl` কীওয়ার্ডের পরে আসে, এই ক্ষেত্রে যা হল `Rectangle`।

এই associated function টি কল করতে, আমরা struct নামের সাথে `::` সিনট্যাক্স ব্যবহার করি; `let sq = Rectangle::square(3);` একটি উদাহরণ। এই ফাংশনটি struct দ্বারা namespaced: `::` সিনট্যাক্স associated functions এবং মডিউল দ্বারা তৈরি namespaces উভয়ের জন্যই ব্যবহৃত হয়। আমরা [Chapter 7][modules]<!-- ignore --> এ মডিউল নিয়ে আলোচনা করব।

### একাধিক `impl` ব্লক

প্রতিটি struct এর একাধিক `impl` ব্লক থাকতে পারে। উদাহরণস্বরূপ, Listing 5-15, Listing 5-16 এ দেখানো কোডের সমতুল্য, যেখানে প্রতিটি method এর নিজস্ব `impl` ব্লক রয়েছে।

<Listing number="5-16" caption="একাধিক `impl` ব্লক ব্যবহার করে Listing 5-15 পুনরায় লেখা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

এখানে এই methods গুলোকে একাধিক `impl` ব্লকে আলাদা করার কোনো কারণ নেই, তবে এটি বৈধ সিনট্যাক্স। আমরা Chapter 10 এ একটি ক্ষেত্র দেখব যেখানে একাধিক `impl` ব্লক কার্যকর, যেখানে আমরা জেনেরিক টাইপ এবং traits নিয়ে আলোচনা করব।

## সারসংক্ষেপ

Structs আপনাকে কাস্টম টাইপ তৈরি করতে দেয় যা আপনার ডোমেনের জন্য অর্থপূর্ণ। structs ব্যবহার করে, আপনি একে অপরের সাথে যুক্ত ডেটার অংশগুলিকে সংযুক্ত রাখতে পারেন এবং আপনার কোডটিকে পরিষ্কার করতে প্রতিটি অংশের নাম দিতে পারেন। `impl` ব্লকে, আপনি এমন ফাংশন সংজ্ঞায়িত করতে পারেন যা আপনার টাইপের সাথে যুক্ত, এবং methods হল এক ধরনের associated function যা আপনাকে আপনার structs এর instance গুলির আচরণ নির্দিষ্ট করতে দেয়।

কিন্তু structs কাস্টম টাইপ তৈরি করার একমাত্র উপায় নয়: আসুন Rust এর enum বৈশিষ্ট্যের দিকে যাই যাতে আপনার টুলবক্সে আরও একটি টুল যোগ করা যায়।

[enums]: ch06-00-enums.html
[trait-objects]: ch18-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
