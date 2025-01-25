## Cargo Workspaces

অধ্যায় 12-এ, আমরা একটি প্যাকেজ তৈরি করেছি যাতে একটি বাইনারি crate এবং একটি লাইব্রেরি crate অন্তর্ভুক্ত ছিল। আপনার প্রজেক্ট develop হওয়ার সাথে সাথে, আপনি হয়তো দেখতে পাবেন যে লাইব্রেরি crate আরও বড় হতে থাকে এবং আপনি আপনার প্যাকেজটিকে আরও একাধিক লাইব্রেরি crate-এ বিভক্ত করতে চান। Cargo _workspaces_ নামে একটি বৈশিষ্ট্য সরবরাহ করে যা একাধিক সম্পর্কিত প্যাকেজ পরিচালনা করতে সাহায্য করতে পারে যা একসাথে develop করা হয়।

### একটি Workspace তৈরি করা

একটি _workspace_ হল প্যাকেজের একটি সেট যা একই _Cargo.lock_ এবং আউটপুট ডিরেক্টরি শেয়ার করে। আসুন একটি workspace ব্যবহার করে একটি প্রজেক্ট তৈরি করি—আমরা তুচ্ছ কোড ব্যবহার করব যাতে আমরা workspace-এর কাঠামোর উপর মনোযোগ দিতে পারি। একটি workspace তৈরি করার একাধিক উপায় রয়েছে, তাই আমরা শুধুমাত্র একটি সাধারণ উপায় দেখাব। আমাদের একটি workspace থাকবে যেখানে একটি বাইনারি এবং দুটি লাইব্রেরি থাকবে। বাইনারি, যা প্রধান কার্যকারিতা প্রদান করবে, দুটি লাইব্রেরির উপর নির্ভর করবে। একটি লাইব্রেরি একটি `add_one` ফাংশন প্রদান করবে এবং দ্বিতীয় লাইব্রেরি একটি `add_two` ফাংশন প্রদান করবে। এই তিনটি crate একই workspace-এর অংশ হবে। আমরা workspace-এর জন্য একটি নতুন ডিরেক্টরি তৈরি করে শুরু করব:

```console
$ mkdir add
$ cd add
```

এর পরে, _add_ ডিরেক্টরিতে, আমরা _Cargo.toml_ ফাইলটি তৈরি করি যা পুরো workspace কনফিগার করবে। এই ফাইলে কোনো `[package]` বিভাগ থাকবে না। পরিবর্তে, এটি একটি `[workspace]` বিভাগ দিয়ে শুরু হবে যা আমাদের workspace-এ সদস্য যোগ করার অনুমতি দেবে। আমরা আমাদের workspace-এ Cargo-এর resolver অ্যালগরিদমের সর্বশেষ এবং সর্বশ্রেষ্ঠ সংস্করণ ব্যবহার করতেও একটি পয়েন্ট করি, `resolver`-কে `“2”`-এ সেট করে।

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

এর পরে, আমরা _add_ ডিরেক্টরির মধ্যে `cargo new` চালিয়ে `adder` বাইনারি crate তৈরি করব:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
    Creating binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

একটি workspace-এর ভিতরে `cargo new` চালালে স্বয়ংক্রিয়ভাবে workspace `Cargo.toml`-এর `[workspace]` সংজ্ঞায় `members` কী-তে নতুন তৈরি প্যাকেজটিও যোগ করা হয়, যেমন:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

এই সময়ে, আমরা `cargo build` চালিয়ে workspace তৈরি করতে পারি। আপনার _add_ ডিরেক্টরির ফাইলগুলি এইরকম দেখতে হবে:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

workspace-এর উপরের স্তরে একটি _target_ ডিরেক্টরি রয়েছে যেখানে কম্পাইল করা আর্টিফ্যাক্টগুলি স্থাপন করা হবে; `adder` প্যাকেজের নিজস্ব _target_ ডিরেক্টরি নেই। এমনকি যদি আমরা _adder_ ডিরেক্টরির ভিতরে থেকে `cargo build` চালাই, কম্পাইল করা আর্টিফ্যাক্টগুলি _add/target_-এ শেষ হবে _add/adder/target_-এর পরিবর্তে। Cargo একটি workspace-এ _target_ ডিরেক্টরিটিকে এভাবে তৈরি করে কারণ একটি workspace-এর crate গুলো একে অপরের উপর নির্ভরশীল হওয়ার কথা। যদি প্রতিটি crate-এর নিজস্ব _target_ ডিরেক্টরি থাকত, তাহলে প্রতিটি crate-কে তার নিজস্ব _target_ ডিরেক্টরিতে আর্টিফ্যাক্টগুলি রাখার জন্য workspace-এর অন্যান্য crate-গুলিকে পুনরায় কম্পাইল করতে হত। একটি _target_ ডিরেক্টরি শেয়ার করে, crateগুলি অপ্রয়োজনীয় পুনর্নির্মাণ এড়াতে পারে।

### Workspace-এ দ্বিতীয় প্যাকেজ তৈরি করা

এর পরে, workspace-এ আরও একটি সদস্য প্যাকেজ তৈরি করা যাক এবং এটিকে `add_one` বলা যাক। `members` তালিকায় _add_one_ পাথ নির্দিষ্ট করতে শীর্ষ-স্তরের _Cargo.toml_ পরিবর্তন করুন:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

তারপর `add_one` নামের একটি নতুন লাইব্রেরি crate তৈরি করুন:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
    Creating library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

আপনার _add_ ডিরেক্টরিতে এখন এই ডিরেক্টরি এবং ফাইলগুলি থাকা উচিত:

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

_add_one/src/lib.rs_ ফাইলে, আসুন একটি `add_one` ফাংশন যোগ করি:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

এখন আমরা আমাদের বাইনারি সহ `adder` প্যাকেজটিকে `add_one` প্যাকেজের উপর নির্ভরশীল করতে পারি যেখানে আমাদের লাইব্রেরি রয়েছে। প্রথমে, আমাদের _adder/Cargo.toml_-এ `add_one`-এর উপর একটি পাথ নির্ভরতা যোগ করতে হবে।

<span class="filename">Filename: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo ধরে নেয় না যে একটি workspace-এর crate গুলি একে অপরের উপর নির্ভরশীল হবে, তাই আমাদের নির্ভরতা সম্পর্কগুলি সম্পর্কে স্পষ্ট হতে হবে।

এর পরে, আসুন `adder` crate-এ `add_one` ফাংশন (যা `add_one` crate থেকে এসেছে) ব্যবহার করি। _adder/src/main.rs_ ফাইলটি খুলুন এবং Listing 14-7-এর মতো `main` ফাংশনটিকে `add_one` ফাংশন কল করার জন্য পরিবর্তন করুন।

<Listing number="14-7" file-name="adder/src/main.rs" caption="`adder` crate-এ `add_one` লাইব্রেরি crate ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

</Listing>

আসুন শীর্ষ-স্তরের _add_ ডিরেক্টরিতে `cargo build` চালিয়ে workspace তৈরি করি!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

_add_ ডিরেক্টরি থেকে বাইনারি crate চালানোর জন্য, আমরা `-p` আর্গুমেন্ট এবং `cargo run` সহ প্যাকেজের নাম ব্যবহার করে workspace-এ কোন প্যাকেজটি চালাতে চাই তা নির্দিষ্ট করতে পারি:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

এটি _adder/src/main.rs_-এর কোড চালায়, যা `add_one` crate-এর উপর নির্ভরশীল।

#### একটি Workspace-এ একটি External প্যাকেজের উপর নির্ভর করা

লক্ষ্য করুন যে workspace-এর উপরের স্তরে শুধুমাত্র একটি _Cargo.lock_ ফাইল রয়েছে, প্রতিটি crate-এর ডিরেক্টরিতে _Cargo.lock_ থাকার পরিবর্তে। এটি নিশ্চিত করে যে সমস্ত crate সমস্ত নির্ভরতার একই সংস্করণ ব্যবহার করছে। যদি আমরা _adder/Cargo.toml_ এবং _add_one/Cargo.toml_ ফাইলগুলিতে `rand` প্যাকেজ যোগ করি, Cargo উভয়কেই `rand`-এর একটি সংস্করণে সমাধান করবে এবং সেটিকে একটি _Cargo.lock_-এ রেকর্ড করবে। workspace-এর সমস্ত crate-কে একই নির্ভরতা ব্যবহার করার মানে হল crate গুলো সবসময় একে অপরের সাথে সামঞ্জস্যপূর্ণ হবে। আসুন `add_one` crate-এ `rand` crate ব্যবহার করতে _add_one/Cargo.toml_ ফাইলের `[dependencies]` বিভাগে `rand` crate যোগ করি:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Filename: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

এখন আমরা _add_one/src/lib.rs_ ফাইলে `use rand;` যোগ করতে পারি এবং _add_ ডিরেক্টরিতে `cargo build` চালিয়ে পুরো workspace তৈরি করলে `rand` crate আসবে এবং কম্পাইল হবে। আমরা একটি warning পাব কারণ আমরা scope-এ আনা `rand`-এর সাথে কিছুই করছি না:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

শীর্ষ-স্তরের _Cargo.lock_ এখন `rand`-এর উপর `add_one`-এর নির্ভরতা সম্পর্কিত তথ্য ধারণ করে। যাইহোক, workspace-এ কোথাও `rand` ব্যবহার করা হলেও, আমরা workspace-এর অন্যান্য crate-এ এটি ব্যবহার করতে পারি না যতক্ষণ না আমরা তাদের _Cargo.toml_ ফাইলগুলিতেও `rand` যোগ করি। উদাহরণস্বরূপ, যদি আমরা `adder` প্যাকেজের জন্য _adder/src/main.rs_ ফাইলে `use rand;` যোগ করি, তাহলে আমরা একটি error পাব:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

এটি ঠিক করতে, `adder` প্যাকেজের জন্য _Cargo.toml_ ফাইলটি সম্পাদনা করুন এবং নির্দেশ করুন যে `rand` এটির জন্যও একটি নির্ভরতা। `adder` প্যাকেজ তৈরি করলে _Cargo.lock_-এ `adder`-এর জন্য নির্ভরতার তালিকায় `rand` যোগ হবে, তবে `rand`-এর কোনো অতিরিক্ত কপি ডাউনলোড করা হবে না। Cargo নিশ্চিত করবে যে workspace-এর প্রতিটি প্যাকেজের প্রতিটি crate `rand` প্যাকেজ ব্যবহার করে `rand`-এর সামঞ্জস্যপূর্ণ সংস্করণগুলি নির্দিষ্ট করা পর্যন্ত একই সংস্করণ ব্যবহার করবে, যা আমাদের স্থান বাঁচাবে এবং নিশ্চিত করবে যে workspace-এর crate গুলি একে অপরের সাথে সামঞ্জস্যপূর্ণ হবে।

যদি workspace-এর crate গুলি একই নির্ভরতার অসামঞ্জস্যপূর্ণ সংস্করণ নির্দিষ্ট করে, তাহলে Cargo তাদের প্রত্যেকটিকে সমাধান করবে, কিন্তু এখনও যতটা সম্ভব কম সংস্করণ সমাধান করার চেষ্টা করবে।

#### একটি Workspace এ একটি Test যোগ করা

আরও একটি উন্নতির জন্য, আসুন `add_one` crate-এর মধ্যে `add_one::add_one` ফাংশনের একটি test যোগ করি:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

এখন শীর্ষ-স্তরের _add_ ডিরেক্টরিতে `cargo test` চালান। এইরকম একটি কাঠামোযুক্ত workspace-এ `cargo test` চালালে workspace-এর সমস্ত crate-এর test চলবে:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-f0253159197f7841)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

আউটপুটের প্রথম বিভাগে দেখা যাচ্ছে যে `add_one` crate-এর `it_works` test পাস করেছে। পরবর্তী বিভাগে দেখা যাচ্ছে যে `adder` crate-এ কোনো test পাওয়া যায়নি এবং তারপর শেষ বিভাগে দেখা যাচ্ছে যে `add_one` crate-এ কোনো ডকুমেন্টেশন test পাওয়া যায়নি।

আমরা শীর্ষ-স্তরের ডিরেক্টরি থেকে `-p` ফ্ল্যাগ ব্যবহার করে এবং আমরা যে crate-এর test করতে চাই তার নাম উল্লেখ করে workspace-এর একটি নির্দিষ্ট crate-এর জন্যও test চালাতে পারি:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

এই আউটপুট দেখায় যে `cargo test` শুধুমাত্র `add_one` crate-এর test চালিয়েছে এবং `adder` crate test চালায়নি।

আপনি যদি workspace-এর crate গুলিকে [crates.io](https://crates.io/)-এ প্রকাশ করেন, তাহলে workspace-এর প্রতিটি crate-কে আলাদাভাবে প্রকাশ করতে হবে। `cargo test`-এর মতো, আমরা `-p` ফ্ল্যাগ ব্যবহার করে এবং আমরা যে crate প্রকাশ করতে চাই তার নাম উল্লেখ করে আমাদের workspace-এ একটি নির্দিষ্ট crate প্রকাশ করতে পারি।

অতিরিক্ত অনুশীলনের জন্য, `add_one` crate-এর মতোই এই workspace-এ একটি `add_two` crate যোগ করুন!

আপনার প্রজেক্ট grow করার সাথে সাথে একটি workspace ব্যবহার করার কথা বিবেচনা করুন: কোডের একটি বড় blob এর চেয়ে ছোট, পৃথক উপাদানগুলি বোঝা সহজ। তদুপরি, workspace-এ crate গুলি রাখলে crateগুলি যদি প্রায়শই একই সময়ে পরিবর্তন করা হয় তবে crate গুলোর মধ্যে সমন্বয় করা সহজ হতে পারে।
