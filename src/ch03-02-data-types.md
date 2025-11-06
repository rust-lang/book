## أنواع البيانات (Data Types)

كل قيمة (value) في Rust لها _نوع بيانات (data type)_ معين، والذي يخبر Rust بنوع البيانات المحددة حتى تعرف كيفية التعامل مع تلك البيانات. سننظر في مجموعتين فرعيتين من أنواع البيانات (data types): القياسية (scalar) والمركبة (compound).

ضع في اعتبارك أن Rust لغة _مكتوبة بشكل ثابت (statically typed)_، مما يعني أنه يجب أن تعرف أنواع (types) جميع المتغيرات (variables) في وقت الترجمة (compile time). عادةً ما يستطيع المصرِّف (compiler) استنتاج النوع (type) الذي نريد استخدامه بناءً على القيمة (value) وكيفية استخدامنا لها. في الحالات التي تكون فيها العديد من الأنواع ممكنة، مثل عندما حولنا `String` إلى نوع رقمي باستخدام `parse` في قسم [«مقارنة التخمين بالرقم السري»][comparing-the-guess-to-the-secret-number]<!-- ignore --> في الفصل الثاني، يجب علينا إضافة تعليق توضيحي للنوع، مثل هذا:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

إذا لم نضف تعليق النوع التوضيحي (type annotation) `: u32` الموضح في الكود السابق، فسيعرض Rust الخطأ التالي، مما يعني أن المصرِّف (compiler) يحتاج إلى مزيد من المعلومات منا لمعرفة النوع (type) الذي نريد استخدامه:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

سترى تعليقات توضيحية (type annotations) مختلفة للأنواع لأنواع البيانات (data types) الأخرى.

### الأنواع القياسية (Scalar Types)

يمثل النوع _القياسي (scalar)_ قيمة (value) واحدة. لدى Rust أربعة أنواع قياسية (scalar types) أساسية: الأعداد الصحيحة (integers)، وأعداد الفاصلة العائمة (floating-point numbers)، والقيم المنطقية (Booleans)، والأحرف (characters). قد تتعرف على هذه من لغات البرمجة الأخرى. دعنا ننتقل إلى كيفية عملها في Rust.

#### أنواع الأعداد الصحيحة (Integer Types)

_العدد الصحيح (integer)_ هو رقم بدون مكون كسري. استخدمنا نوع عدد صحيح (integer type) واحد في الفصل الثاني، وهو نوع `u32`. يشير إعلان النوع (type declaration) هذا إلى أن القيمة (value) المرتبطة به يجب أن تكون عددًا صحيحًا غير موقّع (unsigned) (تبدأ أنواع الأعداد الصحيحة الموقّعة (signed) بـ `i` بدلاً من `u`) يشغل 32 بتًا من المساحة. يوضح الجدول 3-1 أنواع الأعداد الصحيحة (integer types) المدمجة في Rust. يمكننا استخدام أي من هذه المتغيرات (variants) للإعلان عن نوع قيمة عدد صحيح.

<span class="caption">الجدول 3-1: أنواع الأعداد الصحيحة في Rust</span>

| الطول            | موقّع    | غير موقّع |
| ---------------- | ------- | -------- |
| 8-بت             | `i8`    | `u8`     |
| 16-بت            | `i16`   | `u16`    |
| 32-بت            | `i32`   | `u32`    |
| 64-بت            | `i64`   | `u64`    |
| 128-بت           | `i128`  | `u128`   |
| يعتمد على البنية | `isize` | `usize`  |

يمكن أن يكون كل متغير (variant) إما موقعًا (signed) أو غير موقع (unsigned) وله حجم صريح. يشير _الموقّع (signed)_ و _غير الموقّع (unsigned)_ إلى ما إذا كان من الممكن أن يكون الرقم سالبًا - بمعنى آخر، ما إذا كان الرقم يحتاج إلى أن يكون معه إشارة (موقع) أو ما إذا كان سيكون إيجابيًا فقط ويمكن بالتالي تمثيله بدون إشارة (غير موقع). إنه مثل كتابة الأرقام على الورق: عندما تكون الإشارة مهمة، يتم عرض الرقم بعلامة زائد أو علامة ناقص؛ ومع ذلك، عندما يكون من الآمن افتراض أن الرقم إيجابي، يتم عرضه بدون إشارة. يتم تخزين الأرقام الموقعة باستخدام تمثيل [المتمم الثنائي][twos-complement]<!-- ignore -->.

يمكن لكل متغير موقع تخزين أرقام من -(2<sup>n - 1</sup>) إلى 2<sup>n - 1</sup> - 1 ضمناً، حيث _n_ هو عدد البتات التي يستخدمها ذلك المتغير. لذلك، يمكن لـ `i8` تخزين أرقام من -(2<sup>7</sup>) إلى 2<sup>7</sup> - 1، والذي يساوي -128 إلى 127. يمكن للمتغيرات غير الموقعة تخزين أرقام من 0 إلى 2<sup>n</sup> - 1، لذلك يمكن لـ `u8` تخزين أرقام من 0 إلى 2<sup>8</sup> - 1، والذي يساوي 0 إلى 255.

بالإضافة إلى ذلك، يعتمد نوعا `isize` و `usize` على بنية الكمبيوتر الذي يعمل عليه برنامجك: 64 بتًا إذا كنت على بنية 64 بتًا و 32 بتًا إذا كنت على بنية 32 بتًا.

يمكنك كتابة الحرفيات الصحيحة (integer literals) بأي من الأشكال الموضحة في الجدول 3-2. لاحظ أن الحرفيات الرقمية (numeric literals) التي يمكن أن تكون أنواعًا رقمية متعددة تسمح بلاحقة النوع (type suffix)، مثل `57u8`، لتحديد النوع (type). يمكن للحرفيات الرقمية (numeric literals) أيضًا استخدام `_` كفاصل بصري لجعل الرقم أسهل في القراءة، مثل `1_000`، والذي سيكون له نفس القيمة (value) كما لو كنت قد حددت `1000`.

<span class="caption">الجدول 3-2: الحرفيات الصحيحة في Rust</span>

| الحرفيات الرقمية | مثال          |
| ---------------- | ------------- |
| عشري             | `98_222`      |
| ست عشري          | `0xff`        |
| ثماني            | `0o77`        |
| ثنائي            | `0b1111_0000` |
| بايت (`u8` فقط)  | `b'A'`        |

إذن كيف تعرف أي نوع من الأعداد الصحيحة (integer type) تستخدم؟ إذا لم تكن متأكدًا، فإن الإعدادات الافتراضية في Rust هي عمومًا أماكن جيدة للبدء: تكون أنواع الأعداد الصحيحة (integer types) افتراضيًا `i32`. الموقف الأساسي الذي قد تستخدم فيه `isize` أو `usize` هو عند فهرسة (indexing) نوع من المجموعات (collections).

> ##### تجاوز العدد الصحيح (Integer Overflow)
>
> لنفترض أن لديك متغيرًا (variable) من النوع (type) `u8` يمكنه الاحتفاظ بقيم (values) بين 0 و 255. إذا حاولت تغيير المتغير (variable) إلى قيمة (value) خارج هذا النطاق، مثل 256، فسيحدث _تجاوز العدد الصحيح (integer overflow)_، والذي يمكن أن يؤدي إلى أحد سلوكين. عند الترجمة (compilation) في وضع التصحيح (debug mode)، يتضمن Rust فحوصات لتجاوز الأعداد الصحيحة (integer overflow) التي تتسبب في _الذعر (panic)_ لبرنامجك في وقت التشغيل (runtime) إذا حدث هذا السلوك. يستخدم Rust مصطلح _الذعر (panic)_ عندما يخرج البرنامج مع خطأ؛ سنناقش حالات الذعر (panics) بشكل أكثر تفصيلاً في قسم [«الأخطاء غير القابلة للاسترداد مع `panic!`»][unrecoverable-errors-with-panic]<!-- ignore --> في الفصل التاسع.
>
> عند الترجمة (compilation) في وضع الإصدار (release mode) باستخدام علامة `--release`، لا يتضمن Rust فحوصات لتجاوز الأعداد الصحيحة (integer overflow) التي تسبب الذعر (panic). بدلاً من ذلك، إذا حدث تجاوز (overflow)، يقوم Rust بإجراء _التفاف المتمم الثنائي (two's complement wrapping)_. باختصار، القيم (values) الأكبر من القيمة (value) القصوى التي يمكن للنوع (type) الاحتفاظ بها «تلتف» إلى الحد الأدنى من القيم (values) التي يمكن للنوع (type) الاحتفاظ بها. في حالة `u8`، تصبح القيمة (value) 256 هي 0، والقيمة (value) 257 تصبح 1، وهكذا. لن يصاب البرنامج بالذعر (panic)، لكن المتغير (variable) سيكون له قيمة (value) ربما لم تكن تتوقعها. يعتبر الاعتماد على سلوك التفاف تجاوز العدد الصحيح (integer overflow wrapping behavior) خطأً.
>
> للتعامل صراحةً مع احتمال التجاوز، يمكنك استخدام عائلات الطرق هذه التي توفرها المكتبة القياسية لأنواع الأرقام الأولية:
>
> - الالتفاف في جميع الأوضاع باستخدام طرق `wrapping_*`، مثل `wrapping_add`.
> - إرجاع قيمة `None` إذا كان هناك تجاوز باستخدام طرق `checked_*`.
> - إرجاع القيمة وقيمة منطقية تشير إلى ما إذا كان هناك تجاوز باستخدام طرق `overflowing_*`.
> - التشبع عند القيم الدنيا أو القصوى للقيمة باستخدام طرق `saturating_*`.

#### أنواع الفاصلة العائمة (Floating-Point Types)

يحتوي Rust أيضًا على نوعين أوليين لـ _أرقام الفاصلة العائمة (floating-point numbers)_، وهي أرقام ذات نقاط عشرية. أنواع الفاصلة العائمة (floating-point types) في Rust هي `f32` و `f64`، والتي تبلغ أحجامها 32 بتًا و 64 بتًا، على التوالي. النوع (type) الافتراضي هو `f64` لأنه على وحدات المعالجة المركزية الحديثة، فإن سرعته تقريبًا نفس سرعة `f32` ولكنه قادر على مزيد من الدقة. جميع أنواع الفاصلة العائمة (floating-point types) موقّعة (signed).

إليك مثال يوضح أرقام الفاصلة العائمة أثناء العمل:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

يتم تمثيل أرقام الفاصلة العائمة وفقًا لمعيار IEEE-754.

#### العمليات الرقمية (Numeric Operations)

يدعم Rust العمليات الرياضية الأساسية التي تتوقعها لجميع أنواع الأرقام: الجمع والطرح والضرب والقسمة والباقي. تقطع القسمة الصحيحة نحو الصفر إلى أقرب عدد صحيح. يوضح الكود التالي كيفية استخدام كل عملية رقمية (numeric operation) في عبارة (statement) `let`:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

يستخدم كل تعبير (expression) في هذه العبارات (statements) عامل رياضي (operator) ويقيّم إلى قيمة (value) واحدة، والتي يتم بعد ذلك ربطها بمتغير (variable). يحتوي [الملحق B][appendix_b]<!-- ignore --> على قائمة بجميع العوامل (operators) التي توفرها Rust.

#### النوع المنطقي (Boolean Type)

كما في معظم لغات البرمجة الأخرى، فإن النوع المنطقي (Boolean type) في Rust له قيمتان (values) ممكنتان: `true` و `false`. القيم المنطقية (Boolean values) بحجم بايت واحد. يتم تحديد النوع المنطقي (Boolean type) في Rust باستخدام `bool`. على سبيل المثال:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

الطريقة الأساسية لاستخدام القيم المنطقية (Boolean values) هي من خلال الشروط (conditions)، مثل تعبير (expression) `if`. سنغطي كيفية عمل تعبيرات (expressions) `if` في Rust في قسم [«تدفق التحكم»][control-flow]<!-- ignore -->.

#### نوع الحرف (Character Type)

نوع (type) `char` في Rust هو النوع الأبجدي الأكثر بدائية في اللغة. فيما يلي بعض الأمثلة على إعلان قيم (values) `char`:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

لاحظ أننا نحدد حرفيات (literals) `char` بعلامات الاقتباس المفردة، على عكس حرفيات السلسلة (string literals)، التي تستخدم علامات الاقتباس المزدوجة. نوع (type) `char` في Rust بحجم أربعة بايتات ويمثل قيمة (value) Unicode Scalar، مما يعني أنه يمكن أن يمثل أكثر بكثير من مجرد ASCII. الحروف المشكّلة، والرموز التعبيرية الصينية واليابانية والكورية، والرموز التعبيرية، ومسافات العرض الصفرية كلها قيم `char` صالحة في Rust. تتراوح قيم Unicode Scalar من `U+0000` إلى `U+D7FF` ومن `U+E000` إلى `U+10FFFF` ضمناً. ومع ذلك، فإن «الحرف» ليس في الواقع مفهومًا في Unicode، لذا فإن حدسك البشري حول ماهية «الحرف» قد لا يتطابق مع ما هو `char` في Rust. سنناقش هذا الموضوع بالتفصيل في [«تخزين النص المشفر UTF-8 بالسلاسل»][strings]<!-- ignore --> في الفصل الثامن.

### الأنواع المركبة (Compound Types)

يمكن للـ _أنواع المركبة (compound types)_ تجميع قيم (values) متعددة في نوع (type) واحد. يحتوي Rust على نوعين مركبين (compound types) أوليين: المجموعات (tuples) والمصفوفات (arrays).

#### نوع المجموعة (Tuple Type)

_المجموعة (tuple)_ هي طريقة عامة لتجميع عدد من القيم (values) بأنواع (types) متنوعة في نوع مركب (compound type) واحد. للمجموعات (tuples) طول ثابت: بمجرد إعلانها، لا يمكنها النمو أو الانكماش في الحجم.

نقوم بإنشاء مجموعة (tuple) عن طريق كتابة قائمة مفصولة بفواصل من القيم (values) داخل أقواس. كل موضع في المجموعة (tuple) له نوع (type)، ولا يجب أن تكون أنواع (types) القيم (values) المختلفة في المجموعة (tuple) هي نفسها. لقد أضفنا تعليقات توضيحية (type annotations) اختيارية للنوع (type) في هذا المثال:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

يربط المتغير (variable) `tup` بالمجموعة (tuple) بأكملها لأن المجموعة (tuple) تعتبر عنصرًا (element) مركبًا واحدًا. للحصول على القيم (values) الفردية من المجموعة (tuple)، يمكننا استخدام مطابقة النمط (pattern matching) لتفكيك (destructuring) قيمة (value) المجموعة (tuple)، مثل هذا:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

ينشئ هذا البرنامج أولاً مجموعة (tuple) ويربطها بالمتغير (variable) `tup`. ثم يستخدم نمطًا (pattern) مع `let` لأخذ `tup` وتحويله إلى ثلاثة متغيرات (variables) منفصلة، `x` و `y` و `z`. يسمى هذا _التفكيك (destructuring)_ لأنه يكسر المجموعة (tuple) الواحدة إلى ثلاثة أجزاء. أخيرًا، يطبع البرنامج قيمة (value) `y`، وهي `6.4`.

يمكننا أيضًا الوصول إلى عنصر (element) المجموعة (tuple) مباشرة باستخدام نقطة (`.`) متبوعة بفهرس (index) القيمة (value) التي نريد الوصول إليها. على سبيل المثال:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

ينشئ هذا البرنامج المجموعة (tuple) `x` ثم يصل إلى كل عنصر (element) من عناصر (elements) المجموعة (tuple) باستخدام فهارسها (indexes) الخاصة. كما هو الحال مع معظم لغات البرمجة، يكون الفهرس (index) الأول في المجموعة (tuple) هو 0.

للمجموعة (tuple) بدون أي قيم (values) اسم خاص، _الوحدة (unit)_. يتم كتابة هذه القيمة (value) ونوعها (type) المقابل على حد سواء `()` وتمثل قيمة (value) فارغة أو نوع إرجاع (return type) فارغ. تقوم التعبيرات (expressions) ضمنيًا بإرجاع قيمة (value) الوحدة (unit) إذا لم ترجع أي قيمة (value) أخرى.

#### نوع المصفوفة (Array Type)

طريقة أخرى لوجود مجموعة (collection) من قيم (values) متعددة هي باستخدام _مصفوفة (array)_. على عكس المجموعة (tuple)، يجب أن يكون لكل عنصر (element) من عناصر (elements) المصفوفة (array) نفس النوع (type). على عكس المصفوفات (arrays) في بعض اللغات الأخرى، فإن المصفوفات (arrays) في Rust لها طول ثابت.

نكتب القيم (values) في المصفوفة (array) كقائمة مفصولة بفواصل داخل أقواس مربعة:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

تكون المصفوفات (arrays) مفيدة عندما تريد تخصيص بياناتك على المكدس (stack)، كما هو الحال مع الأنواع (types) الأخرى التي رأيناها حتى الآن، بدلاً من الكومة (heap) (سنناقش المكدس (stack) والكومة (heap) بمزيد من التفاصيل في [الفصل الرابع][stack-and-heap]<!-- ignore -->) أو عندما تريد التأكد من أن لديك دائمًا عددًا ثابتًا من العناصر (elements). المصفوفة (array) ليست مرنة مثل نوع المتجه (vector). المتجه (vector) هو نوع مجموعة (collection type) مماثل يوفره المكتبة القياسية (standard library) _يُسمح_ له بالنمو أو الانكماش في الحجم لأن محتوياته تعيش على الكومة (heap). إذا لم تكن متأكدًا من استخدام مصفوفة (array) أو متجه (vector)، فمن المحتمل أن تستخدم متجهًا (vector). يناقش [الفصل الثامن][vectors]<!-- ignore --> المتجهات (vectors) بمزيد من التفاصيل.

ومع ذلك، تكون المصفوفات (arrays) أكثر فائدة عندما تعرف أن عدد العناصر (elements) لن يحتاج إلى التغيير. على سبيل المثال، إذا كنت تستخدم أسماء الأشهر في برنامج، فمن المحتمل أن تستخدم مصفوفة (array) بدلاً من متجه (vector) لأنك تعلم أنها ستحتوي دائمًا على 12 عنصرًا (element):

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

تكتب نوع (type) المصفوفة (array) باستخدام أقواس مربعة مع نوع (type) كل عنصر (element)، وفاصلة منقوطة، ثم عدد العناصر (elements) في المصفوفة (array)، مثل هذا:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

هنا، `i32` هو نوع (type) كل عنصر (element). بعد الفاصلة المنقوطة، يشير الرقم `5` إلى أن المصفوفة (array) تحتوي على خمسة عناصر (elements).

يمكنك أيضًا تهيئة مصفوفة (array) لتحتوي على نفس القيمة (value) لكل عنصر (element) عن طريق تحديد القيمة (value) الأولية، متبوعة بفاصلة منقوطة، ثم طول المصفوفة (array) بين أقواس مربعة، كما هو موضح هنا:

```rust
let a = [3; 5];
```

ستحتوي المصفوفة (array) المسماة `a` على `5` عناصر (elements) سيتم تعيينها جميعًا على القيمة (value) `3` في البداية. هذا هو نفس كتابة `let a = [3, 3, 3, 3, 3];` ولكن بطريقة أكثر إيجازًا.

<!-- Old headings. Do not remove or links may break. -->

<a id="accessing-array-elements"></a>

#### الوصول إلى عناصر المصفوفة (Accessing Array Elements)

المصفوفة (array) هي جزء واحد من الذاكرة (memory) بحجم معروف وثابت يمكن تخصيصه على المكدس (stack). يمكنك الوصول إلى عناصر (elements) المصفوفة (array) باستخدام الفهرسة (indexing)، مثل هذا:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

في هذا المثال، سيحصل المتغير (variable) المسمى `first` على القيمة (value) `1` لأن هذه هي القيمة (value) عند الفهرس (index) `[0]` في المصفوفة (array). سيحصل المتغير (variable) المسمى `second` على القيمة (value) `2` من الفهرس (index) `[1]` في المصفوفة (array).

#### الوصول غير الصالح إلى عنصر المصفوفة (Invalid Array Element Access)

دعنا نرى ما يحدث إذا حاولت الوصول إلى عنصر (element) من مصفوفة (array) يتجاوز نهاية المصفوفة (array). لنفترض أنك تقوم بتشغيل هذا الكود، على غرار لعبة التخمين في الفصل الثاني، للحصول على فهرس (index) مصفوفة (array) من المستخدم:

<span class="filename">اسم الملف: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

يتم ترجمة (compilation) هذا الكود بنجاح. إذا قمت بتشغيل هذا الكود باستخدام `cargo run` وأدخلت `0` أو `1` أو `2` أو `3` أو `4`، فسيطبع البرنامج القيمة (value) المقابلة عند ذلك الفهرس (index) في المصفوفة (array). إذا أدخلت بدلاً من ذلك رقمًا يتجاوز نهاية المصفوفة (array)، مثل `10`، فسترى إخراجًا مثل هذا:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

أدى البرنامج إلى خطأ في وقت التشغيل (runtime) عند نقطة استخدام قيمة (value) غير صالحة في عملية الفهرسة (indexing). خرج البرنامج برسالة خطأ ولم ينفذ عبارة (statement) `println!` النهائية. عندما تحاول الوصول إلى عنصر (element) باستخدام الفهرسة (indexing)، سيتحقق Rust من أن الفهرس (index) الذي حددته أقل من طول المصفوفة (array). إذا كان الفهرس (index) أكبر من أو يساوي الطول، فسيصاب Rust بالذعر (panic). يجب أن يحدث هذا الفحص في وقت التشغيل (runtime)، خاصة في هذه الحالة، لأن المصرِّف (compiler) لا يمكنه معرفة القيمة (value) التي سيدخلها المستخدم عندما يقوم بتشغيل الكود لاحقًا.

هذا مثال على مبادئ أمان الذاكرة في Rust أثناء العمل. في العديد من اللغات منخفضة المستوى، لا يتم إجراء هذا النوع من الفحص، وعندما تقدم فهرسًا غير صحيح، يمكن الوصول إلى ذاكرة غير صالحة. يحميك Rust من هذا النوع من الأخطاء عن طريق الخروج فورًا بدلاً من السماح بالوصول إلى الذاكرة والاستمرار. يناقش الفصل التاسع المزيد من معالجة الأخطاء في Rust وكيف يمكنك كتابة كود آمن وقابل للقراءة لا يصاب بالذعر ولا يسمح بالوصول إلى ذاكرة غير صالحة.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendix_b]: appendix-02-operators.md
