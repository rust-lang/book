## تعريف الهياكل وإنشاء نسخ منها

الهياكل (structs) تشبه المجموعات (tuples)، التي تمت مناقشتها في قسم [â€œنوع المجموعةâ€][tuples]<!-- ignore -->، في أن كلاهما يحتوي على قيم متعددة مترابطة. مثل المجموعات (tuples)، يمكن أن تكون أجزاء الهيكل (struct) من أنواع مختلفة. على عكس المجموعات (tuples)، في الهيكل (struct) ستقوم بتسمية كل جزء من البيانات بحيث يكون واضحاً ما تعنيه القيم. إضافة هذه الأسماء تعني أن الهياكل (structs) أكثر مرونة من المجموعات (tuples): لا يتعين عليك الاعتماد على ترتيب البيانات لتحديد أو الوصول إلى قيم النسخة (instance).

لتعريف هيكل (struct)، نُدخل الكلمة المفتاحية `struct` ونسمي الهيكل (struct) بأكمله. يجب أن يصف اسم الهيكل (struct) أهمية أجزاء البيانات التي يتم ترجمتها معاً. ثم، داخل الأقواس المعقوفة، نحدد أسماء وأنواع أجزاء البيانات، والتي نسميها _الحقول_ (fields). على سبيل المثال، يُظهر Listing 5-1 هيكلاً (struct) يخزن معلومات حول حساب مستخدم.

<Listing number="5-1" file-name="src/main.rs" caption="تعريف هيكل `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

لاستخدام هيكل (struct) بعد تعريفه، نقوم بإنشاء _نسخة_ (instance) من ذلك الهيكل (struct) عن طريق تحديد قيم محددة لكل من الحقول (fields). نقوم بإنشاء نسخة (instance) من خلال ذكر اسم الهيكل (struct) ثم إضافة أقواس معقوفة تحتوي على أزواج _`key: value`_، حيث المفاتيح هي أسماء الحقول (fields) والقيم هي البيانات التي نريد تخزينها في تلك الحقول (fields). لا يتعين علينا تحديد الحقول (fields) بنفس الترتيب الذي أعلناها به في الهيكل (struct). بمعنى آخر، تعريف الهيكل (struct) يشبه قالباً عاماً للنوع، والنسخ (instances) تملأ ذلك القالب ببيانات محددة لإنشاء قيم من النوع. على سبيل المثال، يمكننا التصريح عن مستخدم معين كما هو موضح في Listing 5-2.

<Listing number="5-2" file-name="src/main.rs" caption="إنشاء نسخة من هيكل `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

للحصول على قيمة محددة من هيكل (struct)، نستخدم التدوين النقطي (dot notation). على سبيل المثال، للوصول إلى عنوان البريد الإلكتروني لهذا المستخدم، نستخدم `user1.email`. إذا كانت النسخة (instance) قابلة للتغيير (mutable)، يمكننا تغيير قيمة باستخدام التدوين النقطي (dot notation) والإسناد إلى حقل (field) معين. يوضح Listing 5-3 كيفية تغيير القيمة في حقل (field) `email` من نسخة (instance) `User` قابلة للتغيير (mutable).

<Listing number="5-3" file-name="src/main.rs" caption="تغيير القيمة في حقل `email` من نسخة `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

لاحظ أن النسخة (instance) بأكملها يجب أن تكون قابلة للتغيير (mutable)؛ Rust لا تسمح لنا بوضع علامة على حقول (fields) معينة فقط كقابلة للتغيير (mutable). كما هو الحال مع أي تعبير، يمكننا إنشاء نسخة (instance) جديدة من الهيكل (struct) كآخر تعبير في جسم الدالة (function) لإرجاع تلك النسخة (instance) الجديدة ضمنياً.

يُظهر Listing 5-4 دالة (function) `build_user` التي تُرجع نسخة (instance) `User` مع البريد الإلكتروني واسم المستخدم المعطيين. يحصل حقل (field) `active` على القيمة `true`، ويحصل `sign_in_count` على قيمة `1`.

<Listing number="5-4" file-name="src/main.rs" caption="دالة `build_user` تأخذ بريداً إلكترونياً واسم مستخدم وتُرجع نسخة `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

من المنطقي تسمية معاملات الدالة (function) بنفس اسم حقول (fields) الهيكل (struct)، ولكن الاضطرار إلى تكرار أسماء حقول (fields) `email` و `username` والمتغيرات أمر مُمل قليلاً. إذا كان الهيكل (struct) يحتوي على المزيد من الحقول (fields)، فإن تكرار كل اسم سيصبح أكثر إزعاجاً. لحسن الحظ، هناك اختصار مريح!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### استخدام اختصار تهيئة الحقل

نظراً لأن أسماء المعاملات وأسماء حقول (fields) الهيكل (struct) متطابقة تماماً في Listing 5-4، يمكننا استخدام بناء _اختصار تهيئة الحقل_ (field init shorthand) لإعادة كتابة `build_user` بحيث تتصرف بالضبط بنفس الطريقة ولكن دون تكرار `username` و `email`، كما هو موضح في Listing 5-5.

<Listing number="5-5" file-name="src/main.rs" caption="دالة `build_user` تستخدم اختصار تهيئة الحقل لأن معاملي `username` و `email` لهما نفس اسم حقول الهيكل">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

هنا، نقوم بإنشاء نسخة (instance) جديدة من هيكل (struct) `User`، الذي يحتوي على حقل (field) اسمه `email`. نريد تعيين قيمة حقل (field) `email` إلى القيمة الموجودة في معامل `email` من دالة (function) `build_user`. نظراً لأن حقل (field) `email` ومعامل `email` لهما نفس الاسم، نحتاج فقط إلى كتابة `email` بدلاً من `email: email`.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-instances-from-other-instances-with-struct-update-syntax"></a>

### إنشاء نسخ باستخدام بناء تحديث الهيكل

غالباً ما يكون من المفيد إنشاء نسخة (instance) جديدة من هيكل (struct) تتضمن معظم القيم من نسخة (instance) أخرى من نفس النوع، ولكن تغيير بعضها. يمكنك القيام بذلك باستخدام بناء تحديث الهيكل (struct update syntax).

أولاً، في Listing 5-6 نوضح كيفية إنشاء نسخة (instance) `User` جديدة في `user2` بالطريقة العادية، بدون بناء التحديث. نحدد قيمة جديدة لـ `email` ولكن نستخدم القيم نفسها من `user1` التي أنشأناها في Listing 5-2.

<Listing number="5-6" file-name="src/main.rs" caption="إنشاء نسخة `User` جديدة باستخدام جميع القيم باستثناء واحدة من `user1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

باستخدام بناء تحديث الهيكل (struct update syntax)، يمكننا تحقيق نفس التأثير بكود أقل، كما هو موضح في Listing 5-7. يحدد البناء `..` أن الحقول (fields) المتبقية التي لم يتم تعيينها صراحة يجب أن يكون لها نفس القيمة كالحقول (fields) في النسخة (instance) المعطاة.

<Listing number="5-7" file-name="src/main.rs" caption="استخدام بناء تحديث الهيكل لتعيين قيمة `email` جديدة لنسخة `User` ولكن لاستخدام بقية القيم من `user1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

الكود في Listing 5-7 أيضاً ينشئ نسخة (instance) في `user2` لها قيمة مختلفة لـ `email` ولكن لها نفس القيم لحقول (fields) `username` و `active` و `sign_in_count` من `user1`. يجب أن يأتي `..user1` أخيراً لتحديد أن أي حقول (fields) متبقية يجب أن تحصل على قيمها من الحقول (fields) المقابلة في `user1`، ولكن يمكننا اختيار تحديد قيم لأي عدد من الحقول (fields) نريده بأي ترتيب، بغض النظر عن ترتيب الحقول (fields) في تعريف الهيكل (struct).

لاحظ أن بناء تحديث الهيكل (struct update syntax) يستخدم `=` مثل الإسناد؛ وذلك لأنه ينقل البيانات، تماماً كما رأينا في قسم [â€œالمتغيرات والبيانات المتفاعلة مع النقلâ€][move]<!-- ignore -->. في هذا المثال، لا يمكننا استخدام `user1` بعد إنشاء `user2` لأن `String` في حقل (field) `username` من `user1` تم نقله (moved) إلى `user2`. إذا كنا قد أعطينا `user2` قيم `String` جديدة لكل من `email` و `username`، وبالتالي استخدمنا فقط قيم `active` و `sign_in_count` من `user1`، فإن `user1` سيظل صالحاً بعد إنشاء `user2`. كل من `active` و `sign_in_count` هما أنواع تنفذ خاصية (trait) `Copy`، لذا فإن السلوك الذي ناقشناه في قسم [â€œبيانات المكدس فقط: Copyâ€][copy]<!-- ignore --> سينطبق. يمكننا أيضاً استخدام `user1.email` في هذا المثال، لأن قيمته لم يتم نقلها (moved) من `user1`.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-tuple-structs-without-named-fields-to-create-different-types"></a>

### إنشاء أنواع مختلفة باستخدام هياكل المجموعة

يدعم Rust أيضاً الهياكل (structs) التي تشبه المجموعات (tuples)، والتي تسمى _هياكل المجموعة_ (tuple structs). هياكل المجموعة (tuple structs) لها المعنى الإضافي الذي يوفره اسم الهيكل (struct) ولكن ليس لديها أسماء مرتبطة بحقولها (fields)؛ بل لديها أنواع الحقول (fields) فقط. هياكل المجموعة (tuple structs) مفيدة عندما تريد إعطاء المجموعة (tuple) بأكملها اسماً وجعل المجموعة (tuple) نوعاً مختلفاً عن المجموعات (tuples) الأخرى، وعندما تكون تسمية كل حقل (field) كما في الهيكل (struct) العادي مطولة أو زائدة.

لتعريف هيكل مجموعة (tuple struct)، ابدأ بالكلمة المفتاحية `struct` واسم الهيكل (struct) متبوعاً بالأنواع في المجموعة (tuple). على سبيل المثال، هنا نعرف ونستخدم هيكلي مجموعة (tuple structs) اسمهما `Color` و `Point`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

لاحظ أن قيم `black` و `origin` هي أنواع مختلفة لأنها نسخ (instances) من هياكل مجموعة (tuple structs) مختلفة. كل هيكل (struct) تعرفه هو نوعه الخاص، حتى لو كانت الحقول (fields) داخل الهيكل (struct) قد يكون لها نفس الأنواع. على سبيل المثال، دالة (function) تأخذ معاملاً من نوع `Color` لا يمكنها أخذ `Point` كوسيطة، حتى لو كان كلا النوعين مكونين من ثلاث قيم `i32`. خلاف ذلك، نسخ (instances) هيكل المجموعة (tuple struct) تشبه المجموعات (tuples) من حيث أنه يمكنك تفكيكها إلى أجزائها الفردية، ويمكنك استخدام `.` متبوعاً بالفهرس (index) للوصول إلى قيمة فردية. على عكس المجموعات (tuples)، تتطلب هياكل المجموعة (tuple structs) تسمية نوع الهيكل (struct) عند تفكيكها. على سبيل المثال، سنكتب `let Point(x, y, z) = origin;` لتفكيك القيم في نقطة `origin` إلى متغيرات اسمها `x` و `y` و `z`.

<!-- Old headings. Do not remove or links may break. -->

<a id="unit-like-structs-without-any-fields"></a>

### تعريف هياكل شبيهة بالوحدة

يمكنك أيضاً تعريف هياكل (structs) ليس لها أي حقول (fields)! تسمى هذه _هياكل شبيهة بالوحدة_ (unit-like structs) لأنها تتصرف بشكل مشابه لـ `()`، نوع الوحدة (unit type) الذي ذكرناه في قسم [â€œنوع المجموعةâ€][tuples]<!-- ignore -->. الهياكل الشبيهة بالوحدة (unit-like structs) يمكن أن تكون مفيدة عندما تحتاج إلى تنفيذ خاصية (trait) على نوع ما ولكن ليس لديك أي بيانات تريد تخزينها في النوع نفسه. سنناقش الخصائص (traits) في الفصل 10. إليك مثال على التصريح عن وإنشاء نسخة (instance) من هيكل وحدة (unit-like struct) اسمه `AlwaysEqual`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

لتعريف `AlwaysEqual`، نستخدم الكلمة المفتاحية `struct`، والاسم الذي نريده، ثم فاصلة منقوطة. لا حاجة للأقواس المعقوفة أو الأقواس! ثم، يمكننا الحصول على نسخة (instance) من `AlwaysEqual` في المتغير `subject` بطريقة مماثلة: باستخدام الاسم الذي عرفناه، دون أي أقواس معقوفة أو أقواس. تخيل أننا لاحقاً سننفذ سلوكاً لهذا النوع بحيث تكون كل نسخة (instance) من `AlwaysEqual` متساوية دائماً مع كل نسخة (instance) من أي نوع آخر، ربما للحصول على نتيجة معروفة لأغراض الاختبار. لن نحتاج إلى أي بيانات لتنفيذ هذا السلوك! سترى في الفصل 10 كيفية تعريف الخصائص (traits) وتنفيذها على أي نوع، بما في ذلك الهياكل الشبيهة بالوحدة (unit-like structs).

> ### ملكية بيانات الهيكل
>
> في تعريف هيكل (struct) `User` في Listing 5-1، استخدمنا نوع `String` المملوك بدلاً من نوع شريحة السلسلة (string slice) `&str`. هذا اختيار متعمد لأننا نريد أن تمتلك كل نسخة (instance) من هذا الهيكل (struct) جميع بياناتها وأن تكون تلك البيانات صالحة طالما أن الهيكل (struct) بأكمله صالح.
>
> من الممكن أيضاً للهياكل (structs) تخزين مراجع (references) للبيانات المملوكة من شيء آخر، ولكن القيام بذلك يتطلب استخدام _أوقات الحياة_ (lifetimes)، وهي ميزة في Rust سنناقشها في الفصل 10. تضمن مدد الصلاحية (lifetimes) أن البيانات المشار إليها بواسطة هيكل (struct) صالحة طالما أن الهيكل (struct) صالح. لنفترض أنك حاولت تخزين مرجع (reference) في هيكل (struct) دون تحديد مدد الصلاحية (lifetimes)، مثل ما يلي في _src/main.rs_؛ هذا لن ينجح:
>
> <Listing file-name="src/main.rs">
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> </Listing>
>
> سيشتكي المصرِّف من أنه يحتاج إلى محددات مدة الصلاحية:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> في الفصل 10، سنناقش كيفية إصلاح هذه الأخطاء بحيث يمكنك تخزين مراجع (references) في الهياكل (structs)، ولكن في الوقت الحالي، سنصلح أخطاء مثل هذه باستخدام الأنواع المملوكة مثل `String` بدلاً من المراجع (references) مثل `&str`.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
