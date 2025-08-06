## مسیرها برای اشاره به یک آیتم در درخت ماژول

برای نشان دادن به Rust که یک آیتم را در درخت ماژول کجا پیدا کند، از یک مسیر استفاده می‌کنیم، مشابه استفاده از مسیر هنگام پیمایش در یک فایل‌سیستم. برای فراخوانی یک تابع، باید مسیر آن را بدانیم.

یک مسیر می‌تواند به دو شکل باشد:

- یک _مسیر مطلق_ مسیری کامل است که از ریشه جعبه (crate) شروع می‌شود؛ برای کدی که از یک جعبه (crate) خارجی می‌آید، مسیر مطلق با نام جعبه (crate) شروع می‌شود، و برای کدی که از جعبه (crate) فعلی می‌آید، با کلمه کلیدی `crate` شروع می‌شود.
- یک _مسیر نسبی_ از ماژول فعلی شروع می‌شود و از `self`، `super` یا یک شناسه در ماژول فعلی استفاده می‌کند.

هر دو مسیر مطلق و نسبی با یک یا چند شناسه که با دو نقطه دوبل (`::`) جدا شده‌اند دنبال می‌شوند.

با بازگشت به لیستینگ 7-1، فرض کنید که می‌خواهیم تابع `add_to_waitlist` را فراخوانی کنیم. این کار مشابه پرسیدن این است: مسیر تابع `add_to_waitlist` چیست؟ لیستینگ 7-3 شامل لیستینگ 7-1 با حذف برخی از ماژول‌ها و توابع است.

ما دو روش برای فراخوانی تابع `add_to_waitlist` از یک تابع جدید، `eat_at_restaurant`، که در ریشه جعبه (crate) تعریف شده است، نشان خواهیم داد. این مسیرها درست هستند، اما یک مشکل دیگر وجود دارد که مانع کامپایل این مثال به شکل فعلی می‌شود. بعداً توضیح خواهیم داد که چرا.

تابع `eat_at_restaurant` بخشی از API عمومی جعبه (crate) کتابخانه‌ای ما است، بنابراین آن را با کلمه کلیدی `pub` علامت می‌زنیم. در بخش [«آشکار کردن مسیرها با کلمه کلیدی `pub`»][pub]، به جزئیات بیشتری درباره `pub` خواهیم پرداخت.

<Listing number="7-3" file-name="src/lib.rs" caption="فراخوانی تابع `add_to_waitlist` با استفاده از مسیرهای مطلق و نسبی">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

</Listing>

بار اولی که تابع `add_to_waitlist` را در `eat_at_restaurant` فراخوانی می‌کنیم، از یک مسیر مطلق استفاده می‌کنیم. تابع `add_to_waitlist` در همان جعبه (crate) تعریف شده است که `eat_at_restaurant` در آن قرار دارد، که به این معنی است که می‌توانیم از کلمه کلیدی `crate` برای شروع مسیر مطلق استفاده کنیم. سپس هر یک از ماژول‌های متوالی را شامل می‌کنیم تا به `add_to_waitlist` برسیم. می‌توانید یک فایل‌سیستم با ساختار مشابه تصور کنید: ما مسیر `/front_of_house/hosting/add_to_waitlist` را برای اجرای برنامه `add_to_waitlist` مشخص می‌کنیم؛ استفاده از نام `crate` برای شروع از ریشه جعبه (crate) مانند استفاده از `/` برای شروع از ریشه فایل‌سیستم در شل است.

بار دوم که تابع `add_to_waitlist` را در `eat_at_restaurant` فراخوانی می‌کنیم، از یک مسیر نسبی استفاده می‌کنیم. مسیر با `front_of_house` شروع می‌شود، که نام ماژولی است که در همان سطح از درخت ماژول به عنوان `eat_at_restaurant` تعریف شده است. اینجا معادل فایل‌سیستم استفاده از مسیر `front_of_house/hosting/add_to_waitlist` است. شروع با نام ماژول به این معنی است که مسیر نسبی است.

### انتخاب بین مسیرهای مطلق و نسبی

انتخاب بین استفاده از مسیر نسبی یا مطلق یک تصمیم است که بر اساس پروژه شما گرفته می‌شود، و به این بستگی دارد که آیا احتمال بیشتری دارد کد تعریف آیتم را به طور مستقل از یا همراه با کدی که از آیتم استفاده می‌کند جابجا کنید. برای مثال، اگر ماژول `front_of_house` و تابع `eat_at_restaurant` را به یک ماژول به نام `customer_experience` منتقل کنیم، باید مسیر مطلق به `add_to_waitlist` را به‌روزرسانی کنیم، اما مسیر نسبی همچنان معتبر خواهد بود. با این حال، اگر تابع `eat_at_restaurant` را به طور مستقل به یک ماژول به نام `dining` منتقل کنیم، مسیر مطلق به فراخوانی `add_to_waitlist` تغییر نمی‌کند، اما مسیر نسبی باید به‌روزرسانی شود. ترجیح ما به طور کلی این است که مسیرهای مطلق را مشخص کنیم زیرا احتمال بیشتری دارد که بخواهیم تعریف کد و فراخوانی آیتم‌ها را مستقل از یکدیگر جابجا کنیم.

بیایید سعی کنیم کد لیستینگ 7-3 را کامپایل کنیم و ببینیم چرا هنوز کامپایل نمی‌شود! خطاهایی که دریافت می‌کنیم در لیستینگ 7-4 نشان داده شده‌اند.

<Listing number="7-4" caption="خطاهای کامپایلر هنگام ساخت کد در لیستینگ 7-3">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

</Listing>

پیام‌های خطا می‌گویند که ماژول `hosting` خصوصی است. به عبارت دیگر، ما مسیرهای صحیح برای ماژول `hosting` و تابع `add_to_waitlist` داریم، اما Rust به ما اجازه نمی‌دهد از آن‌ها استفاده کنیم زیرا به بخش‌های خصوصی دسترسی ندارد. در Rust، تمام آیتم‌ها (توابع، متدها، ساختارها، enumها، ماژول‌ها و ثابت‌ها) به صورت پیش‌فرض برای ماژول‌های والد خصوصی هستند. اگر بخواهید آیتمی مانند یک تابع یا ساختار را خصوصی کنید، آن را در یک ماژول قرار می‌دهید.

آیتم‌های موجود در یک ماژول والد نمی‌توانند از آیتم‌های خصوصی درون ماژول‌های فرزند استفاده کنند، اما آیتم‌های درون ماژول‌های فرزند می‌توانند از آیتم‌های ماژول‌های اجداد خود استفاده کنند. این به این دلیل است که ماژول‌های فرزند جزئیات پیاده‌سازی خود را بسته‌بندی و پنهان می‌کنند، اما ماژول‌های فرزند می‌توانند زمینه‌ای که در آن تعریف شده‌اند را ببینند. برای ادامه مثال، قواعد حریم خصوصی را مانند دفتر پشتی یک رستوران تصور کنید: آنچه در آنجا می‌گذرد برای مشتریان رستوران خصوصی است، اما مدیران دفتر می‌توانند همه چیز را در رستوران ببینند و انجام دهند.

Rust تصمیم گرفته است که سیستم ماژول به این صورت کار کند تا پنهان کردن جزئیات پیاده‌سازی داخلی به صورت پیش‌فرض باشد. به این ترتیب، می‌دانید کدام بخش‌های کد داخلی را می‌توانید تغییر دهید بدون اینکه کد بیرونی را خراب کنید. با این حال، Rust به شما این امکان را می‌دهد که بخش‌های داخلی کد ماژول‌های فرزند را به ماژول‌های اجداد بیرونی با استفاده از کلمه کلیدی `pub` عمومی کنید.

### آشکار کردن مسیرها با کلمه کلیدی `pub`

بیایید به خطای لیستینگ 7-4 برگردیم که به ما گفت ماژول `hosting` خصوصی است. ما می‌خواهیم تابع `eat_at_restaurant` در ماژول والد به تابع `add_to_waitlist` در ماژول فرزند دسترسی داشته باشد، بنابراین ماژول `hosting` را با کلمه کلیدی `pub` علامت می‌زنیم، همان‌طور که در لیستینگ 7-5 نشان داده شده است.

<Listing number="7-5" file-name="src/lib.rs" caption="اعلان ماژول `hosting` به عنوان `pub` برای استفاده از آن در `eat_at_restaurant`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

</Listing>

متأسفانه، کد در لیستینگ 7-5 همچنان به خطاهای کامپایلر منجر می‌شود، همان‌طور که در لیستینگ 7-6 نشان داده شده است.

<Listing number="7-6" caption="خطاهای کامپایلر هنگام ساخت کد در لیستینگ 7-5">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

</Listing>

چه اتفاقی افتاد؟ اضافه کردن کلمه کلیدی `pub` در جلوی `mod hosting` ماژول را عمومی می‌کند. با این تغییر، اگر به `front_of_house` دسترسی داشته باشیم، می‌توانیم به `hosting` نیز دسترسی داشته باشیم. اما _محتویات_ `hosting` همچنان خصوصی است؛ عمومی کردن ماژول به معنای عمومی کردن محتوای آن نیست. کلمه کلیدی `pub` روی یک ماژول فقط به کدهای موجود در ماژول‌های اجداد اجازه می‌دهد به آن ارجاع دهند، نه اینکه به کد داخلی آن دسترسی داشته باشند. از آنجایی که ماژول‌ها به عنوان ظرف عمل می‌کنند، تنها عمومی کردن ماژول کافی نیست؛ باید فراتر رفته و یک یا چند مورد از آیتم‌های درون ماژول را نیز عمومی کنیم.

خطاهای موجود در لیستینگ 7-6 نشان می‌دهند که تابع `add_to_waitlist` خصوصی است. قواعد حریم خصوصی برای ساختارها، enumها، توابع، متدها و همچنین ماژول‌ها اعمال می‌شوند.

بیایید تابع `add_to_waitlist` را نیز با اضافه کردن کلمه کلیدی `pub` قبل از تعریف آن عمومی کنیم، همان‌طور که در لیستینگ 7-7 نشان داده شده است.

<Listing number="7-7" file-name="src/lib.rs" caption="اضافه کردن کلمه کلیدی `pub` به `mod hosting` و `fn add_to_waitlist` به ما اجازه می‌دهد تابع را از `eat_at_restaurant` فراخوانی کنیم">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

</Listing>

Now the code will compile! To see why adding the `pub` keyword lets us use
these paths in `eat_at_restaurant` with respect to the privacy rules, let’s look
at the absolute and the relative paths.

In the absolute path, we start with `crate`, the root of our crate’s module
tree. The `front_of_house` module is defined in the crate root. While
`front_of_house` isn’t public, because the `eat_at_restaurant` function is
defined in the same module as `front_of_house` (that is, `eat_at_restaurant`
and `front_of_house` are siblings), we can refer to `front_of_house` from
`eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can
access the parent module of `hosting`, so we can access `hosting`. Finally, the
`add_to_waitlist` function is marked with `pub` and we can access its parent
module, so this function call works!

In the relative path, the logic is the same as the absolute path except for the
first step: rather than starting from the crate root, the path starts from
`front_of_house`. The `front_of_house` module is defined within the same module
as `eat_at_restaurant`, so the relative path starting from the module in which
`eat_at_restaurant` is defined works. Then, because `hosting` and
`add_to_waitlist` are marked with `pub`, the rest of the path works, and this
function call is valid!

If you plan on sharing your library crate so other projects can use your code,
your public API is your contract with users of your crate that determines how
they can interact with your code. There are many considerations around managing
changes to your public API to make it easier for people to depend on your
crate. These considerations are out of the scope of this book; if you’re
interested in this topic, see [The Rust API Guidelines][api-guidelines].

> #### بهترین شیوه‌ها برای بسته‌هایی که یک جعبه (crate) باینری و یک جعبه (crate) کتابخانه‌ای دارند
>
> We mentioned that a package can contain both a _src/main.rs_ binary crate
> root as well as a _src/lib.rs_ library crate root, and both crates will have
> the package name by default. Typically, packages with this pattern of
> containing both a library and a binary crate will have just enough code in the
> binary crate to start an executable that calls code within the library crate.
> This lets other projects benefit from most of the functionality that the
> package provides because the library crate’s code can be shared.
>
> درخت ماژول باید در _src/lib.rs_ تعریف شود. سپس، هر آیتم عمومی را می‌توان در جعبه (crate) باینری با شروع مسیرها با نام بسته استفاده کرد. جعبه (crate) باینری به یک کاربر از جعبه (crate) کتابخانه‌ای تبدیل می‌شود، درست مثل اینکه یک جعبه (crate) کاملاً خارجی از جعبه (crate) کتابخانه‌ای استفاده می‌کند: تنها می‌تواند از API عمومی استفاده کند. این کار به شما کمک می‌کند یک API خوب طراحی کنید؛ نه تنها نویسنده آن هستید، بلکه یک کاربر نیز هستید!
>
> In [Chapter 12][ch12]<!-- ignore -->, we’ll demonstrate this organizational
> practice with a command-line program that will contain both a binary crate
> and a library crate.

### Starting Relative Paths with `super`

We can construct relative paths that begin in the parent module, rather than
the current module or the crate root, by using `super` at the start of the
path. This is like starting a filesystem path with the `..` syntax. Using
`super` allows us to reference an item that we know is in the parent module,
which can make rearranging the module tree easier when the module is closely
related to the parent but the parent might be moved elsewhere in the module
tree someday.

Consider the code in Listing 7-8 that models the situation in which a chef
fixes an incorrect order and personally brings it out to the customer. The
function `fix_incorrect_order` defined in the `back_of_house` module calls the
function `deliver_order` defined in the parent module by specifying the path to
`deliver_order`, starting with `super`.

<Listing number="7-8" file-name="src/lib.rs" caption="فراخوانی یک تابع با استفاده از یک مسیر نسبی که با `super` شروع می‌شود">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

</Listing>

تابع `fix_incorrect_order` در ماژول `back_of_house` است، بنابراین می‌توانیم از `super` برای رفتن به ماژول والد `back_of_house` استفاده کنیم، که در این مورد `crate`، یعنی ریشه است. از آنجا به دنبال `deliver_order` می‌گردیم و آن را پیدا می‌کنیم. موفقیت! ما فکر می‌کنیم که ماژول `back_of_house` و تابع `deliver_order` احتمالاً در همان رابطه با یکدیگر باقی می‌مانند و اگر بخواهیم درخت ماژول جعبه (crate) را سازماندهی مجدد کنیم، با هم جابجا می‌شوند. بنابراین، از `super` استفاده کردیم تا در آینده، اگر این کد به ماژول دیگری منتقل شد، تغییرات کمتری در کد لازم باشد.

### عمومی کردن ساختارها و enumها

ما همچنین می‌توانیم از `pub` برای مشخص کردن ساختارها و enumها به عنوان عمومی استفاده کنیم، اما چند جزئیات اضافی در مورد استفاده از `pub` با ساختارها و enumها وجود دارد. اگر از `pub` قبل از تعریف یک ساختار استفاده کنیم، ساختار عمومی می‌شود، اما فیلدهای ساختار همچنان خصوصی خواهند بود. ما می‌توانیم هر فیلد را به صورت موردی عمومی یا خصوصی کنیم. در لیستینگ 7-9، یک ساختار عمومی به نام `back_of_house::Breakfast` تعریف کرده‌ایم که یک فیلد عمومی به نام `toast` دارد اما فیلد `seasonal_fruit` خصوصی است. این مدل‌سازی حالتی است که در آن مشتری می‌تواند نوع نان همراه با وعده غذایی را انتخاب کند، اما سرآشپز تصمیم می‌گیرد که کدام میوه همراه وعده غذایی باشد بر اساس آنچه در فصل و موجودی است. میوه‌های موجود به سرعت تغییر می‌کنند، بنابراین مشتریان نمی‌توانند میوه را انتخاب کنند یا حتی ببینند که چه میوه‌ای دریافت خواهند کرد.

<Listing number="7-9" file-name="src/lib.rs" caption="یک ساختار با برخی فیلدهای عمومی و برخی خصوصی">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

</Listing>

از آنجا که فیلد `toast` در ساختار `back_of_house::Breakfast` عمومی است، می‌توانیم در `eat_at_restaurant` به این فیلد با استفاده از نقطه‌گذاری مقدار بدهیم یا مقدار آن را بخوانیم. توجه کنید که نمی‌توانیم از فیلد `seasonal_fruit` در `eat_at_restaurant` استفاده کنیم، زیرا `seasonal_fruit` خصوصی است. خطی که مقدار فیلد `seasonal_fruit` را تغییر می‌دهد را لغو کامنت کنید تا ببینید چه خطایی دریافت می‌کنید!

همچنین توجه کنید که چون `back_of_house::Breakfast` یک فیلد خصوصی دارد، ساختار باید یک تابع وابسته عمومی ارائه دهد که یک نمونه از `Breakfast` بسازد (ما آن را اینجا `summer` نامیده‌ایم). اگر `Breakfast` چنین تابعی نداشت، نمی‌توانستیم یک نمونه از `Breakfast` را در `eat_at_restaurant` ایجاد کنیم، زیرا نمی‌توانستیم مقدار فیلد خصوصی `seasonal_fruit` را در `eat_at_restaurant` تنظیم کنیم.

در مقابل، اگر یک enum را عمومی کنیم، تمام متغیرهای آن نیز عمومی می‌شوند. ما فقط به `pub` قبل از کلمه کلیدی `enum` نیاز داریم، همان‌طور که در لیستینگ 7-10 نشان داده شده است.


<Listing number="7-10" file-name="src/lib.rs" caption="Designating an enum as public makes all its variants public">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

</Listing>

از آنجایی که enum `Appetizer` را عمومی کردیم، می‌توانیم از متغیرهای `Soup` و `Salad` در `eat_at_restaurant` استفاده کنیم.

Enums خیلی مفید نیستند مگر اینکه متغیرهای آن‌ها عمومی باشند؛ اضافه کردن `pub` به تمام متغیرهای enum در هر مورد کار خسته‌کننده‌ای خواهد بود، بنابراین به طور پیش‌فرض متغیرهای enum عمومی هستند. ساختارها اغلب بدون عمومی بودن فیلدهایشان مفید هستند، بنابراین فیلدهای ساختار از قانون کلی پیروی می‌کنند که همه چیز به صورت پیش‌فرض خصوصی است مگر اینکه با `pub` مشخص شود.

یک وضعیت دیگر مرتبط با `pub` وجود دارد که هنوز آن را پوشش نداده‌ایم، و آن آخرین ویژگی سیستم ماژول ما است: کلمه کلیدی `use`. ابتدا `use` را به تنهایی بررسی خواهیم کرد، و سپس نشان خواهیم داد چگونه `pub` و `use` را ترکیب کنیم.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
