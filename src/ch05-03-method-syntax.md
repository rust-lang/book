## الطرق (Methods)

الطرق (methods) تشبه الدوال (functions): نعلنها باستخدام الكلمة المفتاحية `fn` واسم، ويمكن أن تحتوي على معاملات وقيمة إرجاع، وتحتوي على بعض الشيفرة التي تُنفَّذ عندما يتم استدعاء الطريقة (method) من مكان آخر. على عكس الدوال (functions)، يتم تعريف الطرق (methods) ضمن سياق بنية (struct) (أو enum أو كائن trait، والتي نغطيها في [الفصل 6][enums]<!-- ignore --> و[الفصل 18][trait-objects]<!-- ignore -->، على التوالي)، ومعاملها الأول دائمًا `self`، والذي يمثل نسخة (instance) البنية (struct) التي يتم استدعاء الطريقة (method) عليها.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-methods"></a>

### صيغة الطرق (Method Syntax)

لنغير الدالة (function) `area` التي تحتوي على نسخة (instance) من `Rectangle` كمعامل، وبدلاً من ذلك نجعلها طريقة (method) `area` معرَّفة على البنية (struct) `Rectangle`، كما هو موضح في القائمة 5-13.

<Listing number="5-13" file-name="src/main.rs" caption="تعريف طريقة (method) `area` على البنية (struct) `Rectangle`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

لتعريف الدالة (function) ضمن سياق `Rectangle`، نبدأ كتلة `impl` (implementation) لـ `Rectangle`. كل شيء ضمن كتلة `impl` هذه سيرتبط بنوع `Rectangle`. ثم، ننقل الدالة (function) `area` داخل أقواس `impl` المعقوفة ونغير المعامل الأول (والوحيد في هذه الحالة) ليكون `self` في التوقيع وفي كل مكان داخل الجسم. في `main`، حيث استدعينا الدالة (function) `area` ومررنا `rect1` كوسيط، يمكننا بدلاً من ذلك استخدام _صيغة الطرق_ (method syntax) لاستدعاء الطريقة (method) `area` على نسخة (instance) `Rectangle` الخاصة بنا. صيغة الطرق (method syntax) تأتي بعد النسخة (instance): نضيف نقطة متبوعة باسم الطريقة (method)، والأقواس، وأي وسائط.

في التوقيع (signature) لـ `area`، نستخدم `&self` بدلاً من `rectangle: &Rectangle`. الـ `&self` هو في الواقع اختصار لـ `self: &Self`. داخل كتلة `impl`، النوع `Self` هو اسم مستعار للنوع الذي تكون كتلة `impl` من أجله. يجب أن يكون للطرق (methods) معامل باسم `self` من النوع `Self` كمعامل أول لها، لذلك تتيح لك Rust اختصار هذا بالاسم `self` فقط في موضع المعامل الأول. لاحظ أننا ما زلنا بحاجة إلى استخدام `&` أمام الاختصار `self` للإشارة إلى أن هذه الطريقة (method) تستعير (borrows) نسخة (instance) `Self`، تمامًا كما فعلنا في `rectangle: &Rectangle`. يمكن للطرق (methods) أن تأخذ ملكية (ownership) `self`، أو تستعير (borrow) `self` بشكل غير قابل للتعديل (immutable borrow)، كما فعلنا هنا، أو تستعير (borrow) `self` بشكل قابل للتعديل (mutable borrow)، تمامًا كما يمكنها مع أي معامل آخر.

اخترنا `&self` هنا لنفس السبب الذي استخدمنا فيه `&Rectangle` في نسخة الدالة (function): لا نريد أخذ الملكية (ownership)، ونريد فقط قراءة البيانات في البنية (struct)، وليس الكتابة إليها. إذا أردنا تغيير النسخة (instance) التي استدعينا الطريقة (method) عليها كجزء مما تفعله الطريقة (method)، سنستخدم `&mut self` كمعامل أول. وجود طريقة (method) تأخذ ملكية (ownership) النسخة (instance) باستخدام `self` فقط كمعامل أول أمر نادر؛ هذه التقنية تُستخدم عادةً عندما تحول الطريقة (method) `self` إلى شيء آخر وتريد منع المستدعي من استخدام النسخة (instance) الأصلية بعد التحويل.

السبب الرئيسي لاستخدام الطرق (methods) بدلاً من الدوال (functions)، بالإضافة إلى توفير صيغة الطرق (method syntax) وعدم الحاجة إلى تكرار نوع `self` في توقيع (signature) كل طريقة (method)، هو للتنظيم. لقد وضعنا جميع الأشياء التي يمكننا القيام بها مع نسخة (instance) من نوع في كتلة `impl` واحدة بدلاً من جعل المستخدمين المستقبليين لشيفرتنا يبحثون عن قدرات `Rectangle` في أماكن مختلفة في المكتبة التي نوفرها.

لاحظ أنه يمكننا اختيار إعطاء طريقة (method) نفس اسم أحد حقول (fields) البنية (struct). على سبيل المثال، يمكننا تعريف طريقة (method) على `Rectangle` تسمى أيضًا `width`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

هنا، نختار جعل الطريقة (method) `width` تُرجع `true` إذا كانت القيمة في حقل (field) `width` للنسخة (instance) أكبر من `0` و`false` إذا كانت القيمة `0`: يمكننا استخدام حقل (field) داخل طريقة (method) بنفس الاسم لأي غرض. في `main`، عندما نتبع `rect1.width` بأقواس، يعرف Rust أننا نعني الطريقة (method) `width`. عندما لا نستخدم الأقواس، يعرف Rust أننا نعني الحقل (field) `width`.

غالبًا، ولكن ليس دائمًا، عندما نعطي طريقة (method) نفس اسم الحقل (field)، نريدها أن تُرجع القيمة في الحقل (field) فقط ولا تفعل شيئًا آخر. تسمى الطرق (methods) مثل هذه _getters_، ولا تنفذها Rust تلقائيًا لحقول (fields) البنية (struct) كما تفعل بعض اللغات الأخرى. تكون الـ Getters مفيدة لأنه يمكنك جعل الحقل (field) خاصًا ولكن الطريقة (method) عامة وبالتالي تمكين الوصول للقراءة فقط إلى هذا الحقل (field) كجزء من واجهة API العامة للنوع. سنناقش ما هو العام والخاص وكيفية تحديد حقل (field) أو طريقة (method) كعامة أو خاصة في [الفصل 7][public]<!-- ignore -->.

> ### أين هو العامل `->` (Operator)؟
>
> في C و C++، يُستخدم عاملان مختلفان لاستدعاء الطرق (methods): تستخدم `.` إذا كنت تستدعي طريقة (method) على الكائن مباشرة و`->` إذا كنت تستدعي الطريقة (method) على مؤشر (pointer) إلى الكائن وتحتاج إلى إلغاء الإشارة إلى المؤشر (pointer) أولاً. بعبارة أخرى، إذا كان `object` مؤشرًا (pointer)، فإن `object->something()` مشابه لـ `(*object).something()`.
>
> لا تحتوي Rust على ما يعادل العامل (operator) `->`؛ بدلاً من ذلك، تحتوي Rust على ميزة تسمى _الإشارة التلقائية وإلغاء الإشارة التلقائية_ (automatic referencing and dereferencing). استدعاء الطرق (methods) هو أحد الأماكن القليلة في Rust بهذا السلوك.
>
> إليك كيف يعمل: عندما تستدعي طريقة (method) باستخدام `object.something()`، تضيف Rust تلقائيًا `&` أو `&mut` أو `*` بحيث يتطابق `object` مع توقيع (signature) الطريقة (method). بعبارة أخرى، ما يلي هو نفسه:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> الأول يبدو أكثر وضوحًا. سلوك الإشارة التلقائية (automatic referencing) هذا يعمل لأن الطرق (methods) لها مستقبل واضح - نوع `self`. بالنظر إلى المستقبل واسم الطريقة (method)، يمكن لـ Rust معرفة بشكل نهائي ما إذا كانت الطريقة (method) تقرأ (`&self`)، أو تعدل (`&mut self`)، أو تستهلك (`self`). حقيقة أن Rust تجعل الاستعارة (borrowing) ضمنية لمستقبلي الطرق (methods) هي جزء كبير من جعل الملكية (ownership) مريحة في الممارسة.

### طرق مع معاملات إضافية

لنتدرب على استخدام الطرق (methods) من خلال تطبيق طريقة (method) ثانية على البنية (struct) `Rectangle`. هذه المرة نريد أن تأخذ نسخة (instance) من `Rectangle` نسخة (instance) أخرى من `Rectangle` وتُرجع `true` إذا كان `Rectangle` الثاني يمكن أن يتناسب تمامًا داخل `self` (`Rectangle` الأول)؛ وإلا، يجب أن تُرجع `false`. أي، بمجرد أن نُعرّف الطريقة (method) `can_hold`، نريد أن نكون قادرين على كتابة البرنامج الموضح في القائمة 5-14.

<Listing number="5-14" file-name="src/main.rs" caption="استخدام الطريقة (method) `can_hold` التي لم تُكتب بعد">

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

الناتج المتوقع سيبدو كالتالي لأن كلا البعدين لـ `rect2` أصغر من أبعاد `rect1`, لكن `rect3` أعرض من `rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

نعلم أننا نريد تعريف طريقة (method)، لذلك ستكون داخل كتلة `impl Rectangle`. اسم الطريقة (method) سيكون `can_hold`، وستأخذ استعارة (borrow) غير قابلة للتعديل (immutable borrow) من `Rectangle` آخر كمعامل. يمكننا معرفة نوع المعامل بالنظر إلى الشيفرة التي تستدعي الطريقة (method): `rect1.can_hold(&rect2)` تمرر `&rect2`، وهي استعارة (borrow) غير قابلة للتعديل (immutable borrow) لـ `rect2`، نسخة (instance) من `Rectangle`. هذا منطقي لأننا نحتاج فقط إلى قراءة `rect2` (بدلاً من الكتابة، والتي تعني أننا سنحتاج إلى استعارة (borrow) قابلة للتعديل (mutable borrow))، ونريد أن يحتفظ `main` بملكية (ownership) `rect2` حتى نتمكن من استخدامه مرة أخرى بعد استدعاء الطريقة (method) `can_hold`. قيمة الإرجاع لـ `can_hold` ستكون Boolean، وسيتحقق التطبيق مما إذا كان عرض وارتفاع `self` أكبر من عرض وارتفاع `Rectangle` الآخر، على التوالي. لنضف الطريقة (method) الجديدة `can_hold` إلى كتلة `impl` من القائمة 5-13، الموضحة في القائمة 5-15.

<Listing number="5-15" file-name="src/main.rs" caption="تطبيق الطريقة (method) `can_hold` على `Rectangle` التي تأخذ نسخة (instance) أخرى من `Rectangle` كمعامل">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

عندما نشغل هذه الشيفرة مع الدالة (function) `main` في القائمة 5-14، سنحصل على الناتج المرغوب. يمكن للطرق (methods) أن تأخذ معاملات متعددة نضيفها إلى التوقيع (signature) بعد معامل `self`، وتعمل هذه المعاملات تمامًا مثل المعاملات في الدوال (functions).

### الدوال المرتبطة (Associated Functions)

جميع الدوال (functions) المعرَّفة داخل كتلة `impl` تسمى _دوال مرتبطة_ (associated functions) لأنها مرتبطة (associated) بالنوع المسمى بعد `impl`. يمكننا تعريف دوال مرتبطة (associated functions) لا تحتوي على `self` كمعامل أول لها (وبالتالي ليست طرق (methods)) لأنها لا تحتاج إلى نسخة (instance) من النوع للعمل معها. لقد استخدمنا بالفعل دالة (function) واحدة مثل هذه: الدالة (function) `String::from` المعرَّفة على نوع `String`.

الدوال المرتبطة (associated functions) التي ليست طرق (methods) غالبًا ما تُستخدم للمُنشئات (constructors) التي ستُرجع نسخة (instance) جديدة من البنية (struct). غالبًا ما تسمى هذه `new`، لكن `new` ليس اسمًا خاصًا وليس مدمجًا في اللغة. على سبيل المثال، يمكننا اختيار توفير دالة مرتبطة (associated function) باسم `square` سيكون لها معامل بُعد واحد وتستخدمه كعرض وارتفاع، مما يجعل من الأسهل إنشاء `Rectangle` مربع بدلاً من الحاجة إلى تحديد نفس القيمة مرتين:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

الكلمات المفتاحية `Self` في نوع الإرجاع وفي جسم الدالة (function) هي أسماء مستعارة للنوع الذي يظهر بعد الكلمة المفتاحية `impl`، والذي في هذه الحالة هو `Rectangle`.

لاستدعاء هذه الدالة المرتبطة (associated function)، نستخدم صيغة `::` مع اسم البنية (struct)؛ `let sq = Rectangle::square(3);` هو مثال. هذه الدالة (function) مُحدَّدة النطاق بواسطة البنية (struct): صيغة `::` تُستخدم لكل من الدوال المرتبطة (associated functions) والنطاقات (namespaces) التي تم إنشاؤها بواسطة الوحدات (modules). سنناقش الوحدات (modules) في [الفصل 7][modules]<!-- ignore -->.

### كتل `impl` متعددة

يُسمح لكل بنية (struct) أن تحتوي على كتل `impl` متعددة. على سبيل المثال، القائمة 5-15 مكافئة للشيفرة الموضحة في القائمة 5-16، التي تحتوي على كل طريقة (method) في كتلة `impl` خاصة بها.

<Listing number="5-16" caption="إعادة كتابة القائمة 5-15 باستخدام كتل `impl` متعددة">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

لا يوجد سبب لفصل هذه الطرق (methods) إلى كتل `impl` متعددة هنا، لكن هذه صيغة صحيحة. سنرى حالة تكون فيها كتل `impl` المتعددة مفيدة في الفصل 10، حيث نناقش الأنواع العمومية (generic types) والخصائص (traits).

## الخلاصة

تتيح لك البنيات (structs) إنشاء أنواع مخصصة ذات معنى لمجالك. باستخدام البنيات (structs)، يمكنك الاحتفاظ بأجزاء البيانات المرتبطة متصلة ببعضها البعض وتسمية كل جزء لجعل شيفرتك واضحة. في كتل `impl`، يمكنك تعريف دوال مرتبطة (associated functions) بنوعك، والطرق (methods) هي نوع من الدوال المرتبطة (associated functions) التي تتيح لك تحديد السلوك الذي تمتلكه نسخ (instances) بنياتك (structs).

لكن البنيات (structs) ليست الطريقة الوحيدة لإنشاء أنواع مخصصة: لننتقل إلى ميزة enum في Rust لإضافة أداة أخرى إلى صندوق أدواتك.

[enums]: ch06-00-enums.html
[trait-objects]: ch18-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
