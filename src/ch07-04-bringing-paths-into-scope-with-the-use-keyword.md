## إحضار المسارات إلى النطاق باستخدام الكلمة المفتاحية `use`

قد يبدو كتابة المسارات (paths) لاستدعاء الدوال أمرًا غير مريح ومتكرر. في القائمة 7-7، سواء اخترنا المسار (path) المطلق أو النسبي لدالة `add_to_waitlist`، في كل مرة أردنا استدعاء `add_to_waitlist` كان علينا تحديد `front_of_house` و `hosting` أيضًا. لحسن الحظ، هناك طريقة لتبسيط هذه العملية: يمكننا إنشاء اختصار لمسار (path) باستخدام الكلمة المفتاحية `use` مرة واحدة ثم استخدام الاسم الأقصر في كل مكان آخر في النطاق (scope).

في القائمة 7-11، نُحضر وحدة (module) `crate::front_of_house::hosting` إلى نطاق (scope) دالة `eat_at_restaurant` حتى نحتاج فقط إلى تحديد `hosting::add_to_waitlist` لاستدعاء دالة `add_to_waitlist` في `eat_at_restaurant`.

<Listing number="7-11" file-name="src/lib.rs" caption="إحضار وحدة (module) إلى النطاق (scope) باستخدام `use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

إضافة `use` ومسار (path) في نطاق (scope) مشابه لإنشاء رابط رمزي في نظام الملفات. بإضافة `use crate::front_of_house::hosting` في جذر الصندوق (crate root)، يصبح `hosting` الآن اسمًا صالحًا في ذلك النطاق (scope)، تمامًا كما لو كانت وحدة (module) `hosting` قد تم تعريفها في جذر الصندوق (crate root). المسارات (paths) المُحضَرة إلى النطاق (scope) باستخدام `use` تفحص الخصوصية (privacy) أيضًا، مثل أي مسارات (paths) أخرى.

لاحظ أن `use` تُنشئ الاختصار فقط للنطاق (scope) المحدد الذي يحدث فيه `use`. تنقل القائمة 7-12 دالة `eat_at_restaurant` إلى وحدة فرعية جديدة تُسمى `customer`، والتي تكون بعد ذلك نطاقًا (scope) مختلفًا عن تعليمة `use`، لذلك لن يتم تصريف جسم الدالة.

<Listing number="7-12" file-name="src/lib.rs" caption="تعليمة `use` تُطبق فقط في النطاق (scope) الذي توجد فيه.">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

يُظهر خطأ المُصرّف أن الاختصار لم يعد يُطبق داخل وحدة (module) `customer`:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

لاحظ أن هناك أيضًا تحذيرًا بأن `use` لم يعد مُستخدَمًا في نطاقه (scope)! لإصلاح هذه المشكلة، انقل `use` داخل وحدة (module) `customer` أيضًا، أو أشر إلى الاختصار في الوحدة (module) الأب باستخدام `super::hosting` داخل وحدة (module) `customer` الفرعية.

### إنشاء مسارات (paths) `use` اصطلاحية

في القائمة 7-11، ربما تساءلت لماذا حددنا `use crate::front_of_house::hosting` ثم استدعينا `hosting::add_to_waitlist` في `eat_at_restaurant`، بدلاً من تحديد مسار (path) `use` حتى دالة `add_to_waitlist` لتحقيق نفس النتيجة، كما في القائمة 7-13.

<Listing number="7-13" file-name="src/lib.rs" caption="إحضار دالة `add_to_waitlist` إلى النطاق (scope) باستخدام `use`، وهو أمر غير اصطلاحي">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

على الرغم من أن كلًا من القائمة 7-11 والقائمة 7-13 تُنجزان نفس المهمة، فإن القائمة 7-11 هي الطريقة الاصطلاحية لإحضار دالة إلى النطاق (scope) باستخدام `use`. إحضار الوحدة (module) الأب للدالة إلى النطاق (scope) باستخدام `use` يعني أن علينا تحديد الوحدة (module) الأب عند استدعاء الدالة. تحديد الوحدة (module) الأب عند استدعاء الدالة يوضح أن الدالة ليست معرّفة محليًا بينما لا يزال يُقلل من تكرار المسار (path) الكامل. الكود في القائمة 7-13 غير واضح بشأن مكان تعريف `add_to_waitlist`.

من ناحية أخرى، عند إحضار البنى structs والتعدادات enums والعناصر الأخرى باستخدام `use`، من الاصطلاحي تحديد المسار (path) الكامل. توضح القائمة 7-14 الطريقة الاصطلاحية لإحضار بنية `HashMap` من المكتبة المعيارية إلى نطاق (scope) صندوق ثنائي (binary crate).

<Listing number="7-14" file-name="src/main.rs" caption="إحضار `HashMap` إلى النطاق (scope) بطريقة اصطلاحية">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

لا يوجد سبب قوي وراء هذا الاصطلاح: إنه فقط الاتفاقية التي ظهرت، واعتاد الناس على قراءة وكتابة كود Rust بهذه الطريقة.

الاستثناء من هذا الاصطلاح هو إذا كنا نُحضر عنصرين بنفس الاسم إلى النطاق (scope) باستخدام تعليمات `use`، لأن Rust لا يسمح بذلك. توضح القائمة 7-15 كيفية إحضار نوعين `Result` إلى النطاق (scope) لهما نفس الاسم لكن وحدات (modules) أب مختلفة، وكيفية الإشارة إليهما.

<Listing number="7-15" file-name="src/lib.rs" caption="إحضار نوعين بنفس الاسم إلى نفس النطاق (scope) يتطلب استخدام وحداتهما (modules) الأب.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

كما ترى، استخدام الوحدات (modules) الأب يُميز بين نوعي `Result`. إذا حددنا بدلاً من ذلك `use std::fmt::Result` و `use std::io::Result`، سيكون لدينا نوعا `Result` في نفس النطاق (scope)، ولن يعرف Rust أيهما نعني عندما نستخدم `Result`.

### توفير أسماء جديدة باستخدام الكلمة المفتاحية `as`

هناك حل آخر لمشكلة إحضار نوعين بنفس الاسم إلى نفس النطاق (scope) باستخدام `use`: بعد المسار (path)، يمكننا تحديد `as` واسم محلي جديد، أو _اسم مستعار (alias)_، للنوع. توضح القائمة 7-16 طريقة أخرى لكتابة الكود في القائمة 7-15 بإعادة تسمية أحد نوعي `Result` باستخدام `as`.

<Listing number="7-16" file-name="src/lib.rs" caption="إعادة تسمية نوع عند إحضاره إلى النطاق (scope) باستخدام الكلمة المفتاحية `as`">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

في تعليمة `use` الثانية، اخترنا الاسم الجديد `IoResult` لنوع `std::io::Result`، والذي لن يتعارض مع `Result` من `std::fmt` الذي أحضرناه أيضًا إلى النطاق (scope). تُعتبر القائمة 7-15 والقائمة 7-16 اصطلاحيتين، لذا الخيار متروك لك!

### إعادة التصدير باستخدام `pub use`

عندما نُحضر اسمًا إلى النطاق (scope) باستخدام الكلمة المفتاحية `use`، يكون الاسم خاصًا (private) بالنطاق (scope) الذي استوردناه إليه. لتمكين الكود خارج ذلك النطاق (scope) من الإشارة إلى ذلك الاسم كما لو كان معرّفًا في ذلك النطاق (scope)، يمكننا دمج `pub` و `use`. تُسمى هذه التقنية _إعادة التصدير (re-exporting)_ لأننا نُحضر عنصرًا إلى النطاق (scope) ولكن أيضًا نجعل ذلك العنصر متاحًا للآخرين لإحضاره إلى نطاقهم (scope).

توضح القائمة 7-17 الكود في القائمة 7-11 مع تغيير `use` في الوحدة الجذر إلى `pub use`.

<Listing number="7-17" file-name="src/lib.rs" caption="جعل اسم متاحًا لأي كود للاستخدام من نطاق (scope) جديد باستخدام `pub use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

قبل هذا التغيير، كان على الكود الخارجي استدعاء دالة `add_to_waitlist` باستخدام المسار (path) `restaurant::front_of_house::hosting::add_to_waitlist()`، والذي كان سيتطلب أيضًا وضع علامة `pub` على وحدة (module) `front_of_house`. الآن بعد أن أعاد `pub use` تصدير وحدة (module) `hosting` من الوحدة الجذر، يمكن للكود الخارجي استخدام المسار (path) `restaurant::hosting::add_to_waitlist()` بدلاً من ذلك.

إعادة التصدير مفيدة عندما تكون البنية الداخلية لكودك مختلفة عن كيفية تفكير المبرمجين الذين يستدعون كودك حول المجال. على سبيل المثال، في هذا الاستعارة للمطعم، يفكر الأشخاص الذين يديرون المطعم في "الجزء الأمامي" و"الجزء الخلفي". لكن العملاء الذين يزورون مطعمًا ربما لن يفكروا في أجزاء المطعم بتلك المصطلحات. باستخدام `pub use`، يمكننا كتابة كودنا ببنية واحدة ولكن كشف بنية مختلفة. يجعل القيام بذلك مكتبتنا منظمة بشكل جيد للمبرمجين الذين يعملون على المكتبة والمبرمجين الذين يستدعون المكتبة. سننظر في مثال آخر على `pub use` وكيف يؤثر على توثيق صندوقك في ["تصدير واجهة برمجة تطبيقات عامة مريحة"][ch14-pub-use]<!-- ignore --> في الفصل 14.

### استخدام الحزم (Packages) الخارجية

في الفصل 2، برمجنا مشروع لعبة التخمين الذي استخدم حزمة (package) خارجية تُسمى `rand` للحصول على أرقام عشوائية. لاستخدام `rand` في مشروعنا، أضفنا هذا السطر إلى _Cargo.toml_:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

إضافة `rand` كاعتمادية في _Cargo.toml_ تخبر Cargo بتنزيل حزمة (package) `rand` وأي اعتماديات من [crates.io](https://crates.io/) وجعل `rand` متاحة لمشروعنا.

بعد ذلك، لإحضار تعريفات `rand` إلى نطاق (scope) حزمتنا (package)، أضفنا سطر `use` يبدأ باسم الصندوق (crate)، `rand`، وسردنا العناصر التي أردنا إحضارها إلى النطاق (scope). تذكر أنه في ["توليد رقم عشوائي"][rand]<!-- ignore --> في الفصل 2، أحضرنا سمة `Rng` إلى النطاق (scope) واستدعينا دالة `rand::thread_rng`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

أتاح أعضاء مجتمع Rust العديد من الحزم (packages) على [crates.io](https://crates.io/)، وسحب أي منها إلى حزمتك (package) يتضمن نفس الخطوات: إدراجها في ملف _Cargo.toml_ الخاص بحزمتك (package) واستخدام `use` لإحضار العناصر من صناديقها (crates) إلى النطاق (scope).

لاحظ أن المكتبة المعيارية `std` هي أيضًا صندوق (crate) خارجي بالنسبة لحزمتنا (package). لأن المكتبة المعيارية تأتي مع لغة Rust، لا نحتاج إلى تغيير _Cargo.toml_ لتضمين `std`. لكننا نحتاج إلى الإشارة إليها باستخدام `use` لإحضار العناصر من هناك إلى نطاق (scope) حزمتنا (package). على سبيل المثال، مع `HashMap` سنستخدم هذا السطر:

```rust
use std::collections::HashMap;
```

هذا مسار مطلق (absolute path) يبدأ بـ `std`، اسم صندوق (crate) المكتبة المعيارية.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-nested-paths-to-clean-up-large-use-lists"></a>

### استخدام المسارات (Paths) المتداخلة لتنظيف قوائم `use`

إذا كنا نستخدم عناصر متعددة معرّفة في نفس الصندوق (crate) أو نفس الوحدة (module)، فإن إدراج كل عنصر في سطره الخاص يمكن أن يستهلك الكثير من المساحة العمودية في ملفاتنا. على سبيل المثال، تعليمتا `use` اللتان كانتا لدينا في لعبة التخمين في القائمة 2-4 تُحضران عناصر من `std` إلى النطاق (scope):

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

بدلاً من ذلك، يمكننا استخدام المسارات (paths) المتداخلة لإحضار نفس العناصر إلى النطاق (scope) في سطر واحد. نفعل ذلك بتحديد الجزء المشترك من المسار (path)، متبوعًا بنقطتين متتاليتين، ثم أقواس معقوفة حول قائمة بأجزاء المسارات (paths) التي تختلف، كما هو موضح في القائمة 7-18.

<Listing number="7-18" file-name="src/main.rs" caption="تحديد مسار (path) متداخل لإحضار عناصر متعددة بنفس البادئة إلى النطاق (scope)">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

في البرامج الأكبر، إحضار العديد من العناصر إلى النطاق (scope) من نفس الصندوق (crate) أو الوحدة (module) باستخدام المسارات (paths) المتداخلة يمكن أن يُقلل من عدد تعليمات `use` المنفصلة المطلوبة كثيرًا!

يمكننا استخدام مسار (path) متداخل في أي مستوى في مسار (path)، وهو مفيد عند دمج تعليمتي `use` تشتركان في مسار فرعي (path). على سبيل المثال، توضح القائمة 7-19 تعليمتي `use`: واحدة تُحضر `std::io` إلى النطاق (scope) وواحدة تُحضر `std::io::Write` إلى النطاق (scope).

<Listing number="7-19" file-name="src/lib.rs" caption="تعليمتا `use` حيث واحدة مسار فرعي للأخرى">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

الجزء المشترك من هذين المسارين (paths) هو `std::io`، وهذا هو المسار (path) الأول الكامل. لدمج هذين المسارين (paths) في تعليمة `use` واحدة، يمكننا استخدام `self` في المسار (path) المتداخل، كما هو موضح في القائمة 7-20.

<Listing number="7-20" file-name="src/lib.rs" caption="دمج المسارات (paths) في القائمة 7-19 في تعليمة `use` واحدة">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

هذا السطر يُحضر `std::io` و `std::io::Write` إلى النطاق (scope).

<!-- Old headings. Do not remove or links may break. -->

<a id="the-glob-operator"></a>

### استيراد العناصر باستخدام عامل النجمة

إذا أردنا إحضار _جميع_ العناصر العامة (public) المعرّفة في مسار (path) إلى النطاق (scope)، يمكننا تحديد ذلك المسار (path) متبوعًا بعامل النجمة `*`:

```rust
use std::collections::*;
```

تعليمة `use` هذه تُحضر جميع العناصر العامة (public) المعرّفة في `std::collections` إلى النطاق (scope) الحالي. كن حذرًا عند استخدام عامل النجمة! يمكن أن تجعل النجمة من الصعب معرفة الأسماء الموجودة في النطاق (scope) ومن أين تم تعريف اسم مستخدم في برنامجك. بالإضافة إلى ذلك، إذا غيّرت الاعتمادية تعريفاتها، فإن ما استوردته يتغير أيضًا، مما قد يؤدي إلى أخطاء مُصرّف عند ترقية الاعتمادية إذا أضافت الاعتمادية تعريفًا بنفس اسم تعريف لك في نفس النطاق (scope)، على سبيل المثال.

يُستخدم عامل النجمة غالبًا عند الاختبار لإحضار كل شيء قيد الاختبار إلى وحدة (module) `tests`؛ سنتحدث عن ذلك في ["كيفية كتابة الاختبارات"][writing-tests]<!-- ignore --> في الفصل 11. يُستخدم عامل النجمة أحيانًا أيضًا كجزء من نمط prelude: راجع [توثيق المكتبة المعيارية](../std/prelude/index.html#other-preludes)<!-- ignore --> لمزيد من المعلومات حول هذا النمط.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
