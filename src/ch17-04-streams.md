## Stream‌ها: Futures به صورت متوالی

تا اینجا در این فصل، بیشتر به آینده‌های فردی (_individual futures_) پایبند بوده‌ایم. یک استثنای بزرگ استفاده از کانال async بود. به یاد بیاورید چگونه در ابتدای این فصل در بخش [“ارسال پیام”][17-02-messages]<!-- ignore --> از گیرنده کانال async استفاده کردیم. متد async به نام `recv` یک دنباله از آیتم‌ها را در طول زمان تولید می‌کند. این یک نمونه از یک الگوی کلی‌تر به نام _stream_ است.

ما پیش‌تر در فصل ۱۳ با یک توالی از آیتم‌ها مواجه شدیم، زمانی که به `Iterator` و متد `next` آن در بخش [ویژگی Iterator و متد `next`][iterator-trait]<!-- ignore --> پرداختیم، اما بین `Iterator`ها و گیرنده‌ی ناهمگام کانال‌ها دو تفاوت وجود دارد.
تفاوت اول مربوط به *زمان* است: `Iterator`ها همگام (synchronous) هستند، در حالی که گیرنده‌ی کانال ناهمگام (asynchronous) است.
تفاوت دوم در *رابط برنامه‌نویسی کاربردی (API)* است. وقتی به‌صورت مستقیم با `Iterator` کار می‌کنیم، از متد همگام `next` استفاده می‌کنیم. در `stream`‌ مربوط به `trpl::Receiver`، ما به جای آن متد ناهمگام `recv` را فراخوانی کردیم.
با این وجود، این APIها از لحاظ کارکرد بسیار مشابه هستند، و این شباهت اتفاقی نیست. یک *stream* در واقع شکل ناهمگام پیمایش (iteration) است. در حالی که `trpl::Receiver` به‌طور خاص منتظر دریافت پیام می‌ماند، API عمومی‌تر stream بسیار گسترده‌تر است: این API، آیتم بعدی را به همان شیوه‌ای که `Iterator` فراهم می‌کند، ولی به‌صورت ناهمگام ارائه می‌دهد.


<Listing number="17-30" caption="ایجاد یک stream از یک iterator و چاپ مقادیر آن" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-30/src/main.rs:stream}}
```

</Listing>

ما با یک آرایه از اعداد شروع می‌کنیم، آن را به یک iterator تبدیل کرده و سپس متد `map` را فراخوانی می‌کنیم تا تمام مقادیر را دو برابر کنیم. سپس با استفاده از تابع `trpl::stream_from_iter`، این iterator را به یک stream تبدیل می‌کنیم. در ادامه، با استفاده از حلقه `while let`، بر روی آیتم‌های موجود در stream که به مرور می‌رسند، حلقه می‌زنیم.

متأسفانه، وقتی سعی می‌کنیم این کد را اجرا کنیم، کامپایل نمی‌شود و به جای آن گزارش می‌دهد که متد `next` در دسترس نیست:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-30
cargo build
copy only the error output
-->

```console
error[E0599]: no method named `next` found for struct `Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = note: the full type name has been written to 'file:///projects/async-await/target/debug/deps/async_await-575db3dd3197d257.long-type-14490787947592691573.txt'
   = note: consider using `--verbose` to print the full type name to the console
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

همان‌طور که این خروجی توضیح می‌دهد، دلیل خطای کامپایلر این است که برای استفاده از متد `next` باید ویژگی مناسب در دامنه باشد. با توجه به بحث‌هایی که تاکنون داشته‌ایم، ممکن است منطقی باشد که انتظار داشته باشید این ویژگی `Stream` باشد، اما در واقع `StreamExt` است. `Ext` که مخفف _extension_ است، یک الگوی رایج در جامعه Rust برای گسترش یک ویژگی با ویژگی دیگر است.

ما در انتهای این فصل ویژگی‌های `Stream` و `StreamExt` را با جزئیات بیشتری توضیح خواهیم داد، اما فعلاً تنها چیزی که باید بدانید این است که ویژگی `Stream` یک رابط سطح پایین تعریف می‌کند که به طور مؤثری ویژگی‌های `Iterator` و `Future` را ترکیب می‌کند. `StreamExt` مجموعه‌ای از APIهای سطح بالاتر را روی `Stream` ارائه می‌دهد، از جمله متد `next` و همچنین متدهای کاربردی دیگر مشابه آنچه ویژگی `Iterator` ارائه می‌دهد. `Stream` و `StreamExt` هنوز بخشی از کتابخانه استاندارد Rust نیستند، اما بیشتر crateهای اکوسیستم از همین تعریف استفاده می‌کنند.


برای رفع خطای کامپایل، باید یک دستور `use` برای `trpl::StreamExt` اضافه کنیم، همان‌طور که در فهرست 17-31 آمده است.

<Listing number="17-31" caption="استفاده موفق از یک iterator به‌عنوان پایه‌ای برای یک stream" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-31/src/main.rs:all}}
```

</Listing>

با قرار دادن همه این قطعات در کنار هم، این کد به همان روشی که می‌خواهیم کار می‌کند! مهم‌تر از همه، اکنون که `StreamExt` در دامنه داریم، می‌توانیم از تمام متدهای کاربردی آن استفاده کنیم، درست مانند iteratorها. برای مثال، در فهرست 17-32، از متد `filter` برای فیلتر کردن همه چیز به جز مضرب‌های سه و پنج استفاده می‌کنیم.

<Listing number="17-32" caption="فیلتر کردن یک `Stream` با استفاده از متد `StreamExt::filter`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-32/src/main.rs:all}}
```

</Listing>

البته این خیلی جالب نیست، چون می‌توانستیم همین کار را با iteratorهای معمولی و بدون هیچ async انجام دهیم. بیایید ببینیم چه کاری می‌توانیم انجام دهیم که _منحصربه‌فرد_ برای stream‌ها باشد.

### ترکیب Stream‌ها

بسیاری از مفاهیم به طور طبیعی به‌عنوان stream‌ها نمایش داده می‌شوند: آیتم‌هایی که در یک صف در دسترس می‌شوند، بخش‌هایی از داده که به صورت تدریجی از سیستم فایل خوانده می‌شوند وقتی مجموعه داده کامل برای حافظه کامپیوتر بیش از حد بزرگ است، یا داده‌هایی که به مرور زمان از طریق شبکه می‌رسند. چون stream‌ها نیز futures هستند، می‌توانیم از آن‌ها با هر نوع دیگر future استفاده کنیم و آن‌ها را به روش‌های جالبی ترکیب کنیم. برای مثال، می‌توانیم رویدادها را به صورت دسته‌ای جمع کنیم تا از ایجاد تعداد زیادی فراخوانی شبکه جلوگیری کنیم، تایم‌اوت‌هایی روی دنباله‌ای از عملیات‌های طولانی تنظیم کنیم، یا رویدادهای رابط کاربری را کنترل کنیم تا از انجام کارهای غیرضروری اجتناب کنیم.

بیایید با ساخت یک stream کوچک از پیام‌ها شروع کنیم که به‌عنوان یک جایگزین برای یک stream از داده‌هایی که ممکن است از یک WebSocket یا یک پروتکل ارتباطی بلادرنگ دیگر ببینیم، همان‌طور که در لیست ۱۷-۳۳ نشان داده شده است.


<Listing number="17-33" caption="استفاده از گیرنده `rx` به‌عنوان یک `ReceiverStream`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-33/src/main.rs:all}}
```

</Listing>

ابتدا یک تابع به نام `get_messages` ایجاد می‌کنیم که `impl Stream<Item = String>` را بازمی‌گرداند. برای پیاده‌سازی آن، یک کانال async ایجاد می‌کنیم، بر روی ۱۰ حرف اول الفبای انگلیسی حلقه می‌زنیم، و آن‌ها را از طریق کانال ارسال می‌کنیم.

همچنین از یک نوع جدید به نام `ReceiverStream` استفاده می‌کنیم، که `rx` گیرنده از `trpl::channel` را به یک `Stream` با متد `next` تبدیل می‌کند. دوباره در `main`، از یک حلقه `while let` برای چاپ تمام پیام‌ها از stream استفاده می‌کنیم.

وقتی این کد را اجرا می‌کنیم، دقیقاً نتایجی را که انتظار داریم دریافت می‌کنیم:


<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Message: 'a'
Message: 'b'
Message: 'c'
Message: 'd'
Message: 'e'
Message: 'f'
Message: 'g'
Message: 'h'
Message: 'i'
Message: 'j'
```

دوباره، می‌توانستیم این کار را با API معمولی `Receiver` یا حتی API معمولی `Iterator` انجام دهیم، اما بیایید ویژگی‌ای اضافه کنیم که نیاز به streams داشته باشد: اضافه کردن یک تایم‌اوت که برای هر آیتم در stream اعمال شود، و یک تأخیر روی آیتم‌هایی که ارسال می‌کنیم، همان‌طور که در لیست ۱۷-۳۴ نشان داده شده است.

<Listing number="17-34" caption="استفاده از متد `StreamExt::timeout` برای تعیین یک محدودیت زمانی برای آیتم‌های موجود در یک stream" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-34/src/main.rs:timeout}}
```

</Listing>

ابتدا یک تایم‌اوت به stream با استفاده از متد `timeout` اضافه می‌کنیم، که از ویژگی `StreamExt` می‌آید. سپس بدنه حلقه `while let` را به‌روزرسانی می‌کنیم، زیرا اکنون stream یک `Result` بازمی‌گرداند. حالت `Ok` نشان‌دهنده این است که یک پیام به‌موقع رسیده است؛ حالت `Err` نشان می‌دهد که تایم‌اوت قبل از رسیدن هر پیامی منقضی شده است. روی این نتیجه یک `match` انجام می‌دهیم و یا پیام را وقتی با موفقیت دریافت می‌کنیم چاپ می‌کنیم، یا اخطاری درباره تایم‌اوت چاپ می‌کنیم. در نهایت، توجه کنید که پس از اعمال تایم‌اوت به پیام‌ها، آن‌ها را pin می‌کنیم، زیرا ابزار تایم‌اوت یک stream تولید می‌کند که باید pin شود تا بتوان آن را poll کرد.

با این حال، چون بین پیام‌ها تأخیری وجود ندارد، این تایم‌اوت رفتار برنامه را تغییر نمی‌دهد. بیایید یک تأخیر متغیر به پیام‌هایی که ارسال می‌کنیم اضافه کنیم، همان‌طور که در لیست ۱۷-۳۵ نشان داده شده است.

<Listing number="17-35" caption="ارسال پیام‌ها از طریق `tx` با یک تأخیر async بدون تبدیل `get_messages` به یک تابع async" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-35/src/main.rs:messages}}
```

</Listing>


برای خوابیدن بین پیام‌ها در تابع `get_messages` بدون مسدود کردن، باید از async استفاده کنیم. با این حال، نمی‌توانیم خود `get_messages` را به یک تابع async تبدیل کنیم، زیرا در این صورت یک `Future<Output = Stream<Item = String>>` به جای یک `Stream<Item = String>` بازمی‌گرداند. کاربر باید خود `get_messages` را منتظر بماند تا به stream دسترسی پیدا کند. اما به یاد داشته باشید: هر چیزی در یک آینده مشخص به‌صورت خطی اتفاق می‌افتد؛ همزمانی _بین_ آینده‌ها اتفاق می‌افتد. انتظار برای `get_messages` نیاز دارد که تمام پیام‌ها را ارسال کند، از جمله خوابیدن بین ارسال هر پیام، قبل از بازگرداندن stream گیرنده. در نتیجه، زمان محدود بی‌فایده می‌شود. هیچ تأخیری در خود stream وجود نخواهد داشت: تمام تأخیرها قبل از در دسترس قرار گرفتن stream اتفاق می‌افتد.

در عوض، `get_messages` را به‌عنوان یک تابع معمولی که یک stream بازمی‌گرداند باقی می‌گذاریم و یک تسک برای مدیریت فراخوانی‌های async `sleep` ایجاد می‌کنیم.

> نکته: فراخوانی `spawn_task` به این روش کار می‌کند زیرا ما از قبل runtime خود را تنظیم کرده‌ایم. فراخوانی این پیاده‌سازی خاص از `spawn_task` _بدون_ تنظیم اولیه یک runtime باعث panic می‌شود. پیاده‌سازی‌های دیگر معاملات متفاوتی انتخاب می‌کنند: ممکن است یک runtime جدید ایجاد کنند و بنابراین از panic اجتناب کنند، اما با کمی سربار اضافی مواجه شوند، یا به سادگی راهی مستقل برای ایجاد تسک‌ها بدون ارجاع به یک runtime ارائه ندهند. باید مطمئن شوید که می‌دانید runtime شما چه معامله‌ای انتخاب کرده است و کد خود را بر این اساس بنویسید!

اکنون کد ما نتیجه بسیار جالب‌تری دارد! بین هر جفت پیام، یک خطا گزارش می‌شود: `Problem: Elapsed(())`.

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Message: 'a'
Problem: Elapsed(())
Message: 'b'
Message: 'c'
Problem: Elapsed(())
Message: 'd'
Message: 'e'
Problem: Elapsed(())
Message: 'f'
Message: 'g'
Problem: Elapsed(())
Message: 'h'
Message: 'i'
Problem: Elapsed(())
Message: 'j'
```

تایم‌اوت از رسیدن پیام‌ها در نهایت جلوگیری نمی‌کند. ما همچنان تمام پیام‌های اصلی را دریافت می‌کنیم، زیرا کانال ما _بدون محدودیت_ است: می‌تواند به اندازه‌ای که در حافظه جا شود پیام‌ها را نگه دارد. اگر پیام قبل از تایم‌اوت نرسد، handler stream ما آن را مدیریت می‌کند، اما وقتی دوباره stream را poll کند، ممکن است پیام اکنون رسیده باشد.

اگر به رفتار متفاوتی نیاز دارید، می‌توانید از انواع دیگر کانال‌ها یا به طور کلی انواع دیگر streamها استفاده کنید. بیایید یکی از این موارد را در عمل ببینیم، با ترکیب یک stream از فواصل زمانی با این stream از پیام‌ها.

### ترکیب Streamها

ابتدا، یک stream دیگر ایجاد می‌کنیم که اگر به طور مستقیم اجرا شود، هر میلی‌ثانیه یک آیتم ارسال می‌کند. برای سادگی، می‌توانیم از تابع `sleep` برای ارسال یک پیام با تأخیر استفاده کنیم و آن را با همان روشی که در `get_messages` استفاده کردیم—ایجاد یک stream از یک کانال—ترکیب کنیم. تفاوت این است که این بار، می‌خواهیم تعداد فواصل زمانی که گذشته‌اند را بازگردانیم، بنابراین نوع بازگشتی `impl Stream<Item = u32>` خواهد بود، و می‌توانیم تابع را `get_intervals` بنامیم (نگاه کنید به لیست ۱۷-۳۶).


<Listing number="17-36" caption="ایجاد یک stream با یک شمارنده که هر میلی‌ثانیه یک بار ارسال می‌شود" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-36/src/main.rs:intervals}}
```

</Listing>

ابتدا یک متغیر `count` را درون task تعریف می‌کنیم. (می‌توانستیم آن را خارج از task نیز تعریف کنیم، اما محدود کردن دامنه هر متغیر داده‌شده واضح‌تر است.) سپس یک حلقه بی‌نهایت ایجاد می‌کنیم. در هر تکرار حلقه، به صورت ناهمزمان به مدت یک میلی‌ثانیه می‌خوابد، مقدار `count` را افزایش می‌دهد و سپس آن را از طریق کانال ارسال می‌کند. از آنجا که همه این‌ها درون taskی که توسط `spawn_task` ایجاد شده است قرار دارد، همه آن—از جمله حلقه بی‌نهایت—همراه با runtime پاک‌سازی می‌شود.

این نوع حلقه بی‌نهایت، که تنها زمانی به پایان می‌رسد که کل runtime از بین برود، در async Rust نسبتاً رایج است: بسیاری از برنامه‌ها نیاز دارند که به طور نامحدود اجرا شوند. با async، این کار چیزی دیگر را مسدود نمی‌کند، تا زمانی که حداقل یک نقطه انتظار (_await point_) در هر تکرار از حلقه وجود داشته باشد.

حالا، درون بلوک async تابع اصلی ما، می‌توانیم تلاش کنیم که streamهای `messages` و `intervals` را با هم ترکیب کنیم، همان‌طور که در لیست ۱۷-۳۷ نشان داده شده است.

<Listing number="17-37" caption="تلاش برای ترکیب streamهای `messages` و `intervals`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-37/src/main.rs:main}}
```

</Listing>

ابتدا `get_intervals` را فراخوانی می‌کنیم. سپس streamهای `messages` و `intervals` را با استفاده از متد `merge` ترکیب می‌کنیم. این متد چندین stream را به یک stream ترکیب می‌کند که آیتم‌ها را از هر یک از streamهای منبع، به محض در دسترس بودن، تولید می‌کند، بدون اینکه ترتیب خاصی را اعمال کند. در نهایت، به جای اینکه روی `messages` حلقه بزنیم، روی این stream ترکیبی حلقه می‌زنیم.

در این مرحله، نه `messages` و نه `intervals` نیازی به pin یا mutable بودن ندارند، زیرا هر دو در یک stream واحد به نام `merged` ترکیب می‌شوند. با این حال، این فراخوانی به `merge` کامپایل نمی‌شود! (فراخوانی `next` در حلقه `while let` هم کامپایل نمی‌شود، اما به آن برمی‌گردیم.) دلیل آن این است که این دو stream انواع مختلفی دارند. stream `messages` نوع `Timeout<impl Stream<Item = String>>` دارد، جایی که `Timeout` نوعی است که ویژگی `Stream` را برای فراخوانی `timeout` پیاده‌سازی می‌کند. stream `intervals` نوع `impl Stream<Item = u32>` دارد. برای ترکیب این دو stream، باید یکی از آن‌ها را به نوع دیگری تبدیل کنیم. ما stream `intervals` را بازبینی می‌کنیم، زیرا `messages` قبلاً در قالب اصلی مورد نظر ما است و باید خطاهای timeout را مدیریت کند (نگاه کنید به لیست ۱۷-۳۸).

<!-- We cannot directly test this one, because it never stops. -->

<Listing number="17-38" caption="هماهنگ کردن نوع‌های stream `intervals` با نوع stream `messages`" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-38/src/main.rs:main}}
```

</Listing>

ابتدا می‌توانیم از متد کمکی `map` برای تبدیل `intervals` به یک رشته استفاده کنیم. دوم، نیاز داریم که `Timeout` از `messages` را مدیریت کنیم. با این حال، چون واقعاً _نمی‌خواهیم_ تایم‌اوتی برای `intervals` داشته باشیم، می‌توانیم یک تایم‌اوت ایجاد کنیم که طولانی‌تر از مدت‌های دیگر مورد استفاده ما باشد. در اینجا، یک تایم‌اوت ۱۰ ثانیه‌ای با استفاده از `Duration::from_secs(10)` ایجاد می‌کنیم. در نهایت، نیاز داریم که `stream` را متغیر (`mutable`) کنیم تا فراخوانی‌های `next` در حلقه `while let` بتوانند روی stream تکرار کنند و آن را pin کنیم تا این کار ایمن باشد. این ما را _تقریباً_ به جایی که باید برسیم می‌رساند. همه چیز از نظر نوع بررسی می‌شود. اما اگر این کد را اجرا کنید، دو مشکل وجود خواهد داشت. اول، هیچ‌گاه متوقف نمی‌شود! باید با زدن <span class="keystroke">ctrl-c</span> آن را متوقف کنید. دوم، پیام‌های الفبای انگلیسی در میان تمام پیام‌های شمارنده interval دفن خواهند شد:


<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the tasks running differently rather than
changes in the compiler -->

```text
--snip--
Interval: 38
Interval: 39
Interval: 40
Message: 'a'
Interval: 41
Interval: 42
Interval: 43
--snip--
```

لیست ۱۷-۳۹ یک روش برای حل این دو مشکل آخر را نشان می‌دهد.


<Listing number="17-39" caption="استفاده از `throttle` و `take` برای مدیریت streams ترکیب‌شده" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-39/src/main.rs:throttle}}
```

</Listing>

ابتدا از متد `throttle` روی stream `intervals` استفاده می‌کنیم تا این stream باعث غرق شدن stream `messages` نشود. _Throttling_ روشی برای محدود کردن نرخ فراخوانی یک تابع است—یا در این مورد، محدود کردن نرخ poll کردن یک stream. یک بار در هر ۱۰۰ میلی‌ثانیه کافی خواهد بود، زیرا تقریباً به همان اندازه پیام‌های ما می‌رسند.

برای محدود کردن تعداد آیتم‌هایی که از یک stream قبول می‌کنیم، متد `take` را روی stream `merged` اعمال می‌کنیم، زیرا می‌خواهیم خروجی نهایی را محدود کنیم، نه فقط یکی از streamها را.

اکنون وقتی برنامه را اجرا می‌کنیم، پس از دریافت ۲۰ آیتم از stream متوقف می‌شود و intervals باعث غرق شدن messages نمی‌شود. همچنین، ما دیگر `Interval: 100` یا `Interval: 200` و موارد مشابه را نمی‌بینیم، بلکه به جای آن `Interval: 1`، `Interval: 2` و به همین ترتیب دریافت می‌کنیم—حتی اگر یک stream منبع داریم که _می‌تواند_ هر میلی‌ثانیه یک رویداد تولید کند. دلیل این است که فراخوانی `throttle` یک stream جدید تولید می‌کند که stream اصلی را بسته‌بندی می‌کند تا stream اصلی فقط با نرخ throttle و نه با نرخ "ذاتی" خود poll شود. ما یک سری پیام interval غیرقابل پردازش نداریم که انتخاب کرده باشیم آن‌ها را نادیده بگیریم. بلکه، ما هرگز آن پیام‌های interval را در وهله اول تولید نمی‌کنیم! این همان "تنبلی" ذاتی futures در Rust است که دوباره به کار گرفته می‌شود و به ما اجازه می‌دهد ویژگی‌های عملکردی خود را انتخاب کنیم.


<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Interval: 1
Message: 'a'
Interval: 2
Interval: 3
Problem: Elapsed(())
Interval: 4
Message: 'b'
Interval: 5
Message: 'c'
Interval: 6
Interval: 7
Problem: Elapsed(())
Interval: 8
Message: 'd'
Interval: 9
Message: 'e'
Interval: 10
Interval: 11
Problem: Elapsed(())
Interval: 12
```

تنها یک مورد باقی مانده که باید مدیریت کنیم: خطاها! با هر دو stream مبتنی بر کانال، فراخوانی‌های `send` ممکن است در صورتی که طرف دیگر کانال بسته شود، با شکست مواجه شوند—و این به نحوه اجرای runtime برای futures که stream را تشکیل می‌دهند بستگی دارد. تاکنون این احتمال را با فراخوانی `unwrap` نادیده گرفته‌ایم، اما در یک برنامه با رفتار مناسب، باید به‌طور صریح خطا را مدیریت کنیم، حداقل با پایان دادن به حلقه تا دیگر پیام ارسال نکنیم. لیست ۱۷-۴۰ یک استراتژی ساده برای مدیریت خطا را نشان می‌دهد: چاپ مشکل و سپس `break` از حلقه‌ها.


<Listing number="17-40" caption="مدیریت خطاها و خاتمه دادن به حلقه‌ها">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-40/src/main.rs:errors}}
```

</Listing>

همان‌طور که معمول است، روش درست برای مدیریت یک خطای ارسال پیام می‌تواند متفاوت باشد؛ فقط مطمئن شوید که یک استراتژی دارید.

اکنون که مقدار زیادی از کد async را در عمل مشاهده کردیم، بیایید کمی به عقب برگردیم و به جزئیات نحوه کارکرد `Future`، `Stream` و ویژگی‌های کلیدی دیگر که Rust برای اجرای async استفاده می‌کند، بپردازیم.


[17-02-messages]: ch17-02-concurrency-with-async.html#message-passing
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
