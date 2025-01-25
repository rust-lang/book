```markdown
## Shared-State Concurrency

Message passing concurrency handle করার একটি ভালো উপায়, কিন্তু এটাই একমাত্র উপায় নয়। আরেকটি পদ্ধতি হলো একাধিক থ্রেড একই shared data অ্যাক্সেস করা। Go language এর documentation থেকে এই স্লোগানটির অংশটি আবার বিবেচনা করুন: “do not communicate by sharing memory.”

Shared memory এর মাধ্যমে communicate করা দেখতে কেমন হবে? এছাড়াও, কেন message-passing উৎসাহীরা memory sharing ব্যবহার না করার বিষয়ে সতর্ক করেন?

একভাবে দেখলে, যেকোনো প্রোগ্রামিং ভাষার channel গুলো single ownership এর মতো, কারণ একবার আপনি কোনো value channel এর নিচে transfer করলে, আপনার সেই value টি আর ব্যবহার করা উচিত না। Shared memory concurrency হলো multiple ownership এর মতো: একাধিক থ্রেড একই সময়ে একই memory location অ্যাক্সেস করতে পারে। Chapter 15 এ আপনি যেমন দেখেছেন, যেখানে smart pointer multiple ownership সম্ভব করেছে, multiple ownership জটিলতা বাড়াতে পারে কারণ এই বিভিন্ন owner দের manage করার প্রয়োজন। Rust এর type system এবং ownership rules এই management সঠিকভাবে করার জন্য অনেক সাহায্য করে। উদাহরণস্বরূপ, mutexes দেখি, যা shared memory এর জন্য সবচেয়ে সাধারণ concurrency primitive গুলোর মধ্যে একটি।

### Using Mutexes to Allow Access to Data from One Thread at a Time

_Mutex_ হলো _mutual exclusion_ এর সংক্ষিপ্ত রূপ, যেমন, একটি mutex শুধুমাত্র একটি থ্রেডকে যেকোনো নির্দিষ্ট সময়ে কিছু data অ্যাক্সেস করার অনুমতি দেয়। Mutex এর data অ্যাক্সেস করার জন্য, একটি থ্রেডকে প্রথমে সংকেত দিতে হবে যে এটি mutex এর _lock_ acquire করার জন্য অনুরোধ করে অ্যাক্সেস করতে চায়। Lock হলো একটি data structure যা mutex এর অংশ এবং এটি track করে যে বর্তমানে কার data তে exclusive access আছে। তাই, mutex কে locking system এর মাধ্যমে এর ভেতরের data কে _guarding_ করা হিসাবে বর্ণনা করা হয়।

Mutexes ব্যবহার করা কঠিন বলে পরিচিত কারণ আপনাকে দুটি নিয়ম মনে রাখতে হবে:

- data ব্যবহার করার আগে আপনাকে অবশ্যই lock acquire করার চেষ্টা করতে হবে।
- যখন আপনি mutex guard করা data এর কাজ শেষ করে ফেলবেন, তখন আপনাকে অবশ্যই data unlock করতে হবে যাতে অন্যান্য thread lock acquire করতে পারে।

Mutex এর বাস্তব উদাহরণস্বরূপ, একটি conference এ panel discussion এর কথা ভাবুন যেখানে কেবল একটি microphone আছে। কোনো panelist কথা বলার আগে, তাদের microphone ব্যবহার করতে চাওয়ার জন্য জিজ্ঞাসা করতে বা সংকেত দিতে হবে। যখন তারা microphone টি পাবে, তখন তারা যতক্ষণ ইচ্ছে কথা বলতে পারবে এবং তারপর microphone টি পরবর্তী panelist এর কাছে হস্তান্তর করবে যে কথা বলতে চায়। যদি কোনো panelist microphone ব্যবহার করার পর হস্তান্তর করতে ভুলে যায়, তাহলে অন্য কেউ কথা বলতে পারবে না। যদি shared microphone এর management ভুল হয়ে যায়, তাহলে panel টি পরিকল্পনা অনুযায়ী কাজ করবে না!

Mutexes এর management সঠিকভাবে করা বেশ কঠিন হতে পারে, যার কারণে অনেকে channel এর ব্যাপারে বেশ আগ্রহী। তবে, Rust এর type system এবং ownership rules এর জন্য, locking এবং unlocking ভুল হওয়ার কোনো সুযোগ নেই।

#### The API of `Mutex<T>`

Mutex কিভাবে ব্যবহার করতে হয় তার উদাহরণ হিসেবে, চলুন single-threaded context এ একটি mutex ব্যবহার করে শুরু করি, যা Listing 16-12 এ দেখানো হয়েছে:

<Listing number="16-12" file-name="src/main.rs" caption="সহজতার জন্য একটি single-threaded context এ `Mutex<T>` এর API পরীক্ষা করা">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

</Listing>

অন্যান্য type এর মতো, আমরা `new` associated function ব্যবহার করে `Mutex<T>` তৈরি করি। Mutex এর ভিতরের data অ্যাক্সেস করতে, আমরা lock acquire করার জন্য `lock` method ব্যবহার করি। এই কলটি বর্তমান thread কে block করে দেবে, তাই lock পাওয়ার আগ পর্যন্ত এটি কোনো কাজ করতে পারবে না।

যদি অন্য কোনো থ্রেড lock ধরে panic করে, তাহলে `lock` call টি fail করবে। সেক্ষেত্রে, কেউ কখনো lock acquire করতে পারবে না, তাই আমরা `unwrap` করার সিদ্ধান্ত নিয়েছি এবং যদি আমরা সেই পরিস্থিতিতে থাকি তাহলে এই thread টি panic করবে।

Lock acquire করার পর, আমরা return value টি, এখানে `num` নামে আছে, ভিতরের data এর mutable reference হিসেবে ব্যবহার করতে পারি। Type system নিশ্চিত করে যে `m` এর value ব্যবহার করার আগে আমরা lock acquire করেছি। `m` এর type হলো `Mutex<i32>`, `i32` নয়, তাই `i32` value ব্যবহার করতে আমাদের অবশ্যই `lock` call করতে হবে। আমরা ভুলতে পারি না; type system অন্যথায় ভিতরের `i32` অ্যাক্সেস করতে দেবে না।

আপনি হয়তো সন্দেহ করতে পারেন, `Mutex<T>` একটি smart pointer। আরও সঠিকভাবে, `lock` call টি `LockResult` এ wrap করা `MutexGuard` নামক একটি smart pointer _return_ করে, যা আমরা `unwrap` call করে handle করেছি। `MutexGuard` smart pointer টি আমাদের ভিতরের data এর দিকে point করার জন্য `Deref` implement করে; smart pointer এর `Drop` implementation ও আছে যা automatic ভাবে lock release করে দেয় যখন `MutexGuard` scope এর বাইরে চলে যায়, যা inner scope এর শেষে ঘটে। ফলস্বরূপ, lock release করতে ভুলে গিয়ে mutex অন্য thread দ্বারা ব্যবহার করা থেকে block করার ঝুঁকি আমরা নেই না, কারণ lock release automatic ভাবে হয়।

Lock drop করার পর, আমরা mutex এর value print করতে পারি এবং দেখতে পারি যে আমরা ভিতরের `i32` কে 6 এ পরিবর্তন করতে পেরেছি।

#### Sharing a `Mutex<T>` Between Multiple Threads

এখন, `Mutex<T>` ব্যবহার করে একাধিক thread এর মধ্যে একটি value share করার চেষ্টা করি। আমরা ১০টি thread spin up করব এবং প্রত্যেকটিকে counter value 1 করে increment করতে বলব, যাতে counter 0 থেকে 10 এ যায়। Listing 16-13 এর পরবর্তী উদাহরণে compiler error থাকবে এবং `Mutex<T>` ব্যবহার করার এবং Rust কিভাবে সঠিকভাবে ব্যবহার করতে সাহায্য করে সে সম্পর্কে আরও জানতে আমরা সেই error টি ব্যবহার করব।

<Listing number="16-13" file-name="src/main.rs" caption="10টি থ্রেড, প্রত্যেকটি একটি `Mutex<T>` দ্বারা guard করা counter increment করে">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

</Listing>

আমরা `Mutex<T>` এর ভিতরে একটি `i32` রাখার জন্য `counter` variable তৈরি করি, যা আমরা Listing 16-12 তে করেছিলাম। এরপর, আমরা numbers এর range এর উপর iterate করে 10 টি thread তৈরি করি। আমরা `thread::spawn` ব্যবহার করি এবং সব thread কে একই closure দেই: একটি যা counter কে thread এর মধ্যে move করে, `lock` method call করে `Mutex<T>` এর উপর lock acquire করে, এবং তারপর mutex এর value এর সাথে 1 যোগ করে। যখন একটি thread এর closure এর execution শেষ হয়, তখন `num` scope এর বাইরে চলে যাবে এবং lock release করে দেবে যাতে অন্য একটি thread এটি acquire করতে পারে।

Main thread এ, আমরা সব join handle collect করি। তারপর, Listing 16-2 এ আমরা যা করেছিলাম, সে অনুযায়ী সব thread শেষ হয়েছে কিনা তা নিশ্চিত করার জন্য প্রত্যেক handle এর উপর `join` call করি। সেই সময়ে, main thread lock acquire করবে এবং এই প্রোগ্রামের ফলাফল print করবে।

আমরা ইঙ্গিত দিয়েছিলাম যে এই উদাহরণটি compile হবে না। এখন চলুন জেনে নেই কেন!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

Error message বলছে যে `counter` value টি loop এর আগের iteration এ move হয়ে গেছে। Rust আমাদের বলছে যে আমরা `counter` এর ownership একাধিক thread এ move করতে পারব না। Chapter 15 এ আলোচনা করা multiple-ownership method ব্যবহার করে compiler error টি ঠিক করি।

#### Multiple Ownership with Multiple Threads

Chapter 15 এ, আমরা reference counted value তৈরি করার জন্য smart pointer `Rc<T>` ব্যবহার করে একটি value কে multiple owner দিয়েছিলাম। চলুন এখানেও একই কাজ করি এবং দেখি কি হয়। Listing 16-14 এ আমরা `Mutex<T>` কে `Rc<T>` দিয়ে wrap করব এবং thread এ ownership move করার আগে `Rc<T>` clone করব।

<Listing number="16-14" file-name="src/main.rs" caption="একাধিক thread কে `Mutex<T>` এর owner হওয়ার অনুমতি দেওয়ার জন্য `Rc<T>` ব্যবহার করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

</Listing>

আবার, আমরা compile করি এবং... ভিন্ন error পাই! Compiler আমাদের অনেক কিছু শেখাচ্ছে।

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

বাহ, error message টি বেশ লম্বা! এখানে focus করার জন্য গুরুত্বপূর্ণ অংশটি হলো: `` `Rc<Mutex<i32>>` cannot be sent between threads safely ``. compiler আমাদের কারণও বলছে: `` the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``। আমরা পরবর্তী বিভাগে `Send` নিয়ে আলোচনা করব: এটি এমন একটি trait যা নিশ্চিত করে যে thread এর সাথে আমরা যে type গুলো ব্যবহার করি তা concurrent পরিস্থিতিতে ব্যবহারের জন্য তৈরি।

দুর্ভাগ্যবশত, `Rc<T>` thread এর মধ্যে share করার জন্য safe নয়। যখন `Rc<T>` reference count manage করে, তখন এটি `clone` এর প্রত্যেক call এর জন্য count এ যোগ করে এবং যখন প্রত্যেক clone drop করা হয় তখন count থেকে বিয়োগ করে। কিন্তু count এর পরিবর্তনগুলো অন্য কোনো thread দ্বারা interrupt করা যাবে না তা নিশ্চিত করার জন্য এটি কোনো concurrency primitive ব্যবহার করে না। এর কারণে ভুল count হতে পারে—ছোটখাটো bug যা memory leak বা value drop হওয়ার কারণ হতে পারে যা শেষ হওয়ার আগে। আমাদের ঠিক `Rc<T>` এর মতো একটি type দরকার কিন্তু এটি thread-safe উপায়ে reference count এর পরিবর্তন করে।

#### Atomic Reference Counting with `Arc<T>`

সৌভাগ্যবশত, `Arc<T>` হলো `Rc<T>` এর মতো একটি type যা concurrent পরিস্থিতিতে ব্যবহার করার জন্য safe। _a_ দ্বারা বোঝানো হয়েছে _atomic_, মানে হলো এটি একটি _atomically reference counted_ type। Atomics হলো এক ধরনের concurrency primitive যা আমরা এখানে বিস্তারিত আলোচনা করব না: আরও বিস্তারিত জানার জন্য standard library documentation এর [`std::sync::atomic`][atomic]<!-- ignore --> দেখুন। এই মুহূর্তে, আপনাকে শুধু জানতে হবে যে atomics primitive type এর মতো কাজ করে কিন্তু thread এর মধ্যে share করার জন্য safe।

তাহলে আপনি হয়তো ভাবছেন কেন সব primitive type atomic নয় এবং কেন standard library type গুলো default ভাবে `Arc<T>` ব্যবহার করার জন্য implement করা হয়নি। কারণ হলো thread safety এর সাথে performance penalty আসে যা আপনি তখনই দিতে চান যখন আপনার সত্যিই দরকার। যদি আপনি কোনো single thread এর মধ্যে value নিয়ে কাজ করেন, তাহলে atomics যে guarantee দেয় তা enforce না করলে আপনার code দ্রুত run হতে পারে।

চলুন আমাদের উদাহরণে ফিরে যাই: `Arc<T>` এবং `Rc<T>` এর API একই, তাই আমরা `use` line, `new` এর call এবং `clone` এর call পরিবর্তন করে আমাদের প্রোগ্রাম ঠিক করি। Listing 16-15 এর code অবশেষে compile এবং run হবে:

<Listing number="16-15" file-name="src/main.rs" caption="একাধিক thread এর মধ্যে ownership share করার জন্য `Mutex<T>` কে wrap করতে একটি `Arc<T>` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

</Listing>

এই code টি নিচের ফলাফল print করবে:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Result: 10
```

আমরা সফল হয়েছি! আমরা 0 থেকে 10 পর্যন্ত গুনেছি, যা খুব বেশি গুরুত্বপূর্ণ মনে নাও হতে পারে, কিন্তু এটি আমাদের `Mutex<T>` এবং thread safety সম্পর্কে অনেক কিছু শিখিয়েছে। আপনি এই প্রোগ্রামের structure ব্যবহার করে একটি counter increment করার চেয়ে আরও জটিল কাজ করতে পারেন। এই strategy ব্যবহার করে, আপনি একটি calculation কে independent অংশে ভাগ করতে পারেন, সেই অংশগুলোকে thread এর মধ্যে ভাগ করে দিতে পারেন, এবং তারপর প্রত্যেক thread কে তাদের অংশ দিয়ে final result update করার জন্য `Mutex<T>` ব্যবহার করতে পারেন।

লক্ষ্য করুন যে যদি আপনি সাধারণ numerical operation করেন, তাহলে [`std::sync::atomic` module of the standard library][atomic]<!-- ignore --> দ্বারা দেওয়া `Mutex<T>` type এর চেয়ে আরও সহজ type আছে। এই type গুলো primitive type এর জন্য safe, concurrent, atomic access প্রদান করে। আমরা এই উদাহরণে একটি primitive type এর সাথে `Mutex<T>` ব্যবহার করার সিদ্ধান্ত নিয়েছি যাতে আমরা `Mutex<T>` কিভাবে কাজ করে সেদিকে মনোযোগ দিতে পারি।

### Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

আপনি হয়তো লক্ষ্য করেছেন যে `counter` immutable কিন্তু আমরা এর ভিতরের value এর mutable reference পেতে পারি; এর মানে হলো `Mutex<T>` interior mutability প্রদান করে, যেমন `Cell` family করে। Chapter 15 এ আমরা যেভাবে `Rc<T>` এর ভিতরের content mutate করার অনুমতি দেওয়ার জন্য `RefCell<T>` ব্যবহার করেছিলাম, তেমনিভাবে আমরা `Arc<T>` এর ভিতরের content mutate করার জন্য `Mutex<T>` ব্যবহার করি।

আরেকটি বিষয় যা মনে রাখতে হবে তা হলো `Mutex<T>` ব্যবহার করার সময় Rust আপনাকে সব ধরনের logic error থেকে রক্ষা করতে পারবে না। Chapter 15 এ মনে করুন যে `Rc<T>` ব্যবহার করার সাথে reference cycle তৈরি হওয়ার ঝুঁকি ছিল, যেখানে দুটি `Rc<T>` value একে অপরের reference দেয়, যার কারণে memory leak হয়। একইভাবে, `Mutex<T>` এর সাথে _deadlock_ তৈরি হওয়ার ঝুঁকি থাকে। Deadlock তখন ঘটে যখন একটি operation দুটি resource lock করার প্রয়োজন হয় এবং দুটি thread প্রত্যেকটি lock এর একটি acquire করে, যার কারণে তারা একে অপরের জন্য সারাজীবন অপেক্ষা করে। যদি আপনি deadlock সম্পর্কে আগ্রহী হন, তাহলে একটি Rust প্রোগ্রাম তৈরি করার চেষ্টা করুন যেখানে deadlock আছে; তারপর যেকোনো ভাষায় mutex এর জন্য deadlock mitigation strategies নিয়ে research করুন এবং Rust এ implement করার চেষ্টা করুন। `Mutex<T>` এবং `MutexGuard` এর standard library API documentation এ কাজের তথ্য দেওয়া আছে।

আমরা এই chapter টি শেষ করব `Send` এবং `Sync` trait নিয়ে কথা বলার মাধ্যমে এবং কিভাবে আমরা custom type এর সাথে এগুলো ব্যবহার করতে পারি।

[atomic]: ../std/sync/atomic/index.html
```
