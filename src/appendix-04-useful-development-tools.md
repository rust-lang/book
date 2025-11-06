## الملحق د: أدوات التطوير المفيدة

في هذا الملحق، سنتحدث عن بعض أدوات التطوير المفيدة التي يوفرها مشروع Rust.
سننظر في التنسيق التلقائي، والطرق السريعة لتطبيق إصلاحات التحذيرات، وأداة
الفحص (linter)، والتكامل مع بيئات التطوير المتكاملة (IDEs).

### التنسيق التلقائي باستخدام `rustfmt`

تعيد أداة `rustfmt` تنسيق الكود الخاص بك وفقاً لنمط الكود المجتمعي. تستخدم
العديد من المشاريع التعاونية `rustfmt` لمنع الجدالات حول أي نمط يجب استخدامه
عند كتابة Rust: يقوم الجميع بتنسيق الكود الخاص بهم باستخدام هذه الأداة.

تتضمن تثبيتات Rust أداة `rustfmt` بشكل افتراضي، لذا يجب أن يكون لديك بالفعل
البرنامجان `rustfmt` و `cargo-fmt` على نظامك. هذان الأمران مماثلان لـ `rustc`
و `cargo` حيث أن `rustfmt` يتيح تحكماً أكثر دقة بينما يفهم `cargo-fmt` اصطلاحات
المشروع الذي يستخدم Cargo. لتنسيق أي مشروع Cargo، أدخل الأمر التالي:

```console
$ cargo fmt
```

تشغيل هذا الأمر يعيد تنسيق كل كود Rust في الصندوق الحالي. يجب أن يغير هذا نمط
الكود فقط، وليس معنى الكود. لمزيد من المعلومات حول `rustfmt`، راجع
[توثيقها][rustfmt].

### إصلاح الكود الخاص بك باستخدام `rustfix`

تم تضمين أداة `rustfix` مع تثبيتات Rust ويمكنها إصلاح تحذيرات المصرِّف تلقائياً
التي لديها طريقة واضحة لتصحيح المشكلة التي من المحتمل أن تكون ما تريده. ربما
رأيت تحذيرات المصرِّف من قبل. على سبيل المثال، ضع في اعتبارك هذا الكود:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

هنا، نقوم بتعريف المتغير `x` كقابل للتعديل (mutable)، لكننا لا نعدله أبداً.
يحذرنا Rust من ذلك:

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

يقترح التحذير أن نزيل كلمة `mut`. يمكننا تطبيق هذا الاقتراح تلقائياً باستخدام
أداة `rustfix` عن طريق تشغيل الأمر `cargo fix`:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

عندما ننظر إلى _src/main.rs_ مرة أخرى، سنرى أن `cargo fix` قد غير الكود:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

المتغير `x` الآن غير قابل للتعديل (immutable)، ولم يعد التحذير يظهر.

يمكنك أيضاً استخدام أمر `cargo fix` لنقل الكود الخاص بك بين إصدارات Rust
المختلفة. يتم تغطية الإصدارات في [الملحق هـ][editions]<!-- ignore -->.

### المزيد من الفحوصات باستخدام Clippy

أداة Clippy هي مجموعة من الفحوصات لتحليل الكود الخاص بك حتى تتمكن من اكتشاف
الأخطاء الشائعة وتحسين كود Rust الخاص بك. تم تضمين Clippy مع تثبيتات Rust
القياسية.

لتشغيل فحوصات Clippy على أي مشروع Cargo، أدخل الأمر التالي:

```console
$ cargo clippy
```

على سبيل المثال، لنفترض أنك تكتب برنامجاً يستخدم قيمة تقريبية لثابت رياضي، مثل
pi، كما يفعل هذا البرنامج:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

تشغيل `cargo clippy` على هذا المشروع ينتج عنه هذا الخطأ:

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

يتيح لك هذا الخطأ معرفة أن Rust لديه بالفعل ثابت `PI` أكثر دقة معرّف، وأن
برنامجك سيكون أكثر صحة إذا استخدمت الثابت بدلاً من ذلك. ستقوم بعد ذلك بتغيير
الكود الخاص بك لاستخدام ثابت `PI`.

الكود التالي لا ينتج عنه أي أخطاء أو تحذيرات من Clippy:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

لمزيد من المعلومات حول Clippy، راجع [توثيقها][clippy].

### تكامل بيئة التطوير المتكاملة باستخدام `rust-analyzer`

للمساعدة في تكامل بيئة التطوير المتكاملة، يوصي مجتمع Rust باستخدام
[`rust-analyzer`][rust-analyzer]<!-- ignore -->. هذه الأداة هي مجموعة من الأدوات
المساعدة المرتكزة على المصرِّف التي تتحدث [بروتوكول خادم اللغة][lsp]<!--
ignore -->، وهو مواصفة لبيئات التطوير المتكاملة ولغات البرمجة للتواصل مع بعضها
البعض. يمكن لعملاء مختلفين استخدام `rust-analyzer`، مثل [إضافة Rust analyzer
لـ Visual Studio Code][vscode].

قم بزيارة [الصفحة الرئيسية][rust-analyzer]<!-- ignore --> لمشروع
`rust-analyzer` للحصول على تعليمات التثبيت، ثم قم بتثبيت دعم خادم اللغة في بيئة
التطوير المتكاملة الخاصة بك. ستكتسب بيئة التطوير المتكاملة الخاصة بك قدرات مثل
الإكمال التلقائي، والانتقال إلى التعريف، والأخطاء المضمنة.

[rustfmt]: https://github.com/rust-lang/rustfmt
[editions]: appendix-05-editions.md
[clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
