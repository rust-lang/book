## একটি File পড়া

এখন আমরা `file_path` argument-এ specified file টি read করার functionality যোগ করব। প্রথমে আমাদের test করার জন্য একটি sample file-এর প্রয়োজন: আমরা অল্প কিছু text, multiple line এবং কিছু repeated word সহ একটি file ব্যবহার করব। Listing 12-3-এ Emily Dickinson-এর একটি কবিতা আছে যা এই কাজের জন্য উপযুক্ত! আপনার project-এর root level-এ _poem.txt_ নামে একটি file create করুন, এবং “I’m Nobody! Who are you?” কবিতাটি লিখুন।

<Listing number="12-3" file-name="poem.txt" caption="Emily Dickinson-এর একটি কবিতা একটি ভালো test case।">

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

</Listing>

Text টি create করা হয়ে গেলে, _src/main.rs_ edit করুন এবং file read করার জন্য code যোগ করুন, যেমনটি Listing 12-4-এ দেখানো হয়েছে।

<Listing number="12-4" file-name="src/main.rs" caption="দ্বিতীয় argument দ্বারা specified file-এর contents পড়া">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

</Listing>

প্রথমে আমরা `use` statement-এর মাধ্যমে standard library-এর একটি relevant অংশ import করি: file handle করার জন্য আমাদের `std::fs`-এর প্রয়োজন।

`main`-এ, নতুন statement `fs::read_to_string`, `file_path` নেয়, সেই file টি open করে, এবং file-এর contents সহ `std::io::Result<String>` type-এর একটি value return করে।

এরপরে, আমরা আবার একটি temporary `println!` statement যোগ করি যা file read করার পরে `contents`-এর value print করে, যাতে আমরা check করতে পারি যে প্রোগ্রামটি এখনও পর্যন্ত ঠিকঠাক কাজ করছে।

আসুন আমরা এই code-টি প্রথম command line argument হিসেবে যেকোনো string (কারণ আমরা এখনও searching অংশটি implement করিনি) এবং দ্বিতীয় argument হিসেবে _poem.txt_ file দিয়ে run করি:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

দারুণ! Code file-এর contents read করে print করেছে। কিন্তু code-টিতে কিছু সমস্যা রয়েছে। বর্তমানে, `main` function-টির multiple responsibility রয়েছে: সাধারণত, function-গুলো clear এবং maintain করা সহজ হয় যদি প্রতিটি function শুধুমাত্র একটি idea-র জন্য responsible হয়। আরেকটি সমস্যা হল আমরা error গুলোকে যতটা ভালোভাবে handle করা সম্ভব ততটা করছি না। প্রোগ্রামটি এখনও ছোট, তাই এই ত্রুটিগুলো বড় কোনো সমস্যা নয়, কিন্তু প্রোগ্রামটি যত বড় হবে, এগুলোকে পরিষ্কারভাবে ঠিক করা তত কঠিন হবে। প্রোগ্রাম develop করার সময় শুরুতেই refactor করা একটি ভালো অভ্যাস, কারণ অল্প পরিমাণ code refactor করা অনেক সহজ। আমরা এরপরে সেটাই করব।
