## Putting It All Together: Futures, Tasks, and Threads

আমরা [Chapter 16][ch16]<!-- ignore --> এ যেমন দেখেছি, thread concurrency এর জন্য একটি approach provide করে। আমরা এই chapter এ অন্য approach দেখেছি: future এবং stream এর সাথে async ব্যবহার করা। আপনি যদি ভাবছেন কখন একটি method এর উপর অন্য method choose করবেন, তাহলে এর উত্তর হলো: এটা depend করে! এবং অনেক ক্ষেত্রে, choice threads _বা_ async নয় বরং threads _এবং_ async।

অনেক operating system এখন দশক ধরে threading-based concurrency model supply করে আসছে, এবং অনেক programming language ও result হিসেবে এগুলো support করে। তবে, এই model গুলোর কিছু tradeoff ও আছে। অনেক operating system এ, তারা প্রত্যেক thread এর জন্য বেশ কিছু memory ব্যবহার করে, এবং start up এবং shut down করার জন্য কিছু overhead লাগে। Thread শুধুমাত্র তখনই option যখন আপনার operating system এবং hardware support করে। Mainstream desktop এবং mobile computer এর বিপরীতে, কিছু embedded system এ কোনো OS থাকে না, তাই তাদের thread ও থাকে না।

Async model একটি different—এবং ultimately complementary—set of tradeoffs provide করে। Async model এ, concurrent operation এর জন্য নিজেদের thread এর প্রয়োজন হয় না। এর পরিবর্তে, তারা task এ run করতে পারে, যেমন streams section এ synchronous function থেকে কাজ শুরু করার জন্য `trpl::spawn_task` ব্যবহার করার সময় আমরা করেছিলাম। Task হলো thread এর similar, কিন্তু operating system দ্বারা manage হওয়ার পরিবর্তে, এটি library-level code: runtime দ্বারা manage হয়।

আগের section এ, আমরা দেখেছিলাম কিভাবে একটি async channel ব্যবহার করে এবং synchronous code থেকে call করতে পারি এমন একটি async task spawn করে একটি stream তৈরি করা যায়। আমরা thread দিয়ে exactly same কাজ করতে পারি। Listing 17-40 এ, আমরা `trpl::spawn_task` এবং `trpl::sleep` ব্যবহার করেছিলাম। Listing 17-41 এ, আমরা `get_intervals` function এ standard library থেকে `thread::spawn` এবং `thread::sleep` API দিয়ে replace করি।

<Listing number="17-41" caption="`get_intervals` function এর জন্য async `trpl` API এর পরিবর্তে `std::thread` API ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-41/src/main.rs:threads}}
```

</Listing>

আপনি যদি এই code run করেন, তাহলে output Listing 17-40 এর output এর মতোই হবে। এবং notice করুন এখানে calling code এর perspective থেকে কত কম change হয়েছে। আরও কি, যদিও আমাদের function গুলোর একটি runtime এ async task spawn করে এবং অন্যটি OS thread spawn করে, resulting stream গুলোর difference দ্বারা affected হয় না।

তাদের similarity থাকা সত্ত্বেও, এই দুটি approach খুব differently behave করে, যদিও এই very simple উদাহরণে আমরা এটা measure করতে কষ্ট করতে পারি। আমরা যেকোনো modern personal computer এ million async task spawn করতে পারি। যদি আমরা thread দিয়ে তা করার চেষ্টা করতাম, তাহলে আমাদের memory শেষ হয়ে যেত!

তবে, এই API গুলো similar হওয়ার একটি কারণ আছে। Thread synchronous operation এর set এর জন্য boundary হিসেবে কাজ করে; concurrency thread এর _মধ্যে_ possible। Task _asynchronous_ operation এর set এর জন্য boundary হিসেবে কাজ করে; concurrency task এর _মধ্যে_ এবং _ভিতরে_ উভয় ক্ষেত্রেই possible, কারণ একটি task তার body তে future এর মধ্যে switch করতে পারে। সবশেষে, future হলো Rust এর সবচেয়ে granular unit of concurrency, এবং প্রত্যেক future অন্য future এর tree represent করতে পারে। Runtime—specifically, এর executor—task manage করে, এবং task future manage করে। সেই হিসেবে, task গুলো runtime দ্বারা managed lightweight thread এর মতো, যেখানে operating system এর পরিবর্তে runtime দ্বারা manage হওয়ার কারণে additional capability আছে।

এর মানে এই নয় যে async task সবসময় thread এর চেয়ে ভালো (বা vice versa)। Thread এর সাথে concurrency async এর সাথে concurrency এর চেয়ে simpler programming model। এটা strength বা weakness দুটোই হতে পারে। Thread অনেকটা “fire and forget”; তাদের future এর equivalent কিছু নেই, তাই তারা অপারেটিং system ছাড়া আর কারো দ্বারা interrupt হওয়া ছাড়াই completion পর্যন্ত run করে। অর্থাৎ, future এর মতো তাদের _intratask concurrency_ এর জন্য কোনো built-in support নেই। Rust এ thread এর cancellation এর জন্য কোনো mechanism নেই—যা আমরা এই chapter এ explicitly cover করিনি কিন্তু যখন আমরা future শেষ করি, তখন এর state correctly clean up হয়ে যায় এই fact দ্বারা imply করা হয়েছে।

এই limitation এর কারণেও thread future এর চেয়ে compose করা কঠিন। উদাহরণস্বরূপ, thread ব্যবহার করে `timeout` এবং `throttle` method এর মতো helper তৈরি করা অনেক বেশি কঠিন, যা আমরা এই chapter এর আগে তৈরি করেছিলাম। Future হলো richer data structure এর fact মানে হলো তারা আরও naturally compose হতে পারে, যেমনটা আমরা দেখেছি।

Task, তাহলে, আমাদের future এর উপর _additional_ control দেয়, আমাদের group করার জন্য কোথায় এবং কিভাবে choose করার সুযোগ দেয়। এবং দেখা যায় যে thread এবং task প্রায়ই একসাথে ভালোভাবে কাজ করে, কারণ task (কমপক্ষে কিছু runtime এ) thread এর মধ্যে move করা যেতে পারে। আসলে, under the hood এ, আমরা যে runtime ব্যবহার করছি—`spawn_blocking` এবং `spawn_task` function সহ—তা default ভাবে multithreaded! অনেক runtime _work stealing_ নামে একটি approach ব্যবহার করে transparently thread এর মধ্যে task move করার জন্য, thread গুলো বর্তমানে কিভাবে ব্যবহার হচ্ছে তার উপর ভিত্তি করে, system এর overall performance improve করার জন্য। সেই approach এর জন্য আসলে thread _এবং_ task, এবং সেই কারণে future এর ও প্রয়োজন।

কখন কোন method ব্যবহার করবেন তা নিয়ে ভাবার সময়, এই rule of thumb গুলো consider করুন:

-   যদি কাজ _খুব বেশি parallelizable_ হয়, যেমন data এর bunch process করা যেখানে প্রত্যেক part separately process করা যায়, তাহলে thread ভালো choice।
-   যদি কাজ _খুব বেশি concurrent_ হয়, যেমন বিভিন্ন source থেকে message handle করা যা different interval বা different rate এ আসতে পারে, তাহলে async ভালো choice।

এবং যদি আপনার parallelism এবং concurrency উভয়ের প্রয়োজন হয়, তাহলে আপনার thread এবং async এর মধ্যে choose করার প্রয়োজন নেই। আপনি সেগুলোকে freely একসাথে ব্যবহার করতে পারেন, প্রত্যেকটি তার best part play করতে দিয়ে। উদাহরণস্বরূপ, Listing 17-42 real-world Rust code এ এই ধরনের mix এর একটি common উদাহরণ দেখায়।

<Listing number="17-42" caption="Thread এ blocking code দিয়ে message send করা এবং async block এ message await করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-42/src/main.rs:all}}
```

</Listing>

আমরা একটি async channel তৈরি করে শুরু করি, তারপর channel এর sender side এর ownership নিয়ে একটি thread spawn করি। Thread এর মধ্যে, আমরা এক থেকে দশ পর্যন্ত number send করি, প্রত্যেকটির মাঝে এক সেকেন্ড sleep করে। সবশেষে, আমরা `trpl::run` এ pass করা একটি async block দিয়ে তৈরি একটি future run করি যেমনটা আমরা পুরো chapter জুড়ে করে এসেছি। সেই future এ, আমরা সেই message গুলো await করি, ঠিক যেমন আমরা অন্য message-passing উদাহরণগুলোতে দেখেছি।

Chapter টি আমরা যে scenario দিয়ে open করেছিলাম সেখানে ফিরে গেলে, ধরুন dedicated thread ব্যবহার করে video encoding task এর একটি set run করছেন (কারণ video encoding compute-bound) কিন্তু async channel দিয়ে UI কে notify করছেন যে সেই operation গুলো শেষ হয়ে গেছে। এই ধরনের combination এর real-world use case এর অসংখ্য উদাহরণ আছে।

## Summary

এই বইয়ে concurrency নিয়ে এটাই শেষ নয়। [Chapter 21][ch21] এর project এ এই concept গুলো আরও realistic পরিস্থিতিতে apply করা হবে যা এখানে discuss করা simple example এর চেয়ে অনেক বেশি এবং threading বনাম task দিয়ে problem solve করা direct ভাবে compare করা হবে।

এই approach গুলোর মধ্যে আপনি যেটাই choose করেন না কেন, Rust আপনাকে safe, fast, concurrent code লেখার জন্য প্রয়োজনীয় tool provide করে—সেটা high-throughput web server এর জন্য হোক বা embedded operating system এর জন্য হোক।

এরপর, আমরা আলোচনা করব কিভাবে Rust program বড় হওয়ার সাথে সাথে problem model করার এবং solution structure করার idiomatic উপায়। এছাড়াও, আমরা আলোচনা করব Rust এর idiom গুলো object-oriented programming থেকে পরিচিত idiom গুলোর সাথে কিভাবে related।

[ch16]: http://localhost:3000/ch16-00-concurrency.html
[combining-futures]: ch17-03-more-futures.html#building-our-own-async-abstractions
[streams]: ch17-04-streams.html#composing-streams
[ch21]: ch21-00-final-project-a-web-server.html
