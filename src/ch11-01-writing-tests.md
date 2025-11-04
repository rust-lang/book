## كيفية كتابة الاختبارات

_الاختبارات_ هي دوال Rust تتحقق من أن الكود غير الاختباري يعمل بالطريقة
المتوقعة. عادةً ما تقوم أجسام دوال الاختبار بهذه الإجراءات الثلاثة:

- إعداد أي بيانات أو حالة مطلوبة.
- تشغيل الكود الذي تريد اختباره.
- التأكد من أن النتائج هي ما تتوقعه.

لننظر إلى الميزات التي توفرها Rust خصيصاً لكتابة الاختبارات التي
تتخذ هذه الإجراءات، والتي تشمل خاصية `test`، وبعض الماكروهات، وخاصية
`should_panic`.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-anatomy-of-a-test-function"></a>

### هيكلة دوال الاختبار

في أبسط صورها، الاختبار في Rust هو دالة مشروحة بخاصية `test`.
الخصائص هي بيانات وصفية حول أجزاء من كود Rust؛ مثال على ذلك هو
خاصية `derive` التي استخدمناها مع الهياكل في الفصل 5. لتحويل دالة
إلى دالة اختبار، أضف `#[test]` في السطر قبل `fn`. عندما تقوم بتشغيل
اختباراتك باستخدام أمر `cargo test`، تقوم Rust ببناء ملف تنفيذي لتشغيل الاختبارات يقوم بتشغيل
الدوال المشروحة ويبلغ عما إذا كانت كل دالة اختبار تنجح أو
تفشل.

كلما أنشأنا مشروع مكتبة جديد باستخدام Cargo، يتم إنشاء وحدة اختبار مع دالة اختبار
فيها تلقائياً لنا. توفر لك هذه الوحدة
قالباً لكتابة اختباراتك بحيث لا تضطر إلى البحث عن
البنية والصياغة الدقيقة في كل مرة تبدأ فيها مشروعاً جديداً. يمكنك إضافة
عدد دوال الاختبار الإضافية ووحدات الاختبار التي تريدها!

سنستكشف بعض جوانب كيفية عمل الاختبارات عن طريق التجربة مع القالب
قبل أن نختبر أي كود فعلياً. بعد ذلك، سنكتب بعض الاختبارات الواقعية
التي تستدعي بعض الكود الذي كتبناه ونؤكد أن سلوكه صحيح.

لننشئ مشروع مكتبة جديد يسمى `adder` سيضيف رقمين:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

يجب أن تبدو محتويات ملف _src/lib.rs_ في مكتبة `adder` الخاصة بك مثل
القائمة 11-1.

<Listing number="11-1" file-name="src/lib.rs" caption="الكود الذي تم إنشاؤه تلقائياً بواسطة `cargo new`">

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
echo "$ cargo test" > output.txt
RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 cargo test >> output.txt 2>&1
git diff output.txt # commit any relevant changes; discard irrelevant ones
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

</Listing>

يبدأ الملف بدالة مثال `add` بحيث يكون لدينا شيء
للاختبار.

في الوقت الحالي، لنركز فقط على دالة `it_works`. لاحظ شرح `#[test]`
: هذه الخاصية تشير إلى أن هذه دالة اختبار، لذلك يعرف منفذ الاختبار
معاملة هذه الدالة كاختبار. قد يكون لدينا أيضاً دوال غير اختبارية
في وحدة `tests` للمساعدة في إعداد سيناريوهات شائعة أو أداء
عمليات شائعة، لذلك نحتاج دائماً إلى الإشارة إلى الدوال التي هي اختبارات.

يستخدم جسم الدالة المثال ماكرو `assert_eq!` للتأكد من أن `result`،
الذي يحتوي على نتيجة استدعاء `add` بـ 2 و 2، يساوي 4. هذا
التأكيد يعمل كمثال على صيغة الاختبار النموذجي. لنقم بتشغيله
لنرى أن هذا الاختبار ينجح.

أمر `cargo test` يشغل جميع الاختبارات في مشروعنا، كما هو موضح في القائمة
11-2.

<Listing number="11-2" caption="الإخراج من تشغيل الاختبار الذي تم إنشاؤه تلقائياً">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

</Listing>

قام Cargo بتجميع وتشغيل الاختبار. نرى السطر `running 1 test`. السطر التالي
يعرض اسم دالة الاختبار المولدة، تسمى `tests::it_works`،
وأن نتيجة تشغيل ذلك الاختبار هي `ok`. الملخص الإجمالي `test
result: ok.` يعني أن جميع الاختبارات نجحت، والجزء الذي يقرأ `1
passed; 0 failed` يجمع عدد الاختبارات التي نجحت أو فشلت.

من الممكن تحديد اختبار على أنه مُتجاهل بحيث لا يتم تشغيله في حالة معينة
؛ سنغطي ذلك في قسم [تجاهل الاختبارات ما لم يتم طلبها تحديداً
][ignoring]<!-- ignore --> لاحقاً في هذا الفصل. لأننا
لم نفعل ذلك هنا، يظهر الملخص `0 ignored`. يمكننا أيضاً تمرير
معامل لأمر `cargo test` لتشغيل الاختبارات فقط التي يتطابق اسمها مع
سلسلة نصية؛ هذا يسمى _الفلترة_، وسنغطيه في قسم [تشغيل
مجموعة فرعية من الاختبارات حسب الاسم][subset]<!-- ignore -->. هنا، لم
نقم بفلترة الاختبارات التي يتم تشغيلها، لذلك تظهر نهاية الملخص `0 filtered out`.

إحصائية `0 measured` مخصصة لاختبارات المعيار التي تقيس الأداء.
اختبارات المعيار، وقت كتابة هذا، متاحة فقط في Rust الليلية. انظر
[التوثيق حول اختبارات المعيار][bench] لمعرفة المزيد.

الجزء التالي من إخراج الاختبار الذي يبدأ بـ `Doc-tests adder` مخصص
لنتائج أي اختبارات توثيقية. ليس لدينا أي اختبارات توثيقية حتى الآن،
لكن Rust يمكنه تجميع أي أمثلة كود تظهر في توثيق API الخاص بنا.
تساعد هذه الميزة في الحفاظ على التوثيق والكود متزامنين! سنناقش كيفية
كتابة اختبارات التوثيق في قسم [تعليقات التوثيق كاختبارات
][doc-comments]<!-- ignore --> من الفصل 14. في الوقت الحالي، سنتجاهل
إخراج `Doc-tests`.

لنبدأ في تخصيص الاختبار لاحتياجاتنا الخاصة. أولاً، غيّر اسم
دالة `it_works` إلى اسم مختلف، مثل `exploration`، هكذا:

<span class="filename">اسم الملف: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

ثم، شغّل `cargo test` مرة أخرى. يظهر الإخراج الآن `exploration` بدلاً من
`it_works`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

الآن سنضيف اختباراً آخر، لكن هذه المرة سننشئ اختباراً فاشلاً! تفشل الاختبارات
عندما يحدث panic في دالة الاختبار. يتم تشغيل كل اختبار في خيط جديد
، وعندما يرى الخيط الرئيسي أن خيط اختبار قد مات، يتم تحديد الاختبار
على أنه فاشل. في الفصل 9، تحدثنا عن كيف أن أبسط طريقة للـ panic
هي استدعاء ماكرو `panic!`. أدخل الاختبار الجديد كدالة تسمى
`another`، بحيث يبدو ملف _src/lib.rs_ الخاص بك مثل القائمة 11-3.

<Listing number="11-3" file-name="src/lib.rs" caption="إضافة اختبار ثاني سيفشل لأننا نستدعي ماكرو `panic!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

</Listing>

شغّل الاختبارات مرة أخرى باستخدام `cargo test`. يجب أن يبدو الإخراج مثل القائمة
11-4، والتي تظهر أن اختبار `exploration` الخاص بنا نجح وأن `another` فشل.

<Listing number="11-4" caption="نتائج الاختبار عندما ينجح اختبار واحد ويفشل اختبار آخر">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

</Listing>

<!-- manual-regeneration
rg panicked listings/ch11-writing-automated-tests/listing-11-03/output.txt
check the line number of the panic matches the line number in the following paragraph
 -->

بدلاً من `ok`، يظهر السطر `test tests::another` الرسالة `FAILED`. يظهر قسمان جديدان
بين النتائج الفردية والملخص: يعرض الأول
السبب التفصيلي لفشل كل اختبار. في هذه الحالة، نحصل على
التفاصيل التي توضح أن `tests::another` فشل لأنه حدث panic برسالة `Make
this test fail` في السطر 17 في ملف _src/lib.rs_. القسم التالي يسرد
فقط أسماء جميع الاختبارات الفاشلة، وهو مفيد عندما يكون هناك الكثير من
الاختبارات والكثير من إخراج الاختبار الفاشل التفصيلي. يمكننا استخدام اسم
الاختبار الفاشل لتشغيل ذلك الاختبار فقط لتصحيحه بشكل أسهل؛ سنتحدث أكثر
عن طرق تشغيل الاختبارات في قسم [التحكم في كيفية تشغيل الاختبارات
][controlling-how-tests-are-run]<!-- ignore -->.

يعرض سطر الملخص في النهاية: بشكل عام، نتيجة اختبارنا هي `FAILED`. كان
لدينا اختبار واحد نجح واختبار واحد فشل.

الآن بعد أن رأيت كيف تبدو نتائج الاختبار في سيناريوهات مختلفة، لننظر إلى
بعض الماكروهات بخلاف `panic!` التي تكون مفيدة في الاختبارات.

### التحقق من النتائج باستخدام ماكرو `assert!`

يوفر لنا ماكرو `assert!`، المقدم من المكتبة القياسية، دالة مفيدة عندما تريد التأكد من أن
بعض الشروط في الاختبار تُقيّم إلى `true`. نعطي ماكرو `assert!` معامل
يُقيّم إلى قيمة منطقية. إذا كانت القيمة `true`، لا يحدث شيء ويمرّ الاختبار. إذا
كانت القيمة `false`، يستدعي ماكرو `assert!` ماكرو `panic!` لإحداث فشل في الاختبار
. استخدام ماكرو `assert!` يساعدنا في التحقق من أن الكود الخاص بنا يعمل
بالطريقة التي نعتزمها.

في الفصل 5، القائمة 5-15، استخدمنا هيكل `Rectangle` ودالة `can_hold`
، والتي يتم تكرارها هنا في القائمة 11-5. لنضع هذا الكود في ملف
_src/lib.rs_، ثم نكتب بعض الاختبارات له باستخدام ماكرو `assert!`.

<Listing number="11-5" file-name="src/lib.rs" caption="استخدام هيكل `Rectangle` ودالته `can_hold` من الفصل 5">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

</Listing>

دالة `can_hold` ترجع قيمة منطقية، مما يعني أنها حالة استخدام مثالية لـ
ماكرو `assert!`. في القائمة 11-6، نكتب اختباراً يمارس دالة `can_hold`
بإنشاء مثيل `Rectangle` له عرض 8 وارتفاع 7 ونؤكد أنه يمكن
استيعاب مثيل `Rectangle` آخر له عرض 5 وارتفاع 1.

<Listing number="11-6" file-name="src/lib.rs" caption="اختبار لـ `can_hold` يتحقق من أن مستطيلاً أكبر يمكنه في الواقع استيعاب مستطيل أصغر">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

</Listing>

لاحظ أننا أضفنا سطراً جديداً داخل وحدة `tests`: `use super::*;`. وحدة
`tests` هي وحدة عادية تتبع قواعد الرؤية المعتادة التي غطيناها في الفصل 7 في
قسم ["المسارات للإشارة إلى عنصر في شجرة الوحدة"][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -->. لأن وحدة `tests` هي
وحدة داخلية، نحتاج إلى جلب الكود الذي يتم اختباره في الوحدة الخارجية إلى نطاق
الوحدة الداخلية. نستخدم glob هنا لذا أي شيء نعرفه في الوحدة الخارجية متاح
لوحدة `tests` هذه.

أطلقنا على اختبارنا اسم `larger_can_hold_smaller`، وأنشأنا مثيلين من `Rectangle`
اللذين نحتاجهما. ثم استدعينا ماكرو `assert!` ومررنا له نتيجة استدعاء
`larger.can_hold(&smaller)`. من المفترض أن يُرجع هذا التعبير `true`، لذا
يجب أن ينجح اختبارنا. لنكتشف!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

ينجح! لنضف اختباراً آخر، هذه المرة نؤكد أن مستطيلاً أصغر لا يمكنه استيعاب
مستطيل أكبر:

<span class="filename">اسم الملف: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

لأن النتيجة الصحيحة لدالة `can_hold` في هذه الحالة هي `false`، نحتاج إلى
نفي تلك النتيجة قبل أن نمررها إلى ماكرو `assert!`. نتيجة لذلك، سينجح
اختبارنا إذا أرجعت `can_hold` القيمة `false`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

اختباران ينجحان! الآن لنرى ماذا يحدث لنتائج اختبارنا عندما نُدخل خطأ في
كودنا. سنغير تطبيق دالة `can_hold` باستبدال إشارة أكبر من
بإشارة أقل من عند مقارنة العرض:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

تشغيل الاختبارات الآن ينتج ما يلي:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

اختباراتنا كشفت الخطأ! لأن `larger.width` هو 8 و `smaller.width` هو 5،
مقارنة العرض في `can_hold` ترجع الآن `false`: 8 ليست أقل من 5.

### اختبار المساواة باستخدام ماكروهات `assert_eq!` و `assert_ne!`

طريقة شائعة للتحقق من الوظيفة هي اختبار المساواة بين نتيجة
الكود الذي يتم اختباره والقيمة التي تتوقع أن يُرجعها الكود. يمكنك القيام بذلك
باستخدام ماكرو `assert!` وتمرير تعبير يستخدم معامل `==`. ومع ذلك، هذا
اختبار شائع جداً لدرجة أن المكتبة القياسية توفر زوجاً من الماكروهات--`assert_eq!`
و`assert_ne!`--لأداء هذا الاختبار بشكل أكثر ملاءمة. تقارن هذه الماكروهات قيمتين
للمساواة أو عدم المساواة، على التوالي. ستطبع أيضاً القيمتين إذا
فشل التأكيد، مما يسهل رؤية _سبب_ فشل الاختبار؛ بالمقابل، يشير ماكرو
`assert!` فقط إلى أنه حصل على قيمة `false` لتعبير `==`، ولكن ليس
القيمتين اللتين أديتا إلى قيمة `false`.

في القائمة 11-7، نكتب دالة تسمى `add_two` تضيف `2` إلى معاملها، ثم
نختبر هذه الدالة باستخدام ماكرو `assert_eq!`.

<Listing number="11-7" file-name="src/lib.rs" caption="اختبار دالة `add_two` باستخدام ماكرو `assert_eq!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

</Listing>

لنتحقق من أنه ينجح!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

نمرر `4` كمعامل إلى `assert_eq!`، وهو يساوي نتيجة استدعاء `add_two(2)`.
السطر لهذا الاختبار هو `test tests::it_adds_two ... ok`، والنص `ok`
يشير إلى أن اختبارنا نجح!

لنُدخل خطأ في كودنا لنرى كيف يبدو عندما يفشل اختبار يستخدم `assert_eq!`
. غيّر تطبيق دالة `add_two` لتضيف `3` بدلاً من ذلك:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

شغّل الاختبارات مرة أخرى:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

اختبارنا كشف الخطأ! اختبار `it_adds_two` فشل، ويخبرنا السطر
بأن التأكيد الذي فشل كان ``assertion `left == right` failed`` وما هي قيم
`left` و`right`. هذه الرسالة مفيدة لبدء التصحيح: كان معامل `left`
هو `4` ولكن المعامل `right`، حيث لدينا `add_two(2)`، كان `5`. يمكنك
أن تتخيل أن هذا سيكون مفيداً بشكل خاص عندما يكون لدينا الكثير من الاختبارات تحدث.

لاحظ أنه في بعض اللغات والأطر الاختبارية، تسمى معاملات دوال التأكد على المساواة
`expected` و `actual`، والترتيب الذي نحدد فيه المعاملات مهم. ومع ذلك، في
Rust، تسمى `left` و `right`، والترتيب الذي نحدد فيه القيمة التي نتوقعها
والقيمة التي ينتجها الكود الذي يتم اختباره لا يهم. يمكننا كتابة التأكيد في هذا
الاختبار كـ `assert_eq!(add_two(2), 4)`، مما سينتج رسالة فشل تعرض
``assertion failed: `(left == right)``` و`left` ستكون `5` و `right` ستكون
`4`.

ماكرو `assert_ne!` سيمر إذا لم تكن القيمتان اللتان نعطيهما متساويتين وسيفشل
إذا كانتا متساويتين. هذا الماكرو أكثر فائدة للحالات التي لسنا متأكدين فيها من _ماهية_
القيمة، لكننا نعرف ما يجب أن تكون _لا_ تكون عليه القيمة بالتأكيد. على سبيل المثال، إذا كنا نختبر
دالة من المضمون أنها ستغير إدخالها بطريقة ما، لكن _الطريقة_
التي يتم بها تغيير الإدخال تعتمد على يوم الأسبوع الذي نشغل فيه
الاختبارات، أفضل شيء للتأكد منه قد يكون أن إخراج الدالة لا يساوي
الإدخال.

تحت السطح، تستخدم ماكروهات `assert_eq!` و `assert_ne!` المعاملين `==`
و `!=`، على التوالي. عندما تفشل التأكيدات، تطبع هذه الماكروهات معاملاتها
باستخدام تنسيق التصحيح، مما يعني أن القيم التي يتم مقارنتها يجب أن تطبق
خصائص `PartialEq` و `Debug`. جميع الأنواع البدائية والعديد من أنواع المكتبة
القياسية تطبق هذه الخصائص. للهياكل والتعدادات التي تعرفها، ستحتاج إلى
تطبيق `PartialEq` لتؤكد على المساواة بين تلك الأنواع. ستحتاج أيضاً إلى تطبيق
`Debug` لطباعة القيم عندما يفشل التأكيد. لأن كلا الخاصيتين هما خصائص قابلة
للاشتقاق، كما ذُكر في القائمة 5-12 في الفصل 5، يكون هذا عادةً بسيطاً كإضافة
شرح `#[derive(PartialEq, Debug)]` إلى تعريف هيكلك أو تعدادك. انظر
الملحق ج، ["الخصائص القابلة للاشتقاق،"][derivable-traits]<!-- ignore
--> لمزيد من التفاصيل حول هذه وغيرها من الخصائص القابلة للاشتقاق.

### إضافة رسائل فشل مخصصة

يمكنك أيضاً إضافة رسالة مخصصة لتُطبع مع رسالة الفشل كمعاملات
اختيارية لماكروهات `assert!`، `assert_eq!`، و`assert_ne!`. يتم تمرير أي معاملات
محددة بعد المعاملات المطلوبة إلى ماكرو `format!` (الذي نوقش في الفصل 8 في
قسم ["التسلسل بـ `+` أو ماكرو `format!`"][concatenating]<!-- ignore -->)، لذا يمكنك تمرير
سلسلة تنسيق تحتوي على عناصر نائبة `{}` وقيم لتذهب في تلك العناصر النائبة. رسائل
مخصصة مفيدة لتوثيق ما يعنيه التأكيد؛ عندما يفشل اختبار،
سيكون لديك فكرة أفضل عن ما هي المشكلة في الكود.

على سبيل المثال، لنفترض أن لدينا دالة تُحيّي الأشخاص بالاسم ونريد اختبار أن
الاسم الذي نمرره إلى الدالة يظهر في الإخراج:

<span class="filename">اسم الملف: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

متطلبات هذا البرنامج لم يتم الاتفاق عليها بعد، ونحن
متأكدون تماماً من أن نص `Hello` في بداية التحية سيتغير. قررنا
أننا لا نريد أن نضطر إلى تحديث الاختبار عندما تتغير المتطلبات،
لذا بدلاً من التحقق من المساواة الدقيقة للقيمة التي يُرجعها
دالة `greeting`، سنؤكد فقط أن الإخراج يحتوي على نص
معامل الإدخال.

الآن لنُدخل خطأ في هذا الكود بتغيير `greeting` لاستبعاد
`name` لنرى كيف يبدو فشل الاختبار الافتراضي:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

تشغيل هذا الاختبار ينتج ما يلي:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

هذه النتيجة تشير فقط إلى أن التأكيد فشل وأي سطر يوجد فيه
التأكيد. رسالة فشل أكثر فائدة ستطبع القيمة من
دالة `greeting`. لنضف رسالة فشل مخصصة تتكون من سلسلة تنسيق
مع عنصر نائب مملوء بالقيمة الفعلية التي حصلنا عليها من
دالة `greeting`:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

الآن عندما نشغل الاختبار، سنحصل على رسالة خطأ أكثر إفادة:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

يمكننا رؤية القيمة التي حصلنا عليها فعلياً في إخراج الاختبار، والتي ستساعدنا
في تصحيح ما حدث بدلاً من ما كنا نتوقع حدوثه.

### التحقق من حالات Panic باستخدام `should_panic`

بالإضافة إلى التحقق من القيم المُرجعة، من المهم التحقق من أن كودنا
يتعامل مع ظروف الخطأ كما نتوقع. على سبيل المثال، ضع في اعتبارك نوع `Guess`
الذي أنشأناه في الفصل 9، القائمة 9-13. الكود الآخر الذي يستخدم `Guess`
يعتمد على ضمان أن مثيلات `Guess` ستحتوي فقط على قيم
بين 1 و 100. يمكننا كتابة اختبار يضمن أن محاولة إنشاء
مثيل `Guess` بقيمة خارج ذلك النطاق ستسبب panic.

نقوم بذلك بإضافة خاصية `should_panic` إلى دالة الاختبار الخاصة بنا.
ينجح الاختبار إذا حدث panic في الكود داخل الدالة؛ يفشل الاختبار إذا لم
يحدث panic في الكود داخل الدالة.

تُظهر القائمة 11-8 اختباراً يتحقق من أن ظروف الخطأ في `Guess::new`
تحدث عندما نتوقع حدوثها.

<Listing number="11-8" file-name="src/lib.rs" caption="اختبار أن شرطاً سيسبب `panic!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

</Listing>

نضع خاصية `#[should_panic]` بعد خاصية `#[test]` وقبل
دالة الاختبار التي تنطبق عليها. لننظر إلى النتيجة عندما ينجح هذا الاختبار
:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

يبدو جيداً! الآن لنُدخل خطأ في كودنا بإزالة الشرط
أن دالة `new` ستحدث فيها panic إذا كانت القيمة أكبر من 100:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

عندما نشغل الاختبار في القائمة 11-8، سيفشل:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

لا نحصل على رسالة مفيدة جداً في هذه الحالة، ولكن عندما ننظر إلى دالة الاختبار
، نرى أنها مشروحة بـ `#[should_panic]`. الفشل الذي حصلنا عليه
يعني أن الكود في دالة الاختبار لم يُسبّب panic.

الاختبارات التي تستخدم `should_panic` يمكن أن تكون غير دقيقة. اختبار `should_panic`
سينجح حتى لو حدث panic في الاختبار لسبب مختلف عن الذي كنا
نتوقعه. لجعل اختبارات `should_panic` أكثر دقة، يمكننا إضافة معامل
`expected` اختياري إلى خاصية `should_panic`. أداة الاختبار ستتأكد
من أن رسالة الفشل تحتوي على النص المقدم. على سبيل المثال،
ضع في اعتبارك الكود المعدل لـ `Guess` في القائمة 11-9 حيث تحدث دالة `new` 
فيها panic برسائل مختلفة اعتماداً على ما إذا كانت القيمة صغيرة جداً أو
كبيرة جداً.

<Listing number="11-9" file-name="src/lib.rs" caption="اختبار `panic!` برسالة panic تحتوي على سلسلة فرعية محددة">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

</Listing>

سينجح هذا الاختبار لأن القيمة التي وضعناها في معامل `expected`
لخاصية `should_panic` هي سلسلة فرعية من الرسالة التي تحدث فيها panic دالة `Guess::new`
. كان يمكننا أن نحدد رسالة panic الكاملة التي نتوقعها، والتي في هذه الحالة ستكون `Guess value must be less than or equal to
100, got 200`. ما تختار تحديده يعتمد على مدى تفرد أو ديناميكية رسالة panic
وكم تريد أن يكون اختبارك دقيقاً. في هذه
الحالة، سلسلة فرعية من رسالة panic كافية للتأكد من أن الكود في
دالة الاختبار ينفذ حالة `else if value > 100`.

لنرى ماذا يحدث عندما يفشل اختبار `should_panic` مع رسالة `expected`
، لنُدخل خطأ في كودنا مرة أخرى عن طريق تبديل أجسام
`if value < 1` و `else if value > 100`:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

هذه المرة عندما نشغل اختبار `should_panic`، سيفشل:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

تشير رسالة الفشل إلى أن هذا الاختبار حدثت فيه panic كما توقعنا،
ولكن رسالة panic لم تتضمن السلسلة المتوقعة `less than or equal
to 100`. رسالة panic التي حصلنا عليها في هذه الحالة كانت `Guess value must
be greater than or equal to 1, got 200`. الآن يمكننا البدء في معرفة أين
يوجد خطؤنا!

### استخدام `Result<T, E>` في الاختبارات

جميع اختباراتنا حتى الآن تحدث فيها panic عندما تفشل. يمكننا أيضاً كتابة اختبارات تستخدم
`Result<T, E>`! إليك الاختبار من القائمة 11-1، معاد كتابته ليستخدم `Result<T,
E>` ويُرجع `Err` بدلاً من حدوث panic:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

دالة `it_works` لديها الآن نوع الإرجاع `Result<(), String>`. في
جسم الدالة، بدلاً من استدعاء ماكرو `assert_eq!`، نُرجع
`Ok(())` عندما ينجح الاختبار و `Err` مع `String` بداخله عندما
يفشل الاختبار.

كتابة الاختبارات بحيث تُرجع `Result<T, E>` تمكنك من استخدام
عامل علامة الاستفهام في جسم الاختبارات، وهو ما يمكن أن يكون طريقة ملائمة
لكتابة اختبارات يجب أن تفشل إذا أرجعت أي عملية داخلها متغير `Err`
.

لا يمكنك استخدام شرح `#[should_panic]` على اختبارات تستخدم `Result<T,
E>`. للتأكد من أن عملية ما تُرجع متغير `Err`، _لا تستخدم_
عامل علامة الاستفهام على قيمة `Result<T, E>`. بدلاً من ذلك، استخدم
`assert!(value.is_err())`.

الآن بعد أن تعرف عدة طرق لكتابة الاختبارات، لننظر إلى ما يحدث
عندما نشغل اختباراتنا ونستكشف الخيارات المختلفة التي يمكننا استخدامها مع `cargo
test`.

[concatenating]: ch08-02-strings.html#concatenating-with--or-format
[bench]: ../unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignoring-tests-unless-specifically-requested
[subset]: ch11-02-running-tests.html#running-a-subset-of-tests-by-name
[controlling-how-tests-are-run]: ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
