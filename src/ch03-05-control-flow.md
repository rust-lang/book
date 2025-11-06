## تدفق التحكم (Control Flow)

القدرة على تشغيل بعض الأكواد اعتماداً على ما إذا كان شرط (condition) معين `true` والقدرة على تشغيل بعض الأكواد بشكل متكرر بينما يكون الشرط (condition) `true` هي لبنات بناء أساسية في معظم لغات البرمجة. أكثر البنى شيوعاً التي تسمح لك بالتحكم في تدفق التنفيذ (execution flow) لكود Rust هي تعبيرات (expressions) `if` والحلقات (loops).

### تعبيرات `if` (if Expressions)

يسمح لك تعبير (expression) `if` بتفريع كودك اعتماداً على الشروط (conditions). أنت توفر شرطاً (condition) ثم تقول، "إذا تحقق هذا الشرط (condition)، شغل هذه الكتلة (block) من الكود. إذا لم يتحقق الشرط (condition)، لا تشغل هذه الكتلة (block) من الكود."

أنشئ مشروعاً جديداً يسمى _branches_ في دليل _projects_ الخاص بك لاستكشاف تعبير `if`. في ملف _src/main.rs_، أدخل ما يلي:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

جميع تعبيرات (expressions) `if` تبدأ بالكلمة المفتاحية (keyword) `if`، متبوعة بشرط (condition). في هذه الحالة، يتحقق الشرط (condition) مما إذا كان المتغير (variable) `number` له قيمة (value) أقل من 5. نضع كتلة (block) الكود التي سيتم تنفيذها إذا كان الشرط (condition) `true` مباشرة بعد الشرط (condition) داخل أقواس معقوفة. كتل الكود (code blocks) المرتبطة بالشروط (conditions) في تعبيرات (expressions) `if` تسمى أحياناً _أذرع (arms)_، تماماً مثل الأذرع (arms) في تعبيرات (expressions) `match` التي ناقشناها في قسم ["مقارنة التخمين بالرقم السري"][comparing-the-guess-to-the-secret-number]<!-- ignore --> من الفصل 2.

اختيارياً، يمكننا أيضاً تضمين تعبير (expression) `else`، الذي اخترنا القيام به هنا، لإعطاء البرنامج كتلة كود (code block) بديلة لتنفيذها في حال كان تقييم الشرط (condition) `false`. إذا لم توفر تعبير (expression) `else` وكان الشرط (condition) `false`، سيقوم البرنامج ببساطة بتخطي كتلة (block) `if` والانتقال إلى الجزء التالي من الكود.

جرّب تشغيل هذا الكود؛ يجب أن ترى الإخراج التالي:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

لنجرب تغيير قيمة (value) `number` إلى قيمة (value) تجعل الشرط (condition) `false` لنرى ما يحدث:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

شغّل البرنامج مرة أخرى، وانظر إلى الإخراج:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

من الجدير بالذكر أيضاً أن الشرط (condition) في هذا الكود _يجب_ أن يكون `bool`. إذا لم يكن الشرط (condition) `bool`، سنحصل على خطأ. على سبيل المثال، جرب تشغيل الكود التالي:

<span class="filename">اسم الملف: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

شرط (condition) `if` يُقَيَّم إلى قيمة (value) `3` هذه المرة، وRust يطرح خطأ:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

يشير الخطأ إلى أن Rust توقعت `bool` لكن حصلت على عدد صحيح (integer). على عكس لغات مثل Ruby و JavaScript، لن تحاول Rust تلقائياً تحويل الأنواع (types) غير المنطقية إلى منطقية. يجب أن تكون صريحاً وأن توفر دائماً لـ `if` قيمة منطقية (Boolean value) كشرطها (condition). إذا أردنا أن تعمل كتلة كود (code block) `if` فقط عندما لا يكون الرقم مساوياً لـ `0`، على سبيل المثال، يمكننا تغيير تعبير (expression) `if` إلى التالي:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

تشغيل هذا الكود سيطبع `number was something other than zero`.

#### معالجة شروط متعددة مع `else if` (Handling Multiple Conditions)

يمكنك استخدام شروط (conditions) متعددة عن طريق الجمع بين `if` و `else` في تعبير (expression) `else if`. على سبيل المثال:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

هذا البرنامج لديه أربعة مسارات محتملة يمكن أن يسلكها. بعد تشغيله، يجب أن ترى الإخراج التالي:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

عندما يُنَفَّذ هذا البرنامج، يفحص كل تعبير (expression) `if` بالتتابع وينفذ أول جسم (body) يُقَيَّم شرطه (condition) إلى `true`. لاحظ أنه على الرغم من أن 6 قابل للقسمة على 2، لا نرى الإخراج `number is divisible by 2`، ولا نرى النص `number is not divisible by 4, 3, or 2` من كتلة (block) `else`. ذلك لأن Rust تنفذ فقط الكتلة (block) الخاصة بأول شرط (condition) `true`، وبمجرد أن تجد واحداً، لا تفحص حتى البقية.

استخدام الكثير من تعبيرات (expressions) `else if` يمكن أن يزدحم كودك، لذا إذا كان لديك أكثر من واحد، قد ترغب في إعادة هيكلة كودك. يصف الفصل 6 بنية تفريع (branching construct) قوية في Rust تسمى `match` لهذه الحالات.

#### استخدام `if` في جملة `let` (Using if in a let Statement)

لأن `if` هو تعبير (expression)، يمكننا استخدامه على الجانب الأيمن من جملة (statement) `let` لإسناد النتيجة إلى متغير (variable)، كما في القائمة 3-2.

<Listing number="3-2" file-name="src/main.rs" caption="إسناد نتيجة تعبير (expression) `if` إلى متغير (variable)">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

سيتم ربط المتغير (variable) `number` بقيمة (value) بناءً على نتيجة تعبير (expression) `if`. شغّل هذا الكود لترى ما يحدث:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

تذكر أن كتل الكود (code blocks) تُقَيَّم إلى آخر تعبير (expression) فيها، والأرقام بحد ذاتها هي أيضاً تعبيرات (expressions). في هذه الحالة، قيمة (value) تعبير (expression) `if` الكامل تعتمد على أي كتلة كود (code block) تُنَفَّذ. هذا يعني أن القيم (values) التي لديها إمكانية أن تكون نتائج من كل ذراع (arm) في `if` يجب أن تكون من نفس النوع (type)؛ في القائمة 3-2، كانت نتائج كل من ذراع (arm) `if` وذراع (arm) `else` أعداداً صحيحة (integers) من نوع (type) `i32`. إذا كانت الأنواع (types) غير متطابقة، كما في المثال التالي، سنحصل على خطأ:

<span class="filename">اسم الملف: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

عندما نحاول ترجمة (compile) هذا الكود، سنحصل على خطأ. أذرع (arms) `if` و `else` لها أنواع (types) قيم (values) غير متوافقة، وRust تشير بالضبط إلى مكان المشكلة في البرنامج:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

التعبير (expression) في كتلة (block) `if` يُقَيَّم إلى عدد صحيح (integer)، والتعبير (expression) في كتلة (block) `else` يُقَيَّم إلى سلسلة نصية (string). هذا لن يعمل، لأن المتغيرات (variables) يجب أن يكون لها نوع (type) واحد، وRust تحتاج إلى معرفة بشكل قاطع في وقت الترجمة (compile time) ما هو نوع (type) المتغير (variable) `number`. معرفة نوع (type) `number` يسمح للمصرِّف (compiler) بالتحقق من أن النوع (type) صالح في كل مكان نستخدم فيه `number`. لن تكون Rust قادرة على القيام بذلك إذا كان نوع (type) `number` يتحدد فقط في وقت التشغيل (runtime)؛ سيكون المصرِّف (compiler) أكثر تعقيداً وسيقدم ضمانات أقل حول الكود إذا كان عليه تتبع أنواع (types) افتراضية متعددة لأي متغير (variable).

### التكرار مع الحلقات (Repetition with Loops)

من المفيد في كثير من الأحيان تنفيذ كتلة كود (code block) أكثر من مرة. لهذه المهمة، توفر Rust عدة _حلقات (loops)_، والتي ستعمل من خلال الكود داخل جسم الحلقة (loop body) حتى النهاية ثم تبدأ فوراً مرة أخرى من البداية. لتجربة الحلقات (loops)، دعنا ننشئ مشروعاً جديداً يسمى _loops_.

لدى Rust ثلاثة أنواع من الحلقات (loops): `loop` و `while` و `for`. دعنا نجرب كل واحدة.

#### تكرار الكود مع `loop` (Repeating Code with loop)

الكلمة المفتاحية (keyword) `loop` تخبر Rust بتنفيذ كتلة كود (code block) مراراً وتكراراً إلى الأبد أو حتى تخبرها صراحة بالتوقف.

كمثال، غيّر ملف _src/main.rs_ في دليل _loops_ الخاص بك ليبدو كالتالي:

<span class="filename">اسم الملف: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

عندما نشغل هذا البرنامج، سنرى `again!` تُطبع مراراً وتكراراً بشكل مستمر حتى نوقف البرنامج يدوياً. تقدم معظم الترمنلات (terminals) اختصار لوحة المفاتيح <kbd>ctrl</kbd>-<kbd>c</kbd> لمقاطعة برنامج عالق في حلقة (loop) مستمرة. جربه:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

الرمز `^C` يمثل المكان الذي ضغطت فيه <kbd>ctrl</kbd>-<kbd>c</kbd>. قد ترى أو لا ترى الكلمة `again!` مطبوعة بعد `^C`، اعتماداً على مكان وجود الكود في الحلقة (loop) عندما استقبل إشارة المقاطعة.

لحسن الحظ، توفر Rust أيضاً طريقة للخروج من حلقة (loop) باستخدام الكود. يمكنك وضع الكلمة المفتاحية (keyword) `break` داخل الحلقة (loop) لإخبار البرنامج متى يتوقف عن تنفيذ الحلقة (loop). تذكر أننا فعلنا ذلك في لعبة التخمين في قسم ["الخروج بعد تخمين صحيح"][quitting-after-a-correct-guess]<!-- ignore --> من الفصل 2 للخروج من البرنامج عندما فاز المستخدم باللعبة بتخمين الرقم الصحيح.

استخدمنا أيضاً `continue` في لعبة التخمين، والتي في حلقة (loop) تخبر البرنامج بتخطي أي كود متبقي في هذا التكرار (iteration) من الحلقة (loop) والذهاب إلى التكرار (iteration) التالي.

#### إرجاع القيم من الحلقات (Returning Values from Loops)

أحد استخدامات `loop` هو إعادة محاولة عملية تعرف أنها قد تفشل، مثل التحقق مما إذا كان خيط قد أكمل مهمته. قد تحتاج أيضاً إلى تمرير نتيجة تلك العملية من الحلقة (loop) إلى بقية كودك. للقيام بذلك، يمكنك إضافة القيمة (value) التي تريد إرجاعها بعد تعبير (expression) `break` الذي تستخدمه لإيقاف الحلقة (loop)؛ ستُرجع تلك القيمة (value) من الحلقة (loop) بحيث يمكنك استخدامها، كما هو موضح هنا:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

قبل الحلقة (loop)، نعلن عن متغير (variable) يسمى `counter` ونهيئه إلى `0`. ثم، نعلن عن متغير (variable) يسمى `result` لحمل القيمة (value) المُرجَعة من الحلقة (loop). في كل تكرار (iteration) من الحلقة (loop)، نضيف `1` إلى المتغير (variable) `counter`، ثم نتحقق مما إذا كان `counter` يساوي `10`. عندما يكون كذلك، نستخدم الكلمة المفتاحية (keyword) `break` مع القيمة (value) `counter * 2`. بعد الحلقة (loop)، نستخدم فاصلة منقوطة لإنهاء الجملة (statement) التي تُسند القيمة (value) إلى `result`. أخيراً، نطبع القيمة (value) في `result`، والتي في هذه الحالة هي `20`.

يمكنك أيضاً استخدام `return` من داخل حلقة (loop). بينما `break` فقط يخرج من الحلقة (loop) الحالية، `return` دائماً يخرج من الدالة (function) الحالية.

<!-- Old headings. Do not remove or links may break. -->

<a id="loop-labels-to-disambiguate-between-multiple-loops"></a>

#### التمييز مع تسميات الحلقات (Loop Labels to Disambiguate Between Multiple Loops)

إذا كان لديك حلقات (loops) داخل حلقات (loops)، فإن `break` و `continue` تنطبق على أعمق حلقة (loop) عند تلك النقطة. يمكنك اختيارياً تحديد _تسمية حلقة (loop label)_ على حلقة (loop) يمكنك بعد ذلك استخدامها مع `break` أو `continue` لتحديد أن تلك الكلمات المفتاحية (keywords) تنطبق على الحلقة (loop) المُسَمَّاة بدلاً من أعمق حلقة (loop). يجب أن تبدأ تسميات الحلقات (loop labels) بعلامة اقتباس واحدة. إليك مثالاً بحلقتين (loops) متداخلتين:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

الحلقة (loop) الخارجية لها التسمية (label) `'counting_up`، وستعد من 0 إلى 2. الحلقة (loop) الداخلية بدون تسمية (label) تعد من 10 إلى 9. أول `break` الذي لا يحدد تسمية (label) سيخرج من الحلقة (loop) الداخلية فقط. جملة (statement) `break 'counting_up;` ستخرج من الحلقة (loop) الخارجية. يطبع هذا الكود:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

<!-- Old headings. Do not remove or links may break. -->

<a id="conditional-loops-with-while"></a>

#### تبسيط الحلقات الشرطية مع while (Conditional Loops with while)

غالباً ما يحتاج البرنامج إلى تقييم شرط (condition) داخل حلقة (loop). بينما يكون الشرط (condition) `true`، تعمل الحلقة (loop). عندما يتوقف الشرط (condition) عن أن يكون `true`، يستدعي البرنامج `break`، موقفاً الحلقة (loop). من الممكن تنفيذ سلوك مثل هذا باستخدام مزيج من `loop` و `if` و `else` و `break`؛ يمكنك تجربة ذلك الآن في برنامج، إذا أردت. ومع ذلك، هذا النمط شائع جداً لدرجة أن Rust لديها بنية لغوية (construct) مدمجة له، تسمى حلقة (loop) `while`. في القائمة 3-3، نستخدم `while` لجعل البرنامج يحلق ثلاث مرات، يعد للأسفل في كل مرة، ثم، بعد الحلقة (loop)، يطبع رسالة ويخرج.

<Listing number="3-3" file-name="src/main.rs" caption="استخدام حلقة (loop) `while` لتشغيل الكود بينما يُقَيَّم شرط (condition) إلى `true`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

هذه البنية (construct) تلغي الكثير من التداخل الذي سيكون ضرورياً إذا استخدمت `loop` و `if` و `else` و `break`، وهي أوضح. بينما يُقَيَّم شرط (condition) إلى `true`، يعمل الكود؛ وإلا، يخرج من الحلقة (loop).

#### الحلقة عبر مجموعة مع `for` (Looping Through a Collection with for)

يمكنك اختيار استخدام بنية (construct) `while` للحلقة (loop) عبر عناصر (elements) مجموعة (collection)، مثل مصفوفة (array). على سبيل المثال، الحلقة (loop) في القائمة 3-4 تطبع كل عنصر (element) في المصفوفة (array) `a`.

<Listing number="3-4" file-name="src/main.rs" caption="الحلقة (loop) عبر كل عنصر (element) من مجموعة (collection) باستخدام حلقة (loop) `while`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

هنا، يعد الكود عبر العناصر (elements) في المصفوفة (array). يبدأ عند الفهرس (index) `0` ثم يحلق حتى يصل إلى الفهرس (index) النهائي في المصفوفة (array) (أي عندما لا يكون `index < 5` صحيحاً). تشغيل هذا الكود سيطبع كل عنصر (element) في المصفوفة (array):

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

جميع قيم (values) المصفوفة (array) الخمس تظهر في الترمنل (terminal)، كما هو متوقع. على الرغم من أن `index` سيصل إلى قيمة (value) `5` في مرحلة ما، تتوقف الحلقة (loop) عن التنفيذ قبل محاولة جلب قيمة (value) سادسة من المصفوفة (array).

ومع ذلك، هذا النهج عرضة للخطأ؛ يمكن أن نتسبب في توقف البرنامج بشكل مفاجئ (panic) إذا كانت قيمة (value) الفهرس (index) أو شرط (condition) الاختبار غير صحيح. على سبيل المثال، إذا غيّرت تعريف المصفوفة (array) `a` لتحتوي على أربعة عناصر (elements) لكن نسيت تحديث الشرط (condition) إلى `while index < 4`، سيتوقف الكود بشكل مفاجئ (panic). إنه أيضاً بطيء، لأن المصرِّف (compiler) يضيف كود وقت تشغيل (runtime code) لأداء الفحص الشرطي لما إذا كان الفهرس (index) ضمن حدود المصفوفة (array) في كل تكرار (iteration) من الحلقة (loop).

كبديل أكثر إيجازاً، يمكنك استخدام حلقة (loop) `for` وتنفيذ بعض الكود لكل عنصر (element) في مجموعة (collection). تبدو حلقة (loop) `for` مثل الكود في القائمة 3-5.

<Listing number="3-5" file-name="src/main.rs" caption="الحلقة (loop) عبر كل عنصر (element) من مجموعة (collection) باستخدام حلقة (loop) `for`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

عندما نشغل هذا الكود، سنرى نفس الإخراج كما في القائمة 3-4. الأهم من ذلك، لقد زدنا الآن من سلامة الكود وقضينا على فرصة الأخطاء التي قد تنتج عن تجاوز نهاية المصفوفة (array) أو عدم الذهاب بعيداً بما فيه الكفاية وتفويت بعض العناصر (elements). كود الآلة المولد من حلقات (loops) `for` يمكن أن يكون أكثر كفاءة أيضاً لأن الفهرس (index) لا يحتاج إلى مقارنته بطول المصفوفة (array) في كل تكرار (iteration).

باستخدام حلقة (loop) `for`، لن تحتاج إلى تذكر تغيير أي كود آخر إذا غيّرت عدد القيم (values) في المصفوفة (array)، كما كنت ستفعل مع الطريقة المستخدمة في القائمة 3-4.

سلامة وإيجاز حلقات (loops) `for` تجعلها البنية الحلقية (loop construct) الأكثر استخداماً في Rust. حتى في الحالات التي تريد فيها تشغيل بعض الكود عدداً معيناً من المرات، كما في مثال العد التنازلي الذي استخدم حلقة (loop) `while` في القائمة 3-3، معظم مبرمجي Rust سيستخدمون حلقة (loop) `for`. الطريقة للقيام بذلك ستكون باستخدام `Range`، المقدم من المكتبة القياسية (standard library)، والذي يولد جميع الأرقام بالتسلسل بدءاً من رقم واحد وانتهاءً قبل رقم آخر.

إليك كيف سيبدو العد التنازلي باستخدام حلقة (loop) `for` وطريقة (method) أخرى لم نتحدث عنها بعد، `rev`، لعكس النطاق (range):

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

هذا الكود أجمل قليلاً، أليس كذلك؟

## ملخص (Summary)

لقد نجحت! كان هذا فصلاً كبيراً: تعلمت عن المتغيرات (variables)، أنواع البيانات (data types) القياسية (scalar) والمركبة (compound)، الدوال (functions)، التعليقات (comments)، تعبيرات (expressions) `if`، والحلقات (loops)! لممارسة المفاهيم (concepts) التي نوقشت في هذا الفصل، جرب بناء برامج للقيام بما يلي:

- تحويل درجات الحرارة بين فهرنهايت وسلزيوس.
- توليد رقم فيبوناتشي الـ _n_.
- طباعة كلمات أغنية عيد الميلاد "الأيام الاثنا عشر لعيد الميلاد"، مستفيداً من التكرار في الأغنية.

عندما تكون مستعداً للمضي قدماً، سنتحدث عن مفهوم (concept) في Rust _لا_ يوجد عادة في لغات البرمجة الأخرى: الملكية (ownership).

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]: ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
