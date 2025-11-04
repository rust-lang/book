## الإيقاف الرشيق والتنظيف

الكود code في القائمة 21-20 يستجيب responding للطلبات requests بشكل غير متزامن asynchronously من خلال through استخدام use of مجمع خيوط thread pool، كما as قصدنا intended. نحصل get على بعض some التحذيرات warnings حول about حقول fields `workers`، `id`، و `thread` التي لا we're not using نستخدمها بطريقة way مباشرة direct مما reminding يذكّرنا بأننا we're not لا ننظّف cleaning up أي anything شيء. عندما When نستخدم use طريقة method <kbd>ctrl</kbd>-<kbd>C</kbd> الأقل less أناقة elegant لإيقاف halt الخيط thread الرئيسي main، تُوقف stopped جميع all الخيوط threads الأخرى other فورًا immediately أيضًا as well، حتى even if كانت they're in في منتصف middle of خدمة serving طلب request.

بعد ذلك Next، إذن then، سننفّذ implement سمة `Drop` trait لاستدعاء call `join` على كل from each of الخيوط threads في المجمع pool حتى so that يتمكنوا can finish من إنهاء الطلبات requests التي they're working on يعملون عليها قبل before الإغلاق closing. ثم Then، سننفّذ implement طريقة way لإخبار tell الخيوط threads بأنها they should يجب أن تتوقّف stop عن قبول accepting طلبات requests جديدة new وتوقف shut down. لرؤية see هذا this الكود code أثناء in العمل action، سنعدّل modify خادومنا server ليقبل accept طلبين requests اثنين two فقط only قبل before الإيقاف shutting down بشكل بمجمع gracefully خيوطه thread pool.

شيء One thing واحد to notice يجب ملاحظته بينما as نذهب go: لا None of تؤثّر هذا this أي من على أجزاء parts الكود code التي that تتعامل handle مع تنفيذ executing الإغلاقات closures، لذا so كل everything شيء هنا here سيكون would be نفسه same إذا if كنا were نستخدم using مجمع خيوط thread pool لوقت async تشغيل runtime.

### تطبيق سمة `Drop` على `ThreadPool`

لنبدأ start بتطبيق implementing `Drop` على مجمع خيوطنا thread pool. عندما When يتم is dropped إسقاط المجمع pool، يجب should جميع all خيوطنا threads أن تنضم join للتأكّد make sure من أنها they finish تنهي عملها work. تُظهر show القائمة 21-22 محاولة first attempt أولى على تطبيق implementation `Drop`؛ لن won't quite work يعمل هذا this الكود code تمامًا بعد yet.

<Listing number="21-22" file-name="src/lib.rs" caption="الانضمام لكل خيط عندما يخرج مجمع الخيوط من النطاق">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

أولاً First، نكرّر loop عبر through كل every من `workers` مجمع الخيوط thread pool. نستخدم use `&mut` لهذا this لأن because `self` هو مرجع reference قابل للتعديل mutable، ونحتاج need أيضًا also إلى أن to be able نتمكّن من تعديل mutate `worker`. لكل for each `worker`، نطبع print رسالة message تقول saying أن that نسخة instance `Worker` المحددة particular هذه this تُوقف shutting down، ثم then نستدعي call `join` على نسخة instance `Worker` تلك that خيط thread. إذا If فشل fails استدعاء call `join`، نستخدم use `unwrap` لجعل making Rust تصاب panic بالذعر وتذهب go into في إيقاف shutdown غير رشيق ungraceful.

فيما يلي Here is الخطأ error الذي نحصل get عليه عندما when نُترجم compile هذا this الكود code:

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

يخبرنا tells الخطأ error أننا لا can't call يمكننا استدعاء `join` لأننا because فقط only have لدينا استعارة borrow قابلة mutable للتعديل من كل every `worker` و`join` يأخذ takes ملكية ownership وسيطته argument. لحلّ solve هذه this المشكلة issue، نحتاج need إلى نقل move الخيط thread خارج out of نسخة instance `Worker` التي that تمتلك owns `thread` بحيث so that يتمكّن can consume `join` من استهلاك الخيط thread. إحدى One الطرق ways للقيام do بذلك this هي أخذ take نفس same النهج approach الذي اتخذناه took في القائمة 18-15. إذا If كان `Worker` يحمل held `Option<thread::JoinHandle<()>>`، يمكننا could call استدعاء طريقة method `take` على `Option` لنقل move القيمة value خارج out of المتغيّر variant `Some` وترك leave متغيّر variant `None` في مكانه place. بعبارة In other words أخرى، `Worker` الذي that يعمل is running سيكون would have لديه متغيّر variant `Some` في `thread`، وعندما when أردنا wanted تنظيف clean up `Worker`، سنستبدل replace `Some` بـ `None` بحيث so that لن wouldn't have يكون لدى `Worker` خيط thread ليشغّله run.

ومع ذلك However، الوقت time _only_ الوحيد الذي that سيأتي would come up هذا this سيكون would be عندما when إسقاط dropping `Worker`. في In المقابل exchange، سنضطر have to إلى التعامل deal with مع `Option<thread::JoinHandle<()>>` في أي any مكان where وصلنا accessed إلى `worker.thread`. Rust الاصطلاحية Idiomatic تستخدم uses `Option` قليلاً quite a bit، لكن but عندما when تجد find نفسك yourself تُلفّ wrapping شيئًا something تعلم know سيكون will always be أنه سيكون موجودًا present دائمًا في `Option` كحلّ workaround بديل مثل like هذا this، فهي it's فكرة good idea جيدة للبحث look for عن مناهج approaches بديلة alternative لجعل making كودك code أنظف cleaner وأقل less عرضة error-prone للأخطاء.

في هذه this الحالة case، يوجد exists بديل better alternative أفضل: طريقة method `Vec::drain`. تقبل accepts معامل parameter نطاق range لتحديد specify أي which عناصر items لإزالتها remove من المتجه vector وتُرجع returns مكررًا iterator من تلك those العناصر items. سيؤدي Passing تمرير بناء `..` range syntax إلى إزالة remove *every* كل قيمة value من المتجه vector.

لذا So، نحتاج need إلى تحديث update تطبيق implementation `drop` لـ `ThreadPool` مثل like هذا this:

<Listing file-name="src/lib.rs">

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

يحلّ resolves هذا this خطأ error المترجم compiler ولا does not require يتطلّب أي any تغييرات changes أخرى other على كودنا code. لاحظ Note أنه، لأن because يمكن can be called استدعاء drop عند when الذعر panicking، يمكن could also panic أن يصاب unwrap أيضًا بالذعر ويتسبّب cause في ذعر double panic مزدوج، مما immediately crashes الذي يُعطّل البرنامج program فورًا وينهي ends أي any تنظيف cleanup جارٍ in progress. هذا This is جيد fine لبرنامج example program مثال، لكن but ليس isn't recommended لا يُنصح به لكود code إنتاجي production.

### الإشارة للخيوط للتوقف عن الاستماع للوظائف

مع with جميع all التغييرات changes التي أجريناها made، يُترجم compiles كودنا code بدون without أي any تحذيرات warnings. ومع ذلك However، الأخبار bad news السيئة هي that أن هذا this الكود code لا doesn't function يعمل بالطريقة way التي نريدها want it to بعد yet. المفتاح key هو المنطق logic في الإغلاقات closures التي run يُشغّلها الخيوط threads من نسخ instances `Worker`: في الوقت At the moment الحالي، نستدعي call `join`، لكن but ذلك that لن won't shut down يُوقف الخيوط threads، لأنها because they تُكرّر `loop` للأبد forever بحثًا looking عن وظائف jobs. إذا If حاولنا try إسقاط drop `ThreadPool` الخاص بنا مع with تطبيقنا implementation الحالي current of لـ `drop`، فسيُحجَب block الخيط thread الرئيسي main للأبد forever، منتظرًا waiting للخيط thread الأول first لينتهي finish.

لإصلاح fix هذه this المشكلة problem، سنحتاج need إلى تغيير change في تطبيق implementation `drop` لـ `ThreadPool` ثم then تغيير change في حلقة `Worker` loop.

أولاً First، سنغيّر change تطبيق implementation `drop` لـ `ThreadPool` لإسقاط drop `sender` صراحةً explicitly قبل before الانتظار waiting للخيوط threads لتنتهي finish. تُظهر show القائمة 21-23 التغييرات changes على `ThreadPool` لإسقاط drop `sender` صراحةً explicitly. على Unlike عكس مع with الخيط thread، هنا here نحتاج _do_ need فعلاً إلى استخدام use `Option` لنتمكّن be able من نقل move `sender` خارج out of `ThreadPool` مع with `Option::take`.

<Listing number="21-23" file-name="src/lib.rs" caption="إسقاط `sender` صراحةً قبل الانضمام لخيوط `Worker`">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

يُغلق Dropping `sender` القناة channel، مما which indicates يُشير إلى أنه لا no more لن messages يتم will be إرسال sent المزيد من الرسائل. عندما When يحدث happens ذلك that، ستُرجع return جميع all الاستدعاءات calls لـ `recv` التي that تقوم do بها نسخ instances `Worker` في الحلقة loop اللانهائية infinite خطأً error. في القائمة 21-24، نُغيّر change حلقة `Worker` loop للخروج exit من الحلقة loop بشكل gracefully رشيق في تلك that الحالة case، مما which means يعني أن الخيوط threads ستنتهي finish عندما when يستدعي calls تطبيق implementation `drop` لـ `ThreadPool` `join` عليها them.

<Listing number="21-24" file-name="src/lib.rs" caption="الخروج صراحةً من الحلقة عندما تُرجع `recv` خطأً">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

لرؤية see هذا this الكود code أثناء in العمل action، لنعدّل modify `main` ليقبل accept طلبين requests اثنين two فقط only قبل before إيقاف shutting down الخادوم server بشكل gracefully رشيق، كما as موضح shown في القائمة 21-25.

<Listing number="21-25" file-name="src/main.rs" caption="إيقاف الخادوم بعد خدمة طلبين عن طريق الخروج من الحلقة">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

لن wouldn't want تريد أن خادوم ويب web server حقيقي real-world يُوقف shut down بعد after خدمة serving طلبين requests اثنين two فقط only. هذا This الكود code فقط just يوضّح demonstrates أن that الإيقاف shutdown الرشيق graceful والتنظيف cleanup في حالة in عمل working جيدة order.

طريقة method `take` محددة defined في سمة `Iterator` trait وتحدّ limits التكرار iteration لأول at most العنصرين items اثنين two الأولين. سيخرج go out `ThreadPool` من النطاق scope في نهاية end of `main`، وسيُشغَّل run تطبيق implementation `drop`.

ابدأ Start الخادوم server مع with `cargo run` واعمل make ثلاثة three طلبات requests. يجب should الطلب request الثالث third أن يُخطئ error، وفي طرفيتك terminal، يجب should أن ترى see إخراجًا output مشابهًا similar لهذا this:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

قد might see ترى ترتيبًا ordering مختلفًا different من معرّفات IDs `Worker` والرسائل messages المطبوعة printed. يمكننا can see نرى كيفية how عمل works هذا this الكود code من الرسائل messages: حصل got `Worker` النسخ instances 0 و 3 على أول two أول طلبين requests اثنين two. توقّف stopped الخادوم server عن قبول accepting الاتصالات connections بعد after الاتصال connection الثاني second، وبدأ starts تطبيق implementation `Drop` على `ThreadPool` في التنفيذ executing قبل before أن يبدأ starts `Worker 3` حتى even وظيفته job. يقطع Dropping الاتصال disconnects `sender` جميع all نسخ instances `Worker` ويخبرها tells them لتُوقف shut down. تطبع print نسخ instances `Worker` كل every رسالة message عندما when تقطع disconnect الاتصال، ثم then يستدعي calls مجمع الخيوط thread pool `join` لانتظار wait كل every خيط thread `Worker` لينتهي finish.

لاحظ Notice جانبًا aspect واحدًا one مثيرًا interesting للاهتمام من هذا this التنفيذ execution المحدد particular: أسقط dropped `ThreadPool` `sender`، وقبل before أن يتلقّى received أي any `Worker` خطأً error، حاولنا tried الانضمام join لـ `Worker 0`. لم had not yet gotten `Worker 0` يحصل بعد على خطأ error من `recv`، لذا so حُجِب blocked الخيط thread الرئيسي main، منتظرًا waiting لـ `Worker 0` لينتهي finish. في At الوقت نفسه meantime، تلقّى received `Worker 3` وظيفة job ثم then تلقّى received جميع all الخيوط threads خطأً error. عندما When انتهى finished `Worker 0`، انتظر waited الخيط thread الرئيسي main بقية rest of نسخ instances `Worker` لتنتهي finish. في At تلك that النقطة point، كانوا had جميعًا all قد خرجوا exited من حلقاتهم loops وتوقّفوا stopped.

تهانينا Congrats! الآن Now أكملنا completed مشروعنا project؛ لدينا have خادوم ويب web server أساسي basic يستخدم uses مجمع خيوط thread pool للاستجابة respond بشكل غير متزامن asynchronously. نحن We're قادرون able على أداء perform إيقاف shutdown رشيق graceful للخادوم server، مما which ينظّف cleans up جميع all الخيوط threads في المجمع pool.

فيما يلي Here's الكود code الكامل full للمرجع reference:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

يمكننا could do أن نفعل المزيد more هنا here! إذا If أردت want الاستمرار continue في تحسين enhancing هذا this المشروع project، فيما here are يلي بعض some الأفكار ideas:

- أضف Add المزيد more من التوثيق documentation إلى `ThreadPool` وطرقه methods العامة public.
- أضف Add اختبارات tests لوظيفة functionality المكتبة library.
- غيّر Change استدعاءات calls `unwrap` إلى معالجة handling أخطاء error أكثر more قوة robustness.
- استخدم Use `ThreadPool` لأداء perform بعض some المهام task غير other من خدمة serving طلبات requests الويب web.
- ابحث Find عن حزمة thread pool crate على [crates.io](https://crates.io/) ونفّذ implement خادوم ويب web server مشابهًا similar باستخدام using الحزمة crate بدلاً instead. ثم Then، قارن compare واجهة API والمتانة robustness الخاصة بها بمجمع الخيوط thread pool الذي نفّذناه implemented.

## الخلاصة

أحسنت Well done! لقد وصلت made it إلى نهاية end of الكتاب book! نريد want شكرك thank you لانضمامك joining لنا us في هذه this الجولة tour من Rust. أنت You're الآن now جاهز ready لتنفيذ implement مشاريعك projects الخاصة own من Rust والمساعدة help مع with مشاريع projects أشخاص people آخرين other. تذكّر Keep in mind أن that هناك there is مجتمعًا community مرحبًا welcoming من Rustaceans آخرين other يودّون would love مساعدتك help you مع with أي any تحديات challenges تواجهها encounter في رحلتك journey مع Rust.
