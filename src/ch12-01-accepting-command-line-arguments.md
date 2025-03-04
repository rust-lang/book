
## কমান্ড লাইন আর্গুমেন্ট গ্রহণ করা

আসুন, বরাবরের মতো, `cargo new` ব্যবহার করে একটি নতুন project তৈরি করি। আমরা আমাদের projectটির নাম দেব `minigrep`, যাতে এটিকে আপনার সিস্টেমে ஏற்கனவே থাকা `grep` tool থেকে আলাদা করা যায়।

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

প্রথম কাজটি হল `minigrep`-কে দুটি কমান্ড লাইন আর্গুমেন্ট গ্রহণ করতে সক্ষম করা: file path এবং যে string টি অনুসন্ধান করতে হবে সেটি। অর্থাৎ, আমরা চাই আমাদের প্রোগ্রামটি `cargo run`, দুটি হাইফেন (যা নির্দেশ করে যে নিম্নলিখিত আর্গুমেন্টগুলি `cargo`-র জন্য নয়, আমাদের প্রোগ্রামের জন্য), একটি search string, এবং যে file-এ অনুসন্ধান করতে হবে তার path সহ চালাতে:

```console
$ cargo run -- searchstring example-filename.txt
```

এখন, `cargo new` দ্বারা generate করা প্রোগ্রামটি আমাদের দেওয়া arguments process করতে পারে না। [crates.io](https://crates.io/)-তে কিছু existing library আছে যারা কমান্ড লাইন আর্গুমেন্ট গ্রহণ করে এমন প্রোগ্রাম লিখতে সাহায্য করতে পারে, কিন্তু যেহেতু আপনি এই concept টি শিখছেন, তাই আসুন আমরা নিজেরাই এই ক্ষমতাটি implement করি।

### আর্গুমেন্ট ভ্যালুগুলো পড়া

`minigrep`-এ আমরা যে command line argument-গুলো pass করি, সেগুলোর value read করার জন্য, আমরা Rust-এর standard library-তে থাকা `std::env::args` function টি ব্যবহার করব। এই function টি `minigrep`-এ pass করা command line argument-গুলোর একটি iterator return করে। আমরা [Chapter 13][ch13]-এ iterator সম্পর্কে বিস্তারিত আলোচনা করব। এখন, আপনার iterator সম্পর্কে শুধুমাত্র দুটি বিষয় জানতে হবে: iterator-গুলো values-এর একটি series produce করে, এবং আমরা একটি iterator-এর উপর `collect` method call করে এটিকে একটি collection-এ পরিণত করতে পারি, যেমন একটি vector, যেখানে iterator-এর produce করা সমস্ত element থাকবে।

Listing 12-1-এর code আপনার `minigrep` প্রোগ্রামকে যেকোনো command line argument read করতে এবং তারপর value-গুলোকে একটি vector-এ collect করতে দেয়।

<Listing number="12-1" file-name="src/main.rs" caption="কমান্ড লাইন আর্গুমেন্টগুলো একটি ভেক্টরে সংগ্রহ করা এবং সেগুলো প্রিন্ট করা">

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

</Listing>

প্রথমে আমরা `std::env` module-টিকে `use` statement-এর মাধ্যমে scope-এ আনি, যাতে আমরা এর `args` function-টি ব্যবহার করতে পারি। লক্ষ্য করুন যে `std::env::args` function-টি দুটি স্তরের module-এর মধ্যে nested। আমরা যেমন [Chapter
7][ch7-idiomatic-use]-এ আলোচনা করেছি, যেসব ক্ষেত্রে desired function একাধিক module-এর মধ্যে nested থাকে, সেক্ষেত্রে আমরা function-এর পরিবর্তে parent module-টিকে scope-এ আনতে চেয়েছি। এটা করার মাধ্যমে, আমরা সহজেই `std::env`-এর অন্যান্য function-গুলো ব্যবহার করতে পারি। এছাড়াও, `use std::env::args` যোগ করে শুধুমাত্র `args` দিয়ে function-টিকে call করার চেয়ে এটি কম ambiguous, কারণ `args` সহজেই current module-এ defined কোনো function বলে ভুল হতে পারে।

> ### `args` ফাংশন এবং ইনভ্যালিড ইউনিকোড
>
> মনে রাখবেন যে, যদি কোনো argument-এ invalid Unicode থাকে, তাহলে `std::env::args` প্যানিক করবে। যদি আপনার প্রোগ্রামের invalid Unicode যুক্ত argument গ্রহণ করার প্রয়োজন হয়, তাহলে এর পরিবর্তে `std::env::args_os` ব্যবহার করুন। সেই function-টি একটি iterator return করে যা `String` value-এর পরিবর্তে `OsString` value produce করে। আমরা এখানে সরলতার জন্য `std::env::args` ব্যবহার করা বেছে নিয়েছি, কারণ `OsString` value-গুলো platform অনুযায়ী ভিন্ন হয় এবং `String` value-গুলোর চেয়ে এগুলোর সাথে কাজ করা আরও জটিল।

`main`-এর প্রথম লাইনে, আমরা `env::args` call করি, এবং iterator-টিকে তৎক্ষণাৎ `collect` ব্যবহার করে একটি vector-এ পরিণত করি, যেখানে iterator দ্বারা produced সমস্ত value থাকে। আমরা `collect` function ব্যবহার করে বিভিন্ন ধরনের collection তৈরি করতে পারি, তাই আমরা `args`-এর type টি explicit ভাবে annotate করি যাতে বোঝা যায় যে আমরা string-এর একটি vector চাই। যদিও Rust-এ খুব কমই type annotate করার প্রয়োজন হয়, `collect` এমন একটি function যেখানে প্রায়ই annotate করার প্রয়োজন হয় কারণ Rust নিজে থেকে বুঝতে পারে না যে আপনি কী ধরনের collection চান।

অবশেষে, আমরা debug macro ব্যবহার করে vector-টি print করি। আসুন প্রথমে কোনো argument ছাড়া এবং তারপর দুটি argument দিয়ে code টি run করে দেখি:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

লক্ষ্য করুন যে vector-এর প্রথম value টি হল `"target/debug/minigrep"`, যেটি আমাদের binary-র নাম। এটি C-তে arguments list-এর আচরণের সাথে মেলে, যা প্রোগ্রামগুলোকে execution-এর সময় তাদের যে নামে invoke করা হয়েছিল সেটি ব্যবহার করতে দেয়। প্রোগ্রামের নামটি access করতে পারা সুবিধাজনক হতে পারে যদি আপনি এটিকে message-এ print করতে চান বা প্রোগ্রামের behavior পরিবর্তন করতে চান এই ভিত্তিতে যে প্রোগ্রামটি invoke করার জন্য কোন command line alias ব্যবহার করা হয়েছে। কিন্তু এই chapter-এর উদ্দেশ্যের জন্য, আমরা এটিকে ignore করব এবং শুধুমাত্র আমাদের প্রয়োজনীয় দুটি argument save করব।

### আর্গুমেন্ট ভ্যালুগুলো ভেরিয়েবলে সংরক্ষণ করা

প্রোগ্রামটি এখন command line argument হিসেবে specified value-গুলো access করতে পারছে। এখন আমাদের দুটি argument-এর value-গুলোকে variable-এ save করতে হবে যাতে আমরা value-গুলো প্রোগ্রামের বাকি অংশে ব্যবহার করতে পারি। Listing 12-2 তে আমরা সেটাই করব।

<Listing number="12-2" file-name="src/main.rs" caption="query আর্গুমেন্ট এবং file path আর্গুমেন্ট রাখার জন্য ভেরিয়েবল তৈরি করা">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

</Listing>

আমরা যখন vector-টি print করেছিলাম তখন যেমন দেখেছিলাম, প্রোগ্রামের নামটি vector-এর প্রথম value, `args[0]` দখল করে, তাই আমরা argument-গুলো index 1 থেকে শুরু করছি। `minigrep` যে প্রথম argument টি নেয় সেটি হল সেই string যেটি আমরা search করছি, তাই আমরা প্রথম argument-টির একটি reference `query` variable-এ রাখি। দ্বিতীয় argument টি হবে file path, তাই আমরা দ্বিতীয় argument-টির একটি reference `file_path` variable-এ রাখি।

আমরা অস্থায়ীভাবে এই variable-গুলোর value print করি এটা প্রমাণ করার জন্য যে code টি আমাদের ইচ্ছা অনুযায়ী কাজ করছে। আসুন `test` এবং `sample.txt` argument-গুলো দিয়ে এই প্রোগ্রামটি আবার run করি:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

দারুণ, প্রোগ্রামটি কাজ করছে! আমাদের প্রয়োজনীয় argument-গুলোর value সঠিক variable-গুলোতে save করা হচ্ছে। পরে আমরা কিছু error handling যোগ করব কিছু সম্ভাব্য ত্রুটিপূর্ণ পরিস্থিতি, যেমন যখন user কোনো argument provide করে না; আপাতত, আমরা সেই পরিস্থিতিটি ignore করব এবং এর পরিবর্তে file-reading ক্ষমতা যোগ করার উপর কাজ করব।

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
