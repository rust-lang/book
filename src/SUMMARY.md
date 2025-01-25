# Rust প্রোগ্রামিং ভাষা

[The Rust Programming Language](title-page.md)
[ভূমিকা](foreword.md)
[সূচনা](ch00-00-introduction.md)

## শুরু করা

- [শুরু করা](ch01-00-getting-started.md)
    - [ইনস্টলেশন](ch01-01-installation.md)
    - [হ্যালো, ওয়ার্ল্ড!](ch01-02-hello-world.md)
    - [হ্যালো, কার্গো!](ch01-03-hello-cargo.md)

- [একটি গেসিং গেম প্রোগ্রামিং](ch02-00-guessing-game-tutorial.md)

- [সাধারণ প্রোগ্রামিং ধারণা](ch03-00-common-programming-concepts.md)
    - [ভেরিয়েবল এবং পরিবর্তনশীলতা](ch03-01-variables-and-mutability.md)
    - [ডেটা প্রকার](ch03-02-data-types.md)
    - [ফাংশন](ch03-03-how-functions-work.md)
    - [মন্তব্য](ch03-04-comments.md)
    - [কন্ট্রোল ফ্লো](ch03-05-control-flow.md)

- [মালিকানা বোঝা](ch04-00-understanding-ownership.md)
    - [মালিকানা কী?](ch04-01-what-is-ownership.md)
    - [রেফারেন্স এবং বরোইং](ch04-02-references-and-borrowing.md)
    - [স্লাইস টাইপ](ch04-03-slices.md)

- [সম্পর্কিত ডেটা গঠন করতে Struct ব্যবহার করা](ch05-00-structs.md)
    - [Struct সংজ্ঞায়িত এবং দৃষ্টান্ত তৈরি করা](ch05-01-defining-structs.md)
    - [Struct ব্যবহার করে একটি উদাহরণ প্রোগ্রাম](ch05-02-example-structs.md)
    - [মেথড সিনট্যাক্স](ch05-03-method-syntax.md)

- [Enum এবং প্যাটার্ন ম্যাচিং](ch06-00-enums.md)
    - [একটি Enum সংজ্ঞায়িত করা](ch06-01-defining-an-enum.md)
    - [`match` কন্ট্রোল ফ্লো কনস্ট্রাক্ট](ch06-02-match.md)
    - [`if let` এবং `let else` এর সাথে সংক্ষিপ্ত কন্ট্রোল ফ্লো](ch06-03-if-let.md)

## বেসিক Rust জ্ঞান

- [প্যাকেজ, ক্রেট এবং মডিউল সহ ক্রমবর্ধমান প্রকল্পগুলি পরিচালনা করা](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
    - [প্যাকেজ এবং ক্রেট](ch07-01-packages-and-crates.md)
    - [স্কোপ এবং গোপনীয়তা নিয়ন্ত্রণ করতে মডিউল সংজ্ঞায়িত করা](ch07-02-defining-modules-to-control-scope-and-privacy.md)
    - [মডিউল ট্রি-তে একটি আইটেম উল্লেখ করার জন্য পাথ](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
    - [`use` কীওয়ার্ডের সাথে স্কোপে পাথ আনা](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
    - [মডিউলগুলিকে বিভিন্ন ফাইলে আলাদা করা](ch07-05-separating-modules-into-different-files.md)

- [সাধারণ সংগ্রহ](ch08-00-common-collections.md)
    - [ভেক্টরগুলির সাথে মানগুলির তালিকা সংরক্ষণ করা](ch08-01-vectors.md)
    - [স্ট্রিংগুলির সাথে UTF-8 এনকোড করা পাঠ্য সংরক্ষণ করা](ch08-02-strings.md)
    - [হ্যাশ ম্যাপে সম্পর্কিত মানগুলির সাথে কী সংরক্ষণ করা](ch08-03-hash-maps.md)

- [ত্রুটি হ্যান্ডলিং](ch09-00-error-handling.md)
    - [`panic!` এর সাথে পুনরুদ্ধার করা যায় না এমন ত্রুটি](ch09-01-unrecoverable-errors-with-panic.md)
    - [`Result` সহ পুনরুদ্ধারযোগ্য ত্রুটি](ch09-02-recoverable-errors-with-result.md)
    - [`panic!` করা নাকি না করা?](ch09-03-to-panic-or-not-to-panic.md)

- [জেনেরিক প্রকার, trait এবং লাইফটাইম](ch10-00-generics.md)
    - [জেনেরিক ডেটা প্রকার](ch10-01-syntax.md)
    - [Traits: শেয়ার করা আচরণ সংজ্ঞায়িত করা](ch10-02-traits.md)
    - [লাইফটাইমের সাথে রেফারেন্স যাচাই করা](ch10-03-lifetime-syntax.md)

- [স্বয়ংক্রিয় পরীক্ষা লেখা](ch11-00-testing.md)
    - [কীভাবে পরীক্ষা লিখতে হয়](ch11-01-writing-tests.md)
    - [কীভাবে পরীক্ষা চালানো হয় তা নিয়ন্ত্রণ করা](ch11-02-running-tests.md)
    - [পরীক্ষা সংস্থা](ch11-03-test-organization.md)

- [একটি I/O প্রকল্প: একটি কমান্ড লাইন প্রোগ্রাম তৈরি করা](ch12-00-an-io-project.md)
    - [কমান্ড লাইন আর্গুমেন্ট গ্রহণ করা](ch12-01-accepting-command-line-arguments.md)
    - [একটি ফাইল পড়া](ch12-02-reading-a-file.md)
    - [মডুলারিটি এবং ত্রুটি হ্যান্ডলিং উন্নত করতে রিফ্যাক্টরিং](ch12-03-improving-error-handling-and-modularity.md)
    - [পরীক্ষা চালিত উন্নয়ন সহ লাইব্রেরির কার্যকারিতা বিকাশ করা](ch12-04-testing-the-librarys-functionality.md)
    - [পরিবেশের ভেরিয়েবলগুলির সাথে কাজ করা](ch12-05-working-with-environment-variables.md)
    - [স্ট্যান্ডার্ড আউটপুটের পরিবর্তে স্ট্যান্ডার্ড ত্রুটিতে ত্রুটি বার্তা লেখা](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Rust এ চিন্তা করা

- [কার্যকরী ভাষার বৈশিষ্ট্য: পুনরাবৃত্তিকারী এবং ক্লোজার](ch13-00-functional-features.md)
    - [ক্লোজার: বেনামী ফাংশন যা তাদের পরিবেশ ক্যাপচার করে](ch13-01-closures.md)
    - [পুনরাবৃত্তিকারীর সাথে আইটেমের একটি সিরিজ প্রক্রিয়াকরণ](ch13-02-iterators.md)
    - [আমাদের I/O প্রকল্পের উন্নতি](ch13-03-improving-our-io-project.md)
    - [কর্মক্ষমতা তুলনা: লুপ বনাম পুনরাবৃত্তিকারী](ch13-04-performance.md)

- [Cargo এবং Crates.io সম্পর্কে আরও](ch14-00-more-about-cargo.md)
    - [রিলিজ প্রোফাইলের সাথে বিল্ডগুলি কাস্টমাইজ করা](ch14-01-release-profiles.md)
    - [Crates.io-এ একটি ক্রেট প্রকাশ করা](ch14-02-publishing-to-crates-io.md)
    - [কার্গো ওয়ার্কস্পেস](ch14-03-cargo-workspaces.md)
    - [`cargo install` সহ Crates.io থেকে বাইনারি ইনস্টল করা](ch14-04-installing-binaries.md)
    - [কাস্টম কমান্ডের সাথে Cargo প্রসারিত করা](ch14-05-extending-cargo.md)

- [স্মার্ট পয়েন্টার](ch15-00-smart-pointers.md)
    - [হিপের ডেটার দিকে নির্দেশ করতে `Box<T>` ব্যবহার করা](ch15-01-box.md)
    - [`Deref` trait এর সাথে স্মার্ট পয়েন্টারগুলিকে সাধারণ রেফারেন্সের মতো ব্যবহার করা](ch15-02-deref.md)
    - [`Drop` trait এর সাথে পরিষ্কার করার সময় কোড চালানো](ch15-03-drop.md)
    - [`Rc<T>`, রেফারেন্স গণনা করা স্মার্ট পয়েন্টার](ch15-04-rc.md)
    - [`RefCell<T>` এবং অভ্যন্তরীণ পরিবর্তনশীলতা প্যাটার্ন](ch15-05-interior-mutability.md)
    - [রেফারেন্স চক্র মেমরি লিক করতে পারে](ch15-06-reference-cycles.md)

- [নির্ভীক কনকারেন্সি](ch16-00-concurrency.md)
    - [একযোগে কোড চালানোর জন্য থ্রেড ব্যবহার করা](ch16-01-threads.md)
    - [থ্রেডের মধ্যে ডেটা স্থানান্তর করতে বার্তা পাসিং ব্যবহার করা](ch16-02-message-passing.md)
    - [শেয়ার্ড-স্টেট কনকারেন্সি](ch16-03-shared-state.md)
    - [`Sync` এবং `Send` traits এর সাথে প্রসারিত কনকারেন্সি](ch16-04-extensible-concurrency-sync-and-send.md)

- [অ্যাসিঙ্ক্রোনাস প্রোগ্রামিংয়ের মৌলিক বিষয়: Async, Await, Futures এবং Streams](ch17-00-async-await.md)
    - [Futures এবং Async সিনট্যাক্স](ch17-01-futures-and-syntax.md)
    - [Async এর সাথে কনকারেন্সি প্রয়োগ করা](ch17-02-concurrency-with-async.md)
    - [যেকোনো সংখ্যক Futures এর সাথে কাজ করা](ch17-03-more-futures.md)
    - [স্ট্রিম: সিকোয়েন্সে Futures](ch17-04-streams.md)
    - [Async এর জন্য Traits এর দিকে আরও ভালোভাবে নজর](ch17-05-traits-for-async.md)
    - [Futures, কাজ এবং থ্রেড](ch17-06-futures-tasks-threads.md)

- [Rust এর অবজেক্ট ওরিয়েন্টেড প্রোগ্রামিং বৈশিষ্ট্য](ch18-00-oop.md)
    - [অবজেক্ট-ওরিয়েন্টেড ভাষার বৈশিষ্ট্য](ch18-01-what-is-oo.md)
    - [Trait অবজেক্ট ব্যবহার করা যা বিভিন্ন প্রকারের মানের জন্য অনুমতি দেয়](ch18-02-trait-objects.md)
    - [একটি অবজেক্ট-ওরিয়েন্টেড ডিজাইন প্যাটার্ন প্রয়োগ করা](ch18-03-oo-design-patterns.md)

## উন্নত বিষয়

- [প্যাটার্ন এবং ম্যাচিং](ch19-00-patterns.md)
    - [যেখানে প্যাটার্ন ব্যবহার করা যেতে পারে সেই সমস্ত স্থান](ch19-01-all-the-places-for-patterns.md)
    - [রেফুটেবিলিটি: একটি প্যাটার্ন মেলাতে ব্যর্থ হতে পারে কিনা](ch19-02-refutability.md)
    - [প্যাটার্ন সিনট্যাক্স](ch19-03-pattern-syntax.md)

- [উন্নত বৈশিষ্ট্য](ch20-00-advanced-features.md)
    - [আনসেফ Rust](ch20-01-unsafe-rust.md)
    - [উন্নত Traits](ch20-02-advanced-traits.md)
    - [উন্নত প্রকার](ch20-03-advanced-types.md)
    - [উন্নত ফাংশন এবং ক্লোজার](ch20-04-advanced-functions-and-closures.md)
    - [ম্যাক্রো](ch20-05-macros.md)

- [ফাইনাল প্রজেক্ট: একটি মাল্টিথ্রেডেড ওয়েব সার্ভার তৈরি করা](ch21-00-final-project-a-web-server.md)
    - [একটি সিঙ্গেল-থ্রেডেড ওয়েব সার্ভার তৈরি করা](ch21-01-single-threaded.md)
    - [আমাদের সিঙ্গেল-থ্রেডেড সার্ভারকে একটি মাল্টিথ্রেডেড সার্ভারে পরিণত করা](ch21-02-multithreaded.md)
    - [সুন্দরভাবে বন্ধ করা এবং পরিষ্কার করা](ch21-03-graceful-shutdown-and-cleanup.md)

- [পরিশিষ্ট](appendix-00.md)
    - [A - কীওয়ার্ড](appendix-01-keywords.md)
    - [B - অপারেটর এবং প্রতীক](appendix-02-operators.md)
    - [C - ডেরিভেবল traits](appendix-03-derivable-traits.md)
    - [D - দরকারী উন্নয়ন সরঞ্জাম](appendix-04-useful-development-tools.md)
    - [E - সংস্করণ](appendix-05-editions.md)
    - [F - বইটির অনুবাদ](appendix-06-translation.md)
    - [G - Rust কিভাবে তৈরি করা হয় এবং "নাইটলি Rust"](appendix-07-nightly-rust.md)
