## Unsafe Rust

এতক্ষণ পর্যন্ত আমরা যেসব code নিয়ে আলোচনা করেছি, সেগুলোর সব code compile time এ Rust এর memory safety guarantee enforce করেছে। তবে, Rust এর ভিতরে hidden একটি দ্বিতীয় language আছে যা এই memory safety guarantee enforce করে না: একে _unsafe Rust_ বলা হয় এবং এটি regular Rust এর মতোই কাজ করে, কিন্তু আমাদের extra superpower দেয়।

Unsafe Rust exist করে কারণ, by nature, static analysis conservative হয়। যখন compiler code guarantee uphold করে কিনা তা determine করার চেষ্টা করে, তখন কিছু invalid program accept করার চেয়ে কিছু valid program reject করা ভালো। যদিও code _might_ be okay, যদি Rust compiler এর confident হওয়ার জন্য যথেষ্ট information না থাকে, তাহলে এটি code reject করবে। এই case গুলোতে, compiler কে বলার জন্য আপনি unsafe code ব্যবহার করতে পারেন, "Trust me, I know what I’m doing"। তবে warned থাকুন যে আপনি নিজের risk এ unsafe Rust ব্যবহার করছেন: আপনি যদি unsafe code incorrectly ব্যবহার করেন, তাহলে memory unsafety এর কারণে problem হতে পারে, যেমন null pointer dereferencing।

Rust এর একটি unsafe alter ego থাকার অন্য কারণ হলো underlying computer hardware inherently unsafe। যদি Rust unsafe operation করার allow না করত, তাহলে আপনি কিছু task করতে পারতেন না। Rust এর low-level system programming, যেমন operating system এর সাথে directly interact করা বা এমনকি নিজের operating system লেখার মতো কাজ করার allow করার প্রয়োজন। Low-level system programming নিয়ে কাজ করা language এর goal গুলোর মধ্যে একটি। চলুন explore করি unsafe Rust দিয়ে আমরা কি করতে পারি এবং কিভাবে করতে পারি।

### Unsafe Superpowers

Unsafe Rust এ switch করার জন্য, `unsafe` keyword ব্যবহার করুন এবং তারপর একটি নতুন block শুরু করুন যা unsafe code hold করে। আপনি unsafe Rust এ পাঁচটি action নিতে পারেন যা আপনি safe Rust এ পারেন না, যাকে আমরা _unsafe superpowers_ বলি। সেই superpower গুলোতে include আছে:

- Raw pointer dereference করার ability
- Unsafe function বা method call করা
- Mutable static variable access বা modify করা
- Unsafe trait implement করা
- একটি `union` এর field access করা

এটা বোঝা গুরুত্বপূর্ণ যে `unsafe` borrow checker turn off করে না বা Rust এর অন্য কোনো safety check disable করে না: যদি আপনি unsafe code এ reference ব্যবহার করেন, তবুও সেটি check করা হবে। `unsafe` keyword শুধুমাত্র এই পাঁচটি feature এ access দেয় যা compiler memory safety এর জন্য check করে না। আপনি unsafe block এর ভিতরে still কিছু safety পাবেন।

এছাড়াও, `unsafe` মানে এই নয় যে block এর ভিতরের code necessarily dangerous বা সেখানে অবশ্যই memory safety problem থাকবে: intent হলো programmer হিসেবে, আপনি নিশ্চিত করবেন যে `unsafe` block এর ভিতরের code valid উপায়ে memory access করবে।

মানুষ ভুল করে, এবং ভুল হবেই, কিন্তু `unsafe` annotation করা block এর ভিতরে এই পাঁচটি unsafe operation require করার মাধ্যমে আপনি জানতে পারবেন যে memory safety এর সাথে related যেকোনো error অবশ্যই `unsafe` block এর ভিতরে থাকবে। `unsafe` block ছোট রাখুন; memory bug investigate করার সময় পরে আপনি কৃতজ্ঞ হবেন।

Unsafe code যত বেশি possible isolate করার জন্য, safe abstraction এর ভিতরে unsafe code enclose করা এবং একটি safe API provide করা best, যা আমরা chapter এর পরে unsafe function এবং method examine করার সময় discuss করব। Standard library এর কিছু অংশ unsafe code এর উপর safe abstraction হিসেবে implement করা হয়েছে যা audit করা হয়েছে। Safe abstraction এ unsafe code wrap করা `unsafe` এর ব্যবহারকে সব জায়গায় leak হওয়া থেকে prevent করে যেখানে আপনি বা আপনার user unsafe code দিয়ে implement করা functionality ব্যবহার করতে চাইতে পারেন, কারণ safe abstraction ব্যবহার করা safe।

চলুন একে একে পাঁচটি unsafe superpower দেখি। আমরা unsafe code এর safe interface provide করে এমন কিছু abstraction ও দেখব।

### Dereferencing a Raw Pointer

Chapter 4 এর [“Dangling References”][dangling-references]<!-- ignore --> section এ, আমরা mention করেছিলাম compiler নিশ্চিত করে যে reference গুলো সবসময় valid। Unsafe Rust এ _raw pointer_ নামে দুটি নতুন type আছে যা reference এর similar। Reference এর মতো, raw pointer immutable বা mutable হতে পারে এবং `*const T` এবং `*mut T` হিসেবে লেখা হয়। Asterisk dereference operator নয়; এটা type name এর অংশ। Raw pointer এর context এ, _immutable_ মানে হলো pointer dereference করার পর directly assign করা যাবে না।

Reference এবং smart pointer থেকে different, raw pointer:

- Borrowing rule ignore করার allow করে একই location এ immutable এবং mutable pointer বা multiple mutable pointer থাকার মাধ্যমে
- Valid memory point করার guarantee দেয় না
- Null হওয়ার allow করে
- Automatic cleanup implement করে না

Rust এর এই guarantee enforce করা থেকে opt out করার মাধ্যমে, আপনি greater performance বা অন্য কোনো language বা hardware এর সাথে interface করার ability এর বিনিময়ে guaranteed safety ছেড়ে দিতে পারেন যেখানে Rust এর guarantee apply হয় না।

Listing 20-1 দেখায় কিভাবে একটি immutable এবং একটি mutable raw pointer তৈরি করতে হয়।

<Listing number="20-1" caption="Raw borrow operator দিয়ে raw pointer তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

Notice করুন যে আমরা এই code এ `unsafe` keyword include করিনি। আমরা safe code এ raw pointer তৈরি করতে পারি; আমরা শুধুমাত্র unsafe block এর বাইরে raw pointer dereference করতে পারি না, যা আপনি একটু পরেই দেখবেন।

আমরা raw borrow operator ব্যবহার করে raw pointer তৈরি করেছি: `&raw const num` একটি `*const i32` immutable raw pointer তৈরি করে, এবং `&raw mut num` একটি `*mut i32` mutable raw pointer তৈরি করে। যেহেতু আমরা সেগুলোকে directly local variable থেকে তৈরি করেছি, তাই আমরা জানি যে এই particular raw pointer গুলো valid, কিন্তু আমরা যেকোনো raw pointer নিয়ে এই assumption করতে পারি না।

এটা demonstrate করার জন্য, এরপর আমরা এমন একটি raw pointer তৈরি করব যার validity নিয়ে আমরা নিশ্চিত হতে পারি না, raw reference operator ব্যবহার করার পরিবর্তে একটি value cast করার জন্য `as` ব্যবহার করে। Listing 20-2 দেখায় কিভাবে memory তে arbitrary location এ একটি raw pointer তৈরি করতে হয়। Arbitrary memory ব্যবহার করার চেষ্টা undefined: সেই address এ data থাকতে পারে নাও থাকতে পারে, compiler code optimize করতে পারে তাই কোনো memory access নাও থাকতে পারে, অথবা program segmentation fault এর সাথে error করতে পারে। সাধারণত, এইরকম code লেখার কোনো ভালো reason নেই, বিশেষ করে যেখানে আপনি এর পরিবর্তে raw borrow operator ব্যবহার করতে পারেন, কিন্তু এটা possible।

<Listing number="20-2" caption="Arbitrary memory address এ একটি raw pointer তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

মনে করুন যে আমরা safe code এ raw pointer তৈরি করতে পারি, কিন্তু আমরা raw pointer _dereference_ করতে পারি না এবং point করা data read করতে পারি না। Listing 20-3 এ, আমরা `*` dereference operator ব্যবহার করি raw pointer এর উপর যার জন্য একটি `unsafe` block require করে।

<Listing number="20-3" caption="একটি `unsafe` block এর ভিতরে raw pointer dereference করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

</Listing>

Pointer তৈরি করা কোনো harm করে না; যখন আমরা সেই value access করার চেষ্টা করি তখনই যার দিকে point করা হয়েছে, তখন আমরা invalid value deal করতে পারি।

আরও note করুন যে Listing 20-1 এবং 20-3 এ, আমরা `*const i32` এবং `*mut i32` raw pointer তৈরি করেছিলাম যা একই memory location point করে যেখানে `num` store করা আছে। যদি আমরা এর পরিবর্তে `num` এ immutable এবং mutable reference তৈরি করার চেষ্টা করতাম, তাহলে code compile হতো না কারণ Rust এর ownership rule একই সময়ে mutable reference এবং যেকোনো immutable reference allow করে না। Raw pointer এর সাথে, আমরা একই location এ একটি mutable pointer এবং একটি immutable pointer তৈরি করতে পারি এবং mutable pointer দিয়ে data change করতে পারি, potentially data race তৈরি করে। Be careful!

এইসব danger এর সাথে, আপনি raw pointer কেন ব্যবহার করবেন? একটি major use case হলো যখন C code এর সাথে interface করবেন, যা আপনি পরবর্তী section, [“Calling an Unsafe Function or Method”](#calling-an-unsafe-function-or-method)<!-- ignore --> এ দেখবেন। অন্য case হলো যখন safe abstraction তৈরি করবেন যা borrow checker বোঝে না। আমরা unsafe function introduce করব এবং তারপর unsafe code ব্যবহার করে এমন safe abstraction এর একটি উদাহরণ দেখব।

### Calling an Unsafe Function or Method

Unsafe block এ perform করতে পারা দ্বিতীয় type এর operation হলো unsafe function call করা। Unsafe function এবং method regular function এবং method এর মতোই দেখায়, কিন্তু definition এর বাকি অংশের আগে তাদের একটি extra `unsafe` থাকে। এই context এ `unsafe` keyword indicate করে যে function এর কিছু requirement আছে যা আমাদের এই function call করার সময় uphold করার প্রয়োজন, কারণ Rust guarantee দিতে পারে না যে আমরা এই requirement meet করেছি। `unsafe` block এর ভিতরে একটি unsafe function call করে, আমরা বলছি যে আমরা এই function এর documentation পড়েছি এবং বুঝতে পেরেছি কিভাবে এটিকে properly ব্যবহার করতে হয়, এবং আমরা verify করেছি যে আমরা function এর contract fulfill করছি।

এখানে `dangerous` নামের একটি unsafe function দেওয়া হলো যা এর body তে কিছুই করে না:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

আমাদের অবশ্যই separate `unsafe` block এর ভিতরে `dangerous` function call করতে হবে। যদি আমরা `unsafe` block ছাড়া `dangerous` call করার চেষ্টা করি, তাহলে আমরা error পাব:

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

`unsafe` block এর সাথে, আমরা Rust কে assert করছি যে আমরা function এর documentation পড়েছি, আমরা বুঝতে পেরেছি কিভাবে এটিকে properly ব্যবহার করতে হয়, এবং আমরা verify করেছি যে আমরা function এর contract fulfill করছি।

Unsafe function এর body তে unsafe operation perform করার জন্য, আপনার still একটি `unsafe` block ব্যবহার করার প্রয়োজন যেমন regular function এর ভিতরে করেন, এবং compiler আপনাকে warn করবে যদি আপনি ভুলে যান। এটা `unsafe` block কে যত ছোট possible তত ছোট রাখতে সাহায্য করে, কারণ unsafe operation পুরো function body জুড়ে প্রয়োজন নাও হতে পারে।

#### Creating a Safe Abstraction over Unsafe Code

শুধু কোনো function এ unsafe code থাকলেই পুরো function unsafe mark করার প্রয়োজন নেই। আসলে, unsafe code wrap করে safe function তৈরি করা একটি common abstraction। উদাহরণস্বরূপ, চলুন standard library থেকে `split_at_mut` function study করি, যার জন্য কিছু unsafe code এর প্রয়োজন। আমরা explore করব কিভাবে আমরা এটা implement করতে পারি। এই safe method টি mutable slice এ define করা হয়েছে: এটি একটি slice নেয় এবং argument হিসেবে দেওয়া index এ split করে slice টিকে দুটি slice এ পরিণত করে। Listing 20-4 দেখায় কিভাবে `split_at_mut` ব্যবহার করতে হয়।

<Listing number="20-4" caption="Safe `split_at_mut` function ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

আমরা শুধুমাত্র safe Rust ব্যবহার করে এই function implement করতে পারি না। একটি attempt Listing 20-5 এর মতো দেখতে হতে পারে, যা compile হবে না। Simplicity এর জন্য, আমরা `split_at_mut` কে একটি function হিসেবে implement করব method এর পরিবর্তে এবং শুধুমাত্র generic type `T` এর পরিবর্তে `i32` value এর slice এর জন্য।

<Listing number="20-5" caption="শুধুমাত্র safe Rust ব্যবহার করে `split_at_mut` implement করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

এই function প্রথমে slice এর total length পায়। তারপর এটি check করে argument হিসেবে দেওয়া index slice এর ভিতরে আছে কিনা, length এর চেয়ে ছোট বা সমান কিনা তা check করে। Assertion মানে হলো যদি আমরা slice split করার জন্য length এর চেয়ে বড় কোনো index pass করি, তাহলে function সেই index ব্যবহার করার attempt করার আগে panic করবে।

তারপর আমরা একটি tuple এ দুটি mutable slice return করি: একটি original slice এর শুরু থেকে `mid` index পর্যন্ত এবং অন্যটি `mid` থেকে slice এর শেষ পর্যন্ত।

যখন আমরা Listing 20-5 এর code compile করার চেষ্টা করি, তখন আমরা একটি error পাব।

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

Rust এর borrow checker বুঝতে পারে না যে আমরা slice এর different part borrow করছি; এটি শুধু জানে যে আমরা একই slice থেকে দুবার borrow করছি। Slice এর different part borrow করা fundamentally okay কারণ দুটি slice overlap করছে না, কিন্তু Rust এত smart নয় যে এটা জানবে। যখন আমরা জানি যে code okay, কিন্তু Rust জানে না, তখন unsafe code এর দিকে reach করার সময়।

Listing 20-6 দেখায় কিভাবে একটি `unsafe` block, raw pointer, এবং কিছু unsafe function call ব্যবহার করে `split_at_mut` এর implementation কাজ করানো যায়।

<Listing number="20-6" caption="`split_at_mut` function এর implementation এ unsafe code ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

Chapter 4 এর [“The Slice Type”][the-slice-type]<!-- ignore --> section থেকে মনে করুন যে slice হলো কিছু data এবং slice এর length এর একটি pointer। আমরা slice এর length পাওয়ার জন্য `len` method এবং slice এর raw pointer access করার জন্য `as_mut_ptr` method ব্যবহার করি। এই ক্ষেত্রে, যেহেতু আমাদের কাছে `i32` value এর mutable slice আছে, তাই `as_mut_ptr` type `*mut i32` এর একটি raw pointer return করে, যা আমরা variable `ptr` এ store করেছি।

আমরা assertion রাখি যে `mid` index slice এর ভিতরে আছে। তারপর আমরা unsafe code এ আসি: `slice::from_raw_parts_mut` function একটি raw pointer এবং একটি length নেয়, এবং এটি একটি slice তৈরি করে। আমরা এই function টি ব্যবহার করি `ptr` থেকে শুরু হওয়া এবং `mid` item long এমন একটি slice তৈরি করার জন্য। তারপর আমরা argument হিসেবে `mid` দিয়ে `ptr` এর উপর `add` method call করি `mid` এ শুরু হওয়া একটি raw pointer পাওয়ার জন্য, এবং আমরা সেই pointer এবং `mid` এর পরে থাকা remaining number of item কে length হিসেবে ব্যবহার করে একটি slice তৈরি করি।

`slice::from_raw_parts_mut` function unsafe কারণ এটি একটি raw pointer নেয় এবং এই pointer valid কিনা তা trust করতে হয়। Raw pointer এর উপর `add` method ও unsafe, কারণ offset location ও valid pointer কিনা তা trust করতে হয়। তাই, আমাদের `slice::from_raw_parts_mut` এবং `add` এ আমাদের call এর চারপাশে `unsafe` block রাখতে হয়েছিল যাতে আমরা সেগুলোকে call করতে পারি। Code দেখে এবং `mid` অবশ্যই `len` এর চেয়ে ছোট বা সমান হতে হবে এমন assertion add করে, আমরা বলতে পারি যে `unsafe` block এর ভিতরে ব্যবহার করা সব raw pointer slice এর ভিতরের data এর valid pointer হবে। এটি `unsafe` এর acceptable এবং appropriate use।

Note করুন যে আমাদের resulting `split_at_mut` function কে `unsafe` mark করার প্রয়োজন নেই, এবং আমরা এই function safe Rust থেকে call করতে পারি। আমরা unsafe code এর safe abstraction তৈরি করেছি এমন একটি function implement করার মাধ্যমে যা safe উপায়ে `unsafe` code ব্যবহার করে, কারণ এটি এই function এর access থাকা data থেকে valid pointer তৈরি করে।

অন্যদিকে, Listing 20-7 এ `slice::from_raw_parts_mut` এর ব্যবহার সম্ভবত crash করবে যখন slice ব্যবহার করা হবে। এই code একটি arbitrary memory location নেয় এবং 10,000 item long একটি slice তৈরি করে।

<Listing number="20-7" caption="একটি arbitrary memory location থেকে slice তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

সেই arbitrary location এ memory এর ownership আমাদের নেই, এবং এই code যে slice তৈরি করে তা valid `i32` value contain করে তার কোনো guarantee নেই। `values` কে valid slice মনে করে ব্যবহার করার attempt করলে undefined behaviour result হবে।

#### Using `extern` Functions to Call External Code

মাঝে মাঝে, আপনার Rust code এর অন্য language এ লেখা code এর সাথে interact করার প্রয়োজন হতে পারে। এর জন্য, Rust এ `extern` keyword আছে যা _Foreign Function Interface (FFI)_ তৈরি এবং ব্যবহার করা facilitate করে। FFI হলো একটি programming language এর জন্য function define করার এবং অন্য (foreign) programming language কে সেই function গুলো call করতে enable করার একটি উপায়।

Listing 20-8 দেখায় কিভাবে C standard library থেকে `abs` function এর সাথে integration set up করতে হয়। `extern` block এর ভিতরে declare করা function গুলো সাধারণত Rust code থেকে call করা unsafe, তাই সেগুলোকে `unsafe` ও mark করতে হয়। কারণ হলো অন্য language Rust এর rule এবং guarantee enforce করে না, এবং Rust সেগুলো check করতে পারে না, তাই programmer এর উপর responsibility পরে safety ensure করার।

<Listing number="20-8" file-name="src/main.rs" caption="অন্য language এ define করা `extern` function declare এবং call করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

`unsafe extern "C"` block এর ভিতরে, আমরা call করতে চাই এমন অন্য language থেকে external function এর name এবং signature list করি। `“C”` part define করে যে external function কোন _application binary interface (ABI)_ ব্যবহার করে: ABI define করে কিভাবে assembly level এ function call করতে হয়। `“C”` ABI most common এবং C programming language এর ABI follow করে।

তবে, এই particular function এ কোনো memory safety consideration নেই। আসলে, আমরা জানি যে `abs` এর যেকোনো call সবসময় যেকোনো `i32` এর জন্য safe হবে, তাই `unsafe extern` block এ থাকলেও এই specific function call করা safe তা বলার জন্য `safe` keyword ব্যবহার করতে পারি। একবার আমরা change করলে, এটাকে call করার জন্য আর `unsafe` block এর প্রয়োজন হবে না, যেমনটা Listing 20-9 এ দেখানো হয়েছে।

<Listing number="20-9" file-name="src/main.rs" caption="Explicitly একটি function কে `safe` mark করা একটি `unsafe extern` block এর ভিতরে এবং safe ভাবে call করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

একটি function কে `safe` mark করলে inherent ভাবে এটিকে safe করে না! এর পরিবর্তে, এটি Rust এর কাছে করা একটি promise এর মতো যে এটি _is_ safe। আপনার still responsibility আছে এটা নিশ্চিত করা যে সেই promise রাখা হয়েছে!

> #### Calling Rust Functions from Other Languages
>
> আমরা interface তৈরি করার জন্য `extern` ও ব্যবহার করতে পারি যা অন্য language কে Rust function call করার allow করে। পুরো `extern` block তৈরি করার পরিবর্তে, আমরা `extern` keyword add করি এবং relevant function এর জন্য `fn` keyword এর ঠিক আগে ব্যবহার করার জন্য ABI specify করি। আমাদের Rust compiler কে এই function এর name mangle না করার বলার জন্য একটি `#[unsafe(no_mangle)]` annotation add করার প্রয়োজন। _Mangling_ হলো যখন compiler কোনো function এর দেওয়া name কে একটি different name এ change করে যাতে compilation process এর অন্য অংশগুলো consume করার জন্য আরও information থাকে কিন্তু human readable কম হয়। প্রত্যেক programming language compiler name গুলো সামান্য different ভাবে mangle করে, তাই Rust function অন্য language দ্বারা nameable হওয়ার জন্য, আমাদের অবশ্যই Rust compiler এর name mangling disable করতে হবে। এটি unsafe কারণ built-in mangling ছাড়া library তে name collision হতে পারে, তাই আমাদের responsibility হলো নিশ্চিত করা যে export করা name mangling ছাড়া export করার জন্য safe।
>
> Following example এ, আমরা C code থেকে `call_from_c` function accessible করি, shared library তে compile করার পর এবং C থেকে link করার পর:
>
> ```rust
> #[unsafe(no_mangle)]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
>
> `extern` এর এই ব্যবহারের জন্য `unsafe` এর প্রয়োজন নেই।
>
## Unsafe Rust

এতক্ষণ পর্যন্ত আমরা যেসব code নিয়ে আলোচনা করেছি, সেগুলোর সব code compile time এ Rust এর memory safety guarantee enforce করেছে। তবে, Rust এর ভিতরে hidden একটি দ্বিতীয় language আছে যা এই memory safety guarantee enforce করে না: একে _unsafe Rust_ বলা হয় এবং এটি regular Rust এর মতোই কাজ করে, কিন্তু আমাদের extra superpower দেয়।

Unsafe Rust exist করে কারণ, by nature, static analysis conservative হয়। যখন compiler code guarantee uphold করে কিনা তা determine করার চেষ্টা করে, তখন কিছু invalid program accept করার চেয়ে কিছু valid program reject করা ভালো। যদিও code _might_ be okay, যদি Rust compiler এর confident হওয়ার জন্য যথেষ্ট information না থাকে, তাহলে এটি code reject করবে। এই case গুলোতে, compiler কে বলার জন্য আপনি unsafe code ব্যবহার করতে পারেন, "Trust me, I know what I’m doing"। তবে warned থাকুন যে আপনি নিজের risk এ unsafe Rust ব্যবহার করছেন: আপনি যদি unsafe code incorrectly ব্যবহার করেন, তাহলে memory unsafety এর কারণে problem হতে পারে, যেমন null pointer dereferencing।

Rust এর একটি unsafe alter ego থাকার অন্য কারণ হলো underlying computer hardware inherently unsafe। যদি Rust unsafe operation করার allow না করত, তাহলে আপনি কিছু task করতে পারতেন না। Rust এর low-level system programming, যেমন operating system এর সাথে directly interact করা বা এমনকি নিজের operating system লেখার মতো কাজ করার allow করার প্রয়োজন। Low-level system programming নিয়ে কাজ করা language এর goal গুলোর মধ্যে একটি। চলুন explore করি unsafe Rust দিয়ে আমরা কি করতে পারি এবং কিভাবে করতে পারি।

### Unsafe Superpowers

Unsafe Rust এ switch করার জন্য, `unsafe` keyword ব্যবহার করুন এবং তারপর একটি নতুন block শুরু করুন যা unsafe code hold করে। আপনি unsafe Rust এ পাঁচটি action নিতে পারেন যা আপনি safe Rust এ পারেন না, যাকে আমরা _unsafe superpowers_ বলি। সেই superpower গুলোতে include আছে:

- Raw pointer dereference করার ability
- Unsafe function বা method call করা
- Mutable static variable access বা modify করা
- Unsafe trait implement করা
- একটি `union` এর field access করা

এটা বোঝা গুরুত্বপূর্ণ যে `unsafe` borrow checker turn off করে না বা Rust এর অন্য কোনো safety check disable করে না: যদি আপনি unsafe code এ reference ব্যবহার করেন, তবুও সেটি check করা হবে। `unsafe` keyword শুধুমাত্র এই পাঁচটি feature এ access দেয় যা compiler memory safety এর জন্য check করে না। আপনি unsafe block এর ভিতরে still কিছু safety পাবেন।

এছাড়াও, `unsafe` মানে এই নয় যে block এর ভিতরের code necessarily dangerous বা সেখানে অবশ্যই memory safety problem থাকবে: intent হলো programmer হিসেবে, আপনি নিশ্চিত করবেন যে `unsafe` block এর ভিতরের code valid উপায়ে memory access করবে।

মানুষ ভুল করে, এবং ভুল হবেই, কিন্তু `unsafe` annotation করা block এর ভিতরে এই পাঁচটি unsafe operation require করার মাধ্যমে আপনি জানতে পারবেন যে memory safety এর সাথে related যেকোনো error অবশ্যই `unsafe` block এর ভিতরে থাকবে। `unsafe` block ছোট রাখুন; memory bug investigate করার সময় পরে আপনি কৃতজ্ঞ হবেন।

Unsafe code যত বেশি possible isolate করার জন্য, safe abstraction এর ভিতরে unsafe code enclose করা এবং একটি safe API provide করা best, যা আমরা chapter এর পরে unsafe function এবং method examine করার সময় discuss করব। Standard library এর কিছু অংশ unsafe code এর উপর safe abstraction হিসেবে implement করা হয়েছে যা audit করা হয়েছে। Safe abstraction এ unsafe code wrap করা `unsafe` এর ব্যবহারকে সব জায়গায় leak হওয়া থেকে prevent করে যেখানে আপনি বা আপনার user unsafe code দিয়ে implement করা functionality ব্যবহার করতে চাইতে পারেন, কারণ safe abstraction ব্যবহার করা safe।

চলুন একে একে পাঁচটি unsafe superpower দেখি। আমরা unsafe code এর safe interface provide করে এমন কিছু abstraction ও দেখব।

### Dereferencing a Raw Pointer

Chapter 4 এর [“Dangling References”][dangling-references]<!-- ignore --> section এ, আমরা mention করেছিলাম compiler নিশ্চিত করে যে reference গুলো সবসময় valid। Unsafe Rust এ _raw pointer_ নামে দুটি নতুন type আছে যা reference এর similar। Reference এর মতো, raw pointer immutable বা mutable হতে পারে এবং `*const T` এবং `*mut T` হিসেবে লেখা হয়। Asterisk dereference operator নয়; এটা type name এর অংশ। Raw pointer এর context এ, _immutable_ মানে হলো pointer dereference করার পর directly assign করা যাবে না।

Reference এবং smart pointer থেকে different, raw pointer:

- Borrowing rule ignore করার allow করে একই location এ immutable এবং mutable pointer বা multiple mutable pointer থাকার মাধ্যমে
- Valid memory point করার guarantee দেয় না
- Null হওয়ার allow করে
- Automatic cleanup implement করে না

Rust এর এই guarantee enforce করা থেকে opt out করার মাধ্যমে, আপনি greater performance বা অন্য কোনো language বা hardware এর সাথে interface করার ability এর বিনিময়ে guaranteed safety ছেড়ে দিতে পারেন যেখানে Rust এর guarantee apply হয় না।

Listing 20-1 দেখায় কিভাবে একটি immutable এবং একটি mutable raw pointer তৈরি করতে হয়।

<Listing number="20-1" caption="Raw borrow operator দিয়ে raw pointer তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

Notice করুন যে আমরা এই code এ `unsafe` keyword include করিনি। আমরা safe code এ raw pointer তৈরি করতে পারি; আমরা শুধুমাত্র unsafe block এর বাইরে raw pointer dereference করতে পারি না, যা আপনি একটু পরেই দেখবেন।

আমরা raw borrow operator ব্যবহার করে raw pointer তৈরি করেছি: `&raw const num` একটি `*const i32` immutable raw pointer তৈরি করে, এবং `&raw mut num` একটি `*mut i32` mutable raw pointer তৈরি করে। যেহেতু আমরা সেগুলোকে directly local variable থেকে তৈরি করেছি, তাই আমরা জানি যে এই particular raw pointer গুলো valid, কিন্তু আমরা যেকোনো raw pointer নিয়ে এই assumption করতে পারি না।

এটা demonstrate করার জন্য, এরপর আমরা এমন একটি raw pointer তৈরি করব যার validity নিয়ে আমরা নিশ্চিত হতে পারি না, raw reference operator ব্যবহার করার পরিবর্তে একটি value cast করার জন্য `as` ব্যবহার করে। Listing 20-2 দেখায় কিভাবে memory তে arbitrary location এ একটি raw pointer তৈরি করতে হয়। Arbitrary memory ব্যবহার করার চেষ্টা undefined: সেই address এ data থাকতে পারে নাও থাকতে পারে, compiler code optimize করতে পারে তাই কোনো memory access নাও থাকতে পারে, অথবা program segmentation fault এর সাথে error করতে পারে। সাধারণত, এইরকম code লেখার কোনো ভালো reason নেই, বিশেষ করে যেখানে আপনি এর পরিবর্তে raw borrow operator ব্যবহার করতে পারেন, কিন্তু এটা possible।

<Listing number="20-2" caption="Arbitrary memory address এ একটি raw pointer তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

মনে করুন যে আমরা safe code এ raw pointer তৈরি করতে পারি, কিন্তু আমরা raw pointer _dereference_ করতে পারি না এবং point করা data read করতে পারি না। Listing 20-3 এ, আমরা `*` dereference operator ব্যবহার করি raw pointer এর উপর যার জন্য একটি `unsafe` block require করে।

<Listing number="20-3" caption="একটি `unsafe` block এর ভিতরে raw pointer dereference করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

</Listing>

Pointer তৈরি করা কোনো harm করে না; যখন আমরা সেই value access করার চেষ্টা করি তখনই যার দিকে point করা হয়েছে, তখন আমরা invalid value deal করতে পারি।

আরও note করুন যে Listing 20-1 এবং 20-3 এ, আমরা `*const i32` এবং `*mut i32` raw pointer তৈরি করেছিলাম যা একই memory location point করে যেখানে `num` store করা আছে। যদি আমরা এর পরিবর্তে `num` এ immutable এবং mutable reference তৈরি করার চেষ্টা করতাম, তাহলে code compile হতো না কারণ Rust এর ownership rule একই সময়ে mutable reference এবং যেকোনো immutable reference allow করে না। Raw pointer এর সাথে, আমরা একই location এ একটি mutable pointer এবং একটি immutable pointer তৈরি করতে পারি এবং mutable pointer দিয়ে data change করতে পারি, potentially data race তৈরি করে। Be careful!

এইসব danger এর সাথে, আপনি raw pointer কেন ব্যবহার করবেন? একটি major use case হলো যখন C code এর সাথে interface করবেন, যা আপনি পরবর্তী section, [“Calling an Unsafe Function or Method”](#calling-an-unsafe-function-or-method)<!-- ignore --> এ দেখবেন। অন্য case হলো যখন safe abstraction তৈরি করবেন যা borrow checker বোঝে না। আমরা unsafe function introduce করব এবং তারপর unsafe code ব্যবহার করে এমন safe abstraction এর একটি উদাহরণ দেখব।

### Calling an Unsafe Function or Method

Unsafe block এ perform করতে পারা দ্বিতীয় type এর operation হলো unsafe function call করা। Unsafe function এবং method regular function এবং method এর মতোই দেখায়, কিন্তু definition এর বাকি অংশের আগে তাদের একটি extra `unsafe` থাকে। এই context এ `unsafe` keyword indicate করে যে function এর কিছু requirement আছে যা আমাদের এই function call করার সময় uphold করার প্রয়োজন, কারণ Rust guarantee দিতে পারে না যে আমরা এই requirement meet করেছি। `unsafe` block এর ভিতরে একটি unsafe function call করে, আমরা বলছি যে আমরা এই function এর documentation পড়েছি এবং বুঝতে পেরেছি কিভাবে এটিকে properly ব্যবহার করতে হয়, এবং আমরা verify করেছি যে আমরা function এর contract fulfill করছি।

এখানে `dangerous` নামের একটি unsafe function দেওয়া হলো যা এর body তে কিছুই করে না:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

আমাদের অবশ্যই separate `unsafe` block এর ভিতরে `dangerous` function call করতে হবে। যদি আমরা `unsafe` block ছাড়া `dangerous` call করার চেষ্টা করি, তাহলে আমরা error পাব:

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

`unsafe` block এর সাথে, আমরা Rust কে assert করছি যে আমরা function এর documentation পড়েছি, আমরা বুঝতে পেরেছি কিভাবে এটিকে properly ব্যবহার করতে হয়, এবং আমরা verify করেছি যে আমরা function এর contract fulfill করছি।

Unsafe function এর body তে unsafe operation perform করার জন্য, আপনার still একটি `unsafe` block ব্যবহার করার প্রয়োজন যেমন regular function এর ভিতরে করেন, এবং compiler আপনাকে warn করবে যদি আপনি ভুলে যান। এটা `unsafe` block কে যত ছোট possible তত ছোট রাখতে সাহায্য করে, কারণ unsafe operation পুরো function body জুড়ে প্রয়োজন নাও হতে পারে।

#### Creating a Safe Abstraction over Unsafe Code

শুধু কোনো function এ unsafe code থাকলেই পুরো function unsafe mark করার প্রয়োজন নেই। আসলে, unsafe code wrap করে safe function তৈরি করা একটি common abstraction। উদাহরণস্বরূপ, চলুন standard library থেকে `split_at_mut` function study করি, যার জন্য কিছু unsafe code এর প্রয়োজন। আমরা explore করব কিভাবে আমরা এটা implement করতে পারি। এই safe method টি mutable slice এ define করা হয়েছে: এটি একটি slice নেয় এবং argument হিসেবে দেওয়া index এ split করে slice টিকে দুটি slice এ পরিণত করে। Listing 20-4 দেখায় কিভাবে `split_at_mut` ব্যবহার করতে হয়।

<Listing number="20-4" caption="Safe `split_at_mut` function ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

আমরা শুধুমাত্র safe Rust ব্যবহার করে এই function implement করতে পারি না। একটি attempt Listing 20-5 এর মতো দেখতে হতে পারে, যা compile হবে না। Simplicity এর জন্য, আমরা `split_at_mut` কে একটি function হিসেবে implement করব method এর পরিবর্তে এবং শুধুমাত্র generic type `T` এর পরিবর্তে `i32` value এর slice এর জন্য।

<Listing number="20-5" caption="শুধুমাত্র safe Rust ব্যবহার করে `split_at_mut` implement করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

এই function প্রথমে slice এর total length পায়। তারপর এটি check করে argument হিসেবে দেওয়া index slice এর ভিতরে আছে কিনা, length এর চেয়ে ছোট বা সমান কিনা তা check করে। Assertion মানে হলো যদি আমরা slice split করার জন্য length এর চেয়ে বড় কোনো index pass করি, তাহলে function সেই index ব্যবহার করার attempt করার আগে panic করবে।

তারপর আমরা একটি tuple এ দুটি mutable slice return করি: একটি original slice এর শুরু থেকে `mid` index পর্যন্ত এবং অন্যটি `mid` থেকে slice এর শেষ পর্যন্ত।

যখন আমরা Listing 20-5 এর code compile করার চেষ্টা করি, তখন আমরা একটি error পাব।

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

Rust এর borrow checker বুঝতে পারে না যে আমরা slice এর different part borrow করছি; এটি শুধু জানে যে আমরা একই slice থেকে দুবার borrow করছি। Slice এর different part borrow করা fundamentally okay কারণ দুটি slice overlap করছে না, কিন্তু Rust এত smart নয় যে এটা জানবে। যখন আমরা জানি যে code okay, কিন্তু Rust জানে না, তখন unsafe code এর দিকে reach করার সময়।

Listing 20-6 দেখায় কিভাবে একটি `unsafe` block, raw pointer, এবং কিছু unsafe function call ব্যবহার করে `split_at_mut` এর implementation কাজ করানো যায়।

<Listing number="20-6" caption="`split_at_mut` function এর implementation এ unsafe code ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

Chapter 4 এর [“The Slice Type”][the-slice-type]<!-- ignore --> section থেকে মনে করুন যে slice হলো কিছু data এবং slice এর length এর একটি pointer। আমরা slice এর length পাওয়ার জন্য `len` method এবং slice এর raw pointer access করার জন্য `as_mut_ptr` method ব্যবহার করি। এই ক্ষেত্রে, যেহেতু আমাদের কাছে `i32` value এর mutable slice আছে, তাই `as_mut_ptr` type `*mut i32` এর একটি raw pointer return করে, যা আমরা variable `ptr` এ store করেছি।

আমরা assertion রাখি যে `mid` index slice এর ভিতরে আছে। তারপর আমরা unsafe code এ আসি: `slice::from_raw_parts_mut` function একটি raw pointer এবং একটি length নেয়, এবং এটি একটি slice তৈরি করে। আমরা এই function টি ব্যবহার করি `ptr` থেকে শুরু হওয়া এবং `mid` item long এমন একটি slice তৈরি করার জন্য। তারপর আমরা argument হিসেবে `mid` দিয়ে `ptr` এর উপর `add` method call করি `mid` এ শুরু হওয়া একটি raw pointer পাওয়ার জন্য, এবং আমরা সেই pointer এবং `mid` এর পরে থাকা remaining number of item কে length হিসেবে ব্যবহার করে একটি slice তৈরি করি।

`slice::from_raw_parts_mut` function unsafe কারণ এটি একটি raw pointer নেয় এবং এই pointer valid কিনা তা trust করতে হয়। Raw pointer এর উপর `add` method ও unsafe, কারণ offset location ও valid pointer কিনা তা trust করতে হয়। তাই, আমাদের `slice::from_raw_parts_mut` এবং `add` এ আমাদের call এর চারপাশে `unsafe` block রাখতে হয়েছিল যাতে আমরা সেগুলোকে call করতে পারি। Code দেখে এবং `mid` অবশ্যই `len` এর চেয়ে ছোট বা সমান হতে হবে এমন assertion add করে, আমরা বলতে পারি যে `unsafe` block এর ভিতরে ব্যবহার করা সব raw pointer slice এর ভিতরের data এর valid pointer হবে। এটি `unsafe` এর acceptable এবং appropriate use।

Note করুন যে আমাদের resulting `split_at_mut` function কে `unsafe` mark করার প্রয়োজন নেই, এবং আমরা এই function safe Rust থেকে call করতে পারি। আমরা unsafe code এর safe abstraction তৈরি করেছি এমন একটি function implement করার মাধ্যমে যা safe উপায়ে `unsafe` code ব্যবহার করে, কারণ এটি এই function এর access থাকা data থেকে valid pointer তৈরি করে।

অন্যদিকে, Listing 20-7 এ `slice::from_raw_parts_mut` এর ব্যবহার সম্ভবত crash করবে যখন slice ব্যবহার করা হবে। এই code একটি arbitrary memory location নেয় এবং 10,000 item long একটি slice তৈরি করে।

<Listing number="20-7" caption="একটি arbitrary memory location থেকে slice তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

সেই arbitrary location এ memory এর ownership আমাদের নেই, এবং এই code যে slice তৈরি করে তা valid `i32` value contain করে তার কোনো guarantee নেই। `values` কে valid slice মনে করে ব্যবহার করার attempt করলে undefined behaviour result হবে।

#### Using `extern` Functions to Call External Code

মাঝে মাঝে, আপনার Rust code এর অন্য language এ লেখা code এর সাথে interact করার প্রয়োজন হতে পারে। এর জন্য, Rust এ `extern` keyword আছে যা _Foreign Function Interface (FFI)_ তৈরি এবং ব্যবহার করা facilitate করে। FFI হলো একটি programming language এর জন্য function define করার এবং অন্য (foreign) programming language কে সেই function গুলো call করতে enable করার একটি উপায়।

Listing 20-8 দেখায় কিভাবে C standard library থেকে `abs` function এর সাথে integration set up করতে হয়। `extern` block এর ভিতরে declare করা function গুলো সাধারণত Rust code থেকে call করা unsafe, তাই সেগুলোকে `unsafe` ও mark করতে হয়। কারণ হলো অন্য language Rust এর rule এবং guarantee enforce করে না, এবং Rust সেগুলো check করতে পারে না, তাই programmer এর উপর responsibility পরে safety ensure করার।

<Listing number="20-8" file-name="src/main.rs" caption="অন্য language এ define করা `extern` function declare এবং call করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

`unsafe extern "C"` block এর ভিতরে, আমরা call করতে চাই এমন অন্য language থেকে external function এর name এবং signature list করি। `“C”` part define করে যে external function কোন _application binary interface (ABI)_ ব্যবহার করে: ABI define করে কিভাবে assembly level এ function call করতে হয়। `“C”` ABI most common এবং C programming language এর ABI follow করে।

তবে, এই particular function এ কোনো memory safety consideration নেই। আসলে, আমরা জানি যে `abs` এর যেকোনো call সবসময় যেকোনো `i32` এর জন্য safe হবে, তাই `unsafe extern` block এ থাকলেও এই specific function call করা safe তা বলার জন্য `safe` keyword ব্যবহার করতে পারি। একবার আমরা change করলে, এটাকে call করার জন্য আর `unsafe` block এর প্রয়োজন হবে না, যেমনটা Listing 20-9 এ দেখানো হয়েছে।

<Listing number="20-9" file-name="src/main.rs" caption="Explicitly একটি function কে `safe` mark করা একটি `unsafe extern` block এর ভিতরে এবং safe ভাবে call করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

একটি function কে `safe` mark করলে inherent ভাবে এটিকে safe করে না! এর পরিবর্তে, এটি Rust এর কাছে করা একটি promise এর মতো যে এটি _is_ safe। আপনার still responsibility আছে এটা নিশ্চিত করা যে সেই promise রাখা হয়েছে!

> #### Calling Rust Functions from Other Languages
>
> আমরা interface তৈরি করার জন্য `extern` ও ব্যবহার করতে পারি যা অন্য language কে Rust function call করার allow করে। পুরো `extern` block তৈরি করার পরিবর্তে, আমরা `extern` keyword add করি এবং relevant function এর জন্য `fn` keyword এর ঠিক আগে ব্যবহার করার জন্য ABI specify করি। আমাদের Rust compiler কে এই function এর name mangle না করার বলার জন্য একটি `#[unsafe(no_mangle)]` annotation add করার প্রয়োজন। _Mangling_ হলো যখন compiler কোনো function এর দেওয়া name কে একটি different name এ change করে যাতে compilation process এর অন্য অংশগুলো consume করার জন্য আরও information থাকে কিন্তু human readable কম হয়। প্রত্যেক programming language compiler name গুলো সামান্য different ভাবে mangle করে, তাই Rust function অন্য language দ্বারা nameable হওয়ার জন্য, আমাদের অবশ্যই Rust compiler এর name mangling disable করতে হবে। এটি unsafe কারণ built-in mangling ছাড়া library তে name collision হতে পারে, তাই আমাদের responsibility হলো নিশ্চিত করা যে export করা name mangling ছাড়া export করার জন্য safe।
>
> Following example এ, আমরা C code থেকে `call_from_c` function accessible করি, shared library তে compile করার পর এবং C থেকে link করার পর:
>
> ```rust
> #[unsafe(no_mangle)]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
>
> `extern` এর এই ব্যবহারের জন্য `unsafe` এর প্রয়োজন নেই।
