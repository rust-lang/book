## Workspaces در Cargo

در فصل 12، ما یک پکیج ساختیم که شامل یک crate باینری و یک crate کتابخانه‌ای بود. همان‌طور که پروژه شما توسعه می‌یابد، ممکن است متوجه شوید که crate کتابخانه‌ای همچنان بزرگ‌تر می‌شود و بخواهید پکیج خود را بیشتر به crate‌های کتابخانه‌ای چندگانه تقسیم کنید. Cargo یک ویژگی به نام _workspaces_ ارائه می‌دهد که می‌تواند به مدیریت پکیج‌های مرتبط که به صورت همزمان توسعه داده می‌شوند کمک کند.

### ایجاد یک Workspace

یک _workspace_ مجموعه‌ای از پکیج‌ها است که یک فایل _Cargo.lock_ و دایرکتوری خروجی مشترک دارند. بیایید یک پروژه با استفاده از workspace ایجاد کنیم—ما از کد ساده‌ای استفاده خواهیم کرد تا بتوانیم بر ساختار workspace تمرکز کنیم. راه‌های متعددی برای ساختن یک workspace وجود دارد، بنابراین فقط یک روش رایج را نشان خواهیم داد. ما یک workspace شامل یک باینری و دو کتابخانه خواهیم داشت. باینری که عملکرد اصلی را فراهم خواهد کرد، به دو کتابخانه وابسته خواهد بود. یک کتابخانه تابع `add_one` و کتابخانه دیگر تابع `add_two` ارائه خواهد داد. این سه crate بخشی از یک workspace خواهند بود. ابتدا با ایجاد یک دایرکتوری جدید برای workspace شروع می‌کنیم:

```console
$ mkdir add
$ cd add
```

سپس، در دایرکتوری _add_، فایل _Cargo.toml_ را ایجاد می‌کنیم که کل workspace را پیکربندی می‌کند. این فایل بخش `[package]` نخواهد داشت. در عوض، با یک بخش `[workspace]` شروع می‌شود که به ما اجازه می‌دهد اعضا را به workspace اضافه کنیم. همچنین نسخه جدیدتر الگوریتم resolver Cargo را با تنظیم `resolver` به `"2"` استفاده می‌کنیم.

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

سپس، crate باینری `adder` را با اجرای `cargo new` در دایرکتوری _add_ ایجاد می‌کنیم:

```console
$ cargo new adder
    Creating binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

اجرای `cargo new` داخل یک workspace به صورت خودکار پکیج تازه ایجاد شده را به کلید `members` در تعریف `[workspace]` در فایل `Cargo.toml` workspace اضافه می‌کند، به این صورت:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

در این مرحله، می‌توانیم workspace را با اجرای دستور `cargo build` بسازیم. فایل‌های موجود در دایرکتوری _add_ شما باید به این صورت باشند:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Workspace یک دایرکتوری _target_ در سطح بالا دارد که فایل‌های کامپایل‌شده در آن قرار خواهند گرفت. پکیج `adder` دایرکتوری _target_ اختصاصی خود را ندارد. حتی اگر دستور `cargo build` را از داخل دایرکتوری _adder_ اجرا کنیم، فایل‌های کامپایل‌شده همچنان در _add/target_ قرار می‌گیرند نه در _add/adder/target_. Cargo دایرکتوری _target_ را در یک workspace به این صورت ساختاردهی می‌کند زیرا crate‌های موجود در یک workspace برای وابستگی به یکدیگر طراحی شده‌اند. اگر هر crate دایرکتوری _target_ اختصاصی خود را داشت، هر crate مجبور بود هر کدام از crate‌های دیگر را در workspace دوباره کامپایل کند تا فایل‌های کامپایل‌شده را در دایرکتوری _target_ خود قرار دهد. با به اشتراک‌گذاری یک دایرکتوری _target_، crate‌ها می‌توانند از ساخت مجدد غیرضروری جلوگیری کنند.

### ایجاد پکیج دوم در Workspace

حالا، بیایید یک پکیج عضو دیگر در workspace ایجاد کنیم و آن را `add_one` بنامیم. فایل _Cargo.toml_ در سطح بالا را تغییر دهید تا مسیر _add_one_ را در لیست `members` مشخص کنید:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

سپس یک crate کتابخانه‌ای جدید به نام `add_one` ایجاد کنید:

```console
$ cargo new add_one --lib
    Creating library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

دایرکتوری _add_ شما اکنون باید شامل این دایرکتوری‌ها و فایل‌ها باشد:

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

در فایل _add_one/src/lib.rs_، تابعی به نام `add_one` اضافه کنیم:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

حالا می‌توانیم پکیج `adder` که حاوی باینری ما است را وابسته به پکیج `add_one` که حاوی کتابخانه ما است کنیم. ابتدا باید یک وابستگی مسیر (path dependency) به `add_one` در فایل _adder/Cargo.toml_ اضافه کنیم.

<span class="filename">Filename: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo فرض نمی‌کند که crate‌های موجود در یک workspace به یکدیگر وابسته هستند، بنابراین ما باید به صراحت روابط وابستگی را مشخص کنیم.

در ادامه، بیایید از تابع `add_one` (از crate به نام `add_one`) در crate به نام `adder` استفاده کنیم. فایل _adder/src/main.rs_ را باز کنید و تابع `main` را تغییر دهید تا تابع `add_one` را فراخوانی کند، همان‌طور که در لیست ۱۴-۷ نشان داده شده است.

<Listing number="14-7" file-name="adder/src/main.rs" caption="Using the `add_one` library crate in the `adder` crate">


```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

</Listing>

بیایید workspace را با اجرای دستور `cargo build` در دایرکتوری سطح بالای _add_ بسازیم!

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

برای اجرای crate باینری از دایرکتوری _add_، می‌توانیم با استفاده از آرگومان `-p` و نام پکیج همراه با دستور `cargo run` مشخص کنیم کدام پکیج در workspace اجرا شود:

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

این کد در فایل _adder/src/main.rs_ را اجرا می‌کند که به crate `add_one` وابسته است.

#### وابستگی به یک پکیج خارجی در یک Workspace

توجه کنید که workspace فقط یک فایل _Cargo.lock_ در سطح بالا دارد، به جای اینکه هر crate دایرکتوری خود فایل _Cargo.lock_ داشته باشد. این اطمینان حاصل می‌کند که تمام crate‌ها از همان نسخه تمام وابستگی‌ها استفاده می‌کنند. اگر پکیج `rand` را به فایل‌های _adder/Cargo.toml_ و _add_one/Cargo.toml_ اضافه کنیم، Cargo هر دو را به یک نسخه از `rand` تبدیل می‌کند و آن را در فایل _Cargo.lock_ ثبت می‌کند. اطمینان از اینکه همه crate‌های موجود در workspace از همان وابستگی‌ها استفاده می‌کنند، به این معناست که crate‌ها همیشه با یکدیگر سازگار خواهند بود. بیایید پکیج `rand` را به بخش `[dependencies]` در فایل _add_one/Cargo.toml_ اضافه کنیم تا بتوانیم از crate `rand` در crate `add_one` استفاده کنیم:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Filename: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

حالا می‌توانیم `use rand;` را به فایل _add_one/src/lib.rs_ اضافه کنیم و با اجرای دستور `cargo build` در دایرکتوری _add_ کل workspace را بسازیم، که crate `rand` را وارد کرده و کامپایل خواهد کرد. یک هشدار دریافت خواهیم کرد زیرا به `rand` که به محدوده وارد شده است اشاره‌ای نمی‌کنیم:

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

فایل _Cargo.lock_ در سطح بالا اکنون اطلاعاتی درباره وابستگی `add_one` به `rand` دارد. با این حال، حتی اگر `rand` در جایی از workspace استفاده شود، نمی‌توانیم از آن در crate‌های دیگر workspace استفاده کنیم مگر اینکه `rand` را به فایل‌های _Cargo.toml_ آن‌ها نیز اضافه کنیم. برای مثال، اگر `use rand;` را به فایل _adder/src/main.rs_ برای پکیج `adder` اضافه کنیم، با خطا مواجه خواهیم شد:

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

برای رفع این مشکل، فایل _Cargo.toml_ پکیج `adder` را ویرایش کرده و مشخص کنید که `rand` برای آن نیز یک وابستگی است. ساختن پکیج `adder`، `rand` را به لیست وابستگی‌های `adder` در فایل _Cargo.lock_ اضافه می‌کند، اما هیچ نسخه اضافی از `rand` دانلود نخواهد شد. Cargo اطمینان حاصل می‌کند که هر crate در هر پکیجی از workspace که از پکیج `rand` استفاده می‌کند، از همان نسخه استفاده کند، به شرطی که نسخه‌های سازگار از `rand` را مشخص کنند. این کار فضای ما را ذخیره کرده و تضمین می‌کند که crate‌های workspace با یکدیگر سازگار خواهند بود.

اگر crate‌های workspace نسخه‌های ناسازگار از یک وابستگی را مشخص کنند، Cargo هر یک از آن‌ها را جداگانه حل خواهد کرد، اما همچنان تلاش می‌کند که تعداد نسخه‌های حل‌شده را به حداقل برساند.

#### افزودن یک تست به یک Workspace

برای یک بهبود دیگر، بیایید یک تست برای تابع `add_one::add_one` در crate `add_one` اضافه کنیم:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

حالا دستور `cargo test` را در دایرکتوری سطح بالای _add_ اجرا کنید. اجرای دستور `cargo test` در یک workspace با ساختاری مانند این، تست‌های تمام crate‌های موجود در workspace را اجرا خواهد کرد:

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-f0253159197f7841)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

بخش اول خروجی نشان می‌دهد که تست `it_works` در crate `add_one` پاس شده است. بخش بعدی نشان می‌دهد که هیچ تستی در crate `adder` پیدا نشده است، و سپس بخش آخر نشان می‌دهد که هیچ تست مستنداتی در crate `add_one` پیدا نشده است.

ما همچنین می‌توانیم تست‌های یک crate خاص در workspace را از دایرکتوری سطح بالا با استفاده از گزینه `-p` و مشخص کردن نام crate‌ای که می‌خواهیم تست کنیم، اجرا کنیم:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
این خروجی نشان می‌دهد که `cargo test` فقط تست‌های crate `add_one` را اجرا کرده و تست‌های crate `adder` را اجرا نکرده است.

اگر crate‌های موجود در workspace را در [crates.io](https://crates.io/) منتشر کنید، هر crate در workspace باید به صورت جداگانه منتشر شود. مشابه با `cargo test`، می‌توانیم یک crate خاص را در workspace خود با استفاده از گزینه `-p` و مشخص کردن نام crate‌ای که می‌خواهیم منتشر کنیم، منتشر کنیم.

برای تمرین بیشتر، یک crate جدید به نام `add_two` به این workspace اضافه کنید، به شیوه‌ای مشابه crate `add_one`!

همان‌طور که پروژه شما رشد می‌کند، استفاده از یک workspace را در نظر بگیرید: فهمیدن اجزای کوچک‌تر و جداگانه آسان‌تر از کار کردن با یک کد بزرگ و یکپارچه است. علاوه بر این، نگه داشتن crate‌ها در یک workspace می‌تواند هماهنگی بین آن‌ها را آسان‌تر کند، به خصوص اگر crate‌ها اغلب به طور همزمان تغییر کنند.
