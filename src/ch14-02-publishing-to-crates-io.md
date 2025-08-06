## انتشار یک Crate در Crates.io

ما از پکیج‌های موجود در [crates.io](https://crates.io/)<!-- ignore --> به عنوان وابستگی‌های پروژه خود استفاده کرده‌ایم، اما شما همچنین می‌توانید کد خود را با دیگران به اشتراک بگذارید با انتشار پکیج‌های خودتان. رجیستری Crates.io کد منبع پکیج‌های شما را توزیع می‌کند، بنابراین به طور عمده میزبان کدهای منبع باز است.

Rust و Cargo ویژگی‌هایی دارند که باعث می‌شود پکیج منتشرشده شما برای دیگران راحت‌تر پیدا شده و استفاده شود. ما ابتدا درباره برخی از این ویژگی‌ها صحبت می‌کنیم و سپس توضیح می‌دهیم چگونه یک پکیج منتشر کنیم.

### ایجاد نظرات مستندات مفید

مستندسازی دقیق پکیج‌های شما به دیگر کاربران کمک می‌کند بدانند چگونه و چه زمانی از آن‌ها استفاده کنند، بنابراین ارزش دارد که وقت خود را برای نوشتن مستندات صرف کنید. در فصل 3، نحوه اضافه کردن نظرات به کد Rust با استفاده از دو اسلش `//` را بررسی کردیم. Rust همچنین نوع خاصی از نظرات برای مستندات دارد که به نام _نظرات مستندات_ شناخته می‌شود و مستندات HTML تولید می‌کند. این مستندات HTML محتوای نظرات مستندات را برای آیتم‌های عمومی API نمایش می‌دهد که برای برنامه‌نویسانی که به دنبال دانستن چگونگی _استفاده از_ crate شما هستند، طراحی شده است و نه چگونگی _پیاده‌سازی_ crate شما.

نظرات مستندات به جای دو اسلش از سه اسلش `///` استفاده می‌کنند و از نشانه‌گذاری Markdown برای قالب‌بندی متن پشتیبانی می‌کنند. نظرات مستندات را درست قبل از آیتمی که قرار است مستندسازی شود قرار دهید. لیستینگ 14-1 نظرات مستندات برای یک تابع `add_one` در یک crate به نام `my_crate` را نشان می‌دهد.

<Listing number="14-1" file-name="src/lib.rs" caption="یک نظر مستند برای یک تابع">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

</Listing>

اینجا، ما توضیحی درباره عملکرد تابع `add_one` می‌دهیم، بخشی با عنوان `Examples` شروع می‌کنیم، و سپس کدی که نشان می‌دهد چگونه از تابع `add_one` استفاده کنیم ارائه می‌دهیم. می‌توانیم مستندات HTML را از این نظر مستند با اجرای دستور `cargo doc` تولید کنیم. این دستور ابزار `rustdoc` که با Rust توزیع شده را اجرا می‌کند و مستندات HTML تولیدشده را در دایرکتوری _target/doc_ قرار می‌دهد.

<<<<<<< HEAD
برای راحتی، اجرای دستور `cargo doc --open` مستندات HTML را برای crate فعلی شما (و همچنین مستندات همه وابستگی‌های crate شما) می‌سازد و نتیجه را در مرورگر وب باز می‌کند. به تابع `add_one` بروید و خواهید دید که چگونه متن موجود در نظرات مستندات نمایش داده می‌شود، همانطور که در شکل 14-1 نشان داده شده است:
=======
For convenience, running `cargo doc --open` will build the HTML for your
current crate’s documentation (as well as the documentation for all of your
crate’s dependencies) and open the result in a web browser. Navigate to the
`add_one` function and you’ll see how the text in the documentation comments is
rendered, as shown in Figure 14-1.
>>>>>>> upstream/main

<img alt="مستندات HTML تولیدشده برای تابع `add_one` از `my_crate`" src="img/trpl14-01.png" class="center" />

<span class="caption">شکل 14-1: مستندات HTML برای تابع `add_one`</span>

#### بخش‌های متداول مورد استفاده

ما در لیستینگ 14-1 از عنوان Markdown `# Examples` برای ایجاد یک بخش در HTML با عنوان "Examples" استفاده کردیم. در اینجا برخی دیگر از بخش‌هایی که نویسندگان crate معمولاً در مستندات خود استفاده می‌کنند آورده شده است:

- **Panics**: سناریوهایی که در آن ممکن است تابع مستند شده باعث ایجاد panic شود. فراخوانان تابع که نمی‌خواهند برنامه‌هایشان panic کنند باید مطمئن شوند که تابع را در این شرایط فراخوانی نمی‌کنند.
- **Errors**: اگر تابع یک مقدار `Result` بازگرداند، توضیح انواع خطاهایی که ممکن است رخ دهد و شرایطی که ممکن است این خطاها را ایجاد کند، برای فراخوانان مفید است تا بتوانند کدهایی برای مدیریت انواع مختلف خطاها بنویسند.
- **Safety**: اگر تابع `unsafe` برای فراخوانی باشد (ما عدم ایمنی را در فصل 20 بررسی خواهیم کرد)، باید بخشی توضیح دهد که چرا تابع ناامن است و اصولی را که تابع از فراخوانان انتظار دارد رعایت کنند پوشش دهد.

بیشتر نظرات مستندات به همه این بخش‌ها نیاز ندارند، اما این یک چک‌لیست خوب برای یادآوری جنبه‌هایی از کد شما است که کاربران علاقه‌مند به دانستن آن هستند.

#### نظرات مستندات به عنوان تست

<<<<<<< HEAD
اضافه کردن بلوک‌های کد مثال به نظرات مستندات شما می‌تواند به نمایش نحوه استفاده از کتابخانه شما کمک کند، و انجام این کار یک مزیت اضافی دارد: اجرای دستور `cargo test`، مثال‌های کد در مستندات شما را به عنوان تست اجرا خواهد کرد! هیچ چیزی بهتر از مستندات با مثال نیست. اما هیچ چیزی بدتر از مثال‌هایی نیست که کار نمی‌کنند زیرا کد از زمان نوشته شدن مستندات تغییر کرده است. اگر `cargo test` را با مستندات تابع `add_one` از لیستینگ 14-1 اجرا کنیم، بخشی در نتایج تست مانند زیر خواهیم دید:
=======
Adding example code blocks in your documentation comments can help demonstrate
how to use your library, and doing so has an additional bonus: running `cargo
test` will run the code examples in your documentation as tests! Nothing is
better than documentation with examples. But nothing is worse than examples
that don’t work because the code has changed since the documentation was
written. If we run `cargo test` with the documentation for the `add_one`
function from Listing 14-1, we will see a section in the test results that looks
like this:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->
>>>>>>> upstream/main

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

<<<<<<< HEAD
اکنون، اگر تابع یا مثال را تغییر دهیم به طوری که `assert_eq!` در مثال باعث panic شود و دوباره `cargo test` را اجرا کنیم، خواهیم دید که تست‌های مستندات تشخیص می‌دهند که مثال و کد با یکدیگر همگام نیستند!
=======
Now, if we change either the function or the example so the `assert_eq!` in the
example panics, and run `cargo test` again, we’ll see that the doc tests catch
that the example and the code are out of sync with each other!
>>>>>>> upstream/main

#### مستندسازی آیتم‌های شامل شده

<<<<<<< HEAD
سبک نظر مستند `//!` مستندات را به آیتمی که نظرات را شامل می‌شود اضافه می‌کند، به جای آیتم‌هایی که بعد از نظرات قرار دارند. ما معمولاً از این نظرات مستند در فایل اصلی crate (_src/lib.rs_ بر اساس قرارداد) یا در داخل یک ماژول برای مستندسازی کل crate یا ماژول استفاده می‌کنیم.

برای مثال، برای اضافه کردن مستنداتی که هدف crate `my_crate` را که شامل تابع `add_one` است توضیح می‌دهد، نظرات مستندی که با `//!` شروع می‌شوند را به ابتدای فایل _src/lib.rs_ اضافه می‌کنیم، همان‌طور که در لیستینگ 14-2 نشان داده شده است:
=======
The style of doc comment `//!` adds documentation to the item that *contains*
the comments rather than to the items *following* the comments. We typically use
these doc comments inside the crate root file (_src/lib.rs_ by convention) or
inside a module to document the crate or the module as a whole.

For example, to add documentation that describes the purpose of the `my_crate`
crate that contains the `add_one` function, we add documentation comments that
start with `//!` to the beginning of the _src/lib.rs_ file, as shown in Listing
14-2.
>>>>>>> upstream/main

<Listing number="14-2" file-name="src/lib.rs" caption="مستندات برای کل crate `my_crate`">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

</Listing>

توجه داشته باشید که هیچ کدی بعد از آخرین خطی که با `//!` شروع می‌شود وجود ندارد. چون ما نظرات را با `//!` شروع کرده‌ایم به جای `///`، ما در حال مستندسازی آیتمی هستیم که این نظر را شامل می‌شود به جای آیتمی که بعد از این نظر قرار دارد. در این مورد، آن آیتم فایل _src/lib.rs_ است که ریشه crate است. این نظرات کل crate را توضیح می‌دهند.

<<<<<<< HEAD
وقتی `cargo doc --open` را اجرا می‌کنیم، این نظرات در صفحه اول مستندات crate `my_crate` بالای لیست آیتم‌های عمومی در crate نمایش داده می‌شوند، همان‌طور که در شکل 14-2 نشان داده شده است:
=======
When we run `cargo doc --open`, these comments will display on the front
page of the documentation for `my_crate` above the list of public items in the
crate, as shown in Figure 14-2.
>>>>>>> upstream/main

<img alt="Rendered HTML documentation with a comment for the crate as a whole" src="img/trpl14-02.png" class="center" />

<span class="caption">شکل 14-2: مستندات تولید شده برای `my_crate`، شامل توضیحات در مورد کل crate</span>

نظرات مستندات داخل آیتم‌ها به ویژه برای توصیف crates و ماژول‌ها مفید هستند. از آن‌ها برای توضیح هدف کلی container استفاده کنید تا به کاربران خود در درک سازمان‌دهی crate کمک کنید.

### صادرات یک API عمومی کارآمد با استفاده از `pub use`

ساختار API عمومی شما یک موضوع مهم هنگام انتشار یک crate است. افرادی که از crate شما استفاده می‌کنند، کمتر از شما با ساختار آن آشنا هستند و ممکن است در یافتن قسمت‌هایی که می‌خواهند استفاده کنند، اگر crate شما دارای یک سلسله‌مراتب ماژول بزرگ باشد، دچار مشکل شوند.

<<<<<<< HEAD
در فصل 7، نحوه عمومی کردن آیتم‌ها با استفاده از کلمه کلیدی `pub` و آوردن آیتم‌ها به یک scope با استفاده از کلمه کلیدی `use` را پوشش دادیم. با این حال، ساختاری که هنگام توسعه یک crate برای شما منطقی به نظر می‌رسد ممکن است برای کاربران شما چندان مناسب نباشد. ممکن است بخواهید ساختارهای خود را در یک سلسله‌مراتب با چندین سطح سازماندهی کنید، اما سپس افرادی که می‌خواهند از یک نوع تعریف‌شده عمیق در سلسله‌مراتب استفاده کنند ممکن است در پیدا کردن آن نوع دچار مشکل شوند. همچنین ممکن است مجبور شوند به جای `use my_crate::UsefulType;`، چیزی مانند `use my_crate::some_module::another_module::UsefulType;` بنویسند که ناخوشایند است.

خبر خوب این است که اگر ساختار _برای دیگران راحت نیست_، نیازی نیست سازمان‌دهی داخلی خود را دوباره بچینید: به جای آن می‌توانید آیتم‌ها را با استفاده از `pub use` مجدداً صادر کنید تا یک ساختار عمومی متفاوت از ساختار خصوصی خود ایجاد کنید. صادرات مجدد یک آیتم عمومی در یک مکان را می‌گیرد و آن را در یک مکان دیگر عمومی می‌کند، گویی که در مکان دیگر تعریف شده است.

برای مثال، فرض کنید ما یک کتابخانه به نام `art` برای مدل‌سازی مفاهیم هنری ایجاد کرده‌ایم. در این کتابخانه دو ماژول وجود دارند: یک ماژول `kinds` که شامل دو enum به نام‌های `PrimaryColor` و `SecondaryColor` است و یک ماژول `utils` که شامل یک تابع به نام `mix` است، همان‌طور که در لیستینگ 14-3 نشان داده شده است:
=======
In Chapter 7, we covered how to make items public using the `pub` keyword, and
how to bring items into a scope with the `use` keyword. However, the structure
that makes sense to you while you’re developing a crate might not be very
convenient for your users. You might want to organize your structs in a
hierarchy containing multiple levels, but then people who want to use a type
you’ve defined deep in the hierarchy might have trouble finding out that type
exists. They might also be annoyed at having to enter `use
my_crate::some_module::another_module::UsefulType;` rather than `use
my_crate::UsefulType;`.

The good news is that if the structure _isn’t_ convenient for others to use
from another library, you don’t have to rearrange your internal organization:
instead, you can re-export items to make a public structure that’s different
from your private structure by using `pub use`. *Re-exporting* takes a public
item in one location and makes it public in another location, as if it were
defined in the other location instead.

For example, say we made a library named `art` for modeling artistic concepts.
Within this library are two modules: a `kinds` module containing two enums
named `PrimaryColor` and `SecondaryColor` and a `utils` module containing a
function named `mix`, as shown in Listing 14-3.
>>>>>>> upstream/main

<Listing number="14-3" file-name="src/lib.rs" caption="یک کتابخانه `art` با آیتم‌هایی که در ماژول‌های `kinds` و `utils` سازماندهی شده‌اند">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

</Listing>

<<<<<<< HEAD
شکل 14-3 نشان می‌دهد که صفحه اول مستندات این crate که توسط `cargo doc` تولید شده است چگونه به نظر می‌رسد:
=======
Figure 14-3 shows what the front page of the documentation for this crate
generated by `cargo doc` would look like.
>>>>>>> upstream/main

<img alt="مستندات تولید شده برای crate `art` که ماژول‌های `kinds` و `utils` را لیست می‌کند" src="img/trpl14-03.png" class="center" />

<span class="caption">شکل 14-3: صفحه اول مستندات crate `art` که ماژول‌های `kinds` و `utils` را لیست می‌کند</span>

توجه کنید که انواع `PrimaryColor` و `SecondaryColor` در صفحه اول لیست نشده‌اند، و تابع `mix` نیز لیست نشده است. برای دیدن آن‌ها باید روی `kinds` و `utils` کلیک کنیم.

<<<<<<< HEAD
یک crate دیگر که به این کتابخانه وابسته است نیاز دارد که بیانیه‌های `use` مشخص کنند که آیتم‌ها را از `art` به scope می‌آورند، و ساختار ماژول تعریف‌شده کنونی را بیان کنند. لیستینگ 14-4 یک مثال از crate‌ای که آیتم‌های `PrimaryColor` و `mix` را از crate `art` استفاده می‌کند نشان می‌دهد:
=======
Another crate that depends on this library would need `use` statements that
bring the items from `art` into scope, specifying the module structure that’s
currently defined. Listing 14-4 shows an example of a crate that uses the
`PrimaryColor` and `mix` items from the `art` crate.
>>>>>>> upstream/main

<Listing number="14-4" file-name="src/main.rs" caption="A crate using the `art` crate’s items with its internal structure exported">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

</Listing>

نویسنده کدی که در لیستینگ 14-4 نشان داده شده و از crate `art` استفاده می‌کند، مجبور بوده متوجه شود که `PrimaryColor` در ماژول `kinds` و `mix` در ماژول `utils` قرار دارد. ساختار ماژول crate `art` بیشتر برای توسعه‌دهندگانی که روی این crate کار می‌کنند مرتبط است تا کسانی که از آن استفاده می‌کنند. ساختار داخلی اطلاعات مفیدی برای کسی که می‌خواهد نحوه استفاده از crate `art` را بفهمد ارائه نمی‌دهد، بلکه بیشتر باعث سردرگمی می‌شود، زیرا توسعه‌دهندگانی که از آن استفاده می‌کنند باید بفهمند کجا را باید جستجو کنند و نام‌های ماژول را در بیانیه‌های `use` مشخص کنند.

<<<<<<< HEAD
برای حذف سازمان‌دهی داخلی از API عمومی، می‌توانیم کد crate `art` را در لیستینگ 14-3 تغییر دهیم تا بیانیه‌های `pub use` را برای صادرات مجدد آیتم‌ها در سطح بالا اضافه کنیم، همان‌طور که در لیستینگ 14-5 نشان داده شده است:
=======
To remove the internal organization from the public API, we can modify the
`art` crate code in Listing 14-3 to add `pub use` statements to re-export the
items at the top level, as shown in Listing 14-5.
>>>>>>> upstream/main

<Listing number="14-5" file-name="src/lib.rs" caption="افزودن بیانیه‌های `pub use` برای صادرات مجدد آیتم‌ها">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

</Listing>

مستندات API که `cargo doc` برای این crate تولید می‌کند اکنون صادرات‌های مجدد را در صفحه اول لیست کرده و به آن‌ها لینک می‌دهد، همان‌طور که در شکل 14-4 نشان داده شده است. این کار پیدا کردن انواع `PrimaryColor` و `SecondaryColor` و تابع `mix` را آسان‌تر می‌کند.

<img alt="مستندات تولیدشده برای crate `art` با صادرات‌های مجدد در صفحه اول" src="img/trpl14-04.png" class="center" />

<span class="caption">شکل 14-4: صفحه اول مستندات برای crate `art` که صادرات‌های مجدد را لیست می‌کند</span>

<<<<<<< HEAD
کاربران crate `art` همچنان می‌توانند ساختار داخلی را از لیستینگ 14-3 ببینند و استفاده کنند، همان‌طور که در لیستینگ 14-4 نشان داده شده است، یا می‌توانند از ساختار راحت‌تر در لیستینگ 14-5 استفاده کنند، همان‌طور که در لیستینگ 14-6 نشان داده شده است:
=======
The `art` crate users can still see and use the internal structure from Listing
14-3 as demonstrated in Listing 14-4, or they can use the more convenient
structure in Listing 14-5, as shown in Listing 14-6.
>>>>>>> upstream/main

<Listing number="14-6" file-name="src/main.rs" caption="یک برنامه که از آیتم‌های صادرات‌شده مجدد crate `art` استفاده می‌کند">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

</Listing>

در مواردی که ماژول‌های تو در تو زیادی وجود دارند، صادرات مجدد انواع در سطح بالا با `pub use` می‌تواند تفاوت بزرگی در تجربه افرادی که از crate استفاده می‌کنند ایجاد کند. یکی دیگر از استفاده‌های رایج `pub use`، صادرات مجدد تعاریف یک وابستگی در crate فعلی برای تبدیل تعاریف آن به بخشی از API عمومی crate شما است.

ایجاد یک ساختار API عمومی مفید بیشتر شبیه یک هنر است تا یک علم، و می‌توانید با آزمون و خطا API‌ای پیدا کنید که بهترین کارکرد را برای کاربران شما داشته باشد. انتخاب `pub use` به شما انعطاف می‌دهد که چگونه crate خود را به صورت داخلی ساختار دهید و آن ساختار داخلی را از چیزی که به کاربران خود ارائه می‌دهید جدا کنید. به برخی از کدهای crate‌هایی که نصب کرده‌اید نگاهی بیندازید تا ببینید آیا ساختار داخلی آن‌ها با API عمومی آن‌ها تفاوت دارد یا خیر.

### تنظیم یک حساب در Crates.io

قبل از اینکه بتوانید هر crate‌ای را منتشر کنید، نیاز دارید که یک حساب در [crates.io](https://crates.io/)<!-- ignore --> ایجاد کنید و یک توکن API دریافت کنید. برای این کار، به صفحه اصلی در [crates.io](https://crates.io/)<!-- ignore --> بروید و از طریق حساب GitHub وارد شوید. (در حال حاضر حساب GitHub یک نیاز است، اما ممکن است سایت در آینده از روش‌های دیگری برای ایجاد حساب پشتیبانی کند.) پس از ورود به سیستم، به تنظیمات حساب خود در [https://crates.io/me/](https://crates.io/me/)<!-- ignore --> بروید و کلید API خود را دریافت کنید. سپس دستور `cargo login` را اجرا کرده و کلید API خود را وارد کنید، مانند این:

```console
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```

<<<<<<< HEAD
این دستور Cargo را از توکن API شما مطلع کرده و آن را به صورت محلی در فایل _~/.cargo/credentials_ ذخیره می‌کند. توجه داشته باشید که این توکن یک _راز_ است: آن را با هیچ‌کس دیگری به اشتراک نگذارید. اگر به هر دلیلی این توکن را با کسی به اشتراک گذاشتید، باید آن را لغو کنید و یک توکن جدید در [crates.io](https://crates.io/)<!-- ignore --> ایجاد کنید.
=======
This command will inform Cargo of your API token and store it locally in
_~/.cargo/credentials.toml_. Note that this token is a _secret_: do not share
it with anyone else. If you do share it with anyone for any reason, you should
revoke it and generate a new token on [crates.io](https://crates.io/)<!-- ignore
-->.
>>>>>>> upstream/main

### افزودن متادیتا به یک Crate جدید

فرض کنید یک crate دارید که می‌خواهید منتشر کنید. قبل از انتشار، نیاز دارید که برخی متادیتا را در بخش `[package]` فایل _Cargo.toml_ crate خود اضافه کنید.

crate شما باید یک نام منحصر به فرد داشته باشد. در حالی که به صورت محلی روی یک crate کار می‌کنید، می‌توانید هر نامی که دوست دارید برای crate خود انتخاب کنید. با این حال، نام‌های crate در [crates.io](https://crates.io/)<!-- ignore --> به صورت اولین درخواست‌کننده تخصیص داده می‌شوند. هنگامی که یک نام برای یک crate گرفته شود، هیچ کس دیگری نمی‌تواند یک crate با آن نام منتشر کند. قبل از تلاش برای انتشار یک crate، جستجو کنید که نامی که می‌خواهید استفاده کنید در دسترس است یا خیر. اگر نام استفاده شده باشد، باید یک نام دیگر پیدا کنید و فیلد `name` را در فایل _Cargo.toml_ در زیر بخش `[package]` ویرایش کنید تا از نام جدید برای انتشار استفاده کنید، مانند زیر:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

<<<<<<< HEAD
حتی اگر یک نام منحصر به فرد انتخاب کرده باشید، زمانی که `cargo publish` را برای انتشار crate در این مرحله اجرا کنید، یک هشدار و سپس یک خطا دریافت خواهید کرد:
=======
Even if you’ve chosen a unique name, when you run `cargo publish` to publish
the crate at this point, you’ll get a warning and then an error:

<!-- manual-regeneration
Create a new package with an unregistered name, making no further modifications
  to the generated package, so it is missing the description and license fields.
cargo publish
copy just the relevant lines below
-->
>>>>>>> upstream/main

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

<<<<<<< HEAD
این خطا به دلیل این است که شما برخی اطلاعات حیاتی را از دست داده‌اید: یک توضیح و یک مجوز مورد نیاز است تا افراد بدانند crate شما چه کاری انجام می‌دهد و تحت چه شرایطی می‌توانند از آن استفاده کنند. در فایل _Cargo.toml_، یک توضیح اضافه کنید که فقط یک یا دو جمله باشد، زیرا این توضیح همراه crate شما در نتایج جستجو ظاهر خواهد شد. برای فیلد `license`، باید یک _مقدار شناسگر مجوز_ ارائه دهید. [پروژه Software Package Data Exchange (SPDX)](http://spdx.org/licenses/) لیستی از شناسگرهایی که می‌توانید برای این مقدار استفاده کنید را ارائه می‌دهد. برای مثال، برای مشخص کردن اینکه crate خود را با استفاده از مجوز MIT منتشر کرده‌اید، شناسگر `MIT` را اضافه کنید:
=======
This results in an error because you’re missing some crucial information: a
description and license are required so people will know what your crate does
and under what terms they can use it. In _Cargo.toml_, add a description that's
just a sentence or two, because it will appear with your crate in search
results. For the `license` field, you need to give a _license identifier value_.
The [Linux Foundation’s Software Package Data Exchange (SPDX)][spdx] lists the
identifiers you can use for this value. For example, to specify that you’ve
licensed your crate using the MIT License, add the `MIT` identifier:
>>>>>>> upstream/main

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
license = "MIT"
```

اگر می‌خواهید از مجوزی استفاده کنید که در لیست SPDX موجود نیست، باید متن آن مجوز را در یک فایل قرار دهید، فایل را در پروژه خود اضافه کنید و سپس از کلید `license-file` برای مشخص کردن نام آن فایل به جای استفاده از کلید `license` استفاده کنید.

راهنمایی درباره اینکه کدام مجوز برای پروژه شما مناسب است، فراتر از محدوده این کتاب است. بسیاری از افراد در جامعه Rust پروژه‌های خود را به همان روشی که Rust مجوز داده است، با استفاده از یک مجوز دوگانه `MIT OR Apache-2.0` مجوز می‌دهند. این روش نشان می‌دهد که شما می‌توانید چندین شناسه مجوز را با جدا کردن آن‌ها با `OR` مشخص کنید تا چندین مجوز برای پروژه خود داشته باشید.

با یک نام منحصر به فرد، نسخه، توضیحات، و یک مجوز اضافه شده، فایل _Cargo.toml_ برای یک پروژه آماده انتشار ممکن است به این صورت باشد:

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

<<<<<<< HEAD
[مستندات Cargo](https://doc.rust-lang.org/cargo/) سایر متادیتاهایی که می‌توانید مشخص کنید تا دیگران بتوانند crate شما را راحت‌تر پیدا کرده و استفاده کنند توضیح می‌دهد.
=======
[Cargo’s documentation](https://doc.rust-lang.org/cargo/) describes other
metadata you can specify to ensure that others can discover and use your crate
more easily.
>>>>>>> upstream/main

### انتشار در Crates.io

اکنون که یک حساب ایجاد کرده‌اید، توکن API خود را ذخیره کرده‌اید، نامی برای crate خود انتخاب کرده‌اید، و متادیتای مورد نیاز را مشخص کرده‌اید، آماده انتشار هستید! انتشار یک crate نسخه‌ای خاص از آن را در [crates.io](https://crates.io/)<!-- ignore --> آپلود می‌کند تا دیگران بتوانند از آن استفاده کنند.

<<<<<<< HEAD
دقت کنید، زیرا انتشار _دائمی_ است. نسخه هرگز نمی‌تواند بازنویسی شود، و کد نمی‌تواند حذف شود. یکی از اهداف اصلی [crates.io](https://crates.io/)<!-- ignore --> این است که به عنوان یک آرشیو دائمی از کد عمل کند، به طوری که ساخت‌های همه پروژه‌هایی که به crates از [crates.io](https://crates.io/)<!-- ignore --> وابسته هستند، همچنان کار کنند. اجازه حذف نسخه‌ها تحقق این هدف را غیرممکن می‌کند. با این حال، هیچ محدودیتی برای تعداد نسخه‌های crate که می‌توانید منتشر کنید وجود ندارد.
=======
Be careful, because a publish is _permanent_. The version can never be
overwritten, and the code cannot be deleted except in certain circumstances.
One major goal of Crates.io is to act as a permanent archive of code so that
builds of all projects that depend on crates from
[crates.io](https://crates.io/)<!-- ignore --> will continue to work. Allowing
version deletions would make fulfilling that goal impossible. However, there is
no limit to the number of crate versions you can publish.
>>>>>>> upstream/main

دستور `cargo publish` را دوباره اجرا کنید. اکنون باید موفق شود:

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
    Packaged 6 files, 1.2KiB (895.0B compressed)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
    Uploaded guessing_game v0.1.0 to registry `crates-io`
note: waiting for `guessing_game v0.1.0` to be available at registry
`crates-io`.
You may press ctrl-c to skip waiting; the crate should be available shortly.
   Published guessing_game v0.1.0 at registry `crates-io`
```

تبریک می‌گویم! شما اکنون کد خود را با جامعه Rust به اشتراک گذاشته‌اید و هر کسی می‌تواند به راحتی crate شما را به عنوان یک وابستگی به پروژه خود اضافه کند.

### انتشار نسخه جدیدی از یک Crate موجود

<<<<<<< HEAD
وقتی تغییراتی در crate خود ایجاد کرده‌اید و آماده انتشار یک نسخه جدید هستید، مقدار `version` مشخص‌شده در فایل _Cargo.toml_ خود را تغییر داده و دوباره منتشر کنید. از [قوانین نسخه‌بندی معنایی (Semantic Versioning)][semver] استفاده کنید تا تصمیم بگیرید که بر اساس نوع تغییراتی که ایجاد کرده‌اید، چه شماره نسخه‌ای مناسب است. سپس دستور `cargo publish` را اجرا کنید تا نسخه جدید آپلود شود.
=======
When you’ve made changes to your crate and are ready to release a new version,
you change the `version` value specified in your _Cargo.toml_ file and
republish. Use the [Semantic Versioning rules][semver] to decide what an
appropriate next version number is, based on the kinds of changes you’ve made.
Then run `cargo publish` to upload the new version.
>>>>>>> upstream/main

<!-- لینک قدیمی، حذف نکنید -->

<a id="removing-versions-from-cratesio-with-cargo-yank"></a>

### از رده خارج کردن نسخه‌ها از Crates.io با استفاده از `cargo yank`

<<<<<<< HEAD
اگرچه نمی‌توانید نسخه‌های قبلی یک crate را حذف کنید، می‌توانید از اضافه شدن آن‌ها به عنوان وابستگی جدید در پروژه‌های آینده جلوگیری کنید. این ویژگی زمانی مفید است که یک نسخه از crate به هر دلیلی خراب باشد. در چنین مواردی، Cargo از _یَنک کردن_ (yanking) یک نسخه از crate پشتیبانی می‌کند.

یَنک کردن یک نسخه باعث می‌شود که پروژه‌های جدید نتوانند به آن نسخه وابسته شوند، در حالی که تمام پروژه‌های موجود که به آن نسخه وابسته هستند به کار خود ادامه می‌دهند. به طور خلاصه، یَنک به این معناست که تمام پروژه‌هایی که دارای فایل _Cargo.lock_ هستند شکسته نخواهند شد و هر فایل _Cargo.lock_ جدیدی که تولید شود از نسخه یَنک‌شده استفاده نخواهد کرد.
=======
Although you can’t remove previous versions of a crate, you can prevent any
future projects from adding them as a new dependency. This is useful when a
crate version is broken for one reason or another. In such situations, Cargo
supports yanking a crate version.

_Yanking_ a version prevents new projects from depending on that version while
allowing all existing projects that depend on it to continue. Essentially, a
yank means that all projects with a _Cargo.lock_ will not break, and any future
_Cargo.lock_ files generated will not use the yanked version.
>>>>>>> upstream/main

برای یَنک کردن یک نسخه از یک crate، در دایرکتوری crate‌ای که قبلاً منتشر کرده‌اید، دستور `cargo yank` را اجرا کرده و نسخه‌ای که می‌خواهید یَنک کنید را مشخص کنید. به عنوان مثال، اگر ما یک crate به نام `guessing_game` نسخه 1.0.1 منتشر کرده باشیم و بخواهیم آن را یَنک کنیم، در دایرکتوری پروژه `guessing_game` این دستور را اجرا می‌کنیم:

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```

با افزودن گزینه `--undo` به دستور، می‌توانید یَنک را لغو کرده و به پروژه‌ها اجازه دهید دوباره به آن نسخه وابسته شوند:

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

یَنک _هیچ کدی را حذف نمی‌کند_. به عنوان مثال، نمی‌تواند اطلاعات حساسی که به طور تصادفی آپلود شده‌اند را حذف کند. اگر چنین اتفاقی افتاد، باید فوراً آن اطلاعات حساس را بازنشانی کنید.

[spdx]: https://spdx.org/licenses/
[semver]: https://semver.org/
