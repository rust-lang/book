## Using Trait Objects That Allow for Values of Different Types

Chapter 8 এ, আমরা mention করেছিলাম যে vector এর একটি limitation হলো এটি শুধুমাত্র এক type এর element store করতে পারে। আমরা Listing 8-9 এ একটি workaround তৈরি করেছিলাম যেখানে আমরা একটি `SpreadsheetCell` enum define করেছিলাম যেখানে integer, float, এবং text hold করার জন্য variant ছিল। এর মানে হলো আমরা প্রত্যেক cell এ ভিন্ন type এর data store করতে পারতাম এবং still cell এর row represent করে এমন একটি vector পেতাম। এটা perfectly ভালো একটি solution যখন আমাদের interchangeable item গুলো fixed set of type হয় যা আমরা code compile করার সময় জানি।

তবে, মাঝে মাঝে আমরা চাই আমাদের library user যেনো এমন type এর set extend করতে পারে যা particular situation এ valid। কিভাবে আমরা এটা achieve করতে পারি তা দেখানোর জন্য, আমরা graphical user interface (GUI) tool এর একটি উদাহরণ তৈরি করব যা item এর একটি list এর উপর iterate করে প্রত্যেকটিকে screen এ draw করার জন্য `draw` method call করে—GUI tool এর জন্য একটি common technique। আমরা `gui` নামে একটি library crate তৈরি করব যেখানে GUI library এর structure থাকবে। এই crate এ `Button` বা `TextField` এর মতো কিছু type থাকতে পারে যা মানুষজন ব্যবহার করতে পারবে। এছাড়াও, `gui` user রা draw করা যায় এমন নিজেদের type তৈরি করতে চাইবে: উদাহরণস্বরূপ, একজন programmer একটি `Image` add করতে পারে এবং অন্যজন একটি `SelectBox` add করতে পারে।

আমরা এই উদাহরণের জন্য fully fledged GUI library implement করব না, তবে দেখাবো কিভাবে অংশগুলো একসাথে fit হবে। Library লেখার সময়, আমরা জানতে পারব না এবং define করতে পারব না যে অন্য programmer রা কি type তৈরি করতে চাইতে পারে। কিন্তু আমরা জানি যে `gui` কে অনেক ভিন্ন type এর value এর track রাখতে হবে, এবং এই ভিন্ন type এর value গুলোর প্রত্যেকটিতে `draw` method call করতে হবে। `draw` method call করার সময় কি হবে তা সঠিকভাবে জানার প্রয়োজন নেই, শুধু জানতে হবে যে value তে call করার জন্য সেই method available থাকবে।

Inheritance থাকা একটি language এ এটা করার জন্য, আমরা `Component` নামে একটি class define করতে পারি যেখানে `draw` নামে একটি method থাকবে। অন্য class গুলো, যেমন `Button`, `Image`, এবং `SelectBox`, `Component` থেকে inherit করত এবং তাই `draw` method inherit করত। তারা প্রত্যেকটি তাদের custom behaviour define করার জন্য `draw` method override করতে পারত, কিন্তু framework সব type কে `Component` instance এর মতো treat করতে পারত এবং সেগুলোতে `draw` call করত। কিন্তু যেহেতু Rust এ inheritance নেই, তাই user দের নতুন type দিয়ে extend করার সুযোগ দেওয়ার জন্য `gui` library structure করার জন্য আমাদের অন্য উপায় প্রয়োজন।

### Defining a Trait for Common Behavior

আমরা `gui` তে যে behaviour চাই তা implement করার জন্য, আমরা `Draw` নামে একটি trait define করব যেখানে `draw` নামে একটি method থাকবে। তারপর আমরা একটি vector define করতে পারি যা একটি _trait object_ নেয়। একটি trait object আমাদের specified trait implement করা একটি type এর instance এবং runtime এ সেই type এর trait method lookup করার জন্য ব্যবহৃত একটি table দুটোই point করে। আমরা কোনো pointer যেমন একটি `&` reference বা একটি `Box<T>` smart pointer specify করে, তারপর `dyn` keyword দিয়ে, এবং তারপর relevant trait specify করে trait object তৈরি করি। (আমরা Chapter 20 এর section ["Dynamically Sized Types and the `Sized` Trait."][dynamically-sized]<!-- ignore --> এ trait object কে pointer ব্যবহার করতে হয় তার কারণ নিয়ে আলোচনা করব) আমরা generic বা concrete type এর পরিবর্তে trait object ব্যবহার করতে পারি। আমরা যেখানে trait object ব্যবহার করব, Rust এর type system compile time এ নিশ্চিত করবে যে সেই context এ ব্যবহৃত যেকোনো value trait object এর trait implement করবে। ফলে, compile time এ আমাদের সব possible type জানার প্রয়োজন নেই।

আমরা mention করেছি যে, Rust এ, আমরা struct এবং enum কে "object" call করা থেকে refrain করি যাতে তাদের অন্য language এর object থেকে আলাদা করা যায়। একটি struct বা enum এ, struct field এর data এবং `impl` block এর behaviour separate করা থাকে, যেখানে অন্য language এ, data এবং behaviour combine করে একটি concept তৈরি করে যাকে প্রায়ই object label করা হয়। তবে, trait object গুলো অন্য language এর object এর মতোই কারণ তারা data এবং behaviour combine করে। কিন্তু trait object traditional object থেকে different, কারণ আমরা trait object এ data add করতে পারি না। Trait object অন্য language এর object এর মতো generally useful নয়: তাদের specific purpose হলো common behaviour এর across এ abstraction allow করা।

Listing 18-3 দেখায় কিভাবে `draw` নামের একটি method সহ `Draw` নামে একটি trait define করতে হয়:

<Listing number="18-3" file-name="src/lib.rs" caption="`Draw` trait এর definition">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

</Listing>

এই syntax আমাদের Chapter 10 এ trait define করা নিয়ে আলোচনার থেকে familiar লাগা উচিত। এরপর কিছু নতুন syntax আসছে: Listing 18-4 এ `Screen` নামে একটি struct define করা হয়েছে যেখানে `components` নামে একটি vector আছে। এই vector টির type হলো `Box<dyn Draw>`, যা একটি trait object; এটি `Box` এর ভিতরে থাকা যেকোনো type এর stand-in যা `Draw` trait implement করে।

<Listing number="18-4" file-name="src/lib.rs" caption="`Draw` trait implement করা trait object এর vector ধারণ করা `components` field সহ `Screen` struct এর definition">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-04/src/lib.rs:here}}
```

</Listing>

`Screen` struct এর উপর, আমরা `run` নামে একটি method define করব যা এর প্রত্যেক `components` এর উপর `draw` method call করবে, যা Listing 18-5 এ দেখানো হয়েছে:

<Listing number="18-5" file-name="src/lib.rs" caption="`Screen` এর উপর `run` method যা প্রত্যেক component এর উপর `draw` method call করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-05/src/lib.rs:here}}
```

</Listing>

এটি trait bound দিয়ে generic type parameter ব্যবহার করা struct define করার থেকে ভিন্নভাবে কাজ করে। Generic type parameter শুধুমাত্র একটি সময়ে একটি concrete type দিয়ে substitute করা যেতে পারে, যেখানে trait object runtime এ trait object এর জন্য fill in করার জন্য multiple concrete type allow করে। উদাহরণস্বরূপ, আমরা Listing 18-6 এর মতো একটি generic type এবং একটি trait bound ব্যবহার করে `Screen` struct define করতে পারতাম:

<Listing number="18-6" file-name="src/lib.rs" caption="Generics এবং trait bound ব্যবহার করে `Screen` struct এবং এর `run` method এর alternate implementation">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-06/src/lib.rs:here}}
```

</Listing>

এটি আমাদের restrict করে `Screen` instance এ এমন component এর list রাখার জন্য যেগুলো সব `Button` type এর বা সব `TextField` type এর। আপনি যদি সবসময় homogeneous collection রাখতে চান, তাহলে generics এবং trait bound ব্যবহার করা preferable কারণ concrete type ব্যবহার করার জন্য definition compile time এ monomorphize হবে।

অন্যদিকে, trait object ব্যবহার করা method এর সাথে, একটি `Screen` instance একটি `Vec<T>` hold করতে পারে যেখানে একটি `Box<Button>` এবং একটি `Box<TextField>` দুটোই থাকতে পারে। চলুন দেখি এটা কিভাবে কাজ করে, এবং তারপর আমরা runtime performance implication নিয়ে আলোচনা করব।

### Implementing the Trait

এখন আমরা কিছু type add করব যা `Draw` trait implement করে। আমরা `Button` type provide করব। আবারও, আসলে একটি GUI library implement করা এই বইয়ের scope এর বাইরে, তাই `draw` method এর body তে কোনো useful implementation থাকবে না। Implementation দেখতে কেমন হতে পারে তা imagine করার জন্য, `Button` struct এ `width`, `height`, এবং `label` এর জন্য field থাকতে পারে, যা Listing 18-7 এ দেখানো হয়েছে:

<Listing number="18-7" file-name="src/lib.rs" caption="একটি `Button` struct যা `Draw` trait implement করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-07/src/lib.rs:here}}
```

</Listing>

`Button` এর `width`, `height`, এবং `label` field অন্য component এর field থেকে differ করবে; উদাহরণস্বরূপ, একটি `TextField` type এ সেই same field এর সাথে একটি `placeholder` field ও থাকতে পারে। Screen এ draw করতে চাওয়া প্রত্যেক type `Draw` trait implement করবে কিন্তু particular type draw করার জন্য `draw` method এ different code ব্যবহার করবে, যেমনটা এখানে `Button` করেছে (actual GUI code ছাড়া, যেমন mention করা হয়েছে)। উদাহরণস্বরূপ, `Button` type এ extra `impl` block থাকতে পারে যা user button click করলে কি হবে সেই related method ধারণ করে। এই ধরনের method `TextField` এর মতো type এর জন্য apply হবে না।

যদি আমাদের library ব্যবহার করা কেউ `SelectBox` struct implement করার সিদ্ধান্ত নেয় যেখানে `width`, `height`, এবং `options` field আছে, তাহলে তারা `SelectBox` type এর উপর `Draw` trait implement করে, যা Listing 18-8 এ দেখানো হয়েছে:

<Listing number="18-8" file-name="src/main.rs" caption="`gui` ব্যবহার করা অন্য crate এবং `SelectBox` struct এ `Draw` trait implement করা">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-08/src/main.rs:here}}
```

</Listing>

আমাদের library এর user এখন তাদের `main` function লিখে একটি `Screen` instance তৈরি করতে পারে। `Screen` instance এ, তারা `SelectBox` এবং `Button` add করতে পারে প্রত্যেকটিকে `Box<T>` এ রেখে trait object বানানোর মাধ্যমে। তারপর তারা `Screen` instance এ `run` method call করতে পারে, যা প্রত্যেক component এ `draw` call করবে। Listing 18-9 এই implementation দেখায়:

<Listing number="18-9" file-name="src/main.rs" caption="Same trait implement করা ভিন্ন type এর value store করার জন্য trait object ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-09/src/main.rs:here}}
```

</Listing>

যখন আমরা library লিখেছিলাম, তখন আমরা জানতাম না যে কেউ `SelectBox` type add করতে পারে, কিন্তু আমাদের `Screen` implementation নতুন type এর উপর operate করতে পেরেছিল এবং এটিকে draw করতে পেরেছিল কারণ `SelectBox` `Draw` trait implement করে, মানে এটি `draw` method implement করে।

এই concept—একটি value concrete type এর পরিবর্তে শুধুমাত্র response করা message নিয়ে concern থাকা—dynamic typed language এ _duck typing_ এর concept এর similar: যদি এটা হাঁসের মতো হাঁটে এবং হাঁসের মতো ডাকে, তাহলে এটা অবশ্যই হাঁস! Listing 18-5 এ `Screen` এর উপর `run` এর implementation এ, `run` এর প্রত্যেক component এর concrete type জানার প্রয়োজন নেই। এটি check করে না যে একটি component `Button` নাকি `SelectBox` এর instance, এটি শুধু component এর উপর `draw` method call করে। `components` vector এ value এর type হিসেবে `Box<dyn Draw>` specify করার মাধ্যমে, আমরা `Screen` কে এমন value এর need define করেছি যেগুলোতে আমরা `draw` method call করতে পারি।

Trait object এবং duck typing ব্যবহার করা code এর মতো code লেখার জন্য Rust এর type system ব্যবহার করার advantage হলো আমাদের never runtime এ check করতে হয় কিনা যে একটি value particular method implement করে বা worry করতে হয় যদি কোনো value method implement না করে কিন্তু আমরা still call করি। Rust আমাদের code compile করবে না যদি value গুলো trait object এর required trait implement না করে।

উদাহরণস্বরূপ, Listing 18-10 দেখায় কি হবে যদি আমরা component হিসেবে `String` দিয়ে একটি `Screen` তৈরি করার চেষ্টা করি:

<Listing number="18-10" file-name="src/main.rs" caption="এমন একটি type ব্যবহার করার চেষ্টা করা যা trait object এর trait implement করে না">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-10/src/main.rs}}
```

</Listing>

আমরা এই error টি পাব কারণ `String` `Draw` trait implement করে না:

```console
{{#include ../listings/ch18-oop/listing-18-10/output.txt}}
```

এই error আমাদের জানায় যে হয় আমরা `Screen` এ এমন কিছু pass করছি যা আমাদের pass করার কথা ছিল না এবং তাই আমাদের অন্য type pass করা উচিত অথবা `String` এ `Draw` implement করা উচিত যাতে `Screen` এটির উপর `draw` call করতে পারে।

### Trait Objects Perform Dynamic Dispatch

Chapter 10 এর [“Performance of Code Using Generics”][performance-of-code-using-generics]<!-- ignore --> section এ compiler দ্বারা generics এর উপর perform করা monomorphization process নিয়ে আমাদের discussion মনে করুন: compiler function এর nongeneric implementation generate করে এবং প্রত্যেক concrete type এর জন্য method generate করে যা আমরা generic type parameter এর জায়গায় ব্যবহার করি। Monomorphization থেকে result হওয়া code _static dispatch_ করে, যা তখন হয় যখন compiler compile time এ জানে আপনি কোন method call করছেন। এটি _dynamic dispatch_ এর opposite, যা তখন হয় যখন compiler compile time এ বলতে পারে না আপনি কোন method call করছেন। Dynamic dispatch এর ক্ষেত্রে, compiler এমন code emit করে যা runtime এ figure out করবে কোন method call করতে হবে।

যখন আমরা trait object ব্যবহার করি, Rust কে অবশ্যই dynamic dispatch ব্যবহার করতে হবে। Compiler জানে না যে trait object ব্যবহার করা code এর সাথে কোন type গুলো ব্যবহার হতে পারে, তাই এটি জানে না কোন method কোন type এ implement করা হয়েছে তা call করতে হবে। এর পরিবর্তে, runtime এ, Rust trait object এর ভিতরের pointer গুলো ব্যবহার করে কোন method call করতে হবে তা জানার জন্য। এই lookup এ runtime cost লাগে যা static dispatch এর সাথে হয় না। Dynamic dispatch compiler কে method এর code inline করা থেকে ও prevent করে, যা কিছু optimization prevent করে, এবং dynamic dispatch আপনি কোথায় ব্যবহার করতে পারবেন আর পারবেন না তা নিয়ে Rust এর কিছু rule ও আছে, যাকে [_dyn compatibility_][dyn-compatibility] বলা হয়। তবে, Listing 18-5 এ লেখা code এ এবং Listing 18-9 এ support করার জন্য আমরা extra flexibility পেয়েছিলাম, তাই এটি consider করার মতো একটি tradeoff।

[performance-of-code-using-generics]: ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch20-03-advanced-types.html#dynamically-sized-types-and-the-sized-trait
[dyn-compatibility]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
