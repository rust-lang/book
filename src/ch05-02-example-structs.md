## برنامج مثال باستخدام الهياكل

لفهم متى قد نرغب في استخدام الهياكل (structs)، دعنا نكتب برنامجاً يحسب مساحة مستطيل. سنبدأ باستخدام متغيرات فردية ثم سنعيد هيكلة البرنامج حتى نستخدم الهياكل (structs) بدلاً من ذلك.

لنُنشئ مشروعاً ثنائياً جديداً باستخدام Cargo اسمه _rectangles_ والذي سيأخذ عرض وارتفاع مستطيل محدد بالبكسل ويحسب مساحة المستطيل. يوضح Listing 5-8 برنامجاً قصيراً بطريقة واحدة للقيام بذلك بالضبط في _src/main.rs_ الخاص بمشروعنا.

<Listing number="5-8" file-name="src/main.rs" caption="حساب مساحة مستطيل محدد بمتغيرات عرض وارتفاع منفصلة">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

</Listing>

الآن، قم بتشغيل هذا البرنامج باستخدام `cargo run`:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

ينجح هذا الكود في معرفة مساحة المستطيل عن طريق استدعاء دالة `area` مع كل بُعد، لكن يمكننا فعل المزيد لجعل هذا الكود واضحاً وقابلاً للقراءة.

المشكلة في هذا الكود واضحة في توقيع `area`:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

من المفترض أن تحسب دالة `area` مساحة مستطيل واحد، لكن الدالة التي كتبناها لها معاملان، وليس واضحاً في أي مكان في برنامجنا أن المعاملين مرتبطان. سيكون أكثر قابلية للقراءة وأكثر قابلية للإدارة تجميع العرض والارتفاع معاً. لقد ناقشنا بالفعل طريقة واحدة قد نفعل بها ذلك في قسم [â€œنوع المجموعةâ€][the-tuple-type]<!-- ignore --> من الفصل 3: باستخدام المجموعات.

### إعادة الهيكلة باستخدام المجموعات

يُظهر Listing 5-9 نسخة أخرى من برنامجنا تستخدم المجموعات.

<Listing number="5-9" file-name="src/main.rs" caption="تحديد عرض وارتفاع المستطيل باستخدام مجموعة">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

</Listing>

بطريقة ما، هذا البرنامج أفضل. تتيح لنا المجموعات إضافة بعض البنية، ونحن الآن نمرر وسيطة واحدة فقط. لكن بطريقة أخرى، هذه النسخة أقل وضوحاً: المجموعات لا تسمي عناصرها، لذا يتعين علينا الفهرسة إلى أجزاء المجموعة، مما يجعل حسابنا أقل وضوحاً.

الخلط بين العرض والارتفاع لن يهم لحساب المساحة، ولكن إذا أردنا رسم المستطيل على الشاشة، فسيكون مهماً! سيتعين علينا أن نتذكر أن `width` هو فهرس المجموعة `0` و `height` هو فهرس المجموعة `1`. سيكون هذا أصعب على شخص آخر لمعرفته وتذكره إذا كان سيستخدم كودنا. نظراً لأننا لم ننقل معنى بياناتنا في كودنا، فمن الأسهل الآن إدخال أخطاء.

<!-- Old headings. Do not remove or links may break. -->

<a id="refactoring-with-structs-adding-more-meaning"></a>

### إعادة الهيكلة باستخدام الهياكل

نستخدم الهياكل (structs) لإضافة معنى من خلال تصنيف البيانات. يمكننا تحويل المجموعة (tuple) التي نستخدمها إلى هيكل (struct) باسم للكل بالإضافة إلى أسماء للأجزاء، كما هو موضح في Listing 5-10.

<Listing number="5-10" file-name="src/main.rs" caption="تعريف هيكل `Rectangle`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

</Listing>

هنا، قمنا بتعريف هيكل (struct) وسميناه `Rectangle`. داخل الأقواس المعقوفة، عرّفنا الحقول (fields) كـ `width` و `height`، وكلاهما من النوع `u32`. ثم، في `main`، أنشأنا نسخة (instance) معينة من `Rectangle` لها عرض `30` وارتفاع `50`.

دالتنا (function) `area` الآن معرّفة بمعامل واحد، سميناه `rectangle`، ونوعه هو استعارة (borrow) غير قابلة للتغيير (immutable borrow) لنسخة (instance) من هيكل (struct) `Rectangle`. كما ذُكر في الفصل 4، نريد استعارة (borrow) الهيكل (struct) بدلاً من أخذ ملكيته (ownership). بهذه الطريقة، تحتفظ `main` بملكيتها (ownership) ويمكنها الاستمرار في استخدام `rect1`، وهو السبب في استخدامنا لـ `&` في توقيع الدالة (function signature) وحيث نستدعي الدالة (function).

تصل دالة (function) `area` إلى حقلي (fields) `width` و `height` من نسخة (instance) `Rectangle` (لاحظ أن الوصول إلى حقول (fields) نسخة (instance) هيكل (struct) مستعارة (borrowed) لا ينقل قيم الحقول (fields)، وهذا هو السبب في أنك غالباً ما ترى استعارات (borrows) الهياكل (structs)). توقيع دالتنا (function signature) لـ `area` يقول الآن بالضبط ما نعنيه: احسب مساحة `Rectangle`، باستخدام حقلي (fields) `width` و `height`. هذا ينقل أن العرض والارتفاع مرتبطان ببعضهما البعض، ويعطي أسماء وصفية للقيم بدلاً من استخدام قيم فهرس (index) المجموعة (tuple) `0` و `1`. هذا فوز من أجل الوضوح.

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-useful-functionality-with-derived-traits"></a>

### إضافة الوظائف باستخدام الخصائص المشتقة

سيكون من المفيد أن نكون قادرين على طباعة نسخة (instance) من `Rectangle` أثناء تصحيح أخطاء برنامجنا ورؤية القيم لجميع حقولها (fields). يحاول Listing 5-11 استخدام [ماكرو `println!`][println]<!-- ignore --> كما استخدمناه في الفصول السابقة. ومع ذلك، لن ينجح هذا.

<Listing number="5-11" file-name="src/main.rs" caption="محاولة طباعة نسخة `Rectangle`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

</Listing>

عندما نقوم بترجمة هذا الكود، نحصل على خطأ بهذه الرسالة الأساسية:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

يمكن لماكرو `println!` القيام بأنواع عديدة من التنسيق، وبشكل افتراضي، تخبر الأقواس المعقوفة `println!` باستخدام التنسيق المعروف باسم `Display`: مخرجات مخصصة للاستهلاك المباشر للمستخدم النهائي. الأنواع البدائية التي رأيناها حتى الآن تنفذ `Display` بشكل افتراضي لأن هناك طريقة واحدة فقط قد تريد بها إظهار `1` أو أي نوع بدائي آخر للمستخدم. ولكن مع الهياكل (structs)، فإن الطريقة التي يجب أن ينسق بها `println!` المخرجات أقل وضوحاً لأن هناك المزيد من إمكانيات العرض: هل تريد فواصل أم لا؟ هل تريد طباعة الأقواس المعقوفة؟ هل يجب إظهار جميع الحقول (fields)؟ بسبب هذا الغموض، لا تحاول Rust تخمين ما نريد، والهياكل (structs) ليس لديها تطبيق مُوفَّر لـ `Display` لاستخدامه مع `println!` والعنصر النائب `{}`.

إذا استمررنا في قراءة الأخطاء، سنجد هذه الملاحظة المفيدة:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

لنجربها! ستبدو استدعاء ماكرو `println!` الآن كـ `println!("rect1 is {rect1:?}");`. وضع المحدد `:?` داخل الأقواس المعقوفة يخبر `println!` أننا نريد استخدام تنسيق مخرجات يسمى `Debug`. تمكننا خاصية (trait) `Debug` من طباعة هيكلنا (struct) بطريقة مفيدة للمطورين بحيث يمكننا رؤية قيمته أثناء تصحيح أخطاء كودنا.

قم بترجمة الكود مع هذا التغيير. يا للأسف! ما زلنا نحصل على خطأ:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

لكن مرة أخرى، يعطينا المصرِّف ملاحظة مفيدة:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Rust _تتضمن_ وظائف لطباعة معلومات التصحيح، ولكن يتعين علينا صراحة الاشتراك لإتاحة هذه الوظيفة لهيكلنا (struct). للقيام بذلك، نضيف السمة الخارجية (attribute) `#[derive(Debug)]` قبل تعريف الهيكل (struct) مباشرة، كما هو موضح في Listing 5-12.

<Listing number="5-12" file-name="src/main.rs" caption="إضافة السمة لاشتقاق خاصية `Debug` وطباعة نسخة `Rectangle` باستخدام تنسيق التصحيح">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

</Listing>

الآن عندما نشغل البرنامج، لن نحصل على أي أخطاء، وسنرى المخرجات التالية:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

رائع! ليست المخرجات الأجمل، لكنها تُظهر قيم جميع الحقول (fields) لهذه النسخة (instance)، مما سيساعد بالتأكيد أثناء التصحيح. عندما يكون لدينا هياكل (structs) أكبر، يكون من المفيد الحصول على مخرجات أسهل قليلاً للقراءة؛ في تلك الحالات، يمكننا استخدام `{:#?}` بدلاً من `{:?}` في سلسلة `println!`. في هذا المثال، سيؤدي استخدام نمط `{:#?}` إلى إخراج ما يلي:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

طريقة أخرى لطباعة قيمة باستخدام تنسيق `Debug` هي استخدام [ماكرو `dbg!`][dbg]<!-- ignore -->، الذي يأخذ ملكية (ownership) تعبير (على عكس `println!`، الذي يأخذ مرجعاً (reference))، يطبع الملف ورقم السطر حيث يحدث استدعاء ماكرو `dbg!` في كودك مع القيمة الناتجة لذلك التعبير، ويعيد ملكية (ownership) القيمة.

> ملاحظة: استدعاء ماكرو `dbg!` يطبع إلى تدفق وحدة التحكم للخطأ القياسي (`stderr`)، على عكس `println!`، الذي يطبع إلى تدفق وحدة التحكم للإخراج القياسي (`stdout`). سنتحدث أكثر عن `stderr` و `stdout` في [قسم â€œإعادة توجيه الأخطاء إلى الخطأ القياسيâ€ في الفصل 12][err]<!-- ignore -->.

هذا مثال حيث نهتم بالقيمة التي يتم تعيينها لحقل (field) `width`، بالإضافة إلى قيمة الهيكل (struct) بأكمله في `rect1`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

يمكننا وضع `dbg!` حول التعبير `30 * scale` ولأن `dbg!` تعيد ملكية (ownership) قيمة التعبير، فإن حقل (field) `width` سيحصل على نفس القيمة كما لو لم يكن لدينا استدعاء `dbg!` هناك. لا نريد أن يأخذ `dbg!` ملكية (ownership) `rect1`، لذا نستخدم مرجعاً (reference) لـ `rect1` في الاستدعاء التالي. هذا ما يبدو عليه مخرج هذا المثال:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

يمكننا رؤية الجزء الأول من المخرجات جاء من _src/main.rs_ السطر 10 حيث نصحح التعبير `30 * scale`، وقيمته الناتجة هي `60` (تنسيق `Debug` المنفذ للأعداد الصحيحة هو طباعة قيمتها فقط). استدعاء `dbg!` في السطر 14 من _src/main.rs_ يخرج قيمة `&rect1`، وهي هيكل `Rectangle`. تستخدم هذه المخرجات تنسيق `Debug` الجميل لنوع `Rectangle`. يمكن أن يكون ماكرو `dbg!` مفيداً حقاً عندما تحاول معرفة ما يفعله كودك!

بالإضافة إلى خاصية (trait) `Debug`، قدمت Rust عدداً من الخصائص (traits) لنا لاستخدامها مع سمة (attribute) `derive` التي يمكن أن تضيف سلوكاً مفيداً لأنواعنا المخصصة. تلك الخصائص (traits) وسلوكياتها مذكورة في [الملحق C][app-c]<!-- ignore -->. سنغطي كيفية تنفيذ هذه الخصائص (traits) بسلوك مخصص بالإضافة إلى كيفية إنشاء خصائصك (traits) الخاصة في الفصل 10. هناك أيضاً العديد من السمات (attributes) الأخرى غير `derive`؛ لمزيد من المعلومات، راجع [قسم â€œالسماتâ€ من مرجع Rust][attributes].

دالتنا (function) `area` محددة جداً: إنها تحسب فقط مساحة المستطيلات. سيكون من المفيد ربط هذا السلوك بشكل أوثق بهيكل (struct) `Rectangle` الخاص بنا لأنه لن يعمل مع أي نوع آخر. دعنا ننظر في كيف يمكننا الاستمرار في إعادة هيكلة هذا الكود عن طريق تحويل دالة (function) `area` إلى طريقة (method) `area` معرفة على نوع `Rectangle` الخاص بنا.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
