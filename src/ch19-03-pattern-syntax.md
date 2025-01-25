## Refutability: Whether a Pattern Might Fail to Match

Pattern দুটি form এ আসে: refutable এবং irrefutable। যে pattern গুলো pass করা যেকোনো possible value এর জন্য match করবে সেগুলো হলো _irrefutable_। Example হিসেবে `let x = 5;` statement এ `x` বলা যায়, কারণ `x` যেকোনো কিছুর সাথে match করে এবং তাই match করতে fail করতে পারে না। যে pattern গুলো কিছু possible value এর জন্য match করতে fail করতে পারে সেগুলো হলো _refutable_। Example হিসেবে `if let Some(x) = a_value` expression এ `Some(x)` বলা যায়, কারণ যদি variable `a_value` তে থাকা value `Some` এর পরিবর্তে `None` হয়, তাহলে `Some(x)` pattern টি match করবে না।

Function parameter, `let` statement, এবং `for` loop শুধুমাত্র irrefutable pattern accept করতে পারে, কারণ যখন value match করে না তখন program কোনো meaningful কিছু করতে পারে না। `if let` এবং `while let` expression এবং `let`-`else` statement refutable এবং irrefutable pattern accept করে, কিন্তু compiler irrefutable pattern এর বিরুদ্ধে warn করে কারণ by definition তারা possible failure handle করার জন্য intended: একটি conditional এর functionality হলো success বা failure এর উপর depend করে differently perform করার ability।

সাধারণভাবে, refutable এবং irrefutable pattern এর মধ্যে পার্থক্য নিয়ে আপনার worry করার প্রয়োজন নেই; তবে, refutability এর concept নিয়ে familiar থাকার প্রয়োজন যাতে আপনি error message এ দেখলে respond করতে পারেন। সেই case গুলোতে, code এর intended behaviour এর উপর depend করে হয় pattern অথবা pattern এর সাথে ব্যবহার করা construct change করার প্রয়োজন হবে।

চলুন একটি উদাহরণ দেখি যেখানে আমরা refutable pattern ব্যবহার করার চেষ্টা করব যেখানে Rust এর irrefutable pattern এর প্রয়োজন এবং vice versa। Listing 19-8 একটি `let` statement দেখায়, কিন্তু pattern এর জন্য আমরা specified করেছি `Some(x)`, যা একটি refutable pattern। আপনি যেমনটা আশা করছেন, এই code compile হবে না।

<Listing number="19-8" caption="`let` এর সাথে refutable pattern ব্যবহার করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

যদি `some_option_value` একটি `None` value হতো, তাহলে এটি `Some(x)` pattern এর সাথে match করতে fail করত, মানে pattern টি refutable। তবে, `let` statement শুধুমাত্র irrefutable pattern accept করতে পারে কারণ `None` value এর সাথে code valid ভাবে করার মতো কিছুই নেই। Compile time এ, Rust complain করবে যে আমরা refutable pattern ব্যবহার করার চেষ্টা করেছি যেখানে irrefutable pattern require করা হয়েছে:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

যেহেতু আমরা `Some(x)` pattern দিয়ে সব valid value cover করিনি (এবং করতে পারতাম না!), Rust rightfully একটি compiler error produce করে।

যদি আমাদের এমন refutable pattern থাকে যেখানে একটি irrefutable pattern এর প্রয়োজন, তাহলে আমরা pattern ব্যবহার করা code change করে এটা fix করতে পারি: `let` ব্যবহার করার পরিবর্তে, আমরা `if let` ব্যবহার করতে পারি। তাহলে যদি pattern match না করে, তাহলে code শুধু curly bracket এর ভিতরের code skip করবে, valid ভাবে continue করার একটি সুযোগ পাবে। Listing 19-9 দেখায় কিভাবে Listing 19-8 এর code fix করতে হয়।

<Listing number="19-9" caption="`let` এর পরিবর্তে refutable pattern এর সাথে `if let` এবং একটি block ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

আমরা code কে out দিয়েছি! এই code এখন perfectly valid। তবে, যদি আমরা `if let` এ একটি irrefutable pattern দেই (একটি pattern যা সবসময় match করবে), যেমন `x`, যা Listing 19-10 এ দেখানো হয়েছে, তাহলে compiler একটি warning দেবে।

<Listing number="19-10" caption="`if let` এর সাথে irrefutable pattern ব্যবহার করার চেষ্টা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

Rust complain করে যে irrefutable pattern এর সাথে `if let` ব্যবহার করা কোনো sense করে না:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

এই কারণে, match arm এ refutable pattern ব্যবহার করা উচিত, last arm ছাড়া, যা irrefutable pattern দিয়ে remaining সব value match করবে। Rust আমাদের একটি arm থাকা `match` এ irrefutable pattern ব্যবহার করার allow করে, কিন্তু এই syntax particularly useful নয় এবং simple `let` statement দিয়ে replace করা যেতে পারে।

এখন যেহেতু আপনি pattern কোথায় ব্যবহার করতে হয় এবং refutable এবং irrefutable pattern এর মধ্যে পার্থক্য জানেন, তাই চলুন pattern তৈরি করার জন্য আমরা যে সব syntax ব্যবহার করতে পারি সেগুলো নিয়ে আলোচনা করি।

## Pattern Syntax

এই section এ, আমরা pattern এ valid সব syntax collect করব এবং আলোচনা করব কেন এবং কখন আপনার প্রত্যেকটি ব্যবহার করার প্রয়োজন হতে পারে।

### Matching Literals

Chapter 6 এ যেমন দেখেছেন, আপনি directly literal এর সাথে pattern match করতে পারেন। নিচের code কিছু উদাহরণ দেয়:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

এই code print করে `one` কারণ `x` এ value হলো 1. যখন আপনি কোনো particular concrete value পেলে আপনার code কে action নিতে চান তখন এই syntax useful।

### Matching Named Variables

Named variable হলো irrefutable pattern যা যেকোনো value এর সাথে match করে, এবং আমরা বইয়ে বহুবার এটি ব্যবহার করেছি। তবে, যখন আপনি `match`, `if let`, বা `while let` expression এ named variable ব্যবহার করেন, তখন একটি complication আছে। যেহেতু এই ধরনের প্রত্যেক expression একটি নতুন scope শুরু করে, তাই expression এর ভিতরে pattern এর অংশ হিসেবে declare করা variable গুলো বাইরে একই name এর variable গুলোকে shadow করবে, যেমনটা সব variable এর সাথে হয়। Listing 19-11 এ, আমরা `x` নামের একটি variable declare করি `Some(5)` value দিয়ে এবং `y` নামের একটি variable declare করি `10` value দিয়ে। তারপর আমরা value `x` এর উপর একটি `match` expression তৈরি করি। Match arm এর pattern এবং শেষ এ `println!` এর দিকে তাকান, এবং এই code run করার আগে বা আরও কিছু পড়ার আগে figure out করার চেষ্টা করুন code কি print করবে।

<Listing number="19-11" file-name="src/main.rs" caption="একটি `match` expression যার একটি arm একটি নতুন variable introduce করে যা existing variable `y` কে shadow করে">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

</Listing>

চলুন দেখা যাক যখন `match` expression run হয় তখন কি ঘটে। প্রথম match arm এর pattern `x` এর defined value এর সাথে match করে না, তাই code continue করে।

Second match arm এর pattern `y` নামে একটি নতুন variable introduce করে যা একটি `Some` value এর ভিতরের যেকোনো value এর সাথে match করবে। যেহেতু আমরা `match` expression এর ভিতরের নতুন scope এ আছি, তাই এটি একটি নতুন `y` variable, শুরুতে value 10 দিয়ে declare করা `y` নয়। এই নতুন `y` binding `Some` এর ভিতরের যেকোনো value এর সাথে match করবে, যা `x` এ আমাদের আছে। তাই, এই নতুন `y` `x` এ থাকা `Some` এর ভিতরের value এর সাথে bind হবে। সেই value টি হলো `5`, তাই সেই arm এর expression execute হয় এবং print করে `Matched, y = 5`।

যদি `x` `Some(5)` এর পরিবর্তে `None` value হতো, তাহলে প্রথম দুটি arm এর pattern match করত না, তাই value underscore এর সাথে match করত। আমরা underscore arm এর pattern এ `x` variable introduce করিনি, তাই expression এ `x` এখনও বাইরের `x` যা shadow করা হয়নি। এই hypothetical case এ, `match` print করত `Default case, x = None`।

যখন `match` expression done হয়, তখন এর scope শেষ হয়, এবং ভিতরের `y` এর scope ও শেষ হয়। Last `println!` produce করে `at the end: x = Some(5), y = 10`।

একটি `match` expression তৈরি করার জন্য যা existing `y` variable কে shadow না করে outer `x` এবং `y` এর value compare করে, আমাদের এর পরিবর্তে একটি match guard conditional ব্যবহার করার প্রয়োজন। আমরা এই chapter এর পরে [“Extra Conditionals with Match Guards”](#extra-conditionals-with-match-guards)<!-- ignore --> section এ match guard নিয়ে discuss করব।

### Multiple Patterns

আপনি `|` syntax ব্যবহার করে multiple pattern match করতে পারেন, যা pattern _or_ operator। উদাহরণস্বরূপ, নিচের code এ আমরা `x` এর value match arm এর সাথে match করি, যার প্রথমটিতে একটি _or_ option আছে, মানে যদি `x` এর value সেই arm এর যেকোনো value এর সাথে match করে, তাহলে সেই arm এর code run হবে:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

এই code print করে `one or two`।

### Matching Ranges of Values with `..=`

`..=` syntax আমাদের inclusive range এর value এর সাথে match করার allow করে। নিচের code এ, যখন একটি pattern given range এর ভিতরের যেকোনো value এর সাথে match করে, তখন সেই arm execute হবে:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

যদি `x` 1, 2, 3, 4, বা 5 হয়, তাহলে প্রথম arm match করবে। একই idea express করার জন্য `|` operator ব্যবহার করার চেয়ে multiple match value এর জন্য এই syntax অনেক বেশি convenient; যদি আমরা `|` ব্যবহার করতাম, তাহলে আমাদের specify করতে হতো `1 | 2 | 3 | 4 | 5`। একটি range specify করা অনেক short, বিশেষ করে যদি আমরা 1 থেকে 1,000 এর মধ্যে যেকোনো number match করতে চাই!

Compiler check করে যে compile time এ range empty নয়, এবং যেহেতু Rust শুধুমাত্র এমন type এর জন্য বলতে পারে range empty কিনা তা হলো `char` এবং numeric value, range শুধুমাত্র numeric বা `char` value এর সাথে allow করা হয়।

এখানে `char` value এর range ব্যবহার করার একটি উদাহরণ দেওয়া হলো:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Rust বলতে পারে যে `'c'` প্রথম pattern এর range এর ভিতরে আছে এবং print করে `early ASCII letter`।

### Destructuring to Break Apart Values

আমরা value এর different part ব্যবহার করার জন্য struct, enum, এবং tuple destructure করার জন্য pattern ব্যবহার করতে পারি। চলুন প্রত্যেক value আলোচনা করি।

#### Destructuring Structs

Listing 19-12 এ দুটি field, `x` এবং `y` সহ একটি `Point` struct দেখানো হয়েছে, যা আমরা `let` statement দিয়ে pattern ব্যবহার করে break করতে পারি।

<Listing number="19-12" file-name="src/main.rs" caption="Struct এর field গুলোকে separate variable এ destructure করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

</Listing>

এই code `a` এবং `b` variable তৈরি করে যা `p` struct এর `x` এবং `y` field এর value match করে। এই উদাহরণ দেখায় যে pattern এ variable এর name struct এর field এর name এর সাথে match করার প্রয়োজন নেই। তবে, কোন variable কোন field থেকে এসেছে তা মনে রাখাকে সহজ করার জন্য variable name এর সাথে field name match করা common। এই common usage এর কারণে, এবং `let Point { x: x, y: y } = p;` লেখায় অনেক duplication থাকার কারণে, Rust এ struct field match করা pattern এর জন্য একটি shorthand আছে: আপনার শুধুমাত্র struct field এর name list করার প্রয়োজন, এবং pattern থেকে তৈরি হওয়া variable গুলোর name same হবে। Listing 19-13 Listing 19-12 এর code এর মতোই behave করে, কিন্তু `let` pattern এ তৈরি হওয়া variable গুলো হলো `x` এবং `y` `a` এবং `b` এর পরিবর্তে।

<Listing number="19-13" file-name="src/main.rs" caption="struct field shorthand ব্যবহার করে struct field destructure করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

</Listing>

এই code variable `x` এবং `y` তৈরি করে যা `p` variable এর `x` এবং `y` field এর সাথে match করে। Outcome হলো variable `x` এবং `y` তে `p` struct এর value গুলো থাকে।

আমরা literal value দিয়েও destructure করতে পারি, struct এর pattern এর অংশ হিসেবে, সব field এর জন্য variable তৈরি করার পরিবর্তে। এটা করলে আমরা variable create করার সময় কিছু particular value এর জন্য কিছু field test করতে পারি।

Listing 19-14 এ, আমাদের একটি `match` expression আছে যা `Point` value গুলোকে তিনটি case এ separate করে: যে point গুলো সরাসরি `x` axis এ থাকে (যা true যখন `y = 0`), `y` axis এ (`x = 0`), বা কোনোটাই নয়।

<Listing number="19-14" file-name="src/main.rs" caption="একটি pattern এ literal value destructure এবং match করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

</Listing>

প্রথম arm যেকোনো point এর সাথে match করবে যা `x` axis এ আছে, specify করে যে `y` field match করে যদি এর value literal `0` এর সাথে match করে। Pattern এখনও একটি `x` variable তৈরি করে যা আমরা এই arm এর code এ ব্যবহার করতে পারি।

Similarly, দ্বিতীয় arm `y` axis এ থাকা যেকোনো point এর সাথে match করে specify করে যে `x` field match করে যদি এর value `0` হয় এবং `y` field এর value এর জন্য একটি variable `y` তৈরি করে। তৃতীয় arm কোনো literal specify করে না, তাই এটি অন্য যেকোনো `Point` এর সাথে match করে এবং `x` এবং `y` field উভয়ের জন্য variable তৈরি করে।

এই উদাহরণে, value `p` `x` এ 0 থাকার কারণে দ্বিতীয় arm এর সাথে match করে, তাই এই code print করবে `On the y axis at 7`।

মনে রাখবেন যে একটি `match` expression first matching pattern পাওয়ার সাথে সাথেই arm check করা stop করে, তাই `Point { x: 0, y: 0}` `x` axis এবং `y` axis এ থাকা সত্ত্বেও, এই code শুধুমাত্র print করবে `On the x axis at 0`।

#### Destructuring Enums

আমরা এই বইয়ে enum destructure করেছি (উদাহরণস্বরূপ, Chapter 6 এর Listing 6-5), কিন্তু এখনও explicitly discuss করিনি যে enum destructure করার pattern enum এর ভিতরে store করা data এর way এর সাথে correspond করে। উদাহরণস্বরূপ, Listing 19-15 এ আমরা Listing 6-2 থেকে `Message` enum ব্যবহার করি এবং এমন pattern এর সাথে একটি `match` লিখি যা প্রত্যেক inner value destructure করবে।

<Listing number="19-15" file-name="src/main.rs" caption="ভিন্ন ধরনের value hold করা enum variant destructure করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

</Listing>

এই code print করবে `Change the color to red 0, green 160, and blue 255`। অন্য arm থেকে code run করার জন্য `msg` এর value change করার চেষ্টা করুন।

Data ছাড়া enum variant এর জন্য, যেমন `Message::Quit`, আমরা value কে আর destructure করতে পারি না। আমরা শুধুমাত্র literal `Message::Quit` value এর উপর match করতে পারি, এবং সেই pattern এ কোনো variable নেই।

Struct-like enum variant এর জন্য, যেমন `Message::Move`, আমরা pattern ব্যবহার করতে পারি similar to struct match করার জন্য specify করা pattern এর মতো। Variant name এর পরে, আমরা curly bracket place করি এবং তারপর variable সহ field list করি যাতে আমরা code এ ব্যবহার করার জন্য part গুলো break করতে পারি। এখানে আমরা Listing 19-13 এ করার মতো shorthand form ব্যবহার করি।

Tuple-like enum variant এর জন্য, যেমন `Message::Write` যা একটি element এর সাথে tuple hold করে এবং `Message::ChangeColor` যা তিনটি element এর সাথে tuple hold করে, pattern tuple match করার জন্য specify করা pattern এর similar। Pattern এ variable এর number matching করা variant এ element এর number এর সাথে match করা উচিত।

#### Destructuring Nested Structs and Enums

এখন পর্যন্ত, আমাদের সব উদাহরণ struct বা enum এক level deep matching করছে, কিন্তু matching nested item এও কাজ করতে পারে! উদাহরণস্বরূপ, আমরা Listing 19-15 এর code refactor করতে পারি `ChangeColor` message এ RGB এবং HSV color support করার জন্য, যা Listing 19-16 এ দেখানো হয়েছে।

<Listing number="19-16" caption="Nested enum এ matching">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

</Listing>

`match` expression এ প্রথম arm এর pattern একটি `Message::ChangeColor` enum variant এর সাথে match করে যেখানে একটি `Color::Rgb` variant আছে; তারপর pattern তিনটি inner `i32` value এর সাথে bind করে। Second arm এর pattern ও `Message::ChangeColor` enum variant এর সাথে match করে, কিন্তু inner enum match করে `Color::Hsv` এর সাথে। আমরা একটি `match` expression এ এই complex condition গুলো specify করতে পারি, এমনকি যদি দুটি enum involve থাকেও।

#### Destructuring Structs and Tuples

আমরা আরও complex উপায়ে destructuring pattern mix, match, এবং nest করতে পারি। নিচের উদাহরণ একটি complicated destructure দেখায় যেখানে আমরা struct এবং tuple কে একটি tuple এর ভিতরে nest করি এবং সব primitive value destructure করি:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

এই code আমাদের complex type গুলোকে তাদের component part এ break করার সুযোগ দেয় যাতে আমরা interested value গুলো আলাদাভাবে ব্যবহার করতে পারি।

Pattern দিয়ে destructuring হলো value এর অংশগুলো ব্যবহার করার জন্য একটি convenient way, যেমন struct এর প্রত্যেক field থেকে value, একে অপরের থেকে আলাদাভাবে।

### Ignoring Values in a Pattern

আপনি দেখেছেন যে pattern এ value ignore করা মাঝে মাঝে useful, যেমন `match` এর last arm এ, catchall পাওয়ার জন্য যা আসলে কিছু করে না কিন্তু সব remaining possible value account করে। Pattern এ সম্পূর্ণ value বা value এর part ignore করার কয়েকটি উপায় আছে: `_` pattern ব্যবহার করে (যা আপনি দেখেছেন), অন্য pattern এর ভিতরে `_` pattern ব্যবহার করে, underscore দিয়ে শুরু হয় এমন name ব্যবহার করে, বা একটি value এর remaining part ignore করার জন্য `..` ব্যবহার করে। চলুন explore করি কিভাবে এবং কেন এই pattern গুলোর প্রত্যেকটি ব্যবহার করতে হয়।
