## Standard Output-এর পরিবর্তে Standard Error-এ Error Message লেখা

এখন আমরা `println!` macro ব্যবহার করে আমাদের সমস্ত output terminal-এ লিখছি। বেশিরভাগ terminal-এ, দুই ধরনের output রয়েছে: সাধারণ তথ্যের জন্য _standard output_ (`stdout`) এবং error message-এর জন্য _standard error_ (`stderr`।)। এই পার্থক্য users-দের প্রোগ্রামের successful output-কে একটি file-এ direct করা বেছে নিতে সক্ষম করে, কিন্তু তবুও error message গুলো screen-এ print করে।

`println!` macro শুধুমাত্র standard output-এ print করতে সক্ষম, তাই standard error-এ print করার জন্য আমাদের অন্য কিছু ব্যবহার করতে হবে।

### Error গুলো কোথায় লেখা হচ্ছে তা Check করা

প্রথমে আসুন দেখি কীভাবে `minigrep` দ্বারা printed content বর্তমানে standard output-এ লেখা হচ্ছে, সেইসাথে যে কোনও error message-ও যা আমরা পরিবর্তে standard error-এ লিখতে চাই। আমরা standard output stream-কে একটি file-এ redirect করার সময় ইচ্ছাকৃতভাবে একটি error সৃষ্টি করে তা করব। আমরা standard error stream redirect করব না, তাই standard error-এ পাঠানো যেকোনো content screen-এ প্রদর্শিত হতে থাকবে।

Command line program গুলো standard error stream-এ error message পাঠাবে বলে আশা করা হয় যাতে আমরা standard output stream-কে একটি file-এ redirect করলেও screen-এ error message দেখতে পাই। আমাদের প্রোগ্রামটি বর্তমানে ভালোভাবে আচরণ করছে না: আমরা দেখতে পাব যে এটি পরিবর্তে error message output-টি একটি file-এ save করে!

এই behavior টি প্রদর্শন করার জন্য, আমরা `>` এবং file path, _output.txt_ দিয়ে প্রোগ্রামটি run করব, যেখানে আমরা standard output stream-টি redirect করতে চাই। আমরা কোনো argument pass করব না, যার ফলে একটি error হওয়া উচিত:

```console
$ cargo run > output.txt
```

`>` syntax টি shell-কে screen-এর পরিবর্তে standard output-এর contents _output.txt_-এ লিখতে বলে। আমরা যে error message-টি আশা করছিলাম সেটি screen-এ printed হতে দেখিনি, তার মানে এটি file-এ চলে গেছে। _output.txt_-তে এটি রয়েছে:

```text
Problem parsing arguments: not enough arguments
```

হ্যাঁ, আমাদের error message টি standard output-এ print করা হচ্ছে। এই ধরনের error message-গুলো standard error-এ print করা আরও বেশি useful, যাতে successful run-এর data শুধুমাত্র file-টিতে থাকে। আমরা সেটা পরিবর্তন করব।

### Standard Error-এ Error Print করা

Error message গুলো কীভাবে print করা হয় তা পরিবর্তন করতে আমরা Listing 12-24-এর code ব্যবহার করব। এই chapter-এ আগে করা refactoring-এর কারণে, error message print করে এমন সমস্ত code একটি function, `main`-এ রয়েছে। Standard library `eprintln!` macro provide করে যা standard error stream-এ print করে, তাই আসুন দুটি জায়গা পরিবর্তন করি যেখানে আমরা error print করার জন্য `println!` call করছিলাম, পরিবর্তে `eprintln!` ব্যবহার করার জন্য।

<Listing number="12-24" file-name="src/main.rs" caption="`eprintln!` ব্যবহার করে standard output-এর পরিবর্তে standard error-এ error message লেখা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

আসুন এখন কোনো argument ছাড়াই এবং `>` দিয়ে standard output redirect করে একইভাবে প্রোগ্রামটি আবার run করি:

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

এখন আমরা screen-এ error দেখতে পাচ্ছি এবং _output.txt_-তে কিছুই নেই, যা command line program-গুলোর ক্ষেত্রে আমাদের প্রত্যাশিত behavior।

আসুন প্রোগ্রামটি আবার এমন argument দিয়ে run করি যা কোনো error সৃষ্টি করে না কিন্তু তবুও standard output-কে একটি file-এ redirect করে, এইভাবে:

```console
$ cargo run -- to poem.txt > output.txt
```

আমরা terminal-এ কোনো output দেখতে পাব না, এবং _output.txt_-তে আমাদের result গুলো থাকবে:

<span class="filename">Filename: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

এটি প্রদর্শন করে যে আমরা এখন successful output-এর জন্য standard output এবং error output-এর জন্য standard error ব্যবহার করছি।

## সারসংক্ষেপ

এই chapter-এ ఇప్పటి পর্যন্ত শেখা কিছু major concept-এর পুনরাবৃত্তি করা হয়েছে এবং Rust-এ common I/O operation গুলো কীভাবে perform করতে হয় তা আলোচনা করা হয়েছে। Command line argument, file, environment variable, এবং error print করার জন্য `eprintln!` macro ব্যবহার করে, আপনি এখন command line application লেখার জন্য প্রস্তুত। পূর্ববর্তী chapter-গুলোর concept-গুলোর সাথে মিলিত হয়ে, আপনার code ভালোভাবে organized হবে, উপযুক্ত data structure-গুলোতে data কার্যকরভাবে store করবে, error-গুলোকে সুন্দরভাবে handle করবে এবং ভালোভাবে tested হবে।

এরপরে, আমরা functional language দ্বারা প্রভাবিত Rust-এর কিছু feature explore করব: closure এবং iterator।
