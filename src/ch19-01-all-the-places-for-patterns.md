## All the Places Patterns Can Be Used

Rust এ pattern অনেক জায়গায় pop up হয়, এবং আপনি realize না করেই সেগুলো অনেক ব্যবহার করছেন! এই section এ pattern valid এমন সব জায়গা নিয়ে আলোচনা করা হয়েছে।

### `match` Arms

Chapter 6 এ discuss করা অনুযায়ী, আমরা `match` expression এর arm এ pattern ব্যবহার করি। Formally, `match` expression keyword `match`, match করার জন্য একটি value, এবং এক বা একাধিক match arm যা একটি pattern এবং সেই arm এর pattern এর সাথে value match করলে run করার জন্য expression নিয়ে গঠিত, যেমন এইরকম:

```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

উদাহরণস্বরূপ, এখানে Listing 6-5 থেকে `match` expression দেওয়া হলো যা variable `x` এ থাকা একটি `Option<i32>` value এর উপর match করে:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

এই `match` expression এর pattern গুলো হলো arrow এর বাম দিকে থাকা `None` এবং `Some(i)`।

`match` expression এর একটি requirement হলো এটিকে _exhaustive_ হতে হয় যার মানে `match` expression এর value এর জন্য সব possibility account করা উচিত। সব possibility cover করা হয়েছে কিনা তা নিশ্চিত করার একটি উপায় হলো last arm এর জন্য একটি catchall pattern থাকা: উদাহরণস্বরূপ, যেকোনো value match করা variable name কখনো fail হতে পারে না এবং এভাবে remaining সব case cover করে।

Particular pattern `_` যেকোনো কিছুর সাথে match করবে, কিন্তু এটি কোনো variable এর সাথে bind হয় না, তাই এটি প্রায়ই last match arm এ ব্যবহার করা হয়। যখন আপনি specified নয় এমন যেকোনো value ignore করতে চান তখন `_` pattern useful হতে পারে, উদাহরণস্বরূপ। আমরা এই chapter এর পরে [“Ignoring Values in a Pattern”][ignoring-values-in-a-pattern]<!-- ignore --> section এ `_` pattern নিয়ে আরও বিস্তারিত আলোচনা করব।

### Conditional `if let` Expressions

Chapter 6 এ আমরা discuss করেছিলাম কিভাবে `if let` expression ব্যবহার করতে হয় মূলত একটি `match` এর equivalent লেখার একটি short way হিসেবে যা শুধুমাত্র একটি case match করে। Optionally, `if let` এর একটি corresponding `else` থাকতে পারে যেখানে `if let` এর pattern match না করলে run করার code থাকে।

Listing 19-1 এ দেখানো হয়েছে যে `if let`, `else if`, এবং `else if let` expression mix এবং match করাও possible। এটা করলে আমরা `match` expression এর চেয়ে বেশি flexibility পাই যেখানে pattern এর সাথে compare করার জন্য আমরা শুধুমাত্র একটি value express করতে পারি। এছাড়াও, Rust এ `if let`, `else if`, `else if let` arms এর series এর condition গুলো একে অপরের সাথে related হতে হয় এমন কোনো requirement নেই।

Listing 19-1 এ code determine করে যে আপনার background এর color কি হবে কিছু condition এর জন্য check করার একটি series এর উপর ভিত্তি করে। এই উদাহরণের জন্য, আমরা hardcoded value দিয়ে variable তৈরি করেছি যা একটি real program user input থেকে receive করতে পারত।

<Listing number="19-1" file-name="src/main.rs" caption="Mixing `if let`, `else if`, `else if let`, and `else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs}}
```

</Listing>

যদি user কোনো favorite color specify করে, তাহলে সেই color background হিসেবে ব্যবহার করা হয়। যদি কোনো favorite color specify করা না হয় এবং আজ মঙ্গলবার হয়, তাহলে background color green হবে। অন্যথায়, যদি user string হিসেবে তাদের age specify করে এবং আমরা যদি এটিকে successfully number হিসেবে parse করতে পারি, তাহলে number এর value এর উপর depend করে color হয় purple বা orange হবে। যদি এই condition গুলোর কোনটি apply না হয়, তাহলে background color blue হবে।

এই conditional structure আমাদের complex requirement support করার সুযোগ দেয়। এখানে আমাদের hardcoded value এর সাথে, এই উদাহরণটি print করবে `Using purple as the background color`।

আপনি দেখতে পাচ্ছেন যে `if let` ও নতুন variable introduce করতে পারে যা `match` arm এর মতো একই ভাবে existing variable কে shadow করে: `if let Ok(age) = age` line এ একটি নতুন `age` variable introduce করা হয়েছে যেখানে `Ok` variant এর ভিতরের value আছে, যা existing `age` variable কে shadow করে। এর মানে হলো আমাদের `if age > 30` condition সেই block এর ভিতরে রাখতে হবে: আমরা `if let Ok(age) = age && age > 30` এর মধ্যে এই condition দুটোকে combine করতে পারি না। নতুন `age` যা আমরা 30 এর সাথে compare করতে চাই সেটি curly bracket দিয়ে শুরু হওয়া নতুন scope শুরু না হওয়া পর্যন্ত valid নয়।

`if let` expression ব্যবহার করার downside হলো compiler exhaustiveness check করে না, যেখানে `match` expression এর সাথে করে। যদি আমরা last `else` block omit করি এবং তাই কিছু case handle করতে miss করি, তাহলে compiler আমাদের possible logic bug নিয়ে alert করবে না।

### `while let` Conditional Loops

`if let` এর মতো construction এ similar, `while let` conditional loop একটি `while` loop কে run করার allow করে যতক্ষণ একটি pattern match করতে থাকে। আমরা প্রথমে Chapter 17 এ `while let` loop দেখেছিলাম, যেখানে stream নতুন value produce করা পর্যন্ত loop continue রাখার জন্য এটি ব্যবহার করেছিলাম। একইভাবে, Listing 19-2 এ আমরা একটি `while let` loop দেখাই যা thread এর মধ্যে send হওয়া message এর জন্য wait করে, কিন্তু এই ক্ষেত্রে `Option` এর পরিবর্তে একটি `Result` check করে।

<Listing number="19-2" caption="`rx.recv()` যতক্ষণ `Ok` return করে ততক্ষণ value print করার জন্য একটি `while let` loop ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

এই উদাহরণটি 1, 2, এবং 3 print করে। যখন আমরা Chapter 16 এ `recv` দেখেছিলাম, তখন আমরা directly error unwrap করেছিলাম, বা `for` loop ব্যবহার করে iterator হিসেবে interact করেছিলাম। Listing 19-2 এ দেখানো হিসাবে, আমরা `while let` ও ব্যবহার করতে পারি, কারণ `recv` method যতক্ষণ sender message produce করে ততক্ষণ `Ok` return করে, এবং sender side disconnect হওয়ার পর একটি `Err` produce করে।

### `for` Loops

একটি `for` loop এ, `for` keyword এর directly পরে থাকা value হলো একটি pattern। উদাহরণস্বরূপ, `for x in y` তে `x` হলো pattern। Listing 19-3 এ দেখিয়েছে কিভাবে `for` loop এ pattern ব্যবহার করে destructure করতে হয়, বা `for` loop এর অংশ হিসেবে একটি tuple break করতে হয়।

<Listing number="19-3" caption="একটি tuple destructure করার জন্য `for` loop এ একটি pattern ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs:here}}
```

</Listing>

Listing 19-3 এর code টি নিচের output print করবে:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-03/output.txt}}
```

আমরা `enumerate` method ব্যবহার করে একটি iterator adapt করি যাতে এটি একটি value এবং সেই value এর index produce করে, একটি tuple এ place করা। Produced প্রথম value হলো tuple `(0, 'a')`। যখন এই value pattern `(index, value)` এর সাথে match হয়, তখন `index` হবে `0` এবং `value` হবে `'a'`, output এর প্রথম লাইন print করে।

### `let` Statements

এই chapter এর আগে, আমরা শুধুমাত্র `match` এবং `if let` এর সাথে pattern ব্যবহার করা explicitly discuss করেছিলাম, কিন্তু আসলে, আমরা `let` statement সহ অন্য জায়গায়ও pattern ব্যবহার করেছি। উদাহরণস্বরূপ, `let` দিয়ে এই straightforward variable assignment টি consider করুন:

```rust
let x = 5;
```

যখনই আপনি এই ধরনের `let` statement ব্যবহার করেছেন, তখনই আপনি pattern ব্যবহার করেছেন, যদিও আপনি হয়তো realize করেন নি! আরও formally, একটি `let` statement দেখতে এমন:

```text
let PATTERN = EXPRESSION;
```

`let x = 5;` এর মতো statement এ `PATTERN` slot এ একটি variable name এর সাথে, variable name হলো pattern এর একটি particularly simple form। Rust pattern এর সাথে expression compare করে এবং এটি পাওয়া যেকোনো name assign করে। তাই `let x = 5;` উদাহরণে, `x` হলো একটি pattern যার মানে হলো "এখানে যা match করে তাকে variable `x` এর সাথে bind করো"। যেহেতু name `x` পুরো pattern, তাই এই pattern effectively মানে হলো "যা value আছে, সবকিছু `x` variable এ bind করো"।

`let` এর pattern matching aspect আরও clear ভাবে দেখার জন্য, Listing 19-4 consider করুন, যা tuple destructure করার জন্য `let` এর সাথে একটি pattern ব্যবহার করে।

<Listing number="19-4" caption="একটি tuple destructure করার জন্য একটি pattern ব্যবহার করা এবং একসাথে তিনটি variable তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

এখানে, আমরা একটি pattern এর সাথে একটি tuple match করি। Rust value `(1, 2, 3)` কে pattern `(x, y, z)` এর সাথে compare করে এবং দেখে যে value pattern এর সাথে match করে, তাই Rust `1` কে `x` এর সাথে, `2` কে `y` এর সাথে, এবং `3` কে `z` এর সাথে bind করে। আপনি এই tuple pattern কে এর ভিতরে nesting করা তিনটি individual variable pattern হিসেবে ভাবতে পারেন।

যদি pattern এর element এর number tuple এর element এর number এর সাথে match না করে, তাহলে overall type match করবে না এবং আমরা একটি compiler error পাব। উদাহরণস্বরূপ, Listing 19-5 এ তিনটি element থাকা একটি tuple কে দুটি variable এ destructure করার attempt দেখানো হয়েছে, যা কাজ করবে না।

<Listing number="19-5" caption="Incorrectly একটি pattern construct করা যার variable tuple এর element এর number এর সাথে match করে না">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

এই code compile করার attempt করলে এই type error টি result হবে:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

Error fix করার জন্য, আমরা tuple এর এক বা একাধিক value `_` বা `..` ব্যবহার করে ignore করতে পারতাম, যা আপনি [“Ignoring Values in a Pattern”][ignoring-values-in-a-pattern]<!-- ignore --> section এ দেখবেন। যদি problem হয় যে pattern এ আমাদের অনেক variable আছে, তাহলে solution হলো variable remove করে type গুলো match করানো যাতে variable এর number tuple এর element এর number এর সমান হয়।

### Function Parameters

Function parameter ও pattern হতে পারে। Listing 19-6 এর code, যা `foo` নামে একটি function declare করে যা `i32` type এর `x` নামের একটি parameter নেয়, তা এখন আপনার পরিচিত লাগা উচিত।

<Listing number="19-6" caption="একটি function signature parameter এ pattern ব্যবহার করে">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

`x` part টি একটি pattern! `let` এর মতো করে, আমরা function এর argument এ একটি pattern এর সাথে একটি tuple match করতে পারতাম। Listing 19-7 একটি function এ pass করার সময় একটি tuple এর value split করে।

<Listing number="19-7" file-name="src/main.rs" caption="Parameter সহ একটি function যা একটি tuple destructure করে">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

এই code print করে `Current location: (3, 5)`। Value `&(3, 5)` pattern `&(x, y)` এর সাথে match করে, তাই `x` হলো value `3` এবং `y` হলো value `5`।

Closure parameter list এ ও আমরা pattern ব্যবহার করতে পারি, function parameter list এ করার মতোই, কারণ closure গুলো function এর similar, যেমন Chapter 13 এ discuss করা হয়েছে।

এই মুহূর্তে, আপনি pattern ব্যবহার করার কয়েকটি উপায় দেখেছেন, কিন্তু pattern আমরা যেখানে ব্যবহার করতে পারি সব জায়গায় same ভাবে কাজ করে না। কিছু জায়গায়, pattern গুলো irrefutable হওয়া প্রয়োজন; অন্য circumstances এ, তারা refutable হতে পারে। আমরা এরপর এই দুটি concept নিয়ে discuss করব।

[ignoring-values-in-a-pattern]: ch19-03-pattern-syntax.html#ignoring-values-in-a-pattern
