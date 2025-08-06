## Refactoring to Improve Modularity and Error Handling

To improve our program, we’ll fix four problems that have to do with the
program’s structure and how it’s handling potential errors. First, our `main`
function now performs two tasks: it parses arguments and reads files. As our
program grows, the number of separate tasks the `main` function handles will
increase. As a function gains responsibilities, it becomes more difficult to
reason about, harder to test, and harder to change without breaking one of its
parts. It’s best to separate functionality so each function is responsible for
one task.

This issue also ties into the second problem: although `query` and `file_path`
are configuration variables to our program, variables like `contents` are used
to perform the program’s logic. The longer `main` becomes, the more variables
we’ll need to bring into scope; the more variables we have in scope, the harder
it will be to keep track of the purpose of each. It’s best to group the
configuration variables into one structure to make their purpose clear.

The third problem is that we’ve used `expect` to print an error message when
reading the file fails, but the error message just prints `Should have been
able to read the file`. Reading a file can fail in a number of ways: for
example, the file could be missing, or we might not have permission to open it.
Right now, regardless of the situation, we’d print the same error message for
everything, which wouldn’t give the user any information!

Fourth, we use `expect` to handle an error, and if the user runs our program
without specifying enough arguments, they’ll get an `index out of bounds` error
from Rust that doesn’t clearly explain the problem. It would be best if all the
error-handling code were in one place so future maintainers had only one place
to consult the code if the error-handling logic needed to change. Having all the
error-handling code in one place will also ensure that we’re printing messages
that will be meaningful to our end users.

Let’s address these four problems by refactoring our project.

### Separation of Concerns for Binary Projects

The organizational problem of allocating responsibility for multiple tasks to
the `main` function is common to many binary projects. As a result, the Rust
community has developed guidelines for splitting the separate concerns of a
binary program when `main` starts getting large. This process has the following
steps:

- Split your program into a _main.rs_ file and a _lib.rs_ file and move your
  program’s logic to _lib.rs_.
- As long as your command line parsing logic is small, it can remain in
  _main.rs_.
- When the command line parsing logic starts getting complicated, extract it
  from _main.rs_ and move it to _lib.rs_.

The responsibilities that remain in the `main` function after this process
should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a `run` function in _lib.rs_
- Handling the error if `run` returns an error

This pattern is about separating concerns: _main.rs_ handles running the
program and _lib.rs_ handles all the logic of the task at hand. Because you
can’t test the `main` function directly, this structure lets you test all of
your program’s logic by moving it into functions in _lib.rs_. The code that
remains in _main.rs_ will be small enough to verify its correctness by reading
it. Let’s rework our program by following this process.

#### Extracting the Argument Parser

We’ll extract the functionality for parsing arguments into a function that
`main` will call to prepare for moving the command line parsing logic to
_src/lib.rs_. Listing 12-5 shows the new start of `main` that calls a new
function `parse_config`, which we’ll define in _src/main.rs_ for the moment.

<Listing number="12-5" file-name="src/main.rs" caption="استخراج تابع `parse_config` از `main`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

ما همچنان آرگومان‌های خط فرمان را به یک بردار جمع‌آوری می‌کنیم، اما به جای اینکه مقدار آرگومان در اندیس (index)۱ را به متغیر `query` و مقدار آرگومان در اندیس (index)۲ را به متغیر `file_path` در تابع `main` اختصاص دهیم، کل بردار را به تابع `parse_config` ارسال می‌کنیم. تابع `parse_config` سپس منطق مشخص می‌کند که کدام آرگومان در کدام متغیر قرار می‌گیرد و مقادیر را به تابع `main` بازمی‌گرداند. ما همچنان متغیرهای `query` و `file_path` را در `main` ایجاد می‌کنیم، اما `main` دیگر مسئول تعیین ارتباط آرگومان‌های خط فرمان و متغیرها نیست.

این تغییر ممکن است برای برنامه کوچک ما زیاده‌روی به نظر برسد، اما ما در حال بازسازی کد به صورت گام‌های کوچک و تدریجی هستیم. پس از اعمال این تغییر، دوباره برنامه را اجرا کنید تا اطمینان حاصل کنید که تجزیه آرگومان همچنان کار می‌کند. بررسی مداوم پیشرفت کد کمک می‌کند تا در صورت بروز مشکلات، علت آن‌ها را سریع‌تر شناسایی کنید.

#### گروه‌بندی مقادیر پیکربندی

می‌توانیم یک گام کوچک دیگر برای بهبود بیشتر تابع `parse_config` برداریم. در حال حاضر، ما یک tuple بازمی‌گردانیم، اما بلافاصله آن tuple را به قسمت‌های جداگانه تقسیم می‌کنیم. این نشانه‌ای است که شاید هنوز انتزاع درستی نداریم.

نشانه دیگری که نشان می‌دهد جا برای بهبود وجود دارد، قسمت `config` در `parse_config` است، که نشان می‌دهد دو مقداری که بازمی‌گردانیم به هم مرتبط هستند و هر دو بخشی از یک مقدار پیکربندی هستند. ما در حال حاضر این معنا را در ساختار داده‌ها به جز با گروه‌بندی دو مقدار در یک tuple منتقل نمی‌کنیم؛ در عوض، این دو مقدار را در یک struct قرار می‌دهیم و به هر یک از فیلدهای struct نامی معنادار می‌دهیم. انجام این کار درک نحوه ارتباط مقادیر مختلف و هدف آن‌ها را برای نگهداری‌کنندگان آینده این کد آسان‌تر می‌کند.

لیست ۱۲-۶ بهبودهای تابع `parse_config` را نشان می‌دهد.

<Listing number="12-6" file-name="src/main.rs" caption="بازسازی `parse_config` برای بازگرداندن یک نمونه از struct `Config`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

ما یک ساختار جدید به نام `Config` تعریف کرده‌ایم که دارای فیلدهایی با نام‌های `query` و `file_path` است. امضای تابع `parse_config` اکنون نشان می‌دهد که این تابع یک مقدار `Config` را بازمی‌گرداند. در بدنه تابع `parse_config`، جایی که قبلاً اسلایس‌های رشته‌ای را که به مقادیر `String` در `args` اشاره می‌کردند بازمی‌گرداندیم، اکنون `Config` را طوری تعریف می‌کنیم که دارای مقادیر `String` متعلق به خود باشد.

متغیر `args` در تابع `main` مالک مقادیر آرگومان است و فقط به تابع `parse_config` اجازه قرض گرفتن آن‌ها را می‌دهد، به این معنی که اگر `Config` بخواهد مالک مقادیر در `args` شود، قوانین قرض‌گیری Rust را نقض می‌کنیم.

چندین روش برای مدیریت داده‌های `String` وجود دارد؛ ساده‌ترین و شاید ناکارآمدترین روش، فراخوانی متد `clone` روی مقادیر است. این کار یک کپی کامل از داده‌ها برای نمونه `Config` ایجاد می‌کند که مالک آن است. این روش زمان و حافظه بیشتری نسبت به ذخیره یک مرجع به داده‌ها نیاز دارد. با این حال، کپی کردن داده‌ها باعث می‌شود که کد ما بسیار ساده شود زیرا نیازی به مدیریت طول عمر مراجع نداریم؛ در این شرایط، از دست دادن کمی کارایی برای دستیابی به سادگی ارزشمند است.

> ### هزینه‌ها و مزایای استفاده از `clone`
>
> در بین بسیاری از برنامه‌نویسان Rust، تمایلی به استفاده از `clone` برای رفع مشکلات مالکیت به دلیل هزینه اجرای آن وجود دارد. در [فصل ۱۳][ch13]<!-- ignore -->، یاد خواهید گرفت که چگونه در این نوع موقعیت‌ها از روش‌های کارآمدتر استفاده کنید. اما در حال حاضر، کپی کردن چند رشته برای ادامه پیشرفت اشکالی ندارد زیرا این کپی‌ها فقط یک‌بار انجام می‌شوند و مسیر فایل و رشته جستجوی شما بسیار کوچک هستند. بهتر است یک برنامه کارا که کمی ناکارآمد است داشته باشید تا اینکه در اولین تلاش خود برای نوشتن کد، بهینه‌سازی بیش از حد انجام دهید. با تجربه بیشتر در Rust، شروع با راه‌حل کارآمدتر آسان‌تر خواهد بود، اما در حال حاضر استفاده از `clone` کاملاً قابل قبول است.

ما تابع `main` را به‌روزرسانی کردیم تا نمونه‌ای از `Config` که توسط `parse_config` بازگردانده می‌شود را در یک متغیر به نام `config` قرار دهد، و کدی که قبلاً از متغیرهای جداگانه `query` و `file_path` استفاده می‌کرد، اکنون از فیلدهای موجود در struct `Config` استفاده می‌کند.

اکنون کد ما به‌وضوح نشان می‌دهد که `query` و `file_path` به هم مرتبط هستند و هدف آن‌ها تنظیم نحوه کار برنامه است. هر کدی که از این مقادیر استفاده می‌کند می‌داند که باید آن‌ها را در نمونه `config` در فیلدهایی که نام آن‌ها برای هدفشان انتخاب شده است، پیدا کند.

#### ایجاد سازنده برای `Config`

تا اینجا، منطق مسئول تجزیه آرگومان‌های خط فرمان را از `main` استخراج کرده و در تابع `parse_config` قرار داده‌ایم. این کار به ما کمک کرد ببینیم که مقادیر `query` و `file_path` به هم مرتبط هستند و این رابطه باید در کد ما منتقل شود. سپس یک struct به نام `Config` اضافه کردیم تا هدف مشترک `query` و `file_path` را نام‌گذاری کنیم و بتوانیم نام مقادیر را به‌عنوان فیلدهای struct از تابع `parse_config` بازگردانیم.

حالا که هدف تابع `parse_config` ایجاد یک نمونه از `Config` است، می‌توانیم `parse_config` را از یک تابع معمولی به یک تابع با نام `new` تغییر دهیم که به struct `Config` مرتبط است. این تغییر کد را به‌صورت idiomatic‌تر می‌کند. ما می‌توانیم نمونه‌هایی از انواع موجود در کتابخانه استاندارد، مانند `String`، را با فراخوانی `String::new` ایجاد کنیم. به همین ترتیب، با تغییر `parse_config` به تابع `new` مرتبط با `Config`، می‌توانیم نمونه‌هایی از `Config` را با فراخوانی `Config::new` ایجاد کنیم. لیست ۱۲-۷ تغییرات لازم را نشان می‌دهد.

<Listing number="12-7" file-name="src/main.rs" caption="تغییر `parse_config` به `Config::new`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

ما تابع `main` را که در آن `parse_config` را فراخوانی می‌کردیم به‌روزرسانی کرده‌ایم تا به‌جای آن `Config::new` را فراخوانی کند. نام `parse_config` را به `new` تغییر داده و آن را در یک بلوک `impl` قرار داده‌ایم که تابع `new` را به `Config` مرتبط می‌کند. کد را دوباره کامپایل کنید تا مطمئن شوید که کار می‌کند.

### رفع مشکلات مدیریت خطا

حالا روی رفع مشکلات مدیریت خطا کار می‌کنیم. به خاطر بیاورید که تلاش برای دسترسی به مقادیر موجود در بردار `args` در اندیس (index)۱ یا ۲ باعث می‌شود برنامه در صورت داشتن کمتر از سه آیتم، دچار وحشت شود. برنامه را بدون هیچ آرگومانی اجرا کنید؛ این حالت به شکل زیر خواهد بود:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

خط `index out of bounds: the len is 1 but the index is 1` یک پیام خطا است که برای برنامه‌نویسان در نظر گرفته شده است. این پیام به کاربران نهایی کمکی نمی‌کند تا بفهمند باید چه کار کنند. حالا این مشکل را رفع می‌کنیم.

#### بهبود پیام خطا

در لیست ۱۲-۸، یک بررسی در تابع `new` اضافه می‌کنیم که بررسی می‌کند آیا آرایه به‌اندازه کافی طولانی است تا بتوان به اندیس‌های ۱ و ۲ دسترسی داشت. اگر طول آرایه کافی نباشد، برنامه دچار وحشت می‌شود و یک پیام خطای بهتر نمایش می‌دهد.

<Listing number="12-8" file-name="src/main.rs" caption="افزودن بررسی برای تعداد آرگومان‌ها">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

این کد شبیه به [تابع `Guess::new` که در لیست ۹-۱۳ نوشتیم][ch9-custom-types]<!-- ignore --> است، جایی که وقتی آرگومان `value` خارج از محدوده مقادیر معتبر بود، `panic!` فراخوانی کردیم. به جای بررسی محدوده مقادیر، در اینجا بررسی می‌کنیم که طول `args` حداقل برابر با `3` باشد و بقیه تابع می‌تواند با فرض اینکه این شرط برقرار شده است، عمل کند. اگر `args` کمتر از سه آیتم داشته باشد، این شرط `true` خواهد بود و ما ماکرو `panic!` را برای خاتمه برنامه بلافاصله فراخوانی می‌کنیم.

با این چند خط اضافی در `new`، بیایید دوباره برنامه را بدون هیچ آرگومانی اجرا کنیم تا ببینیم اکنون پیام خطا چگونه است:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

این خروجی بهتر است: اکنون یک پیام خطای منطقی داریم. با این حال، هنوز اطلاعات اضافی داریم که نمی‌خواهیم به کاربران خود ارائه دهیم. شاید تکنیکی که در لیست ۹-۱۳ استفاده کردیم بهترین گزینه برای اینجا نباشد: یک فراخوانی به `panic!` برای مشکل برنامه‌نویسی مناسب‌تر است تا یک مشکل استفاده، [همان‌طور که در فصل ۹ بحث شد][ch9-error-guidelines]<!-- ignore -->. در عوض، از تکنیک دیگری که در فصل ۹ یاد گرفتید استفاده می‌کنیم—[بازگرداندن یک `Result`][ch9-result]<!-- ignore --> که نشان‌دهنده موفقیت یا خطا است.

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### بازگرداندن یک `Result` به جای فراخوانی `panic!`

ما می‌توانیم به جای آن، یک مقدار `Result` بازگردانیم که در صورت موفقیت شامل یک نمونه از `Config` باشد و در صورت خطا مشکل را توصیف کند. همچنین قصد داریم نام تابع را از `new` به `build` تغییر دهیم زیرا بسیاری از برنامه‌نویسان انتظار دارند که توابع `new` هرگز شکست نخورند. وقتی `Config::build` با `main` ارتباط برقرار می‌کند، می‌توانیم از نوع `Result` برای اعلام مشکل استفاده کنیم. سپس می‌توانیم `main` را تغییر دهیم تا یک واریانت `Err` را به یک پیام خطای عملی‌تر برای کاربران خود تبدیل کنیم، بدون متن‌های اضافی مربوط به `thread 'main'` و `RUST_BACKTRACE` که یک فراخوانی به `panic!` ایجاد می‌کند.

لیست ۱۲-۹ تغییراتی را که باید در مقدار بازگشتی تابع که اکنون آن را `Config::build` می‌نامیم و بدنه تابع برای بازگرداندن یک `Result` ایجاد کنیم، نشان می‌دهد. توجه داشته باشید که این کد تا زمانی که `main` را نیز به‌روزرسانی نکنیم کامپایل نمی‌شود، که این کار را در لیست بعدی انجام خواهیم داد.

<Listing number="12-9" file-name="src/main.rs" caption="بازگرداندن یک `Result` از `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

#### تابع `build` و بازگشت مقدار `Result`

تابع `build` ما اکنون یک مقدار `Result` را بازمی‌گرداند که در صورت موفقیت شامل یک نمونه از `Config` و در صورت خطا یک مقدار رشته‌ای ثابت (`string literal`) است. مقادیر خطای ما همیشه رشته‌های ثابت با طول عمر `'static` خواهند بود.

ما دو تغییر در بدنه تابع ایجاد کرده‌ایم: به جای فراخوانی `panic!` زمانی که کاربر آرگومان‌های کافی ارائه نمی‌دهد، اکنون یک مقدار `Err` بازمی‌گردانیم و مقدار بازگشتی `Config` را در یک `Ok` قرار داده‌ایم. این تغییرات باعث می‌شوند تابع با امضای نوع جدید خود سازگار باشد.

بازگرداندن مقدار `Err` از `Config::build` به تابع `main` اجازه می‌دهد که مقدار `Result` بازگشتی از تابع `build` را مدیریت کرده و در صورت بروز خطا، فرآیند را به شکلی تمیزتر خاتمه دهد.

<a id="calling-confignew-and-handling-errors"></a>

#### فراخوانی `Config::build` و مدیریت خطاها

برای مدیریت حالت خطا و چاپ یک پیام دوستانه برای کاربر، باید تابع `main` را به‌روزرسانی کنیم تا مقدار `Result` بازگردانده‌شده توسط `Config::build` را مدیریت کند. این کار در لیست ۱۲-۱۰ نشان داده شده است. همچنین مسئولیت خاتمه دادن ابزار خط فرمان با کد خطای غیر صفر را از `panic!` گرفته و به صورت دستی پیاده‌سازی خواهیم کرد. کد خروجی غیر صفر به عنوان یک قرارداد برای اعلام وضعیت خطا به فرآیندی که برنامه ما را فراخوانده است، استفاده می‌شود.

<Listing number="12-10" file-name="src/main.rs" caption="خروج با کد خطا در صورت شکست در ساخت یک `Config`">


```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

در این لیستینگ، ما از متدی استفاده کرده‌ایم که هنوز جزئیات آن را به‌طور کامل پوشش نداده‌ایم: `unwrap_or_else`. این متد که در استاندارد کتابخانه Rust برای `Result<T, E>` تعریف شده است، به ما امکان می‌دهد مدیریت خطاهای سفارشی و بدون استفاده از `panic!` را تعریف کنیم. اگر مقدار `Result` از نوع `Ok` باشد، رفتار این متد مشابه `unwrap` است: مقدار داخلی که `Ok` در خود قرار داده را بازمی‌گرداند. با این حال، اگر مقدار از نوع `Err` باشد، این متد کدی را که در _closure_ تعریف کرده‌ایم اجرا می‌کند. _Closure_ یک تابع ناشناس است که آن را تعریف کرده و به‌عنوان آرگومان به `unwrap_or_else` ارسال می‌کنیم. 

ما closures را به تفصیل در [فصل ۱۳][ch13] توضیح خواهیم داد. فعلاً کافی است بدانید که `unwrap_or_else` مقدار داخلی `Err` را به _closure_ می‌دهد. در اینجا، مقدار استاتیک `"not enough arguments"` که در لیستینگ 12-9 اضافه کردیم، به _closure_ ارسال شده و به آرگومان `err` تخصیص داده می‌شود، که بین خط عمودی‌ها قرار دارد. کد درون _closure_ سپس می‌تواند از مقدار `err` استفاده کند.

ما همچنین یک خط جدید `use` اضافه کرده‌ایم تا `process` را از کتابخانه استاندارد به محدوده بیاوریم. کدی که در حالت خطا اجرا می‌شود تنها شامل دو خط است: ابتدا مقدار `err` را چاپ می‌کنیم و سپس `process::exit` را فراخوانی می‌کنیم. تابع `process::exit` بلافاصله برنامه را متوقف کرده و عددی که به‌عنوان کد وضعیت خروج ارسال شده است را بازمی‌گرداند. این روش شبیه مدیریت مبتنی بر `panic!` است که در لیستینگ 12-8 استفاده کردیم، اما دیگر خروجی اضافی تولید نمی‌شود. حالا آن را آزمایش کنیم:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

عالی! این خروجی برای کاربران ما بسیار دوستانه‌تر است.

### Extracting Logic from `main`

Now that we’ve finished refactoring the configuration parsing, let’s turn to
the program’s logic. As we stated in [“Separation of Concerns for Binary
Projects”](#separation-of-concerns-for-binary-projects)<!-- ignore -->, we’ll
extract a function named `run` that will hold all the logic currently in the
`main` function that isn’t involved with setting up configuration or handling
errors. When we’re done, `main` will be concise and easy to verify by
inspection, and we’ll be able to write tests for all the other logic.

Listing 12-11 shows the extracted `run` function. For now, we’re just making
the small, incremental improvement of extracting the function. We’re still
defining the function in _src/main.rs_.

<Listing number="12-11" file-name="src/main.rs" caption="استخراج تابع `run` از `main`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

با این تغییرات، `main` اکنون تابع `run` را فراخوانی می‌کند و مسئولیت اجرای منطق اصلی برنامه را به آن واگذار می‌کند. این جداسازی باعث می‌شود تابع `main` ساده‌تر شود و ما بتوانیم تست‌های دقیقی برای بخش‌های مختلف کد بنویسیم. این روش به بهبود قابلیت نگهداری و خوانایی کد کمک شایانی می‌کند.

### بازگرداندن خطاها از تابع `run`

اکنون که منطق باقی‌مانده برنامه را در تابع `run` جدا کرده‌ایم، می‌توانیم مانند `Config::build` در لیستینگ 12-9، مدیریت خطا را بهبود بخشیم. به جای اجازه دادن به برنامه برای اجرای `panic` با فراخوانی `expect`، تابع `run` در صورت بروز مشکل یک `Result<T, E>` بازمی‌گرداند. این رویکرد به ما امکان می‌دهد منطق مرتبط با مدیریت خطا را به صورت کاربرپسندانه‌ای در تابع `main` متمرکز کنیم. تغییرات لازم برای امضا و بدنه تابع `run` در لیستینگ 12-12 نشان داده شده است:

<Listing number="12-12" file-name="src/main.rs" caption="تغییر تابع `run` برای بازگرداندن `Result`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

### تغییرات مهم

<ul dir="rtl">
  <li>
    <strong>تغییر نوع بازگشتی:</strong>  
    نوع بازگشتی تابع <code>run</code> به <code>Result&lt;(), Box&lt;dyn Error&gt;&gt;</code> تغییر داده شده است. این تابع قبلاً نوع واحد (<code>()</code>) را بازمی‌گرداند، که همچنان برای حالت موفقیت حفظ شده است.  
    برای نوع خطا از یک <strong>شیء صفات</strong> به نام <code>Box&lt;dyn Error&gt;</code> استفاده کرده‌ایم (و با استفاده از <code>use</code>، <code>std::error::Error</code> را به محدوده آورده‌ایم). در فصل 18 بیشتر درباره شیء صفات صحبت خواهیم کرد. فعلاً کافی است بدانید که <code>Box&lt;dyn Error&gt;</code> به این معنا است که تابع می‌تواند نوعی از مقدار را که صفت <code>Error</code> را پیاده‌سازی کرده بازگرداند، بدون اینکه نوع خاصی را مشخص کند. کلمه کلیدی <code>dyn</code> به معنای <strong>دینامیک</strong> است.
  </li>
  <li>
    <strong>حذف <code>expect</code> و استفاده از عملگر <code>?</code>:</strong>  
    به جای استفاده از <code>panic!</code> در صورت بروز خطا، عملگر <code>?</code> مقدار خطا را از تابع جاری بازمی‌گرداند تا فراخوانی‌کننده بتواند آن را مدیریت کند.
  </li>
  <li>
    <strong>بازگرداندن مقدار <code>Ok</code> در حالت موفقیت:</strong>  
    تابع <code>run</code> اکنون در حالت موفقیت مقدار <code>Ok</code> را بازمی‌گرداند. ما نوع موفقیت تابع را به عنوان <code>()</code> در امضا تعریف کرده‌ایم، که به این معنا است که باید مقدار نوع واحد را در مقدار <code>Ok</code> قرار دهیم. نحو <code>Ok(())</code> ممکن است در ابتدا کمی عجیب به نظر برسد، اما استفاده از <code>()</code> به این صورت روش استاندارد برای نشان دادن این است که تابع <code>run</code> تنها برای تأثیرات جانبی فراخوانی شده و مقداری بازنمی‌گرداند که به آن نیاز داشته باشیم.
  </li>
</ul>
```

### بررسی کد

اجرای این کد باعث می‌شود که کد کامپایل شود اما یک هشدار نمایش دهد:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust به ما یادآوری می‌کند که کد ما مقدار `Result` را نادیده گرفته است و این مقدار ممکن است نشان‌دهنده بروز خطا باشد. اما ما بررسی نمی‌کنیم که آیا خطایی رخ داده است یا خیر، و کامپایلر به ما یادآوری می‌کند که احتمالاً نیاز به مدیریت خطا در این بخش داریم. اکنون این مشکل را اصلاح خواهیم کرد.

#### مدیریت خطاهای بازگردانده‌شده از `run` در `main`

ما خطاها را بررسی کرده و با استفاده از تکنیکی مشابه آنچه در `Config::build` در لیست ۱۲-۱۰ استفاده کردیم مدیریت می‌کنیم، اما با یک تفاوت کوچک:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

ما به جای `unwrap_or_else` از `if let` استفاده می‌کنیم تا بررسی کنیم آیا `run` یک مقدار `Err` بازمی‌گرداند یا خیر و در صورت وقوع، `process::exit(1)` را فراخوانی کنیم. تابع `run` مقداری بازنمی‌گرداند که بخواهیم به همان شیوه‌ای که `Config::build` نمونه `Config` را بازمی‌گرداند آن را `unwrap` کنیم. از آنجایی که `run` در صورت موفقیت مقدار `()` بازمی‌گرداند، ما فقط به شناسایی یک خطا اهمیت می‌دهیم، بنابراین نیازی به `unwrap_or_else` برای بازگرداندن مقدار آن نداریم، که تنها `()` خواهد بود.

بدنه‌های `if let` و `unwrap_or_else` در هر دو حالت یکسان هستند: ما خطا را چاپ کرده و خارج می‌شویم.

### Splitting Code into a Library Crate

Our `minigrep` project is looking good so far! Now we’ll split the
_src/main.rs_ file and put some code into the _src/lib.rs_ file. That way, we
can test the code and have a _src/main.rs_ file with fewer responsibilities.

Let’s move all the code that isn’t in the `main` function from _src/main.rs_ to
_src/lib.rs_:

- The `run` function definition
- The relevant `use` statements
- The definition of `Config`
- The `Config::build` function definition

The contents of _src/lib.rs_ should have the signatures shown in Listing 12-13
(we’ve omitted the bodies of the functions for brevity). Note that this won’t
compile until we modify _src/main.rs_ in Listing 12-14.

<Listing number="12-13" file-name="src/lib.rs" caption="Moving `Config` and `run` into *src/lib.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

</Listing>

We’ve made liberal use of the `pub` keyword: on `Config`, on its fields and its
`build` method, and on the `run` function. We now have a library crate that has
a public API we can test!

Now we need to bring the code we moved to _src/lib.rs_ into the scope of the
binary crate in _src/main.rs_, as shown in Listing 12-14.

<Listing number="12-14" file-name="src/main.rs" caption="Using the `minigrep` library crate in *src/main.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

We add a `use minigrep::Config` line to bring the `Config` type from the
library crate into the binary crate’s scope, and we prefix the `run` function
with our crate name. Now all the functionality should be connected and should
work. Run the program with `cargo run` and make sure everything works correctly.

وای! این یک کار سخت بود، اما ما خودمان را برای موفقیت در آینده آماده کردیم. اکنون مدیریت خطاها بسیار آسان‌تر شده است و کد ما ماژولارتر شده است. از اینجا به بعد تقریباً تمام کارهای ما در فایل _src/lib.rs_ انجام خواهد شد.

بیایید از این ماژولاریت جدید برای انجام کاری استفاده کنیم که با کد قبلی دشوار بود اما با کد جدید آسان است: نوشتن چند تست!

[ch13]: ch13-00-functional-features.html  
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation  
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling  
[ch9-result]: ch09-02-recoverable-errors-with-result.html  
[ch18]: ch18-00-oop.html  
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator  
