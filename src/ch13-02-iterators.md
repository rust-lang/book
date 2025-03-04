## Iterator-এর সাহায্যে Item-গুলোর একটি Series-কে Process করা

Iterator pattern আপনাকে item-গুলোর একটি sequence-এর উপর পর্যায়ক্রমে কিছু task perform করার অনুমতি দেয়। একটি iterator প্রতিটি item-এর উপর iterate করার logic এবং sequence কখন শেষ হয়েছে তা নির্ধারণ করার জন্য responsible। আপনি যখন iterator ব্যবহার করেন, তখন আপনাকে সেই logic নিজে reimplement করতে হবে না।

Rust-এ, iterator-গুলো _lazy_, অর্থাৎ যতক্ষণ না আপনি iterator-কে consume করার জন্য method call করেন ততক্ষণ পর্যন্ত সেগুলোর কোনো effect নেই। উদাহরণস্বরূপ, Listing 13-10-এর code `Vec<T>`-তে defined `iter` method-টিকে call করে vector `v1`-এর item-গুলোর উপর একটি iterator create করে। এই code নিজে থেকে কোনো useful কাজ করে না।

<Listing number="13-10" file-name="src/main.rs" caption="একটি iterator তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

Iterator-টি `v1_iter` variable-এ store করা হয়। একবার আমরা একটি iterator create করার পরে, আমরা এটিকে বিভিন্ন উপায়ে ব্যবহার করতে পারি। Chapter 3-এর Listing 3-5-এ, আমরা একটি array-এর উপর একটি `for` loop ব্যবহার করে iterate করেছিলাম যাতে এর প্রতিটি item-এ কিছু code execute করা যায়। এর ভেতরে এটি implicitly একটি iterator create করে consume করেছিল, কিন্তু এখন পর্যন্ত আমরা ঠিক কীভাবে এটি কাজ করে তা এড়িয়ে গেছি।

Listing 13-11-এর উদাহরণে, আমরা iterator create করাকে `for` loop-এ iterator ব্যবহার করা থেকে আলাদা করি। যখন `v1_iter`-এর iterator ব্যবহার করে `for` loop-টি call করা হয়, তখন iterator-এর প্রতিটি element loop-এর একটি iteration-এ ব্যবহৃত হয়, যেটি প্রতিটি value print করে।

<Listing number="13-11" file-name="src/main.rs" caption="একটি `for` লুপে একটি iterator ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

যেসব language-এ তাদের standard library দ্বারা iterator provide করা হয় না, সেগুলোতে আপনি সম্ভবত index 0-তে একটি variable শুরু করে, একটি value পাওয়ার জন্য সেই variable-টি ব্যবহার করে vector-এ index করে, এবং loop-এ variable value-টি increment করে যতক্ষণ না এটি vector-এর মোট item সংখ্যার সমান হয়, এভাবে একই functionality লিখতেন।

Iterator-গুলো আপনার জন্য সেই সমস্ত logic handle করে, repetitive code কমিয়ে দেয় যেখানে আপনার ভুল হওয়ার সম্ভাবনা থাকতে পারে। Iterator-গুলো আপনাকে একই logic বিভিন্ন ধরনের sequence-এর সাথে ব্যবহার করার flexibility দেয়, শুধুমাত্র vector-এর মতো data structure-গুলোতেই নয় যেগুলোতে আপনি index করতে পারেন। আসুন পরীক্ষা করি কীভাবে iterator-গুলো তা করে।

### `Iterator` Trait এবং `next` Method

সমস্ত iterator `Iterator` নামক একটি trait implement করে যা standard library-তে define করা হয়েছে। Trait-টির definition এইরকম:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // default implementation সহ method গুলো সরানো হয়েছে
}
```

লক্ষ্য করুন এই definition-এ কিছু new syntax ব্যবহার করা হয়েছে: `type Item` এবং `Self::Item`, যেগুলো এই trait-এর সাথে একটি _associated type_ define করছে। আমরা Chapter 20-এ associated type সম্পর্কে বিস্তারিত আলোচনা করব। আপাতত, আপনার যা জানা দরকার তা হল এই code বলছে যে `Iterator` trait implement করার জন্য আপনাকে একটি `Item` type-ও define করতে হবে, এবং এই `Item` type-টি `next` method-এর return type-এ ব্যবহৃত হয়। অন্য কথায়, `Item` type-টি হবে iterator থেকে returned type।

`Iterator` trait-টির implementor-দের শুধুমাত্র একটি method define করতে হয়: `next` method, যেটি iterator-এর একটি item প্রতিবারে `Some`-এ wrap করে return করে এবং iteration শেষ হয়ে গেলে `None` return করে।

আমরা সরাসরি iterator-গুলোতে `next` method call করতে পারি; Listing 13-12 প্রদর্শন করে vector থেকে create করা iterator-এ `next`-এর repeated call থেকে কী value return করা হয়।

<Listing number="13-12" file-name="src/lib.rs" caption="একটি iterator-এ `next` method কল করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

লক্ষ্য করুন যে আমাদের `v1_iter`-কে mutable করতে হয়েছিল: একটি iterator-এ `next` method call করা internal state পরিবর্তন করে যা iterator ব্যবহার করে track রাখে যে এটি sequence-এ কোথায় আছে। অন্য কথায়, এই code টি iterator-কে _consume_ করে, বা ব্যবহার করে। `Next`-এর প্রতিটি call iterator থেকে একটি item খেয়ে ফেলে। যখন আমরা একটি `for` loop ব্যবহার করি তখন আমাদের `v1_iter`-কে mutable করতে হয়নি কারণ loop টি `v1_iter`-এর ownership নিয়েছিল এবং এটিকে behind the scenes mutable করেছিল।

আরও লক্ষ্য করুন যে `next`-এ call থেকে আমরা যে value-গুলো পাই সেগুলো হল vector-এর value-গুলোর immutable reference। `Iter` method immutable reference-গুলোর উপর একটি iterator produce করে। যদি আমরা এমন একটি iterator create করতে চাই যেটি `v1`-এর ownership নেয় এবং owned value return করে, তাহলে আমরা `iter`-এর পরিবর্তে `into_iter` call করতে পারি। একইভাবে, যদি আমরা mutable reference-গুলোর উপর iterate করতে চাই, তাহলে আমরা `iter`-এর পরিবর্তে `iter_mut` call করতে পারি।

### যে Method-গুলো Iterator-কে Consume করে

`Iterator` trait-টিতে standard library দ্বারা provide করা default implementation সহ আরও বেশ কয়েকটি method রয়েছে; আপনি `Iterator` trait-এর জন্য standard library API documentation দেখে এই method গুলো সম্পর্কে জানতে পারেন। এই method-গুলোর মধ্যে কিছু তাদের definition-এ `next` method call করে, যে কারণে `Iterator` trait implement করার সময় আপনাকে `next` method implement করতে হয়।

যে method গুলো `next` কল করে তাদের _consuming adapter_ বলা হয়, কারণ সেগুলোকে call করলে iterator টি ব্যবহৃত হয়ে যায়। একটি উদাহরণ হল `sum` method, যেটি iterator-এর ownership নেয় এবং repeatedly `next` call করে item-গুলোর মধ্যে iterate করে, এইভাবে iterator-টিকে consume করে। এটি iterate করার সময়, এটি প্রতিটি item-কে একটি running total-এর সাথে যোগ করে এবং iteration সম্পূর্ণ হলে total টি return করে। Listing 13-13 `sum` method-এর ব্যবহারের চিত্র তুলে ধরে এমন একটি test ধারণ করে:

<Listing number="13-13" file-name="src/lib.rs" caption="Iterator-এর সমস্ত item-এর total পেতে `sum` method কল করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

`Sum`-এ call করার পরে আমরা `v1_iter` ব্যবহার করতে পারি না কারণ `sum` যে iterator-এ call করা হয় সেটির ownership নেয়।

### যে Method-গুলো অন্যান্য Iterator Produce করে

_Iterator adapter_ হল `Iterator` trait-এ defined method যেগুলো iterator-কে consume করে না। পরিবর্তে, সেগুলো original iterator-এর কিছু aspect পরিবর্তন করে different iterator produce করে।

Listing 13-14 iterator adapter method `map`-এ call করার একটি উদাহরণ দেখায়, যেটি item-গুলোর মধ্যে iterate করার সময় প্রতিটি item-এ call করার জন্য একটি closure নেয়। `Map` method টি একটি new iterator return করে যেটি modified item গুলো produce করে। এখানে closure টি একটি new iterator create করে যেখানে vector-এর প্রতিটি item-এর সাথে 1 যোগ করা হবে:

<Listing number="13-14" file-name="src/main.rs" caption="নতুন iterator তৈরি করতে iterator adapter `map` কল করা">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

তবে, এই code একটি warning produce করে:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

Listing 13-14-এর code কিছুই করে না; আমরা যে closure টি specify করেছি সেটি কখনও call করা হয় না। Warning টি আমাদের মনে করিয়ে দেয় কেন: iterator adapter গুলো lazy, এবং আমাদের এখানে iterator-টিকে consume করতে হবে।

এই warning টি ঠিক করতে এবং iterator-টিকে consume করতে, আমরা `collect` method টি ব্যবহার করব, যেটি আমরা Chapter 12-তে Listing 12-1-এ `env::args`-এর সাথে ব্যবহার করেছি। এই method টি iterator-টিকে consume করে এবং resulting value গুলোকে একটি collection data type-এ collect করে।

Listing 13-15-এ, আমরা `map`-এ call থেকে returned iterator-এর উপর iterate করার result গুলোকে একটি vector-এ collect করি। এই vector-টিতে original vector-এর প্রতিটি item-এর সাথে 1 যোগ করা থাকবে।

<Listing number="13-15" file-name="src/main.rs" caption="একটি নতুন iterator তৈরি করতে `map` মেথড কল করা এবং তারপর নতুন iterator টিকে consume করে একটি vector তৈরি করতে `collect` মেথড কল করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

যেহেতু `map` একটি closure নেয়, তাই আমরা প্রতিটি item-এ perform করতে চাই এমন যেকোনো operation specify করতে পারি। এটি একটি দুর্দান্ত উদাহরণ যে কীভাবে closure গুলো আপনাকে কিছু behavior customize করতে দেয়, সেইসাথে `Iterator` trait provide করা iteration behavior-টিকে reuse করতে দেয়।

আপনি readable উপায়ে complex action perform করার জন্য iterator adapter-এ multiple call chain করতে পারেন। কিন্তু যেহেতু সমস্ত iterator lazy, তাই আপনাকে iterator adapter-গুলোতে call থেকে result পেতে consuming adapter method গুলোর মধ্যে একটিকে call করতে হবে।

### তাদের Environment Capture করে এমন Closure ব্যবহার করা

অনেক iterator adapter argument হিসেবে closure নেয়, এবং commonly iterator adapter-গুলোতে argument হিসেবে আমরা যে closure গুলো specify করব সেগুলো হবে এমন closure যেগুলো তাদের environment capture করে।

এই উদাহরণের জন্য, আমরা `filter` method টি ব্যবহার করব যেটি একটি closure নেয়। Closure টি iterator থেকে একটি item পায় এবং একটি `bool` return করে। যদি closure টি `true` return করে, তাহলে value টি `filter` দ্বারা produced iteration-এ include করা হবে। যদি closure টি `false` return করে, তাহলে value টি include করা হবে না।

Listing 13-16-এ, আমরা `Shoe` struct instance-গুলোর একটি collection-এর উপর iterate করার জন্য `shoe_size` variable-টিকে তার environment থেকে capture করে এমন একটি closure-এর সাথে `filter` ব্যবহার করি। এটি শুধুমাত্র specified size-এর জুতাগুলো return করবে।

<Listing number="13-16" file-name="src/lib.rs" caption="`shoe_size` ক্যাপচার করে এমন একটি ক্লোজারের সাথে `filter` মেথড ব্যবহার করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

`Shoes_in_size` function টি parameter হিসেবে জুতাগুলোর একটি vector এবং একটি জুতার size-এর ownership নেয়। এটি specified size-এর শুধুমাত্র জুতাগুলো ধারণকারী একটি vector return করে।

`Shoes_in_size`-এর body-তে, আমরা vector-টির ownership নেয় এমন একটি iterator create করার জন্য `into_iter` call করি। তারপর আমরা সেই iterator-টিকে একটি new iterator-এ adapt করার জন্য `filter` call করি যেটিতে শুধুমাত্র সেই element গুলো থাকে যেগুলোর জন্য closure টি `true` return করে।

Closure টি environment থেকে `shoe_size` parameter টি capture করে এবং প্রতিটি জুতার size-এর সাথে value-টিকে compare করে, শুধুমাত্র specified size-এর জুতাগুলো রাখে। অবশেষে, `collect` call করা adapted iterator দ্বারা returned value গুলোকে gather করে function দ্বারা returned একটি vector-এ রাখে।

Test টি দেখায় যে যখন আমরা `shoes_in_size` call করি, তখন আমরা শুধুমাত্র সেই জুতাগুলো ফেরত পাই যেগুলোর size আমাদের specified value-এর সমান।
