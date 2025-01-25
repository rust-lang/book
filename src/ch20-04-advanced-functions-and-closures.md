## Advanced Functions and Closures

এই section এ function pointer এবং closure return করা সহ function এবং closure related কিছু advanced feature explore করা হয়েছে।

### Function Pointers

আমরা discuss করেছি কিভাবে function এ closure pass করতে হয়; আপনি function এ regular function ও pass করতে পারেন! যখন আপনি একটি নতুন closure define করার পরিবর্তে ইতিমধ্যে define করা একটি function pass করতে চান তখন এই technique useful। Function type `fn` (with a lowercase f) এ coerce করে, `Fn` closure trait এর সাথে confuse করা উচিত না। `fn` type কে _function pointer_ বলা হয়। Function pointer দিয়ে function pass করলে আপনি function গুলোকে অন্য function এর argument হিসেবে ব্যবহার করতে পারবেন।

Parameter একটি function pointer এমন specify করার syntax closure এর similar, যা Listing 20-28 এ দেখানো হয়েছে, যেখানে আমরা `add_one` নামের একটি function define করেছি যা তার parameter এর সাথে এক যোগ করে। `do_twice` function দুটি parameter নেয়: একটি function pointer যা যেকোনো function এর জন্য i32 parameter নেয় এবং একটি i32 return করে, এবং একটি i32 value। `do_twice` function `f` function টি দুইবার call করে, এটিকে `arg` value pass করে, তারপর দুটি function call result একসাথে add করে। `main` function `add_one` এবং `5` argument দিয়ে `do_twice` call করে।

<Listing number="20-28" file-name="src/main.rs" caption="Function pointer কে argument হিসেবে accept করার জন্য `fn` type ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

এই code print করে `The answer is: 12`। আমরা `do_twice` এ parameter `f` declare করি এমন একটি `fn` হিসেবে যা `i32` type এর একটি parameter নেয় এবং একটি `i32` return করে। তারপর আমরা `do_twice` এর body তে `f` call করতে পারি। `main` এ, আমরা `do_twice` এর first argument হিসেবে function name `add_one` pass করতে পারি।

Closure এর বিপরীতে, `fn` একটি type, trait নয়, তাই আমরা trait bound হিসেবে `Fn` trait গুলোর একটির সাথে একটি generic type parameter declare করার পরিবর্তে parameter type হিসেবে directly `fn` specify করি।

Function pointer closure trait (`Fn`, `FnMut`, এবং `FnOnce`) এর তিনটেই implement করে, মানে আপনি সবসময় একটি function pointer কে closure expect করা function এর argument হিসেবে pass করতে পারেন। Generic type এবং closure trait গুলোর একটি ব্যবহার করে function লেখা best যাতে আপনার function function বা closure দুটোই accept করতে পারে।

তবে, এমন একটি উদাহরণ যেখানে আপনি শুধু `fn` accept করতে চান এবং closure নয় তা হলো যখন external code এর সাথে interface করেন যেখানে closure নেই: C function argument হিসেবে function accept করতে পারে, কিন্তু C তে closure নেই।

এমন একটি উদাহরণের জন্য যেখানে আপনি inline define করা একটি closure অথবা একটি named function ব্যবহার করতে পারেন, চলুন standard library তে `Iterator` trait দ্বারা provide করা `map` method এর use দেখি। Number এর vector কে string এর vector এ convert করার জন্য `map` function ব্যবহার করার জন্য, আমরা একটি closure ব্যবহার করতে পারতাম, যেমন:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-15-map-closure/src/main.rs:here}}
```

অথবা আমরা closure এর পরিবর্তে `map` এ argument হিসেবে একটি named function ব্যবহার করতে পারতাম, যেমন:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-16-map-function/src/main.rs:here}}
```

Note করুন যে আমাদের fully qualified syntax ব্যবহার করার প্রয়োজন যা নিয়ে আমরা [“Advanced Traits”][advanced-traits]<!-- ignore --> section এ আগে আলোচনা করেছি কারণ `to_string` নামের multiple function available আছে। এখানে, আমরা `ToString` trait এ define করা `to_string` function ব্যবহার করছি, যা standard library এমন যেকোনো type এর জন্য implement করেছে যা `Display` implement করে।

Chapter 6 এর [“Enum values”][enum-values]<!-- ignore --> section থেকে মনে করুন যে আমরা define করা প্রত্যেক enum variant এর নাম initializer function ও হয়ে যায়। আমরা এই initializer function গুলোকে function pointer হিসেবে ব্যবহার করতে পারি যা closure trait implement করে, মানে আমরা initializer function গুলোকে closure নেওয়া method এর argument হিসেবে specify করতে পারি, যেমন:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-17-map-initializer/src/main.rs:here}}
```

এখানে আমরা `Status::Value` instance তৈরি করি `Status::Value` এর initializer function ব্যবহার করে `map` call করা range এ থাকা প্রত্যেক `u32` value দিয়ে। কেউ কেউ এই style prefer করে, এবং কেউ কেউ closure ব্যবহার করতে prefer করে। তারা একই code এ compile হয়, তাই আপনার কাছে যেটা clear মনে হয় সেটা ব্যবহার করুন।

### Returning Closures

Closure trait দ্বারা represent করা হয়, যার মানে আপনি directly closure return করতে পারেন না। বেশিরভাগ case এ যেখানে আপনি একটি trait return করতে চাইতে পারেন, সেখানে এর পরিবর্তে আপনি সেই concrete type ব্যবহার করতে পারেন যা function এর return value হিসেবে trait implement করে। তবে, আপনি closure এর সাথে এটা করতে পারবেন না কারণ তাদের concrete type নেই যা return করা যায়; আপনি function pointer `fn` কে return type হিসেবে ব্যবহার করার allow না, উদাহরণস্বরূপ।

এর পরিবর্তে, আপনি normally Chapter 10 এ আমরা যে `impl Trait` syntax শিখেছি তা ব্যবহার করবেন। আপনি যেকোনো function type return করতে পারেন, `Fn`, `FnOnce` এবং `FnMut` ব্যবহার করে। উদাহরণস্বরূপ, এই code ঠিকঠাক কাজ করবে:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-18-returns-closure/src/lib.rs}}
```

তবে, Chapter 13 এর [“Closure Type Inference and Annotation”][closure-types]<!-- ignore --> section এ আমরা যেমন mention করেছিলাম, প্রত্যেক closure ও তার নিজের distinct type। যদি আপনাকে same signature কিন্তু different implementation আছে এমন multiple function এর সাথে কাজ করার প্রয়োজন হয়, তাহলে আপনাকে সেগুলোর জন্য একটি trait object ব্যবহার করার প্রয়োজন হবে:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-19-returns-closure-trait-object/src/main.rs}}
```

এই code ঠিকঠাক compile হবে—কিন্তু যদি আমরা `impl Fn(i32) -> i32` এর সাথে stick করার চেষ্টা করতাম তাহলে এটা compile হতো না। trait object নিয়ে আরও জানতে, Chapter 18 এর section [“Using Trait Objects That Allow for Values of Different Types”][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore --> refer করুন।

Next, চলুন macro নিয়ে আলোচনা করি!

[advanced-traits]: ch20-02-advanced-traits.html#advanced-traits
[enum-values]: ch06-01-defining-an-enum.html#enum-values
[closure-types]: ch13-01-closures.html#closure-type-inference-and-annotation
[using-trait-objects-that-allow-for-values-of-different-types]: ch18-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
