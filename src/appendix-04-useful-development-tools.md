## পরিশিষ্ট D - দরকারী ডেভেলপমেন্ট টুল (Useful Development Tools)

এই পরিশিষ্টে, আমরা কিছু দরকারী ডেভেলপমেন্ট টুল নিয়ে আলোচনা করব যা Rust প্রোজেক্ট সরবরাহ করে। আমরা স্বয়ংক্রিয় ফরম্যাটিং, ওয়ার্নিং ফিক্স প্রয়োগ করার দ্রুত উপায়, একটি লিন্টার এবং IDE-এর সাথে ইন্টিগ্রেশন দেখব।

### `rustfmt` দিয়ে স্বয়ংক্রিয় ফরম্যাটিং (Automatic Formatting)

`rustfmt` টুলটি কমিউনিটি কোড স্টাইল অনুযায়ী আপনার কোড রিফরম্যাট করে। অনেক সহযোগিতামূলক প্রোজেক্ট Rust লেখার সময় কোন স্টাইল ব্যবহার করতে হবে তা নিয়ে তর্ক এড়াতে `rustfmt` ব্যবহার করে: প্রত্যেকে টুল ব্যবহার করে তাদের কোড ফরম্যাট করে।

`rustfmt` ইনস্টল করতে, নিম্নলিখিতটি লিখুন:

```console
$ rustup component add rustfmt
```

এই কমান্ডটি আপনাকে `rustfmt` এবং `cargo-fmt` দেয়, একইভাবে Rust আপনাকে `rustc` এবং `cargo` উভয়ই দেয়। যেকোনো Cargo প্রোজেক্ট ফরম্যাট করতে, নিম্নলিখিতটি লিখুন:

```console
$ cargo fmt
```

এই কমান্ডটি চালানো বর্তমান ক্রেটের সমস্ত Rust কোডকে রিফরম্যাট করে। এটি শুধুমাত্র কোড স্টাইল পরিবর্তন করবে, কোড শব্দার্থ নয়। `rustfmt` সম্পর্কে আরও তথ্যের জন্য, [এর ডকুমেন্টেশন][rustfmt] দেখুন।

[rustfmt]: https://github.com/rust-lang/rustfmt

### `rustfix` দিয়ে আপনার কোড ঠিক করুন (Fix Your Code)

`rustfix` টুলটি Rust ইনস্টলেশনের সাথে অন্তর্ভুক্ত এবং স্বয়ংক্রিয়ভাবে কম্পাইলার ওয়ার্নিংগুলি ঠিক করতে পারে যেগুলির সমস্যা সমাধানের একটি পরিষ্কার উপায় রয়েছে যা সম্ভবত আপনি চান। সম্ভবত আপনি আগেও কম্পাইলার ওয়ার্নিং দেখেছেন। উদাহরণস্বরূপ, এই কোডটি বিবেচনা করুন:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

এখানে, আমরা `x` ভেরিয়েবলটিকে মিউটেবল হিসাবে সংজ্ঞায়িত করছি, কিন্তু আমরা আসলে এটিকে কখনই মিউটেট করি না। Rust আমাদের সেই সম্পর্কে সতর্ক করে:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

ওয়ার্নিংটি সাজেস্ট করে যে আমরা `mut` কীওয়ার্ডটি সরিয়ে দিই। আমরা `cargo fix` কমান্ডটি চালিয়ে `rustfix` টুল ব্যবহার করে স্বয়ংক্রিয়ভাবে সেই সাজেশনটি প্রয়োগ করতে পারি:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

আমরা যখন আবার _src/main.rs_ দেখি, তখন আমরা দেখতে পাব যে `cargo fix` কোড পরিবর্তন করেছে:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

`x` ভেরিয়েবলটি এখন ইমিউটেবল, এবং ওয়ার্নিংটি আর দেখা যায় না।

আপনি বিভিন্ন Rust এডিশনের মধ্যে আপনার কোড পরিবর্তন করতে `cargo fix` কমান্ডটিও ব্যবহার করতে পারেন। এডিশনগুলি [পরিশিষ্ট E][editions]-এ কভার করা হয়েছে।

### Clippy-র সাথে আরও লিন্ট (More Lints with Clippy)

Clippy টুলটি আপনার কোড অ্যানালাইজ করার জন্য লিন্টের একটি সংগ্রহ, যাতে আপনি সাধারণ ভুলগুলি ধরতে পারেন এবং আপনার Rust কোড উন্নত করতে পারেন।

Clippy ইনস্টল করতে, নিম্নলিখিতটি লিখুন:

```console
$ rustup component add clippy
```

যেকোনো Cargo প্রোজেক্টে Clippy-র লিন্ট চালানোর জন্য, নিম্নলিখিতটি লিখুন:

```console
$ cargo clippy
```

উদাহরণস্বরূপ, ধরুন আপনি এমন একটি প্রোগ্রাম লেখেন যা একটি গাণিতিক কনস্ট্যান্টের আসন্ন মান ব্যবহার করে, যেমন pi, যেমনটি এই প্রোগ্রামটি করে:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

এই প্রোজেক্টে `cargo clippy` চালালে এই এররটি আসে:

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

এই এররটি আপনাকে জানায় যে Rust-এ ইতিমধ্যেই একটি আরও সুনির্দিষ্ট `PI` কনস্ট্যান্ট সংজ্ঞায়িত করা হয়েছে এবং আপনি যদি কনস্ট্যান্টটি ব্যবহার করেন তবে আপনার প্রোগ্রামটি আরও সঠিক হবে। তারপর আপনি `PI` কনস্ট্যান্ট ব্যবহার করার জন্য আপনার কোড পরিবর্তন করবেন। নিম্নলিখিত কোডটি Clippy থেকে কোনো এরর বা ওয়ার্নিং দেয় না:

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

IDE ইন্টিগ্রেশনে সাহায্য করার জন্য, Rust কমিউনিটি [`rust-analyzer`][rust-analyzer]<!-- ignore --> ব্যবহার করার পরামর্শ দেয়। এই টুলটি কম্পাইলার-কেন্দ্রিক ইউটিলিটিগুলির একটি সেট যা [ল্যাঙ্গুয়েজ সার্ভার প্রোটোকল][lsp]<!-- ignore -->-এ কথা বলে, যা IDE এবং প্রোগ্রামিং ভাষাগুলির একে অপরের সাথে যোগাযোগ করার জন্য একটি স্পেসিফিকেশন। বিভিন্ন ক্লায়েন্ট `rust-analyzer` ব্যবহার করতে পারে, যেমন [Visual Studio Code-এর জন্য Rust analyzer প্লাগ-ইন][vscode]।

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

ইনস্টলেশন নির্দেশাবলীর জন্য `rust-analyzer` প্রকল্পের [হোম পেজ][rust-analyzer]<!-- ignore --> দেখুন, তারপর আপনার নির্দিষ্ট IDE-তে ল্যাঙ্গুয়েজ সার্ভার সাপোর্ট ইনস্টল করুন। আপনার IDE অটোকমপ্লিশন, জাম্প টু ডেফিনিশন এবং ইনলাইন এরর-এর মতো ক্ষমতা অর্জন করবে।

[rust-analyzer]: https://rust-analyzer.github.io
[editions]: appendix-05-editions.md
