## ضمیمه د - ابزارهای مفید توسعه

در این ضمیمه، ما درباره برخی ابزارهای مفید توسعه که پروژه Rust ارائه می‌دهد صحبت می‌کنیم. به فرمت‌دهی خودکار، روش‌های سریع برای اعمال اصلاحات هشدارها، یک تحلیلگر کد (linter) و یکپارچه‌سازی با محیط‌های توسعه یکپارچه (IDE) خواهیم پرداخت.

### فرمت‌دهی خودکار با `rustfmt`

نصب‌های Rust به‌صورت پیش‌فرض شامل `rustfmt`
هستند، بنابراین احتمالاً هم‌اکنون برنامه‌های
`rustfmt` و `cargo-fmt` روی سیستم شما
نصب شده‌اند. این دو دستور همانند `rustc` و `cargo`
هستند؛ به این صورت که `rustfmt` کنترل دقیق‌تری
ارائه می‌دهد و `cargo-fmt` با ساختار و قراردادهای
پروژه‌های مبتنی بر Cargo آشنایی دارد. برای قالب‌بندی
هر پروژه‌ی Cargo، دستور زیر را وارد کنید:

```console
$ cargo fmt
```

اجرای این دستور تمام کدهای Rust در crate فعلی را مجدداً فرمت می‌کند. این کار باید فقط سبک کدنویسی را تغییر دهد، نه معنای کد را. برای اطلاعات بیشتر در مورد `rustfmt`، به [مستندات آن][rustfmt] مراجعه کنید.

[rustfmt]: https://github.com/rust-lang/rustfmt

### اصلاح کد شما با `rustfix`

ابزار `rustfix` همراه با نصب Rust ارائه می‌شود و
می‌تواند هشدارهای کامپایلر را به‌صورت خودکار
برطرف کند، آن هم در مواردی که راه‌حل مشخصی
برای رفع مشکل وجود دارد و احتمالاً همان چیزی است
که شما انتظار دارید. احتمالاً پیش از این با
هشدارهای کامپایلر روبه‌رو شده‌اید. برای مثال،
به کد زیر توجه کنید:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

در اینجا، متغیر `x` را به‌صورت قابل‌تغییر (mutable)  
تعریف کرده‌ایم، اما در عمل هیچ‌گاه آن را تغییر نمی‌دهیم.  
Rust در این مورد به ما هشدار می‌دهد:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

هشدار پیشنهاد می‌دهد که کلمه‌ی کلیدی `mut`
را حذف کنیم. می‌توانیم این پیشنهاد را به‌صورت
خودکار با استفاده از ابزار `rustfix` و اجرای دستور
`cargo fix` اعمال کنیم:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

وقتی دوباره فایل _src/main.rs_ را بررسی کنیم، خواهیم دید که `cargo fix` کد را تغییر داده است:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

متغیر `x` اکنون غیرقابل‌تغییر (immutable) شده است
و هشدار نیز دیگر نمایش داده نمی‌شود.

همچنین می‌توانید از دستور `cargo fix` برای انتقال کد خود بین نسخه‌های مختلف Rust استفاده کنید. نسخه‌ها در [ضمیمه ه][editions] پوشش داده شده‌اند.

### لینت‌های بیشتر با Clippy

ابزار Clippy مجموعه‌ای از lintها برای تحلیل کد شماست
تا بتوانید خطاهای رایج را شناسایی کرده و کد Rust خود
را بهبود دهید. Clippy همراه با نصب استاندارد Rust
در دسترس است.

برای اجرای تحلیلگرهای Clippy روی هر پروژه Cargo، دستور زیر را وارد کنید:

```console
$ cargo clippy
```

به عنوان مثال، فرض کنید برنامه‌ای می‌نویسید که از یک مقدار تقریبی برای یک ثابت ریاضی، مانند pi، استفاده می‌کند، همانطور که این برنامه انجام می‌دهد:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

اجرای `cargo clippy` روی این پروژه منجر به این خطا می‌شود:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

این خطا به شما اطلاع می‌دهد که Rust از پیش
ثابت `PI` دقیق‌تری را تعریف کرده است، و استفاده از
این ثابت در برنامه‌تان باعث درستی بیشتر کد می‌شود.
بنابراین باید کد خود را طوری تغییر دهید که از
ثابت `PI` استفاده کند.

کد زیر هیچ خطا یا هشداری از Clippy ایجاد نمی‌کند:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

برای اطلاعات بیشتر درباره Clippy، به [مستندات آن][clippy] مراجعه کنید.

### یکپارچه‌سازی IDE با استفاده از `rust-analyzer`

برای یکپارچه‌سازی بهتر با محیط‌های توسعه (IDE)،
جامعه‌ی Rust استفاده از [`rust-analyzer`][rust-analyzer]
را توصیه می‌کند. این ابزار مجموعه‌ای از ابزارهای
وابسته به کامپایلر است که با [پروتکل زبان سرور (LSP)][lsp]
ارتباط برقرار می‌کند؛ این پروتکل مشخصاتی است برای
ارتباط میان IDEها و زبان‌های برنامه‌نویسی.
کلاینت‌های مختلفی می‌توانند از `rust-analyzer`
استفاده کنند، مانند [افزونه‌ی Rust Analyzer برای
Visual Studio Code][vscode].

برای دریافت دستورالعمل نصب، به [صفحه‌ی اصلی
پروژه‌ی `rust-analyzer`][rust-analyzer] مراجعه کنید،
سپس پشتیبانی از language server را در IDE خود نصب نمایید.
پس از آن، امکاناتی مانند تکمیل خودکار، پرش به تعریف،
و نمایش خطاها به‌صورت درون‌خطی به IDE شما اضافه خواهد شد.

[rustfmt]: https://github.com/rust-lang/rustfmt
[editions]: appendix-05-editions.md
[clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
