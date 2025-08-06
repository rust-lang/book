
# زبان برنامه‌نویسی راست

<div align="right">
  
![وضعیت ساخت](https://github.com/rust-lang/book/workflows/CI/badge.svg)
</div>

<div align="center">
  
[English](README-EN.md) | [فارسی](README.md)
  
</div>

<div dir="rtl">

**لطفاً توجه داشته باشید که این ترجمه هنوز کامل نشده است و نیاز به بازبینی و ویرایش دقیق دارد که به زودی تکمیل خواهد شد. شما می‌توانید ترجمه‌های در حال انجام را در شاخه [`persian-draft`](https://github.com/drunkleen/rust-book-persian/tree/persian-draft) مخزن پیدا کنید.**




این مخزن شامل منبع کتاب "زبان برنامه‌نویسی راست" به زبان فارسی است.

[کتاب انگلیسی به صورت نسخه چاپی از No Starch Press در دسترس است][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

همچنین می‌توانید کتاب را به صورت رایگان آنلاین بخوانید. لطفاً کتاب را به صورت ارسال شده با آخرین نسخه‌های [stable]، [beta]، یا [nightly] راست مشاهده کنید. توجه داشته باشید که مشکلات در آن نسخه‌ها ممکن است در این مخزن قبلاً رفع شده باشد، زیرا آن نسخه‌ها به صورت مکرر به‌روز نمی‌شوند.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

فقط برای دانلود تمام لیست‌های کد که در کتاب ظاهر می‌شوند، به [releases] مراجعه کنید.

[releases]: https://github.com/drunkleen/rust-book-persian/releases

## نیازمندی‌ها

برای ساخت کتاب نیاز به [mdBook] دارید، ترجیحاً همان نسخه‌ای که rust-lang/rust در [این فایل][rust-mdbook] استفاده می‌کند. برای دریافت آن:

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
cargo install mdbook --locked --version <version_num>
```
Example:

```bash
$ cargo install --locked --path packages/mdbook-trpl --force
```

این کتاب همچنین از دو افزونه mdbook استفاده می‌کند که بخشی از این مخزن هستند. اگر آنها را نصب نکنید، هنگام ساخت هشدارهایی مشاهده خواهید کرد و خروجی به درستی نمایش داده نمی‌شود، اما هنوز هم می‌توانید کتاب را بسازید. برای استفاده از افزونه‌ها، باید دستور زیر را اجرا کنید:

```bash
cargo install --locked --path packages/mdbook-trpl
```

## ساخت

برای ساختن کتاب عبارات زیر را تایپ کنید:

```bash
$ mdbook build
```


خروجی در زیرمجموعه `book` خواهد بود. برای مشاهده، آن را در مرورگر وب خود باز کنید.

_Firefox:_

```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_

```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

برای اجرای تست‌ها:

```bash
$ mdbook test --library-path packages/trpl/target/debug/deps
```
## مشارکت

ما از کمک شما خوشحال می‌شویم! لطفاً [CONTRIBUTING.md][contrib] را ببینید تا با انواع مشارکت‌هایی که به دنبال آنها هستیم آشنا شوید.

[contrib]: https://github.com/drunkleen/rust-book-persian/blob/main/CONTRIBUTING.md

به دلیل اینکه کتاب [چاپ شده است][nostarch] و همچنین می‌خواهیم نسخه آنلاین کتاب را تا حد ممکن نزدیک به نسخه چاپی نگه داریم، ممکن است برای ما بیشتر از حد معمول طول بکشد تا به مشکل یا درخواست کشش شما رسیدگی کنیم.

## بررسی املاء

برای اسکن فایل‌های منبع برای خطاهای املایی، می‌توانید از اسکریپت `spellcheck.sh` که در پوشه `ci` موجود است استفاده کنید. این اسکریپت به یک فرهنگ لغت از کلمات معتبر نیاز دارد که در `ci/dictionary.txt` ارائه شده است. اگر اسکریپت یک مثبت کاذب تولید کرد (مثلاً شما از کلمه `BTreeMap` استفاده کرده‌اید که اسکریپت آن را نامعتبر می‌داند)، باید این کلمه را به `ci/dictionary.txt` اضافه کنید (ترتیب مرتب‌شده را برای سازگاری حفظ کنید).

</div>