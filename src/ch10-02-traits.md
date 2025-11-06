<!-- Old headings. Do not remove or links may break. -->

<a id="traits-defining-shared-behavior"></a>

## تعريف السلوك المشترك باستخدام السمات

_السمة_ (trait) تحدد الوظيفة التي يمتلكها نوع معين ويمكن مشاركتها مع أنواع أخرى. يمكننا استخدام السمات (traits) لتعريف السلوك المشترك بطريقة مجردة (abstract). يمكننا استخدام _حدود السمات_ (trait bounds) لتحديد أن النوع العام (generic type) يمكن أن يكون أي نوع له سلوك معين.

> ملاحظة: السمات مشابهة لميزة تُسمى غالبًا _الواجهات_ (interfaces) في اللغات الأخرى، على الرغم من وجود بعض الاختلافات.

### تعريف سمة (trait)

يتكون سلوك النوع من الدوال التي يمكننا استدعاؤها على ذلك النوع. تشترك الأنواع المختلفة في نفس السلوك إذا كان بإمكاننا استدعاء نفس الدوال على كل تلك الأنواع. تعريفات السمات (trait) هي طريقة لتجميع توقيعات الدوال معًا لتحديد مجموعة من السلوكيات اللازمة لتحقيق غرض ما.

على سبيل المثال، لنفترض أن لدينا عدة بنيات (structs) تحتوي على أنواع وكميات مختلفة من النصوص: بنية `NewsArticle` التي تحتوي على قصة إخبارية مودعة في موقع معين وبنية `SocialPost` التي يمكن أن تحتوي، كحد أقصى، على 280 حرفًا إلى جانب البيانات الوصفية التي تشير إلى ما إذا كانت منشورًا جديدًا أو إعادة نشر أو ردًا على منشور آخر.

نريد إنشاء مكتبة ترجمة وسائط تُسمى `aggregator` يمكنها عرض ملخصات للبيانات التي قد تكون مخزنة في نسخة من `NewsArticle` أو `SocialPost`. للقيام بذلك، نحتاج إلى ملخص من كل نوع، وسنطلب ذلك الملخص عن طريق استدعاء دالة `summarize` على نسخة. يُظهر القائمة 10-12 تعريف سمة (trait) `Summary` العامة التي تعبر عن هذا السلوك.

<Listing number="10-12" file-name="src/lib.rs" caption="سمة `Summary` التي تتكون من السلوك المقدم من دالة `summarize`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

</Listing>

هنا، نعلن عن سمة (trait) باستخدام الكلمة المفتاحية `trait` ثم اسم السمة (trait)، وهو `Summary` في هذه الحالة. نعلن أيضًا السمة (trait) كـ `pub` حتى تتمكن الصناديق التي تعتمد على هذا الصندوق من استخدام هذه السمة (trait) أيضًا، كما سنرى في بعض الأمثلة. داخل الأقواس المعقوفة، نعلن عن توقيعات الدوال التي تصف سلوكيات الأنواع التي تنفذ (implementation) هذه السمة (trait)، والتي في هذه الحالة هي `fn summarize(&self) -> String`.

بعد توقيع الدالة، بدلاً من توفير تنفيذ (implementation) داخل الأقواس المعقوفة، نستخدم فاصلة منقوطة. يجب على كل نوع ينفذ (implementation) هذه السمة (trait) أن يوفر سلوكه المخصص لجسم الدالة. سيفرض المصرِّف أن أي نوع له سمة (trait) `Summary` سيكون لديه الدالة `summarize` معرفة بهذا التوقيع بالضبط.

يمكن أن تحتوي السمة (trait) على دوال متعددة في جسمها: يتم سرد توقيعات الدوال واحدة في كل سطر، وكل سطر ينتهي بفاصلة منقوطة.

### تنفيذ (implementation) سمة (trait) على نوع

الآن بعد أن عرّفنا التوقيعات المطلوبة لدوال سمة (trait) `Summary`، يمكننا تنفيذها (implementation) على الأنواع في مجمع الوسائط الخاص بنا. توضح القائمة 10-13 تنفيذ (implementation) سمة (trait) `Summary` على بنية `NewsArticle` التي تستخدم العنوان الرئيسي والكاتب والموقع لإنشاء القيمة المرجعة لـ `summarize`. بالنسبة لبنية `SocialPost`، نعرّف `summarize` على أنه اسم المستخدم متبوعًا بالنص الكامل للمنشور، بافتراض أن محتوى المنشور محدود بالفعل بـ 280 حرفًا.

<Listing number="10-13" file-name="src/lib.rs" caption="تنفيذ سمة `Summary` على أنواع `NewsArticle` و `SocialPost`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

</Listing>

تنفيذ (implementation) سمة (trait) على نوع مشابه لتنفيذ (implementation) الدوال العادية. الفرق هو أنه بعد `impl`، نضع اسم السمة (trait) التي نريد تنفيذها (implementation)، ثم نستخدم الكلمة المفتاحية `for`، ثم نحدد اسم النوع الذي نريد تنفيذ (implementation) السمة (trait) له. داخل كتلة `impl`، نضع توقيعات الدوال التي عرّفها تعريف السمة (trait). بدلاً من إضافة فاصلة منقوطة بعد كل توقيع، نستخدم الأقواس المعقوفة ونملأ جسم الدالة بالسلوك المحدد الذي نريده لدوال السمة (trait) لهذا النوع المعين.

الآن بعد أن نفذت (implementation) المكتبة سمة (trait) `Summary` على `NewsArticle` و `SocialPost`، يمكن لمستخدمي الصندوق استدعاء دوال السمة (trait) على نسخ من `NewsArticle` و `SocialPost` بنفس الطريقة التي نستدعي بها الدوال العادية. الفرق الوحيد هو أن المستخدم يجب أن يجلب السمة (trait) إلى النطاق بالإضافة إلى الأنواع. إليك مثالاً على كيفية استخدام صندوق ثنائي لمكتبة `aggregator` الخاصة بنا:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

يطبع هذا الكود `1 new post: horse_ebooks: of course, as you probably already know, people`.

يمكن أيضًا للصناديق الأخرى التي تعتمد على صندوق `aggregator` جلب سمة (trait) `Summary` إلى النطاق لتنفيذ (implementation) `Summary` على أنواعها الخاصة. أحد القيود (bounds) التي يجب ملاحظتها هو أنه يمكننا تنفيذ (implementation) سمة (trait) على نوع فقط إذا كانت السمة (trait) أو النوع، أو كليهما، محليين لصندوقنا. على سبيل المثال، يمكننا تنفيذ (implementation) سمات (traits) المكتبة القياسية مثل `Display` على نوع مخصص مثل `SocialPost` كجزء من وظائف صندوق `aggregator` لأن النوع `SocialPost` محلي لصندوق `aggregator` الخاص بنا. يمكننا أيضًا تنفيذ (implementation) `Summary` على `Vec<T>` في صندوق `aggregator` لأن سمة (trait) `Summary` محلية لصندوق `aggregator` الخاص بنا.

لكن لا يمكننا تنفيذ (implementation) سمات (traits) خارجية على أنواع خارجية. على سبيل المثال، لا يمكننا تنفيذ (implementation) سمة (trait) `Display` على `Vec<T>` داخل صندوق `aggregator`، لأن `Display` و `Vec<T>` كلاهما معرفان في المكتبة القياسية وليسا محليين لصندوق `aggregator` الخاص بنا. هذا القيد (bound) جزء من خاصية تسمى _التماسك_ (coherence)، وبشكل أكثر تحديدًا _قاعدة اليتيم_ (orphan rule)، سميت كذلك لأن النوع الأصلي غير موجود. تضمن هذه القاعدة أن كود الأشخاص الآخرين لا يمكنه كسر كودك والعكس صحيح. بدون القاعدة، يمكن لصندوقين تنفيذ (implementation) نفس السمة (trait) لنفس النوع، ولن تعرف Rust أي تنفيذ (implementation) تستخدم.

<!-- Old headings. Do not remove or links may break. -->

<a id="default-implementations"></a>

### استخدام التنفيذات (implementations) الافتراضية

في بعض الأحيان يكون من المفيد وجود سلوك افتراضي لبعض أو كل الدوال في سمة (trait) بدلاً من طلب تنفيذات (implementations) لجميع الدوال على كل نوع. ثم، عندما ننفذ (implementation) السمة (trait) على نوع معين، يمكننا الاحتفاظ بالسلوك الافتراضي لكل دالة أو تجاوزه.

في القائمة 10-14، نحدد نصًا افتراضيًا لدالة `summarize` من سمة (trait) `Summary` بدلاً من تعريف توقيع الدالة فقط، كما فعلنا في القائمة 10-12.

<Listing number="10-14" file-name="src/lib.rs" caption="تعريف سمة `Summary` مع تنفيذ افتراضي لدالة `summarize`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

</Listing>

لاستخدام تنفيذ (implementation) افتراضي لتلخيص نسخ من `NewsArticle`، نحدد كتلة `impl` فارغة بـ `impl Summary for NewsArticle {}`.

على الرغم من أننا لم نعد نعرّف دالة `summarize` على `NewsArticle` مباشرة، فقد قدمنا تنفيذًا (implementation) افتراضيًا وحددنا أن `NewsArticle` ينفذ (implementation) سمة (trait) `Summary`. ونتيجة لذلك، لا يزال بإمكاننا استدعاء دالة `summarize` على نسخة من `NewsArticle`، مثل هذا:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

يطبع هذا الكود `New article available! (Read more...)`.

إنشاء تنفيذ (implementation) افتراضي لا يتطلب منا تغيير أي شيء حول تنفيذ (implementation) `Summary` على `SocialPost` في القائمة 10-13. السبب هو أن صيغة تجاوز تنفيذ (implementation) افتراضي هي نفس صيغة تنفيذ (implementation) دالة سمة (trait) لا تحتوي على تنفيذ (implementation) افتراضي.

يمكن للتنفيذات (implementations) الافتراضية استدعاء دوال أخرى في نفس السمة (trait)، حتى لو لم يكن لتلك الدوال الأخرى تنفيذ (implementation) افتراضي. بهذه الطريقة، يمكن للسمة (trait) توفير الكثير من الوظائف المفيدة وتتطلب فقط من المنفذين تحديد جزء صغير منها. على سبيل المثال، يمكننا تعريف سمة (trait) `Summary` لتحتوي على دالة مرتبطة (associated function) `summarize_author` التي يكون تنفيذها (implementation) مطلوبًا، ثم تعريف دالة `summarize` التي لها تنفيذ (implementation) افتراضي يستدعي دالة `summarize_author`:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

لاستخدام هذا الإصدار من `Summary`، نحتاج فقط إلى تعريف `summarize_author` عندما ننفذ (implementation) السمة (trait) على نوع:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

بعد تعريف `summarize_author`، يمكننا استدعاء `summarize` على نسخ من بنية `SocialPost`، وسيستدعي التنفيذ (implementation) الافتراضي لـ `summarize` تعريف `summarize_author` الذي قدمناه. نظرًا لأننا نفذنا (implementation) `summarize_author`، فقد أعطتنا سمة (trait) `Summary` سلوك دالة `summarize` دون مطالبتنا بكتابة المزيد من الكود. إليك كيف يبدو:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

يطبع هذا الكود `1 new post from @horse_ebooks: of course, as you probably already know, people`.

لاحظ أنه ليس من الممكن استدعاء التنفيذ (implementation) الافتراضي من تنفيذ (implementation) متجاوز لنفس الدالة.

### السمات (traits) كمعاملات (parameters)

الآن بعد أن عرفت كيفية تعريف وتنفيذ (implementation) السمات (traits)، يمكننا استكشاف كيفية استخدام السمات (traits) لتعريف الدوال التي تقبل العديد من الأنواع المختلفة. سنستخدم سمة (trait) `Summary` التي نفذناها (implementation) على الأنواع `NewsArticle` و `SocialPost` في القائمة 10-13 لتعريف دالة `notify` تستدعي دالة `summarize` على معاملها (parameter) `item`، والذي هو من نوع ما ينفذ (implementation) سمة (trait) `Summary`. للقيام بذلك، نستخدم صيغة `impl Trait`، مثل هذا:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

بدلاً من نوع محدد للمعامل (parameter) `item`، نحدد الكلمة المفتاحية `impl` واسم السمة (trait). يقبل هذا المعامل (parameter) أي نوع ينفذ (implementation) السمة (trait) المحددة. في جسم `notify`، يمكننا استدعاء أي دالة على `item` تأتي من سمة (trait) `Summary`، مثل `summarize`. يمكننا استدعاء `notify` وتمرير أي نسخة من `NewsArticle` أو `SocialPost`. الكود الذي يستدعي الدالة بأي نوع آخر، مثل `String` أو `i32`، لن يُترجم لأن تلك الأنواع لا تنفذ (implementation) `Summary`.

<!-- Old headings. Do not remove or links may break. -->

<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### صيغة حد السمة (trait bound)

صيغة `impl Trait` تعمل للحالات البسيطة ولكنها في الواقع صيغة مختصرة لشكل أطول يُعرف بـ _حد السمة_ (trait bound)؛ يبدو مثل هذا:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

هذا الشكل الأطول مكافئ للمثال في القسم السابق لكنه أكثر تفصيلاً. نضع حدود السمات (trait bounds) مع تصريح معامل النوع العام (generic type parameter) بعد نقطتين وداخل الأقواس الزاوية.

صيغة `impl Trait` مريحة وتجعل الكود أكثر إيجازًا في الحالات البسيطة، بينما يمكن لصيغة حد السمة (trait bound) الكاملة التعبير عن تعقيد أكبر في حالات أخرى. على سبيل المثال، يمكن أن يكون لدينا معاملان (parameters) ينفذان (implementation) `Summary`. القيام بذلك باستخدام صيغة `impl Trait` يبدو مثل هذا:

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

استخدام `impl Trait` مناسب إذا أردنا أن تسمح هذه الدالة لـ `item1` و `item2` بأن يكونا من أنواع مختلفة (طالما أن كلا النوعين ينفذان (implementation) `Summary`). إذا أردنا إجبار كلا المعاملين (parameters) على أن يكونا من نفس النوع، فيجب علينا استخدام حد سمة (trait bound)، مثل هذا:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

النوع العام (generic type) `T` المحدد كنوع المعاملين (parameters) `item1` و `item2` يقيد (bound) الدالة بحيث يجب أن يكون النوع المحدد للقيمة الممررة كحجة لـ `item1` و `item2` هو نفسه.

<!-- Old headings. Do not remove or links may break. -->

<a id="specifying-multiple-trait-bounds-with-the--syntax"></a>

#### حدود سمات (trait bounds) متعددة باستخدام صيغة `+`

يمكننا أيضًا تحديد أكثر من حد سمة (trait bound) واحد. لنفترض أننا أردنا `notify` لاستخدام تنسيق العرض بالإضافة إلى `summarize` على `item`: نحدد في تعريف `notify` أن `item` يجب أن ينفذ (implementation) كلاً من `Display` و `Summary`. يمكننا القيام بذلك باستخدام صيغة `+`:

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

صيغة `+` صالحة أيضًا مع حدود السمات (trait bounds) على الأنواع العمومية (generic types):

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

مع تحديد حدي السمات (trait bounds)، يمكن لجسم `notify` استدعاء `summarize` واستخدام `{}` لتنسيق `item`.

#### حدود سمات (trait bounds) أوضح باستخدام جمل `where`

استخدام عدد كبير جدًا من حدود السمات (trait bounds) له عيوبه. كل نوع عمومي (generic type) له حدود سماته (trait bounds) الخاصة، لذلك يمكن للدوال ذات معاملات الأنواع العمومية (generic type parameters) المتعددة أن تحتوي على الكثير من معلومات حدود السمات (trait bounds) بين اسم الدالة وقائمة معاملاتها، مما يجعل توقيع الدالة صعب القراءة. لهذا السبب، لدى Rust صيغة بديلة لتحديد حدود السمات (trait bounds) داخل جملة `where` بعد توقيع الدالة. لذا، بدلاً من كتابة هذا:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

يمكننا استخدام جملة `where`، مثل هذا:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

توقيع هذه الدالة أقل تشوشًا: اسم الدالة وقائمة المعاملات (parameters) ونوع الإرجاع قريبون من بعضهم البعض، على غرار دالة بدون الكثير من حدود السمات (trait bounds).

### إرجاع أنواع تنفذ (implementation) السمات (traits)

يمكننا أيضًا استخدام صيغة `impl Trait` في موضع الإرجاع لإرجاع قيمة من نوع ما ينفذ (implementation) سمة (trait)، كما هو موضح هنا:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

باستخدام `impl Summary` لنوع الإرجاع، نحدد أن الدالة `returns_summarizable` تُرجع نوعًا ما ينفذ (implementation) سمة (trait) `Summary` دون تسمية النوع المحدد. في هذه الحالة، `returns_summarizable` يُرجع `SocialPost`، لكن الكود الذي يستدعي هذه الدالة لا يحتاج إلى معرفة ذلك.

القدرة على تحديد نوع إرجاع فقط بالسمة (trait) التي ينفذها (implementation) مفيدة بشكل خاص في سياق الإغلاقات (closures) والمكررات (iterators)، والتي نغطيها في الفصل 13. تنشئ الإغلاقات والمكررات أنواعًا يعرفها المصرِّف فقط أو أنواعًا طويلة جدًا للتحديد. تتيح لك صيغة `impl Trait` تحديد أن دالة تُرجع نوعًا ما ينفذ (implementation) سمة (trait) `Iterator` بإيجاز دون الحاجة إلى كتابة نوع طويل جدًا.

ومع ذلك، يمكنك استخدام `impl Trait` فقط إذا كنت تُرجع نوعًا واحدًا. على سبيل المثال، هذا الكود الذي يُرجع إما `NewsArticle` أو `SocialPost` مع نوع الإرجاع المحدد كـ `impl Summary` لن يعمل:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

إرجاع إما `NewsArticle` أو `SocialPost` غير مسموح به بسبب القيود المتعلقة بكيفية تنفيذ صيغة `impl Trait` في المصرِّف. سنغطي كيفية كتابة دالة بهذا السلوك في قسم ["استخدام كائنات السمات للتجريد على السلوك المشترك"][trait-objects]<!-- ignore --> من الفصل 18.

### استخدام حدود السمات (trait bounds) لتنفيذ (implementation) الدوال بشكل مشروط

باستخدام حد سمة (trait bound) مع كتلة `impl` التي تستخدم معاملات الأنواع العمومية (generic type parameters)، يمكننا تنفيذ (implementation) الدوال بشكل مشروط للأنواع التي تنفذ (implementation) السمات (traits) المحددة. على سبيل المثال، النوع `Pair<T>` في القائمة 10-15 ينفذ (implementation) دائمًا الدالة المرتبطة (associated function) `new` لإرجاع نسخة جديدة من `Pair<T>` (تذكر من قسم ["صيغة الدوال"][methods]<!-- ignore --> من الفصل 5 أن `Self` هو اسم مستعار لنوع كتلة `impl`، والذي في هذه الحالة هو `Pair<T>`). لكن في كتلة `impl` التالية، `Pair<T>` ينفذ (implementation) دالة `cmp_display` فقط إذا كان نوعه الداخلي `T` ينفذ (implementation) سمة (trait) `PartialOrd` التي تمكن المقارنة _و_ سمة (trait) `Display` التي تمكن الطباعة.

<Listing number="10-15" file-name="src/lib.rs" caption="تنفيذ (implementation) الدوال بشكل مشروط على نوع عمومي (generic type) اعتمادًا على حدود السمات (trait bounds)">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

</Listing>

يمكننا أيضًا تنفيذ (implementation) سمة (trait) بشكل مشروط لأي نوع ينفذ (implementation) سمة (trait) أخرى. تنفيذات (implementations) سمة (trait) على أي نوع يفي بحدود السمات (trait bounds) تسمى _التنفيذات الشاملة_ (blanket implementations) وتُستخدم على نطاق واسع في مكتبة Rust القياسية. على سبيل المثال، تنفذ (implementation) المكتبة القياسية سمة (trait) `ToString` على أي نوع ينفذ (implementation) سمة (trait) `Display`. كتلة `impl` في المكتبة القياسية تبدو مشابهة لهذا الكود:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

لأن المكتبة القياسية لديها هذا التنفيذ (implementation) الشامل، يمكننا استدعاء دالة `to_string` المعرفة بواسطة سمة (trait) `ToString` على أي نوع ينفذ (implementation) سمة (trait) `Display`. على سبيل المثال، يمكننا تحويل الأعداد الصحيحة إلى قيم `String` المقابلة لها مثل هذا لأن الأعداد الصحيحة تنفذ (implementation) `Display`:

```rust
let s = 3.to_string();
```

تظهر التنفيذات (implementations) الشاملة في الوثائق الخاصة بالسمة (trait) في قسم "Implementors".

تتيح لنا السمات (traits) وحدود السمات (trait bounds) كتابة كود يستخدم معاملات الأنواع العمومية (generic type parameters) لتقليل التكرار ولكن أيضًا تحديد للمصرِّف أننا نريد أن يكون للنوع العام (generic type) سلوك معين. يمكن للمصرِّف بعد ذلك استخدام معلومات حدود السمات (trait bounds) للتحقق من أن جميع الأنواع المحددة المستخدمة مع كودنا توفر السلوك الصحيح. في اللغات ذات الكتابة الديناميكية، سنحصل على خطأ في وقت التشغيل إذا استدعينا دالة على نوع لم يعرّف الدالة. لكن Rust تنقل هذه الأخطاء إلى وقت الترجمة بحيث نضطر إلى إصلاح المشاكل قبل أن يتمكن كودنا من التشغيل. بالإضافة إلى ذلك، لا يتعين علينا كتابة كود يتحقق من السلوك في وقت التشغيل، لأننا تحققنا بالفعل في وقت الترجمة. يؤدي القيام بذلك إلى تحسين الأداء دون الحاجة إلى التخلي عن مرونة الأنواع العمومية (generic types).

[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[methods]: ch05-03-method-syntax.html#method-syntax
