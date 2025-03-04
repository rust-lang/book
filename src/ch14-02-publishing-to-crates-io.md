## Crates.io-তে একটি Crate পাবলিশ করা

আমরা আমাদের প্রোজেক্টের dependencies হিসেবে [crates.io](https://crates.io/) থেকে প্যাকেজ ব্যবহার করেছি, কিন্তু আপনি আপনার নিজের প্যাকেজ publish করে অন্যদের সাথে আপনার কোড share করতে পারেন। [Crates.io](https://crates.io/)-এর crate registry আপনার প্যাকেজগুলোর source code distribute করে, তাই এটি primarily open source কোড হোস্ট করে।

Rust এবং Cargo-তে এমন feature রয়েছে যা আপনার published package-কে অন্যদের জন্য খুঁজে পাওয়া এবং ব্যবহার করা সহজ করে তোলে। আমরা এরপরে এই feature গুলোর মধ্যে কয়েকটি নিয়ে আলোচনা করব এবং তারপর একটি package কীভাবে publish করতে হয় তা ব্যাখ্যা করব।

### প্রয়োজনীয় ডকুমেন্টেশন কমেন্ট তৈরি করা

আপনার প্যাকেজগুলো সঠিকভাবে document করা অন্যান্য user-দের জানতে সাহায্য করবে যে সেগুলো কীভাবে এবং কখন ব্যবহার করতে হবে, তাই ডকুমেন্টেশন লেখার জন্য সময় ব্যয় করা উচিত। Chapter 3-তে, আমরা আলোচনা করেছি কীভাবে দুটি স্ল্যাশ, `//` ব্যবহার করে Rust কোডে comment করতে হয়। Rust-এ ডকুমেন্টেশনের জন্য একটি বিশেষ ধরনের comment-ও রয়েছে, সুবিধাজনকভাবে _ডকুমেন্টেশন কমেন্ট_ নামে পরিচিত, যা HTML ডকুমেন্টেশন generate করবে। HTML ডকুমেন্টেশন public API item-গুলোর জন্য ডকুমেন্টেশন কমেন্টের contents প্রদর্শন করে, যা সেইসব প্রোগ্রামারদের জন্য উদ্দিষ্ট যারা আপনার crate কীভাবে _ব্যবহার_ করতে হয় তা জানতে আগ্রহী, আপনার crate কীভাবে _ইমপ্লিমেন্ট_ করা হয়েছে তার বিপরীতে।

ডকুমেন্টেশন কমেন্টগুলো দুটির পরিবর্তে তিনটি স্ল্যাশ, `///` ব্যবহার করে এবং text format করার জন্য Markdown notation সাপোর্ট করে। ডকুমেন্টেশন কমেন্টগুলো যে item-গুলোকে ডকুমেন্ট করছে তার ঠিক আগে রাখুন। Listing 14-1 `my_crate` নামের একটি ক্রেটে `add_one` ফাংশনের জন্য ডকুমেন্টেশন কমেন্ট দেখায়।

<Listing number="14-1" file-name="src/lib.rs" caption="একটি ফাংশনের জন্য ডকুমেন্টেশন কমেন্ট">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

</Listing>

এখানে, আমরা `add_one` ফাংশনটি কী করে তার একটি description দিই, `Examples` heading দিয়ে একটি section শুরু করি এবং তারপর `add_one` ফাংশনটি কীভাবে ব্যবহার করতে হয় তা প্রদর্শন করে এমন কোড provide করি। আমরা `cargo doc` চালিয়ে এই ডকুমেন্টেশন কমেন্ট থেকে HTML ডকুমেন্টেশন generate করতে পারি। এই কমান্ডটি Rust-এর সাথে distribute করা `rustdoc` টুলটি চালায় এবং generated HTML ডকুমেন্টেশনটি _target/doc_ ডিরেক্টরিতে রাখে।

সুবিধার জন্য, `cargo doc --open` চালালে আপনার current crate-এর ডকুমেন্টেশনের জন্য HTML build হবে (সেইসাথে আপনার crate-এর সমস্ত dependency-এর ডকুমেন্টেশনও) এবং result টি একটি ওয়েব ব্রাউজারে open হবে। `Add_one` ফাংশনে নেভিগেট করুন এবং আপনি দেখতে পাবেন কীভাবে ডকুমেন্টেশন কমেন্টের text গুলো render করা হয়েছে, যেমনটি Figure 14-1-এ দেখানো হয়েছে:

<img alt="`my_crate`-এর `add_one` ফাংশনের জন্য Rendered HTML ডকুমেন্টেশন" src="img/trpl14-01.png" class="center" />

<span class="caption">Figure 14-1: `add_one` ফাংশনের জন্য HTML ডকুমেন্টেশন</span>

#### সাধারণভাবে ব্যবহৃত Section গুলো

আমরা Listing 14-1-এ `# Examples` Markdown heading টি ব্যবহার করেছি HTML-এ "Examples" শিরোনাম সহ একটি section তৈরি করতে। এখানে আরও কয়েকটি section রয়েছে যা crate author-রা তাদের ডকুমেন্টেশনে সাধারণত ব্যবহার করেন:

-   **Panics**: যে পরিস্থিতিতে document করা ফাংশনটি প্যানিক করতে পারে। ফাংশনের কলাররা যারা চান না যে তাদের প্রোগ্রামগুলো প্যানিক করুক, তাদের নিশ্চিত করা উচিত যে তারা এই পরিস্থিতিতে ফাংশনটিকে কল করবে না।
-   **Errors**: যদি ফাংশনটি একটি `Result` রিটার্ন করে, তাহলে কী ধরনের error ঘটতে পারে এবং কোন পরিস্থিতিতে সেই error গুলো রিটার্ন হতে পারে তার description কলারদের জন্য সহায়ক হতে পারে যাতে তারা different উপায়ে different ধরনের error হ্যান্ডেল করার জন্য কোড লিখতে পারে।
-   **Safety**: যদি ফাংশনটি কল করা `unsafe` হয় (আমরা Chapter 20-এ unsafety নিয়ে আলোচনা করব), তাহলে কেন ফাংশনটি unsafe এবং ফাংশনটি আশা করে যে কলাররা কোন invariant গুলো বজায় রাখবে তা ব্যাখ্যা করে একটি section থাকা উচিত।

বেশিরভাগ ডকুমেন্টেশন কমেন্টের এই সমস্ত section-গুলোর প্রয়োজন নেই, তবে এটি একটি ভালো চেকলিস্ট যা আপনাকে আপনার কোডের সেই দিকগুলো মনে করিয়ে দেবে যেগুলোর বিষয়ে user-রা জানতে আগ্রহী হবে।

#### ডকুমেন্টেশন কমেন্টগুলো টেস্ট হিসেবে

আপনার ডকুমেন্টেশন কমেন্টগুলোতে example code block যোগ করা আপনার লাইব্রেরি কীভাবে ব্যবহার করতে হয় তা প্রদর্শন করতে সাহায্য করতে পারে, এবং এটি করার একটি additional bonus রয়েছে: `cargo test` চালালে আপনার ডকুমেন্টেশনের code example গুলো test হিসেবে চলবে! Example সহ ডকুমেন্টেশনের চেয়ে ভালো কিছু নেই। কিন্তু উদাহরণের চেয়ে খারাপ কিছু নেই যা কাজ করে না কারণ ডকুমেন্টেশন লেখার পর থেকে কোড পরিবর্তন হয়েছে। যদি আমরা Listing 14-1 থেকে `add_one` ফাংশনের জন্য ডকুমেন্টেশন সহ `cargo test` চালাই, তাহলে আমরা test result-এ এইরকম একটি section দেখতে পাব:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

এখন যদি আমরা ফাংশন বা example পরিবর্তন করি যাতে example-এর `assert_eq!` প্যানিক করে এবং আবার `cargo test` চালাই, তাহলে আমরা দেখতে পাব যে doc test গুলো ধরবে যে example এবং code একে অপরের সাথে out of sync!

#### কন্টেইন করা Item গুলোতে কমেন্ট করা

Doc comment-এর `//!` স্টাইলটি comment-এর পরে থাকা item-গুলোর পরিবর্তে comment গুলো ধারণ করা item-টিতে ডকুমেন্টেশন যোগ করে। আমরা সাধারণত এই doc comment গুলো crate root ফাইলের ভিতরে (_src/lib.rs_ convention অনুযায়ী) বা একটি module-এর ভিতরে ব্যবহার করি crate বা module-টিকে সামগ্রিকভাবে document করার জন্য।

উদাহরণস্বরূপ, `add_one` ফাংশন ধারণকারী `my_crate` crate-টির purpose বর্ণনা করে এমন ডকুমেন্টেশন যোগ করার জন্য, আমরা Listing 14-2-তে দেখানো _src/lib.rs_ ফাইলের শুরুতে `//!` দিয়ে শুরু হওয়া ডকুমেন্টেশন কমেন্ট যোগ করি:

<Listing number="14-2" file-name="src/lib.rs" caption="সামগ্রিকভাবে `my_crate` crate-এর জন্য ডকুমেন্টেশন">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

</Listing>

লক্ষ্য করুন `//!` দিয়ে শুরু হওয়া শেষ লাইনের পরে কোনো কোড নেই। যেহেতু আমরা `///`-এর পরিবর্তে `//!` দিয়ে comment গুলো শুরু করেছি, তাই আমরা এই comment-এর পরে থাকা কোনো item-এর পরিবর্তে এই comment ধারণ করা item-টিকে document করছি। এই ক্ষেত্রে, সেই item টি হল _src/lib.rs_ ফাইল, যেটি হল crate root। এই comment গুলো সম্পূর্ণ crate বর্ণনা করে।

যখন আমরা `cargo doc --open` চালাই, তখন এই comment গুলো crate-এর public item-গুলোর list-এর উপরে `my_crate`-এর ডকুমেন্টেশনের front page-এ প্রদর্শিত হবে, যেমনটি Figure 14-2-তে দেখানো হয়েছে:

<img alt="সামগ্রিকভাবে ক্রেটের জন্য একটি মন্তব্য সহ Rendered HTML ডকুমেন্টেশন" src="img/trpl14-02.png" class="center" />

<span class="caption">Figure 14-2: `my_crate`-এর জন্য Rendered ডকুমেন্টেশন, সামগ্রিকভাবে crate বর্ণনা করে এমন মন্তব্য সহ</span>

Item-গুলোর ভেতরের ডকুমেন্টেশন কমেন্টগুলো বিশেষ করে crate এবং module গুলো describe করার জন্য useful। আপনার user-দের crate-এর organization বুঝতে সাহায্য করার জন্য container-এর overall purpose ব্যাখ্যা করতে এগুলো ব্যবহার করুন।

### `pub use`-এর সাহায্যে একটি সুবিধাজনক Public API এক্সপোর্ট করা

আপনার public API-এর structure একটি crate publish করার সময় একটি major consideration। আপনার crate ব্যবহার করা লোকেরা structure-টির সাথে আপনার চেয়ে কম পরিচিত এবং আপনার crate-এ একটি large module hierarchy থাকলে তারা যে অংশগুলো ব্যবহার করতে চায় সেগুলো খুঁজে পেতে অসুবিধা হতে পারে।

Chapter 7-এ, আমরা `pub` keyword ব্যবহার করে কীভাবে item গুলোকে public করতে হয় এবং `use` keyword-এর সাহায্যে item গুলোকে scope-এ আনতে হয় তা দেখেছি। যাইহোক, আপনি যখন একটি crate develop করছেন তখন যে structure টি আপনার কাছে যুক্তিসঙ্গত মনে হয় সেটি আপনার user-দের জন্য খুব সুবিধাজনক নাও হতে পারে। আপনি আপনার struct গুলোকে multiple level ধারণকারী একটি hierarchy-তে organize করতে চাইতে পারেন, কিন্তু তারপর যারা hierarchy-এর গভীরে define করা একটি type ব্যবহার করতে চান তারা হয়তো সেই type-টি existing কিনা তা খুঁজে পেতে সমস্যায় পড়তে পারেন। তাদের `use` `my_crate::some_module::another_module::UsefulType;`-এর পরিবর্তে `use` `my_crate::UsefulType;` লিখতে হতে পারে।

ভাল খবর হল যদি structure-টি অন্য library থেকে ব্যবহার করার জন্য সুবিধাজনক _না_ হয়, তাহলে আপনাকে আপনার internal organization rearrange করতে হবে না: এর পরিবর্তে, আপনি `pub use` ব্যবহার করে আপনার private structure থেকে different একটি public structure তৈরি করতে item গুলোকে re-export করতে পারেন। Re-exporting একটি public item-কে এক জায়গায় নেয় এবং এটিকে অন্য জায়গায় public করে, যেন এটি অন্য জায়গার পরিবর্তে সেখানেই define করা হয়েছিল।

উদাহরণস্বরূপ, ধরা যাক আমরা artistic concept গুলো model করার জন্য `art` নামে একটি লাইব্রেরি তৈরি করেছি। এই লাইব্রেরির মধ্যে দুটি module রয়েছে: `kinds` module যাতে `PrimaryColor` এবং `SecondaryColor` নামে দুটি enum রয়েছে এবং `utils` module যাতে `mix` নামে একটি ফাংশন রয়েছে, যেমনটি Listing 14-3-তে দেখানো হয়েছে:

<Listing number="14-3" file-name="src/lib.rs" caption="`kinds` এবং `utils` মডিউলে সংগঠিত আইটেম সহ একটি `art` লাইব্রেরি">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

</Listing>

Figure 14-3 দেখায় যে `cargo doc` দ্বারা generate করা এই crate-এর ডকুমেন্টেশনের front page-টি কেমন হবে:

<img alt="`art` crate-এর জন্য Rendered ডকুমেন্টেশন যা `kinds` এবং `utils` মডিউলগুলোর তালিকা করে" src="img/trpl14-03.png" class="center" />

<span class="caption">Figure 14-3: `art`-এর ডকুমেন্টেশনের Front page যা `kinds` এবং `utils` module-গুলোর তালিকা করে</span>

লক্ষ্য করুন যে `PrimaryColor` এবং `SecondaryColor` type গুলো front page-এ list করা হয়নি, `mix` ফাংশনটিও নয়। সেগুলো দেখতে আমাদের `kinds` এবং `utils`-এ ক্লিক করতে হবে।

অন্য একটি crate যেটি এই লাইব্রেরির উপর নির্ভর করে, সেটির `use` statement প্রয়োজন হবে যা `art` থেকে item গুলোকে scope-এ আনে, বর্তমানে define করা module structure টি specify করে। Listing 14-4 একটি crate-এর উদাহরণ দেখায় যেটি `art` crate থেকে `PrimaryColor` এবং `mix` item গুলো ব্যবহার করে:

<Listing number="14-4" file-name="src/main.rs" caption="`art` crate-এর item গুলো ব্যবহার করে এমন একটি crate যার internal structure এক্সপোর্ট করা হয়েছে">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

</Listing>

Listing 14-4-এর code-এর author, যিনি `art` crate ব্যবহার করেন, তাকে খুঁজে বের করতে হয়েছিল যে `PrimaryColor` `kinds` module-এ রয়েছে এবং `mix` `utils` module-এ রয়েছে। `Art` crate-এর module structure টি `art` crate-এ কাজ করা developer-দের জন্য এটি ব্যবহারকারীদের চেয়ে বেশি relevant। Internal structure-টিতে `art` crate কীভাবে ব্যবহার করতে হয় তা বোঝার চেষ্টা করা কারও জন্য কোনো useful information নেই, বরং বিভ্রান্তির কারণ হয় কারণ যারা এটি ব্যবহার করেন তাদের খুঁজে বের করতে হবে কোথায় খুঁজতে হবে এবং `use` statement-গুলোতে module-এর নাম specify করতে হবে।

Public API থেকে internal organization সরানোর জন্য, আমরা Listing 14-3-এর `art` crate code modify করে top level-এ item গুলোকে re-export করতে `pub use` statement যোগ করতে পারি, যেমনটি Listing 14-5-এ দেখানো হয়েছে:

<Listing number="14-5" file-name="src/lib.rs" caption="Item গুলোকে re-export করতে `pub use` স্টেটমেন্ট যোগ করা">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

</Listing>

`Cargo doc` এই crate-এর জন্য যে API ডকুমেন্টেশন generate করে সেটি এখন front page-এ re-export গুলোকে list করবে এবং লিঙ্ক করবে, যেমনটি Figure 14-4-এ দেখানো হয়েছে, `PrimaryColor` এবং `SecondaryColor` type গুলো এবং `mix` ফাংশনটিকে খুঁজে পাওয়া সহজ করে তুলবে।

<img alt="Front page-এ re-export সহ `art` crate-এর জন্য Rendered ডকুমেন্টেশন" src="img/trpl14-04.png" class="center" />

<span class="caption">Figure 14-4: `art`-এর ডকুমেন্টেশনের Front page যা re-export-গুলোর তালিকা করে</span>

`Art` crate user-রা এখনও Listing 14-3 থেকে internal structure দেখতে এবং ব্যবহার করতে পারে যেমনটি Listing 14-4-এ দেখানো হয়েছে, অথবা তারা Listing 14-5-এর আরও সুবিধাজনক structure টি ব্যবহার করতে পারে, যেমনটি Listing 14-6-এ দেখানো হয়েছে:

<Listing number="14-6" file-name="src/main.rs" caption="`art` crate থেকে re-exported item গুলো ব্যবহার করে এমন একটি প্রোগ্রাম">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

</Listing>

যেসব ক্ষেত্রে অনেকগুলি nested module রয়েছে, সেখানে `pub use`-এর সাহায্যে top level-এ type গুলোকে re-export করা crate ব্যবহার করা লোকেদের অভিজ্ঞতায় significant difference আনতে পারে। `Pub use`-এর আরেকটি সাধারণ ব্যবহার হল current crate-এ একটি dependency-এর definition গুলোকে re-export করা যাতে সেই crate-এর definition গুলো আপনার crate-এর public API-এর অংশ হয়।

একটি useful public API structure তৈরি করা বিজ্ঞানের চেয়ে একটি শিল্প বেশি, এবং আপনি আপনার user-দের জন্য সবচেয়ে উপযুক্ত API খুঁজে বের করার জন্য iterate করতে পারেন। `Pub use` বেছে নেওয়া আপনাকে flexibility দেয় যে আপনি কীভাবে আপনার crate-কে internally structure করবেন এবং আপনার user-দের কাছে যা present করবেন তা থেকে সেই internal structure-কে decouple করবেন। আপনি যে crate গুলো install করেছেন সেগুলোর মধ্যে কয়েকটির code দেখুন যাতে বোঝা যায় যে তাদের internal structure তাদের public API থেকে আলাদা কিনা।

### একটি Crates.io অ্যাকাউন্ট সেট আপ করা

আপনি কোনো crate publish করার আগে, আপনাকে [crates.io](https://crates.io/)-তে একটি অ্যাকাউন্ট তৈরি করতে হবে এবং একটি API টোকেন পেতে হবে। এটি করার জন্য, [crates.io](https://crates.io/)-এ হোম পেজে যান এবং একটি GitHub অ্যাকাউন্টের মাধ্যমে লগ ইন করুন। (GitHub অ্যাকাউন্টটি বর্তমানে একটি requirement, কিন্তু site-টি ভবিষ্যতে অ্যাকাউন্ট তৈরি করার অন্য উপায় support করতে পারে।) একবার আপনি লগ ইন করার পরে, [https://crates.io/me/](https://crates.io/me/)-এ আপনার অ্যাকাউন্ট সেটিংসে যান এবং আপনার API key পান। তারপর `cargo login` কমান্ডটি চালান এবং prompt করা হলে আপনার API key টি পেস্ট করুন, এইভাবে:

```console
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```

এই কমান্ডটি Cargo-কে আপনার API টোকেন সম্পর্কে জানাবে এবং এটিকে locally _~/.cargo/credentials_-এ store করবে। মনে রাখবেন যে এই টোকেনটি একটি _গোপনীয়তা_: এটি অন্য কারও সাথে share করবেন না। আপনি যদি কোনো কারণে এটি কারও সাথে share করেন, তাহলে আপনার এটি revoke করা উচিত এবং [crates.io](https://crates.io/)-তে একটি new token generate করা উচিত।

### একটি নতুন Crate-এ মেটাডেটা যোগ করা

ধরা যাক আপনার কাছে একটি crate আছে যা আপনি publish করতে চান। Publish করার আগে, আপনাকে crate-এর _Cargo.toml_ ফাইলের `[package]` section-এ কিছু metadata যোগ করতে হবে।

আপনার crate-এর একটি unique name প্রয়োজন হবে। আপনি locally একটি crate-এ কাজ করার সময়, আপনি একটি crate-এর নাম যা খুশি রাখতে পারেন। যাইহোক, [crates.io](https://crates.io/)-তে crate-এর নামগুলো first-come, first-served ভিত্তিতে বরাদ্দ করা হয়। একবার একটি crate-এর নাম নেওয়া হলে, অন্য কেউ সেই নামে একটি crate publish করতে পারবে না। একটি crate publish করার চেষ্টা করার আগে, আপনি যে নামটি ব্যবহার করতে চান সেটি search করুন। যদি নামটি ব্যবহার করা হয়ে থাকে, তাহলে আপনাকে অন্য একটি নাম খুঁজে বের করতে হবে এবং publish করার জন্য new name টি ব্যবহার করতে _Cargo.toml_ ফাইলের `[package]` section-এর অধীনে `name` field টি edit করতে হবে, এইভাবে:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

এমনকি যদি আপনি একটি unique name বেছে নিয়ে থাকেন, তাহলেও আপনি যখন এই সময়ে crate publish করার জন্য `cargo publish` চালান, তখন আপনি একটি warning এবং তারপর একটি error পাবেন:

<!-- manual-regeneration
Create a new package with an unregistered name, making no further modifications
  to the generated package, so it is missing the description and license fields.
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error (status 400 Bad Request): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for more information on configuring these fields
```

এই error-টির কারণ হল আপনি কিছু crucial information missing রেখেছেন: একটি description এবং license প্রয়োজন যাতে লোকেরা জানতে পারে আপনার crate কী করে এবং কোন শর্তে তারা এটি ব্যবহার করতে পারে। _Cargo.toml_-এ, একটি description যোগ করুন যা শুধুমাত্র একটি বা দুটি বাক্য, কারণ এটি আপনার crate-এর সাথে search result-এ প্রদর্শিত হবে। `License` field-এর জন্য, আপনাকে একটি _লাইসেন্স শনাক্তকারী মান_ দিতে হবে। [Linux Foundation’s Software Package Data Exchange (SPDX)][spdx] আপনি এই value-টির জন্য যে identifier গুলো ব্যবহার করতে পারেন সেগুলোর তালিকা করে। উদাহরণস্বরূপ, আপনি যদি specify করতে চান যে আপনি MIT লাইসেন্স ব্যবহার করে আপনার crate লাইসেন্স করেছেন, তাহলে `MIT` identifier যোগ করুন:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
license = "MIT"
```

আপনি যদি এমন একটি লাইসেন্স ব্যবহার করতে চান যা SPDX-এ নেই, তাহলে আপনাকে সেই লাইসেন্সের text একটি ফাইলে রাখতে হবে, ফাইলটিকে আপনার প্রোজেক্টে include করতে হবে এবং তারপর `license` key ব্যবহার করার পরিবর্তে সেই ফাইলের নাম specify করতে `license-file` ব্যবহার করতে হবে।

আপনার প্রোজেক্টের জন্য কোন লাইসেন্স উপযুক্ত সে সম্পর্কে গাইডেন্স এই বইয়ের সুযোগের বাইরে। Rust community-র অনেকে `MIT OR Apache-2.0`-এর dual license ব্যবহার করে Rust-এর মতোই তাদের প্রোজেক্ট লাইসেন্স করে। এই practice টি প্রদর্শন করে যে আপনি আপনার প্রোজেক্টের জন্য multiple license রাখতে `OR` দ্বারা separated multiple license identifier-ও specify করতে পারেন।

একটি unique name, version, আপনার description এবং একটি লাইসেন্স যোগ করার সাথে, publish করার জন্য প্রস্তুত একটি প্রোজেক্টের জন্য _Cargo.toml_ ফাইলটি এইরকম হতে পারে:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

অন্যরা যাতে আপনার crate আরও সহজে খুঁজে পেতে এবং ব্যবহার করতে পারে তা নিশ্চিত করতে আপনি যে অন্যান্য metadata specify করতে পারেন [Cargo-এর ডকুমেন্টেশন](https://doc.rust-lang.org/cargo/) তা describe করে।

### Crates.io-তে পাবলিশ করা

এখন যেহেতু আপনি একটি অ্যাকাউন্ট তৈরি করেছেন, আপনার API টোকেন save করেছেন, আপনার crate-এর জন্য একটি নাম বেছে নিয়েছেন এবং প্রয়োজনীয় metadata specify করেছেন, আপনি publish করার জন্য প্রস্তুত! একটি crate publish করা অন্যদের ব্যবহারের জন্য [crates.io](https://crates.io/)-তে একটি specific version আপলোড করে।

সতর্ক থাকুন, কারণ একটি publish হল _স্থায়ী_। Version টি কখনই overwrite করা যাবে না এবং code delete করা যাবে না। [Crates.io](https://crates.io/)-এর একটি প্রধান লক্ষ্য হল code-এর একটি permanent archive হিসেবে কাজ করা যাতে [crates.io](https://crates.io/) থেকে crate-গুলোর উপর নির্ভর করে এমন সমস্ত প্রোজেক্টের build গুলো কাজ করতে থাকে। Version deletion-এর অনুমতি দিলে সেই লক্ষ্য পূরণ করা অসম্ভব হয়ে যেত। তবে, আপনি কতগুলো crate version publish করতে পারবেন তার কোনো limit নেই।

আবার `cargo publish` কমান্ডটি চালান। এটি এখন সফল হওয়া উচিত:

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
```

অভিনন্দন! আপনি এখন Rust community-র সাথে আপনার কোড share করেছেন এবং যে কেউ সহজেই তাদের প্রোজেক্টের dependency হিসেবে আপনার crate যোগ করতে পারে।

### একটি বিদ্যমান Crate-এর একটি নতুন Version পাবলিশ করা

আপনি যখন আপনার crate-এ পরিবর্তন করেছেন এবং একটি new version release করার জন্য প্রস্তুত, তখন আপনি আপনার _Cargo.toml_ ফাইলে specified `version` value পরিবর্তন করুন এবং পুনরায় publish করুন। আপনি কী ধরনের পরিবর্তন করেছেন তার উপর ভিত্তি করে একটি উপযুক্ত পরবর্তী version number কী তা decide করতে [Semantic Versioning নিয়ম][semver] ব্যবহার করুন। তারপর new version আপলোড করতে `cargo publish` চালান।

<!-- পুরানো লিঙ্ক, সরাবেন না -->

<a id="removing-versions-from-cratesio-with-cargo-yank"></a>

### `cargo yank`-এর সাহায্যে Crates.io থেকে Version Deprecate করা

যদিও আপনি একটি crate-এর previous version গুলো remove করতে পারবেন না, আপনি future-এর কোনো প্রোজেক্টকে সেগুলোকে new dependency হিসেবে যোগ করা থেকে আটকাতে পারেন। এটি useful যখন একটি crate version কোনো না কোনো কারণে broken থাকে। এই ধরনের পরিস্থিতিতে, Cargo একটি crate version _yank_ করা support করে।

একটি version yank করা new project গুলোকে সেই version-টির উপর নির্ভর করা থেকে বিরত রাখে এবং সেইসাথে এটির উপর নির্ভর করে এমন সমস্ত existing project গুলোকে continue রাখার অনুমতি দেয়। মূলত, একটি yank-এর অর্থ হল _Cargo.lock_ সহ সমস্ত প্রোজেক্ট break করবে না এবং future-এ generate করা কোনো _Cargo.lock_ ফাইল yank করা version টি ব্যবহার করবে না।

একটি crate-এর একটি version yank করতে, আপনি পূর্বে publish করেছেন এমন crate-টির ডিরেক্টরিতে, `cargo yank` চালান এবং কোন version টি yank করতে চান তা specify করুন। উদাহরণস্বরূপ, যদি আমরা `guessing_game` নামের একটি crate-এর version 1.0.1 publish করে থাকি এবং আমরা এটিকে yank করতে চাই, তাহলে `guessing_game`-এর প্রোজেক্ট ডিরেক্টরিতে আমরা চালাব:

<!-- manual-regeneration:
cargo yank carol-test --version 2.1.0
cargo yank carol-test --version 2.1.0 --undo
-->

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```

কমান্ডে `--undo` যোগ করে, আপনি একটি yank undo করতে পারেন এবং প্রোজেক্টগুলোকে আবার একটি version-এর উপর নির্ভর করা শুরু করার অনুমতি দিতে পারেন:

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

একটি yank কোনো কোড _delete করে না_। উদাহরণস্বরূপ, এটি accidentally আপলোড করা গোপনীয়তা delete করতে পারে না। যদি সেটি ঘটে, তাহলে আপনাকে অবশ্যই সেই গোপনীয়তাগুলো immediately reset করতে হবে।

[spdx]: http://spdx.org/licenses/
[semver]: http://semver.org/
