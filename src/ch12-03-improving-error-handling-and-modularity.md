## Modularity এবং Error Handling উন্নত করার জন্য Refactoring

আমাদের প্রোগ্রামটিকে উন্নত করার জন্য, আমরা চারটি সমস্যা সমাধান করব যেগুলির প্রোগ্রামের structure এবং এটি কীভাবে potential error গুলি handle করছে তার সাথে সম্পর্ক রয়েছে। প্রথমত, আমাদের `main` function এখন দুটি কাজ করে: এটি argument parse করে এবং file read করে। আমাদের প্রোগ্রাম যত বাড়বে, `main` function-এর handle করা আলাদা কাজের সংখ্যাও বাড়বে। একটি function-এর responsibility যত বাড়ে, সেটি সম্পর্কে reasoning করা, test করা এবং সেটির কোনো একটি অংশ break না করে পরিবর্তন করা তত কঠিন হয়ে পড়ে। Functionality আলাদা করা সবচেয়ে ভালো যাতে প্রতিটি function একটি কাজের জন্য responsible হয়।

এই সমস্যাটি দ্বিতীয় সমস্যার সাথেও জড়িত: যদিও `query` এবং `file_path` আমাদের প্রোগ্রামের configuration variable, `contents`-এর মতো variable গুলো প্রোগ্রামের logic perform করার জন্য ব্যবহৃত হয়। `main` যত দীর্ঘ হবে, তত বেশি variable আমাদের scope-এ আনতে হবে; আমাদের scope-এ যত বেশি variable থাকবে, প্রতিটির purpose ট্র্যাক রাখা তত কঠিন হবে। Configuration variable গুলোকে একটি structure-এ group করা সবচেয়ে ভালো যাতে তাদের purpose স্পষ্ট হয়।

তৃতীয় সমস্যাটি হল, file read fail করলে আমরা একটি error message print করার জন্য `expect` ব্যবহার করেছি, কিন্তু error message টি শুধুমাত্র `Should have been able to read the file` প্রিন্ট করে। File read করার ক্ষেত্রে বিভিন্নভাবে fail হতে পারে: উদাহরণস্বরূপ, file টি missing থাকতে পারে, অথবা আমাদের এটি open করার permission নাও থাকতে পারে। এখন, পরিস্থিতি যাই হোক না কেন, আমরা সবকিছুর জন্য একই error message প্রিন্ট করব, যা user-কে কোনো information দেবে না!

চতুর্থত, আমরা একটি error handle করার জন্য `expect` ব্যবহার করি, এবং যদি user পর্যাপ্ত argument specify না করে আমাদের প্রোগ্রাম run করে, তাহলে তারা Rust-এর কাছ থেকে একটি `index out of bounds` error পাবে যা সমস্যাটি স্পষ্ট ভাবে ব্যাখ্যা করে না। Error-handling code-গুলো এক জায়গায় থাকলে সবচেয়ে ভালো হবে, যাতে future-এ maintainer-দের error-handling logic পরিবর্তন করার প্রয়োজন হলে code-এর শুধুমাত্র একটি জায়গাতেই consult করতে হয়। সমস্ত error-handling code এক জায়গায় থাকলে এটাও নিশ্চিত হবে যে আমরা এমন message প্রিন্ট করছি যা আমাদের end user-দের কাছে অর্থপূর্ণ হবে।

আসুন আমাদের project refactor করে এই চারটি সমস্যার সমাধান করি।

### বাইনারি প্রোজেক্টের জন্য Separation of Concerns

`main` function-এ একাধিক কাজের responsibility allocate করার সাংগঠনিক সমস্যাটি অনেক binary project-এর ক্ষেত্রে common। ফলস্বরূপ, Rust community একটি binary program-এর আলাদা concern গুলোকে split করার জন্য guidelines develop করেছে, যখন `main` বড় হতে শুরু করে। এই process-টিতে নিম্নলিখিত step গুলো রয়েছে:

-   আপনার প্রোগ্রামকে একটি _main.rs_ file এবং একটি _lib.rs_ file-এ split করুন এবং আপনার প্রোগ্রামের logic-কে _lib.rs_-এ move করুন।
-   যতক্ষণ আপনার command line parsing logic ছোট থাকে, ততক্ষণ এটি _main.rs_-এ থাকতে পারে।
-   যখন command line parsing logic জটিল হতে শুরু করে, তখন এটিকে _main.rs_ থেকে extract করুন এবং _lib.rs_-এ move করুন।

এই process-এর পরে `main` function-এ যে responsibility গুলো থাকা উচিত সেগুলো নিম্নলিখিতগুলির মধ্যে limited হওয়া উচিত:

-   Argument value-গুলো দিয়ে command line parsing logic call করা
-   অন্যান্য configuration set up করা
-   _lib.rs_-এ একটি `run` function call করা
-   যদি `run` কোনো error return করে তাহলে error handle করা

এই pattern-টি concern গুলোকে আলাদা করার বিষয়ে: _main.rs_ প্রোগ্রাম run করা handle করে এবং _lib.rs_ বর্তমান task-এর সমস্ত logic handle করে। যেহেতু আপনি সরাসরি `main` function test করতে পারবেন না, তাই এই structure আপনাকে _lib.rs_-এর function গুলোতে move করে আপনার প্রোগ্রামের সমস্ত logic test করতে দেয়। _main.rs_-এ যে code অবশিষ্ট থাকে তা এতটাই ছোট হবে যে এটি পড়ে এর সঠিকতা verify করা যাবে। আসুন এই process টি follow করে আমাদের প্রোগ্রামটিকে পুনরায় কাজ করি।

#### Argument Parser-কে Extract করা

আমরা argument parse করার functionality-টিকে একটি function-এ extract করব যাকে `main` call করবে command line parsing logic-কে _src/lib.rs_-এ move করার জন্য প্রস্তুত করতে। Listing 12-5 `main`-এর নতুন start দেখায় যা `parse_config` নামক একটি নতুন function call করে, যেটি আমরা আপাতত _src/main.rs_-এ define করব।

<Listing number="12-5" file-name="src/main.rs" caption="`main` থেকে একটি `parse_config` ফাংশন Extract করা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

আমরা এখনও command line argument-গুলোকে একটি vector-এ collect করছি, কিন্তু `main` function-এর মধ্যে index 1-এ থাকা argument value-টিকে `query` variable-এ এবং index 2-এ থাকা argument value-টিকে `file_path` variable-এ assign করার পরিবর্তে, আমরা পুরো vector-টিকে `parse_config` function-এ pass করি। `parse_config` function-টিতে এরপর সেই logic থাকে যা নির্ধারণ করে কোন argument কোন variable-এ যাবে এবং value-গুলো `main`-এ ফেরত পাঠায়। আমরা এখনও `main`-এ `query` এবং `file_path` variable create করি, কিন্তু command line argument এবং variable গুলো কীভাবে correspond করে তা নির্ধারণ করার responsibility আর `main`-এর নেই।

আমাদের ছোট প্রোগ্রামের জন্য এই rework টি overkill মনে হতে পারে, কিন্তু আমরা ছোট, incremental step-এ refactor করছি। এই পরিবর্তনটি করার পরে, argument parsing এখনও কাজ করছে কিনা তা verify করার জন্য প্রোগ্রামটি আবার run করুন। আপনার progress প্রায়শই check করা ভালো, যাতে সমস্যা দেখা দিলে তার কারণ সনাক্ত করতে সুবিধা হয়।

#### Configuration Value গুলো Grouping করা

আমরা `parse_config` function-টিকে আরও উন্নত করার জন্য আরেকটি ছোট step নিতে পারি। এখন, আমরা একটি tuple return করছি, কিন্তু তারপরে আমরা সেই tuple-টিকে আবার individual part-এ ভেঙে দিচ্ছি। এটি একটি লক্ষণ যে সম্ভবত আমাদের এখনও সঠিক abstraction নেই।

আরেকটি indicator যা দেখায় যে উন্নতির জায়গা রয়েছে তা হল `parse_config`-এর `config` অংশটি, যা বোঝায় যে আমরা যে দুটি value return করি সেগুলি related এবং উভয়ই একটি configuration value-এর অংশ। আমরা বর্তমানে data-র structure-এ এই অর্থটি প্রকাশ করছি না, শুধুমাত্র দুটি value-কে একটি tuple-এ group করে; এর পরিবর্তে আমরা দুটি value-কে একটি struct-এ রাখব এবং struct-এর প্রতিটি field-কে একটি অর্থপূর্ণ নাম দেব। এটি করলে future-এ এই code-এর maintainer-দের জন্য এটা বোঝা সহজ হবে যে কীভাবে বিভিন্ন value একে অপরের সাথে related এবং তাদের purpose কী।

Listing 12-6 `parse_config` function-এর improvement গুলো দেখায়।

<Listing number="12-6" file-name="src/main.rs" caption="`parse_config` কে একটি `Config` struct এর instance return করার জন্য Refactor করা">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

আমরা `Config` নামের একটি struct যোগ করেছি যার field-গুলোর নাম `query` এবং `file_path` হিসেবে define করা হয়েছে। `parse_config`-এর signature এখন নির্দেশ করে যে এটি একটি `Config` value return করে। `parse_config`-এর body-তে, যেখানে আমরা `args`-এ `String` value-গুলোকে reference করা string slice return করতাম, সেখানে এখন আমরা `Config`-কে define করি owned `String` value ধারণ করার জন্য। `main`-এর `args` variable-টি argument value-গুলোর owner এবং `parse_config` function-কে শুধুমাত্র সেগুলো borrow করতে দিচ্ছে, যার অর্থ হল যদি `Config`, `args`-এর value-গুলোর ownership নেওয়ার চেষ্টা করত তাহলে আমরা Rust-এর borrowing rule violate করতাম।

আমরা `String` data manage করার জন্য বেশ কয়েকটি উপায় অবলম্বন করতে পারি; সবচেয়ে সহজ, যদিও কিছুটা inefficient, উপায় হল value-গুলোর উপর `clone` method call করা। এটি `Config` instance-এর own করার জন্য data-র একটি full copy তৈরি করবে, যা string data-র একটি reference store করার চেয়ে বেশি সময় এবং memory নেয়। যাইহোক, data clone করা আমাদের code-কে আরও straightforward করে তোলে কারণ আমাদের reference-গুলোর lifetime manage করতে হয় না; এই পরিস্থিতিতে, simplicity অর্জনের জন্য performance-এর সামান্য ত্যাগ একটি worthwhile trade-off।

> ### `clone` ব্যবহারের Trade-Off
>
> অনেক Rustaceans-দের মধ্যে ownership-এর সমস্যা সমাধানের জন্য `clone` ব্যবহার করা এড়িয়ে যাওয়ার প্রবণতা রয়েছে কারণ এর runtime cost আছে।
> [Chapter 13][ch13]-এ, আপনি শিখবেন কীভাবে এই ধরনের পরিস্থিতিতে আরও efficient method ব্যবহার করতে হয়। কিন্তু আপাতত, progress চালিয়ে যাওয়ার জন্য কয়েকটি string copy করা ঠিক আছে কারণ আপনি এই copy গুলো শুধুমাত্র একবার করবেন এবং আপনার file path এবং query string খুব ছোট। প্রথম চেষ্টাতেই code hyperoptimize করার চেষ্টা করার চেয়ে একটি working প্রোগ্রাম থাকা ভালো যা কিছুটা inefficient। আপনি Rust-এর সাথে আরও experienced হওয়ার সাথে সাথে, সবচেয়ে efficient solution দিয়ে শুরু করা আরও সহজ হবে, কিন্তু আপাতত, `clone` call করা perfectly acceptable।

আমরা `main` update করেছি যাতে এটি `parse_config` দ্বারা returned `Config`-এর instance-টিকে `config` নামের একটি variable-এ রাখে, এবং আমরা সেই code update করেছি যেটি আগে আলাদা `query` এবং `file_path` variable ব্যবহার করত যাতে এটি এখন পরিবর্তে `Config` struct-এর field গুলো ব্যবহার করে।

এখন আমাদের code আরও স্পষ্টভাবে প্রকাশ করে যে `query` এবং `file_path` related এবং তাদের purpose হল প্রোগ্রামটি কীভাবে কাজ করবে তা configure করা। এই value গুলো ব্যবহার করে এমন যেকোনো code জানে যে সেগুলিকে `config` instance-এর মধ্যে তাদের purpose-এর জন্য named field-গুলোতে খুঁজতে হবে।

#### `Config`-এর জন্য একটি Constructor তৈরি করা

এখনও পর্যন্ত, আমরা command line argument parse করার জন্য responsible logic-টিকে `main` থেকে extract করে `parse_config` function-এ রেখেছি। এটি করতে গিয়ে আমরা দেখতে পেলাম যে `query` এবং `file_path` value গুলো related ছিল, এবং সেই relationship আমাদের code-এ প্রকাশ করা উচিত। তারপরে আমরা `query` এবং `file_path`-এর related purpose-টির নাম দেওয়ার জন্য এবং `parse_config` function থেকে value-গুলোর নাম struct field name হিসেবে return করতে সক্ষম হওয়ার জন্য একটি `Config` struct যোগ করেছি।

সুতরাং এখন যেহেতু `parse_config` function-টির purpose হল একটি `Config` instance create করা, তাই আমরা `parse_config`-কে একটি plain function থেকে `Config` struct-এর সাথে associated `new` নামের একটি function-এ পরিবর্তন করতে পারি। এই পরিবর্তনটি code-টিকে আরও idiomatic করে তুলবে। আমরা standard library-তে type-গুলোর instance create করতে পারি, যেমন `String`, `String::new` call করে। একইভাবে, `parse_config`-কে `Config`-এর সাথে associated একটি `new` function-এ পরিবর্তন করে, আমরা `Config::new` call করে `Config`-এর instance create করতে পারব। Listing 12-7 আমাদের যে পরিবর্তনগুলো করতে হবে সেগুলো দেখায়।

<Listing number="12-7" file-name="src/main.rs" caption="`parse_config` কে `Config::new`-এ পরিবর্তন করা">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

আমরা `main` update করেছি যেখানে আমরা `parse_config` call করছিলাম, পরিবর্তে `Config::new` call করার জন্য। আমরা `parse_config`-এর নাম পরিবর্তন করে `new` করেছি এবং এটিকে একটি `impl` block-এর মধ্যে move করেছি, যা `new` function-টিকে `Config`-এর সাথে associate করে। এই code টি আবার compile করে দেখুন এটা নিশ্চিত করতে যে এটি কাজ করছে।

### Error Handling ঠিক করা

এখন আমরা আমাদের error handling ঠিক করার জন্য কাজ করব। মনে রাখবেন যে vector-এ তিনটির কম item থাকলে `args` vector-এর index 1 বা index 2-এ value access করার চেষ্টা করলে প্রোগ্রামটি panic করবে। কোনো argument ছাড়া প্রোগ্রামটি run করার চেষ্টা করুন; এটি এইরকম দেখাবে:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

`index out of bounds: the len is 1 but the index is 1` লাইনটি programmers-দের জন্য উদ্দিষ্ট একটি error message। এটি আমাদের end user-দের বুঝতে সাহায্য করবে না যে পরিবর্তে তাদের কী করা উচিত। আসুন এখন সেটা ঠিক করি।

#### Error Message-এর উন্নতি

Listing 12-8-এ, আমরা `new` function-এ একটি check যোগ করি যা index 1 এবং index 2 access করার আগে verify করবে যে slice-টি যথেষ্ট long কিনা। যদি slice-টি যথেষ্ট long না হয়, তাহলে প্রোগ্রামটি panic করে এবং একটি better error message প্রদর্শন করে।

<Listing number="12-8" file-name="src/main.rs" caption="Argument-এর সংখ্যার জন্য একটি check যোগ করা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

এই code-টি [Listing 9-13-এ লেখা আমাদের `Guess::new` function][ch9-custom-types]-এর মতো, যেখানে `value` argument-টি valid value-গুলোর range-এর বাইরে থাকলে আমরা `panic!` call করেছিলাম। এখানে value-গুলোর একটি range check করার পরিবর্তে, আমরা check করছি যে `args`-এর length কমপক্ষে `3` কিনা এবং function-এর বাকি অংশ এই assumption-এর অধীনে operate করতে পারে যে এই condition পূরণ হয়েছে। যদি `args`-এ তিনটির কম item থাকে, তাহলে এই condition-টি `true` হবে, এবং আমরা প্রোগ্রামটিকে immediately end করার জন্য `panic!` macro call করি।

`new`-তে এই কয়েকটি অতিরিক্ত code line সহ, আসুন আবার কোনো argument ছাড়াই প্রোগ্রামটি run করি এটা দেখতে যে error-টি এখন কেমন দেখায়:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

এই output টি আরও ভালো: আমাদের এখন একটি reasonable error message রয়েছে। যাইহোক, আমাদের কাছে extraneous information-ও রয়েছে যা আমরা আমাদের user-দের দিতে চাই না। সম্ভবত Listing 9-13-এ আমরা যে technique ব্যবহার করেছি সেটি এখানে ব্যবহার করার জন্য সেরা নয়: `panic!`-এ call করা usage problem-এর চেয়ে programming problem-এর জন্য বেশি উপযুক্ত, [যেমন Chapter 9-এ আলোচনা করা হয়েছে][ch9-error-guidelines]। পরিবর্তে, আমরা Chapter 9-এ শেখা অন্য technique টি ব্যবহার করব—[একটি `Result` return করা][ch9-result] যা success বা error indicate করে।

<!-- পুরানো heading। সরাবেন না নাহলে link ভেঙে যেতে পারে। -->

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### `panic!` Call করার পরিবর্তে একটি `Result` Return করা

আমরা পরিবর্তে একটি `Result` value return করতে পারি যাতে successful case-এ একটি `Config` instance থাকবে এবং error case-এ problem টি describe করবে। আমরা function-টির নামও `new` থেকে `build`-এ পরিবর্তন করতে যাচ্ছি কারণ অনেক programmer আশা করেন `new` function গুলো কখনই fail করবে না। যখন `Config::build`, `main`-এর সাথে communicate করছে, তখন আমরা `Result` type ব্যবহার করে signal দিতে পারি যে একটি problem ছিল। তারপরে আমরা `main` পরিবর্তন করে `Err` variant-কে আমাদের user-দের জন্য আরও practical error-এ convert করতে পারি, `thread 'main'` এবং `RUST_BACKTRACE` সম্পর্কে আশেপাশের text ছাড়াই যা `panic!`-এ call করার কারণে ঘটে।

Listing 12-9-এ আমরা এখন যে function টিকে `Config::build` বলছি, তার return value এবং একটি `Result` return করার জন্য function-এর body-তে যে পরিবর্তনগুলো করতে হবে সেগুলো দেখানো হলো। মনে রাখবেন যে যতক্ষণ না আমরা `main` update করি, ততক্ষণ পর্যন্ত এটি compile হবে না, যা আমরা পরবর্তী listing-এ করব।

<Listing number="12-9" file-name="src/main.rs" caption="`Config::build` থেকে একটি `Result` Return করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

আমাদের `build` function success case-এ একটি `Config` instance এবং error case-এ একটি string literal সহ একটি `Result` return করে। আমাদের error value গুলো সব সময় string literal হবে যাদের `'static` lifetime আছে।

আমরা function-এর body-তে দুটি পরিবর্তন করেছি: user পর্যাপ্ত argument pass না করলে `panic!` call করার পরিবর্তে, আমরা এখন একটি `Err` value return করি, এবং আমরা `Config` return value-টিকে একটি `Ok`-এ wrap করেছি। এই পরিবর্তনগুলো function-টিকে এর new type signature-এর সাথে সঙ্গতিপূর্ণ করে তোলে।

`Config::build` থেকে একটি `Err` value return করা `main` function-কে `build` function থেকে returned `Result` value handle করতে এবং error case-এ আরও cleanly process exit করতে দেয়।

<!-- পুরানো heading। সরাবেন না নাহলে link ভেঙে যেতে পারে। -->
<a id="calling-confignew-and-handling-errors"></a>

#### `Config::build` কল করা এবং Error হ্যান্ডেল করা

Error case হ্যান্ডেল করতে এবং একটি user-friendly message প্রিন্ট করতে, আমাদের `main` update করতে হবে যাতে এটি `Config::build` দ্বারা returned `Result` হ্যান্ডেল করতে পারে, যেমনটি Listing 12-10-এ দেখানো হয়েছে। আমরা `panic!` থেকে nonzero error code সহ command line tool exit করার responsibility-ও সরিয়ে নেব এবং পরিবর্তে এটি নিজে implement করব। একটি nonzero exit status হল এমন একটি convention যা আমাদের প্রোগ্রাম call করা process-কে signal দেয় যে প্রোগ্রামটি একটি error state-এর সাথে exit করেছে।

<Listing number="12-10" file-name="src/main.rs" caption="`Config` build fail করলে একটি error code দিয়ে Exit করা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

এই listing-এ, আমরা এমন একটি method ব্যবহার করেছি যা আমরা এখনও বিস্তারিতভাবে আলোচনা করিনি: `unwrap_or_else`, যেটি standard library দ্বারা `Result<T, E>`-তে define করা হয়েছে। `unwrap_or_else` ব্যবহার করা আমাদের কিছু custom, non-`panic!` error handling define করার সুযোগ দেয়। যদি `Result` একটি `Ok` value হয়, তাহলে এই method-টির behavior `unwrap`-এর মতোই: এটি `Ok` যে inner value-টিকে wrap করে রেখেছে সেটি return করে। যাইহোক, যদি value-টি একটি `Err` value হয়, তাহলে এই method টি _closure_-এর মধ্যে থাকা code call করে, যেটি হল একটি anonymous function যা আমরা define করি এবং `unwrap_or_else`-এ argument হিসেবে pass করি। আমরা [Chapter 13][ch13]-এ closure সম্পর্কে আরও বিস্তারিত আলোচনা করব। আপাতত, আপনার শুধু এটা জানলেই চলবে যে `unwrap_or_else`, `Err`-এর inner value, যেটি এই ক্ষেত্রে Listing 12-9-এ যোগ করা static string `"not enough arguments"`, সেটিকে vertical pipe-গুলোর মধ্যে থাকা `err` argument-এর মাধ্যমে আমাদের closure-এ pass করবে। Closure-এর ভেতরের code তারপর run করার সময় `err` value-টিকে ব্যবহার করতে পারবে।

আমরা standard library থেকে `process` কে scope-এ আনার জন্য একটি নতুন `use` লাইন যোগ করেছি। Error case-এ যে closure-টি run করবে তার code-এ শুধুমাত্র দুটি লাইন রয়েছে: আমরা `err` value-টি print করি এবং তারপর `process::exit` call করি। `process::exit` function প্রোগ্রামটিকে immediately stop করবে এবং exit status code হিসেবে যে number টি pass করা হয়েছিল সেটি return করবে। এটি Listing 12-8-এ ব্যবহৃত `panic!`-ভিত্তিক handling-এর মতোই, কিন্তু আমরা আর সমস্ত extra output পাচ্ছি না। আসুন চেষ্টা করা যাক:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

দারুণ! এই output টি আমাদের user-দের জন্য অনেক বেশি friendly।

### `main` থেকে Logic Extract করা

এখন যেহেতু আমরা configuration parsing refactor করা শেষ করেছি, আসুন প্রোগ্রামের logic-এর দিকে মনোযোগ দিই। আমরা যেমন [“বাইনারি প্রোজেক্টের জন্য Separation of Concerns”](#separation-of-concerns-for-binary-projects)-এ উল্লেখ করেছি, আমরা `run` নামের একটি function extract করব যেটি বর্তমানে `main` function-এ থাকা সমস্ত logic ধারণ করবে যা configuration set up করা বা error handle করার সাথে জড়িত নয়। যখন আমাদের কাজ শেষ হবে, `main` সংক্ষিপ্ত হবে এবং inspection-এর মাধ্যমে সহজেই verify করা যাবে, এবং আমরা অন্যান্য সমস্ত logic-এর জন্য test লিখতে পারব।

Listing 12-11-এ extract করা `run` function দেখানো হয়েছে। আপাতত, আমরা শুধুমাত্র function extract করার ছোট, incremental improvement করছি। আমরা এখনও _src/main.rs_-এ function-টি define করছি।

<Listing number="12-11" file-name="src/main.rs" caption="প্রোগ্রামের বাকি লজিক যুক্ত একটি `run` ফাংশন Extract করা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

`run` function-এ এখন file read করা থেকে শুরু করে `main`-এর সমস্ত অবশিষ্ট logic রয়েছে। `run` function টি `Config` instance-টিকে argument হিসেবে নেয়।

#### `run` Function থেকে Error Return করা

প্রোগ্রামের অবশিষ্ট logic `run` function-এ আলাদা করার সাথে, আমরা error handling-এর উন্নতি করতে পারি, যেমনটি আমরা Listing 12-9-এ `Config::build`-এর সাথে করেছিলাম। `expect` call করে প্রোগ্রামটিকে panic করার অনুমতি দেওয়ার পরিবর্তে, `run` function টি কোনো কিছু ভুল হলে একটি `Result<T, E>` return করবে। এটি আমাদেরকে user-friendly উপায়ে error handle করার জন্য logic-কে `main`-এ আরও consolidate করার সুযোগ দেবে। Listing 12-12 `run`-এর signature এবং body-তে আমাদের যে পরিবর্তনগুলো করতে হবে সেগুলো দেখায়।

<Listing number="12-12" file-name="src/main.rs" caption="`Result` Return করার জন্য `run` ফাংশন পরিবর্তন করা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

আমরা এখানে তিনটি উল্লেখযোগ্য পরিবর্তন করেছি। প্রথমত, আমরা `run` function-এর return type পরিবর্তন করে `Result<(), Box<dyn Error>>` করেছি। এই function টি আগে unit type, `()`, return করত, এবং আমরা `Ok` case-এ returned value হিসেবে সেটিই রাখছি।

Error type-এর জন্য, আমরা _trait object_ `Box<dyn Error>` ব্যবহার করেছি (এবং আমরা উপরের দিকে একটি `use` statement দিয়ে `std::error::Error` কে scope-এ এনেছি)। আমরা [Chapter 18][ch18]-এ trait object নিয়ে আলোচনা করব। আপাতত, শুধু জেনে রাখুন যে `Box<dyn Error>` মানে function টি এমন একটি type return করবে যা `Error` trait implement করে, কিন্তু আমাদের specify করতে হবে না যে return value-টি specific কোন type-এর হবে। এটি আমাদেরকে error value return করার flexibility দেয় যা different error case-এ different type-এর হতে পারে। `dyn` keyword টি _dynamic_-এর সংক্ষিপ্ত রূপ।

দ্বিতীয়ত, আমরা [Chapter 9][ch9-question-mark]-এ আলোচনা করা `?` operator-এর পক্ষে `expect`-এর call সরিয়ে দিয়েছি। কোনো error-এর উপর `panic!` করার পরিবর্তে, `?` current function থেকে error value-টি return করবে যাতে caller সেটি handle করতে পারে।

তৃতীয়ত, `run` function টি এখন success case-এ একটি `Ok` value return করে। আমরা signature-এ `run` function-এর success type `()` হিসেবে declare করেছি, যার অর্থ হল আমাদের unit type value-টিকে `Ok` value-তে wrap করতে হবে। এই `Ok(())` syntax টি প্রথমে একটু অদ্ভুত লাগতে পারে, কিন্তু এইভাবে `()` ব্যবহার করা হল idiomatic উপায় এটা indicate করার জন্য যে আমরা `run` কে শুধুমাত্র এর side effect-গুলোর জন্য call করছি; এটি আমাদের প্রয়োজনীয় কোনো value return করে না।

আপনি যখন এই code run করবেন, এটি compile হবে কিন্তু একটি warning প্রদর্শন করবে:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust আমাদের বলছে যে আমাদের code `Result` value-টিকে ignore করেছে এবং `Result` value-টি indicate করতে পারে যে একটি error ঘটেছে। কিন্তু আমরা check করছি না যে কোনো error হয়েছে কিনা, এবং compiler আমাদের মনে করিয়ে দিচ্ছে যে সম্ভবত আমাদের এখানে কিছু error-handling code থাকা উচিত ছিল! আসুন এখনই সেই সমস্যাটি সংশোধন করি।

#### `main`-এ `run` থেকে Returned Error হ্যান্ডেল করা

আমরা error-গুলো check করব এবং Listing 12-10-এ `Config::build`-এর সাথে ব্যবহৃত পদ্ধতির মতোই একটি technique ব্যবহার করে সেগুলি handle করব, তবে সামান্য ভিন্নতা সহ:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

আমরা `unwrap_or_else`-এর পরিবর্তে `if let` ব্যবহার করি এটা check করার জন্য যে `run` একটি `Err` value return করে কিনা এবং যদি করে তবে `process::exit(1)` call করার জন্য। `run` function এমন কোনো value return করে না যা আমরা `unwrap` করতে চাই, যেমন `Config::build`, `Config` instance return করে। যেহেতু `run` success case-এ `()` return করে, তাই আমরা শুধুমাত্র একটি error detect করার বিষয়ে আগ্রহী, তাই unwrapped value return করার জন্য আমাদের `unwrap_or_else`-এর প্রয়োজন নেই, যেটি শুধুমাত্র `()` হবে।

`if let` এবং `unwrap_or_else` function-গুলোর body উভয় ক্ষেত্রেই একই: আমরা error print করি এবং exit করি।

### কোডকে একটি Library Crate-এ Split করা

আমাদের `minigrep` project-টি এখনও পর্যন্ত ভালো দেখাচ্ছে! এখন আমরা _src/main.rs_ file-টিকে split করব এবং কিছু code _src/lib.rs_ file-এ রাখব। এইভাবে, আমরা code test করতে পারব এবং _src/main.rs_ file-টিতে responsibility কম রাখতে পারব।

আসুন _src/main.rs_ থেকে _src/lib.rs_-এ সমস্ত code সরিয়ে নিই যা `main` function-এ নেই:

-   `run` function definition
-   Relevant `use` statement-গুলো
-   `Config`-এর definition
-   `Config::build` function definition

_src/lib.rs_-এর contents-এ Listing 12-13-এ দেখানো signature গুলো থাকা উচিত (সংक्षिप्तতার জন্য আমরা function-গুলোর body বাদ দিয়েছি)। মনে রাখবেন যে যতক্ষণ না আমরা Listing 12-14-এ _src/main.rs_ modify করি, ততক্ষণ পর্যন্ত এটি compile হবে না।

<Listing number="12-13" file-name="src/lib.rs" caption="`Config` এবং `run` কে *src/lib.rs*-এ সরানো">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```

</Listing>

আমরা `pub` keyword-টির উদার ব্যবহার করেছি: `Config`-এ, এর field এবং এর `build` method-এ এবং `run` function-এ। আমাদের এখন একটি library crate রয়েছে যার একটি public API রয়েছে যা আমরা test করতে পারি!

এখন আমাদের Listing 12-14-এ দেখানো code-টিকে _src/lib.rs_-এ সরানো code binary crate-এর scope-এ _src/main.rs_-এ আনতে হবে।

<Listing number="12-14" file-name="src/main.rs" caption="`minigrep` library crate-টিকে *src/main.rs*-এ ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

আমরা library crate থেকে binary crate-এর scope-এ `Config` type-টি আনার জন্য একটি `use minigrep::Config` লাইন যোগ করি এবং আমরা `run` function-টির আগে আমাদের crate-এর নাম prefix করি। এখন সমস্ত functionality সংযুক্ত হওয়া উচিত এবং কাজ করা উচিত। `cargo run` দিয়ে প্রোগ্রামটি run করুন এবং নিশ্চিত করুন যে সবকিছু সঠিকভাবে কাজ করছে।

উফ! এটা অনেক কাজ ছিল, কিন্তু আমরা ভবিষ্যতে নিজেদের সাফল্যের জন্য প্রস্তুত করেছি। এখন error handle করা অনেক সহজ, এবং আমরা code-টিকে আরও modular করেছি। এখন থেকে আমাদের প্রায় সমস্ত কাজ _src/lib.rs_-এ করা হবে।

আসুন এই নতুন পাওয়া modularity-র সুবিধা নিই এমন কিছু করে যা পুরানো code-এর সাথে করা কঠিন হত কিন্তু নতুন code-এর সাথে সহজ: আমরা কিছু test লিখব!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch18]: ch18-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
