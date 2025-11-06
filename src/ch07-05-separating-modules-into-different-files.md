## فصل الوحدات إلى ملفات مختلفة

حتى الآن، جميع الأمثلة في هذا الفصل عرّفت وحدات (modules) متعددة في ملف واحد. عندما تصبح الوحدات (modules) كبيرة، قد ترغب في نقل تعريفاتها إلى ملف منفصل لجعل الكود أسهل في التنقل.

على سبيل المثال، لنبدأ من الكود في القائمة 7-17 الذي كان يحتوي على وحدات (modules) مطعم متعددة. سنستخرج الوحدات (modules) إلى ملفات بدلاً من تعريف جميع الوحدات (modules) في ملف جذر الصندوق (crate root). في هذه الحالة، ملف جذر الصندوق (crate root) هو _src/lib.rs_، لكن هذا الإجراء يعمل أيضاً مع الصناديق الثنائية (binary crates) التي يكون ملف جذرها _src/main.rs_.

أولاً، سنستخرج وحدة (module) `front_of_house` إلى ملفها الخاص. احذف الكود داخل الأقواس المعقوفة لوحدة (module) `front_of_house`، مع ترك تصريح `mod front_of_house;` فقط، بحيث يحتوي _src/lib.rs_ على الكود الموضح في القائمة 7-21. لاحظ أن هذا لن يُترجم حتى ننشئ ملف _src/front_of_house.rs_ في القائمة 7-22.

<Listing number="7-21" file-name="src/lib.rs" caption="تصريح وحدة (module) `front_of_house` التي سيكون جسمها في *src/front_of_house.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

</Listing>

بعد ذلك، ضع الكود الذي كان في الأقواس المعقوفة في ملف جديد يُسمى _src/front_of_house.rs_، كما هو موضح في القائمة 7-22. يعرف المصرِّف أن يبحث في هذا الملف لأنه صادف تصريح الوحدة (module) في جذر الصندوق (crate root) بالاسم `front_of_house`.

<Listing number="7-22" file-name="src/front_of_house.rs" caption="التعريفات داخل وحدة (module) `front_of_house` في *src/front_of_house.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

</Listing>

لاحظ أنك تحتاج فقط لتحميل ملف باستخدام تصريح `mod` _مرة واحدة_ في شجرة الوحدات (module tree) الخاصة بك. بمجرد أن يعرف المصرِّف أن الملف جزء من المشروع (ويعرف أين في شجرة الوحدات (module tree) يقع الكود بسبب مكان وضعك لجملة `mod`)، يجب أن تشير الملفات الأخرى في مشروعك إلى كود الملف المُحمَّل باستخدام مسار (path) إلى حيث تم تصريحه، كما تم تغطيته في قسم ["مسارات (paths) للإشارة إلى عنصر في شجرة الوحدات (module tree)"][paths]<!-- ignore -->. بعبارة أخرى، `mod` _ليست_ عملية "تضمين" ربما رأيتها في لغات البرمجة الأخرى.

بعد ذلك، سنستخرج وحدة (module) `hosting` إلى ملفها الخاص. العملية مختلفة قليلاً لأن `hosting` هي وحدة فرعية من `front_of_house`، وليست من الوحدة الجذرية. سنضع ملف `hosting` في دليل جديد سيُسمى على اسم أسلافه في شجرة الوحدات (module tree)، في هذه الحالة _src/front_of_house_.

لبدء نقل `hosting`، نغيّر _src/front_of_house.rs_ ليحتوي فقط على تصريح وحدة (module) `hosting`:

<Listing file-name="src/front_of_house.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

</Listing>

ثم، ننشئ دليل _src/front_of_house_ وملف _hosting.rs_ ليحتوي على التعريفات الموجودة في وحدة (module) `hosting`:

<Listing file-name="src/front_of_house/hosting.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

</Listing>

إذا وضعنا بدلاً من ذلك _hosting.rs_ في دليل _src_، سيتوقع المصرِّف أن كود _hosting.rs_ موجود في وحدة (module) `hosting` مُصرَّح بها في جذر الصندوق (crate root) وليس مُصرَّح بها كوحدة فرعية من وحدة (module) `front_of_house`. قواعد المصرِّف حول أي ملفات يجب التحقق منها لكود أي وحدات (modules) تعني أن الأدلة والملفات تطابق شجرة الوحدات (module tree) بشكل أوثق.

> ### مسارات ملفات بديلة
>
> حتى الآن قمنا بتغطية مسارات الملفات الأكثر شيوعاً التي يستخدمها مصرِّف Rust، لكن Rust يدعم أيضاً أسلوباً قديماً لمسار الملف. بالنسبة لوحدة (module) تُسمى `front_of_house` مُصرَّح بها في جذر الصندوق (crate root)، سيبحث المصرِّف عن كود الوحدة (module) في:
>
> - _src/front_of_house.rs_ (ما قمنا بتغطيته)
> - _src/front_of_house/mod.rs_ (أسلوب قديم، مسار لا يزال مدعوماً)
>
> بالنسبة لوحدة (module) تُسمى `hosting` والتي هي وحدة فرعية من `front_of_house`، سيبحث المصرِّف عن كود الوحدة (module) في:
>
> - _src/front_of_house/hosting.rs_ (ما قمنا بتغطيته)
> - _src/front_of_house/hosting/mod.rs_ (أسلوب قديم، مسار لا يزال مدعوماً)
>
> إذا استخدمت كلا الأسلوبين لنفس الوحدة (module)، ستحصل على خطأ في المصرِّف. استخدام مزيج من كلا الأسلوبين لوحدات (modules) مختلفة في نفس المشروع مسموح به لكنه قد يكون مربكاً للأشخاص الذين يتنقلون في مشروعك.
>
> العيب الرئيسي للأسلوب الذي يستخدم ملفات تُسمى _mod.rs_ هو أن مشروعك قد ينتهي به الأمر بالعديد من الملفات المسماة _mod.rs_، والتي يمكن أن تصبح مربكة عندما تفتحها في محررك في نفس الوقت.

لقد نقلنا كود كل وحدة (module) إلى ملف منفصل، وشجرة الوحدات (module tree) تبقى كما هي. استدعاءات الدالة في `eat_at_restaurant` ستعمل دون أي تعديل، حتى لو كانت التعريفات موجودة في ملفات مختلفة. هذه التقنية تسمح لك بنقل الوحدات (modules) إلى ملفات جديدة مع نموها في الحجم.

لاحظ أن جملة `pub use crate::front_of_house::hosting` في _src/lib.rs_ لم تتغير أيضاً، ولا يكون لـ `use` أي تأثير على أي ملفات يتم ترجمتها كجزء من الصندوق (crate). الكلمة المفتاحية `mod` تُصرِّح الوحدات (modules)، وRust يبحث في ملف بنفس اسم الوحدة (module) عن الكود الذي يدخل في تلك الوحدة (module).

## ملخص

Rust يتيح لك تقسيم حزمة (package) إلى صناديق (crates) متعددة وصندوق (crate) إلى وحدات (modules) بحيث يمكنك الإشارة إلى عناصر معرّفة في وحدة (module) من وحدة (module) أخرى. يمكنك القيام بذلك عن طريق تحديد مسارات (paths) مطلقة أو نسبية. يمكن إحضار هذه المسارات (paths) إلى النطاق (scope) باستخدام جملة `use` بحيث يمكنك استخدام مسار (path) أقصر لاستخدامات متعددة للعنصر في ذلك النطاق (scope). كود الوحدة (module) خاص (private) بشكل افتراضي، لكن يمكنك جعل التعريفات عامة (public) بإضافة الكلمة المفتاحية `pub`.

في الفصل التالي، سننظر في بعض هياكل بيانات المجموعات في المكتبة القياسية التي يمكنك استخدامها في كودك المنظم بشكل جيد.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
