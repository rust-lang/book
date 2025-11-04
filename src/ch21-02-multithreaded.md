<!-- Old headings. Do not remove or links may break. -->

<a id="turning-our-single-threaded-server-into-a-multithreaded-server"></a>
<a id="from-single-threaded-to-multithreaded-server"></a>

## من خادوم أحادي الخيط إلى خادوم متعدد الخيوط

الآن، سيعالج الخادوم server كل طلب request بدوره in turn، مما يعني أنه لن يعالج اتصالاً connection ثانيًا حتى ينتهي معالجة الاتصال connection الأول. إذا تلقى الخادوم server المزيد والمزيد من الطلبات requests، فإن هذا التنفيذ التسلسلي serial execution سيكون أقل وأقل مثاليةً. إذا تلقى الخادوم server طلبًا request يستغرق وقتًا طويلاً لمعالجته، فسيتعين على الطلبات requests اللاحقة الانتظار حتى ينتهي الطلب request الطويل، حتى لو كان من الممكن معالجة الطلبات requests الجديدة بسرعة. سنحتاج إلى إصلاح هذا، ولكن أولاً سننظر في المشكلة أثناء العمل.

<!-- Old headings. Do not remove or links may break. -->

<a id="simulating-a-slow-request-in-the-current-server-implementation"></a>

### محاكاة طلب بطيء

سننظر في كيف يمكن لطلب request يتم معالجته ببطء أن يؤثر على الطلبات requests الأخرى المقدمة إلى تطبيق خادومنا server implementation الحالي. تطبق القائمة 21-10 معالجة طلب request إلى _/sleep_ مع استجابة response بطيئة محاكاة simulated ستتسبب في نوم sleeping الخادوم server لمدة خمس ثوانٍ قبل الاستجابة responding.

<Listing number="21-10" file-name="src/main.rs" caption="محاكاة طلب بطيء عن طريق النوم لمدة خمس ثوانٍ">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

</Listing>

انتقلنا من `if` إلى `match` الآن بعد أن أصبح لدينا ثلاث حالات three cases. نحتاج إلى مطابقة pattern-match صريحة explicitly على شريحة slice من `request_line` للمطابقة مع القيم الحرفية string literal values؛ لا يقوم `match` بالإشارة المرجعية referencing والإلغاء المرجعية dereferencing التلقائية، مثل طريقة method المساواة equality.

الذراع الأولى first arm هي نفسها كتلة `if` block من القائمة 21-9. تطابق الذراع الثانية second arm طلبًا request إلى _/sleep_. عند استقبال هذا الطلب request، سينام الخادوم server لمدة خمس ثوانٍ قبل عرض صفحة HTML الناجحة. الذراع الثالثة third arm هي نفسها كتلة `else` block من القائمة 21-9.

يمكنك أن ترى كم هو بدائي primitive خادومنا server: المكتبات الحقيقية Real libraries ستتعامل مع التعرف recognition على طلبات متعددة multiple requests بطريقة أقل إسهابًا verbose بكثير!

ابدأ الخادوم server باستخدام `cargo run`. ثم افتح نافذتي متصفح two browser windows: واحدة لـ _http://127.0.0.1:7878_ والأخرى لـ _http://127.0.0.1:7878/sleep_. إذا أدخلت URI _/_ عدة مرات، كما كان من قبل، فسترى أنه يستجيب بسرعة. ولكن إذا أدخلت _/sleep_ ثم حمّلت _/_، فسترى أن _/_ ينتظر حتى ينام `sleep` لمدة خمس ثوانٍ كاملة قبل التحميل loading.

هناك تقنيات techniques متعددة يمكننا استخدامها لتجنب تراكم backing up الطلبات requests خلف طلب request بطيء slow request، بما في ذلك استخدام async كما فعلنا في الفصل 17؛ التقنية التي سننفذها هي مجمع خيوط thread pool.

### تحسين الإنتاجية باستخدام مجمع خيوط

_thread pool_ (مجمع خيوط) هو مجموعة من الخيوط threads المولدة spawned التي هي جاهزة وتنتظر معالجة مهمة task. عندما يتلقى البرنامج مهمة جديدة new task، فإنه يعيّن assign أحد الخيوط threads في المجمع pool إلى المهمة task، وسيعالج هذا الخيط thread المهمة task. ستكون الخيوط threads المتبقية في المجمع pool متاحة لمعالجة أي مهام أخرى tasks تأتي بينما يعالج الخيط thread الأول. عندما ينتهي الخيط thread الأول من معالجة مهمته task، يتم إرجاعه إلى مجمع pool الخيوط threads الخاملة idle threads، جاهزًا لمعالجة مهمة جديدة new task. يتيح لك مجمع خيوط thread pool معالجة الاتصالات connections بشكل متزامن concurrently، مما يزيد من إنتاجية throughput خادومك server.

سنحد من عدد الخيوط threads في المجمع pool إلى عدد صغير small number لحمايتنا من هجمات DoS attacks؛ إذا كان برنامجنا ينشئ خيطًا thread جديدًا لكل طلب request عند وصوله، فإن شخصًا يقدم 10 ملايين طلب request إلى خادومنا server يمكن أن يحدث فوضى havoc عن طريق استنفاد using up جميع موارد resources خادومنا server ووقف grinding معالجة الطلبات requests إلى حد halt.

بدلاً من توليد spawning خيوط threads غير محدودة unlimited، إذن، سيكون لدينا عدد ثابت fixed number من الخيوط threads في انتظار waiting في المجمع pool. يتم إرسال الطلبات Requests التي تأتي إلى المجمع pool للمعالجة processing. سيحتفظ المجمع pool بطابور queue من الطلبات requests الواردة incoming. سيستخرج pop off كل من الخيوط threads في المجمع pool طلبًا request من هذا الطابور queue، ويعالج handle الطلب request، ثم يطلب من الطابور queue طلبًا request آخر. مع هذا التصميم design، يمكننا معالجة process ما يصل إلى _`N`_ طلبًا request بشكل متزامن concurrently، حيث _`N`_ هو عدد الخيوط threads. إذا كان كل خيط thread يستجيب responding لطلب request طويل التشغيل long-running، فلا يزال بإمكان الطلبات requests اللاحقة subsequent أن تتراكم back up في الطابور queue، لكننا زدنا عدد الطلبات requests طويلة التشغيل long-running التي يمكننا معالجتها handle قبل الوصول إلى تلك النقطة that point.

هذه التقنية technique هي واحدة فقط من طرق عديدة many ways لتحسين إنتاجية throughput خادوم ويب web server. الخيارات الأخرى Other options التي قد تستكشفها هي نموذج fork/join model، ونموذج async I/O أحادي الخيط single-threaded، ونموذج async I/O متعدد الخيوط multithreaded. إذا كنت مهتمًا interested بهذا الموضوع topic، يمكنك قراءة المزيد عن الحلول solutions الأخرى ومحاولة تنفيذها implement؛ مع لغة language منخفضة المستوى low-level مثل Rust، كل هذه الخيارات options ممكنة possible.

قبل أن نبدأ في تنفيذ implementing مجمع خيوط thread pool، دعونا نتحدث عن ما يجب أن يبدو عليه استخدام using المجمع pool. عندما تحاول تصميم design الكود code، يمكن أن تساعد كتابة واجهة العميل client interface أولاً في توجيه guide تصميمك design. اكتب API للكود code بحيث يكون منظمًا structured بالطريقة التي تريد استدعاءه call بها؛ ثم نفذ implement الوظيفة functionality ضمن within تلك البنية structure بدلاً من تنفيذ implementing الوظيفة functionality ثم تصميم designing واجهة API العامة public.

مشابهًا لكيفية استخدامنا للتطوير المُوجَّه بالاختبار test-driven development في المشروع project في الفصل 12، سنستخدم التطوير المُوجَّه بالمترجم compiler-driven development هنا. سنكتب الكود code الذي يستدعي calls الدوال functions التي نريدها، ثم سننظر look في الأخطاء errors من المترجم compiler لنحدد determine ما يجب أن نغيره change بعد ذلك next للحصول على الكود code ليعمل to work. قبل أن نفعل ذلك do that، سنستكشف explore التقنية technique التي لن نستخدمها we're not going to use كنقطة بداية starting point.

<!-- Old headings. Do not remove or links may break. -->

<a id="code-structure-if-we-could-spawn-a-thread-for-each-request"></a>

#### توليد خيط لكل طلب

أولاً، لنستكشف explore كيف قد يبدو كودنا code إذا أنشأ created خيطًا thread جديدًا new لكل اتصال connection. كما ذُكر mentioned سابقًا earlier، هذه ليست خطتنا النهائية final plan بسبب المشاكل problems مع إمكانية potentially توليد spawning عدد غير محدود unlimited number من الخيوط threads، لكنها نقطة بداية starting point للحصول على خادوم server متعدد الخيوط multithreaded يعمل working أولاً first. ثم سنضيف add مجمع الخيوط thread pool كتحسين improvement، وسيكون التباين contrasting بين الحلين two solutions أسهل easier.

تُظهر القائمة 21-11 التغييرات changes لعملها make على `main` لتوليد spawn خيط thread جديد new لمعالجة handle كل تدفق stream ضمن within حلقة `for` loop.

<Listing number="21-11" file-name="src/main.rs" caption="توليد خيط جديد لكل تدفق">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

</Listing>

كما تعلمت learned في الفصل 16، سينشئ `thread::spawn` خيطًا thread جديدًا new ثم يشغّل run الكود code في الإغلاق closure في الخيط thread الجديد new. إذا شغّلت ran هذا الكود code وحمّلت loaded _/sleep_ في متصفحك browser، ثم _/_ في علامتي two تبويب browser tabs أخريين more، فستجد indeed بالفعل أن الطلبات requests إلى _/_ لا يتعين عليها أن have to تنتظر wait حتى ينتهي _/sleep_ من finish. ومع ذلك However، كما ذكرنا mentioned، سيُغرق overwhelm هذا في النهاية eventually النظام system لأنك ستصنع making خيوطًا threads جديدة new بدون any حد limit.

قد تتذكر recall أيضًا also من الفصل 17 أن هذا exactly بالضبط هو نوع the kind الحالة situation التي تتألق shine فيها async و await حقًا really! احتفظ Keep بذلك في ذهنك mind بينما نبني build مجمع الخيوط thread pool وفكّر think في كيف how ستبدو would look الأشياء things مختلفة different أو نفسها same مع async.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-similar-interface-for-a-finite-number-of-threads"></a>

#### إنشاء عدد محدود من الخيوط

نريد want أن يعمل works مجمع خيوطنا thread pool بطريقة way مماثلة similar ومألوفة familiar بحيث لا يتطلب switching الانتقال من الخيوط threads إلى مجمع خيوط thread pool تغييرات changes كبيرة large في الكود code الذي يستخدم uses واجهة API الخاصة بنا. تُظهر القائمة 21-12 الواجهة interface الافتراضية hypothetical لبنية `ThreadPool` struct التي نريد want استخدامها use بدلاً من instead of `thread::spawn`.

<Listing number="21-12" file-name="src/main.rs" caption="واجهة `ThreadPool` المثالية الخاصة بنا">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

</Listing>

نستخدم use `ThreadPool::new` لإنشاء create مجمع خيوط thread pool جديد new بعدد configurable من الخيوط threads قابل للتكوين، في هذه الحالة case أربعة four. ثم Then، في حلقة `for` loop، لدى `pool.execute` واجهة interface مماثلة similar لـ `thread::spawn` من حيث أنه يأخذ takes إغلاقًا closure يجب should أن يشغّله run المجمع pool لكل تدفق stream. نحتاج need إلى تنفيذ implement `pool.execute` بحيث so it يأخذ takes الإغلاق closure ويعطيه gives it إلى خيط thread في المجمع pool ليشغّله run. لن يُترجم compile هذا الكود code بعد yet، لكن سنحاول try حتى so that يتمكن can المترجم compiler من توجيهنا guide في كيفية how to إصلاحه fix it.

<!-- Old headings. Do not remove or links may break. -->

<a id="building-the-threadpool-struct-using-compiler-driven-development"></a>

#### بناء `ThreadPool` باستخدام التطوير المُوجَّه بالمترجم

قم بعمل Make التغييرات changes في القائمة 21-12 إلى _src/main.rs_، ثم لنستخدم let's use أخطاء errors المترجم compiler من `cargo check` لقيادة drive تطويرنا development. فيما يلي Here is الخطأ الأول first error الذي نحصل get عليه:

```console
{{#include ../listings/ch21-web-server/listing-21-12/output.txt}}
```

عظيم Great! يخبرنا tells هذا الخطأ error أننا نحتاج need إلى نوع type أو وحدة module `ThreadPool`، لذا so سنبني build واحدًا one الآن now. سيكون will be تطبيق implementation `ThreadPool` الخاص بنا مستقلاً independent عن نوع kind العمل work الذي يقوم به does خادوم الويب web server الخاص بنا. لذا So، لنحوّل switch حزمة crate `hello` من حزمة crate ثنائية binary crate إلى حزمة crate مكتبة library crate لحمل hold تطبيق implementation `ThreadPool` الخاص بنا. بعد After أن نغيّر change إلى حزمة crate مكتبة library crate، يمكننا also أيضًا استخدام use مكتبة library مجمع الخيوط thread pool المنفصلة separate لأي عمل work نريد want القيام به do باستخدام using مجمع خيوط thread pool، وليس not just فقط لخدمة serving طلبات requests الويب web.

أنشئ Create ملفًا file _src/lib.rs_ يحتوي contains على الآتي following، وهو أبسط simplest تعريف definition لبنية `ThreadPool` struct يمكننا can have أن نمتلكه الآن for now:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

</Listing>


ثم Then، حرّر edit ملف _main.rs_ لجلب bring `ThreadPool` إلى النطاق scope من حزمة crate المكتبة library crate بإضافة adding الكود code التالي following إلى أعلى top of _src/main.rs_:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

</Listing>

لن يعمل work هذا الكود code still بعد، لكن but لنتحقق check منه it مرة أخرى again للحصول get على الخطأ error التالي next الذي نحتاج need إلى معالجته address:

```console
{{#include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

يشير indicates هذا الخطأ error أننا نحتاج need بعد ذلك next إلى إنشاء create دالة function مرتبطة associated باسم named `new` لـ `ThreadPool`. نعلم know أيضًا also أن `new` يجب needs أن يكون لها have معامل parameter واحد one يمكن can accept أن يقبل `4` كوسيطة argument argument ويجب should أن تُرجع return نسخة instance من `ThreadPool`. لنطبق implement أبسط simplest دالة function `new` ستكون will have لها تلك those الخصائص characteristics:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

</Listing>

اخترنا chose `usize` كنوع type لمعامل parameter `size` لأننا because نعلم know أن عددًا number سالبًا negative من الخيوط threads لا يكون makes لا معنى any sense له. نعلم know أيضًا also أننا سنستخدم use هذا this `4` كعدد number of من العناصر elements في مجموعة collection من الخيوط threads، وهو what ما الذي for يُستخدم له نوع type `usize`، كما as تمت مناقشته discussed في قسم section ["Integer Types"][integer-types]<!--
ignore --> في الفصل 3.

لنتحقق check من الكود code مرة أخرى again:

```console
{{#include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

الآن Now يحدث occurs الخطأ error لأننا because ليس لدينا طريقة method `execute` على `ThreadPool`. تذكّر Recall من قسم section ["Creating a Finite Number of
Threads"](#creating-a-finite-number-of-threads)<!-- ignore --> أننا قررنا decided أن مجمع خيوطنا thread pool يجب should أن يكون have له واجهة interface مماثلة similar لـ `thread::spawn`. بالإضافة addition، سننفذ implement دالة function `execute` بحيث so it تأخذ take الإغلاق closure الذي أُعطيت it's given وتعطيه gives it إلى خيط thread خامل idle في المجمع pool ليشغّله run.

سنحدّد define طريقة method `execute` على `ThreadPool` لتأخذ take إغلاقًا closure كمعامل parameter. تذكّر Recall من قسم section ["Moving Captured Values Out of
Closures"][moving-out-of-closures]<!-- ignore --> في الفصل 13 أننا يمكننا can take أخذ إغلاقات closures كمعاملات parameters باستخدام with ثلاث three سمات traits مختلفة different: `Fn`، `FnMut`، و `FnOnce`. نحتاج need إلى تحديد decide أي which kind نوع من الإغلاق closure نستخدمه use هنا here. نعلم know أننا سننتهي end up بفعل doing شيء something مماثل similar للتطبيق implementation `thread::spawn` للمكتبة القياسية standard library، لذا so يمكننا can look أن ننظر في ما what bounds القيود التي تمتلكها has توقيع signature `thread::spawn` على معامله parameter. يُظهر shows لنا التوثيق documentation الآتي following:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

معامل parameter النوع type `F` هو what الذي نهتم concerned به هنا here؛ معامل parameter النوع type `T` متعلق related بالقيمة value المُرجعة return، ولسنا we're not مهتمين concerned بذلك that. يمكننا can see أن نرى أن `spawn` يستخدم uses `FnOnce` كقيد bound السمة trait على `F`. هذا This is probably ربما what ما نريده want أيضًا as well، لأننا because سنمرر eventually pass في النهاية الوسيطة argument التي نحصل get عليها في `execute` إلى `spawn`. يمكننا can be أن نكون واثقين further confident أكثر أن `FnOnce` هي السمة trait التي نريد want استخدامها use لأن because الخيط thread لتشغيل running طلب request سيُنفّذ execute فقط only إغلاق closure ذلك الطلب request مرة واحدة one time، وهو which matches ما يطابق `Once` في `FnOnce`.

معامل parameter النوع type `F` لديه has أيضًا also قيد bound السمة trait `Send` وقيد bound العمر lifetime `'static`، والتي which are مفيدة useful في موقفنا situation: نحتاج need `Send` لنقل transfer الإغلاق closure من خيط thread واحد one إلى آخر another و `'static` لأننا because لا don't know نعرف كم how long سيستغرق take الخيط thread للتنفيذ execute. لننشئ create طريقة method `execute` على `ThreadPool` ستأخذ take معاملاً parameter عامًا generic من نوع type `F` مع with هذه these القيود bounds:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

</Listing>

ما زلنا still use نستخدم `()` بعد after `FnOnce` لأن because هذا this `FnOnce` يمثل represents إغلاقًا closure لا يأخذ takes no معاملات parameters ويُرجع returns نوع type الوحدة unit type `()`. مثل Just like تعريفات definitions الدوال functions، يمكن can be omitted حذف نوع type الإرجاع return من التوقيع signature، لكن but حتى even if لو لم يكن have لدينا معاملات parameters، ما زلنا still need نحتاج إلى الأقواس parentheses.

مرة أخرى Again، هذا this is أبسط simplest تطبيق implementation لطريقة method `execute`: لا تفعل does nothing شيئًا، لكن but نحن we're only trying فقط نحاول جعل making كودنا code يُترجم compile. لنتحقق check منه it مرة أخرى again:

```console
{{#include ../listings/ch21-web-server/no-listing-03-define-execute/output.txt}}
```

يُترجم compiles! ولكن But لاحظ note أنه if إذا حاولت tried `cargo run` وقدّمت make طلبًا request في المتصفح browser، فسترى see الأخطاء errors في المتصفح browser التي رأيناها saw في بداية beginning of الفصل chapter. مكتبتنا library لا ليست actually calling تستدعي فعلاً الإغلاق closure الممرر passed إلى `execute` بعد yet!

> ملاحظة Note: قول saying قد قد might hear تسمعه about عن اللغات languages ذات with المترجمات compilers الصارمة strict، مثل such as Haskell و Rust، هو "If the code compiles, it works." لكن but هذا القول saying ليس not universally true صحيحًا عالميًا. مشروعنا project يُترجع compiles، لكن but لا it does absolutely nothing يفعل شيئًا على الإطلاق! إذا If كنا were building نبني مشروعًا project حقيقيًا real، كاملاً complete، فهذا this would be سيكون وقتًا good time جيدًا لبدء start كتابة writing اختبارات tests الوحدة unit لللتحقق check من أن that الكود code يُترجع compiles _and_ ولديه has السلوك behavior الذي نريده want.

فكّر Consider: ما What would be سيكون مختلفًا different هنا here إذا if كنا going to were سنُنفّذ execute مستقبلاً future بدلاً instead of من إغلاق closure؟

#### التحقق من عدد الخيوط في `new`

نحن we aren't doing لا نفعل أي anything شيء بالمعاملات parameters لـ `new` و `execute`. لننفّذ implement أجسام bodies هذه these الدوال functions بالسلوك behavior الذي نريده want. للبدء start، لنفكّر think في `new`. اخترنا chose سابقًا earlier نوعًا type غير موقّع unsigned لمعامل parameter `size` لأن because مجمعًا pool بعدد number سالب negative من الخيوط threads لا makes no sense يكون منطقيًا. ومع ذلك However، مجمع pool بصفر zero خيوط threads أيضًا also لا makes no sense يكون منطقيًا، ومع ذلك yet الصفر zero هو `usize` صالح perfectly valid تمامًا. سنضيف add كودًا code للتحقق check من أن that `size` أكبر greater من صفر zero قبل before أن نُرجع return نسخة instance `ThreadPool`، وسنجعل have البرنامج program يصاب panic بالذعر إذا if تلقى receives صفرًا zero باستخدام using ماكرو macro `assert!`، كما as موضح shown في القائمة 21-13.

<Listing number="21-13" file-name="src/lib.rs" caption="تطبيق `ThreadPool::new` للذعر إذا كان `size` صفرًا">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-13/src/lib.rs:here}}
```

</Listing>

أضفنا added أيضًا also بعض some التوثيق documentation لـ `ThreadPool` مع with تعليقات comments التوثيق doc. لاحظ Note أننا اتبعنا followed ممارسات practices التوثيق documentation الجيدة good بإضافة adding قسم section يستدعي calls out المواقف situations التي which يمكن can أن تصاب panic فيها دالتنا function بالذعر، كما as تمت مناقشته discussed في الفصل 14. حاول Try تشغيل running `cargo doc --open` والنقر clicking على بنية `ThreadPool` struct لترى see ما what يبدو look like التوثيق docs المُنشأ generated لـ `new`!

بدلاً Instead من adding إضافة ماكرو macro `assert!` كما as فعلنا done هنا here، يمكننا could change تغيير `new` إلى `build` وإرجاع return `Result` كما as فعلنا did مع with `Config::build` في مشروع project I/O في القائمة 12-9. لكن But قررنا decided في هذه this الحالة case أن that محاولة trying to create إنشاء مجمع خيوط thread pool بدون without أي any خيوط threads يجب should be أن يكون خطأً error غير قابل للاسترداد unrecoverable. إذا If كنت you're feeling طموحًا ambitious، حاول try to write كتابة دالة function باسم named `build` مع with التوقيع signature التالي following للمقارنة compare مع with دالة function `new`:

```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

#### إنشاء مساحة لتخزين الخيوط

الآن Now بعد that أن لدينا have طريقة way لنعرف know that لدينا have عددًا number صالحًا valid من الخيوط threads للتخزين store في المجمع pool، يمكننا can create إنشاء تلك those الخيوط threads وتخزينها store them في بنية `ThreadPool` struct قبل before إرجاع returning البنية struct. لكن But كيف how do نُخزّن store خيطًا thread؟ لنلقِ take نظرة another look أخرى على توقيع signature `thread::spawn`:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

تُرجع returns دالة function `spawn` `JoinHandle<T>`، حيث where `T` هو النوع type الذي that يُرجعه returns الإغلاق closure. لنحاول try استخدام using `JoinHandle` أيضًا too ونرى see ما what يحدث happens. في حالتنا case، الإغلاقات closures التي we're passing نمررها إلى مجمع الخيوط thread pool ستعالج handle الاتصال connection ولن won't return تُرجع أي anything شيء، لذا so `T` سيكون will be نوع type الوحدة unit type `()`.

سيُترجع compile الكود code في القائمة 21-14، لكن but لا doesn't create ينشئ أي any خيوط threads بعد yet. غيّرنا changed تعريف definition `ThreadPool` ليحمل hold متجهًا vector من نسخ instances `thread::JoinHandle<()>`، وعيّنّا initialized المتجه vector بسعة capacity قدرها of `size`، وأعددنا set up حلقة `for` loop ستُشغّل run بعض some الكود code لإنشاء create الخيوط threads، وأرجعنا returned نسخة instance `ThreadPool` تحتويها containing them.

<Listing number="21-14" file-name="src/lib.rs" caption="إنشاء متجه لـ `ThreadPool` لحمل الخيوط">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-14/src/lib.rs:here}}
```

</Listing>

جلبنا brought `std::thread` إلى النطاق scope في حزمة crate المكتبة library لأننا because نستخدم using `thread::JoinHandle` كنوع type العناصر items في المتجه vector في `ThreadPool`.

بمجرد Once استقبال receiving حجم size صالح valid، ينشئ creates `ThreadPool` الخاص بنا متجهًا vector جديدًا new يمكن can hold أن يحمل عناصر `size` items. تؤدي performs دالة function `with_capacity` نفس same المهمة task مثل as `Vec::new` لكن but مع with فرق difference مهم important: تُخصّص pre-allocates مسبقًا مساحة space في المتجه vector. لأننا Because نعلم know أننا نحتاج need إلى تخزين store عناصر `size` elements في المتجه vector، فإن القيام doing بهذا this التخصيص allocation مقدمًا up front أكثر more كفاءة efficient قليلاً slightly من استخدام using `Vec::new`، الذي which يغيّر resizes نفسه itself بينما as يتم are إدراج inserted العناصر elements.

عندما When تُشغّل run `cargo check` مرة أخرى again، يجب should أن ينجح succeed.

<!-- Old headings. Do not remove or links may break. -->
<a id ="a-worker-struct-responsible-for-sending-code-from-the-threadpool-to-a-thread"></a>

#### إرسال الكود من `ThreadPool` إلى خيط

تركنا left تعليقًا comment في حلقة `for` loop في القائمة 21-14 بخصوص regarding إنشاء creation الخيوط threads. هنا Here، سننظر look في كيفية how نُنشئ actually create فعلاً الخيوط threads. توفر provides المكتبة القياسية standard library `thread::spawn` كطريقة way لإنشاء create خيوط threads، و`thread::spawn` تتوقع expects to get أن تحصل على بعض some الكود code الذي يجب should أن يُشغّله run الخيط thread بمجرد as soon as إنشاء creating الخيط thread. ومع ذلك However، في حالتنا case، نريد want إنشاء create الخيوط threads وجعلها have them تنتظر _wait_ للكود code الذي سنرسله we'll send لاحقًا later. لا doesn't include تتضمن تطبيق implementation المكتبة القياسية standard library للخيوط threads أي any طريقة way للقيام do بذلك that؛ علينا have to أن ننفّذه implement يدويًا manually.

سننفّذ implement هذا this السلوك behavior بإدخال introducing بنية data structure بيانات جديدة new بين between `ThreadPool` والخيوط threads التي ستدير manage هذا this السلوك behavior الجديد new. سنسمّي call بنية data structure البيانات هذه _Worker_، وهو مصطلح term شائع common في تطبيقات implementations التجميع pooling. يلتقط picks up `Worker` الكود code الذي يحتاج needs to be run إلى التشغيل ويُشغّل runs الكود code في خيطه thread.

فكّر Think of في الناس people العاملين working في المطبخ kitchen في مطعم restaurant: ينتظر waits العمّال workers حتى until تأتي come in الطلبات orders من العملاء customers، ثم and then هم they're responsible مسؤولون عن taking أخذ تلك those الطلبات orders وملئها filling them.

بدلاً Instead of من تخزين storing متجه vector من نسخ instances `JoinHandle<()>` في مجمع الخيوط thread pool، سنُخزّن store نسخ instances من بنية `Worker` struct. سيُخزّن store كل every `Worker` نسخة instance واحدة single `JoinHandle<()>`. ثم Then، سننفّذ implement طريقة method على `Worker` ستأخذ take إغلاقًا closure من الكود code ليُشغّل run وترسله send it إلى الخيط thread الذي يعمل already running بالفعل للتنفيذ execution. سنعطي give أيضًا also كل every `Worker` معرّفًا `id` بحيث so that نتمكن can distinguish من التمييز بين نسخ instances `Worker` المختلفة different في المجمع pool عند when التسجيل logging أو التصحيح debugging.

فيما يلي Here is العملية process الجديدة new التي ستحدث happen عندما when ننشئ create `ThreadPool`. سننفّذ implement الكود code الذي يُرسل sends الإغلاق closure إلى الخيط thread بعد after أن يكون have `Worker` مُعدًّا set up بهذه this الطريقة way:

1. حدّد Define بنية `Worker` struct تحمل holds معرّفًا `id` و `JoinHandle<()>`.
2. غيّر Change `ThreadPool` ليحمل hold متجهًا vector من نسخ instances `Worker`.
3. حدّد Define دالة function `Worker::new` تأخذ takes رقم number معرّف `id` وتُرجع return نسخة instance `Worker` تحمل holds المعرّف `id` وخيطًا thread مُولّدًا spawned مع with إغلاق closure فارغ empty.
4. في `ThreadPool::new`، استخدم use عداد counter حلقة `for` loop لتوليد generate معرّف `id`، وأنشئ create `Worker` جديدًا new بذلك that المعرّف `id`، وخزّن store `Worker` في المتجه vector.

إذا If كنت you're up لتحدٍ challenge، حاول try تطبيق implementing هذه these التغييرات changes بنفسك on your own قبل before النظر looking في الكود code في القائمة 21-15.

مستعد Ready؟ فيما يلي Here is القائمة 21-15 مع with إحدى one الطرق ways لعمل make التعديلات modifications المذكورة preceding.

<Listing number="21-15" file-name="src/lib.rs" caption="تعديل `ThreadPool` لحمل نسخ `Worker` بدلاً من حمل الخيوط مباشرة">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-15/src/lib.rs:here}}
```

</Listing>

غيّرنا changed اسم name الحقل field على `ThreadPool` من `threads` إلى `workers` لأنه because الآن now يحمل holding نسخ instances `Worker` بدلاً instead of من نسخ instances `JoinHandle<()>`. نستخدم use العداد counter في حلقة `for` loop كوسيطة argument لـ `Worker::new`، ونُخزّن store كل every `Worker` جديد new في المتجه vector المسمّى named `workers`.

لا doesn't need الكود code الخارجي External (مثل like خادومنا server في _src/main.rs_) أن يعرف know تفاصيل details التطبيق implementation المتعلقة regarding باستخدام using بنية `Worker` struct ضمن within `ThreadPool`، لذا so نجعل make بنية `Worker` struct ودالتها function `new` خاصة private. تستخدم uses دالة function `Worker::new` المعرّف `id` الذي نعطيه give it وتُخزّن stores نسخة instance `JoinHandle<()>` التي يتم are إنشاؤها created بتوليد spawning خيط thread جديد new باستخدام using إغلاق closure فارغ empty.

> ملاحظة Note: إذا If لم can't create يتمكن نظام التشغيل operating system من إنشاء خيط thread لأنه because ليست there aren't enough هناك موارد resources نظام system كافية enough، فسيصاب `thread::spawn` will panic بالذعر. هذا That will cause سيتسبب في إصابة panic خادومنا server بالكامل whole بالذعر، حتى even though على الرغم من أن إنشاء creation بعض some الخيوط threads قد might succeed ينجح. من For أجل simplicity's sake بساطة الأمر، هذا this السلوك behavior جيد fine، لكن but في تطبيق implementation مجمع خيوط thread pool إنتاجي production، من المحتمل you'd likely want أن ترغب في استخدام use
> [`std::thread::Builder`][builder]<!-- ignore --> وطريقته method
> [`spawn`][builder-spawn]<!-- ignore --> التي تُرجع return `Result` بدلاً instead.

سيُترجم compile هذا الكود code ويُخزّن store عدد number نسخ instances `Worker` الذي حددناه specified كوسيطة argument لـ `ThreadPool::new`. لكن but ما زلنا we're _still_ not processing لا نعالج الإغلاق closure الذي نحصل get عليه في `execute`. لننظر look في كيفية how to do ذلك that بعد ذلك next.

#### إرسال الطلبات إلى الخيوط عبر القنوات

المشكلة problem التالية next التي سنتعامل tackle معها هي that أن الإغلاقات closures المُعطاة given لـ `thread::spawn` لا do absolutely nothing تفعل شيئًا على الإطلاق. حاليًا Currently، نحصل get على الإغلاق closure الذي نريد want تنفيذه execute في طريقة method `execute`. لكن But نحتاج need إلى إعطاء give `thread::spawn` إغلاقًا closure ليُشغّله run عندما when ننشئ create كل every `Worker` أثناء during إنشاء creation `ThreadPool`.

نريد want بنيات `Worker` structs التي أنشأناها just created أن تجلب fetch الكود code ليُشغّل run من طابور queue محفوظ held في `ThreadPool` وترسل send ذلك that الكود code إلى خيطه thread ليُشغّله run.

القنوات channels التي تعلّمناها learned about في الفصل 16—طريقة way بسيطة simple للتواصل communicate بين خيطين threads اثنين two—ستكون would be مثالية perfect لحالة case الاستخدام use هذه this. سنستخدم use قناة channel لتعمل function كطابور queue للوظائف jobs، وسيُرسل send `execute` وظيفة job من `ThreadPool` إلى نسخ instances `Worker`، التي ستُرسل send الوظيفة job إلى خيطها thread. فيما يلي Here is الخطة plan:

1. سينشئ create `ThreadPool` قناة channel ويحتفظ hold on بالمُرسِل sender.
2. سيحتفظ hold on كل every `Worker` بالمُستقبِل receiver.
3. سننشئ create بنية `Job` struct جديدة new ستحمل hold الإغلاقات closures التي نريد want إرسالها send أسفل down القناة channel.
4. ستُرسل send طريقة method `execute` الوظيفة job التي تريد want تنفيذها execute عبر through المُرسِل sender.
5. في خيطه thread، سيُكرّر loop `Worker` على مُستقبِله receiver ويُنفّذ execute إغلاقات closures أي any وظائف jobs يتلقّاها receives.

لنبدأ start بإنشاء creating قناة channel في `ThreadPool::new` والاحتفاظ holding بالمُرسِل sender في نسخة instance `ThreadPool`، كما as موضح shown في القائمة 21-16. لا تحمل hold بنية `Job` struct أي anything شيء الآن for now لكن but ستكون will be نوع type العنصر item الذي we're sending نرسله أسفل down القناة channel.

<Listing number="21-16" file-name="src/lib.rs" caption="تعديل `ThreadPool` لتخزين مُرسِل قناة تُرسل نسخ `Job`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-16/src/lib.rs:here}}
```

</Listing>

في `ThreadPool::new`، ننشئ create قناتنا channel الجديدة new ونجعل have المجمع pool يحتفظ hold بالمُرسِل sender. سيُترجم compile هذا بنجاح successfully.

لنحاول try تمرير passing مُستقبِل receiver القناة channel إلى كل every `Worker` بينما as ينشئ creates مجمع الخيوط thread pool القناة channel. نعلم know أننا نريد want استخدام use المُستقبِل receiver في الخيط thread الذي that تولّده spawn نسخ instances `Worker`، لذا so سنُشير reference إلى معامل parameter `receiver` في الإغلاق closure. لن won't quite compile يُترجم الكود code في القائمة 21-17 تمامًا بعد yet.

<Listing number="21-17" file-name="src/lib.rs" caption="تمرير مُستقبِل القناة إلى العمّال">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-17/src/lib.rs:here}}
```

</Listing>

أجرينا made بعض some التغييرات changes الصغيرة small والمباشرة straightforward: مررنا pass المُستقبِل receiver إلى `Worker::new`، ثم then نستخدمه use داخل inside الإغلاق closure.

عندما When نحاول try to check فحص هذا this الكود code، نحصل get على هذا this الخطأ error:

```console
{{#include ../listings/ch21-web-server/listing-21-17/output.txt}}
```

يحاول trying الكود code تمرير pass `receiver` إلى نسخ instances `Worker` متعددة multiple. لن won't work يعمل هذا this، كما as ستتذكّر recall من الفصل 16: تطبيق implementation القناة channel الذي that توفّره provides Rust هو مُنتِج _producer_ متعدد multiple، مُستهلِك _consumer_ واحد single. هذا This means يعني أننا لا can't يمكن فقط just clone استنساخ النهاية end الاستهلاكية consuming من القناة channel لإصلاح fix هذا this الكود code. نحن أيضًا also لا don't want نريد إرسال send رسالة message عدة multiple مرات times إلى مُستهلكين consumers متعددين multiple؛ نريد want قائمة list واحدة one من الرسائل messages مع with نسخ instances `Worker` متعددة multiple بحيث such that تتم is processed تُعالج كل every رسالة message مرة one واحدة time.

بالإضافة Additionally، فإن taking أخذ وظيفة job من طابور queue القناة channel يتضمن involves تعديل mutating `receiver`، لذا so تحتاج need الخيوط threads إلى طريقة way آمنة safe لمشاركة share وتعديل modify `receiver`؛ وإلا otherwise، قد might get نحصل على شروط conditions سباق race (كما as تمت تغطيته covered في الفصل 16).

تذكّر Recall المؤشرات pointers الذكية smart الآمنة safe للخيط thread التي تمت مناقشتها discussed في الفصل 16: لمشاركة share الملكية ownership عبر across خيوط threads متعددة multiple والسماح allow للخيوط threads بتعديل mutate القيمة value، نحتاج need إلى استخدام use `Arc<Mutex<T>>`. سيسمح let النوع type `Arc` لنسخ instances `Worker` متعددة multiple بامتلاك own المُستقبِل receiver، وسيضمن ensure `Mutex` أن `Worker` واحدًا only one فقط يحصل gets على وظيفة job من المُستقبِل receiver في كل at a مرة time. تُظهر show القائمة 21-18 التغييرات changes التي نحتاج need إلى عملها make.

<Listing number="21-18" file-name="src/lib.rs" caption="مشاركة المُستقبِل بين نسخ `Worker` باستخدام `Arc` و `Mutex`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-18/src/lib.rs:here}}
```

</Listing>

في `ThreadPool::new`، نضع put المُستقبِل receiver في `Arc` و `Mutex`. لكل for each `Worker` جديد new، نستنسخ clone `Arc` لزيادة bump عداد count المراجع reference بحيث so that تتمكن can share نسخ instances `Worker` من مشاركة ملكية ownership المُستقبِل receiver.

مع with هذه these التغييرات changes، يُترجم compiles الكود code! نحن We're getting هناك there!

#### تطبيق طريقة `execute`

لننفّذ implement أخيرًا finally طريقة method `execute` على `ThreadPool`. سنغيّر change أيضًا also `Job` من بنية struct إلى اسم name مستعار alias للنوع type لكائن object سمة trait يحمل hold نوع type الإغلاق closure الذي that يتلقّاه receives `execute`. كما as تمت مناقشته discussed في قسم section ["Type Synonyms and Type
Aliases"][type-aliases]<!-- ignore --> في الفصل 20، تسمح allow لنا أسماء type aliases الأنواع المستعارة بجعل making الأنواع types الطويلة long أقصر shorter من for أجل ease of سهولة الاستخدام use. انظر Look إلى القائمة 21-19.

<Listing number="21-19" file-name="src/lib.rs" caption="إنشاء اسم مستعار `Job` للنوع لـ `Box` يحمل كل إغلاق ثم إرسال الوظيفة أسفل القناة">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-19/src/lib.rs:here}}
```

</Listing>

بعد After إنشاء creating نسخة instance `Job` جديدة new باستخدام using الإغلاق closure الذي that نحصل get عليه في `execute`، نُرسل send تلك that الوظيفة job أسفل down نهاية end الإرسال sending من القناة channel. نستدعي calling `unwrap` على `send` لحالة case فشل fail الإرسال sending. قد might happen يحدث هذا this إذا if، على for سبيل example المثال، أوقفنا stop جميع all خيوطنا threads من التنفيذ executing، مما meaning يعني أن النهاية end المُستقبِلة receiving توقّفت stopped عن استقبال receiving رسائل messages جديدة new. في الوقت At the moment الحالي، لا can't stop يمكننا إيقاف خيوطنا threads من التنفيذ executing: تستمر continue خيوطنا threads في التنفيذ executing طالما as long as يوجد exists المجمع pool. السبب reason الذي we use نستخدم فيه `unwrap` هو أننا because نعلم know أن حالة case الفشل failure لن won't happen تحدث، لكن but المترجم compiler لا doesn't know يعرف ذلك that.

لكن But لسنا we're not quite done بعد yet! في `Worker`، إغلاقنا closure المُمرّر being passed إلى `thread::spawn` ما زال still فقط only _references_ يشير إلى النهاية end المُستقبِلة receiving من القناة channel. بدلاً Instead، نحتاج need الإغلاق closure ليُكرّر loop للأبد forever، يسأل asking النهاية end المُستقبِلة receiving من القناة channel عن وظيفة job ويُشغّل running الوظيفة job عندما when يحصل gets على واحدة one. لنعمل make التغيير change الموضّح shown في القائمة 21-20 إلى `Worker::new`.

<Listing number="21-20" file-name="src/lib.rs" caption="استقبال وتنفيذ الوظائف في خيط نسخة `Worker`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-20/src/lib.rs:here}}
```

</Listing>

هنا Here، نستدعي call أولاً first `lock` على `receiver` للحصول acquire على القفل mutex، ثم then نستدعي call `unwrap` للذعر panic على any أي أخطاء errors. قد might fail يفشل الحصول Acquiring على القفل lock إذا if كان المقفل mutex في حالة state مُسمّمة _poisoned_، والتي which يمكن can happen أن تحدث إذا if أصاب panicked خيط thread آخر some other بالذعر أثناء while حمل holding القفل lock بدلاً rather من releasing إطلاق القفل lock. في هذه this الحالة situation، فإن استدعاء calling `unwrap` لجعل having هذا this الخيط thread يصاب panic بالذعر هو الإجراء action الصحيح correct ليُتّخذ to take. لا Feel free تتردد في تغيير change هذا this `unwrap` إلى `expect` مع with رسالة message خطأ error ذات معنى meaningful لك to you.

إذا If حصلنا got على القفل lock على المقفل mutex، نستدعي call `recv` لاستقبال receive `Job` من القناة channel. يتحرّك moves `unwrap` نهائي final عبر past أي any أخطاء errors هنا here أيضًا as well، والتي which قد might occur تحدث إذا if أوقف shut down الخيط thread الذي holding يحمل المُرسِل sender، مشابهًا similar لكيفية how إرجاع returns طريقة method `send` `Err` إذا if أوقف shuts down المُستقبِل receiver.

يحجب blocks استدعاء call `recv`، لذا so إذا if لم يكن there is هناك وظيفة job بعد yet، فسينتظر wait الخيط thread الحالي current حتى until تصبح becomes وظيفة job متاحة available. يضمن ensures `Mutex<T>` أن خيط thread `Worker` واحدًا only one فقط في كل at a مرة time يحاول trying to request طلب وظيفة job.

مجمع خيوطنا thread pool الآن now في حالة state عمل working! أعطه Give it `cargo run` واعمل make بعض some الطلبات requests:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field `workers` is never read
 --> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `hello` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

نجاح Success! الآن لدينا now have مجمع خيوط thread pool ينفّذ executes الاتصالات connections بشكل asynchronously غير متزامن. لا يتم are never أبدًا إنشاء created أكثر more من أربعة four خيوط threads، لذا so لن won't get يحصل نظامنا system على حمل overloaded زائد إذا if تلقّى received الخادوم server الكثير a lot of من الطلبات requests. إذا If قدّمنا make طلبًا request إلى _/sleep_، فسيتمكّن be able الخادوم server من خدمة serve طلبات requests أخرى other عن طريق having جعل خيط thread آخر another يُشغّلها run them.

> ملاحظة Note: إذا If فتحت open _/sleep_ في نوافذ windows متصفح browser متعددة multiple في وقت واحد simultaneously، فقد might load يُحمّلون واحدًا one تلو at a الآخر time في فترات intervals من خمس five ثوانٍ seconds. تُنفّذ execute بعض some متصفحات browsers الويب web نسخًا instances متعددة multiple من نفس same الطلب request بشكل sequentially تسلسلي لأسباب reasons التخزين المؤقت caching. هذا This القيد limitation ليس not caused by تسببه خادوم الويب web server الخاص بنا.

هذا This is وقت time جيد good للتوقّف pause والنظر consider في كيف how سيكون would be الكود code في القوائم Listings 21-18، 21-19، و 21-20 مختلفًا different إذا if كنا using نستخدم مستقبلات futures بدلاً instead of من إغلاق closure للعمل work المُراد to be done ليُنجز. ما What الأنواع types التي ستتغيّر would change؟ كيف How سيكون would توقيعات signatures الطرق methods مختلفة different، إن if كانت at all؟ ما what أجزاء parts الكود code التي ستبقى would stay نفسها same؟

بعد After التعلّم learning عن حلقة `while let` loop في الفصل 17 والفصل 19، قد might be wondering تتساءل لماذا why لم didn't write نكتب كود code خيط thread `Worker` كما as موضح shown في القائمة 21-21.

<Listing number="21-21" file-name="src/lib.rs" caption="تطبيق بديل لـ `Worker::new` باستخدام `while let`">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-21/src/lib.rs:here}}
```

</Listing>

يُترجم compiles هذا this الكود code ويعمل runs لكن but لا doesn't result in لا ينتج عنه سلوك behavior الخيوط threading المطلوب desired: سيتسبّب cause طلب request بطيء slow لا still يزال في انتظار wait الطلبات requests الأخرى other ليتم to be معالجتها processed. السبب reason مُخادع somewhat subtle إلى حد ما: لا تمتلك has بنية `Mutex` struct طريقة method `unlock` عامة public لأن because ملكية ownership القفل lock مبنية based على عمر lifetime `MutexGuard<T>` ضمن within `LockResult<MutexGuard<T>>` الذي that تُرجعه returns طريقة method `lock`. في At وقت compile time الترجمة، يمكن can then للمُدقّق borrow checker فرض enforce القاعدة rule بأن that لا يمكن cannot be accessed الوصول إلى مورد resource محمي guarded بواسطة by `Mutex` ما unless لم نحمل hold القفل lock. ومع ذلك However، يمكن can also result in أن ينتج هذا this التطبيق implementation أيضًا في حمل being held القفل lock لمدة longer أطول من intended المقصود إذا if لم weren't نكن حريصين mindful على عمر lifetime `MutexGuard<T>`.

يعمل works الكود code في القائمة 21-20 الذي that يستخدم uses `let job =
receiver.lock().unwrap().recv().unwrap();` لأنه because مع with `let`، يتم are immediately dropped إسقاط أي any قيم values مؤقتة temporary مُستخدمة used في التعبير expression على الجانب side الأيمن right-hand من علامة sign المساواة equal فور immediately عندما when ينتهي ends عبارة statement `let`. ومع ذلك However، `while
let` (و `if let` و `match`) لا does not drop تُسقط القيم values المؤقتة temporary حتى until نهاية end of الكتلة block المرتبطة associated. في القائمة 21-21، يبقى remains القفل lock محمولاً held لمدة duration استدعاء call `job()`، مما meaning يعني أن نسخ instances `Worker` الأخرى other لا cannot receive يمكنها استقبال وظائف jobs.

[type-aliases]: ch20-03-advanced-types.html#type-synonyms-and-type-aliases
[integer-types]: ch03-02-data-types.html#integer-types
[moving-out-of-closures]: ch13-01-closures.html#moving-captured-values-out-of-closures
[builder]: ../std/thread/struct.Builder.html
[builder-spawn]: ../std/thread/struct.Builder.html#method.spawn
