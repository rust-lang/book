## `Deref` Trait-এর সাহায্যে Smart Pointer-গুলোকে সাধারণ রেফারেন্সের মতো ব্যবহার করা

`Deref` trait implement করার মাধ্যমে আপনি _dereference operator_ `*`-এর behavior কাস্টমাইজ করতে পারবেন (গুণ বা glob অপারেটরের সাথে বিভ্রান্ত হবেন না)। একটি স্মার্ট পয়েন্টারকে সাধারণ রেফারেন্সের মতো ব্যবহার করার জন্য `Deref` implement করে, আপনি এমন কোড লিখতে পারেন যা রেফারেন্সে operate করে এবং সেই কোডটি স্মার্ট পয়েন্টারগুলোর সাথেও ব্যবহার করতে পারেন।

আসুন প্রথমে দেখি কিভাবে dereference operator টি regular reference-এর সাথে কাজ করে। তারপর আমরা `Box<T>`-এর মতো আচরণ করে এমন একটি custom type define করার চেষ্টা করব এবং দেখব কেন dereference operator টি আমাদের newly defined type-এ reference-এর মতো কাজ করে না। আমরা explore করব কিভাবে `Deref` trait implement করা smart pointer গুলোকে reference-এর মতোই কাজ করতে সক্ষম করে। তারপর আমরা Rust-এর _deref coercion_ feature দেখব এবং এটি কীভাবে আমাদের reference বা smart pointer-এর সাথে কাজ করতে দেয়।

> দ্রষ্টব্য: আমরা যে `MyBox<T>` টাইপটি তৈরি করতে যাচ্ছি এবং real `Box<T>`-এর মধ্যে একটি বড় পার্থক্য রয়েছে: আমাদের version-টি heap-এ ডেটা store করবে না। আমরা এই উদাহরণটিকে `Deref`-এর উপর ফোকাস করছি, তাই ডেটা আসলে কোথায় store করা হয়েছে তা পয়েন্টারের মতো আচরণের চেয়ে কম গুরুত্বপূর্ণ।

<!-- পুরানো লিঙ্ক, সরাবেন না -->

<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>

### পয়েন্টারকে অনুসরণ করে Value-তে যাওয়া

একটি regular reference হল এক ধরনের পয়েন্টার, এবং একটি পয়েন্টারকে অন্য কোথাও store করা একটি value-এর একটি তীরচিহ্ন হিসেবে ভাবা যেতে পারে। Listing 15-6-এ, আমরা একটি `i32` value-এর একটি reference তৈরি করি এবং তারপর value-এর reference-টিকে follow করতে dereference operator ব্যবহার করি:

<Listing number="15-6" file-name="src/main.rs" caption="একটি `i32` value-এর reference-কে follow করতে dereference operator ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-6/src/main.rs}}
```

</Listing>

`X` variable-টি একটি `i32` value `5` ধারণ করে। আমরা `y`-কে `x`-এর একটি reference-এর সমান set করি। আমরা assert করতে পারি যে `x`, `5`-এর সমান। যাইহোক, যদি আমরা `y`-এর value সম্পর্কে একটি assertion করতে চাই, তাহলে আমাদের `*y` ব্যবহার করতে হবে reference-টিকে follow করে যে value-টির দিকে এটি point করছে (তাই _dereference_) যাতে compiler actual value-টি compare করতে পারে। একবার আমরা `y` কে dereference করলে, আমরা integer value-টিতে অ্যাক্সেস পাই যেখানে `y` point করছে যা আমরা `5`-এর সাথে compare করতে পারি।

যদি আমরা `assert_eq!(5, y);` লেখার চেষ্টা করতাম, তাহলে আমরা এই compilation error পেতাম:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

একটি সংখ্যা এবং একটি সংখ্যার reference-এর মধ্যে তুলনা করার অনুমতি নেই কারণ সেগুলো different type। আমাদের অবশ্যই dereference operator ব্যবহার করতে হবে reference-টিকে follow করে যে value-টির দিকে এটি point করছে।

### `Box<T>`-কে রেফারেন্সের মতো ব্যবহার করা

আমরা Listing 15-6-এর কোডটিকে একটি reference-এর পরিবর্তে একটি `Box<T>` ব্যবহার করার জন্য পুনরায় লিখতে পারি; Listing 15-7-এ `Box<T>`-তে ব্যবহৃত dereference operator টি Listing 15-6-এ reference-এ ব্যবহৃত dereference operator-এর মতোই কাজ করে:

<Listing number="15-7" file-name="src/main.rs" caption="একটি `Box<i32>`-তে dereference operator ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

Listing 15-7 এবং Listing 15-6-এর মধ্যে প্রধান পার্থক্য হল এখানে আমরা `y`-কে set করি `x`-এর value-এর দিকে point করা একটি reference-এর পরিবর্তে `x`-এর একটি copied value-এর দিকে point করা একটি `Box<T>`-এর instance হিসেবে। শেষ assertion-এ, আমরা `Box<T>`-এর পয়েন্টারকে follow করতে dereference operator ব্যবহার করতে পারি একইভাবে যেভাবে আমরা করতাম যখন `y` একটি reference ছিল। এরপরে, আমরা explore করব `Box<T>`-এর মধ্যে কী special যা আমাদের dereference operator ব্যবহার করতে সক্ষম করে, আমাদের নিজস্ব type define করে।

### আমাদের নিজস্ব Smart Pointer Define করা

আসুন standard library দ্বারা provide করা `Box<T>` type-এর মতো একটি স্মার্ট পয়েন্টার তৈরি করি যাতে experience করা যায় কীভাবে স্মার্ট পয়েন্টারগুলো default ভাবে reference থেকে ভিন্ন আচরণ করে। তারপর আমরা দেখব কিভাবে dereference operator ব্যবহার করার ক্ষমতা যোগ করতে হয়।

`Box<T>` type-টি ultimately একটি element সহ একটি tuple struct হিসাবে define করা হয়, তাই Listing 15-8 একই ভাবে একটি `MyBox<T>` type define করে। আমরা `Box<T>`-তে define করা `new` ফাংশনের সাথে মেলানোর জন্য একটি `new` ফাংশনও define করব।

<Listing number="15-8" file-name="src/main.rs" caption="একটি `MyBox<T>` টাইপ Define করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-8/src/main.rs:here}}
```

</Listing>

আমরা `MyBox` নামক একটি struct define করি এবং একটি জেনেরিক প্যারামিটার `T` declare করি, কারণ আমরা চাই আমাদের type যেকোনো type-এর value ধারণ করুক। `MyBox` type হল type `T`-এর একটি element সহ একটি tuple struct। `MyBox::new` ফাংশনটি type `T`-এর একটি প্যারামিটার নেয় এবং একটি `MyBox` ইন্সট্যান্স রিটার্ন করে যা passed করা value ধারণ করে।

আসুন Listing 15-8-এ Listing 15-7-এর `main` ফাংশনটি যোগ করার চেষ্টা করি এবং `Box<T>`-এর পরিবর্তে আমরা define করা `MyBox<T>` type ব্যবহার করার জন্য এটিকে পরিবর্তন করি। Listing 15-9-এর কোডটি compile হবে না কারণ Rust জানে না কিভাবে `MyBox`-কে dereference করতে হয়।

<Listing number="15-9" file-name="src/main.rs" caption="`Box<T>` এবং reference যেভাবে ব্যবহার করেছি সেভাবে `MyBox<T>` ব্যবহার করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

এখানে resulting compilation error রয়েছে:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

আমাদের `MyBox<T>` type-টিকে dereference করা যাবে না কারণ আমরা আমাদের type-এ সেই ক্ষমতা implement করিনি। `*` অপারেটরের সাহায্যে dereferencing enable করতে, আমরা `Deref` trait implement করি।

<!-- পুরানো লিঙ্ক, সরাবেন না -->

<a id="treating-a-type-like-a-reference-by-implementing-the-deref-trait"></a>

### `Deref` Trait Implement করা

Chapter 10-এর [“একটি Type-এ একটি Trait Implement করা”][impl-trait]-এ আলোচনা করা হয়েছে, একটি trait implement করার জন্য, আমাদের trait-এর প্রয়োজনীয় method গুলোর জন্য implementation provide করতে হবে। Standard library দ্বারা provide করা `Deref` trait-এর জন্য আমাদের `deref` নামক একটি method implement করতে হবে যা `self` borrow করে এবং ভেতরের ডেটার একটি reference রিটার্ন করে। Listing 15-10 `MyBox<T>`-এর definition-এ যোগ করার জন্য `Deref`-এর একটি implementation রয়েছে:

<Listing number="15-10" file-name="src/main.rs" caption="`MyBox<T>`-তে `Deref` ইমপ্লিমেন্ট করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

`Type Target = T;` syntax টি `Deref` trait-এর ব্যবহার করার জন্য একটি associated type define করে। Associated type গুলো হল একটি জেনেরিক প্যারামিটার declare করার একটি সামান্য ভিন্ন উপায়, কিন্তু আপাতত আপনাকে সেগুলো নিয়ে চিন্তা করতে হবে না; আমরা সেগুলো Chapter 20-এ আরও বিশদে আলোচনা করব।

আমরা `deref` method-এর body-কে `&self.0` দিয়ে পূরণ করি যাতে `deref` সেই value-টির একটি reference রিটার্ন করে যাকে আমরা `*` অপারেটর দিয়ে অ্যাক্সেস করতে চাই; Chapter 5-এর [“বিভিন্ন Type তৈরি করতে Named Field ছাড়া Tuple Struct ব্যবহার করা”][tuple-structs] বিভাগ থেকে স্মরণ করুন যে `.0` একটি tuple struct-এর প্রথম value অ্যাক্সেস করে। Listing 15-9-এর `main` ফাংশনটি যেটি `MyBox<T>` value-তে `*` কল করে এখন compile হয় এবং assertion গুলো pass করে!

`Deref` trait ছাড়া, compiler শুধুমাত্র `&` reference গুলোকে dereference করতে পারে। `Deref` method compiler-কে `Deref` implement করে এমন যেকোনো type-এর value নেওয়ার এবং `deref` method call করে একটি `&` reference পাওয়ার ক্ষমতা দেয় যা এটি কীভাবে dereference করতে হয় তা জানে।

যখন আমরা Listing 15-9-এ `*y` enter করেছিলাম, তখন behind the scenes-এ Rust আসলে এই কোডটি চালিয়েছিল:

```rust,ignore
*(y.deref())
```

Rust `*` operator-টিকে `deref` method-এ একটি call এবং তারপর একটি plain dereference দিয়ে প্রতিস্থাপন করে যাতে আমাদের ভাবতে না হয় যে আমাদের `deref` method call করতে হবে কিনা। এই Rust feature টি আমাদের এমন কোড লিখতে দেয় যা identically function করে, আমাদের কাছে একটি regular reference থাকুক বা `Deref` implement করে এমন একটি type থাকুক।

`Deref` method-টি কেন একটি value-এর reference রিটার্ন করে এবং `*(y.deref())`-এর বন্ধনীর বাইরের plain dereference টি কেন এখনও প্রয়োজনীয়, তার কারণ হল ownership system। যদি `deref` method টি value-এর reference-এর পরিবর্তে সরাসরি value টি রিটার্ন করত, তাহলে value টি `self`-এর বাইরে move করা হত। আমরা এই ক্ষেত্রে বা বেশিরভাগ ক্ষেত্রে যেখানে আমরা dereference operator ব্যবহার করি সেখানে `MyBox<T>`-এর ভেতরের value-টির ownership নিতে চাই না।

মনে রাখবেন যে `*` operator টি `deref` method-এ একটি call এবং তারপর `*` operator-এ একটি call দিয়ে প্রতিস্থাপিত হয় শুধুমাত্র একবার, প্রতিবার যখন আমরা আমাদের কোডে একটি `*` ব্যবহার করি। যেহেতু `*` operator-এর substitution infinitely recurse করে না, তাই আমরা `i32` type-এর ডেটা পাই, যেটি Listing 15-9-এর `assert_eq!`-এর `5`-এর সাথে মেলে।

### Function এবং Method-এর সাথে Implicit Deref Coercion

_Deref coercion_ `Deref` trait implement করে এমন একটি type-এর reference-কে অন্য type-এর reference-এ convert করে। উদাহরণস্বরূপ, deref coercion `&String`-কে `&str`-এ convert করতে পারে কারণ `String` `Deref` trait implement করে যাতে এটি `&str` রিটার্ন করে। Deref coercion হল একটি সুবিধা যা Rust function এবং method-গুলোতে argument-এর উপর perform করে এবং শুধুমাত্র সেই type-গুলোতে কাজ করে যেগুলো `Deref` trait implement করে। এটি স্বয়ংক্রিয়ভাবে ঘটে যখন আমরা একটি particular type-এর value-এর একটি reference একটি function বা method-এ argument হিসেবে pass করি যা function বা method definition-এর parameter type-এর সাথে মেলে না। `Deref` method-এ call-গুলোর একটি sequence আমরা provide করা type-টিকে parameter-এর প্রয়োজনীয় type-এ convert করে।

Deref coercion Rust-এ যোগ করা হয়েছিল যাতে function এবং method call লেখার প্রোগ্রামারদের `&` এবং `*` দিয়ে অনেকগুলি explicit reference এবং dereference যোগ করার প্রয়োজন না হয়। Deref coercion feature টি আমাদের আরও কোড লিখতে দেয় যা reference বা smart pointer উভয়ের জন্যই কাজ করতে পারে।

Deref coercion কীভাবে কাজ করে তা দেখতে, আসুন Listing 15-8-এ define করা `MyBox<T>` type-টি এবং Listing 15-10-এ যোগ করা `Deref`-এর implementation ব্যবহার করি। Listing 15-11 একটি ফাংশনের definition দেখায় যার একটি string slice প্যারামিটার রয়েছে:

<Listing number="15-11" file-name="src/main.rs" caption="একটি `hello` ফাংশন যার `name` প্যারামিটারটি `&str` টাইপের">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

আমরা `hello` ফাংশনটিকে argument হিসেবে একটি string slice দিয়ে কল করতে পারি, যেমন উদাহরণস্বরূপ `hello("Rust");`। Deref coercion `MyBox<String>` type-এর value-এর reference দিয়ে `hello` কল করা সম্ভব করে, যেমনটি Listing 15-12-তে দেখানো হয়েছে:

<Listing number="15-12" file-name="src/main.rs" caption="`MyBox<String>` value-এর একটি reference দিয়ে `hello` কল করা, যা deref coercion-এর কারণে কাজ করে">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

এখানে আমরা `hello` ফাংশনটিকে `&m` আর্গুমেন্ট দিয়ে কল করছি, যেটি একটি `MyBox<String>` value-এর reference। যেহেতু আমরা Listing 15-10-এ `MyBox<T>`-তে `Deref` trait implement করেছি, তাই Rust `deref` কল করে `&MyBox<String>`-কে `&String`-এ পরিণত করতে পারে। Standard library `String`-এ `Deref`-এর একটি implementation provide করে যা একটি string slice রিটার্ন করে এবং এটি `Deref`-এর জন্য API ডকুমেন্টেশনে রয়েছে। Rust `&String`-কে `&str`-এ পরিণত করতে আবার `deref` কল করে, যেটি `hello` ফাংশনের definition-এর সাথে মেলে।

যদি Rust deref coercion implement না করত, তাহলে `&MyBox<String>` type-এর value দিয়ে `hello` কল করার জন্য আমাদের Listing 15-12-এর কোডের পরিবর্তে Listing 15-13-এর কোড লিখতে হত।

<Listing number="15-13" file-name="src/main.rs" caption="Rust-এর যদি deref coercion না থাকত তাহলে আমাদের যে কোড লিখতে হত">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

`(*m)` `MyBox<String>`-কে একটি `String`-এ dereference করে। তারপর `&` এবং `[..]` `String`-এর একটি string slice নেয় যা `hello`-এর signature-এর সাথে মেলানোর জন্য সম্পূর্ণ string-এর সমান। Deref coercion ছাড়া এই কোডটি এই সমস্ত symbol জড়িত থাকার কারণে পড়তে, লিখতে এবং বুঝতে আরও কঠিন। Deref coercion Rust-কে আমাদের জন্য স্বয়ংক্রিয়ভাবে এই conversion গুলো handle করার অনুমতি দেয়।

যখন জড়িত type গুলোর জন্য `Deref` trait define করা হয়, তখন Rust type গুলোকে analyze করবে এবং parameter-এর type-এর সাথে মেলানোর জন্য একটি reference পেতে যতবার প্রয়োজন ততবার `Deref::deref` ব্যবহার করবে। কতবার `Deref::deref` insert করতে হবে তা compile time-এ resolve করা হয়, তাই deref coercion-এর সুবিধা নেওয়ার জন্য কোনো runtime penalty নেই!

### Deref Coercion কীভাবে Mutability-র সাথে ইন্টারঅ্যাক্ট করে

আপনি যেভাবে immutable reference-গুলোতে `*` operator override করতে `Deref` trait ব্যবহার করেন, একইভাবে আপনি mutable reference-গুলোতে `*` operator override করতে `DerefMut` trait ব্যবহার করতে পারেন।

Rust তিনটি ক্ষেত্রে type এবং trait implementation খুঁজে পেলে deref coercion করে:

1.  `&T` থেকে `&U`-তে যখন `T: Deref<Target=U>`
2.  `&mut T` থেকে `&mut U`-তে যখন `T: DerefMut<Target=U>`
3.  `&mut T` থেকে `&U`-তে যখন `T: Deref<Target=U>`

প্রথম দুটি ক্ষেত্র একে অপরের মতোই, শুধুমাত্র দ্বিতীয়টি mutability implement করে। প্রথম ক্ষেত্রটি বলে যে যদি আপনার কাছে একটি `&T` থাকে এবং `T` কোনো type `U`-তে `Deref` implement করে, তাহলে আপনি transparently একটি `&U` পেতে পারেন। দ্বিতীয় ক্ষেত্রটি বলে যে mutable reference-গুলোর জন্যও একই deref coercion ঘটে।

তৃতীয় ক্ষেত্রটি আরও জটিল: Rust একটি mutable reference-কে একটি immutable reference-এ coerce করবে। কিন্তু এর বিপরীতটি _সম্ভব নয়_: immutable reference গুলো কখনই mutable reference-এ coerce হবে না। Borrowing rule-গুলোর কারণে, যদি আপনার কাছে একটি mutable reference থাকে, তাহলে সেই mutable reference-টি অবশ্যই সেই ডেটার একমাত্র reference হতে হবে (অন্যথায়, প্রোগ্রামটি compile হবে না)। একটি mutable reference-কে একটি immutable reference-এ convert করা কখনই borrowing rule গুলো ভাঙবে না। একটি immutable reference-কে একটি mutable reference-এ convert করার জন্য প্রয়োজন হবে যে initial immutable reference-টি সেই ডেটার একমাত্র immutable reference, কিন্তু borrowing rule গুলো সেটির গ্যারান্টি দেয় না। অতএব, Rust এই assumption করতে পারে না যে একটি immutable reference-কে একটি mutable reference-এ convert করা সম্ভব।

[impl-trait]: ch10-02-traits.html#implementing-a-trait-on-a-type
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
