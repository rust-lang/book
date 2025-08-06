## کار با تعداد دلخواهی از Futures


وقتی در بخش قبلی از استفاده از دو future به سه future تغییر دادیم، مجبور شدیم به جای استفاده از `join` از `join3` استفاده کنیم. این مسئله آزاردهنده خواهد بود اگر هر بار که تعداد futuresی که می‌خواهیم join کنیم تغییر می‌کند، مجبور به فراخوانی یک تابع متفاوت باشیم. خوشبختانه، یک فرم ماکروی `join` داریم که می‌توانیم به آن تعداد دلخواهی از آرگومان‌ها را ارسال کنیم. این ماکرو همچنین خودش مدیریت انتظار برای futures را انجام می‌دهد. بنابراین، می‌توانیم کد لیست ۱۷-۱۳ را بازنویسی کنیم تا به جای `join3` از `join!` استفاده کنیم، همان‌طور که در لیست ۱۷-۱۴ نشان داده شده است.

<Listing number="17-14" caption="استفاده از `join!` برای منتظر ماندن چندین آینده" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:here}}
```

</Listing>

این قطعاً نسبت به جابجایی بین `join`، `join3`، `join4` و موارد دیگر بهبود یافته است! با این حال، حتی این فرم ماکرو نیز فقط زمانی کار می‌کند که تعداد futures را از قبل بدانیم. اما در دنیای واقعی Rust، اضافه کردن futures به یک مجموعه و سپس انتظار برای کامل شدن برخی یا تمام آن‌ها یک الگوی رایج است.

برای بررسی همه‌ی futureها در یک مجموعه، باید روی همه‌ی آن‌ها پیمایش کنیم و روی همه join کنیم.
تابع `trpl::join_all` هر نوعی را می‌پذیرد که trait `Iterator` را پیاده‌سازی کرده باشد،
که در فصل ۱۳ در بخش [trait پیمایشگر و متد `next`][iterator-trait] درباره‌ی آن آموختید،
پس به نظر می‌رسد که دقیقاً مناسب باشد.
بیایید futureهایمان را در یک بردار قرار دهیم و `join!` را با `join_all` جایگزین کنیم،
همان‌طور که در لیستینگ 17-15 نشان داده شده است.

<Listing number="17-15" caption="ذخیره آینده‌های ناشناس در یک بردار و فراخوانی `join_all`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:here}}
```

</Listing>

متأسفانه، این کد کامپایل نمی‌شود. در عوض، با این خطا مواجه می‌شویم:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-15/
cargo build
copy just the compiler error
-->

```text
error[E0308]: mismatched types
  --> src/main.rs:45:37
   |
10 |         let tx1_fut = async move {
   |                       ---------- the expected `async` block
...
24 |         let rx_fut = async {
   |                      ----- the found `async` block
...
45 |         let futures = vec![tx1_fut, rx_fut, tx_fut];
   |                                     ^^^^^^ expected `async` block, found a different `async` block
   |
   = note: expected `async` block `{async block@src/main.rs:10:23: 10:33}`
              found `async` block `{async block@src/main.rs:24:22: 24:27}`
   = note: no two async blocks, even if identical, have the same type
   = help: consider pinning your async block and casting it to a trait object
```

این ممکن است شگفت‌آور باشد. بالاخره، هیچ‌یک از بلوک‌های async چیزی بازنمی‌گردانند، بنابراین هر کدام یک `Future<Output = ()>` تولید می‌کنند. اما به یاد داشته باشید که `Future` یک ویژگی (_trait_) است و کامپایلر برای هر بلوک async یک enum منحصربه‌فرد ایجاد می‌کند. نمی‌توانید دو struct مختلف را که دستی نوشته شده‌اند در یک `Vec` قرار دهید، و همین قانون برای enumهای مختلفی که توسط کامپایلر تولید می‌شوند اعمال می‌شود.


> توجه: در بخش [استفاده از یک enum برای نگهداری چند مقدار][enum-alt]<!-- ignore --> در فصل ۸،
> روش دیگری برای گنجاندن چند نوع مختلف در یک `Vec` را بررسی کردیم:
> استفاده از یک `enum` برای نمایش هر نوعی که ممکن است در بردار وجود داشته باشد.
> اما در اینجا نمی‌توانیم این کار را انجام دهیم.
> اولاً، هیچ راهی برای نام‌گذاری نوع‌های مختلف نداریم چون آن‌ها ناشناس (anonymous) هستند.
> ثانیاً، دلیل اصلی استفاده‌ی ما از بردار و `join_all` این بود که بتوانیم با مجموعه‌ای پویا از futureها کار کنیم،
> جایی که تنها مهم است همه خروجی‌های آن‌ها یکسان باشند.

> نکته: در بخش فصل ۸ [استفاده از یک Enum برای ذخیره مقادیر متعدد][enum-alt]<!-- ignore -->، درباره یک روش دیگر برای شامل کردن چندین نوع در یک `Vec` صحبت کردیم: استفاده از یک enum برای نمایش هر نوعی که می‌تواند در وکتور ظاهر شود. اما نمی‌توانیم اینجا از آن استفاده کنیم. از یک طرف، هیچ راهی برای نام‌گذاری انواع مختلف نداریم، زیرا آن‌ها ناشناس هستند. از طرف دیگر، دلیلی که ما در وهله اول به دنبال یک وکتور و `join_all` رفتیم، این بود که بتوانیم با یک مجموعه پویا از futures کار کنیم، جایی که فقط به این اهمیت می‌دهیم که همه آن‌ها خروجی یکسانی دارند.

ابتدا هر future درون `vec!` را در یک `Box::new` بسته‌بندی می‌کنیم، همان‌طور که در لیست ۱۷-۱۶ نشان داده شده است.

<Listing number="17-16" caption="استفاده از `Box::new` برای تطبیق انواع futures در یک `Vec`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

متأسفانه، این کد هنوز هم کامپایل نمی‌شود. در واقع، همان خطای پایه‌ای که قبلاً دریافت کردیم، برای فراخوانی‌های دوم و سوم `Box::new` نیز رخ می‌دهد، به همراه خطاهای جدیدی که به ویژگی `Unpin` اشاره دارند. به زودی به خطاهای مرتبط با `Unpin` بازمی‌گردیم. ابتدا، بیایید خطاهای نوع در فراخوانی‌های `Box::new` را با مشخص کردن صریح نوع متغیر `futures` رفع کنیم (نگاه کنید به لیست ۱۷-۱۷).

<Listing number="17-17" caption="برطرف کردن بقیه خطاهای ناسازگاری نوع با استفاده از اعلان صریح نوع" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:here}}
```

</Listing>

این type declaration کمی پیچیده است، بنابراین بیایید آن را مرحله به مرحله بررسی کنیم:

1. نوع داخلی‌ترین، خود future است. به‌طور صریح اعلام می‌کنیم که خروجی future نوع واحد `()` است، با نوشتن `Future<Output = ()>`.
2. سپس ویژگی را با `dyn` علامت‌گذاری می‌کنیم تا به‌صورت دینامیک باشد.
3. کل مرجع ویژگی در یک `Box` بسته‌بندی می‌شود.
4. در نهایت، به‌طور صریح بیان می‌کنیم که `futures` یک `Vec` است که شامل این آیتم‌ها است.

این تغییر تأثیر قابل‌توجهی داشت. اکنون وقتی کامپایلر را اجرا می‌کنیم، فقط خطاهایی که به `Unpin` اشاره دارند باقی می‌مانند. اگرچه سه خطا وجود دارد، اما محتوای آن‌ها بسیار مشابه است.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-17
cargo build
# copy *only* the errors
# fix the paths
-->

```text
error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
   --> src/main.rs:49:24
    |
49  |         trpl::join_all(futures).await;
    |         -------------- ^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
    |         |
    |         required by a bound introduced by this call
    |
    = note: consider using the `pin!` macro
            consider using `Box::pin` if you need to access the pinned value outside of the current scope
    = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `join_all`
   --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:105:14
    |
102 | pub fn join_all<I>(iter: I) -> JoinAll<I::Item>
    |        -------- required by a bound in this function
...
105 |     I::Item: Future,
    |              ^^^^^^ required by this bound in `join_all`

error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
  --> src/main.rs:49:9
   |
49 |         trpl::join_all(futures).await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
  --> src/main.rs:49:33
   |
49 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `async_await` (bin "async_await") due to 3 previous errors
```

این پیام حجم زیادی از اطلاعات را دارد، پس بیایید آن را بخش‌بندی کنیم.
بخش اول پیام می‌گوید که اولین بلاک async (`src/main.rs:8:23: 20:10`) trait `Unpin` را پیاده‌سازی نکرده است
و پیشنهاد می‌کند برای رفع این مشکل از `pin!` یا `Box::pin` استفاده کنیم.
در ادامه‌ی فصل، به جزئیات بیشتری درباره‌ی `Pin` و `Unpin` خواهیم پرداخت.
فعلاً می‌توانیم فقط از توصیه‌ی کامپایلر پیروی کنیم تا مشکل برطرف شود.
در لیستینگ 17-18، ابتدا `Pin` را از `std::pin` وارد می‌کنیم.
سپس نوع `futures` را به‌روزرسانی می‌کنیم، به‌طوری که هر `Box` داخل یک `Pin` قرار گیرد.
در نهایت، از `Box::pin` برای pin کردن خود futureها استفاده می‌کنیم.


<Listing number="17-18" caption="استفاده از `Pin` و `Box::pin` برای برطرف کردن نوع `Vec`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

اگر این کد را کامپایل و اجرا کنیم، در نهایت خروجی موردنظر خود را دریافت می‌کنیم:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
received 'hi'
received 'more'
received 'from'
received 'messages'
received 'the'
received 'for'
received 'future'
received 'you'
```

آه!

اینجا چیزهای بیشتری برای بررسی وجود دارد. برای یک مورد، استفاده از `Pin<Box<T>>` یک مقدار کمی سربار اضافه می‌کند، زیرا این futures را با `Box` روی heap قرار می‌دهیم—و ما فقط این کار را برای هم‌تراز کردن انواع انجام می‌دهیم. بعد از همه این‌ها، ما _واقعاً_ نیازی به تخصیص heap نداریم: این futures به این تابع خاص محدود هستند. همان‌طور که قبلاً ذکر شد، `Pin` خودش یک نوع wrapper است، بنابراین می‌توانیم از مزیت داشتن یک نوع واحد در `Vec` بهره‌مند شویم—دلیل اصلی که به دنبال `Box` رفتیم—بدون انجام تخصیص heap. می‌توانیم مستقیماً از `Pin` با هر future استفاده کنیم، با استفاده از ماکروی `std::pin::pin`.

با این حال، همچنان باید به‌صورت صریح نوع رفرنس پین‌شده را مشخص کنیم؛  
در غیر این صورت، Rust نمی‌داند که این‌ها باید به عنوان trait objectهای داینامیک تفسیر شوند،  
که این همان چیزی است که در `Vec` به آن نیاز داریم.  
بنابراین، `pin` را به لیست واردات‌مان از `std::pin` اضافه می‌کنیم.  
سپس می‌توانیم هر future را هنگام تعریف آن با `pin!` پین کنیم  
و `futures` را به‌صورت یک `Vec` شامل رفرنس‌های mutable پین‌شده به نوع dynamic future تعریف کنیم،  
همان‌طور که در لیستینگ 17-19 نشان داده شده است.

با این حال، باید به‌صراحت نوع مرجع pinned را مشخص کنیم؛ در غیر این صورت، راست همچنان نمی‌داند که این‌ها را به‌عنوان شیءهای ویژگی دینامیک تفسیر کند، که همان چیزی است که برای قرار گرفتن در `Vec` نیاز داریم. بنابراین، هر آینده را وقتی تعریف می‌کنیم `pin!` می‌کنیم و `futures` را به‌عنوان یک `Vec` که شامل مراجع متغیر pinned به نوع ویژگی دینامیک `Future` است تعریف می‌کنیم، همانطور که در فهرست 17-19 نشان داده شده است.

<Listing number="17-19" caption="استفاده مستقیم از `Pin` با ماکروی `pin!` برای اجتناب از تخصیص‌های غیرضروری heap" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:here}}
```

</Listing>

تا اینجا با نادیده گرفتن این واقعیت که ممکن است نوع‌های `Output` مختلفی داشته باشیم، پیش رفتیم. برای مثال، در فهرست 17-20، آینده ناشناس برای `a` ویژگی `Future<Output = u32>` را پیاده‌سازی می‌کند، آینده ناشناس برای `b` ویژگی `Future<Output = &str>` را پیاده‌سازی می‌کند، و آینده ناشناس برای `c` ویژگی `Future<Output = bool>` را پیاده‌سازی می‌کند.

<Listing number="17-20" caption="سه آینده با نوع‌های متفاوت" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:here}}
```

</Listing>

می‌توانیم از `trpl::join!` برای منتظر ماندن استفاده کنیم، زیرا به ما اجازه می‌دهد چندین نوع future را ارسال کنیم و یک tuple از آن انواع تولید می‌کند. اما _نمی‌توانیم_ از `trpl::join_all` استفاده کنیم، زیرا این تابع نیاز دارد که همه futures ارسال‌شده نوع یکسانی داشته باشند. به یاد داشته باشید، همین خطا بود که ما را به این ماجراجویی با `Pin` کشاند!

این یک معاوضه بنیادی است: می‌توانیم با تعداد پویایی از futures با استفاده از `join_all` کار کنیم، به شرطی که همه آن‌ها نوع یکسانی داشته باشند، یا می‌توانیم با تعداد مشخصی از futures با توابع `join` یا ماکروی `join!` کار کنیم، حتی اگر آن‌ها انواع مختلفی داشته باشند. این همان شرایطی است که هنگام کار با هر نوع دیگری در Rust با آن مواجه می‌شویم. Futures خاص نیستند، حتی اگر سینتکس مناسبی برای کار با آن‌ها داشته باشیم، و این یک نکته مثبت است.

### Racing Futures

وقتی آینده‌ها را با خانواده توابع و ماکروهای `join` "منتظر می‌مانیم"، نیاز داریم _همه_ آن‌ها تمام شوند قبل از اینکه به مرحله بعدی برویم. گاهی اوقات، اما، فقط نیاز داریم _یکی_ از آینده‌ها از مجموعه‌ای تمام شود قبل از اینکه به مرحله بعدی برویم—کمی شبیه به مسابقه دادن یک آینده در برابر دیگری.

در لیست ۱۷-۲۱، ما دوباره از `trpl::race` استفاده می‌کنیم تا دو future، یعنی `slow` و `fast`، را در برابر یکدیگر اجرا کنیم.

<Listing number="17-21" caption="استفاده از `race` برای دریافت نتیجه اولین آینده‌ای که تمام می‌شود" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:here}}
```

</Listing>

هر future یک پیام هنگام شروع اجرا چاپ می‌کند، با فراخوانی و انتظار برای `sleep` به مدت مشخصی مکث می‌کند، و سپس یک پیام دیگر هنگام اتمام چاپ می‌کند. سپس، هر دو future یعنی `slow` و `fast` را به `trpl::race` ارسال می‌کنیم و منتظر می‌مانیم تا یکی از آن‌ها به پایان برسد. (نتیجه اینجا چندان شگفت‌آور نیست: `fast` برنده می‌شود.) برخلاف زمانی که در [“اولین برنامه Async ما”][async-program]<!-- ignore --> از `race` استفاده کردیم، اینجا به نمونه `Either` که بازمی‌گرداند توجه نمی‌کنیم، زیرا تمام رفتار جالب در بدنه بلوک‌های async رخ می‌دهد.

توجه کنید که اگر ترتیب آرگومان‌ها به `race` را جابه‌جا کنید، ترتیب پیام‌های "started" تغییر می‌کند، حتی اگر future `fast` همیشه زودتر به پایان برسد. دلیل این است که پیاده‌سازی این تابع خاص `race` منصفانه نیست. این تابع همیشه futures ارسال‌شده را به ترتیب آرگومان‌ها اجرا می‌کند. سایر پیاده‌سازی‌ها _منصفانه_ هستند و به صورت تصادفی انتخاب می‌کنند که کدام future را ابتدا poll کنند. با این حال، صرف‌نظر از اینکه پیاده‌سازی `race` ما منصفانه باشد یا نه، _یکی_ از futures تا اولین `await` در بدنه‌اش اجرا می‌شود قبل از اینکه task دیگری بتواند شروع شود.

به یاد بیاورید از [اولین برنامه Async ما][async-program]<!-- ignore --> که در هر نقطه `await`، Rust به runtime اجازه می‌دهد تا task را متوقف کند و به task دیگری سوئیچ کند اگر future در حال انتظار آماده نباشد. عکس این موضوع هم صادق است: Rust فقط بلوک‌های async را متوقف می‌کند و کنترل را به runtime بازمی‌گرداند در یک نقطه `await`. 

این بدان معناست که اگر در یک بلوک async بدون نقطه `await` مقدار زیادی کار انجام دهید، آن future دیگر futures را از پیشرفت باز می‌دارد. گاهی اوقات ممکن است به این موضوع اشاره شود که یک future _دیگر futures را گرسنه می‌کند_. در برخی موارد، این ممکن است مشکل بزرگی نباشد. با این حال، اگر در حال انجام برخی تنظیمات پرهزینه یا کار طولانی‌مدت هستید، یا اگر futureای دارید که به طور نامحدود یک کار خاص را انجام می‌دهد، باید به این فکر کنید که چه زمانی و کجا کنترل را به runtime بازگردانید.

به همان اندازه، اگر عملیات‌های مسدودکننده طولانی‌مدت دارید، async می‌تواند ابزاری مفید برای ارائه راه‌هایی باشد که بخش‌های مختلف برنامه بتوانند با یکدیگر تعامل داشته باشند.

اما در این موارد _چگونه_ کنترل را به runtime بازمی‌گردانید؟

<!-- Old headings. Do not remove or links may break. -->

<a id="yielding"></a>

### Yielding Control to the Runtime

بیایید یک عملیات طولانی‌مدت را شبیه‌سازی کنیم. لیست ۱۷-۲۲ یک تابع به نام `slow` معرفی می‌کند.

<Listing number="17-22" caption="استفاده از `thread::sleep` برای شبیه‌سازی عملیات کند" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:slow}}
```

</Listing>

این کد از `std::thread::sleep` به جای `trpl::sleep` استفاده می‌کند، به طوری که فراخوانی `slow`، Thread فعلی را برای مدت مشخصی از میلی‌ثانیه‌ها مسدود می‌کند. می‌توانیم از `slow` به عنوان جایگزینی برای عملیات‌های واقعی که هم طولانی‌مدت هستند و هم مسدودکننده، استفاده کنیم.

در لیست ۱۷-۲۳، از `slow` برای شبیه‌سازی انجام این نوع کارهای CPU-bound در یک جفت future استفاده می‌کنیم.

<Listing number="17-23" caption="استفاده از `thread::sleep` برای شبیه‌سازی عملیات کند" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:slow-futures}}
```

</Listing>

برای شروع، هر future فقط _پس از_ انجام یک سری عملیات کند، کنترل را به runtime بازمی‌گرداند. اگر این کد را اجرا کنید، این خروجی را مشاهده خواهید کرد:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-23/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

همان‌طور که در مثال قبلی دیدیم، `race` همچنان به محض اینکه `a` تمام شود، کار را تمام می‌کند. اما بین دو future هیچ تداخل یا جابه‌جایی وجود ندارد. future `a` تمام کار خود را انجام می‌دهد تا زمانی که فراخوانی `trpl::sleep` منتظر بماند، سپس future `b` تمام کار خود را انجام می‌دهد تا زمانی که فراخوانی `trpl::sleep` خودش منتظر بماند، و در نهایت future `a` کامل می‌شود. برای اینکه هر دو future بتوانند بین taskهای کند خود پیشرفت کنند، به نقاط `await` نیاز داریم تا بتوانیم کنترل را به runtime بازگردانیم. این به این معناست که به چیزی نیاز داریم که بتوانیم برای آن منتظر بمانیم!

هم‌اکنون می‌توانیم این نوع انتقال کنترل را در لیست ۱۷-۲۳ مشاهده کنیم: اگر `trpl::sleep` در انتهای future `a` را حذف کنیم، این future بدون اجرای future `b` به طور کامل به پایان می‌رسد. بیایید از تابع `sleep` به‌عنوان نقطه شروعی برای اجازه دادن به عملیات‌ها برای جابه‌جا شدن و پیشرفت استفاده کنیم، همان‌طور که در لیست ۱۷-۲۴ نشان داده شده است.

<Listing number="17-24" caption="استفاده از `sleep` برای اجازه دادن به عملیات‌ها برای پیشرفت متناوب" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

در فهرست 17-24، فراخوانی‌های `trpl::sleep` با نقاط انتظار بین هر فراخوانی به `slow` اضافه می‌کنیم. اکنون کار دو آینده درهم‌تنیده شده است:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-24
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

future `a` هنوز برای مدتی اجرا می‌شود قبل از اینکه کنترل را به `b` منتقل کند، زیرا ابتدا `slow` را فراخوانی می‌کند قبل از اینکه `trpl::sleep` را فراخوانی کند. اما پس از آن، futures هر بار که یکی از آن‌ها به یک نقطه `await` می‌رسد، به صورت متناوب جابه‌جا می‌شوند. در این مورد، ما این کار را پس از هر فراخوانی به `slow` انجام داده‌ایم، اما می‌توانستیم کار را به هر شکلی که برای ما منطقی‌تر است تقسیم کنیم.

با این حال، واقعاً نمی‌خواهیم اینجا _sleep_ کنیم؛ می‌خواهیم به سریع‌ترین شکلی که می‌توانیم پیشرفت کنیم. فقط نیاز داریم کنترل را به runtime بازگردانیم. می‌توانیم این کار را به‌طور مستقیم با استفاده از تابع `yield_now` انجام دهیم. در فهرست 17-25، تمام این فراخوانی‌های `sleep` را با `yield_now` جایگزین می‌کنیم.

<Listing number="17-25" caption="استفاده از `yield_now` برای اجازه دادن به عملیات‌ها برای پیشرفت متناوب" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:yields}}
```

</Listing>

این کد هم از نظر بیان هدف واقعی واضح‌تر است و هم می‌تواند به طور قابل‌توجهی سریع‌تر از استفاده از `sleep` باشد، زیرا تایمرهایی مانند آنچه که توسط `sleep` استفاده می‌شود اغلب محدودیت‌هایی در دقت خود دارند. نسخه‌ای از `sleep` که ما استفاده می‌کنیم، برای مثال، همیشه حداقل به مدت یک میلی‌ثانیه می‌خوابد، حتی اگر یک `Duration` یک نانوثانیه‌ای به آن بدهیم. دوباره، کامپیوترهای مدرن _سریع_ هستند: آن‌ها می‌توانند در یک میلی‌ثانیه کارهای زیادی انجام دهند!

می‌توانید خودتان این را ببینید با راه‌اندازی یک بنچمارک کوچک، مانند آنچه در لیست ۱۷-۲۶ نشان داده شده است. (این روش به‌ویژه دقیقی برای انجام تست عملکرد نیست، اما برای نشان دادن تفاوت در اینجا کافی است.)

<Listing number="17-26" caption="مقایسه عملکرد `sleep` و `yield_now`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-26/src/main.rs:here}}
```

</Listing>

در اینجا، تمام چاپ وضعیت را کنار می‌گذاریم، یک `Duration` یک نانوثانیه‌ای به `trpl::sleep` می‌دهیم و اجازه می‌دهیم هر future به‌صورت مستقل اجرا شود، بدون هیچ جابه‌جایی بین futures. سپس ۱,۰۰۰ بار این عملیات را تکرار می‌کنیم و می‌بینیم که futureی که از `trpl::sleep` استفاده می‌کند در مقایسه با futureی که از `trpl::yield_now` استفاده می‌کند چقدر زمان می‌برد.

نسخه‌ای که از `yield_now` استفاده می‌کند، _بسیار_ سریع‌تر است!

این بدان معناست که async حتی برای وظایف وابسته به CPU می‌تواند مفید باشد، بسته به اینکه برنامه شما چه کار دیگری انجام می‌دهد، زیرا ابزاری مفید برای ساختاردهی روابط بین بخش‌های مختلف برنامه فراهم می‌کند. این نوعی از _چندوظیفه‌گی مشارکتی_ است، جایی که هر آینده قدرت تصمیم‌گیری درباره زمان واگذاری کنترل از طریق نقاط انتظار را دارد. بنابراین، هر آینده نیز مسئولیت دارد که از مسدود کردن بیش از حد طولانی اجتناب کند. در برخی سیستم‌عامل‌های مبتنی بر راست برای سیستم‌های تعبیه‌شده، این _تنها_ نوع چندوظیفه‌گی است!

در کد واقعی، معمولاً فراخوانی توابع را با نقاط `await` در هر خط متناوب نمی‌کنید، البته. در حالی که واگذاری کنترل به این روش نسبتاً کم‌هزینه است، اما رایگان نیست. در بسیاری از موارد، تلاش برای تقسیم یک task که CPU-bound است ممکن است آن را به‌طور قابل توجهی کندتر کند، بنابراین گاهی اوقات برای _عملکرد کلی_ بهتر است که اجازه دهید یک عملیات به‌طور مختصر مسدود شود. همیشه اندازه‌گیری کنید تا ببینید تنگناهای عملکرد واقعی کد شما کجا هستند. اما، این دینامیک اساسی را باید در ذهن داشته باشید، به‌ویژه اگر _واقعاً_ شاهد انجام مقدار زیادی کار به‌صورت ترتیبی باشید، در حالی که انتظار داشتید به‌طور همزمان انجام شود!

### ساخت انتزاعات Async خودمان

ما همچنین می‌توانیم futures را با هم ترکیب کنیم تا الگوهای جدیدی ایجاد کنیم. برای مثال، می‌توانیم یک تابع `timeout` با استفاده از بلوک‌های سازنده async که از قبل داریم، بسازیم. هنگامی که کارمان تمام شد، نتیجه یک بلوک سازنده دیگر خواهد بود که می‌توانیم برای ایجاد انتزاعات async بیشتری از آن استفاده کنیم.

فهرست 17-27 نشان می‌دهد که چگونه انتظار داریم این `timeout` با یک آینده کند کار کند.

<Listing number="17-27" caption="تعریف نحوه کار `timeout` با یک آینده کند" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-27/src/main.rs:here}}
```

</Listing>

بیایید این را پیاده‌سازی کنیم! برای شروع، بیایید به API مورد نیاز برای `timeout` فکر کنیم:

- باید خودش یک تابع async باشد تا بتوانیم منتظر آن بمانیم.
- پارامتر اول آن باید یک آینده برای اجرا باشد. می‌توانیم آن را عمومی کنیم تا بتواند با هر آینده‌ای کار کند.
- پارامتر دوم آن مدت‌زمان حداکثری برای انتظار خواهد بود. اگر از یک `Duration` استفاده کنیم، این کار ارسال آن به `trpl::sleep` را آسان می‌کند.
- باید یک `Result` بازگرداند. اگر آینده با موفقیت کامل شود، `Result` شامل `Ok` با مقدار تولیدشده توسط آینده خواهد بود. اگر زمان محدودیت زودتر سپری شود، `Result` شامل `Err` با مدت‌زمانی که زمان محدودیت برای آن منتظر ماند خواهد بود.

فهرست 17-28 این اعلان را نشان می‌دهد.

<!-- This is not tested because it intentionally does not compile. -->

<Listing number="17-28" caption="تعریف امضای `timeout`" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-28/src/main.rs:declaration}}
```

</Listing>

این اهداف ما برای نوع‌ها را برآورده می‌کند. حالا بیایید به _رفتاری_ که نیاز داریم فکر کنیم: می‌خواهیم آینده ارسال‌شده به آن را در برابر مدت‌زمان محدودیت مسابقه دهیم. می‌توانیم از `trpl::sleep` برای ساختن یک آینده تایمر از مدت‌زمان استفاده کنیم و از `trpl::race` برای اجرای آن تایمر با آینده‌ای که کاربر ارسال می‌کند استفاده کنیم.

ما همچنین می‌دانیم که `race` منصفانه نیست و آرگومان‌ها را به ترتیب ارسال‌شده poll می‌کند. بنابراین، ابتدا `future_to_try` را به `race` ارسال می‌کنیم تا حتی اگر `max_time` مدت زمان بسیار کوتاهی باشد، فرصتی برای تکمیل شدن داشته باشد. اگر `future_to_try` زودتر تمام شود، `race` مقدار `Left` را با خروجی `future_to_try` بازمی‌گرداند. اگر `timer` زودتر تمام شود، `race` مقدار `Right` را با خروجی `()` تایمر بازمی‌گرداند.

در لیست ۱۷-۲۹، نتیجه انتظار برای `trpl::race` را match می‌کنیم.

<Listing number="17-29" caption="تعریف `timeout` با استفاده از `race` و `sleep`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-29/src/main.rs:implementation}}
```

</Listing>

اگر `future_to_try` موفق شود و مقدار `Left(output)` دریافت کنیم، مقدار `Ok(output)` را بازمی‌گردانیم. اگر به جای آن تایمر خواب منقضی شود و مقدار `Right(())` دریافت کنیم، `()` را با `_` نادیده گرفته و به جای آن مقدار `Err(max_time)` را بازمی‌گردانیم.

با این کار، یک `timeout` عملیاتی داریم که از دو ابزار کمکی async دیگر ساخته شده است. اگر کد خود را اجرا کنیم، پس از انقضای timeout، حالت شکست را چاپ خواهد کرد:

```text
Failed after 2 seconds
```

از آنجا که futures می‌توانند با دیگر futures ترکیب شوند، می‌توانید ابزارهای بسیار قدرتمندی با استفاده از بلوک‌های سازنده کوچک‌تر async بسازید. برای مثال، می‌توانید از همین رویکرد برای ترکیب timeoutها با retries استفاده کنید و به نوبه خود از آن‌ها با عملیاتی مانند تماس‌های شبکه (یکی از مثال‌های ابتدای فصل) استفاده کنید.

در عمل، معمولاً مستقیماً با `async` و `await` کار می‌کنید و به طور ثانویه از توابع و ماکروهایی مانند `join`، `join_all`، `race` و غیره استفاده می‌کنید. فقط گاهی نیاز خواهید داشت از `pin` برای استفاده از futures با آن APIها استفاده کنید.

اکنون روش‌های متعددی برای کار با چندین future به طور همزمان دیده‌ایم. در ادامه، بررسی خواهیم کرد که چگونه می‌توانیم با چندین future به صورت متوالی در طول زمان با _streams_ کار کنیم. با این حال، در ابتدا ممکن است بخواهید به چند نکته دیگر توجه کنید:

- ما از یک `Vec` همراه با `join_all` استفاده کردیم تا منتظر بمانیم تمام futures در یک گروه به پایان برسند. چگونه می‌توانید از یک `Vec` برای پردازش یک گروه از futures به صورت متوالی استفاده کنید؟ معاوضه‌های انجام این کار چیست؟

- به نوع `futures::stream::FuturesUnordered` از crate `futures` نگاهی بیندازید. استفاده از آن چگونه می‌تواند با استفاده از یک `Vec` متفاوت باشد؟ (نگران این نباشید که این نوع از بخش `stream` crate آمده است؛ با هر مجموعه‌ای از futures به خوبی کار می‌کند.)

[dyn]: ch12-03-improving-error-handling-and-modularity.html
[enum-alt]: ch08-01-vectors.html#using-an-enum-to-store-multiple-types
[async-program]: ch17-01-futures-and-syntax.html#our-first-async-program
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
