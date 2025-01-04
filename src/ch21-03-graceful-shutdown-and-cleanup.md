## خاموشی و پاک‌سازی منظم

کدی که در لیستینگ 21-20 وجود دارد، همان‌طور که انتظار داشتیم، با استفاده از یک مجموعه نخ (thread pool) به درخواست‌ها به صورت غیرهمزمان پاسخ می‌دهد. ما هشدارهایی در مورد فیلدهای `workers`، `id` و `thread` دریافت می‌کنیم که به طور مستقیم از آن‌ها استفاده نمی‌کنیم و به ما یادآوری می‌کنند که هیچ چیزی را پاک‌سازی نمی‌کنیم. وقتی از روش کم‌ظرافت <kbd>ctrl</kbd>-<kbd>c</kbd> برای متوقف کردن نخ اصلی استفاده می‌کنیم، تمام نخ‌های دیگر نیز بلافاصله متوقف می‌شوند، حتی اگر در میانه ارائه یک درخواست باشند.

سپس، ما `Drop` trait را پیاده‌سازی خواهیم کرد تا `join` را روی هر یک از نخ‌های موجود در مجموعه نخ فراخوانی کنیم تا بتوانند درخواست‌هایی که در حال کار روی آن‌ها هستند را قبل از بسته‌شدن تکمیل کنند. سپس روشی برای اطلاع به نخ‌ها که نباید درخواست‌های جدید بپذیرند و باید خاموش شوند، پیاده‌سازی خواهیم کرد. برای مشاهده عملکرد این کد، سرور خود را تغییر می‌دهیم تا فقط دو درخواست را قبل از خاموشی منظم مجموعه نخ‌ها بپذیرد.

چیزی که باید توجه داشته باشید این است که هیچ‌کدام از این موارد بخش‌هایی از کد را که مدیریت اجرای closureها را بر عهده دارند، تحت تأثیر قرار نمی‌دهند، بنابراین همه چیز در اینجا همان‌طور باقی می‌ماند اگر از یک مجموعه نخ برای یک runtime غیرهمزمان استفاده می‌کردیم.

### پیاده‌سازی `Drop` Trait روی `ThreadPool`

بیایید با پیاده‌سازی `Drop` روی مجموعه نخ شروع کنیم. وقتی مجموعه نخ حذف می‌شود، تمام نخ‌های ما باید به یکدیگر ملحق شوند تا مطمئن شویم کار خود را تکمیل می‌کنند. لیستینگ 21-22 اولین تلاش برای پیاده‌سازی `Drop` را نشان می‌دهد؛ این کد هنوز به درستی کار نخواهد کرد.

<Listing number="21-22" file-name="src/lib.rs" caption="ملحق کردن هر نخ وقتی مجموعه نخ از محدوده خارج می‌شود">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

ابتدا، ما از میان هر یک از `workers` موجود در مجموعه نخ حلقه می‌زنیم. ما برای این کار از `&mut` استفاده می‌کنیم زیرا `self` یک ارجاع قابل تغییر است و ما همچنین نیاز داریم که بتوانیم `worker` را تغییر دهیم. برای هر `worker`، پیامی چاپ می‌کنیم که نشان می‌دهد این `worker` خاص در حال خاموش‌شدن است، و سپس `join` را روی نخ آن `worker` فراخوانی می‌کنیم. اگر فراخوانی `join` شکست بخورد، از `unwrap` استفاده می‌کنیم تا باعث panic شود و خاموشی غیرمنظم اتفاق بیفتد.

اینجا خطایی که هنگام کامپایل این کد دریافت می‌کنیم آمده است:

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

The error tells us we can’t call `join` because we only have a mutable borrow of
each `worker` and `join` takes ownership of its argument. To solve this issue,
we need to move the thread out of the `Worker` instance that owns `thread` so
`join` can consume the thread. One way to do this is by taking the same approach
we did in Listing 18-15. If `Worker` held an `Option<thread::JoinHandle<()>>`,
we could call the `take` method on the `Option` to move the value out of the
`Some` variant and leave a `None` variant in its place. In other words, a
`Worker` that is running would have a `Some` variant in `thread`, and when we
wanted to clean up a `Worker`, we would replace `Some` with `None` so the
`Worker` doesn’t have a thread to run.

However, the _only_ time this would come up would be when dropping the `Worker`.
In exchange, we would have to deal with an `Option<thread::JoinHandle<()>>`
everywhere we access `worker.thread`. Idiomatic Rust uses `Option` quite a bit,
but when you find yourself wrapping something in `Option` as a workaround even
though you know the item will always be present, it is a good idea to look for
alternative approaches. They can make your code cleaner and less error-prone.

In this case, there is a better alternative: the `Vec::drain` method. It accepts
a range parameter to specify which items to remove from the `Vec`, and returns
an iterator of those items. Passing the `..` range syntax will remove *every*
value from the `Vec`.

So we need to update the `ThreadPool` `drop` implementation like this:

<Listing file-name="src/lib.rs">

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

این تغییر خطای کامپایلر را برطرف می‌کند و نیازی به تغییرات دیگر در کد ما ندارد.

### Signaling to the Threads to Stop Listening for Jobs

With all the changes we’ve made, our code compiles without any warnings.
However, the bad news is this code doesn’t function the way we want it to yet.
The key is the logic in the closures run by the threads of the `Worker`
instances: at the moment, we call `join`, but that won’t shut down the threads
because they `loop` forever looking for jobs. If we try to drop our
`ThreadPool` with our current implementation of `drop`, the main thread will
block forever waiting for the first thread to finish.

To fix this problem, we’ll need a change in the `ThreadPool` `drop`
implementation and then a change in the `Worker` loop.

First, we’ll change the `ThreadPool` `drop` implementation to explicitly drop
the `sender` before waiting for the threads to finish. Listing 21-23 shows the
changes to `ThreadPool` to explicitly drop `sender`. Unlike with the `workers`,
here we *do* need to use an `Option` to be able to move `sender` out of
`ThreadPool` with `Option::take`.

<Listing number="21-23" file-name="src/lib.rs" caption="حذف صریح `sender` قبل از ملحق کردن نخ‌های worker">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

حذف `sender` کانال را می‌بندد، که نشان می‌دهد دیگر هیچ پیامی ارسال نخواهد شد. وقتی این اتفاق می‌افتد، تمام فراخوانی‌های `recv` که workers در حلقه بی‌نهایت انجام می‌دهند یک خطا برمی‌گرداند. در لیستینگ 21-24، حلقه `Worker` را تغییر می‌دهیم تا در چنین حالتی به صورت منظم از حلقه خارج شود، که به این معناست که نخ‌ها وقتی پیاده‌سازی `drop` در `ThreadPool` روی آن‌ها `join` را فراخوانی می‌کند تکمیل خواهند شد.

<Listing number="21-24" file-name="src/lib.rs" caption="خروج صریح از حلقه وقتی `recv` یک خطا برمی‌گرداند">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

برای دیدن این کد در عمل، بیایید `main` را تغییر دهیم تا فقط دو درخواست را قبل از خاموش‌شدن منظم سرور بپذیرد، همان‌طور که در لیستینگ 21-25 نشان داده شده است.

<Listing number="21-25" file-name="src/main.rs" caption="خاموش‌کردن سرور پس از ارائه دو درخواست با خروج از حلقه">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

شما نمی‌خواهید یک سرور وب واقعی پس از فقط دو درخواست خاموش شود. این کد فقط نشان می‌دهد که خاموشی منظم و پاک‌سازی به درستی کار می‌کند.

متد `take` که در trait `Iterator` تعریف شده است، تکرار را به حداکثر دو آیتم محدود می‌کند. `ThreadPool` در انتهای `main` از محدوده خارج می‌شود و پیاده‌سازی `drop` اجرا خواهد شد.

سرور را با دستور `cargo run` راه‌اندازی کنید و سه درخواست ارسال کنید. درخواست سوم باید با خطا مواجه شود و در ترمینال خود باید خروجی مشابه زیر را ببینید:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

ممکن است ترتیب متفاوتی از کارگران و پیام‌های چاپ‌شده را مشاهده کنید. از پیام‌ها می‌توان فهمید که این کد چگونه کار می‌کند: کارگران 0 و 3 اولین دو درخواست را دریافت کردند. سرور پس از اتصال دوم دیگر اتصال‌ها را نمی‌پذیرد و پیاده‌سازی `Drop` روی `ThreadPool` شروع به اجرا می‌کند قبل از اینکه کارگر 3 حتی کار خود را شروع کند. حذف `sender` تمام کارگران را قطع کرده و به آن‌ها می‌گوید که خاموش شوند. هر کارگر هنگام قطع شدن یک پیام چاپ می‌کند و سپس مجموعه نخ (thread pool) `join` را فراخوانی می‌کند تا منتظر تکمیل هر نخ کارگر بماند.

به یک جنبه جالب از این اجرای خاص توجه کنید: `ThreadPool` فرستنده را حذف کرد، و قبل از اینکه هر کارگری خطایی دریافت کند، ما سعی کردیم به کارگر 0 ملحق شویم. کارگر 0 هنوز از `recv` خطایی دریافت نکرده بود، بنابراین نخ اصلی منتظر ماند تا کارگر 0 کار خود را به پایان برساند. در همین حال، کارگر 3 یک کار دریافت کرد و سپس تمام نخ‌ها خطا دریافت کردند. وقتی کارگر 0 تمام شد، نخ اصلی منتظر ماند تا بقیه کارگران کار خود را تمام کنند. در آن زمان، همه آن‌ها از حلقه‌های خود خارج شده و متوقف شده بودند.

تبریک می‌گویم! پروژه خود را کامل کردید؛ ما یک سرور وب ساده داریم که از یک مجموعه نخ برای پاسخ‌دهی غیرهمزمان استفاده می‌کند. ما توانستیم سرور را به صورت منظم خاموش کنیم و تمام نخ‌ها در مجموعه را پاک‌سازی کنیم.

در اینجا کد کامل برای مرجع آورده شده است:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

می‌توانستیم بیشتر اینجا انجام دهیم! اگر می‌خواهید این پروژه را بیشتر گسترش دهید، اینجا چند ایده آمده است:

- مستندات بیشتری به `ThreadPool` و متدهای عمومی آن اضافه کنید.
- تست‌هایی برای عملکرد کتابخانه اضافه کنید.
- فراخوانی‌های `unwrap` را به مدیریت خطای قوی‌تر تغییر دهید.
- از `ThreadPool` برای انجام برخی کارها به غیر از ارائه درخواست‌های وب استفاده کنید.
- یک crate مجموعه نخ از [crates.io](https://crates.io/) پیدا کنید و یک سرور وب مشابه با استفاده از آن crate پیاده‌سازی کنید. سپس API و مقاومت آن را با مجموعه نخی که ما پیاده‌سازی کردیم مقایسه کنید.

## خلاصه

آفرین! شما به انتهای این کتاب رسیدید! از شما بابت پیوستن به ما در این سفر با Rust سپاسگزاریم. اکنون آماده‌اید که پروژه‌های Rust خود را پیاده‌سازی کنید و به پروژه‌های دیگران کمک کنید. به یاد داشته باشید که جامعه‌ای خوش‌آمدگوی از Rustaceans وجود دارد که مشتاقانه منتظر کمک به شما در هر چالشی هستند که در مسیر Rust خود با آن مواجه می‌شوید.
