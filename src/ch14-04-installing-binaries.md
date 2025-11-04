<!-- Old headings. Do not remove or links may break. -->

<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

## تثبيت الملفات الثنائية باستخدام `cargo install`

يسمح لك أمر `cargo install` بتثبيت واستخدام الصناديق الثنائية
محليًا. هذا ليس مخصصًا لاستبدال حزم النظام؛ بل هو مخصص ليكون
طريقة ملائمة لمطوري Rust لتثبيت الأدوات التي شاركها الآخرون على
[crates.io](https://crates.io/)<!-- ignore -->. لاحظ أنه يمكنك فقط تثبيت
الحزم التي لديها أهداف ثنائية. _الهدف الثنائي_ (binary target) هو البرنامج القابل للتشغيل
الذي يتم إنشاؤه إذا كان لدى الصندوق ملف _src/main.rs_ أو ملف آخر محدد
كملف ثنائي، على عكس هدف المكتبة الذي لا يمكن تشغيله بمفرده ولكن
مناسب للتضمين داخل برامج أخرى. عادةً، تحتوي الصناديق على
معلومات في ملف README حول ما إذا كان الصندوق مكتبة، أم لديه
هدف ثنائي، أم كلاهما.

جميع الملفات الثنائية المثبتة باستخدام `cargo install` يتم تخزينها في مجلد _bin_
الخاص بجذر التثبيت. إذا قمت بتثبيت Rust باستخدام _rustup.rs_ ولا يوجد لديك أي
إعدادات مخصصة، فسيكون هذا الدليل هو *$HOME/.cargo/bin*. تأكد من أن
هذا الدليل موجود في `$PATH` الخاص بك لتتمكن من تشغيل البرامج التي قمت بتثبيتها
باستخدام `cargo install`.

على سبيل المثال، في الفصل 12 ذكرنا أن هناك تطبيق Rust لأداة
`grep` يسمى `ripgrep` للبحث في الملفات. لتثبيت `ripgrep`، يمكننا
تشغيل التالي:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

السطر قبل الأخير من المخرجات يُظهر موقع واسم
الملف الثنائي المثبت، والذي في حالة `ripgrep` هو `rg`. طالما أن
دليل التثبيت موجود في `$PATH` الخاص بك، كما ذُكر سابقًا، يمكنك
بعد ذلك تشغيل `rg --help` والبدء في استخدام أداة أسرع وأكثر Rustية للبحث في الملفات!
