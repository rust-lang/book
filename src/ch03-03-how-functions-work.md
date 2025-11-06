## الدوال (Functions)

الدوال (functions) منتشرة بكثرة في كود Rust. لقد رأيت بالفعل واحدة من أهم الدوال (functions) في اللغة: دالة (function) `main`، والتي تمثل نقطة الدخول (entry point) للعديد من البرامج. كما رأيت أيضًا الكلمة المفتاحية (keyword) `fn`، والتي تسمح لك بتعريف دوال (functions) جديدة.

يستخدم كود Rust _نمط الثعبان (snake case)_ كأسلوب تقليدي لأسماء الدوال (functions) والمتغيرات (variables)، حيث تكون جميع الحروف صغيرة وتُفصل الكلمات بواسطة شرطات سفلية. إليك برنامج يحتوي على مثال لتعريف دالة (function):

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

نعرّف دالة (function) في Rust بإدخال `fn` متبوعة باسم الدالة (function) ومجموعة من الأقواس. تخبر الأقواس المعقوفة (curly brackets) المصرِّف (compiler) بمكان بداية ونهاية جسم الدالة (function body).

يمكننا استدعاء (call) أي دالة (function) قمنا بتعريفها بإدخال اسمها متبوعًا بمجموعة من الأقواس. نظرًا لأن `another_function` معرّفة في البرنامج، يمكن استدعاؤها (called) من داخل دالة (function) `main`. لاحظ أننا عرّفنا `another_function` _بعد_ دالة (function) `main` في الكود المصدري؛ كان يمكننا تعريفها قبلها أيضًا. لا تهتم Rust بمكان تعريف الدوال (functions)، بل فقط بأنها معرّفة في مكان ما في نطاق (scope) يمكن للمستدعي رؤيته.

لنبدأ مشروع ثنائي جديد باسم _functions_ لاستكشاف الدوال (functions) بشكل أعمق. ضع مثال `another_function` في _src/main.rs_ وقم بتشغيله. يجب أن ترى الناتج التالي:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

تُنفّذ الأسطر بالترتيب الذي تظهر به في دالة (function) `main`. أولاً تُطبع رسالة "Hello, world!"، ثم يتم استدعاء (called) `another_function` وتُطبع رسالتها.

### المعاملات (Parameters)

يمكننا تعريف دوال (functions) تحتوي على _معاملات (parameters)_، وهي متغيرات (variables) خاصة تشكل جزءًا من توقيع الدالة (function signature). عندما تحتوي دالة (function) على معاملات (parameters)، يمكنك تزويدها بقيم (values) محددة لتلك المعاملات (parameters). من الناحية التقنية، تُسمى القيم (values) المحددة _وسائط (arguments)_، ولكن في المحادثات العادية، يميل الناس إلى استخدام كلمتي _معامل (parameter)_ و_وسيطة (argument)_ بشكل متبادل سواء للمتغيرات (variables) في تعريف الدالة (function) أو للقيم (values) المحددة الممررة عند استدعاء الدالة (function).

في هذا الإصدار من `another_function` نضيف معاملًا:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

جرب تشغيل هذا البرنامج؛ يجب أن تحصل على الناتج التالي:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

تعريف `another_function` يحتوي على معامل (parameter) واحد يُسمى `x`. نوع (type) `x` محدد بأنه `i32`. عندما نمرر `5` إلى `another_function`، يضع الماكرو (macro) `println!` قيمة (value) `5` حيث كان زوج الأقواس المعقوفة الذي يحتوي على `x` في سلسلة التنسيق (format string).

في توقيعات الدوال (function signatures)، _يجب_ أن تعلن عن نوع (type) كل معامل (parameter). هذا قرار متعمد في تصميم Rust: يعني طلب تعليقات النوع (type annotations) في تعاريف الدوال (function definitions) أن المصرِّف (compiler) لا يحتاج تقريبًا أبدًا إلى استخدامها في مكان آخر في الكود لمعرفة النوع (type) الذي تعنيه. كما أن المصرِّف (compiler) قادر على تقديم رسائل خطأ أكثر إفادة إذا كان يعرف الأنواع (types) التي تتوقعها الدالة (function).

عند تعريف معاملات (parameters) متعددة، افصل تعريفات المعاملات (parameters) بفواصل، هكذا:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

هذا المثال ينشئ دالة (function) باسم `print_labeled_measurement` بمعاملين (parameters). المعامل (parameter) الأول يُسمى `value` وهو من نوع (type) `i32`. المعامل (parameter) الثاني يُسمى `unit_label` وهو من نوع (type) `char`. تطبع الدالة (function) بعد ذلك نصًا يحتوي على كل من `value` و`unit_label`.

لنجرب تشغيل هذا الكود. استبدل البرنامج الموجود حاليًا في ملف _src/main.rs_ الخاص بمشروع _functions_ بالمثال السابق وقم بتشغيله باستخدام `cargo run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

نظرًا لأننا استدعينا (called) الدالة (function) بقيمة (value) `5` لـ `value` وقيمة (value) `'h'` لـ `unit_label`، يحتوي ناتج البرنامج على تلك القيم (values).

### العبارات والتعبيرات (Statements and Expressions)

تتكون أجسام الدوال (function bodies) من سلسلة من العبارات (statements) تنتهي اختياريًا بتعبير (expression). حتى الآن، لم تتضمن الدوال (functions) التي غطيناها تعبيرًا (expression) نهائيًا، لكنك رأيت تعبيرًا (expression) كجزء من عبارة (statement). نظرًا لأن Rust لغة قائمة على التعبيرات (expression-based language)، فهذا تمييز مهم يجب فهمه. اللغات الأخرى ليس لديها نفس التمييزات، لذا دعنا ننظر إلى ماهية العبارات (statements) والتعبيرات (expressions) وكيف تؤثر اختلافاتهما على أجسام الدوال (function bodies).

- _العبارات (statements)_ هي تعليمات تنفذ إجراءً ما ولا تُرجع قيمة (value).
- _التعبيرات (expressions)_ تُقيّم لتنتج قيمة (value) ناتجة.

لنلقِ نظرة على بعض الأمثلة.

لقد استخدمنا بالفعل العبارات (statements) والتعبيرات (expressions). إنشاء متغير (variable) وتعيين قيمة (value) له باستخدام الكلمة المفتاحية (keyword) `let` هو عبارة (statement). في القائمة 3-1، `let y = 6;` هي عبارة (statement).

<Listing number="3-1" file-name="src/main.rs" caption="تعريف دالة (function) `main` يحتوي على عبارة (statement) واحدة">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

تعريفات الدوال (function definitions) هي أيضًا عبارات (statements)؛ المثال السابق بأكمله هو عبارة (statement) في حد ذاته. (كما سنرى قريبًا، استدعاء دالة (function call) ليس عبارة (statement)، رغم ذلك.)

العبارات (statements) لا تُرجع قيمًا (values). لذلك، لا يمكنك تعيين عبارة (statement) `let` إلى متغير (variable) آخر، كما يحاول الكود التالي القيام به؛ ستحصل على خطأ:

<span class="filename">اسم الملف: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

عند تشغيل هذا البرنامج، سيبدو الخطأ الذي ستحصل عليه كالتالي:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

عبارة (statement) `let y = 6` لا تُرجع قيمة (value)، لذلك لا يوجد شيء يمكن ربط `x` به. هذا يختلف عما يحدث في لغات أخرى، مثل C وRuby، حيث يُرجع التعيين قيمة (value) التعيين. في تلك اللغات، يمكنك كتابة `x = y = 6` ويكون لكل من `x` و`y` القيمة (value) `6`؛ هذا ليس هو الحال في Rust.

التعبيرات (expressions) تُقيّم لتنتج قيمة (value) وتشكل معظم بقية الكود الذي ستكتبه في Rust. ضع في اعتبارك عملية رياضية، مثل `5 + 6`، وهي تعبير (expression) يُقيّم إلى القيمة (value) `11`. يمكن أن تكون التعبيرات (expressions) جزءًا من العبارات (statements): في القائمة 3-1، الرقم `6` في عبارة (statement) `let y = 6;` هو تعبير (expression) يُقيّم إلى القيمة (value) `6`. استدعاء دالة (calling a function) هو تعبير (expression). استدعاء ماكرو (calling a macro) هو تعبير (expression). كتلة (block) نطاق جديدة تُنشأ بأقواس معقوفة هي تعبير (expression)، على سبيل المثال:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

هذا التعبير (expression):

```rust,ignore
{
    let x = 3;
    x + 1
}
```

هو كتلة (block)، في هذه الحالة، تُقيّم إلى `4`. تلك القيمة (value) تُربط بـ `y` كجزء من عبارة (statement) `let`. لاحظ السطر `x + 1` بدون فاصلة منقوطة في النهاية، وهو على عكس معظم الأسطر التي رأيتها حتى الآن. التعبيرات (expressions) لا تتضمن فواصل منقوطة في النهاية. إذا أضفت فاصلة منقوطة في نهاية تعبير (expression)، فإنك تحوّله إلى عبارة (statement)، وبالتالي لن تُرجع قيمة (value). ضع هذا في اعتبارك بينما تستكشف قيم الإرجاع (return values) للدوال (functions) والتعبيرات (expressions) في القسم التالي.

### الدوال التي تُرجع قيمًا (Functions with Return Values)

يمكن للدوال (functions) إرجاع قيم (values) إلى الكود الذي يستدعيها. لا نُسمي قيم الإرجاع (return values)، لكن يجب أن نعلن عن نوعها (type) بعد سهم (`->`). في Rust، قيمة إرجاع (return value) الدالة (function) مرادفة لقيمة (value) التعبير (expression) النهائي في كتلة (block) جسم الدالة (function body). يمكنك العودة مبكرًا من دالة (function) باستخدام الكلمة المفتاحية (keyword) `return` وتحديد قيمة (value)، لكن معظم الدوال (functions) تُرجع التعبير (expression) الأخير ضمنيًا. إليك مثالاً على دالة (function) تُرجع قيمة (value):

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

لا توجد استدعاءات دوال (function calls) أو ماكروهات (macros) أو حتى عبارات (statements) `let` في دالة (function) `five`—فقط الرقم `5` بمفرده. هذه دالة (function) صحيحة تمامًا في Rust. لاحظ أن نوع إرجاع (return type) الدالة (function) محدد أيضًا، بصيغة `-> i32`. جرب تشغيل هذا الكود؛ يجب أن يبدو الناتج كالتالي:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

الرقم `5` في `five` هو قيمة إرجاع (return value) الدالة (function)، ولهذا السبب نوع الإرجاع (return type) هو `i32`. لنفحص هذا بمزيد من التفصيل. هناك نقطتان مهمتان: أولاً، السطر `let x = five();` يوضح أننا نستخدم قيمة إرجاع (return value) دالة (function) لتهيئة متغير (variable). نظرًا لأن الدالة (function) `five` تُرجع `5`، فإن ذلك السطر هو نفسه التالي:

```rust
let x = 5;
```

ثانيًا، دالة (function) `five` ليس لديها معاملات (parameters) وتعرّف نوع قيمة الإرجاع (return type)، لكن جسم الدالة (function body) هو رقم `5` وحيد بدون فاصلة منقوطة لأنه تعبير (expression) نريد إرجاع قيمته (value).

لنلقِ نظرة على مثال آخر:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

سيطبع تشغيل هذا الكود `The value of x is: 6`. لكن ماذا يحدث إذا وضعنا فاصلة منقوطة في نهاية السطر الذي يحتوي على `x + 1`، محوّلين إياه من تعبير (expression) إلى عبارة (statement)؟

<span class="filename">اسم الملف: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

سيؤدي ترجمة هذا الكود إلى خطأ، كما يلي:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

رسالة الخطأ الرئيسية، `mismatched types`، تكشف عن المشكلة الأساسية في هذا الكود. تعريف الدالة (function definition) `plus_one` يقول إنها ستُرجع `i32`، لكن العبارات (statements) لا تُقيّم لتنتج قيمة (value)، وهو ما يُعبر عنه بـ `()`، وهو نوع الوحدة (unit type). لذلك، لا يتم إرجاع أي شيء، وهو ما يتناقض مع تعريف الدالة (function definition) ويؤدي إلى خطأ. في هذا الناتج، تقدم Rust رسالة للمساعدة في تصحيح هذه المشكلة: تقترح إزالة الفاصلة المنقوطة، والتي من شأنها إصلاح الخطأ.
