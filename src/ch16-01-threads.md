## استخدام الخيوط لتشغيل الكود بشكل متزامن

في معظم أنظمة التشغيل الحالية، يتم تشغيل كود البرنامج المنفذ في عملية (Process)، وسيدير نظام التشغيل عمليات متعددة في وقت واحد. داخل البرنامج، يمكنك أيضاً أن يكون لديك أجزاء مستقلة تعمل بشكل متزامن. الميزات التي تشغل هذه الأجزاء المستقلة تسمى الخيوط (Threads). على سبيل المثال، يمكن أن يكون لخادم الويب خيوط متعددة بحيث يمكنه الاستجابة لأكثر من طلب واحد في نفس الوقت.

تقسيم الحساب في برنامجك إلى خيوط متعددة لتشغيل مهام متعددة في نفس الوقت يمكن أن يحسن الأداء، لكنه يضيف أيضاً تعقيداً. نظراً لأن الخيوط يمكن أن تعمل بشكل متزامن، لا يوجد ضمان متأصل حول الترتيب الذي ستعمل به أجزاء من كودك على خيوط مختلفة. هذا يمكن أن يؤدي إلى مشاكل، مثل:

- حالات التسابق (Race conditions)، حيث تصل الخيوط إلى البيانات أو الموارد بترتيب غير متسق
- الجمود (Deadlocks)، حيث ينتظر خيطان بعضهما البعض، مما يمنع كلا الخيطين من المتابعة
- أخطاء تحدث فقط في مواقف معينة ويصعب إعادة إنتاجها وإصلاحها بشكل موثوق

تحاول Rust التخفيف من الآثار السلبية لاستخدام الخيوط، لكن البرمجة في سياق متعدد الخيوط لا يزال يتطلب تفكيراً دقيقاً ويتطلب بنية كود مختلفة عن تلك الموجودة في البرامج التي تعمل في خيط واحد.

تنفذ لغات البرمجة الخيوط بطرق مختلفة قليلاً، وتوفر العديد من أنظمة التشغيل واجهة برمجة تطبيقات (API) يمكن للغة البرمجة استدعاؤها لإنشاء خيوط جديدة. تستخدم المكتبة القياسية لـ Rust نموذج 1:1 لتنفيذ الخيوط، حيث يستخدم البرنامج خيط واحد من نظام التشغيل لكل خيط لغة واحد. هناك صناديق (Crates) تنفذ نماذج أخرى للخيوط تقدم مقايضات مختلفة لنموذج 1:1. (نظام async في Rust، الذي سنراه في الفصل التالي، يوفر نهجاً آخر للتزامن أيضاً.)

### إنشاء خيط جديد باستخدام `spawn`

لإنشاء خيط جديد، نستدعي دالة `thread::spawn` ونمررها إغلاقاً (Closure) (تحدثنا عن الإغلاقات في الفصل 13) يحتوي على الكود الذي نريد تشغيله في الخيط الجديد. يطبع المثال في القائمة 16-1 بعض النص من خيط رئيسي ونص آخر من خيط جديد.

<Listing number="16-1" file-name="src/main.rs" caption="إنشاء خيط جديد لطباعة شيء واحد بينما يطبع الخيط الرئيسي شيئاً آخر">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

لاحظ أنه عندما يكتمل الخيط الرئيسي لبرنامج Rust، يتم إيقاف جميع الخيوط المُطلقة، سواء انتهت من التشغيل أم لا. قد يكون الناتج من هذا البرنامج مختلفاً قليلاً في كل مرة، لكنه سيبدو مشابهاً لما يلي:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

تُجبر استدعاءات `thread::sleep` خيطاً على إيقاف تنفيذه لفترة قصيرة، مما يسمح لخيط مختلف بالعمل. من المحتمل أن تتناوب الخيوط، لكن هذا غير مضمون: يعتمد على كيفية جدولة نظام التشغيل للخيوط. في هذا التشغيل، طُبع الخيط الرئيسي أولاً، على الرغم من ظهور بيان الطباعة من الخيط المُطلق أولاً في الكود. وحتى رغم أننا أخبرنا الخيط المُطلق بالطباعة حتى يصل `i` إلى `9`، فقد وصل فقط إلى `5` قبل إيقاف الخيط الرئيسي.

إذا قمت بتشغيل هذا الكود ورأيت فقط ناتجاً من الخيط الرئيسي، أو لم تر أي تداخل، حاول زيادة الأرقام في النطاقات لإنشاء المزيد من الفرص لنظام التشغيل للتبديل بين الخيوط.

<!-- Old headings. Do not remove or links may break. -->

<a id="waiting-for-all-threads-to-finish-using-join-handles"></a>

### انتظار جميع الخيوط حتى تنتهي

لا يوقف الكود في القائمة 16-1 الخيط المُطلق قبل الأوان في معظم الأحيان بسبب انتهاء الخيط الرئيسي فحسب، ولكن نظراً لعدم وجود ضمان على الترتيب الذي تعمل به الخيوط، لا يمكننا أيضاً ضمان تشغيل الخيط المُطلق على الإطلاق!

يمكننا إصلاح مشكلة عدم تشغيل الخيط المُطلق أو انتهائه قبل الأوان عن طريق حفظ قيمة الإرجاع من `thread::spawn` في متغير. نوع الإرجاع من `thread::spawn` هو `JoinHandle<T>`. `JoinHandle<T>` هي قيمة مملوكة عندما نستدعي الدالة `join` عليها، ستنتظر انتهاء خيطها. تُظهر القائمة 16-2 كيفية استخدام `JoinHandle<T>` من الخيط الذي أنشأناه في القائمة 16-1 وكيفية استدعاء `join` للتأكد من أن الخيط المُطلق ينتهي قبل خروج `main`.

<Listing number="16-2" file-name="src/main.rs" caption="حفظ `JoinHandle<T>` من `thread::spawn` لضمان تشغيل الخيط حتى الاكتمال">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

يحجب استدعاء `join` على المعالج الخيط الذي يعمل حالياً حتى ينتهي الخيط الممثل بالمعالج. الحجب (Blocking) لخيط يعني منع هذا الخيط من أداء العمل أو الخروج. نظراً لأننا وضعنا استدعاء `join` بعد حلقة `for` للخيط الرئيسي، فإن تشغيل القائمة 16-2 يجب أن ينتج ناتجاً مشابهاً لهذا:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

يستمر الخيطان في التناوب، لكن الخيط الرئيسي ينتظر بسبب استدعاء `handle.join()` ولا ينتهي حتى ينتهي الخيط المُطلق.

لكن دعونا نرى ما يحدث عندما ننقل `handle.join()` بدلاً من ذلك قبل حلقة `for` في `main`، كهذا:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

سينتظر الخيط الرئيسي انتهاء الخيط المُطلق ثم يقوم بتشغيل حلقة `for` الخاصة به، لذلك لن يتم تداخل الناتج بعد الآن، كما هو موضح هنا:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

التفاصيل الصغيرة، مثل مكان استدعاء `join`، يمكن أن تؤثر على ما إذا كانت خيوطك تعمل في نفس الوقت أم لا.

### استخدام إغلاقات `move` مع الخيوط

غالباً ما سنستخدم الكلمة المفتاحية `move` مع الإغلاقات الممررة إلى `thread::spawn` لأن الإغلاق سيأخذ بعد ذلك ملكية القيم التي يستخدمها من البيئة، وبالتالي ينقل ملكية تلك القيم من خيط إلى آخر. في ["التقاط المراجع أو نقل الملكية"][capture]<!-- ignore --> في الفصل 13، ناقشنا `move` في سياق الإغلاقات. الآن سنركز أكثر على التفاعل بين `move` و `thread::spawn`.

لاحظ في القائمة 16-1 أن الإغلاق الذي نمرره إلى `thread::spawn` لا يأخذ أي معطيات: نحن لا نستخدم أي بيانات من الخيط الرئيسي في كود الخيط المُطلق. لاستخدام البيانات من الخيط الرئيسي في الخيط المُطلق، يجب على إغلاق الخيط المُطلق التقاط القيم التي يحتاجها. تُظهر القائمة 16-3 محاولة لإنشاء متجه في الخيط الرئيسي واستخدامه في الخيط المُطلق. ومع ذلك، لن يعمل هذا بعد، كما سترى بعد قليل.

<Listing number="16-3" file-name="src/main.rs" caption="محاولة استخدام متجه تم إنشاؤه بواسطة الخيط الرئيسي في خيط آخر">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

يستخدم الإغلاق `v`، لذلك سيلتقط `v` ويجعله جزءاً من بيئة الإغلاق. نظراً لأن `thread::spawn` يشغل هذا الإغلاق في خيط جديد، يجب أن نكون قادرين على الوصول إلى `v` داخل هذا الخيط الجديد. لكن عندما نترجم هذا المثال، نحصل على الخطأ التالي:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

تستنتج Rust كيفية التقاط `v`، ونظراً لأن `println!` يحتاج فقط إلى مرجع إلى `v`، يحاول الإغلاق استعارة `v`. ومع ذلك، هناك مشكلة: لا تستطيع Rust معرفة المدة التي سيعمل فيها الخيط المُطلق، لذلك لا تعرف ما إذا كان المرجع إلى `v` سيكون صالحاً دائماً.

توفر القائمة 16-4 سيناريو أكثر احتمالاً لوجود مرجع إلى `v` لن يكون صالحاً.

<Listing number="16-4" file-name="src/main.rs" caption="خيط به إغلاق يحاول التقاط مرجع إلى `v` من خيط رئيسي يُسقط `v`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

إذا سمحت لنا Rust بتشغيل هذا الكود، فهناك احتمال أن يتم وضع الخيط المُطلق على الفور في الخلفية دون التشغيل على الإطلاق. لدى الخيط المُطلق مرجع إلى `v` بداخله، لكن الخيط الرئيسي يُسقط `v` على الفور، باستخدام دالة `drop` التي ناقشناها في الفصل 15. ثم، عندما يبدأ الخيط المُطلق في التنفيذ، لم يعد `v` صالحاً، لذلك المرجع إليه غير صالح أيضاً. يا للهول!

لإصلاح خطأ المصرِّف في القائمة 16-3، يمكننا استخدام نصيحة رسالة الخطأ:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

بإضافة الكلمة المفتاحية `move` قبل الإغلاق، نجبر الإغلاق على أخذ ملكية القيم التي يستخدمها بدلاً من السماح لـ Rust بالاستنتاج بأنه يجب استعارة القيم. يوضح التعديل على القائمة 16-3 الموضح في القائمة 16-5 سيترجم ويعمل كما نقصد.

<Listing number="16-5" file-name="src/main.rs" caption="استخدام الكلمة المفتاحية `move` لإجبار الإغلاق على أخذ ملكية القيم التي يستخدمها">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

قد نميل إلى محاولة فعل نفس الشيء لإصلاح الكود في القائمة 16-4 حيث استدعى الخيط الرئيسي `drop` باستخدام إغلاق `move`. ومع ذلك، فإن هذا الإصلاح لن يعمل لأن ما تحاول القائمة 16-4 فعله غير مسموح به لسبب مختلف. إذا أضفنا `move` إلى الإغلاق، فسننقل `v` إلى بيئة الإغلاق، ولن نتمكن بعد ذلك من استدعاء `drop` عليه في الخيط الرئيسي. سنحصل على خطأ المصرِّف هذا بدلاً من ذلك:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

أنقذتنا قواعد الملكية في Rust مرة أخرى! حصلنا على خطأ من الكود في القائمة 16-3 لأن Rust كانت محافظة وتستعير `v` فقط للخيط، مما يعني أن الخيط الرئيسي يمكنه نظرياً إبطال مرجع الخيط المُطلق. بإخبار Rust بنقل ملكية `v` إلى الخيط المُطلق، نضمن لـ Rust أن الخيط الرئيسي لن يستخدم `v` بعد الآن. إذا غيرنا القائمة 16-4 بنفس الطريقة، فإننا ننتهك قواعد الملكية عندما نحاول استخدام `v` في الخيط الرئيسي. تتجاوز الكلمة المفتاحية `move` الإعداد الافتراضي المحافظ لـ Rust للاستعارة؛ فهي لا تسمح لنا بانتهاك قواعد الملكية.

الآن بعد أن غطينا ما هي الخيوط والدوال التي توفرها واجهة برمجة تطبيقات الخيوط، دعونا ننظر إلى بعض المواقف التي يمكننا فيها استخدام الخيوط.

[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
