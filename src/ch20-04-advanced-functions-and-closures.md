## توابع پیشرفته و Closureها

این بخش به بررسی برخی از ویژگی‌های پیشرفته مربوط به توابع و Closureها می‌پردازد، از جمله Pointerهای تابع و بازگرداندن Closureها.

### Pointerهای تابع

قبلاً در مورد چگونگی ارسال Closureها به توابع صحبت کردیم؛ شما همچنین می‌توانید توابع معمولی را به توابع دیگر ارسال کنید! این تکنیک زمانی مفید است که بخواهید تابعی که قبلاً تعریف کرده‌اید را ارسال کنید به جای اینکه یک Closureها جدید تعریف کنید. توابع به نوع `fn` (با f کوچک) تبدیل می‌شوند، که نباید با ویژگی Closureها `Fn` اشتباه گرفته شود. نوع `fn` به عنوان یک _اشاره‌گر تابع_ شناخته می‌شود. ارسال توابع با استفاده از Pointerهای تابع به شما این امکان را می‌دهد که از توابع به عنوان آرگومان برای توابع دیگر استفاده کنید.

سینتکس مشخص کردن اینکه یک پارامتر یک اشاره‌گر تابع است، مشابه Closureها است، همان‌طور که در لیست ۲۰-۲۸ نشان داده شده است. در این مثال، تابعی به نام `add_one` تعریف کرده‌ایم که یک واحد به پارامتر خود اضافه می‌کند. تابع `do_twice` دو پارامتر می‌گیرد: یک اشاره‌گر تابع به هر تابعی که یک پارامتر `i32` بگیرد و یک مقدار `i32` برگرداند، و یک مقدار `i32`. تابع `do_twice` تابع `f` را دو بار فراخوانی می‌کند، مقدار `arg` را به آن می‌فرستد و سپس نتایج دو فراخوانی را با هم جمع می‌کند. تابع `main` تابع `do_twice` را با آرگومان‌های `add_one` و `5` فراخوانی می‌کند.

<Listing number="20-28" file-name="src/main.rs" caption="استفاده از نوع `fn` برای پذیرش یک اشاره‌گر تابع به عنوان آرگومان">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

این کد مقدار `The answer is: 12` را چاپ می‌کند. ما مشخص کرده‌ایم که پارامتر `f` در `do_twice` یک `fn` است که یک پارامتر از نوع `i32` می‌گیرد و یک `i32` باز می‌گرداند. سپس می‌توانیم `f` را در بدنه تابع `do_twice` فراخوانی کنیم. در `main`، می‌توانیم نام تابع `add_one` را به عنوان آرگومان اول به `do_twice` ارسال کنیم.

برخلاف Closureها `fn` یک نوع است و نه یک ویژگی، بنابراین ما `fn` را به طور مستقیم به عنوان نوع پارامتر مشخص می‌کنیم، به جای اعلام یک پارامتر جنریک با یکی از ویژگی‌های `Fn` به عنوان محدودیت ویژگی.

Pointerهای تابع تمام سه ویژگی Closureها (`Fn`، `FnMut`، و `FnOnce`) را پیاده‌سازی می‌کنند، به این معنی که شما همیشه می‌توانید یک اشاره‌گر تابع را به عنوان آرگومان برای یک تابع که انتظار یک Closureها را دارد ارسال کنید. بهتر است توابع را با استفاده از یک نوع جنریک و یکی از ویژگی‌های Closureها بنویسید تا توابع شما بتوانند هم توابع و هم Closureها را بپذیرند.

با این حال، یک مثال از جایی که ممکن است بخواهید فقط `fn` را بپذیرید و نه Closureها زمانی است که با کد خارجی که Closureها ندارد تعامل می‌کنید: توابع C می‌توانند توابع را به عنوان آرگومان بپذیرند، اما C Closureها ندارد.

به عنوان مثالی از جایی که می‌توانید از یک Closureها تعریف‌شده درون‌خطی یا یک تابع نام‌گذاری‌شده استفاده کنید، بیایید به استفاده از متد `map` که توسط ویژگی `Iterator` در کتابخانه استاندارد ارائه شده است نگاهی بیندازیم. برای استفاده از تابع `map` برای تبدیل یک بردار اعداد به یک بردار رشته‌ها، می‌توانیم از یک Closureها به این صورت استفاده کنیم:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-15-map-closure/src/main.rs:here}}
```

Or we could name a function as the argument to `map` instead of the closure,
like this:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-16-map-function/src/main.rs:here}}
```

Note that we must use the fully qualified syntax that we talked about earlier
in the [“Advanced Traits”][advanced-traits]<!-- ignore --> section because
there are multiple functions available named `to_string`. Here, we’re using the
`to_string` function defined in the `ToString` trait, which the standard
library has implemented for any type that implements `Display`.

Recall from the [“Enum values”][enum-values]<!-- ignore --> section of Chapter
6 that the name of each enum variant that we define also becomes an initializer
function. We can use these initializer functions as function pointers that
implement the closure traits, which means we can specify the initializer
functions as arguments for methods that take closures, like so:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-17-map-initializer/src/main.rs:here}}
```

Here we create `Status::Value` instances using each `u32` value in the range
that `map` is called on by using the initializer function of `Status::Value`.
Some people prefer this style, and some people prefer to use closures. They
compile to the same code, so use whichever style is clearer to you.

### Returning Closures

Closures are represented by traits, which means you can’t return closures
directly. In most cases where you might want to return a trait, you can instead
use the concrete type that implements the trait as the return value of the
function. However, you can’t do that with closures because they don’t have a
concrete type that is returnable; you’re not allowed to use the function
pointer `fn` as a return type, for example.

Instead, you will normally use the `impl Trait` syntax we learned about in
Chapter 10. You can return any function type, using `Fn`, `FnOnce` and `FnMut`.
For example, this code will work just fine:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-18-returns-closure/src/lib.rs}}
```

However, as we noted in the [“Closure Type Inference and
Annotation”][closure-types]<!-- ignore --> section in Chapter 13, each closure
is also its own distinct type. If you need to work with multiple functions that
have the same signature but different implementations, you will need to use a
trait object for them:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-19-returns-closure-trait-object/src/main.rs}}
```

This code will compile just fine—but it wouldn’t if we had tried to stick with
`impl Fn(i32) -> i32`. For more about trait objects, refer to the section
[“Using Trait Objects That Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore
--> in Chapter 18.

Next, let’s look at macros!

[advanced-traits]: ch20-02-advanced-traits.html#advanced-traits
[enum-values]: ch06-01-defining-an-enum.html#enum-values
[closure-types]: ch13-01-closures.html#closure-type-inference-and-annotation
[using-trait-objects-that-allow-for-values-of-different-types]: ch18-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
