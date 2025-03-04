## Environment Variable-দের সাথে কাজ করা

আমরা `minigrep`-কে আরও উন্নত করব একটি extra feature যোগ করে: case-insensitive searching-এর জন্য একটি option যা user একটি environment variable-এর মাধ্যমে চালু করতে পারবে। আমরা এই feature-টিকে একটি command line option করতে পারতাম এবং users-দের প্রতিবার এটি apply করতে চাইলে সেটি enter করতে বলতে পারতাম, কিন্তু পরিবর্তে এটিকে একটি environment variable করে, আমরা আমাদের user-দের environment variable-টি একবার set করার অনুমতি দিই এবং সেই terminal session-এ তাদের সমস্ত search case-insensitive হয়।

### Case-Insensitive `search` Function-এর জন্য একটি Failing Test লেখা

আমরা প্রথমে একটি নতুন `search_case_insensitive` function যোগ করি যেটি environment variable-টির একটি value থাকলে call করা হবে। আমরা TDD process টি follow করা চালিয়ে যাব, তাই প্রথম step টি হল আবার একটি failing test লেখা। আমরা নতুন `search_case_insensitive` function-এর জন্য একটি নতুন test যোগ করব এবং আমাদের পুরানো test-এর নাম `one_result` থেকে `case_sensitive`-এ পরিবর্তন করব যাতে দুটি test-এর মধ্যে পার্থক্য স্পষ্ট হয়, যেমনটি Listing 12-20-তে দেখানো হয়েছে।

<Listing number="12-20" file-name="src/lib.rs" caption="আমরা যে case-insensitive function টি যোগ করতে যাচ্ছি তার জন্য একটি নতুন failing test যোগ করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

</Listing>

লক্ষ্য করুন যে আমরা পুরানো test-এর `contents`-ও edit করেছি। আমরা `"Duct tape."` text-সহ একটি নতুন line যোগ করেছি যেখানে একটি capital _D_ রয়েছে যা `"duct"` query-র সাথে match করা উচিত নয় যখন আমরা case-sensitive পদ্ধতিতে search করছি। এইভাবে পুরানো test টি পরিবর্তন করা নিশ্চিত করতে সাহায্য করে যে আমরা accidental ভাবে case-sensitive search functionality break করছি না যা আমরা ইতিমধ্যেই implement করেছি। এই test টি এখন pass করা উচিত এবং case-insensitive search-এ কাজ করার সময় এটি pass করতে থাকা উচিত।

Case-_insensitive_ search-এর জন্য নতুন test টি `"rUsT"` কে তার query হিসেবে ব্যবহার করে। আমরা যে `search_case_insensitive` function টি যোগ করতে যাচ্ছি, তাতে `"rUsT"` query-টি `"Rust:"`-যুক্ত line-টির সাথে match করা উচিত যেখানে একটি capital _R_ রয়েছে এবং `"Trust me."` line-টির সাথেও match করা উচিত, যদিও query-র থেকে দুটোতেই আলাদা casing রয়েছে। এটি আমাদের failing test, এবং এটি compile হতে fail করবে কারণ আমরা এখনও `search_case_insensitive` function টি define করিনি। Listing 12-16-এ `search` function-এর জন্য যেভাবে করেছিলাম, সেভাবে একটি skeleton implementation যোগ করতে পারেন যেটি সব সময় একটি empty vector return করে, যাতে test compile হয়ে fail করে।

### `search_case_insensitive` Function Implement করা

Listing 12-21-এ দেখানো `search_case_insensitive` function টি প্রায় `search` function-এর মতোই হবে। পার্থক্য শুধুমাত্র এই যে আমরা `query` এবং প্রতিটি `line`-কে lowercase করব যাতে input argument-গুলোর case যাই হোক না কেন, line-টিতে query আছে কিনা তা check করার সময় সেগুলি একই case-এর হবে।

<Listing number="12-21" file-name="src/lib.rs" caption="`query` এবং line-কে তুলনা করার আগে lowercase করার জন্য `search_case_insensitive` ফাংশনটিকে Define করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

</Listing>

প্রথমে আমরা `query` string-টিকে lowercase করি এবং এটিকে একই নামের একটি নতুন variable-এ store করি, original টিকে shadowing করে। Query-তে `to_lowercase` call করা প্রয়োজন যাতে user-এর query `"rust"`, `"RUST"`, `"Rust"`, বা `"rUsT"` যাই হোক না কেন, আমরা query-টিকে `"rust"` হিসেবে treat করব এবং case-এর প্রতি insensitive হব। যদিও `to_lowercase` basic Unicode handle করবে, এটি 100% accurate হবে না। যদি আমরা একটি real application লিখতাম, তাহলে আমাদের এখানে আরও কিছু কাজ করতে হত, কিন্তু এই section টি environment variable সম্পর্কে, Unicode সম্পর্কে নয়, তাই আমরা এটিকে এখানেই ছেড়ে দেব।

লক্ষ্য করুন যে `query` এখন একটি string slice-এর পরিবর্তে একটি `String`, কারণ `to_lowercase` call করা existing data-কে reference করার পরিবর্তে new data create করে। উদাহরণস্বরূপ, ধরা যাক query হল `"rUsT"`: সেই string slice-টিতে আমাদের ব্যবহার করার জন্য কোনো lowercase `u` বা `t` নেই, তাই আমাদের `"rust"` ধারণকারী একটি নতুন `String` allocate করতে হবে। আমরা যখন এখন `contains` method-এ argument হিসেবে `query` pass করি, তখন আমাদের একটি ampersand যোগ করতে হবে কারণ `contains`-এর signature একটি string slice নেওয়ার জন্য define করা হয়েছে।

এরপরে, আমরা প্রতিটি `line`-এ `to_lowercase`-এ একটি call যোগ করি সমস্ত character lowercase করার জন্য। এখন যেহেতু আমরা `line` এবং `query` কে lowercase-এ convert করেছি, তাই query-র case যাই হোক না কেন আমরা match খুঁজে পাব।

আসুন দেখি এই implementation টি test গুলো pass করে কিনা:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

দারুণ! সেগুলো pass করেছে। এখন, আসুন `run` function থেকে নতুন `search_case_insensitive` function টি call করি। প্রথমে আমরা `Config` struct-এ একটি configuration option যোগ করব case-sensitive এবং case-insensitive search-এর মধ্যে switch করার জন্য। এই field টি যোগ করলে compiler error হবে কারণ আমরা এখনও এই field টি কোথাও initialize করিনি:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:here}}
```

আমরা `ignore_case` field টি যোগ করেছি যেটিতে একটি Boolean রয়েছে। এরপরে, আমাদের `run` function-এর `ignore_case` field-এর value check করতে হবে এবং `search` function বা `search_case_insensitive` function call করতে হবে কিনা তা decide করতে সেটি ব্যবহার করতে হবে, যেমনটি Listing 12-22-তে দেখানো হয়েছে। এটি এখনও compile হবে না।

<Listing number="12-22" file-name="src/lib.rs" caption="`config.ignore_case`-এর value-র উপর ভিত্তি করে `search` বা `search_case_insensitive` Call করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:there}}
```

</Listing>

অবশেষে, আমাদের environment variable-টি check করতে হবে। Environment variable-গুলোর সাথে কাজ করার function গুলো standard library-এর `env` module-এ রয়েছে, তাই আমরা _src/lib.rs_-এর উপরের দিকে সেই module-টিকে scope-এ আনি। তারপর আমরা `env` module থেকে `var` function টি ব্যবহার করব এটা দেখতে যে `IGNORE_CASE` নামের একটি environment variable-এর জন্য কোনো value set করা হয়েছে কিনা, যেমনটি Listing 12-23-এ দেখানো হয়েছে।

<Listing number="12-23" file-name="src/lib.rs" caption="`IGNORE_CASE` নামের একটি environment variable-এ কোনো value আছে কিনা তা Check করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/lib.rs:here}}
```

</Listing>

এখানে, আমরা একটি নতুন variable, `ignore_case` তৈরি করি। এর value set করার জন্য, আমরা `env::var` function call করি এবং এতে `IGNORE_CASE` environment variable-এর নাম pass করি। `env::var` function একটি `Result` return করে যেটি successful `Ok` variant হবে যাতে environment variable-টির value থাকবে যদি environment variable-টি কোনো value-তে set করা থাকে। Environment variable-টি set করা না থাকলে এটি `Err` variant return করবে।

আমরা `Result`-এর উপর `is_ok` method টি ব্যবহার করছি এটা check করার জন্য যে environment variable-টি set করা আছে কিনা, যার অর্থ হল প্রোগ্রামটির case-insensitive search করা উচিত। যদি `IGNORE_CASE` environment variable-টি কোনো কিছুতে set করা না থাকে, তাহলে `is_ok`, `false` return করবে এবং প্রোগ্রামটি case-sensitive search করবে। Environment variable-টির _value_ নিয়ে আমাদের মাথা ঘামানোর দরকার নেই, শুধুমাত্র এটি set করা আছে নাকি unset, তাই আমরা `unwrap`, `expect`, বা `Result`-এ দেখা অন্যান্য method-গুলোর পরিবর্তে `is_ok` check করছি।

আমরা `ignore_case` variable-এর value-টি `Config` instance-এ pass করি যাতে `run` function সেই value টি read করতে পারে এবং Listing 12-22-এ implement করা `search_case_insensitive` বা `search` call করতে হবে কিনা তা decide করতে পারে।

আসুন চেষ্টা করে দেখা যাক! প্রথমে আমরা environment variable set না করে এবং `to` query দিয়ে আমাদের প্রোগ্রামটি run করব, যেটি lowercase-এ _to_ শব্দযুক্ত যেকোনো line-এর সাথে match করা উচিত:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

মনে হচ্ছে এটা এখনও কাজ করছে! এখন আসুন `IGNORE_CASE` কে `1`-এ set করে কিন্তু একই query _to_ দিয়ে প্রোগ্রামটি run করি:

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

আপনি যদি PowerShell ব্যবহার করেন, তাহলে আপনাকে environment variable set করতে হবে এবং প্রোগ্রামটিকে আলাদা command হিসেবে run করতে হবে:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

এটি আপনার shell session-এর বাকি অংশের জন্য `IGNORE_CASE`-কে স্থায়ী করবে। এটিকে `Remove-Item` cmdlet দিয়ে unset করা যেতে পারে:

```console
PS> Remove-Item Env:IGNORE_CASE
```

আমাদের _to_ যুক্ত line গুলো পাওয়া উচিত যেগুলিতে uppercase letter থাকতে পারে:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- to poem.txt
can't extract because of the environment variable
-->

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

চমৎকার, আমরা _To_ যুক্ত line-ও পেয়েছি! আমাদের `minigrep` প্রোগ্রামটি এখন একটি environment variable দ্বারা নিয়ন্ত্রিত case-insensitive searching করতে পারে। এখন আপনি জানেন কীভাবে command line argument বা environment variable ব্যবহার করে set করা option গুলো manage করতে হয়।

কিছু প্রোগ্রাম একই configuration-এর জন্য argument _এবং_ environment variable উভয়কেই অনুমতি দেয়। সেই ক্ষেত্রগুলোতে, প্রোগ্রামগুলো decide করে যে কোনটি প্রাধান্য পাবে। নিজে থেকে আরেকটি exercise-এর জন্য, একটি command line argument বা একটি environment variable-এর মাধ্যমে case sensitivity নিয়ন্ত্রণ করার চেষ্টা করুন। প্রোগ্রামটি case sensitive-এ set করা একটি এবং ignore case-এ set করা একটি দিয়ে run করা হলে command line argument বা environment variable-এর মধ্যে কোনটি প্রাধান্য পাওয়া উচিত তা ঠিক করুন।

`std::env` module-টিতে environment variable-গুলোর সাথে কাজ করার জন্য আরও অনেক useful feature রয়েছে: কী কী available তা দেখতে এর documentation দেখুন।
