# زبان برنامه‌نویسی Rust

[زبان برنامه‌نویسی راست](title-page.md)  
[پیش‌گفتار](foreword.md)  
[مقدمه](ch00-00-introduction.md)  

## شروع کار

- [شروع کار](ch01-00-getting-started.md)  
  - [نصب](ch01-01-installation.md)  
  - [سلام، دنیا!](ch01-02-hello-world.md)  
  - [سلام، Cargo!](ch01-03-hello-cargo.md)  

- [برنامه‌نویسی یک بازی حدس زدن](ch02-00-guessing-game-tutorial.md)  

- [مفاهیم رایج برنامه‌نویسی](ch03-00-common-programming-concepts.md)  
  - [متغیرها و تغییرپذیری](ch03-01-variables-and-mutability.md)  
  - [انواع داده](ch03-02-data-types.md)  
  - [توابع](ch03-03-how-functions-work.md)  
  - [کامنت‌ها](ch03-04-comments.md)  
  - [کنترل جریان](ch03-05-control-flow.md)  

- [درک مالکیت](ch04-00-understanding-ownership.md)  
  - [مالکیت چیست؟](ch04-01-what-is-ownership.md)  
  - [ارجاعات و قرض گرفتن](ch04-02-references-and-borrowing.md)  
  - [نوع Slice](ch04-03-slices.md)  

- [استفاده از Structها برای سازماندهی داده‌های مرتبط](ch05-00-structs.md)  
  - [تعریف و نمونه‌سازی Structها](ch05-01-defining-structs.md)  
  - [یک برنامه نمونه با استفاده از Structها](ch05-02-example-structs.md)  
  - [متد](ch05-03-method-syntax.md)  

- [شمارنده‌ها و تطابق الگو](ch06-00-enums.md)  
  - [تعریف یک Enum](ch06-01-defining-an-enum.md)  
  - [ساختار جریان کنترل `match`](ch06-02-match.md)  
  - [جریان کنترل مختصر با `if let` و `let else`](ch06-03-if-let.md)  

## سواد پایه Rust

- [مدیریت پروژه‌های در حال رشد با پکیج‌ها، جعبه‌ها (crates)، و ماژول‌ها](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)  
  - [پکیج‌ها و جعبه‌ها (crates)](ch07-01-packages-and-crates.md)  
  - [تعریف ماژول‌ها برای کنترل دامنه و حریم خصوصی](ch07-02-defining-modules-to-control-scope-and-privacy.md)  
  - [مسیرها برای ارجاع به یک آیتم در درخت ماژول](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)  
  - [وارد کردن مسیرها با کلمه کلیدی `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)  
  - [جداسازی ماژول‌ها به فایل‌های مختلف](ch07-05-separating-modules-into-different-files.md)  

- [مجموعه‌های رایج](ch08-00-common-collections.md)  
  - [ذخیره لیست مقادیر با بردارها](ch08-01-vectors.md)  
  - [ذخیره متن رمزگذاری‌شده UTF-8 با رشته‌ها](ch08-02-strings.md)  
  - [ذخیره کلیدها با مقادیر مرتبط در نقشه‌های هش](ch08-03-hash-maps.md)  

- [مدیریت خطاها](ch09-00-error-handling.md)  
  - [خطاهای غیرقابل بازیابی با `panic!`](ch09-01-unrecoverable-errors-with-panic.md)  
  - [خطاهای قابل بازیابی با `Result`](ch09-02-recoverable-errors-with-result.md)  
  - [آیا از `panic!` استفاده کنیم یا نه؟](ch09-03-to-panic-or-not-to-panic.md)  

- [انواع جنریک، صفت‌ها، و طول عمرها](ch10-00-generics.md)  
  - [انواع داده جنریک](ch10-01-syntax.md)  
  - [صفت‌ها: تعریف رفتارهای مشترک](ch10-02-traits.md)  
  - [اعتبارسنجی ارجاعات با طول عمرها](ch10-03-lifetime-syntax.md)  

- [نوشتن تست‌های خودکار](ch11-00-testing.md)  
  - [چگونه تست بنویسیم](ch11-01-writing-tests.md)  
  - [کنترل نحوه اجرای تست‌ها](ch11-02-running-tests.md)  
  - [سازماندهی تست‌ها](ch11-03-test-organization.md)  

- [یک پروژه I/O: ساخت یک برنامه خط فرمان](ch12-00-an-io-project.md)  
  - [پذیرش آرگومان‌های خط فرمان](ch12-01-accepting-command-line-arguments.md)  
  - [خواندن یک فایل](ch12-02-reading-a-file.md)  
  - [بازسازی برای بهبود ماژولاریت و مدیریت خطا](ch12-03-improving-error-handling-and-modularity.md)  
  - [توسعه قابلیت‌های کتابخانه با توسعه مبتنی بر تست](ch12-04-testing-the-librarys-functionality.md)  
  - [کار با متغیرهای محیطی](ch12-05-working-with-environment-variables.md)  
  - [نوشتن پیام‌های خطا به خروجی خطا به جای خروجی استاندارد](ch12-06-writing-to-stderr-instead-of-stdout.md)  

## تفکر در Rust

- [ویژگی‌های زبان‌های تابعی: Iteratorها و Closureها](ch13-00-functional-features.md)  
  - [Closureها: توابع ناشناخته که محیط خود را می‌گیرند](ch13-01-closures.md)  
  - [پردازش یک سری آیتم با Iteratorها](ch13-02-iterators.md)  
  - [بهبود پروژه I/O ما](ch13-03-improving-our-io-project.md)  
  - [مقایسه عملکرد: حلقه‌ها در مقابل Iteratorها](ch13-04-performance.md)  

- [بیشتر درباره Cargo و Crates.io](ch14-00-more-about-cargo.md)  
  - [سفارشی‌سازی بیلدها با پروفایل‌های انتشار](ch14-01-release-profiles.md)  
  - [منتشر کردن یک crate در Crates.io](ch14-02-publishing-to-crates-io.md)  
  - [محیط‌های کاری Cargo](ch14-03-cargo-workspaces.md)  
  - [نصب باینری‌ها از Crates.io با `cargo install`](ch14-04-installing-binaries.md)  
  - [گسترش Cargo با دستورات سفارشی](ch14-05-extending-cargo.md)  

- [اشاره‌گرهای هوشمند (Smart Pointers)](ch15-00-smart-pointers.md)  
  - [استفاده از `Box<T>` برای اشاره به داده‌ها در Heap](ch15-01-box.md)  
  - [رفتار اشاره‌گر (Pointer)های هوشمند مانند ارجاعات معمولی با صفت `Deref`](ch15-02-deref.md)  
  - [اجرای کد هنگام پاک‌سازی با صفت `Drop`](ch15-03-drop.md)  
  - [`Rc<T>`، اشاره‌گر (Pointer) هوشمند با شمارش ارجاعات](ch15-04-rc.md)  
  - [`RefCell<T>` و الگوی تغییرپذیری داخلی](ch15-05-interior-mutability.md)  
  - [چرخه‌های ارجاع می‌توانند باعث نشت حافظه شوند](ch15-06-reference-cycles.md)  

- [همزمانی بدون ترس](ch16-00-concurrency.md)  
  - [استفاده از نخ‌ها برای اجرای همزمان کد](ch16-01-threads.md)  
  - [استفاده از پیام‌رسانی برای انتقال داده بین نخ‌ها](ch16-02-message-passing.md)  
  - [همزمانی با وضعیت مشترک](ch16-03-shared-state.md)  
  - [همزمانی قابل گسترش با صفت‌های `Sync` و `Send`](ch16-04-extensible-concurrency-sync-and-send.md)  

- [Async و Await](ch17-00-async-await.md)  
  - [Futures و نحو Async](ch17-01-futures-and-syntax.md)  
  - [همزمانی با Async](ch17-02-concurrency-with-async.md)  
  - [کار با تعداد دلخواهی از Futures](ch17-03-more-futures.md)  
  - [Streams](ch17-04-streams.md)  
  - [بررسی عمیق صفت‌ها برای Async](ch17-05-traits-for-async.md)  
  - [Futures، Tasks، و Threads](ch17-06-futures-tasks-threads.md)  

- [ویژگی‌های برنامه‌نویسی شی‌گرا در Rust](ch18-00-oop.md)  
  - [ویژگی‌های زبان‌های شی‌گرا](ch18-01-what-is-oo.md)  
  - [استفاده از صفت‌های شی‌گرا برای مقادیر از تایپ‌های مختلف](ch18-02-trait-objects.md)  
  - [پیاده‌سازی یک الگوی طراحی شی‌گرا](ch18-03-oo-design-patterns.md)  

## مباحث پیشرفته

- [الگوها و تطبیق](ch19-00-patterns.md)  
  - [تمام مکان‌هایی که می‌توان از الگوها استفاده کرد](ch19-01-all-the-places-for-patterns.md)  
  - [قابلیت رد: آیا ممکن است یک الگو با مقدار مطابقت نداشته باشد؟](ch19-02-refutability.md)  
  - [نحو الگوها](ch19-03-pattern-syntax.md)  


- [ویژگی‌های پیشرفته](ch20-00-advanced-features.md)
  - [Rust ناایمن](ch20-01-unsafe-rust.md)
  - [Traits پیشرفته](ch20-02-advanced-traits.md)
  - [تایپ‌های پیشرفته](ch20-03-advanced-types.md)
  - [توابع و Closures پیشرفته](ch20-04-advanced-functions-and-closures.md)
  - [ماکروها](ch20-05-macros.md)

- [پروژه نهایی: ساخت یک وب‌سرور چندنخی](ch21-00-final-project-a-web-server.md)  
  - [ساخت یک وب‌سرور single-threaded](ch21-01-single-threaded.md)  
  - [تبدیل وب‌سرور تک‌نخی به وب‌سرور چندنخی](ch21-02-multithreaded.md)  
  - [خاموشی ملایم و پاک‌سازی](ch21-03-graceful-shutdown-and-cleanup.md)  

- [ضمیمه](appendix-00.md)  
  - [الف - کلمات کلیدی](appendix-01-keywords.md)  
  - [ب - عملگرها و نمادها](appendix-02-operators.md)  
  - [ج - صفت‌های قابل اشتقاق](appendix-03-derivable-traits.md)  
  - [د - ابزارهای توسعه مفید](appendix-04-useful-development-tools.md)  
  - [ه - نسخه‌ها](appendix-05-editions.md)  
  - [و - ترجمه‌های کتاب](appendix-06-translation.md)  
  - [ی - چگونه Rust ساخته می‌شود و "Nightly Rust"](appendix-07-nightly-rust.md)  
