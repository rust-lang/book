<!-- Old headings. Do not remove or links may break. -->

<a id="defining-modules-to-control-scope-and-privacy"></a>

## التحكم في النطاق والخصوصية باستخدام الوحدات

في هذا القسم، سنتحدث عن الوحدات (modules) وأجزاء أخرى من نظام الوحدات (module system)، وهي _المسارات (paths)_ التي تسمح لك بتسمية العناصر؛ والكلمة المفتاحية `use` التي تجلب المسار (path) إلى النطاق (scope)؛ والكلمة المفتاحية `pub` لجعل العناصر عامة (public). سنناقش أيضاً الكلمة المفتاحية `as`، والحزم (packages) الخارجية، ومشغل glob.

### ورقة غش الوحدات

قبل أن نصل إلى تفاصيل الوحدات (modules) والمسارات (paths)، نقدم هنا مرجعاً سريعاً حول كيفية عمل الوحدات (modules) والمسارات (paths) والكلمة المفتاحية `use` والكلمة المفتاحية `pub` في المصرِّف، وكيف ينظم معظم المطورين شيفرتهم. سنستعرض أمثلة على كل من هذه القواعد خلال هذا الفصل، لكن هذا مكان رائع للرجوع إليه كتذكير بكيفية عمل الوحدات (modules).

- **ابدأ من جذر الصندوق (crate root)**: عند ترجمة صندوق (crate)، يبحث المصرِّف أولاً في ملف جذر الصندوق (crate root) (عادةً _src/lib.rs_ لصندوق مكتبة (library crate) و _src/main.rs_ لصندوق ثنائي (binary crate)) عن الشيفرة المراد ترجمتها.
- **تعريف الوحدات (modules)**: في ملف جذر الصندوق (crate root)، يمكنك تعريف وحدات (modules) جديدة؛ لنفترض أنك تعرف وحدة (module) "حديقة" بـ `mod garden;`. سيبحث المصرِّف عن شيفرة الوحدة (module) في هذه الأماكن:
  - مضمنة، داخل أقواس معقوفة تستبدل الفاصلة المنقوطة التي تتبع `mod garden`
  - في الملف _src/garden.rs_
  - في الملف _src/garden/mod.rs_
- **تعريف الوحدات الفرعية**: في أي ملف آخر غير جذر الصندوق (crate root)، يمكنك تعريف وحدات فرعية. على سبيل المثال، قد تعرف `mod vegetables;` في _src/garden.rs_. سيبحث المصرِّف عن شيفرة الوحدة الفرعية ضمن الدليل المسمى بوحدة الأصل في هذه الأماكن:
  - مضمنة، مباشرة بعد `mod vegetables`، داخل أقواس معقوفة بدلاً من الفاصلة المنقوطة
  - في الملف _src/garden/vegetables.rs_
  - في الملف _src/garden/vegetables/mod.rs_
- **مسارات (paths) الشيفرة في الوحدات (modules)**: بمجرد أن تصبح الوحدة (module) جزءاً من صندوقك (crate)، يمكنك الإشارة إلى الشيفرة في تلك الوحدة (module) من أي مكان آخر في نفس الصندوق (crate)، طالما أن قواعد الخصوصية (privacy) تسمح بذلك، باستخدام المسار (path) إلى الشيفرة. على سبيل المثال، النوع `Asparagus` في وحدة (module) خضروات الحديقة سيكون موجوداً في `crate::garden::vegetables::Asparagus`.
- **خاص (private) مقابل عام (public)**: الشيفرة داخل الوحدة (module) خاصة (private) من وحدات (modules) أصلها بشكل افتراضي. لجعل الوحدة (module) عامة (public)، عرفها بـ `pub mod` بدلاً من `mod`. لجعل العناصر داخل وحدة (module) عامة (public) عامة (public) أيضاً، استخدم `pub` قبل تعريفاتها.
- **الكلمة المفتاحية `use`**: ضمن نطاق (scope)، تنشئ الكلمة المفتاحية `use` اختصارات للعناصر لتقليل تكرار المسارات (paths) الطويلة. في أي نطاق (scope) يمكن أن يشير إلى `crate::garden::vegetables::Asparagus`، يمكنك إنشاء اختصار بـ `use crate::garden::vegetables::Asparagus;`، ومن ثم تحتاج فقط لكتابة `Asparagus` لاستخدام هذا النوع في النطاق (scope).

هنا، ننشئ صندوقاً ثنائياً (binary crate) باسم `backyard` يوضح هذه القواعد. دليل الصندوق (crate)، المسمى أيضاً _backyard_، يحتوي على هذه الملفات والأدلة:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

ملف جذر الصندوق (crate root) في هذه الحالة هو _src/main.rs_، ويحتوي على:

<Listing file-name="src/main.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

</Listing>

السطر `pub mod garden;` يخبر المصرِّف بتضمين الشيفرة التي يجدها في _src/garden.rs_، والتي هي:

<Listing file-name="src/garden.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

</Listing>

هنا، `pub mod vegetables;` يعني أن الشيفرة في _src/garden/vegetables.rs_ مضمنة أيضاً. تلك الشيفرة هي:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

الآن لندخل في تفاصيل هذه القواعد ونوضحها عملياً!

### ترجمة الشيفرة ذات الصلة في الوحدات

تتيح لنا _الوحدات (modules)_ تنظيم الشيفرة داخل الصندوق (crate) من أجل القراءة وسهولة إعادة الاستخدام. تسمح لنا الوحدات (modules) أيضاً بالتحكم في _خصوصية (privacy)_ العناصر لأن الشيفرة داخل الوحدة (module) خاصة (private) بشكل افتراضي. العناصر الخاصة (private) هي تفاصيل تنفيذ داخلية غير متاحة للاستخدام الخارجي. يمكننا اختيار جعل الوحدات (modules) والعناصر داخلها عامة (public)، مما يعرضها للسماح للشيفرة الخارجية باستخدامها والاعتماد عليها.

كمثال، لنكتب صندوق مكتبة (library crate) يوفر وظيفة مطعم. سنعرف توقيعات الدوال لكن سنترك أجسامها فارغة للتركيز على تنظيم الشيفرة بدلاً من تنفيذ المطعم.

في صناعة المطاعم، يُشار إلى بعض أجزاء المطعم بـ front of house والبعض الآخر بـ back of house. _Front of house_ هو حيث يتواجد العملاء؛ وهذا يشمل حيث يجلس المضيفون العملاء، ويأخذ النوادل الطلبات والدفع، ويصنع السقاة المشروبات. _Back of house_ هو حيث يعمل الطهاة والطباخون في المطبخ، وينظف غاسلو الصحون، ويقوم المديرون بالعمل الإداري.

لهيكلة صندوقنا (crate) بهذه الطريقة، يمكننا تنظيم دواله في وحدات (modules) متداخلة. أنشئ مكتبة جديدة باسم `restaurant` بتشغيل `cargo new restaurant --lib`. ثم، أدخل الشيفرة في القائمة 7-1 في _src/lib.rs_ لتعريف بعض الوحدات (modules) وتوقيعات الدوال؛ هذه الشيفرة هي قسم front of house.

<Listing number="7-1" file-name="src/lib.rs" caption="وحدة (module) `front_of_house` تحتوي على وحدات (modules) أخرى تحتوي بدورها على دوال">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

</Listing>

نعرف وحدة (module) بالكلمة المفتاحية `mod` متبوعة باسم الوحدة (module) (في هذه الحالة، `front_of_house`). ثم يذهب جسم الوحدة (module) داخل أقواس معقوفة. داخل الوحدات (modules)، يمكننا وضع وحدات (modules) أخرى، كما في هذه الحالة مع الوحدات (modules) `hosting` و`serving`. يمكن أن تحتوي الوحدات (modules) أيضاً على تعريفات لعناصر أخرى، مثل البنيات، والعدديات، والثوابت، والخصائص، وكما في القائمة 7-1، الدوال.

باستخدام الوحدات (modules)، يمكننا ترجمة التعريفات ذات الصلة معاً وتسمية سبب ترابطها. يمكن للمبرمجين الذين يستخدمون هذه الشيفرة التنقل في الشيفرة بناءً على المجموعات بدلاً من الاضطرار إلى القراءة من خلال جميع التعريفات، مما يسهل العثور على التعريفات ذات الصلة بهم. سيعرف المبرمجون الذين يضيفون وظائف جديدة إلى هذه الشيفرة أين يضعون الشيفرة للحفاظ على تنظيم البرنامج.

في وقت سابق، ذكرنا أن _src/main.rs_ و _src/lib.rs_ تُسمى _جذور الصندوق (crate roots)_. سبب تسميتها هو أن محتويات أي من هذين الملفين يشكل وحدة (module) باسم `crate` في جذر بنية وحدة (module) الصندوق (crate)، المعروفة باسم _شجرة الوحدات (module tree)_.

تُظهر القائمة 7-2 شجرة الوحدات (module tree) للبنية في القائمة 7-1.

<Listing number="7-2" caption="شجرة الوحدات (module tree) للشيفرة في القائمة 7-1">

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

</Listing>

تُظهر هذه الشجرة كيف تتداخل بعض الوحدات (modules) داخل وحدات (modules) أخرى؛ على سبيل المثال، `hosting` تتداخل داخل `front_of_house`. تُظهر الشجرة أيضاً أن بعض الوحدات (modules) هي _أشقاء_، مما يعني أنها معرفة في نفس الوحدة (module)؛ `hosting` و`serving` هما أشقاء معرفان ضمن `front_of_house`. إذا كانت الوحدة (module) A موجودة داخل الوحدة (module) B، نقول أن الوحدة (module) A هي _ابن_ للوحدة (module) B وأن الوحدة (module) B هي _أصل_ للوحدة (module) A. لاحظ أن شجرة الوحدات (module tree) بأكملها متجذرة تحت الوحدة (module) الضمنية باسم `crate`.

قد تذكرك شجرة الوحدات (module tree) بشجرة دليل نظام الملفات على جهاز الكمبيوتر الخاص بك؛ هذه مقارنة مناسبة جداً! تماماً مثل الأدلة في نظام الملفات، تستخدم الوحدات (modules) لتنظيم شيفرتك. وتماماً مثل الملفات في دليل، نحتاج إلى طريقة للعثور على وحداتنا (modules).
