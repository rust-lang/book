# Functional Language Features: Iterators and Closures

Rust-এর design অনেক existing language এবং technique থেকে inspiration নিয়েছে, এবং একটি significant influence হল _functional programming_। Functional style-এ Programming-এ প্রায়শই function-গুলোকে value হিসেবে ব্যবহার করা হয় সেগুলোকে argument হিসেবে pass করে, অন্য function থেকে return করে, পরে execution-এর জন্য variable-এ assign করে, এবং আরও অনেক কিছু।

এই chapter-এ, আমরা functional programming কী বা কী নয় সে বিষয়ে বিতর্ক করব না, বরং Rust-এর কিছু feature নিয়ে আলোচনা করব যেগুলো functional হিসেবে refer করা অনেক language-এর feature-গুলোর মতো।

আরও specifically, আমরা আলোচনা করব:

-   _Closures_, একটি function-এর মতো construct যাকে আপনি একটি variable-এ store করতে পারেন
-   _Iterators_, element-এর একটি series process করার একটি উপায়
-   Chapter 12-এর I/O project-কে improve করতে closure এবং iterator কীভাবে ব্যবহার করবেন
-   Closure এবং iterator-এর performance (Spoiler alert: আপনি যতটা ভাবছেন সেগুলি তার চেয়ে দ্রুত!)

আমরা ইতিমধ্যেই pattern matching এবং enum-এর মতো আরও কিছু Rust feature cover করেছি, যেগুলোও functional style দ্বারা প্রভাবিত। যেহেতু idiomatic, fast Rust code লেখার জন্য closure এবং iterator-এ দক্ষতা অর্জন করা গুরুত্বপূর্ণ, তাই আমরা এই পুরো chapter-টি এগুলোর জন্য উৎসর্গ করব।
