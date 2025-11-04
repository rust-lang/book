## نشر صندوق على Crates.io

لقد استخدمنا الحزم من [crates.io](https://crates.io/)<!-- ignore --> كـ
تبعيات لمشروعنا، لكن يمكنك أيضًا مشاركة كودك مع أشخاص آخرين
من خلال نشر حزمك الخاصة. سجل الصناديق في
[crates.io](https://crates.io/)<!-- ignore --> يوزع الكود المصدري لـ
حزمك، لذا فهو يستضيف بشكل أساسي كودًا مفتوح المصدر.

تحتوي Rust و Cargo على ميزات تجعل حزمتك المنشورة أسهل للأشخاص
للعثور عليها واستخدامها. سنتحدث عن بعض هذه الميزات لاحقًا ثم نشرح
كيفية نشر حزمة.

### إنشاء تعليقات توثيق مفيدة

توثيق حزمك بدقة سيساعد المستخدمين الآخرين على معرفة كيفية ومتى
استخدامها، لذا فإن الأمر يستحق استثمار الوقت في كتابة التوثيق. في الفصل
3، ناقشنا كيفية التعليق على كود Rust باستخدام شرطتين مائلتين، `//`. تحتوي Rust أيضًا على
نوع معين من التعليقات للتوثيق، يُعرف بشكل ملائم باسم
_تعليق توثيق_ (documentation comment)، والذي سينشئ توثيق HTML. يعرض HTML
محتويات تعليقات التوثيق لعناصر API العامة المخصصة
للمبرمجين المهتمين بمعرفة كيفية _استخدام_ صندوقك بدلاً من كيفية
_تنفيذ_ صندوقك.

تستخدم تعليقات التوثيق ثلاث شرطات مائلة، `///`، بدلاً من اثنتين وتدعم
ترميز Markdown لتنسيق النص. ضع تعليقات التوثيق مباشرة
قبل العنصر الذي يتم توثيقه. يُظهر Listing 14-1 تعليقات التوثيق
لدالة `add_one` في صندوق باسم `my_crate`.

<Listing number="14-1" file-name="src/lib.rs" caption="تعليق توثيق لدالة">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

</Listing>

هنا، نعطي وصفًا لما تفعله الدالة `add_one`، ونبدأ
قسمًا بعنوان `Examples`، ثم نقدم كودًا يوضح
كيفية استخدام الدالة `add_one`. يمكننا إنشاء توثيق HTML من
تعليق التوثيق هذا عن طريق تشغيل `cargo doc`. ينفذ هذا الأمر أداة
`rustdoc` الموزعة مع Rust ويضع توثيق HTML الناتج
في دليل _target/doc_.

لسهولة الاستخدام، تشغيل `cargo doc --open` سيبني HTML لتوثيق
صندوقك الحالي (بالإضافة إلى توثيق جميع تبعيات
صندوقك) ويفتح النتيجة في متصفح ويب. انتقل إلى
الدالة `add_one` وسترى كيف يتم عرض النص في تعليقات التوثيق،
كما هو موضح في الشكل 14-1.

<img alt="Rendered HTML documentation for the `add_one` function of `my_crate`" src="img/trpl14-01.png" class="center" />

<span class="caption">الشكل 14-1: توثيق HTML لدالة `add_one`</span>

#### الأقسام المستخدمة بشكل شائع

استخدمنا عنوان Markdown `# Examples` في Listing 14-1 لإنشاء قسم
في HTML بعنوان "Examples". هنا بعض الأقسام الأخرى التي يستخدمها مؤلفو الصناديق
بشكل شائع في توثيقهم:

- **Panics**: هذه هي السيناريوهات التي يمكن أن تدخل فيها الدالة الموثقة
  في حالة ذعر (panic). يجب على المستدعين للدالة الذين لا يريدون أن تدخل برامجهم في حالة ذعر
  التأكد من أنهم لا يستدعون الدالة في هذه الحالات.
- **Errors**: إذا كانت الدالة تُرجع `Result`، فإن وصف أنواع
  الأخطاء التي قد تحدث وما هي الشروط التي قد تسبب إرجاع تلك الأخطاء
  يمكن أن يكون مفيدًا للمستدعين حتى يتمكنوا من كتابة كود للتعامل مع
  أنواع مختلفة من الأخطاء بطرق مختلفة.
- **Safety**: إذا كانت الدالة `unsafe` للاستدعاء (نناقش عدم الأمان في
  الفصل 20)، فيجب أن يكون هناك قسم يشرح لماذا الدالة غير آمنة
  ويغطي الثوابت التي تتوقع الدالة من المستدعين دعمها.

معظم تعليقات التوثيق لا تحتاج إلى كل هذه الأقسام، ولكن هذه
قائمة تحقق جيدة لتذكيرك بجوانب كودك التي سيهتم المستخدمون
بمعرفتها.

#### تعليقات التوثيق كاختبارات

إضافة كتل كود مثالية في تعليقات التوثيق الخاصة بك يمكن أن تساعد في توضيح
كيفية استخدام مكتبتك ولها ميزة إضافية: تشغيل `cargo test` سـ
يشغل أمثلة الكود في توثيقك كاختبارات! لا شيء أفضل من
التوثيق مع الأمثلة. ولكن لا شيء أسوأ من الأمثلة التي لا تعمل
لأن الكود قد تغير منذ كتابة التوثيق. إذا شغّلنا
`cargo test` مع التوثيق للدالة `add_one` من Listing
14-1، سنرى قسمًا في نتائج الاختبار يبدو كالتالي:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

```text
   Doc-tests my_crate

running 1 test
test src/lib.rs - add_one (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

الآن، إذا غيّرنا إما الدالة أو المثال بحيث تدخل `assert_eq!`
في المثال في حالة ذعر، وشغّلنا `cargo test` مرة أخرى، سنرى أن اختبارات التوثيق
تكتشف أن المثال والكود غير متزامنين مع بعضهما البعض!

<!-- Old headings. Do not remove or links may break. -->

<a id="commenting-contained-items"></a>

#### تعليقات العناصر المحتواة

نمط تعليق التوثيق `//!` يضيف التوثيق إلى العنصر الذي *يحتوي*
التعليقات بدلاً من العناصر *التي تلي* التعليقات. نستخدم عادةً
هذه التعليقات التوثيقية داخل ملف جذر الصندوق (_src/lib.rs_ بالاصطلاح)
أو داخل وحدة لتوثيق الصندوق أو الوحدة ككل.

على سبيل المثال، لإضافة توثيق يصف الغرض من صندوق `my_crate`
الذي يحتوي على الدالة `add_one`، نضيف تعليقات توثيق
تبدأ بـ `//!` إلى بداية ملف _src/lib.rs_، كما هو موضح في Listing
14-2.

<Listing number="14-2" file-name="src/lib.rs" caption="التوثيق لصندوق `my_crate` ككل">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

</Listing>

لاحظ أنه لا يوجد أي كود بعد السطر الأخير الذي يبدأ بـ `//!`. لأننا
بدأنا التعليقات بـ `//!` بدلاً من `///`، فنحن نوثق العنصر
الذي يحتوي على هذا التعليق بدلاً من عنصر يلي هذا التعليق. في
هذه الحالة، هذا العنصر هو ملف _src/lib.rs_، وهو جذر الصندوق. هذه
التعليقات تصف الصندوق بأكمله.

عندما نشغل `cargo doc --open`، ستُعرض هذه التعليقات على الصفحة الأمامية
من التوثيق لـ `my_crate` فوق قائمة العناصر العامة في
الصندوق، كما هو موضح في الشكل 14-2.

تعليقات التوثيق داخل العناصر مفيدة لوصف الصناديق
والوحدات بشكل خاص. استخدمها لشرح الغرض العام للحاوية
لمساعدة المستخدمين على فهم تنظيم الصندوق.

<img alt="Rendered HTML documentation with a comment for the crate as a whole" src="img/trpl14-02.png" class="center" />

<span class="caption">الشكل 14-2: التوثيق المُعرَّض لـ `my_crate`،
بما في ذلك التعليق الذي يصف الصندوق ككل</span>

<!-- Old headings. Do not remove or links may break. -->

<a id="exporting-a-convenient-public-api-with-pub-use"></a>

### تصدير API عام مريح

هيكل API العام الخاص بك هو اعتبار رئيسي عند نشر
صندوق. الأشخاص الذين يستخدمون صندوقك أقل معرفة بالهيكل منك
وقد يواجهون صعوبة في إيجاد القطع التي يريدون استخدامها إذا كان صندوقك
له تسلسل هرمي كبير من الوحدات.

في الفصل 7، غطينا كيفية جعل العناصر عامة باستخدام الكلمة المفتاحية `pub`، و
كيفية إحضار العناصر إلى نطاق باستخدام الكلمة المفتاحية `use`. ومع ذلك، الهيكل
الذي يكون منطقيًا لك أثناء تطوير صندوق قد لا يكون مريحًا جدًا
للمستخدمين. قد ترغب في تنظيم الهياكل الخاصة بك في
تسلسل هرمي يحتوي على مستويات متعددة، ولكن بعد ذلك قد يواجه الأشخاص الذين يريدون استخدام نوع
حددته بعمق في التسلسل الهرمي صعوبة في معرفة وجود هذا النوع.
قد يكونون أيضًا منزعجين من الاضطرار لإدخال `use
my_crate::some_module::another_module::UsefulType;` بدلاً من `use
my_crate::UsefulType;`.

الأخبار الجيدة هي أنه إذا لم يكن الهيكل مريحًا للآخرين للاستخدام
من مكتبة أخرى، فلا يتعين عليك إعادة ترتيب تنظيمك الداخلي:
بدلاً من ذلك، يمكنك إعادة تصدير العناصر لإنشاء هيكل عام مختلف
عن هيكلك الخاص باستخدام `pub use`. *إعادة التصدير* (Re-exporting) تأخذ عنصرًا عامًا
في موقع واحد وتجعله عامًا في موقع آخر، كما لو كان
معرّفًا في الموقع الآخر بدلاً من ذلك.

على سبيل المثال، لنفترض أننا أنشأنا مكتبة باسم `art` لنمذجة المفاهيم الفنية.
داخل هذه المكتبة، توجد وحدتان: وحدة `kinds` تحتوي على تعدادين
باسم `PrimaryColor` و `SecondaryColor` ووحدة `utils` تحتوي على
دالة باسم `mix`، كما هو موضح في Listing 14-3.

<Listing number="14-3" file-name="src/lib.rs" caption="مكتبة `art` مع عناصر منظمة في وحدات `kinds` و `utils`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

</Listing>

يُظهر الشكل 14-3 كيف ستبدو الصفحة الأمامية من التوثيق لهذا الصندوق
الذي تم إنشاؤه بواسطة `cargo doc`.

<img alt="Rendered documentation for the `art` crate that lists the `kinds` and `utils` modules" src="img/trpl14-03.png" class="center" />

<span class="caption">الشكل 14-3: الصفحة الأمامية من التوثيق لـ `art`
التي تسرد وحدات `kinds` و `utils`</span>

لاحظ أن نوعي `PrimaryColor` و `SecondaryColor` غير مدرجين على
الصفحة الأمامية، ولا الدالة `mix`. يجب علينا النقر على `kinds` و `utils` لـ
رؤيتهم.

صندوق آخر يعتمد على هذه المكتبة سيحتاج إلى تعليمات `use`
تحضر العناصر من `art` إلى النطاق، مع تحديد هيكل الوحدة الـ
محدد حاليًا. يُظهر Listing 14-4 مثالاً على صندوق يستخدم
عناصر `PrimaryColor` و `mix` من صندوق `art`.

<Listing number="14-4" file-name="src/main.rs" caption="صندوق يستخدم عناصر صندوق `art` مع تصدير هيكله الداخلي">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

</Listing>

مؤلف الكود في Listing 14-4، الذي يستخدم صندوق `art`، كان عليه
معرفة أن `PrimaryColor` موجود في وحدة `kinds` و `mix` موجود في
وحدة `utils`. هيكل وحدة صندوق `art` أكثر صلة بـ
المطورين الذين يعملون على صندوق `art` من أولئك الذين يستخدمونه. الهيكل الداخلي
لا يحتوي على أي معلومات مفيدة لشخص يحاول
فهم كيفية استخدام صندوق `art`، بل يسبب التباسًا لأن
المطورين الذين يستخدمونه عليهم معرفة أين ينظرون، ويجب تحديد
أسماء الوحدات في تعليمات `use`.

لإزالة التنظيم الداخلي من API العام، يمكننا تعديل كود
صندوق `art` في Listing 14-3 لإضافة تعليمات `pub use` لإعادة تصدير
العناصر في المستوى الأعلى، كما هو موضح في Listing 14-5.

<Listing number="14-5" file-name="src/lib.rs" caption="إضافة تعليمات `pub use` لإعادة تصدير العناصر">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

</Listing>

توثيق API الذي ينشئه `cargo doc` لهذا الصندوق سيسرد الآن
ويربط عمليات إعادة التصدير في الصفحة الأمامية، كما هو موضح في الشكل 14-4، مما يجعل
نوعي `PrimaryColor` و `SecondaryColor` والدالة `mix` أسهل للعثور عليها.

<img alt="Rendered documentation for the `art` crate with the re-exports on the front page" src="img/trpl14-04.png" class="center" />

<span class="caption">الشكل 14-4: الصفحة الأمامية من التوثيق لـ `art`
التي تسرد عمليات إعادة التصدير</span>

يمكن لمستخدمي صندوق `art` رؤية واستخدام الهيكل الداخلي من Listing
14-3 كما هو موضح في Listing 14-4، أو يمكنهم استخدام الهيكل الأكثر ملاءمة
في Listing 14-5، كما هو موضح في Listing 14-6.

<Listing number="14-6" file-name="src/main.rs" caption="برنامج يستخدم العناصر المُعاد تصديرها من صندوق `art`">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

</Listing>

في الحالات التي توجد فيها العديد من الوحدات المتداخلة، إعادة تصدير الأنواع في المستوى الأعلى
باستخدام `pub use` يمكن أن يُحدث فرقًا كبيرًا في تجربة
الأشخاص الذين يستخدمون الصندوق. استخدام شائع آخر لـ `pub use` هو إعادة تصدير
تعريفات تبعية في الصندوق الحالي لجعل تعريفات ذلك الصندوق
جزءًا من API العام لصندوقك.

إنشاء هيكل API عام مفيد هو أكثر فن من علم، ويمكنك
التكرار للعثور على API الذي يعمل بشكل أفضل للمستخدمين. اختيار `pub use`
يمنحك مرونة في كيفية هيكلة صندوقك داخليًا ويفصل
هذا الهيكل الداخلي عما تقدمه للمستخدمين. انظر إلى بعض
كود الصناديق التي قمت بتثبيتها لترى ما إذا كان هيكلها الداخلي يختلف
عن API العام الخاص بها.

### إعداد حساب Crates.io

قبل أن تتمكن من نشر أي صناديق، تحتاج إلى إنشاء حساب على
[crates.io](https://crates.io/)<!-- ignore --> والحصول على رمز API. للقيام بذلك،
قم بزيارة الصفحة الرئيسية على [crates.io](https://crates.io/)<!-- ignore --> وسجل
الدخول عبر حساب GitHub. (حساب GitHub مطلوب حاليًا، ولكن
الموقع قد يدعم طرقًا أخرى لإنشاء حساب في المستقبل.) بمجرد
تسجيل الدخول، قم بزيارة إعدادات حسابك في
[https://crates.io/me/](https://crates.io/me/)<!-- ignore --> واسترجع
مفتاح API الخاص بك. ثم، قم بتشغيل أمر `cargo login` والصق مفتاح API الخاص بك عند المطالبة، هكذا:

```console
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```

سيُخبر هذا الأمر Cargo برمز API الخاص بك ويخزنه محليًا في
_~/.cargo/credentials.toml_. لاحظ أن هذا الرمز سر: لا تشاركه
مع أي شخص آخر. إذا شاركته مع أي شخص لأي سبب، يجب
إبطاله وإنشاء رمز جديد على [crates.io](https://crates.io/)<!-- ignore
-->.

### إضافة بيانات وصفية إلى صندوق جديد

لنفترض أن لديك صندوقًا تريد نشره. قبل النشر، ستحتاج إلى
إضافة بعض البيانات الوصفية في قسم `[package]` من ملف _Cargo.toml_
الخاص بالصندوق.

سيحتاج صندوقك إلى اسم فريد. أثناء العمل على صندوق محليًا،
يمكنك تسمية الصندوق بما تريد. ومع ذلك، أسماء الصناديق على
[crates.io](https://crates.io/)<!-- ignore --> يتم توزيعها على أساس من يأتي أولاً،
يُخدم أولاً. بمجرد أخذ اسم صندوق، لا يمكن لأي شخص آخر نشر صندوق
بهذا الاسم. قبل محاولة نشر صندوق، ابحث عن الاسم الذي
تريد استخدامه. إذا تم استخدام الاسم، ستحتاج إلى إيجاد اسم آخر وتحرير
حقل `name` في ملف _Cargo.toml_ تحت قسم `[package]` لـ
استخدام الاسم الجديد للنشر، هكذا:

<span class="filename">اسم الملف: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

حتى لو اخترت اسمًا فريدًا، عندما تشغل `cargo publish` لنشر
الصندوق في هذه المرحلة، ستحصل على تحذير ثم خطأ:

<!-- manual-regeneration
Create a new package with an unregistered name, making no further modifications
  to the generated package, so it is missing the description and license fields.
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error (status 400 Bad Request): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for more information on configuring these fields
```

هذا يؤدي إلى خطأ لأنك تفتقد بعض المعلومات الحاسمة: وصف
ورخصة مطلوبان حتى يعرف الناس ماذا يفعل صندوقك
وبموجب أي شروط يمكنهم استخدامه. في _Cargo.toml_، أضف وصفًا
يكون مجرد جملة أو جملتين، لأنه سيظهر مع صندوقك في نتائج البحث.
بالنسبة لحقل `license`، تحتاج إلى إعطاء _قيمة معرف رخصة_.
يسرد [Linux Foundation's Software Package Data Exchange (SPDX)][spdx]
المعرفات التي يمكنك استخدامها لهذه القيمة. على سبيل المثال، للإشارة إلى أنك
رخّصت صندوقك باستخدام MIT License، أضف معرف `MIT`:

<span class="filename">اسم الملف: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
license = "MIT"
```

إذا كنت تريد استخدام رخصة لا تظهر في SPDX، فأنت بحاجة إلى وضع
نص تلك الرخصة في ملف، وتضمين الملف في مشروعك، ثم
استخدم `license-file` لتحديد اسم ذلك الملف بدلاً من استخدام
مفتاح `license`.

الإرشادات حول الرخصة المناسبة لمشروعك خارج نطاق
هذا الكتاب. يرخص العديد من الأشخاص في مجتمع Rust مشاريعهم في
نفس الطريقة التي تستخدمها Rust باستخدام رخصة مزدوجة من `MIT OR Apache-2.0`. هذه الممارسة
توضح أنه يمكنك أيضًا تحديد معرفات رخصة متعددة مفصولة
بـ `OR` للحصول على رخص متعددة لمشروعك.

مع اسم فريد، والإصدار، والوصف، والرخصة المضافة، يمكن أن يبدو
ملف _Cargo.toml_ لمشروع جاهز للنشر هكذا:

<span class="filename">اسم الملف: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

[وثائق Cargo](https://doc.rust-lang.org/cargo/) تصف البيانات الوصفية الأخرى
التي يمكنك تحديدها للتأكد من أن الآخرين يمكنهم اكتشاف واستخدام صندوقك
بسهولة أكبر.

### النشر على Crates.io

الآن بعد أن أنشأت حسابًا، وحفظت رمز API الخاص بك، واخترت اسمًا لـ
صندوقك، وحددت البيانات الوصفية المطلوبة، أنت مستعد للنشر!
نشر صندوق يرفع إصدارًا محددًا إلى
[crates.io](https://crates.io/)<!-- ignore --> لاستخدام الآخرين.

كن حذرًا، لأن النشر _دائم_. لا يمكن أبدًا
الكتابة فوق الإصدار، ولا يمكن حذف الكود إلا في ظروف معينة.
أحد الأهداف الرئيسية لـ Crates.io هو العمل كأرشيف دائم للكود بحيث
تستمر بنيات جميع المشاريع التي تعتمد على صناديق من
[crates.io](https://crates.io/)<!-- ignore --> في العمل. السماح بـ
حذف الإصدارات سيجعل تحقيق هذا الهدف مستحيلاً. ومع ذلك، لا يوجد
حد لعدد إصدارات الصناديق التي يمكنك نشرها.

قم بتشغيل أمر `cargo publish` مرة أخرى. يجب أن ينجح الآن:

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
    Packaged 6 files, 1.2KiB (895.0B compressed)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
    Uploaded guessing_game v0.1.0 to registry `crates-io`
note: waiting for `guessing_game v0.1.0` to be available at registry
`crates-io`.
You may press ctrl-c to skip waiting; the crate should be available shortly.
   Published guessing_game v0.1.0 at registry `crates-io`
```

تهانينا! لقد شاركت الآن كودك مع مجتمع Rust، و
يمكن لأي شخص إضافة صندوقك بسهولة كتبعية لمشروعه.

### نشر إصدار جديد من صندوق موجود

عندما تجري تغييرات على صندوقك وتكون جاهزًا لإصدار إصدار جديد،
تقوم بتغيير قيمة `version` المحددة في ملف _Cargo.toml_ الخاص بك و
تعيد النشر. استخدم [قواعد Semantic Versioning][semver] لتحديد ما هو
رقم الإصدار التالي المناسب، بناءً على أنواع التغييرات التي أجريتها.
ثم، قم بتشغيل `cargo publish` لرفع الإصدار الجديد.

<!-- Old headings. Do not remove or links may break. -->

<a id="removing-versions-from-cratesio-with-cargo-yank"></a>
<a id="deprecating-versions-from-cratesio-with-cargo-yank"></a>

### إهمال الإصدارات من Crates.io

على الرغم من أنك لا تستطيع إزالة الإصدارات السابقة من صندوق، يمكنك منع أي
مشاريع مستقبلية من إضافتها كتبعية جديدة. هذا مفيد عندما يكون
إصدار صندوق معطلاً لسبب أو لآخر. في مثل هذه الحالات، يدعم Cargo
سحب (yanking) إصدار صندوق.

_السحب_ (Yanking) لإصدار يمنع المشاريع الجديدة من الاعتماد على هذا الإصدار بينما
يسمح لجميع المشاريع الموجودة التي تعتمد عليه بالاستمرار. في الأساس،
السحب يعني أن جميع المشاريع التي لديها _Cargo.lock_ لن تتعطل، وأي
ملفات _Cargo.lock_ مستقبلية لن تستخدم الإصدار المسحوب.

لسحب إصدار من صندوق، في دليل الصندوق الذي
نشرته سابقًا، قم بتشغيل `cargo yank` وحدد الإصدار الذي تريد
سحبه. على سبيل المثال، إذا نشرنا صندوقًا باسم `guessing_game` الإصدار
1.0.1 ونريد سحبه، فسنشغل التالي في دليل المشروع
لـ `guessing_game`:

<!-- manual-regeneration:
cargo yank carol-test --version 2.1.0
cargo yank carol-test --version 2.1.0 --undo
-->

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```

بإضافة `--undo` إلى الأمر، يمكنك أيضًا التراجع عن السحب والسماح للمشاريع
بالبدء في الاعتماد على إصدار مرة أخرى:

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

السحب _لا_ يحذف أي كود. لا يمكنه، على سبيل المثال، حذف أسرار مرفوعة
عن طريق الخطأ. إذا حدث ذلك، يجب عليك إعادة تعيين تلك الأسرار فورًا.

[spdx]: https://spdx.org/licenses/
[semver]: https://semver.org/
