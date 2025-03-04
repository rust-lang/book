## Cargo Workspaces

Chapter 12-এ, আমরা একটি প্যাকেজ তৈরি করেছি যাতে একটি বাইনারি ক্রেট এবং একটি লাইব্রেরি ক্রেট অন্তর্ভুক্ত ছিল। আপনার প্রোজেক্ট develop হওয়ার সাথে সাথে, আপনি হয়তো দেখতে পাবেন যে লাইব্রেরি ক্রেটটি আরও বড় হচ্ছে এবং আপনি আপনার প্যাকেজটিকে আরও multiple library crate-এ split করতে চান। Cargo _workspaces_ নামক একটি feature offer করে যা একসাথে develop করা multiple related package গুলোকে manage করতে সাহায্য করতে পারে।

### একটি ওয়ার্কস্পেস তৈরি করা

একটি _ওয়ার্কস্পেস_ হল প্যাকেজগুলোর একটি set যা একই _Cargo.lock_ এবং আউটপুট ডিরেক্টরি share করে। আসুন একটি ওয়ার্কস্পেস ব্যবহার করে একটি প্রোজেক্ট তৈরি করি—আমরা trivial কোড ব্যবহার করব যাতে আমরা ওয়ার্কস্পেসের structure-এর উপর মনোযোগ দিতে পারি। একটি ওয়ার্কস্পেসকে structure করার একাধিক উপায় রয়েছে, তাই আমরা শুধুমাত্র একটি common উপায় দেখাব। আমাদের কাছে একটি বাইনারি এবং দুটি লাইব্রেরি ধারণকারী একটি ওয়ার্কস্পেস থাকবে। বাইনারি, যেটি main functionality provide করবে, দুটি লাইব্রেরির উপর নির্ভর করবে। একটি লাইব্রেরি একটি `add_one` ফাংশন provide করবে, এবং দ্বিতীয় লাইব্রেরি একটি `add_two` ফাংশন provide করবে। এই তিনটি crate একই ওয়ার্কস্পেসের অংশ হবে। আমরা ওয়ার্কস্পেসের জন্য একটি নতুন ডিরেক্টরি তৈরি করে শুরু করব:

```console
$ mkdir add
$ cd add
```

এরপরে, _add_ ডিরেক্টরিতে, আমরা _Cargo.toml_ ফাইলটি তৈরি করি যা সম্পূর্ণ ওয়ার্কস্পেস configure করবে। এই ফাইলটিতে একটি `[package]` section থাকবে না। পরিবর্তে, এটি একটি `[workspace]` section দিয়ে শুরু হবে যা আমাদের ওয়ার্কস্পেসে member যোগ করার অনুমতি দেবে। আমরা আমাদের ওয়ার্কস্পেসে Cargo-এর resolver algorithm-এর latest and greatest version ব্যবহার করার জন্য `resolver` কে `"2"`-তে set করে রাখি।

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

এরপরে, আমরা _add_ ডিরেক্টরির মধ্যে `cargo new` চালিয়ে `adder` বাইনারি ক্রেট তৈরি করব:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
remove `members = ["adder"]` from Cargo.toml
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
    Creating binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

ওয়ার্কস্পেসের ভিতরে `cargo new` চালানো স্বয়ংক্রিয়ভাবে newly created প্যাকেজটিকে ওয়ার্কস্পেস `Cargo.toml`-এর `[workspace]` ডেফিনিশনে `members` কী-তে যোগ করে, এইভাবে:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

এই সময়ে, আমরা `cargo build` চালিয়ে ওয়ার্কস্পেস build করতে পারি। আপনার _add_ ডিরেক্টরির ফাইলগুলো এইরকম হওয়া উচিত:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

ওয়ার্কস্পেসের top level-এ একটি _target_ ডিরেক্টরি রয়েছে যেখানে compiled artifact গুলো রাখা হবে; `adder` প্যাকেজের নিজস্ব _target_ ডিরেক্টরি নেই। এমনকি যদি আমরা _adder_ ডিরেক্টরির ভেতর থেকে `cargo build` চালাই, তাহলেও compiled artifact গুলো _add/adder/target_-এর পরিবর্তে _add/target_-এ থাকবে। Cargo এইভাবে একটি ওয়ার্কস্পেসে _target_ ডিরেক্টরিকে structure করে কারণ একটি ওয়ার্কস্পেসের crate গুলো একে অপরের উপর নির্ভর করে। যদি প্রতিটি crate-এর নিজস্ব _target_ ডিরেক্টরি থাকত, তাহলে প্রতিটি crate-কে তার নিজস্ব _target_ ডিরেক্টরিতে artifact গুলো রাখার জন্য ওয়ার্কস্পেসের প্রতিটি অন্য crate-কে recompile করতে হত। একটি _target_ ডিরেক্টরি share করে, crate গুলো অপ্রয়োজনীয় rebuilding এড়াতে পারে।

### ওয়ার্কস্পেসে দ্বিতীয় প্যাকেজ তৈরি করা

এরপরে, আসুন ওয়ার্কস্পেসে আরেকটি মেম্বার প্যাকেজ তৈরি করি এবং এটিকে `add_one` নাম দিই। Top-level _Cargo.toml_ পরিবর্তন করে `members` লিস্টে _add_one_ পাথ specify করুন:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

তারপর `add_one` নামের একটি নতুন লাইব্রেরি ক্রেট generate করুন:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
remove `"add_one"` from `members` list in Cargo.toml
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
    Creating library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

আপনার _add_ ডিরেক্টরিতে এখন এই ডিরেক্টরি এবং ফাইলগুলো থাকা উচিত:

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

_Add_one/src/lib.rs_ ফাইলে, আসুন একটি `add_one` ফাংশন যোগ করি:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

এখন আমরা আমাদের বাইনারি সহ `adder` প্যাকেজটিকে `add_one` প্যাকেজের উপর নির্ভর করাতে পারি যেটিতে আমাদের লাইব্রেরি রয়েছে। প্রথমে, আমাদের _adder/Cargo.toml_-এ `add_one`-এর উপর একটি পাথ নির্ভরতা যোগ করতে হবে।

<span class="filename">Filename: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo ধরে নেয় না যে একটি ওয়ার্কস্পেসের crate গুলো একে অপরের উপর নির্ভর করবে, তাই আমাদের dependency relationship গুলো সম্পর্কে explicit হতে হবে।

এরপরে, আসুন `adder` ক্রেটে `add_one` ক্রেট থেকে `add_one` ফাংশনটি ব্যবহার করি। _Adder/src/main.rs_ ফাইলটি খুলুন এবং `main` ফাংশন পরিবর্তন করে `add_one` ফাংশনটিকে কল করুন, যেমনটি Listing 14-7-এ রয়েছে।

<Listing number="14-7" file-name="adder/src/main.rs" caption="`adder` ক্রেটে `add_one` লাইব্রেরি ক্রেট ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-7/add/adder/src/main.rs}}
```

</Listing>

Top-level _add_ ডিরেক্টরিতে `cargo build` চালিয়ে ওয়ার্কস্পেস build করি!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-7/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

_Add_ ডিরেক্টরি থেকে বাইনারি ক্রেট চালানোর জন্য, আমরা `-p` আর্গুমেন্ট এবং প্যাকেজের নাম `cargo run`-এর সাথে ব্যবহার করে ওয়ার্কস্পেসের কোন প্যাকেজটি চালাতে চাই তা specify করতে পারি:

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

এটি _adder/src/main.rs_-এর কোড চালায়, যেটি `add_one` ক্রেটের উপর নির্ভর করে।

#### ওয়ার্কস্পেসে একটি External Package-এর উপর নির্ভর করা

লক্ষ্য করুন যে ওয়ার্কস্পেসের top level-এ শুধুমাত্র একটি _Cargo.lock_ ফাইল রয়েছে, প্রতিটি crate-এর ডিরেক্টরিতে _Cargo.lock_ থাকার পরিবর্তে। এটি নিশ্চিত করে যে সমস্ত crate সমস্ত dependency-এর একই version ব্যবহার করছে। যদি আমরা _adder/Cargo.toml_ এবং _add_one/Cargo.toml_ ফাইলগুলোতে `rand` প্যাকেজ যোগ করি, তাহলে Cargo সেগুলোর উভয়কেই `rand`-এর একটি version-এ resolve করবে এবং সেটিকে একটি _Cargo.lock_-এ record করবে। ওয়ার্কস্পেসের সমস্ত crate-কে একই dependency ব্যবহার করা মানে হল crate গুলো সব সময় একে অপরের সাথে compatible হবে। আসুন _add_one/Cargo.toml_ ফাইলের `[dependencies]` section-এ `rand` crate যোগ করি যাতে আমরা `add_one` crate-এ `rand` crate ব্যবহার করতে পারি:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Filename: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

আমরা এখন _add_one/src/lib.rs_ ফাইলে `use rand;` যোগ করতে পারি, এবং _add_ ডিরেক্টরিতে `cargo build` চালিয়ে সম্পূর্ণ ওয়ার্কস্পেস build করলে `rand` crate টি আসবে এবং compile হবে। আমরা একটি warning পাব কারণ আমরা scope-এ আনা `rand`-কে refer করছি না:

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

Top-level _Cargo.lock_-এ এখন `add_one`-এর `rand`-এর উপর dependency সম্পর্কে তথ্য রয়েছে। যাইহোক, যদিও `rand` ওয়ার্কস্পেসের কোথাও ব্যবহার করা হয়েছে, তবুও আমরা ওয়ার্কস্পেসের অন্যান্য crate-গুলোতে এটি ব্যবহার করতে পারব না যতক্ষণ না আমরা তাদের _Cargo.toml_ ফাইলগুলোতেও `rand` যোগ করি। উদাহরণস্বরূপ, যদি আমরা `adder` প্যাকেজের জন্য _adder/src/main.rs_ ফাইলে `use rand;` যোগ করি, তাহলে আমরা একটি error পাব:

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

এটি ঠিক করতে, `adder` প্যাকেজের জন্য _Cargo.toml_ ফাইলটি edit করুন এবং indicate করুন যে `rand` এটির জন্যও একটি dependency। `Adder` প্যাকেজ build করলে _Cargo.lock_-এ `adder`-এর জন্য dependency-এর list-এ `rand` যোগ হবে, কিন্তু `rand`-এর কোনো additional copy ডাউনলোড হবে না। Cargo নিশ্চিত করবে যে ওয়ার্কস্পেসের প্রতিটি প্যাকেজের প্রতিটি crate `rand` প্যাকেজ ব্যবহার করে একই version ব্যবহার করবে যতক্ষণ না তারা `rand`-এর compatible version specify করে, আমাদের জায়গা বাঁচাবে এবং নিশ্চিত করবে যে ওয়ার্কস্পেসের crate গুলো একে অপরের সাথে compatible হবে।

যদি ওয়ার্কস্পেসের crate গুলো একই dependency-এর incompatible version specify করে, তাহলে Cargo সেগুলোর প্রত্যেকটিকে resolve করবে, কিন্তু তবুও যতটা সম্ভব কম version resolve করার চেষ্টা করবে।

#### ওয়ার্কস্পেসে একটি টেস্ট যোগ করা

আরেকটি enhancement-এর জন্য, আসুন `add_one` crate-এর মধ্যে `add_one::add_one` ফাংশনের একটি test যোগ করি:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

এখন top-level _add_ ডিরেক্টরিতে `cargo test` চালান। এইরকম structure করা একটি ওয়ার্কস্পেসে `cargo test` চালালে ওয়ার্কস্পেসের সমস্ত crate-এর জন্য test গুলো চলবে:

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
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

আউটপুটের প্রথম section টি দেখায় যে `add_one` ক্রেটের `it_works` test টি pass করেছে। পরবর্তী section টি দেখায় যে `adder` ক্রেটে zero test পাওয়া গেছে, এবং তারপর শেষ section টি দেখায় `add_one` ক্রেটে zero ডকুমেন্টেশন test পাওয়া গেছে।

আমরা top-level ডিরেক্টরি থেকে `-p` flag ব্যবহার করে এবং যে crate-টি test করতে চাই তার নাম specify করে একটি ওয়ার্কস্পেসের একটি particular crate-এর জন্য test চালাতে পারি:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

এই আউটপুটটি দেখায় `cargo test` শুধুমাত্র `add_one` ক্রেটের জন্য test গুলো চালিয়েছে এবং `adder` ক্রেটের test গুলো চালায়নি।

আপনি যদি ওয়ার্কস্পেসের crate গুলোকে [crates.io](https://crates.io/)-তে publish করেন, তাহলে ওয়ার্কস্পেসের প্রতিটি crate-কে আলাদাভাবে publish করতে হবে। `Cargo test`-এর মতোই, আমরা `-p` flag ব্যবহার করে এবং যে crate-টি publish করতে চাই তার নাম specify করে আমাদের ওয়ার্কস্পেসের একটি particular crate publish করতে পারি।

অতিরিক্ত practice-এর জন্য, `add_one` crate-এর মতোই এই ওয়ার্কস্পেসে একটি `add_two` crate যোগ করুন!

আপনার প্রোজেক্ট যত বাড়বে, ওয়ার্কস্পেস ব্যবহার করার কথা বিবেচনা করুন: কোডের একটি বড় অংশের চেয়ে ছোট, individual component গুলো বোঝা সহজ। উপরন্তু, crate গুলোকে একটি ওয়ার্কস্পেসে রাখলে crate-গুলোর মধ্যে coordination সহজতর হতে পারে যদি সেগুলো প্রায়শই একই সময়ে পরিবর্তন করা হয়।
