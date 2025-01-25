# Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams

কম্পিউটারকে আমরা যেসব operation করতে বলি, তার অনেকগুলো শেষ হতে বেশ খানিকটা সময় নিতে পারে। যদি আমরা সেই দীর্ঘ সময় ধরে চলা process গুলোর শেষ হওয়ার জন্য অপেক্ষা করার সময় অন্য কিছু করতে পারতাম, তাহলে ভালো হতো। আধুনিক কম্পিউটার একই সময়ে একাধিক operation নিয়ে কাজ করার জন্য দুটি কৌশল দেয়: parallelism এবং concurrency। যখন আমরা parallel বা concurrent operation যুক্ত program লেখা শুরু করি, তখন আমরা খুব দ্রুত _asynchronous programming_ এর নতুন চ্যালেঞ্জগুলোর সম্মুখীন হই, যেখানে operation গুলো শুরু করার ক্রমানুসারে শেষ নাও হতে পারে। এই chapter টি Chapter 16 এর parallelism এবং concurrency এর জন্য thread ব্যবহার করার উপর ভিত্তি করে asynchronous programming এর বিকল্প পদ্ধতি উপস্থাপন করে: Rust এর Futures, Streams, `async` এবং `await` syntax যা তাদের সমর্থন করে এবং asynchronous operation গুলোর মধ্যে manage ও coordinate করার tool গুলো নিয়ে আলোচনা করা হয়েছে।

আসুন একটি উদাহরণ বিবেচনা করি। ধরুন আপনি আপনার পরিবারের একটি celebration এর video export করছেন, এই operation টি শেষ হতে কয়েক মিনিট থেকে কয়েক ঘণ্টা পর্যন্ত সময় লাগতে পারে। Video export CPU এবং GPU এর যতটা সম্ভব power ব্যবহার করবে। যদি আপনার কেবল একটি CPU core থাকত এবং আপনার operating system export শেষ না হওয়া পর্যন্ত pause না করত—মানে, যদি এটি export কে _synchronously_ execute করত—তাহলে সেই task run হওয়ার সময় আপনি আপনার কম্পিউটারে আর কিছুই করতে পারতেন না। এটা বেশ হতাশাজনক অভিজ্ঞতা হতো। সৌভাগ্যবশত, আপনার কম্পিউটারের operating system export কে অদৃশ্যভাবে interrupt করতে পারে যাতে আপনি একই সাথে অন্য কাজ করতে পারেন।

এখন ধরুন আপনি অন্য কারো share করা একটি video download করছেন, এতেও বেশ সময় লাগতে পারে কিন্তু এটি CPU এর বেশি সময় নেয় না। এই ক্ষেত্রে, CPU কে network থেকে data আসার জন্য অপেক্ষা করতে হয়। একবার data আসা শুরু হলে আপনি data পড়া শুরু করতে পারেন, তবে এর সবটা আসতে কিছু সময় লাগতে পারে। Data সম্পূর্ণভাবে আসার পরেও, যদি video টি অনেক বড় হয়, তাহলে এটি load হতে অন্তত এক বা দুই সেকেন্ড লাগতে পারে। এটা খুব বেশি মনে নাও হতে পারে, কিন্তু আধুনিক processor এর জন্য এটা অনেক বেশি সময়, যা প্রতি সেকেন্ডে কয়েক বিলিয়ন operation করতে পারে। আবারও, আপনার operating system আপনার program কে অদৃশ্যভাবে interrupt করবে যাতে network call শেষ হওয়ার জন্য অপেক্ষা করার সময় CPU অন্য কাজ করতে পারে।

Video export হলো _CPU-bound_ বা _compute-bound_ operation এর উদাহরণ। এটি CPU বা GPU এর মধ্যে কম্পিউটারের data processing speed এর potential এবং সেই speed এর কতটা operation এর জন্য dedicate করতে পারে তার দ্বারা সীমাবদ্ধ। Video download হলো _IO-bound_ operation এর উদাহরণ, কারণ এটি কম্পিউটারের _input and output_ এর speed দ্বারা সীমাবদ্ধ; এটি network এর মাধ্যমে data send হওয়ার speed এর চেয়ে বেশি দ্রুত হতে পারে না।

এই দুটি উদাহরণেই, operating system এর অদৃশ্য interrupt গুলো concurrency এর একটি রূপ প্রদান করে। সেই concurrency শুধুমাত্র পুরো program এর level এ ঘটে: operating system অন্য program গুলোকে কাজ করার সুযোগ দেওয়ার জন্য একটি program কে interrupt করে। বেশিরভাগ ক্ষেত্রে, যেহেতু আমরা operating system এর চেয়ে অনেক বেশি granular level এ আমাদের program বুঝি, তাই আমরা concurrency এর এমন সুযোগগুলো খুঁজে বের করতে পারি যা operating system দেখতে পায় না।

উদাহরণস্বরূপ, যদি আমরা file download manage করার জন্য একটি tool তৈরি করি, তাহলে আমাদের program এমনভাবে লিখতে হবে যাতে একটি download শুরু করলে UI lock না হয়ে যায়, এবং ব্যবহারকারীরা একই সময়ে একাধিক download শুরু করতে পারে। Network এর সাথে interact করার জন্য অনেক operating system API _blocking_ হয়; অর্থাৎ, যে data তারা process করছে সেটি সম্পূর্ণভাবে ready না হওয়া পর্যন্ত তারা program এর progress block করে দেয়।

> Note: আপনি যদি একটু ভেবে দেখেন, তাহলে দেখবেন _বেশিরভাগ_ function call এভাবেই কাজ করে। তবে, _blocking_ শব্দটি সাধারণত function call গুলোর জন্য ব্যবহার করা হয় যেগুলো file, network, বা কম্পিউটারের অন্যান্য resource এর সাথে interact করে, কারণ সেই ক্ষেত্রে operation টি _non_-blocking হলে individual program এর সুবিধা হবে।

আমরা প্রত্যেক file download করার জন্য dedicated thread spawn করে আমাদের main thread block করা এড়াতে পারি। তবে, সেই thread গুলোর overhead একটা সময় সমস্যা হয়ে দাঁড়াবে। এটা ভালো হতো যদি call টি প্রথমে block না করত। এটা আরও ভালো হতো যদি আমরা blocking code এর মতো একই direct style এ লিখতে পারতাম, যেমনটা নিচে দেখানো হয়েছে:

```rust,ignore,does_not_compile
let data = fetch_data_from(url).await;
println!("{data}");
```

Rust এর _async_ (যার মানে _asynchronous_) abstraction আমাদের ঠিক সেটাই দেয়। এই chapter এ, async সম্পর্কে আপনি সবকিছু জানতে পারবেন যখন আমরা নিচের বিষয়গুলো নিয়ে আলোচনা করব:

- কিভাবে Rust এর `async` এবং `await` syntax ব্যবহার করতে হয়
- Chapter 16 এ আমরা যে challenge গুলো দেখেছিলাম তার কিছু সমাধান করার জন্য async model কিভাবে ব্যবহার করতে হয়
- কিভাবে multithreading এবং async একে অপরের পরিপূরক সমাধান দেয়, যা আপনি অনেক ক্ষেত্রে combine করতে পারেন

Async বাস্তবে কিভাবে কাজ করে তা দেখার আগে, parallelism এবং concurrency এর মধ্যে পার্থক্য নিয়ে আলোচনা করার জন্য আমাদের একটু থামতে হবে।

### Parallelism and Concurrency

আমরা এতক্ষণ parallelism এবং concurrency কে প্রায় একই অর্থে ব্যবহার করেছি। এখন আমাদের তাদের মধ্যে আরও ভালোভাবে পার্থক্য করতে হবে, কারণ কাজ শুরু করার সাথে সাথে পার্থক্যগুলো দেখা যাবে।

একটি software project এ একটি team কিভাবে কাজ ভাগ করে নিতে পারে তার বিভিন্ন উপায় বিবেচনা করুন। আপনি একজন single member কে একাধিক task assign করতে পারেন, প্রত্যেক member কে একটি task assign করতে পারেন, অথবা দুটি পদ্ধতির সংমিশ্রণ ব্যবহার করতে পারেন।

যখন একজন ব্যক্তি কোনো task শেষ করার আগে বেশ কয়েকটি ভিন্ন task নিয়ে কাজ করে, তখন সেটা হলো _concurrency_। হয়তো আপনার কম্পিউটারে দুটি ভিন্ন project checkout করা আছে, এবং যখন আপনি একটি project এ কাজ করতে করতে বিরক্ত হয়ে যান বা আটকে যান, তখন আপনি অন্যটিতে switch করেন। আপনি একজন মানুষ, তাই আপনি একই সময়ে দুটি task এ progress করতে পারবেন না, কিন্তু আপনি multi-tasking করতে পারেন, একটির পর একটি switch করে progress করতে পারেন (চিত্র 17-1 দেখুন)।

<figure>

<img src="img/trpl17-01.svg" class="center" alt="A diagram with boxes labeled Task A and Task B, with diamonds in them representing subtasks. There are arrows pointing from A1 to B1, B1 to A2, A2 to B2, B2 to A3, A3 to A4, and A4 to B3. The arrows between the subtasks cross the boxes between Task A and Task B." />

<figcaption>Figure 17-1: একটি concurrent workflow, Task A এবং Task B এর মধ্যে switch করা</figcaption>

</figure>

যখন team task গুলোকে group করে প্রত্যেক member কে একটি করে task নিয়ে একা একা কাজ করতে দেয়, তখন সেটা হলো _parallelism_। Team এর প্রত্যেক সদস্য একই সময়ে progress করতে পারে (চিত্র 17-2 দেখুন)।

<figure>

<img src="img/trpl17-02.svg" class="center" alt="A diagram with boxes labeled Task A and Task B, with diamonds in them representing subtasks. There are arrows pointing from A1 to A2, A2 to A3, A3 to A4, B1 to B2, and B2 to B3. No arrows cross between the boxes for Task A and Task B." />

<figcaption>Figure 17-2: একটি parallel workflow, যেখানে Task A এবং Task B তে স্বাধীনভাবে কাজ হয়</figcaption>

</figure>

এই দুটি workflow তেই, আপনাকে হয়তো বিভিন্ন task এর মধ্যে coordinate করতে হতে পারে। হয়তো আপনি _ভেবেছিলেন_ যে কোনো একজনের উপর assign করা task অন্য সবার কাজ থেকে সম্পূর্ণ স্বাধীন, কিন্তু আসলে তার team এর অন্য একজনের কাজ শেষ করার উপর নির্ভর করে। কিছু কাজ parallel ভাবে করা যেতে পারত, কিন্তু কিছু কাজ আসলে _serial_ ছিল: এটি একটির পর একটি task হিসেবে হতে পারত, যেমনটা চিত্র 17-3 এ দেখানো হয়েছে।

<figure>

<img src="img/trpl17-03.svg" class="center" alt="A diagram with boxes labeled Task A and Task B, with diamonds in them representing subtasks. There are arrows pointing from A1 to A2, A2 to a pair of thick vertical lines like a “pause” symbol, from that symbol to A3, B1 to B2, B2 to B3, which is below that symbol, B3 to A3, and B3 to B4." />

<figcaption>Figure 17-3: একটি আংশিকভাবে parallel workflow, যেখানে Task A এবং Task B তে স্বাধীনভাবে কাজ হয় যতক্ষণ না Task A3 Task B3 এর ফলাফলের উপর block হয়ে যায়।</figcaption>

</figure>

একইভাবে, আপনি হয়তো বুঝতে পারবেন যে আপনার নিজের একটি task আপনার অন্য একটি task এর উপর নির্ভরশীল। এখন আপনার concurrent কাজ ও serial হয়ে গেছে।

Parallelism এবং concurrency একে অপরের সাথে intersect ও করতে পারে। আপনি যদি জানতে পারেন যে আপনার কোনো colleague আপনার একটি task শেষ না করা পর্যন্ত আটকে আছে, তাহলে আপনি সম্ভবত আপনার সব effort সেই task এর উপর focus করবেন যাতে আপনার colleague "unblock" হতে পারে। আপনি এবং আপনার colleague আর parallel ভাবে কাজ করতে পারবেন না, এবং আপনি নিজের task এ concurrent ভাবেও কাজ করতে পারবেন না।

Software এবং hardware এর ক্ষেত্রেও একই basic dynamic কাজ করে। Single CPU core এর একটি machine এ, CPU একবারে শুধুমাত্র একটি operation করতে পারে, কিন্তু এটি concurrent ভাবে কাজ করতে পারে। Thread, process, এবং async এর মতো tool ব্যবহার করে, কম্পিউটার একটি activity pause করতে পারে এবং অন্য activity তে switch করতে পারে এবং শেষ পর্যন্ত আবার প্রথম activity তে ফিরে আসতে পারে। Multiple CPU core এর একটি machine এ, এটি parallel ভাবেও কাজ করতে পারে। একটি core একটি task perform করতে পারে, যেখানে অন্য core একটি সম্পূর্ণ সম্পর্কহীন task perform করতে পারে, এবং সেই operation গুলো একই সময়ে ঘটে।

Rust এ async নিয়ে কাজ করার সময়, আমরা সবসময় concurrency নিয়ে কাজ করি। Hardware, operating system, এবং async runtime (async runtime নিয়ে শীঘ্রই আরও আলোচনা করা হবে) এর উপর নির্ভর করে, সেই concurrency under the hood parallelism ও ব্যবহার করতে পারে।

এখন, Rust এ async programming আসলে কিভাবে কাজ করে তা নিয়ে আলোচনা করা যাক।
