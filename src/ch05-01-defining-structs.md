## تعريف الهياكل وإنشاء نسخ منها

الهياكل تشبه المجموعات (tuples)، التي تمت مناقشتها في قسم [â€œنوع المجموعةâ€][tuples]<!-- ignore -->، في أن كلاهما يحتوي على قيم متعددة مترابطة. مثل المجموعات، يمكن أن تكون أجزاء الهيكل من أنواع مختلفة. على عكس المجموعات، في الهيكل ستقوم بتسمية كل جزء من البيانات بحيث يكون واضحاً ما تعنيه القيم. إضافة هذه الأسماء تعني أن الهياكل أكثر مرونة من المجموعات: لا يتعين عليك الاعتماد على ترتيب البيانات لتحديد أو الوصول إلى قيم النسخة.

لتعريف هيكل، نُدخل الكلمة المفتاحية `struct` ونسمي الهيكل بأكمله. يجب أن يصف اسم الهيكل أهمية أجزاء البيانات التي يتم ترجمتها معاً. ثم، داخل الأقواس المعقوفة، نحدد أسماء وأنواع أجزاء البيانات، والتي نسميها _الحقول_ (fields). على سبيل المثال، يُظهر Listing 5-1 هيكلاً يخزن معلومات حول حساب مستخدم.

<Listing number="5-1" file-name="src/main.rs" caption="تعريف هيكل `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

لاستخدام هيكل بعد تعريفه، نقوم بإنشاء _نسخة_ (instance) من ذلك الهيكل عن طريق تحديد قيم محددة لكل من الحقول. نقوم بإنشاء نسخة من خلال ذكر اسم الهيكل ثم إضافة أقواس معقوفة تحتوي على أزواج _`key: value`_، حيث المفاتيح هي أسماء الحقول والقيم هي البيانات التي نريد تخزينها في تلك الحقول. لا يتعين علينا تحديد الحقول بنفس الترتيب الذي أعلناها به في الهيكل. بمعنى آخر، تعريف الهيكل يشبه قالباً عاماً للنوع، والنسخ تملأ ذلك القالب ببيانات محددة لإنشاء قيم من النوع. على سبيل المثال، يمكننا التصريح عن مستخدم معين كما هو موضح في Listing 5-2.

<Listing number="5-2" file-name="src/main.rs" caption="إنشاء نسخة من هيكل `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

للحصول على قيمة محددة من هيكل، نستخدم التدوين النقطي (dot notation). على سبيل المثال، للوصول إلى عنوان البريد الإلكتروني لهذا المستخدم، نستخدم `user1.email`. إذا كانت النسخة قابلة للتغيير (mutable)، يمكننا تغيير قيمة باستخدام التدوين النقطي والإسناد إلى حقل معين. يوضح Listing 5-3 كيفية تغيير القيمة في حقل `email` من نسخة `User` قابلة للتغيير.

<Listing number="5-3" file-name="src/main.rs" caption="تغيير القيمة في حقل `email` من نسخة `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

لاحظ أن النسخة بأكملها يجب أن تكون قابلة للتغيير؛ Rust لا تسمح لنا بوضع علامة على حقول معينة فقط كقابلة للتغيير. كما هو الحال مع أي تعبير، يمكننا إنشاء نسخة جديدة من الهيكل كآخر تعبير في جسم الدالة لإرجاع تلك النسخة الجديدة ضمنياً.

يُظهر Listing 5-4 دالة `build_user` التي تُرجع نسخة `User` مع البريد الإلكتروني واسم المستخدم المعطيين. يحصل حقل `active` على القيمة `true`، ويحصل `sign_in_count` على قيمة `1`.

<Listing number="5-4" file-name="src/main.rs" caption="دالة `build_user` تأخذ بريداً إلكترونياً واسم مستخدم وتُرجع نسخة `User`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

من المنطقي تسمية معاملات الدالة بنفس اسم حقول الهيكل، ولكن الاضطرار إلى تكرار أسماء حقول `email` و `username` والمتغيرات أمر مُمل قليلاً. إذا كان الهيكل يحتوي على المزيد من الحقول، فإن تكرار كل اسم سيصبح أكثر إزعاجاً. لحسن الحظ، هناك اختصار مريح!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### استخدام اختصار تهيئة الحقل

نظراً لأن أسماء المعاملات وأسماء حقول الهيكل متطابقة تماماً في Listing 5-4، يمكننا استخدام بناء _اختصار تهيئة الحقل_ (field init shorthand) لإعادة كتابة `build_user` بحيث تتصرف بالضبط بنفس الطريقة ولكن دون تكرار `username` و `email`، كما هو موضح في Listing 5-5.

<Listing number="5-5" file-name="src/main.rs" caption="دالة `build_user` تستخدم اختصار تهيئة الحقل لأن معاملي `username` و `email` لهما نفس اسم حقول الهيكل">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

هنا، نقوم بإنشاء نسخة جديدة من هيكل `User`، الذي يحتوي على حقل اسمه `email`. نريد تعيين قيمة حقل `email` إلى القيمة الموجودة في معامل `email` من دالة `build_user`. نظراً لأن حقل `email` ومعامل `email` لهما نفس الاسم، نحتاج فقط إلى كتابة `email` بدلاً من `email: email`.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-instances-from-other-instances-with-struct-update-syntax"></a>

### إنشاء نسخ باستخدام بناء تحديث الهيكل

غالباً ما يكون من المفيد إنشاء نسخة جديدة من هيكل تتضمن معظم القيم من نسخة أخرى من نفس النوع، ولكن تغيير بعضها. يمكنك القيام بذلك باستخدام بناء تحديث الهيكل (struct update syntax).

أولاً، في Listing 5-6 نوضح كيفية إنشاء نسخة `User` جديدة في `user2` بالطريقة العادية، بدون بناء التحديث. نحدد قيمة جديدة لـ `email` ولكن نستخدم القيم نفسها من `user1` التي أنشأناها في Listing 5-2.

<Listing number="5-6" file-name="src/main.rs" caption="إنشاء نسخة `User` جديدة باستخدام جميع القيم باستثناء واحدة من `user1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

باستخدام بناء تحديث الهيكل، يمكننا تحقيق نفس التأثير بكود أقل، كما هو موضح في Listing 5-7. يحدد البناء `..` أن الحقول المتبقية التي لم يتم تعيينها صراحة يجب أن يكون لها نفس القيمة كالحقول في النسخة المعطاة.

<Listing number="5-7" file-name="src/main.rs" caption="استخدام بناء تحديث الهيكل لتعيين قيمة `email` جديدة لنسخة `User` ولكن لاستخدام بقية القيم من `user1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

الكود في Listing 5-7 أيضاً ينشئ نسخة في `user2` لها قيمة مختلفة لـ `email` ولكن لها نفس القيم لحقول `username` و `active` و `sign_in_count` من `user1`. يجب أن يأتي `..user1` أخيراً لتحديد أن أي حقول متبقية يجب أن تحصل على قيمها من الحقول المقابلة في `user1`، ولكن يمكننا اختيار تحديد قيم لأي عدد من الحقول نريده بأي ترتيب، بغض النظر عن ترتيب الحقول في تعريف الهيكل.

لاحظ أن بناء تحديث الهيكل يستخدم `=` مثل الإسناد؛ وذلك لأنه ينقل البيانات، تماماً كما رأينا في قسم [â€œالمتغيرات والبيانات المتفاعلة مع النقلâ€][move]<!-- ignore -->. في هذا المثال، لا يمكننا استخدام `user1` بعد إنشاء `user2` لأن `String` في حقل `username` من `user1` تم نقله إلى `user2`. إذا كنا قد أعطينا `user2` قيم `String` جديدة لكل من `email` و `username`، وبالتالي استخدمنا فقط قيم `active` و `sign_in_count` من `user1`، فإن `user1` سيظل صالحاً بعد إنشاء `user2`. كل من `active` و `sign_in_count` هما أنواع تنفذ خاصية `Copy`، لذا فإن السلوك الذي ناقشناه في قسم [â€œبيانات المكدس فقط: Copyâ€][copy]<!-- ignore --> سينطبق. يمكننا أيضاً استخدام `user1.email` في هذا المثال، لأن قيمته لم يتم نقلها من `user1`.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-tuple-structs-without-named-fields-to-create-different-types"></a>

### إنشاء أنواع مختلفة باستخدام هياكل المجموعة

يدعم Rust أيضاً الهياكل التي تشبه المجموعات، والتي تسمى _هياكل المجموعة_ (tuple structs). هياكل المجموعة لها المعنى الإضافي الذي يوفره اسم الهيكل ولكن ليس لديها أسماء مرتبطة بحقولها؛ بل لديها أنواع الحقول فقط. هياكل المجموعة مفيدة عندما تريد إعطاء المجموعة بأكملها اسماً وجعل المجموعة نوعاً مختلفاً عن المجموعات الأخرى، وعندما تكون تسمية كل حقل كما في الهيكل العادي مطولة أو زائدة.

لتعريف هيكل مجموعة، ابدأ بالكلمة المفتاحية `struct` واسم الهيكل متبوعاً بالأنواع في المجموعة. على سبيل المثال، هنا نعرف ونستخدم هيكلي مجموعة اسمهما `Color` و `Point`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

لاحظ أن قيم `black` و `origin` هي أنواع مختلفة لأنها نسخ من هياكل مجموعة مختلفة. كل هيكل تعرفه هو نوعه الخاص، حتى لو كانت الحقول داخل الهيكل قد يكون لها نفس الأنواع. على سبيل المثال، دالة تأخذ معاملاً من نوع `Color` لا يمكنها أخذ `Point` كوسيطة، حتى لو كان كلا النوعين مكونين من ثلاث قيم `i32`. خلاف ذلك، نسخ هيكل المجموعة تشبه المجموعات من حيث أنه يمكنك تفكيكها إلى أجزائها الفردية، ويمكنك استخدام `.` متبوعاً بالفهرس للوصول إلى قيمة فردية. على عكس المجموعات، تتطلب هياكل المجموعة تسمية نوع الهيكل عند تفكيكها. على سبيل المثال، سنكتب `let Point(x, y, z) = origin;` لتفكيك القيم في نقطة `origin` إلى متغيرات اسمها `x` و `y` و `z`.

<!-- Old headings. Do not remove or links may break. -->

<a id="unit-like-structs-without-any-fields"></a>

### تعريف هياكل شبيهة بالوحدة

يمكنك أيضاً تعريف هياكل ليس لها أي حقول! تسمى هذه _هياكل شبيهة بالوحدة_ (unit-like structs) لأنها تتصرف بشكل مشابه لـ `()`، نوع الوحدة الذي ذكرناه في قسم [â€œنوع المجموعةâ€][tuples]<!-- ignore -->. الهياكل الشبيهة بالوحدة يمكن أن تكون مفيدة عندما تحتاج إلى تنفيذ خاصية (trait) على نوع ما ولكن ليس لديك أي بيانات تريد تخزينها في النوع نفسه. سنناقش الخصائص في الفصل 10. إليك مثال على التصريح عن وإنشاء نسخة من هيكل وحدة اسمه `AlwaysEqual`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

لتعريف `AlwaysEqual`، نستخدم الكلمة المفتاحية `struct`، والاسم الذي نريده، ثم فاصلة منقوطة. لا حاجة للأقواس المعقوفة أو الأقواس! ثم، يمكننا الحصول على نسخة من `AlwaysEqual` في المتغير `subject` بطريقة مماثلة: باستخدام الاسم الذي عرفناه، دون أي أقواس معقوفة أو أقواس. تخيل أننا لاحقاً سننفذ سلوكاً لهذا النوع بحيث تكون كل نسخة من `AlwaysEqual` متساوية دائماً مع كل نسخة من أي نوع آخر، ربما للحصول على نتيجة معروفة لأغراض الاختبار. لن نحتاج إلى أي بيانات لتنفيذ هذا السلوك! سترى في الفصل 10 كيفية تعريف الخصائص وتنفيذها على أي نوع، بما في ذلك الهياكل الشبيهة بالوحدة.

> ### ملكية بيانات الهيكل
>
> في تعريف هيكل `User` في Listing 5-1، استخدمنا نوع `String` المملوك بدلاً من نوع شريحة السلسلة `&str`. هذا اختيار متعمد لأننا نريد أن تمتلك كل نسخة من هذا الهيكل جميع بياناتها وأن تكون تلك البيانات صالحة طالما أن الهيكل بأكمله صالح.
>
> من الممكن أيضاً للهياكل تخزين مراجع للبيانات المملوكة من شيء آخر، ولكن القيام بذلك يتطلب استخدام _أوقات الحياة_ (lifetimes)، وهي ميزة في Rust سنناقشها في الفصل 10. تضمن مدد الصلاحية أن البيانات المشار إليها بواسطة هيكل صالحة طالما أن الهيكل صالح. لنفترض أنك حاولت تخزين مرجع في هيكل دون تحديد مدد الصلاحية، مثل ما يلي في _src/main.rs_؛ هذا لن ينجح:
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
> في الفصل 10، سنناقش كيفية إصلاح هذه الأخطاء بحيث يمكنك تخزين مراجع في الهياكل، ولكن في الوقت الحالي، سنصلح أخطاء مثل هذه باستخدام الأنواع المملوكة مثل `String` بدلاً من المراجع مثل `&str`.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
