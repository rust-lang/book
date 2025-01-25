## পরিশিষ্ট D - প্রয়োজনীয় ডেভেলপমেন্ট সরঞ্জাম

এই পরিশিষ্টে, আমরা কিছু প্রয়োজনীয় ডেভেলপমেন্ট সরঞ্জাম নিয়ে আলোচনা করব যা Rust প্রোজেক্ট প্রদান করে। আমরা স্বয়ংক্রিয় ফরম্যাটিং, দ্রুত সতর্কবার্তা সমাধান করার উপায়, একটি লিন্টার এবং IDE-এর সাথে ইন্টিগ্রেশন নিয়ে দেখব।

### `rustfmt` দিয়ে স্বয়ংক্রিয় ফরম্যাটিং

`rustfmt` সরঞ্জামটি কমিউনিটি কোড স্টাইল অনুসারে আপনার কোডটিকে পুনরায় ফরম্যাট করে। অনেক সহযোগী প্রকল্প Rust লেখার সময় কোন শৈলী ব্যবহার করতে হবে তা নিয়ে বিতর্ক এড়াতে `rustfmt` ব্যবহার করে: প্রত্যেকে সরঞ্জামটি ব্যবহার করে তাদের কোড ফরম্যাট করে।

`rustfmt` ইনস্টল করতে, নিম্নলিখিতটি প্রবেশ করুন:

```console
rustup component add rustfmt
```

এই কমান্ডটি আপনাকে `rustfmt` এবং `cargo-fmt` দেয়, অনেকটা Rust আপনাকে `rustc` এবং `cargo` উভয়ই দেওয়ার মতোই। যেকোনো Cargo প্রকল্প ফরম্যাট করতে, নিম্নলিখিতটি প্রবেশ করুন:

```console
cargo fmt
```

এই কমান্ডটি চালানোর ফলে বর্তমান ক্রেটের সমস্ত Rust কোড পুনরায় ফরম্যাট হবে। এটি শুধুমাত্র কোড শৈলী পরিবর্তন করবে, কোডের শব্দার্থ নয়। `rustfmt` সম্পর্কে আরও তথ্যের জন্য, [এর ডকুমেন্টেশন][rustfmt] দেখুন।

[rustfmt]: https://github.com/rust-lang/rustfmt

### `rustfix` দিয়ে আপনার কোড ঠিক করুন

`rustfix` সরঞ্জামটি Rust ইনস্টলেশনের সাথে অন্তর্ভুক্ত থাকে এবং কম্পাইলারের সতর্কবার্তাগুলি স্বয়ংক্রিয়ভাবে ঠিক করতে পারে, যা সমস্যাটি সংশোধন করার একটি সুস্পষ্ট উপায় রয়েছে এবং সম্ভবত আপনি সেটাই চান। সম্ভবত আপনি আগে কম্পাইলারের সতর্কতা দেখেছেন। উদাহরণস্বরূপ, এই কোডটি বিবেচনা করুন:

<span class="filename">Filename: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

এখানে, আমরা `do_something` ফাংশনটিকে 100 বার কল করছি, কিন্তু আমরা `for` লুপের বডিতে `i` ভেরিয়েবলটি কখনই ব্যবহার করি না। Rust আমাদের সেই বিষয়ে সতর্ক করে:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 0..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

সতর্কবার্তায় পরামর্শ দেওয়া হয়েছে যে আমরা পরিবর্তে `_i` একটি নাম হিসাবে ব্যবহার করি: আন্ডারস্কোর নির্দেশ করে যে আমরা এই ভেরিয়েবলটি অব্যবহৃত রাখতে চাই। আমরা `cargo fix` কমান্ডটি চালিয়ে `rustfix` সরঞ্জামটি ব্যবহার করে স্বয়ংক্রিয়ভাবে সেই পরামর্শটি প্রয়োগ করতে পারি:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

যখন আমরা আবার _src/main.rs_ দেখি, তখন আমরা দেখব যে `cargo fix` কোডটি পরিবর্তন করেছে:

<span class="filename">Filename: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

`for` লুপ ভেরিয়েবলের নাম এখন `_i`, এবং সতর্কতাটি আর দেখা যাচ্ছে না।

আপনি বিভিন্ন Rust সংস্করণের মধ্যে আপনার কোড পরিবর্তন করতে `cargo fix` কমান্ডটিও ব্যবহার করতে পারেন। সংস্করণগুলি [পরিশিষ্ট E][editions]-এ আলোচনা করা হয়েছে।

### Clippy-এর সাথে আরও লিন্ট

Clippy টুলটি আপনার কোড বিশ্লেষণ করার জন্য লিন্টের একটি সংগ্রহ, যাতে আপনি সাধারণ ভুলগুলি ধরতে পারেন এবং আপনার Rust কোড উন্নত করতে পারেন।

Clippy ইনস্টল করতে, নিম্নলিখিতটি প্রবেশ করুন:

```console
rustup component add clippy
```

যেকোনো Cargo প্রোজেক্টে Clippy-এর লিন্টগুলি চালানোর জন্য, নিম্নলিখিতটি প্রবেশ করুন:

```console
cargo clippy
```

উদাহরণস্বরূপ, ধরুন আপনি এমন একটি প্রোগ্রাম লিখছেন যা গাণিতিক ধ্রুবকের আনুমানিক মান ব্যবহার করে, যেমন পাই, যেমন এই প্রোগ্রামটি করে:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

এই প্রোজেক্টে `cargo clippy` চালানোর ফলে এই এররটি আসে:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

এই এররটি আপনাকে জানায় যে Rust-এ ইতিমধ্যেই আরও সুনির্দিষ্ট `PI` ধ্রুবক সংজ্ঞায়িত করা আছে এবং আপনি যদি ধ্রুবকটি ব্যবহার করেন তবে আপনার প্রোগ্রামটি আরও সঠিক হবে। এর পরে আপনি আপনার কোডটিকে `PI` ধ্রুবক ব্যবহার করার জন্য পরিবর্তন করবেন। নিম্নলিখিত কোডটি Clippy থেকে কোনো এরর বা সতর্কতা দেখাবে না:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Clippy সম্পর্কে আরও তথ্যের জন্য, [এর ডকুমেন্টেশন][clippy] দেখুন।

[clippy]: https://github.com/rust-lang/rust-clippy

### `rust-analyzer` ব্যবহার করে IDE ইন্টিগ্রেশন

IDE ইন্টিগ্রেশনে সাহায্য করার জন্য, Rust কমিউনিটি [`rust-analyzer`][rust-analyzer] ব্যবহারের সুপারিশ করে। এই সরঞ্জামটি কম্পাইলার-কেন্দ্রিক ইউটিলিটিগুলির একটি সেট যা [Language Server Protocol][lsp] ব্যবহার করে, যা IDE এবং প্রোগ্রামিং ভাষাগুলির একে অপরের সাথে যোগাযোগের জন্য একটি স্পেসিফিকেশন। বিভিন্ন ক্লায়েন্ট `rust-analyzer` ব্যবহার করতে পারে, যেমন [Visual Studio Code-এর জন্য Rust analyzer প্লাগ-ইন][vscode]।

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

ইনস্টলেশন নির্দেশাবলীর জন্য `rust-analyzer` প্রোজেক্টের [হোম পেজ][rust-analyzer] দেখুন, তারপরে আপনার নির্দিষ্ট IDE-তে ভাষা সার্ভারের সমর্থন ইনস্টল করুন। আপনার IDE স্বয়ংক্রিয়ভাবে সম্পূর্ণ করা, সংজ্ঞায় পৌঁছানো এবং ইনলাইন এররের মতো ক্ষমতা অর্জন করবে।

[rust-analyzer]: https://rust-analyzer.github.io
[editions]: appendix-05-editions.md
