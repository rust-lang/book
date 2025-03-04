## Test-Driven Development ব্যবহার করে Library-র Functionality Develop করা

এখন যেহেতু আমরা logic-টিকে _src/lib.rs_-এ extract করেছি এবং argument সংগ্রহ ও error handling _src/main.rs_-এ রেখেছি, তাই আমাদের code-এর core functionality-র জন্য test লেখা অনেক সহজ। আমরা command line থেকে আমাদের binary call না করেই বিভিন্ন argument দিয়ে সরাসরি function call করতে পারি এবং return value গুলো check করতে পারি।

এই section-এ, আমরা নিম্নলিখিত step-গুলো সহ test-driven development (TDD) process ব্যবহার করে `minigrep` প্রোগ্রামে searching logic যোগ করব:

1.  এমন একটি test লিখুন যেটি fail করে এবং আপনি যে কারণে এটি fail করবে বলে আশা করছেন সেই কারণেই fail করছে কিনা তা নিশ্চিত করতে এটি run করুন।
2.  নতুন test-টি pass করানোর জন্য যথেষ্ট code লিখুন বা modify করুন।
3.  আপনি যে code যোগ করেছেন বা পরিবর্তন করেছেন সেটি refactor করুন এবং নিশ্চিত করুন যে test গুলো தொடர்ந்து pass করছে।
4.  Step 1 থেকে পুনরাবৃত্তি করুন!

যদিও software লেখার এটি অন্যতম একটি উপায়, TDD কোড ডিজাইনকে এগিয়ে নিতে সাহায্য করতে পারে। Test pass করানোর code লেখার আগে test লিখলে প্রক্রিয়া জুড়ে high test coverage বজায় রাখতে সহায়তা করে।

আমরা সেই functionality-র implementation test-drive করব যেটি file-এর contents-এ query string-টির জন্য search করবে এবং query-এর সাথে match করে এমন line-গুলোর একটি list তৈরি করবে। আমরা এই functionality-টি `search` নামক একটি function-এ যোগ করব।

### একটি Failing Test লেখা

যেহেতু আমাদের আর প্রয়োজন নেই, তাই আসুন _src/lib.rs_ এবং _src/main.rs_ থেকে `println!` statement গুলো সরিয়ে দিই যেগুলো আমরা প্রোগ্রামের behavior check করার জন্য ব্যবহার করতাম। তারপর, _src/lib.rs_-এ, আমরা একটি `tests` module যোগ করব একটি test function সহ, যেমনটি আমরা [Chapter 11][ch11-anatomy]-এ করেছিলাম। Test function টি specify করে যে `search` function-টির behavior আমরা কেমন চাই: এটি একটি query এবং যে text-এ search করতে হবে সেটি নেবে এবং text-এর শুধুমাত্র সেই line গুলো return করবে যেগুলিতে query রয়েছে। Listing 12-15 এই test টি দেখায়, যেটি এখনও compile হবে না।

<Listing number="12-15" file-name="src/lib.rs" caption="`search` ফাংশনের জন্য একটি failing test তৈরি করা যা আমরা চাই">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

</Listing>

এই test টি `"duct"` string-টির জন্য search করে। আমরা যে text-এ search করছি সেটি তিনটি লাইন, যার মধ্যে শুধুমাত্র একটিতে `"duct"` রয়েছে (লক্ষ্য করুন যে opening double quote-এর পরের backslash টি Rust-কে বলে এই string literal-এর contents-এর শুরুতে একটি newline character না রাখতে)। আমরা assert করি যে `search` function থেকে returned value-টিতে শুধুমাত্র সেই line-টি রয়েছে যা আমরা আশা করি।

আমরা এখনও এই test টি run করে fail হতে দেখতে পাচ্ছি না কারণ test টি এখনও compile-ই হচ্ছে না: `search` function-টি এখনও নেই! TDD নীতি অনুসারে, আমরা Listing 12-16-এ দেখানো `search` function-এর একটি definition যোগ করে test টিকে compile এবং run করানোর জন্য যথেষ্ট code যোগ করব, যেটি সব সময় একটি empty vector return করে। তারপরে test টি compile হয়ে fail করা উচিত, কারণ একটি empty vector `"safe, fast, productive."` line-যুক্ত একটি vector-এর সাথে মেলে না।

<Listing number="12-16" file-name="src/lib.rs" caption="`search` ফাংশনের যথেষ্ট সংজ্ঞা দেওয়া যাতে আমাদের test compile হয়">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

</Listing>

লক্ষ্য করুন যে আমাদের `search`-এর signature-এ একটি explicit lifetime `'a` define করতে হবে এবং সেই lifetime-টি `contents` argument এবং return value-এর সাথে ব্যবহার করতে হবে। [Chapter 10][ch10-lifetimes]-এ স্মরণ করুন যে lifetime parameter গুলো specify করে যে কোন argument lifetime, return value-এর lifetime-এর সাথে connected। এই ক্ষেত্রে, আমরা indicate করছি যে returned vector-টিতে string slice থাকা উচিত যা `contents` argument-এর slice-গুলোকে reference করে ( `query` argument-এর নয়)।

অন্য কথায়, আমরা Rust-কে বলি যে `search` function দ্বারা returned data ততদিন live থাকবে যতদিন `contents` argument-এ `search` function-এ pass করা data live থাকে। এটা গুরুত্বপূর্ণ! একটি slice দ্বারা referenced data-টিকে valid হতে হবে যাতে reference-টি valid হয়; যদি compiler ধরে নেয় যে আমরা `contents`-এর পরিবর্তে `query`-এর string slice তৈরি করছি, তাহলে এটি তার safety checking ভুলভাবে করবে।

যদি আমরা lifetime annotation গুলো ভুলে যাই এবং এই function টি compile করার চেষ্টা করি, তাহলে আমরা এই error পাব:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust-এর পক্ষে জানা সম্ভব নয় যে আমাদের দুটি argument-এর মধ্যে কোনটি প্রয়োজন, তাই আমাদের এটি explicit ভাবে বলতে হবে। যেহেতু `contents` হল সেই argument যাতে আমাদের সমস্ত text রয়েছে এবং আমরা সেই text-এর যে অংশগুলো match করে সেগুলো return করতে চাই, তাই আমরা জানি `contents` হল সেই argument যাকে lifetime syntax ব্যবহার করে return value-এর সাথে connect করা উচিত।

অন্যান্য programming language-গুলোতে আপনাকে signature-এ argument গুলোকে return value-এর সাথে connect করতে হয় না, কিন্তু এই practice টি সময়ের সাথে সহজ হয়ে যাবে। আপনি এই example টিকে Chapter 10-এর ["লাইফটাইম সহ রেফারেন্স ভ্যালিডেট করা"][validating-references-with-lifetimes] বিভাগের উদাহরণগুলোর সাথে তুলনা করতে পারেন।

এখন আসুন test টি run করি:

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

দারুণ, test fail করেছে, ঠিক যেমনটি আমরা আশা করেছিলাম। আসুন test টিকে pass করাই!

### Test Pass করার জন্য Code লেখা

বর্তমানে, আমাদের test fail করছে কারণ আমরা সব সময় একটি empty vector return করি। সেটি ঠিক করতে এবং `search` implement করতে, আমাদের প্রোগ্রামকে নিম্নলিখিত step গুলো follow করতে হবে:

1.  Contents-এর প্রতিটি line-এর মধ্যে iterate করা।
2.  Line-টিতে আমাদের query string আছে কিনা তা check করা।
3.  যদি থাকে, তাহলে আমরা যে value-গুলোর list return করছি তাতে এটি যোগ করা।
4.  যদি না থাকে, তাহলে কিছু না করা।
5.  Match করা result-গুলোর list return করা।

আসুন প্রতিটি step-এর মধ্যে দিয়ে কাজ করি, line-গুলোর মধ্যে iterate করা দিয়ে শুরু করি।

#### `lines` Method-এর সাহায্যে Line-গুলোর মধ্যে Iterate করা

Rust-এ string-গুলোর line-by-line iteration handle করার জন্য একটি সহায়ক method রয়েছে, যার সুবিধাজনকভাবে নাম দেওয়া হয়েছে `lines`, যেটি Listing 12-17-এ দেখানো পদ্ধতিতে কাজ করে। মনে রাখবেন এটি এখনও compile হবে না।

<Listing number="12-17" file-name="src/lib.rs" caption="`contents`-এর প্রতিটি লাইনের মধ্যে Iterate করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

</Listing>

`lines` method টি একটি iterator return করে। আমরা [Chapter 13][ch13-iterators]-এ iterator সম্পর্কে গভীরভাবে আলোচনা করব, কিন্তু স্মরণ করুন যে আপনি [Listing 3-5][ch3-iter]-এ iterator ব্যবহার করার এই উপায়টি দেখেছিলেন, যেখানে আমরা একটি collection-এর প্রতিটি item-এ কিছু code run করার জন্য একটি iterator-এর সাথে একটি `for` loop ব্যবহার করেছিলাম।

#### প্রতিটি Line-এ Query-র জন্য Search করা

এরপরে, আমরা check করব যে current line-টিতে আমাদের query string রয়েছে কিনা। সৌভাগ্যবশত, string-গুলোতে `contains` নামক একটি সহায়ক method রয়েছে যা আমাদের জন্য এটি করে! Listing 12-18-এ দেখানো `search` function-এ `contains` method-টিতে একটি call যোগ করুন। মনে রাখবেন এটি এখনও compile হবে না।

<Listing number="12-18" file-name="src/lib.rs" caption="`query`-তে থাকা string টি line-টিতে আছে কিনা তা দেখার জন্য functionality যোগ করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

</Listing>

এই মুহূর্তে, আমরা functionality তৈরি করছি। Code-টিকে compile করাতে, আমাদের body থেকে একটি value return করতে হবে যেমনটি আমরা function signature-এ indicate করেছিলাম।

#### Matching Line গুলো Store করা

এই function টি শেষ করতে, আমাদের matching line গুলো store করার একটি উপায় প্রয়োজন যা আমরা return করতে চাই। এর জন্য, আমরা `for` loop-এর আগে একটি mutable vector তৈরি করতে পারি এবং vector-এ একটি `line` store করার জন্য `push` method call করতে পারি। `for` loop-এর পরে, আমরা vector টি return করি, যেমনটি Listing 12-19-এ দেখানো হয়েছে।

<Listing number="12-19" file-name="src/lib.rs" caption="যে line গুলো match করে সেগুলো store করা যাতে আমরা সেগুলো return করতে পারি">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

</Listing>

এখন `search` function-টির শুধুমাত্র সেই line গুলো return করা উচিত যেগুলোতে `query` রয়েছে এবং আমাদের test pass করা উচিত। আসুন test টি run করি:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

আমাদের test pass করেছে, তাই আমরা জানি এটি কাজ করছে!

এই সময়ে, আমরা search function-এর implementation refactor করার সুযোগগুলো বিবেচনা করতে পারি, test গুলোকে pass করিয়ে একই functionality বজায় রেখে। Search function-এর code খুব খারাপ নয়, কিন্তু এটি iterator-এর কিছু useful feature-এর সুবিধা নেয় না। আমরা [Chapter 13][ch13-iterators]-এ এই example-এ ফিরে আসব, যেখানে আমরা iterator-গুলো বিস্তারিতভাবে explore করব এবং দেখব কীভাবে এটিকে improve করা যায়।

#### `run` Function-এ `search` Function ব্যবহার করা

এখন যেহেতু `search` function টি কাজ করছে এবং tested, তাই আমাদের `run` function থেকে `search` call করতে হবে। আমাদের `config.query` value এবং `contents` যা `run` file থেকে read করে, সেটিকে `search` function-এ pass করতে হবে। তারপর `run`, `search` থেকে returned প্রতিটি line প্রিন্ট করবে:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

আমরা এখনও `search` থেকে প্রতিটি line return করতে এবং print করতে একটি `for` loop ব্যবহার করছি।

এখন পুরো প্রোগ্রামটি কাজ করা উচিত! আসুন এটি পরীক্ষা করে দেখি, প্রথমে এমন একটি শব্দ দিয়ে যেটি Emily Dickinson-এর কবিতা থেকে ঠিক একটি line return করবে: _frog_।

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

দারুণ! এখন আসুন এমন একটি শব্দ try করি যা multiple line-এর সাথে match করবে, যেমন _body_:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

এবং অবশেষে, আসুন নিশ্চিত করি যে আমরা যখন এমন একটি শব্দের জন্য search করি যা কবিতার কোথাও নেই, যেমন _monomorphization_, তখন আমরা কোনো line পাই না:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

চমৎকার! আমরা একটি classic tool-এর নিজস্ব mini version তৈরি করেছি এবং application গুলোকে কীভাবে structure করতে হয় সে সম্পর্কে অনেক কিছু শিখেছি। আমরা file input এবং output, lifetime, testing এবং command line parsing সম্পর্কেও কিছুটা শিখেছি।

এই project-টি সম্পূর্ণ করার জন্য, আমরা সংক্ষেপে দেখাব কীভাবে environment variable-গুলোর সাথে কাজ করতে হয় এবং কীভাবে standard error-এ print করতে হয়, উভয়ই দরকারী যখন আপনি command line program লেখেন।

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html
