## ضمیمه د - ابزارهای مفید توسعه

در این ضمیمه، ما درباره برخی ابزارهای مفید توسعه که پروژه Rust ارائه می‌دهد صحبت می‌کنیم. به فرمت‌دهی خودکار، روش‌های سریع برای اعمال اصلاحات هشدارها، یک تحلیلگر کد (linter) و یکپارچه‌سازی با محیط‌های توسعه یکپارچه (IDE) خواهیم پرداخت.

### فرمت‌دهی خودکار با `rustfmt`

ابزار `rustfmt` کد شما را بر اساس سبک کدنویسی جامعه فرمت می‌کند. بسیاری از پروژه‌های مشترک از `rustfmt` استفاده می‌کنند تا از بحث در مورد سبک کدنویسی در هنگام نوشتن کدهای Rust جلوگیری شود: همه کدهای خود را با استفاده از این ابزار فرمت می‌کنند.

برای نصب `rustfmt`، دستور زیر را وارد کنید:

```console
$ rustup component add rustfmt
```

این دستور ابزارهای `rustfmt` و `cargo-fmt` را به شما می‌دهد، مشابه به اینکه Rust ابزارهای `rustc` و `cargo` را ارائه می‌دهد. برای فرمت کردن هر پروژه‌ای که از Cargo استفاده می‌کند، دستور زیر را وارد کنید:

```console
$ cargo fmt
```

اجرای این دستور تمام کدهای Rust در crate فعلی را مجدداً فرمت می‌کند. این کار باید فقط سبک کدنویسی را تغییر دهد، نه معنای کد را. برای اطلاعات بیشتر در مورد `rustfmt`، به [مستندات آن][rustfmt] مراجعه کنید.

[rustfmt]: https://github.com/rust-lang/rustfmt

### اصلاح کد شما با `rustfix`

ابزار `rustfix` که همراه با نصب‌های Rust ارائه می‌شود، می‌تواند به طور خودکار هشدارهای کامپایلر را که یک راه حل واضح برای رفع مشکل دارند و احتمالاً همان چیزی است که می‌خواهید، اصلاح کند. احتمالاً قبلاً هشدارهای کامپایلر را دیده‌اید. به عنوان مثال، کد زیر را در نظر بگیرید:

<span class="filename">Filename: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

در اینجا، ما تابع `do_something` را 100 بار فراخوانی می‌کنیم، اما هرگز متغیر `i` را در بدنه حلقه `for` استفاده نمی‌کنیم. Rust در این مورد به ما هشدار می‌دهد:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 0..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

هشدار پیشنهاد می‌دهد که به جای آن از نام `_i` استفاده کنیم: خط زیرنویس نشان می‌دهد که قصد داریم این متغیر استفاده نشده باقی بماند. ما می‌توانیم به طور خودکار این پیشنهاد را با استفاده از ابزار `rustfix` و اجرای دستور `cargo fix` اعمال کنیم:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

وقتی دوباره فایل _src/main.rs_ را بررسی کنیم، خواهیم دید که `cargo fix` کد را تغییر داده است:

<span class="filename">Filename: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

اکنون متغیر حلقه `for` به نام `_i` تغییر یافته است و هشدار دیگر نمایش داده نمی‌شود.

همچنین می‌توانید از دستور `cargo fix` برای انتقال کد خود بین نسخه‌های مختلف Rust استفاده کنید. نسخه‌ها در [ضمیمه ه][editions] پوشش داده شده‌اند.

### لینت‌های بیشتر با Clippy

ابزار Clippy مجموعه‌ای از تحلیلگرهای کد (لینت‌ها) است که کد شما را تحلیل می‌کنند تا بتوانید اشتباهات رایج را پیدا کرده و کد Rust خود را بهبود دهید.

برای نصب Clippy، دستور زیر را وارد کنید:

```console
$ rustup component add clippy
```

برای اجرای تحلیلگرهای Clippy روی هر پروژه Cargo، دستور زیر را وارد کنید:

```console
$ cargo clippy
```

به عنوان مثال، فرض کنید برنامه‌ای می‌نویسید که از یک مقدار تقریبی برای یک ثابت ریاضی، مانند pi، استفاده می‌کند، همانطور که این برنامه انجام می‌دهد:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

اجرای `cargo clippy` روی این پروژه به این خطا منجر می‌شود:

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

این خطا به شما اطلاع می‌دهد که Rust از قبل یک ثابت دقیق‌تر `PI` تعریف کرده است و برنامه شما اگر از این ثابت استفاده کند، صحیح‌تر خواهد بود. سپس کد خود را تغییر می‌دهید تا از ثابت `PI` استفاده کنید. کد زیر هیچ خطا یا هشداری از Clippy تولید نمی‌کند:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

برای اطلاعات بیشتر درباره Clippy، به [مستندات آن][clippy] مراجعه کنید.

[clippy]: https://github.com/rust-lang/rust-clippy

### یکپارچه‌سازی IDE با استفاده از `rust-analyzer`

برای کمک به یکپارچه‌سازی با IDE، جامعه Rust استفاده از [`rust-analyzer`][rust-analyzer]<!-- ignore --> را توصیه می‌کند. این ابزار مجموعه‌ای از ابزارهای متمرکز بر کامپایلر است که با [پروتکل زبان سرور][lsp]<!-- ignore --> کار می‌کند، که یک مشخصه برای ارتباط IDEها و زبان‌های برنامه‌نویسی با یکدیگر است. مشتری‌های مختلف می‌توانند از `rust-analyzer` استفاده کنند، مانند [پلاگین Rust analyzer برای Visual Studio Code][vscode].

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

برای دستورالعمل‌های نصب، به [صفحه اصلی پروژه `rust-analyzer`][rust-analyzer]<!-- ignore --> مراجعه کنید، سپس پشتیبانی از سرور زبان را در IDE خاص خود نصب کنید. IDE شما قابلیت‌هایی مانند تکمیل خودکار، پرش به تعریف و نمایش خطاها به صورت درون‌خطی را به دست خواهد آورد.

[rust-analyzer]: https://rust-analyzer.github.io
[editions]: appendix-05-editions.md
