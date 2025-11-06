## الحزم والصناديق

الأجزاء الأولى من نظام الوحدات (module system) التي سنغطيها هي الحزم (packages) والصناديق (crates).

_الصندوق_ (crate) هو أصغر كمية من الكود التي يأخذها مُصرِّف Rust في الاعتبار في
وقت واحد. حتى لو قمت بتشغيل `rustc` بدلاً من `cargo` ومررت ملف كود مصدري واحد
(كما فعلنا في البداية في [«أساسيات برنامج Rust»][basics]<!-- ignore
--> في الفصل 1)، فإن المُصرِّف يعتبر ذلك الملف صندوقاً (crate). يمكن أن تحتوي الصناديق (crates) على
وحدات (modules)، وقد تُعرَّف الوحدات (modules) في ملفات أخرى يتم تصريفها مع الصندوق (crate)، كما سنرى في
الأقسام القادمة.

يمكن أن يأتي الصندوق (crate) في أحد شكلين: صندوق ثنائي (binary crate) أو صندوق مكتبة (library crate).
_الصناديق الثنائية (binary crates)_ هي برامج يمكنك تصريفها إلى ملف تنفيذي يمكنك تشغيله،
مثل برنامج سطر الأوامر أو خادم. يجب أن يحتوي كل منها على دالة تسمى
`main` تحدد ما يحدث عند تشغيل الملف التنفيذي. جميع الصناديق (crates) التي
أنشأناها حتى الآن كانت صناديق ثنائية (binary crates).

_صناديق المكتبة (library crates)_ لا تحتوي على دالة `main`، ولا يتم تصريفها إلى ملف
تنفيذي. بدلاً من ذلك، فإنها تعرف وظائف مخصصة ليتم مشاركتها مع
مشاريع متعددة. على سبيل المثال، صندوق (crate) `rand` الذي استخدمناه في [الفصل
2][rand]<!-- ignore --> يوفر وظائف لتوليد الأرقام العشوائية.
في معظم الأوقات عندما يقول مستخدمو Rust «صندوق (crate)»، فإنهم يقصدون صندوق مكتبة (library crate)، ويستخدمون
«صندوق (crate)» بالتبادل مع مفهوم البرمجة العام «مكتبة».

_جذر الصندوق (crate root)_ هو ملف مصدري يبدأ منه مُصرِّف Rust ويشكل
الوحدة الجذرية (root module) للصندوق (crate) الخاص بك (سنشرح الوحدات (modules) بعمق في [«التحكم في
النطاق (scope) والخصوصية (privacy) باستخدام الوحدات (modules)»][modules]<!-- ignore -->).

_الحزمة (package)_ هي مجموعة من صندوق (crate) واحد أو أكثر توفر مجموعة من
الوظائف. تحتوي الحزمة (package) على ملف _Cargo.toml_ يصف كيفية
بناء تلك الصناديق (crates). Cargo في الواقع حزمة (package) تحتوي على الصندوق الثنائي (binary crate)
لأداة سطر الأوامر التي كنت تستخدمها لبناء كودك. حزمة (package) Cargo
تحتوي أيضاً على صندوق مكتبة (library crate) يعتمد عليه الصندوق الثنائي (binary crate). المشاريع الأخرى
يمكن أن تعتمد على صندوق مكتبة (library crate) Cargo لاستخدام نفس المنطق الذي تستخدمه أداة
سطر أوامر Cargo.

يمكن أن تحتوي الحزمة (package) على أي عدد تريده من الصناديق الثنائية (binary crates)، ولكن على الأكثر صندوق
مكتبة (library crate) واحد فقط. يجب أن تحتوي الحزمة (package) على صندوق (crate) واحد على الأقل، سواء كان
صندوق مكتبة (library crate) أو صندوق ثنائي (binary crate).

لنتابع ما يحدث عندما ننشئ حزمة (package). أولاً، ندخل
الأمر `cargo new my-project`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

بعد أن نشغل `cargo new my-project`، نستخدم `ls` لنرى ما ينشئه Cargo. في
دليل _my-project_، يوجد ملف _Cargo.toml_، مما يعطينا حزمة (package).
يوجد أيضاً دليل _src_ يحتوي على _main.rs_. افتح _Cargo.toml_ في
محرر النصوص الخاص بك ولاحظ أنه لا يوجد ذكر لـ _src/main.rs_. يتبع Cargo
اتفاقية أن _src/main.rs_ هو جذر صندوق (crate root) ثنائي
بنفس اسم الحزمة (package). وبالمثل، يعلم Cargo أنه إذا كان دليل الحزمة (package)
يحتوي على _src/lib.rs_، فإن الحزمة (package) تحتوي على صندوق مكتبة (library crate)
بنفس اسم الحزمة (package)، و _src/lib.rs_ هو جذر الصندوق (crate root) الخاص بها. يمرر Cargo
ملفات جذر الصندوق (crate root) إلى `rustc` لبناء المكتبة أو الملف الثنائي.

هنا، لدينا حزمة (package) تحتوي فقط على _src/main.rs_، مما يعني أنها تحتوي فقط على
صندوق ثنائي (binary crate) اسمه `my-project`. إذا كانت الحزمة (package) تحتوي على _src/main.rs_
و _src/lib.rs_، فإنها تحتوي على صندوقين (crates): صندوق ثنائي (binary crate) وصندوق مكتبة (library crate)، كلاهما بنفس
اسم الحزمة (package). يمكن أن تحتوي الحزمة (package) على صناديق ثنائية (binary crates) متعددة بوضع ملفات
في دليل _src/bin_: كل ملف سيكون صندوقاً ثنائياً (binary crate) منفصلاً.

[basics]: ch01-02-hello-world.html#rust-program-basics
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
