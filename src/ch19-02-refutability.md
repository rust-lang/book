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
