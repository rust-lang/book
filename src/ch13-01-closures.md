<!-- পুরানো heading। সরাবেন না নাহলে link ভেঙে যেতে পারে। -->

<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## ক্লোজার: Anonymous Function যারা তাদের Environment Capture করতে পারে

Rust-এর closure গুলো হল anonymous function যাদেরকে আপনি একটি variable-এ save করতে পারেন অথবা অন্য function-গুলোতে argument হিসেবে pass করতে পারেন। আপনি একটি জায়গায় closure তৈরি করতে পারেন এবং তারপর অন্য কোনো context-এ evaluate করার জন্য closure টিকে অন্য কোথাও call করতে পারেন। Function-এর বিপরীতে, closure গুলো যে scope-এ define করা হয়েছে সেখান থেকে value capture করতে পারে। আমরা দেখাব কীভাবে এই closure feature গুলো code reuse এবং behavior customization-এর অনুমতি দেয়।

<!-- পুরানো heading। সরাবেন না নাহলে link ভেঙে যেতে পারে। -->

<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### Closure-এর সাহায্যে Environment Capture করা

আমরা প্রথমে পরীক্ষা করব কীভাবে আমরা closure ব্যবহার করে যে environment-এ সেগুলোকে define করা হয়েছে সেখান থেকে value capture করে পরে ব্যবহার করতে পারি। এখানে scenario টি হল: মাঝে মাঝে, আমাদের টি-শার্ট কোম্পানি প্রচারের অংশ হিসেবে আমাদের মেইলিং লিস্টের কাউকে একটি exclusive, limited-edition শার্ট উপহার দেয়। মেইলিং লিস্টের লোকেরা চাইলে তাদের প্রোফাইলে তাদের প্রিয় রং যোগ করতে পারে। যদি বিনামূল্যে শার্টের জন্য নির্বাচিত ব্যক্তির প্রিয় রং set করা থাকে, তাহলে তারা সেই রঙের শার্ট পায়। যদি ব্যক্তিটি প্রিয় রং specify না করে থাকে, তাহলে কোম্পানির কাছে বর্তমানে যে রঙের শার্ট সবচেয়ে বেশি আছে সেটি তারা পায়।

এটি implement করার অনেক উপায় রয়েছে। এই উদাহরণের জন্য, আমরা `ShirtColor` নামক একটি enum ব্যবহার করব যাতে `Red` এবং `Blue` variant রয়েছে (সরলতার জন্য available রং-এর সংখ্যা সীমিত করা হচ্ছে)। আমরা কোম্পানির inventory-কে একটি `Inventory` struct দিয়ে represent করি যার `shirts` নামে একটি field রয়েছে যাতে বর্তমানে stock-এ থাকা শার্টের রং represent করে এমন একটি `Vec<ShirtColor>` রয়েছে। `Inventory`-তে defined `giveaway` method টি বিনামূল্যে শার্ট বিজয়ীর optional শার্টের রং preference পায় এবং ব্যক্তিটি যে শার্টের রং পাবে তা return করে। এই setup টি Listing 13-1-এ দেখানো হয়েছে:

<Listing number="13-1" file-name="src/main.rs" caption="শার্ট কোম্পানির উপহার দেওয়ার পরিস্থিতি">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

`main`-এ defined `store`-এ এই limited-edition প্রচারের জন্য বিতরণ করার জন্য দুটি নীল শার্ট এবং একটি লাল শার্ট অবশিষ্ট রয়েছে। আমরা লাল শার্টের preference-সহ একজন user এবং কোনো preference ছাড়া একজন user-এর জন্য `giveaway` method টি call করি।

আবারও, এই code টি অনেকভাবে implement করা যেতে পারে, এবং এখানে, closure-গুলোর উপর focus করার জন্য, আমরা সেই concept গুলোতেই সীমাবদ্ধ রয়েছি যেগুলো আপনি ইতিমধ্যেই শিখেছেন, শুধুমাত্র `giveaway` method-এর body ছাড়া যেটি একটি closure ব্যবহার করে। `Giveaway` method-এ, আমরা user preference-টিকে `Option<ShirtColor>` type-এর একটি parameter হিসেবে পাই এবং `user_preference`-এ `unwrap_or_else` method call করি। [`unwrap_or_else` method on `Option<T>`][unwrap-or-else] standard library দ্বারা define করা হয়েছে। এটি একটি argument নেয়: কোনো argument ছাড়া একটি closure যা একটি value `T` return করে ( `Option<T>`-এর `Some` variant-এ stored একই type, এক্ষেত্রে `ShirtColor`।)। যদি `Option<T>` হল `Some` variant, `unwrap_or_else`, `Some`-এর ভেতরের value return করে। যদি `Option<T>` হল `None` variant, `unwrap_or_else` closure-টিকে call করে এবং closure দ্বারা returned value-টি return করে।

আমরা `unwrap_or_else`-এ argument হিসেবে closure expression `|| self.most_stocked()` specify করি। এটি এমন একটি closure যা নিজে কোনো parameter নেয় না (যদি closure-টির parameter থাকত, তাহলে সেগুলো দুটি vertical bar-এর মধ্যে থাকত)। Closure-এর body `self.most_stocked()` call করে। আমরা এখানে closure টি define করছি, এবং যদি result-এর প্রয়োজন হয় তাহলে `unwrap_or_else`-এর implementation পরে closure-টিকে evaluate করবে।

এই code run করলে print হবে:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

এখানে একটি interesting বিষয় হল যে আমরা একটি closure pass করেছি যেটি current `Inventory` instance-এ `self.most_stocked()` call করে। Standard library-র আমাদের defined `Inventory` বা `ShirtColor` type, বা এই scenario-তে আমরা যে logic ব্যবহার করতে চাই সে সম্পর্কে কিছু জানার প্রয়োজন ছিল না। Closure টি `self` `Inventory` instance-এর একটি immutable reference capture করে এবং আমরা যে code specify করি তার সাথে এটিকে `unwrap_or_else` method-এ pass করে। অন্যদিকে, function-গুলো এইভাবে তাদের environment capture করতে পারে না।

### ক্লোজার টাইপ ইনফারেন্স এবং অ্যানোটেশন

Function এবং closure-এর মধ্যে আরও পার্থক্য রয়েছে। Closure-গুলোতে সাধারণত parameter-গুলোর type বা return value annotate করার প্রয়োজন হয় না, যেমনটা `fn` function-গুলোতে করতে হয়। Function-গুলোতে type annotation প্রয়োজন কারণ type গুলো আপনার user-দের কাছে exposed একটি explicit interface-এর অংশ। এই interface-টিকে rigidly define করা important যাতে সবাই একমত হতে পারে যে একটি function কী type-এর value ব্যবহার করে এবং return করে। অন্যদিকে, closure-গুলোকে এভাবে exposed interface-এ ব্যবহার করা হয় না: এগুলো variable-এ store করা হয় এবং সেগুলোর নাম না দিয়ে এবং আমাদের library-র user-দের কাছে expose না করে ব্যবহার করা হয়।

Closure গুলো সাধারণত ছোট হয় এবং যেকোনো arbitrary scenario-তে নয়, শুধুমাত্র একটি narrow context-এর মধ্যেই relevant। এই limited context-গুলোর মধ্যে, compiler parameter-গুলোর type এবং return type infer করতে পারে, একইভাবে এটি বেশিরভাগ variable-এর type infer করতে সক্ষম (এমন কিছু বিরল ক্ষেত্র রয়েছে যেখানে compiler-এর closure type annotation-ও প্রয়োজন)।

Variable-গুলোর মতোই, আমরা যদি strictly প্রয়োজনের চেয়ে বেশি verbose না হয়ে explicitness এবং clarity বাড়াতে চাই তাহলে type annotation যোগ করতে পারি। একটি closure-এর জন্য type গুলো annotate করা Listing 13-2-তে দেখানো definition-এর মতো হবে। এই উদাহরণে, আমরা একটি closure define করছি এবং এটিকে একটি variable-এ store করছি, Listing 13-1-এ আমরা যেভাবে argument হিসেবে pass করার জায়গায় closure define করেছিলাম সেভাবে না করে।

<Listing number="13-2" file-name="src/main.rs" caption="ক্লোজারে প্যারামিটার এবং রিটার্ন ভ্যালু টাইপের ঐচ্ছিক টাইপ অ্যানোটেশন যোগ করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

Type annotation যোগ করার সাথে, closure-গুলোর syntax function-গুলোর syntax-এর সাথে আরও সাদৃশ্যপূর্ণ দেখায়। এখানে আমরা একটি function define করি যেটি তার parameter-এর সাথে 1 যোগ করে এবং একটি closure যার একই behavior রয়েছে, তুলনা করার জন্য। আমরা relevant অংশগুলোকে line up করার জন্য কিছু space যোগ করেছি। এটি তুলে ধরে যে কীভাবে closure syntax function syntax-এর মতোই, শুধুমাত্র pipe ব্যবহার করা এবং syntax-এর amount যা optional তা ছাড়া:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

প্রথম line-টি একটি function definition দেখায়, এবং দ্বিতীয় line-টি একটি fully annotated closure definition দেখায়। তৃতীয় line-এ, আমরা closure definition থেকে type annotation গুলো সরিয়ে দিই। চতুর্থ line-এ, আমরা bracket গুলো সরিয়ে দিই, যেগুলো optional কারণ closure body-তে শুধুমাত্র একটি expression রয়েছে। এগুলি সবই valid definition যা call করা হলে একই behavior তৈরি করবে। `add_one_v3` এবং `add_one_v4` line-গুলোতে closure গুলোকে evaluated হতে হবে compile হতে পারার জন্য কারণ type গুলো তাদের usage থেকে infer করা হবে। এটি `let v = Vec::new();`-এর মতোই, যেখানে Rust-এর type infer করতে পারার জন্য type annotation বা কোনো type-এর value `Vec`-এ insert করা প্রয়োজন।

Closure definition-গুলোর জন্য, compiler তাদের প্রতিটি parameter এবং তাদের return value-এর জন্য একটি concrete type infer করবে। উদাহরণস্বরূপ, Listing 13-3 একটি short closure-এর definition দেখায় যেটি শুধুমাত্র parameter হিসেবে পাওয়া value টি return করে। এই closure টি এই উদাহরণের উদ্দেশ্য ছাড়া খুব বেশি useful নয়। লক্ষ্য করুন যে আমরা definition-এ কোনো type annotation যোগ করিনি। যেহেতু কোনো type annotation নেই, তাই আমরা closure-টিকে যেকোনো type দিয়ে call করতে পারি, যা আমরা এখানে প্রথমবার `String` দিয়ে করেছি। যদি আমরা তারপর একটি integer দিয়ে `example_closure` call করার চেষ্টা করি, তাহলে আমরা একটি error পাব।

<Listing number="13-3" file-name="src/main.rs" caption="দুটি ভিন্ন টাইপ দিয়ে ইনফার করা টাইপযুক্ত ক্লোজার কল করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

Compiler আমাদের এই error দেয়:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

প্রথমবার যখন আমরা `String` value দিয়ে `example_closure` call করি, compiler `x`-এর type এবং closure-টির return type `String` হিসেবে infer করে। সেই type গুলো তারপর `example_closure`-এ closure-টিতে lock হয়ে যায়, এবং আমরা যখন একই closure-এর সাথে অন্য type ব্যবহার করার চেষ্টা করি তখন আমরা একটি type error পাই।

### Reference Capture করা বা Ownership Move করা

Closure-গুলো তাদের environment থেকে তিনটি উপায়ে value capture করতে পারে, যা function-এর তিনটি parameter নেওয়ার উপায়কে সরাসরি map করে: immutably borrow করা, mutably borrow করা এবং ownership নেওয়া। Function-এর body captured value-গুলোর সাথে কী করে তার উপর ভিত্তি করে closure ঠিক করবে কোনটি ব্যবহার করতে হবে।

Listing 13-4-এ, আমরা একটি closure define করি যা `list` নামের vector-টিতে একটি immutable reference capture করে কারণ value print করার জন্য এটির শুধুমাত্র একটি immutable reference প্রয়োজন:

<Listing number="13-4" file-name="src/main.rs" caption="একটি অপরিবর্তনীয় রেফারেন্স ক্যাপচার করে এমন একটি ক্লোজার সংজ্ঞায়িত করা এবং কল করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

এই উদাহরণটি আরও তুলে ধরে যে একটি variable একটি closure definition-এর সাথে bind হতে পারে, এবং আমরা পরে variable-এর নাম এবং parentheses ব্যবহার করে closure-টিকে call করতে পারি যেন variable-এর নামটি একটি function-এর নাম।

যেহেতু আমরা একই সময়ে `list`-এ multiple immutable reference রাখতে পারি, তাই closure definition-এর আগে, closure definition-এর পরে কিন্তু closure call করার আগে এবং closure call করার পরে code থেকে `list` এখনও accessible। এই code compile, run এবং print করে:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

এরপরে, Listing 13-5-এ, আমরা closure body পরিবর্তন করি যাতে এটি `list` vector-এ একটি element যোগ করে। Closure টি এখন একটি mutable reference capture করে:

<Listing number="13-5" file-name="src/main.rs" caption="পরিবর্তনযোগ্য রেফারেন্স ক্যাপচার করে এমন একটি ক্লোজার সংজ্ঞায়িত করা এবং কল করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

এই code compile, run এবং print করে:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

লক্ষ্য করুন যে `borrows_mutably` closure-এর definition এবং call-এর মধ্যে আর কোনো `println!` নেই: যখন `borrows_mutably` define করা হয়, তখন এটি `list`-এ একটি mutable reference capture করে। Closure call করার পরে আমরা আবার closure টি ব্যবহার করি না, তাই mutable borrow শেষ হয়ে যায়। Closure definition এবং closure call-এর মধ্যে, print করার জন্য একটি immutable borrow-এর অনুমতি নেই কারণ mutable borrow থাকলে অন্য কোনো borrow-এর অনুমতি নেই। সেখানে একটি `println!` যোগ করে দেখুন আপনি কী error message পান!

এমনকি closure-এর body-র ownership-এর strictly প্রয়োজন না হলেও, আপনি যদি closure-টিকে environment-এ ব্যবহৃত value-গুলোর ownership নিতে বাধ্য করতে চান, তাহলে আপনি parameter list-এর আগে `move` keyword টি ব্যবহার করতে পারেন।

এই technique টি বেশিরভাগ ক্ষেত্রে उपयोगी হয় যখন একটি closure-কে একটি নতুন thread-এ pass করা হয় যাতে data-টিকে move করা যায় যাতে এটি new thread-এর owned হয়। আমরা Chapter 16-এ thread নিয়ে বিস্তারিত আলোচনা করব এবং concurrency নিয়ে কথা বলার সময় আপনি কেন সেগুলো ব্যবহার করতে চাইবেন, কিন্তু আপাতত, আসুন সংক্ষেপে একটি new thread spawn করা explore করি একটি closure ব্যবহার করে যেখানে `move` keyword-এর প্রয়োজন। Listing 13-6, Listing 13-4-কে modify করে main thread-এর পরিবর্তে একটি new thread-এ vector print করার জন্য:

<Listing number="13-6" file-name="src/main.rs" caption="`list`-এর ownership নিতে thread-এর জন্য closure-টিকে বাধ্য করতে `move` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

আমরা একটি new thread spawn করি, thread-টিকে argument হিসেবে run করার জন্য একটি closure দিই। Closure body list-টি print করে। Listing 13-4-এ, closure টি শুধুমাত্র একটি immutable reference ব্যবহার করে `list` capture করেছিল কারণ এটি print করার জন্য `list`-এ least amount-এর access-এর প্রয়োজন ছিল। এই উদাহরণে, যদিও closure body-র এখনও শুধুমাত্র একটি immutable reference প্রয়োজন, তবুও আমাদের specify করতে হবে যে `list`-কে closure-এ move করা উচিত closure definition-এর শুরুতে `move` keyword টি বসিয়ে। New thread টি main thread-এর বাকি অংশ শেষ হওয়ার আগে শেষ হয়ে যেতে পারে, অথবা main thread টি প্রথমে শেষ হতে পারে। যদি main thread `list`-এর ownership বজায় রাখত কিন্তু new thread-এর আগে শেষ হয়ে যেত এবং `list` drop করত, তাহলে thread-এর immutable reference টি invalid হয়ে যেত। অতএব, compiler-এর প্রয়োজন যে `list`-কে new thread-এ দেওয়া closure-টিতে move করা হোক যাতে reference টি valid হয়। `Move` keyword টি সরিয়ে বা closure define করার পরে main thread-এ `list` ব্যবহার করে দেখুন আপনি কী compiler error পান!

<!-- পুরানো heading। সরাবেন না নাহলে link ভেঙে যেতে পারে। -->

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Closure থেকে Captured Value-গুলোকে সরানো এবং `Fn` Traits

Closure টি যে environment-এ define করা হয়েছে সেখান থেকে একটি reference capture করার পরে বা ownership নেওয়ার পরে (এইভাবে closure-এর *মধ্যে* কী move করা হয়েছে, যদি কিছু move করা হয়ে থাকে, তাকে প্রভাবিত করে), closure-এর body-র code define করে যে closure-টি পরে evaluate করা হলে reference বা value-গুলোর কী হবে (এইভাবে closure-এর *বাইরে* কী move করা হয়েছে, যদি কিছু move করা হয়ে থাকে, তাকে প্রভাবিত করে)। একটি closure body নিম্নলিখিত যেকোনো একটি কাজ করতে পারে: একটি captured value-কে closure-এর বাইরে move করা, captured value-কে mutate করা, value-টিকে move বা mutate কোনোটিই না করা, অথবা শুরুতেই environment থেকে কিছুই capture না করা।

একটি closure যেভাবে environment থেকে value capture করে এবং handle করে তা প্রভাবিত করে closure টি কোন trait গুলো implement করে, এবং trait-গুলোর মাধ্যমেই function এবং struct গুলো specify করতে পারে যে তারা কোন ধরনের closure ব্যবহার করতে পারে। Closure-গুলো স্বয়ংক্রিয়ভাবে এই তিনটি `Fn` trait-এর মধ্যে একটি, দুটি, অথবা তিনটিই implement করবে, একটি additive পদ্ধতিতে, closure-এর body কীভাবে value-গুলো handle করে তার উপর নির্ভর করে:

1.  `FnOnce` সেই closure-গুলোর ক্ষেত্রে প্রযোজ্য যাদেরকে একবার call করা যেতে পারে। সমস্ত closure অন্তত এই trait টি implement করে, কারণ সমস্ত closure-কেই call করা যেতে পারে। একটি closure যেটি captured value-গুলোকে তার body-র বাইরে move করে সেটি শুধুমাত্র `FnOnce` implement করবে এবং অন্য কোনো `Fn` trait implement করবে না, কারণ এটিকে শুধুমাত্র একবার call করা যেতে পারে।
2.  `FnMut` সেই closure-গুলোর ক্ষেত্রে প্রযোজ্য যেগুলো captured value-গুলোকে তাদের body-র বাইরে move করে না, কিন্তু captured value-গুলোকে mutate করতে পারে। এই closure-গুলোকে একাধিকবার call করা যেতে পারে।
3.  `Fn` সেই closure-গুলোর ক্ষেত্রে প্রযোজ্য যেগুলো captured value-গুলোকে তাদের body-র বাইরে move করে না এবং captured value-গুলোকে mutate করে না, সেইসাথে সেই closure-গুলোর ক্ষেত্রেও যেগুলো তাদের environment থেকে কিছুই capture করে না। এই closure-গুলোকে তাদের environment-কে mutate না করে একাধিকবার call করা যেতে পারে, যা এমন পরিস্থিতিতে important যেখানে একটি closure-কে একাধিকবার concurrently call করা হয়।

আসুন `Option<T>`-তে `unwrap_or_else` method-টির definition দেখি যেটি আমরা Listing 13-1-এ ব্যবহার করেছি:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

মনে রাখবেন যে `T` হল generic type যা `Option`-এর `Some` variant-এ value-টির type represent করে। সেই type `T`, `unwrap_or_else` function-টির return type-ও: উদাহরণস্বরূপ, যে code `Option<String>`-এ `unwrap_or_else` call করে সেটি একটি `String` পাবে।

এরপরে, লক্ষ্য করুন যে `unwrap_or_else` function-টিতে অতিরিক্ত generic type parameter `F` রয়েছে। `F` type হল `f` নামের parameter-টির type, যেটি হল সেই closure যা আমরা `unwrap_or_else` call করার সময় provide করি।

Generic type `F`-এ specified trait bound হল `FnOnce() -> T`, যার মানে `F` অবশ্যই একবার call করা যেতে হবে, কোনো argument নেবে না এবং একটি `T` return করবে। Trait bound-এ `FnOnce` ব্যবহার করা এই constraint-টি প্রকাশ করে যে `unwrap_or_else` শুধুমাত্র `f`-কে সর্বাধিক একবার call করবে। `Unwrap_or_else`-এর body-তে, আমরা দেখতে পাচ্ছি যে যদি `Option` টি `Some` হয়, তাহলে `f` call করা হবে না। যদি `Option` টি `None` হয়, তাহলে `f` একবার call করা হবে। যেহেতু সমস্ত closure `FnOnce` implement করে, তাই `unwrap_or_else` সমস্ত তিনটি closure-এর প্রকারভেদ accept করে এবং যতটা সম্ভব flexible।

> দ্রষ্টব্য: যদি আমাদের যা করতে হবে তার জন্য environment থেকে value capture করার প্রয়োজন না হয়, তাহলে আমরা closure-এর পরিবর্তে একটি function-এর নাম ব্যবহার করতে পারি। উদাহরণস্বরূপ, যদি value `None` হয় তাহলে একটি নতুন, empty vector পেতে আমরা একটি `Option<Vec<T>>` value-তে `unwrap_or_else(Vec::new)` কল করতে পারতাম। Compiler স্বয়ংক্রিয়ভাবে function definition-এর জন্য প্রযোজ্য `Fn` trait-গুলোর মধ্যে যেটি প্রযোজ্য সেটি implement করে।

এখন আসুন slice-গুলোতে defined standard library method `sort_by_key` দেখি, এটি `unwrap_or_else` থেকে কীভাবে আলাদা এবং কেন `sort_by_key` trait bound-এর জন্য `FnOnce`-এর পরিবর্তে `FnMut` ব্যবহার করে। Closure-টি consider করা slice-এর current item-এর একটি reference-এর আকারে একটি argument পায় এবং type `K`-এর একটি value return করে যেটিকে order করা যেতে পারে। এই function টি useful যখন আপনি প্রতিটি item-এর একটি particular attribute-এর উপর ভিত্তি করে একটি slice sort করতে চান। Listing 13-7-এ, আমাদের কাছে `Rectangle` instance-এর একটি list রয়েছে এবং আমরা সেগুলোকে তাদের `width` attribute-এর উপর ভিত্তি করে low থেকে high-তে order করার জন্য `sort_by_key` ব্যবহার করি:

<Listing number="13-7" file-name="src/main.rs" caption="Width অনুযায়ী আয়তক্ষেত্রগুলোকে order করতে `sort_by_key` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

এই code print করে:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

`Sort_by_key`-কে একটি `FnMut` closure নেওয়ার জন্য define করার কারণ হল এটি closure-টিকে একাধিকবার call করে: slice-এর প্রতিটি item-এর জন্য একবার। Closure `|r| r.width` তার environment থেকে কোনো কিছু capture, mutate বা move করে না, তাই এটি trait bound requirement গুলো পূরণ করে।

বিপরীতে, Listing 13-8 একটি closure-এর উদাহরণ দেখায় যেটি শুধুমাত্র `FnOnce` trait implement করে, কারণ এটি environment থেকে একটি value move করে। Compiler আমাদের এই closure-টিকে `sort_by_key`-এর সাথে ব্যবহার করতে দেবে না:

<Listing number="13-8" file-name="src/main.rs" caption="`sort_by_key`-এর সাথে একটি `FnOnce` ক্লোজার ব্যবহার করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

`List` sort করার সময় `sort_by_key` closure-টিকে কতবার call করে তা count করার এটি একটি contrived, জটিল উপায় (যা কাজ করে না)। এই code টি `value`—closure-এর environment থেকে একটি `String`—`sort_operations` vector-এ push করে এই counting করার চেষ্টা করে। Closure টি `value` capture করে তারপর `value`-এর ownership `sort_operations` vector-এ transfer করে closure-এর বাইরে `value` move করে। এই closure-টিকে একবার call করা যেতে পারে; এটিকে দ্বিতীয়বার call করার চেষ্টা কাজ করবে না কারণ `value` আর environment-এ থাকবে না যাতে এটিকে আবার `sort_operations`-এ push করা যায়! অতএব, এই closure টি শুধুমাত্র `FnOnce` implement করে। যখন আমরা এই code টি compile করার চেষ্টা করি, তখন আমরা এই error পাই যে closure-টি `FnMut` implement করতে হবে বলে `value` কে closure-এর বাইরে move করা যাবে না:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

Error টি closure body-র সেই line-টির দিকে নির্দেশ করে যেটি environment-এর বাইরে `value` move করে। এটি ঠিক করার জন্য, আমাদের closure body পরিবর্তন করতে হবে যাতে এটি environment-এর বাইরে value move না করে। Closure-টি কতবার call করা হয়েছে তা count করার জন্য, environment-এ একটি counter রাখা এবং closure body-তে এর value increment করা এটি calculate করার আরও straightforward উপায়। Listing 13-9-এর closure-টি `sort_by_key`-এর সাথে কাজ করে কারণ এটি শুধুমাত্র `num_sort_operations` counter-এ একটি mutable reference capture করছে এবং তাই একাধিকবার call করা যেতে পারে:

<Listing number="13-9" file-name="src/main.rs" caption="`sort_by_key`-এর সাথে একটি `FnMut` ক্লোজার ব্যবহারের অনুমতি রয়েছে">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

Closure-গুলোর ব্যবহার করে এমন function বা type define বা ব্যবহার করার সময় `Fn` trait গুলো important। পরবর্তী section-এ, আমরা iterator নিয়ে আলোচনা করব। অনেক iterator method closure argument নেয়, তাই আমরা যখন continue করব তখন এই closure-এর details গুলো মনে রাখবেন!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
