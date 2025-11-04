## الأخطاء غير القابلة للاسترداد مع `panic!`

في بعض الأحيان تحدث أشياء سيئة في كودك، وليس هناك ما يمكنك فعله حيال ذلك. في هذه الحالات، لدى Rust الماكرو `panic!`. هناك طريقتان للتسبب في حالة panic عمليًا: من خلال اتخاذ إجراء يتسبب في حدوث panic في كودنا (مثل الوصول إلى مصفوفة بعد نهايتها) أو من خلال استدعاء ماكرو `panic!` بشكل صريح. في كلتا الحالتين، نتسبب في حدوث panic في برنامجنا. بشكل افتراضي، سيقوم هذا الـ panic بطباعة رسالة فشل، والإرجاع (unwinding)، وتنظيف الذاكرة المؤقتة (stack)، والإنهاء. من خلال متغير بيئة، يمكنك أيضًا جعل Rust يعرض مكدس الاستدعاءات (call stack) عند حدوث panic لتسهيل تتبع مصدر الـ panic.

> ### إرجاع الذاكرة المؤقتة أو الإنهاء الفوري استجابة للـ Panic
>
> بشكل افتراضي، عندما يحدث panic، يبدأ البرنامج في _الإرجاع_ (unwinding)، مما يعني أن Rust يرجع إلى الأعلى في الذاكرة المؤقتة (stack) وينظف البيانات من كل دالة يواجهها. ومع ذلك، فإن الرجوع والتنظيف يتطلبان الكثير من العمل. لذلك يسمح لك Rust باختيار البديل وهو _الإنهاء الفوري_ (aborting)، والذي ينهي البرنامج دون تنظيف.
>
> سيحتاج نظام التشغيل بعد ذلك إلى تنظيف الذاكرة التي كان البرنامج يستخدمها. إذا كنت بحاجة في مشروعك إلى جعل الملف الثنائي الناتج صغيرًا قدر الإمكان، يمكنك التبديل من الإرجاع إلى الإنهاء الفوري عند حدوث panic عن طريق إضافة `panic = 'abort'` إلى أقسام `[profile]` المناسبة في ملف _Cargo.toml_ الخاص بك. على سبيل المثال، إذا كنت تريد الإنهاء الفوري عند panic في وضع الإصدار (release mode)، أضف هذا:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

لنجرب استدعاء `panic!` في برنامج بسيط:

<Listing file-name="src/main.rs">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

</Listing>

عند تشغيل البرنامج، سترى شيئًا مثل هذا:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

يتسبب استدعاء `panic!` في رسالة الخطأ الموجودة في السطرين الأخيرين. يعرض السطر الأول رسالة الـ panic الخاصة بنا والمكان في كود المصدر الخاص بنا حيث حدث الـ panic: _src/main.rs:2:5_ يشير إلى أنه السطر الثاني، الحرف الخامس من ملف _src/main.rs_ الخاص بنا.

في هذه الحالة، السطر المشار إليه هو جزء من كودنا، وإذا ذهبنا إلى ذلك السطر، سنرى استدعاء ماكرو `panic!`. في حالات أخرى، قد يكون استدعاء `panic!` في كود يستدعيه كودنا، واسم الملف ورقم السطر الذي أبلغت عنه رسالة الخطأ سيكون كود شخص آخر حيث يتم استدعاء ماكرو `panic!`، وليس السطر من كودنا الذي أدى في النهاية إلى استدعاء `panic!`.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-a-panic-backtrace"></a>

يمكننا استخدام التتبع الرجعي (backtrace) للدوال التي جاء منها استدعاء `panic!` لمعرفة الجزء من كودنا الذي يتسبب في المشكلة. لفهم كيفية استخدام تتبع رجعي لـ `panic!`، دعنا ننظر إلى مثال آخر ونرى كيف يكون الأمر عندما يأتي استدعاء `panic!` من مكتبة بسبب خطأ في كودنا بدلاً من استدعاء الماكرو مباشرة من كودنا. القائمة 9-1 تحتوي على بعض الكود الذي يحاول الوصول إلى فهرس في متجه (vector) خارج نطاق الفهارس الصالحة.

<Listing number="9-1" file-name="src/main.rs" caption="محاولة الوصول إلى عنصر بعد نهاية متجه، مما سيؤدي إلى استدعاء `panic!`">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

</Listing>

هنا، نحاول الوصول إلى العنصر رقم 100 من المتجه الخاص بنا (الذي يكون عند الفهرس 99 لأن الفهرسة تبدأ من صفر)، لكن المتجه يحتوي فقط على ثلاثة عناصر. في هذا الموقف، سيحدث panic في Rust. استخدام `[]` من المفترض أن يعيد عنصرًا، ولكن إذا مررت فهرسًا غير صالح، فلا يوجد عنصر يمكن أن يعيده Rust هنا سيكون صحيحًا.

في C، محاولة القراءة بعد نهاية بنية البيانات هي سلوك غير محدد (undefined behavior). قد تحصل على أي شيء موجود في موقع الذاكرة الذي يتوافق مع ذلك العنصر في بنية البيانات، حتى لو كانت الذاكرة لا تنتمي إلى تلك البنية. يُسمى هذا _قراءة تجاوز الحاجز_ (buffer overread) ويمكن أن يؤدي إلى ثغرات أمنية إذا تمكن المهاجم من التلاعب بالفهرس بطريقة تسمح له بقراءة بيانات لا يُفترض أن يُسمح له بها مخزنة بعد بنية البيانات.

لحماية برنامجك من هذا النوع من الثغرات، إذا حاولت قراءة عنصر في فهرس غير موجود، سيوقف Rust التنفيذ ويرفض الاستمرار. لنجرب ذلك ونرى:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

يشير هذا الخطأ إلى السطر 4 من ملف _main.rs_ الخاص بنا حيث نحاول الوصول إلى الفهرس 99 من المتجه في `v`.

يخبرنا سطر `note:` أنه يمكننا ضبط متغير البيئة `RUST_BACKTRACE` للحصول على تتبع رجعي لما حدث بالضبط للتسبب في الخطأ. _التتبع الرجعي_ (backtrace) هو قائمة بجميع الدوال التي تم استدعاؤها للوصول إلى هذه النقطة. تعمل التتبعات الرجعية في Rust كما تعمل في اللغات الأخرى: المفتاح لقراءة التتبع الرجعي هو البدء من الأعلى والقراءة حتى ترى ملفات كتبتها أنت. هذا هو المكان الذي نشأت فيه المشكلة. الأسطر الموجودة فوق تلك النقطة هي كود استدعاه كودك؛ والأسطر الموجودة أسفلها هي كود استدعى كودك. قد تتضمن هذه الأسطر السابقة واللاحقة كود Rust الأساسي، أو كود المكتبة القياسية، أو الصناديق (crates) التي تستخدمها. دعنا نحاول الحصول على تتبع رجعي عن طريق ضبط متغير البيئة `RUST_BACKTRACE` إلى أي قيمة باستثناء `0`. القائمة 9-2 تظهر ناتجًا مشابهًا لما ستراه.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

<Listing number="9-2" caption="التتبع الرجعي الذي يتم إنشاؤه بواسطة استدعاء `panic!` والمعروض عند ضبط متغير البيئة `RUST_BACKTRACE`">

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

</Listing>

هذا ناتج كبير! قد يكون الناتج الدقيق الذي تراه مختلفًا حسب نظام التشغيل الخاص بك وإصدار Rust. للحصول على تتبعات رجعية تحتوي على هذه المعلومات، يجب تمكين رموز التصحيح (debug symbols). يتم تمكين رموز التصحيح بشكل افتراضي عند استخدام `cargo build` أو `cargo run` بدون علامة `--release`، كما لدينا هنا.

في الناتج الموجود في القائمة 9-2، يشير السطر 6 من التتبع الرجعي إلى السطر في مشروعنا الذي يسبب المشكلة: السطر 4 من _src/main.rs_. إذا كنا لا نريد أن يحدث panic في برنامجنا، يجب أن نبدأ تحقيقنا في الموقع المشار إليه بالسطر الأول الذي يذكر ملفًا كتبناه نحن. في القائمة 9-1، حيث كتبنا عمدًا كودًا سيتسبب في panic، الطريقة لإصلاح الـ panic هي عدم طلب عنصر خارج نطاق فهارس المتجه. عندما يحدث panic في كودك في المستقبل، ستحتاج إلى معرفة الإجراء الذي يتخذه الكود بأي قيم للتسبب في الـ panic وما الذي يجب على الكود فعله بدلاً من ذلك.

سنعود إلى `panic!` ومتى يجب وينبغي ألا نستخدم `panic!` للتعامل مع حالات الخطأ في قسم ["إلى `panic!` أم لا إلى `panic!`"][to-panic-or-not-to-panic]<!-- ignore --> لاحقًا في هذا الفصل. بعد ذلك، سننظر في كيفية الاسترداد من خطأ باستخدام `Result`.

[to-panic-or-not-to-panic]: ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
