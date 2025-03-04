## `Drop` Trait-এর সাহায্যে Cleanup-এর সময় কোড চালানো

স্মার্ট পয়েন্টার প্যাটার্নের জন্য গুরুত্বপূর্ণ দ্বিতীয় trait টি হল `Drop`, যা আপনাকে কাস্টমাইজ করতে দেয় যখন একটি value scope-এর বাইরে চলে যেতে চলেছে তখন কী ঘটবে। আপনি যেকোনো টাইপের জন্য `Drop` trait-এর একটি implementation provide করতে পারেন এবং সেই কোডটি ফাইল বা নেটওয়ার্ক কানেকশনের মতো রিসোর্স release করতে ব্যবহার করা যেতে পারে।

আমরা স্মার্ট পয়েন্টারগুলোর context-এ `Drop` introduce করছি কারণ `Drop` trait-এর functionality প্রায় সব সময় একটি স্মার্ট পয়েন্টার implement করার সময় ব্যবহার করা হয়। উদাহরণস্বরূপ, যখন একটি `Box<T>` ড্রপ করা হয়, তখন এটি heap-এর সেই space-টি deallocate করবে যেখানে box টি point করছে।

কিছু language-এ, কিছু type-এর জন্য, প্রোগ্রামারকে প্রতিবার সেই type-গুলোর একটি instance ব্যবহার করা শেষ হলে মেমরি বা রিসোর্স free করার জন্য কোড কল করতে হয়। উদাহরণের মধ্যে রয়েছে ফাইল হ্যান্ডেল, সকেট বা লক। যদি তারা ভুলে যায়, তাহলে সিস্টেম ওভারলোড হয়ে যেতে পারে এবং ক্র্যাশ করতে পারে। Rust-এ, আপনি specify করতে পারেন যে একটি value scope-এর বাইরে চলে গেলে একটি particular code-এর অংশ run হবে এবং compiler স্বয়ংক্রিয়ভাবে এই কোডটি insert করবে। ফলস্বরূপ, আপনাকে একটি প্রোগ্রামের সর্বত্র cleanup কোড রাখার বিষয়ে সতর্ক থাকতে হবে না যেখানে একটি particular type-এর instance-এর কাজ শেষ হয়েছে—তবুও আপনি রিসোর্স লিক করবেন না!

আপনি `Drop` trait implement করে একটি value scope-এর বাইরে চলে গেলে যে কোডটি run হবে তা specify করেন। `Drop` trait-এর জন্য আপনাকে `drop` নামক একটি method implement করতে হবে যা `self`-এর একটি mutable reference নেয়। Rust কখন `drop` কল করে তা দেখতে, আসুন আপাতত `println!` স্টেটমেন্ট দিয়ে `drop` implement করি।

Listing 15-14 একটি `CustomSmartPointer` struct দেখায় যার একমাত্র কাস্টম functionality হল যে instance টি scope-এর বাইরে চলে গেলে এটি `Dropping CustomSmartPointer!` প্রিন্ট করবে, এটা দেখানোর জন্য যে Rust কখন `drop` ফাংশনটি চালায়।

<Listing number="15-14" file-name="src/main.rs" caption="একটি `CustomSmartPointer` স্ট্রাক্ট যা `Drop` trait implement করে যেখানে আমরা আমাদের cleanup কোড রাখব">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

</Listing>

`Drop` trait টি prelude-এ অন্তর্ভুক্ত, তাই আমাদের এটিকে scope-এ আনার প্রয়োজন নেই। আমরা `CustomSmartPointer`-এ `Drop` trait implement করি এবং `drop` method-এর জন্য একটি implementation provide করি যা `println!` কল করে। `Drop` ফাংশনের body হল সেই জায়গা যেখানে আপনি আপনার type-এর একটি instance scope-এর বাইরে চলে গেলে আপনি যে লজিকটি চালাতে চান সেটি রাখবেন। Rust কখন `drop` কল করবে তা visually প্রদর্শন করার জন্য আমরা এখানে কিছু text প্রিন্ট করছি।

`Main`-এ, আমরা `CustomSmartPointer`-এর দুটি instance তৈরি করি এবং তারপর `CustomSmartPointers created` প্রিন্ট করি। `Main`-এর শেষে, `CustomSmartPointer`-এর আমাদের instance গুলো scope-এর বাইরে চলে যাবে এবং Rust আমাদের `drop` method-এ রাখা কোডটিকে কল করবে, আমাদের final message প্রিন্ট করবে। মনে রাখবেন যে আমাদের explicit ভাবে `drop` method কল করার প্রয়োজন ছিল না।

যখন আমরা এই প্রোগ্রামটি চালাই, তখন আমরা নিম্নলিখিত আউটপুট দেখতে পাব:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

Rust স্বয়ংক্রিয়ভাবে আমাদের instance গুলো scope-এর বাইরে চলে গেলে আমাদের জন্য `drop` কল করেছে, আমরা যে কোড specify করেছি সেটি কল করে। Variable গুলো তাদের তৈরির বিপরীত ক্রমে ড্রপ করা হয়, তাই `d`-কে `c`-এর আগে ড্রপ করা হয়েছিল। এই উদাহরণের উদ্দেশ্য হল আপনাকে `drop` method কীভাবে কাজ করে তার একটি visual guide দেওয়া; সাধারণত আপনি একটি print message-এর পরিবর্তে আপনার type-এর যে cleanup কোড চালানো দরকার তা specify করবেন।

### `std::mem::drop` দিয়ে একটি Value-কে তাড়াতাড়ি Drop করা

দুর্ভাগ্যবশত, স্বয়ংক্রিয় `drop` functionality disable করা straightforward নয়। `Drop` disable করা সাধারণত প্রয়োজন হয় না; `Drop` trait-এর মূল বিষয় হল এটি স্বয়ংক্রিয়ভাবে handle করা হয়। যাইহোক, মাঝে মাঝে, আপনি একটি value তাড়াতাড়ি clean up করতে চাইতে পারেন। একটি উদাহরণ হল যখন স্মার্ট পয়েন্টার ব্যবহার করা হয় যা লক manage করে: আপনি হয়তো `drop` method-কে force করতে চাইতে পারেন যা লক release করে যাতে একই scope-এর অন্যান্য কোড লকটি acquire করতে পারে। Rust আপনাকে `Drop` trait-এর `drop` method ম্যানুয়ালি কল করতে দেয় না; পরিবর্তে আপনাকে standard library দ্বারা provide করা `std::mem::drop` ফাংশনটি কল করতে হবে যদি আপনি একটি value-কে তার scope-এর শেষের আগে ড্রপ করতে বাধ্য করতে চান।

যদি আমরা Listing 15-14 থেকে `main` ফাংশনটিকে modify করে `Drop` trait-এর `drop` method ম্যানুয়ালি কল করার চেষ্টা করি, যেমনটি Listing 15-15-এ দেখানো হয়েছে, তাহলে আমরা একটি compiler error পাব:

<Listing number="15-15" file-name="src/main.rs" caption="`Drop` trait থেকে `drop` মেথড ম্যানুয়ালি কল করে তাড়াতাড়ি clean up করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

</Listing>

যখন আমরা এই কোডটি compile করার চেষ্টা করি, তখন আমরা এই error টি পাব:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

এই error message টি বলে যে আমাদের explicit ভাবে `drop` কল করার অনুমতি নেই। Error message টি _destructor_ শব্দটি ব্যবহার করে, যেটি একটি instance clean up করে এমন একটি ফাংশনের general programming term। একটি _destructor_ একটি _constructor_-এর অনুরূপ, যা একটি instance তৈরি করে। Rust-এর `drop` ফাংশনটি হল একটি particular destructor।

Rust আমাদের explicit ভাবে `drop` কল করতে দেয় না কারণ Rust এখনও স্বয়ংক্রিয়ভাবে `main`-এর শেষে value-টিতে `drop` কল করবে। এটি একটি _double free_ error-এর কারণ হবে কারণ Rust একই value দুবার clean up করার চেষ্টা করবে।

আমরা যখন একটি value scope-এর বাইরে চলে যায় তখন `drop`-এর স্বয়ংক্রিয় insertion disable করতে পারি না এবং আমরা explicit ভাবে `drop` method কল করতে পারি না। সুতরাং, যদি আমাদের একটি value-কে তাড়াতাড়ি clean up করতে বাধ্য করতে হয়, তাহলে আমরা `std::mem::drop` ফাংশনটি ব্যবহার করি।

`Std::mem::drop` ফাংশনটি `Drop` trait-এর `drop` method থেকে আলাদা। আমরা এটিকে argument হিসেবে যে value-টিকে force drop করতে চাই সেটি pass করে কল করি। ফাংশনটি prelude-এ রয়েছে, তাই আমরা Listing 15-15-এর `main`-কে modify করে `drop` ফাংশনটিকে কল করতে পারি, যেমনটি Listing 15-16-এ দেখানো হয়েছে:

<Listing number="15-16" file-name="src/main.rs" caption="Scope-এর বাইরে যাওয়ার আগে একটি value-কে explicit ভাবে drop করতে `std::mem::drop` কল করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

</Listing>

এই কোডটি run করলে নিম্নলিখিতগুলো প্রিন্ট হবে:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

``Dropping CustomSmartPointer with data `some data`!`` text টি `CustomSmartPointer created.` এবং `CustomSmartPointer dropped before the end of main.` text-এর মধ্যে প্রিন্ট করা হয়েছে, এটি দেখায় যে `drop` method code-টি সেই সময়ে `c`-কে drop করার জন্য কল করা হয়েছে।

আপনি cleanup-কে সুবিধাজনক এবং নিরাপদ করতে `Drop` trait implementation-এ specify করা কোডটি বিভিন্ন উপায়ে ব্যবহার করতে পারেন: উদাহরণস্বরূপ, আপনি এটি ব্যবহার করে আপনার নিজের মেমরি অ্যালোকেটর তৈরি করতে পারেন! `Drop` trait এবং Rust-এর ownership system-এর সাহায্যে, আপনাকে clean up করার কথা মনে রাখতে হবে না কারণ Rust এটি স্বয়ংক্রিয়ভাবে করে।

ভুলবশত এখনও ব্যবহৃত value গুলো clean up করার ফলে ഉണ്ടാ হওয়া সমস্যাগুলো নিয়ে আপনাকে চিন্তা করতে হবে না: ownership system যা নিশ্চিত করে যে reference গুলো সব সময় valid, তাও নিশ্চিত করে যে `drop` শুধুমাত্র একবার কল করা হবে যখন value টি আর ব্যবহার করা হবে না।

এখন যেহেতু আমরা `Box<T>` এবং স্মার্ট পয়েন্টারগুলোর কিছু বৈশিষ্ট্য পরীক্ষা করেছি, আসুন standard library-তে define করা আরও কয়েকটি স্মার্ট পয়েন্টার দেখি।
