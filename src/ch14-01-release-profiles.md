## Release Profile-এর সাহায্যে Build Customize করা

Rust-এ, _release profile_ গুলো হল predefined এবং customizable profile যেগুলোতে different configuration থাকে, যা একজন programmer-কে কোড compile করার জন্য বিভিন্ন option-এর উপর আরও বেশি control রাখার সুযোগ দেয়। প্রতিটি profile একে অপরের থেকে independently configure করা হয়।

Cargo-র দুটি প্রধান profile রয়েছে: `dev` প্রোফাইল যা Cargo ব্যবহার করে যখন আপনি `cargo build` চালান এবং `release` প্রোফাইল যা Cargo ব্যবহার করে যখন আপনি `cargo build --release` চালান। `Dev` প্রোফাইলটি development-এর জন্য ভালো default সহ define করা হয়েছে, এবং `release` প্রোফাইলে release build-গুলোর জন্য ভালো default রয়েছে।

এই profile-গুলোর নাম আপনার build-গুলোর output থেকে পরিচিত হতে পারে:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

`Dev` এবং `release` হল compiler-এর ব্যবহৃত এই different profile গুলো।

Cargo-র প্রতিটি profile-এর জন্য default setting রয়েছে যা প্রযোজ্য হয় যখন আপনি project-এর _Cargo.toml_ ফাইলে explicitly কোনো `[profile.*]` section যোগ করেননি। আপনি যে কোনো profile customize করতে চান তার জন্য `[profile.*]` section যোগ করে, আপনি default setting গুলোর যেকোনো subset override করেন। উদাহরণস্বরূপ, `dev` এবং `release` profile-গুলোর জন্য `opt-level` setting-এর default value গুলো এখানে দেওয়া হল:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`Opt-level` setting টি আপনার কোডে Rust যে সংখ্যক optimization apply করবে তা নিয়ন্ত্রণ করে, যার range হল 0 থেকে 3। আরও optimization apply করলে compiling time বাড়ে, তাই আপনি যদি development-এ থাকেন এবং প্রায়শই আপনার কোড compile করেন, তাহলে resulting code ধীরে চললেও আপনি দ্রুত compile করার জন্য কম optimization চাইবেন। তাই `dev`-এর জন্য default `opt-level` হল `0`। যখন আপনি আপনার কোড release করার জন্য প্রস্তুত হন, তখন compile করতে আরও বেশি সময় ব্যয় করা সবচেয়ে ভালো। আপনি release mode-এ শুধুমাত্র একবার compile করবেন, কিন্তু আপনি compiled program টি বহুবার চালাবেন, তাই release mode দ্রুততর কোডের জন্য দীর্ঘতর compile time-এর trade-off করে। সেই কারণেই `release` প্রোফাইলের জন্য ডিফল্ট `opt-level` হল `3`।

আপনি _Cargo.toml_-এ এটির জন্য একটি different value যোগ করে একটি default setting override করতে পারেন। উদাহরণস্বরূপ, যদি আমরা development profile-এ optimization level 1 ব্যবহার করতে চাই, তাহলে আমরা আমাদের প্রোজেক্টের _Cargo.toml_ ফাইলে এই দুটি লাইন যোগ করতে পারি:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

এই কোডটি `0`-এর default setting টিকে override করে। এখন যখন আমরা `cargo build` চালাই, তখন Cargo `dev` প্রোফাইলের জন্য default গুলো ব্যবহার করবে এবং তার সাথে `opt-level`-এ আমাদের customization যোগ করবে। যেহেতু আমরা `opt-level` কে `1`-এ set করেছি, তাই Cargo default-এর চেয়ে বেশি optimization apply করবে, কিন্তু release build-এর মতো ততটা নয়।

প্রতিটি প্রোফাইলের জন্য configuration option এবং default-গুলোর complete list-এর জন্য, [Cargo’s documentation](https://doc.rust-lang.org/cargo/reference/profiles.html) দেখুন।
