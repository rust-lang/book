## Refutability: Whether a Pattern Might Fail to Match

الگوها به دو شکل هستند: قابل‌رد (refutable) و غیرقابل‌رد (irrefutable). الگوهایی که برای هر مقدار ممکن مطابقت دارند _غیرقابل‌رد_ هستند. به‌عنوان مثال، `x` در عبارت `let x = 5;`، زیرا `x` با هر چیزی مطابقت دارد و بنابراین نمی‌تواند از تطابق باز بماند. الگوهایی که ممکن است برای برخی مقادیر ممکن مطابقت نداشته باشند _قابل‌رد_ هستند. به‌عنوان مثال، `Some(x)` در عبارت `if let Some(x) = a_value`، زیرا اگر مقدار در متغیر `a_value` `None` باشد به‌جای `Some`، الگوی `Some(x)` مطابقت نخواهد داشت.

پارامترهای تابع، عبارات `let`، و حلقه‌های `for` فقط می‌توانند الگوهای غیرقابل‌رد بپذیرند، زیرا برنامه نمی‌تواند کاری معنادار انجام دهد وقتی مقادیر مطابقت ندارند. عبارات `if let` و `while let` و عبارت `let`-`else` الگوهای قابل‌رد و غیرقابل‌رد را می‌پذیرند، اما کامپایلر درباره الگوهای غیرقابل‌رد هشدار می‌دهد زیرا به‌طور تعریف‌شده برای مدیریت شکست احتمالی طراحی شده‌اند: عملکرد شرطی در توانایی آن است که بسته به موفقیت یا شکست به‌طور متفاوت عمل کند.

به‌طور کلی، نباید نیازی به نگرانی در مورد تمایز بین الگوهای قابل‌رد و غیرقابل‌رد داشته باشید؛ با این حال، باید با مفهوم قابل‌رد بودن آشنا باشید تا بتوانید زمانی که آن را در یک پیام خطا می‌بینید، واکنش نشان دهید. در این موارد، باید یا الگو را تغییر دهید یا ساختاری که الگو را با آن استفاده می‌کنید، بسته به رفتار موردنظر کد تغییر دهید.

بیایید به مثالی نگاه کنیم که وقتی سعی می‌کنیم از یک الگوی قابل‌رد جایی که راست نیاز به یک الگوی غیرقابل‌رد دارد استفاده کنیم، و برعکس، چه اتفاقی می‌افتد. فهرست 19-8 یک عبارت `let` را نشان می‌دهد، اما برای الگو ما `Some(x)`، یک الگوی قابل‌رد مشخص کرده‌ایم. همان‌طور که ممکن است انتظار داشته باشید، این کد کامپایل نخواهد شد.

<Listing number="19-8" caption="تلاش برای استفاده از یک الگوی قابل‌رد با `let`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

اگر مقدار `some_option_value` `None` باشد، مطابقت با الگوی `Some(x)` شکست خواهد خورد، به این معنا که الگو قابل‌رد است. با این حال، عبارت `let` فقط می‌تواند یک الگوی غیرقابل‌رد بپذیرد زیرا چیزی معتبر وجود ندارد که کد بتواند با مقدار `None` انجام دهد. در زمان کامپایل، راست شکایت می‌کند که ما سعی کرده‌ایم از یک الگوی قابل‌رد جایی که یک الگوی غیرقابل‌رد نیاز است استفاده کنیم:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

زیرا ما هر مقدار معتبری را با الگوی `Some(x)` پوشش ندادیم (و نمی‌توانستیم پوشش دهیم!)، راست به‌درستی یک خطای کامپایلر تولید می‌کند.

اگر یک الگوی قابل‌رد داشته باشیم جایی که یک الگوی غیرقابل‌رد نیاز است، می‌توانیم با تغییر کدی که از الگو استفاده می‌کند آن را رفع کنیم: به‌جای استفاده از `let`، می‌توانیم از `if let` استفاده کنیم. سپس اگر الگو مطابقت نداشته باشد، کد به‌سادگی از اجرای کد داخل آکولادها صرف‌نظر می‌کند و راهی برای ادامه معتبر فراهم می‌کند. فهرست 19-9 نشان می‌دهد که چگونه کد در فهرست 19-8 را رفع کنیم.

<Listing number="19-9" caption="استفاده از `if let` و یک بلوک با الگوهای قابل‌رد به‌جای `let`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

We’ve given the code an out! This code is perfectly valid now. However,
if we give `if let` an irrefutable pattern (a pattern that will always
match), such as `x`, as shown in Listing 19-10, the compiler will give a
warning.

<Listing number="19-10" caption="Attempting to use an irrefutable pattern with `if let`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

Rust complains that it doesn’t make sense to use `if let` with an irrefutable
pattern:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

For this reason, match arms must use refutable patterns, except for the last
arm, which should match any remaining values with an irrefutable pattern. Rust
allows us to use an irrefutable pattern in a `match` with only one arm, but
this syntax isn’t particularly useful and could be replaced with a simpler
`let` statement.

Now that you know where to use patterns and the difference between refutable
and irrefutable patterns, let’s cover all the syntax we can use to create
patterns.
