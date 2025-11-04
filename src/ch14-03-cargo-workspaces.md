## مساحات عمل Cargo

في الفصل 12، بنينا حزمة تضمنت صندوق ثنائي وصندوق مكتبة.
مع تطور مشروعك، قد تجد أن صندوق المكتبة
يستمر في النمو وتريد تقسيم حزمتك أكثر إلى
صناديق مكتبة متعددة. يقدم Cargo ميزة تسمى _مساحات العمل_ (workspaces) التي يمكن أن
تساعد في إدارة حزم متعددة ذات صلة يتم تطويرها معًا.

### إنشاء مساحة عمل

_مساحة العمل_ (workspace) هي مجموعة من الحزم التي تشترك في نفس _Cargo.lock_ ودليل الإخراج.
لنقم بإنشاء مشروع باستخدام مساحة عمل—سنستخدم كودًا بسيطًا حتى
نتمكن من التركيز على هيكل مساحة العمل. هناك طرق متعددة
لهيكلة مساحة عمل، لذا سنعرض فقط طريقة شائعة واحدة. سيكون لدينا
مساحة عمل تحتوي على ملف ثنائي ومكتبتين. سيوفر الملف الثنائي، الذي
الوظيفة الرئيسية، سيعتمد على المكتبتين. ستوفر إحدى المكتبات
دالة `add_one` والمكتبة الأخرى دالة `add_two`.
ستكون هذه الصناديق الثلاثة جزءًا من نفس مساحة العمل. سنبدأ بإنشاء
دليل جديد لمساحة العمل:

```console
$ mkdir add
$ cd add
```

بعد ذلك، في دليل _add_، نُنشئ ملف _Cargo.toml_ الذي سـ
يهيئ مساحة العمل بأكملها. لن يحتوي هذا الملف على قسم `[package]`.
بدلاً من ذلك، سيبدأ بقسم `[workspace]` الذي سيسمح لنا بإضافة
أعضاء إلى مساحة العمل. نحن أيضًا نحرص على استخدام أحدث وأفضل
إصدار من خوارزمية محلل Cargo في مساحة عملنا عن طريق تعيين قيمة
`resolver` إلى `"3"`:

<span class="filename">اسم الملف: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

بعد ذلك، سننشئ صندوق `adder` الثنائي عن طريق تشغيل `cargo new` داخل
دليل _add_:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
remove `members = ["adder"]` from Cargo.toml
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

تشغيل `cargo new` داخل مساحة عمل يضيف أيضًا تلقائيًا الحزمة المُنشأة حديثًا
إلى مفتاح `members` في تعريف `[workspace]` في ملف _Cargo.toml_
الخاص بمساحة العمل، كالتالي:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

في هذه المرحلة، يمكننا بناء مساحة العمل عن طريق تشغيل `cargo build`. يجب أن تبدو الملفات
في دليل _add_ الخاص بك كالتالي:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

تحتوي مساحة العمل على دليل _target_ واحد في المستوى الأعلى الذي سيتم
وضع المصنوعات المصرّفة فيه؛ حزمة `adder` ليس لديها دليل _target_
الخاص بها. حتى لو كنا سنشغل `cargo build` من داخل
دليل _adder_، فإن المصنوعات المصرّفة ستنتهي في _add/target_
بدلاً من _add/adder/target_. يُهيكل Cargo دليل _target_ في
مساحة عمل بهذه الطريقة لأن الصناديق في مساحة عمل من المفترض أن تعتمد على
بعضها البعض. إذا كان لكل صندوق دليل _target_ الخاص به، فسيتعين على كل صندوق
إعادة تصريف كل من الصناديق الأخرى في مساحة العمل لوضع المصنوعات
في دليل _target_ الخاص به. من خلال مشاركة دليل _target_ واحد، يمكن للصناديق
تجنب إعادة البناء غير الضرورية.

### إنشاء الحزمة الثانية في مساحة العمل

بعد ذلك، لنُنشئ حزمة عضو أخرى في مساحة العمل ونسميها
`add_one`. أنشئ صندوق مكتبة جديد باسم `add_one`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
remove `"add_one"` from `members` list in Cargo.toml
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

سيتضمن ملف _Cargo.toml_ في المستوى الأعلى الآن مسار _add_one_ في قائمة `members`:

<span class="filename">اسم الملف: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

يجب أن يحتوي دليل _add_ الخاص بك الآن على هذه الأدلة والملفات:

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

في ملف _add_one/src/lib.rs_، لنضف دالة `add_one`:

<span class="filename">اسم الملف: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

الآن يمكننا جعل حزمة `adder` مع ملفنا الثنائي تعتمد على حزمة `add_one`
التي لديها مكتبتنا. أولاً، سنحتاج إلى إضافة تبعية مسار على
`add_one` إلى _adder/Cargo.toml_.

<span class="filename">اسم الملف: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

لا يفترض Cargo أن الصناديق في مساحة عمل ستعتمد على بعضها البعض، لذا
نحتاج إلى أن نكون صريحين بشأن علاقات التبعية.

بعد ذلك، لنستخدم دالة `add_one` (من صندوق `add_one`) في
صندوق `adder`. افتح ملف _adder/src/main.rs_ وغيّر دالة `main`
لاستدعاء دالة `add_one`، كما في Listing 14-7.

<Listing number="14-7" file-name="adder/src/main.rs" caption="استخدام صندوق مكتبة `add_one` من صندوق `adder`">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

</Listing>

لنبني مساحة العمل عن طريق تشغيل `cargo build` في دليل _add_
في المستوى الأعلى!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

لتشغيل الصندوق الثنائي من دليل _add_، يمكننا تحديد الحزمة
في مساحة العمل التي نريد تشغيلها باستخدام معامل `-p` واسم الحزمة
مع `cargo run`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

هذا يشغل الكود في _adder/src/main.rs_، والذي يعتمد على صندوق `add_one`.

<!-- Old headings. Do not remove or links may break. -->

<a id="depending-on-an-external-package-in-a-workspace"></a>

### الاعتماد على حزمة خارجية

لاحظ أن مساحة العمل لديها ملف _Cargo.lock_ واحد فقط في المستوى الأعلى،
بدلاً من وجود _Cargo.lock_ في دليل كل صندوق. هذا يضمن
أن جميع الصناديق تستخدم نفس الإصدار من جميع التبعيات. إذا أضفنا حزمة `rand`
إلى ملفات _adder/Cargo.toml_ و _add_one/Cargo.toml_، فسيحل Cargo
كلاً منهما إلى إصدار واحد من `rand` ويسجل ذلك في ملف _Cargo.lock_
الواحد. جعل جميع الصناديق في مساحة العمل تستخدم نفس التبعيات
يعني أن الصناديق ستكون دائمًا متوافقة مع بعضها البعض. دعنا نضيف
صندوق `rand` إلى قسم `[dependencies]` في ملف _add_one/Cargo.toml_
حتى نتمكن من استخدام صندوق `rand` في صندوق `add_one`:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">اسم الملف: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

يمكننا الآن إضافة `use rand;` إلى ملف _add_one/src/lib.rs_، وبناء
مساحة العمل بأكملها عن طريق تشغيل `cargo build` في دليل _add_ سيُحضر
ويصرّف صندوق `rand`. سنحصل على تحذير واحد لأننا لا
نشير إلى `rand` الذي أحضرناه إلى النطاق:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

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

يحتوي ملف _Cargo.lock_ في المستوى الأعلى الآن على معلومات حول تبعية
`add_one` على `rand`. ومع ذلك، على الرغم من استخدام `rand` في مكان ما في
مساحة العمل، لا يمكننا استخدامه في صناديق أخرى في مساحة العمل ما لم
نضف `rand` إلى ملفات _Cargo.toml_ الخاصة بها أيضًا. على سبيل المثال، إذا أضفنا `use rand;`
إلى ملف _adder/src/main.rs_ لحزمة `adder`، سنحصل على خطأ:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

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

لإصلاح هذا، حرّر ملف _Cargo.toml_ لحزمة `adder` وأشر إلى
أن `rand` هو أيضًا تبعية لها. بناء حزمة `adder` سيضيف
`rand` إلى قائمة تبعيات `adder` في _Cargo.lock_، ولكن لن
يتم تنزيل نسخ إضافية من `rand`. سيضمن Cargo أن كل
صندوق في كل حزمة في مساحة العمل التي تستخدم حزمة `rand` ستستخدم
نفس الإصدار طالما حددت إصدارات متوافقة من `rand`، مما يوفر لنا
مساحة ويضمن أن الصناديق في مساحة العمل ستكون متوافقة مع
بعضها البعض.

إذا حددت الصناديق في مساحة العمل إصدارات غير متوافقة من نفس
التبعية، سيحل Cargo كل منها ولكن سيظل يحاول حل أقل
عدد ممكن من الإصدارات.

### إضافة اختبار إلى مساحة عمل

لتحسين آخر، دعنا نضيف اختبارًا لدالة `add_one::add_one`
داخل صندوق `add_one`:

<span class="filename">اسم الملف: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

الآن قم بتشغيل `cargo test` في دليل _add_ في المستوى الأعلى. تشغيل `cargo test` في
مساحة عمل مهيكلة مثل هذه سيشغل الاختبارات لجميع الصناديق في
مساحة العمل:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

يُظهر القسم الأول من المخرجات أن اختبار `it_works` في صندوق `add_one`
نجح. يُظهر القسم التالي أنه لم يتم العثور على اختبارات في صندوق `adder`،
ثم يُظهر القسم الأخير أنه لم يتم العثور على اختبارات توثيق في
صندوق `add_one`.

يمكننا أيضًا تشغيل اختبارات لصندوق معين في مساحة عمل من
الدليل في المستوى الأعلى باستخدام علامة `-p` وتحديد اسم الصندوق
الذي نريد اختباره:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

يُظهر هذا المخرج أن `cargo test` شغّل فقط الاختبارات لصندوق `add_one` ولم
يشغّل اختبارات صندوق `adder`.

إذا نشرت الصناديق في مساحة العمل إلى
[crates.io](https://crates.io/)<!-- ignore -->، فسيحتاج كل صندوق في مساحة العمل
إلى نشره بشكل منفصل. مثل `cargo test`، يمكننا نشر
صندوق معين في مساحة عملنا باستخدام علامة `-p` وتحديد
اسم الصندوق الذي نريد نشره.

لممارسة إضافية، أضف صندوق `add_two` إلى مساحة العمل هذه بطريقة مماثلة
لصندوق `add_one`!

مع نمو مشروعك، فكّر في استخدام مساحة عمل: من الأسهل فهم
مكونات أصغر وأسهل من كتلة كبيرة واحدة من الكود.
علاوة على ذلك، الحفاظ على الصناديق في مساحة عمل يمكن أن يجعل التنسيق بين
الصناديق أسهل إذا كان يتم تغييرها في نفس الوقت.
