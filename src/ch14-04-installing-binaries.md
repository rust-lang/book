<!-- পুরানো লিঙ্ক, সরাবেন না -->

<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

## `cargo install`-এর সাহায্যে Crates.io থেকে বাইনারি ইন্সটল করা

`Cargo install` কমান্ড আপনাকে locally বাইনারি ক্রেট ইন্সটল এবং ব্যবহার করার অনুমতি দেয়। এটি সিস্টেম প্যাকেজগুলোকে replace করার উদ্দেশ্যে নয়; এটি Rust ডেভেলপারদের জন্য [crates.io](https://crates.io/)-তে অন্যদের share করা টুল ইন্সটল করার একটি সুবিধাজনক উপায়। মনে রাখবেন যে আপনি শুধুমাত্র সেই প্যাকেজগুলো ইন্সটল করতে পারবেন যেগুলোর বাইনারি টার্গেট রয়েছে। একটি _বাইনারি টার্গেট_ হল রানযোগ্য প্রোগ্রাম যা তৈরি হয় যদি ক্রেটটিতে একটি _src/main.rs_ ফাইল বা বাইনারি হিসেবে specified অন্য কোনো ফাইল থাকে, লাইব্রেরি টার্গেটের বিপরীতে যা নিজে থেকে রানযোগ্য নয় কিন্তু অন্যান্য প্রোগ্রামের মধ্যে include করার জন্য উপযুক্ত। সাধারণত, crate গুলোতে _README_ ফাইলে তথ্য থাকে যে একটি crate লাইব্রেরি, বাইনারি টার্গেট আছে, নাকি দুটোই।

`Cargo install` দিয়ে ইন্সটল করা সমস্ত বাইনারিগুলো ইন্সটলেশন রুটের _bin_ ফোল্ডারে store করা হয়। আপনি যদি _rustup.rs_ ব্যবহার করে Rust ইন্সটল করে থাকেন এবং কোনো কাস্টম কনফিগারেশন না থাকে, তাহলে এই ডিরেক্টরি হবে *$HOME/.cargo/bin*। আপনি `cargo install` দিয়ে ইন্সটল করা প্রোগ্রামগুলো চালাতে সক্ষম হওয়ার জন্য নিশ্চিত করুন যে ডিরেক্টরিটি আপনার `$PATH`-এ রয়েছে।

উদাহরণস্বরূপ, Chapter 12-এ আমরা উল্লেখ করেছি যে ফাইল সার্চ করার জন্য `ripgrep` নামক `grep` টুলের একটি Rust ইমপ্লিমেন্টেশন রয়েছে। `Ripgrep` ইন্সটল করতে, আমরা নিম্নলিখিতটি চালাতে পারি:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

আউটপুটের শেষের দিক থেকে দ্বিতীয় লাইনটি ইন্সটল করা বাইনারির location এবং নাম দেখায়, যেটি `ripgrep`-এর ক্ষেত্রে `rg`। যতক্ষণ ইন্সটলেশন ডিরেক্টরি আপনার `$PATH`-এ রয়েছে, যেমনটি আগে উল্লেখ করা হয়েছে, ততক্ষণ আপনি `rg --help` চালাতে পারেন এবং ফাইল সার্চ করার জন্য একটি দ্রুততর, rustier টুল ব্যবহার করা শুরু করতে পারেন!
