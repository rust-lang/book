## بناء خادوم ويب أحادي الخيط

سنبدأ بتشغيل خادوم ويب أحادي الخيط single-threaded. قبل أن نبدأ، دعونا نلقي نظرة سريعة على البروتوكولات protocols المستخدمة في بناء خوادم الويب. تفاصيل هذه البروتوكولات خارج نطاق هذا الكتاب، ولكن نظرة عامة موجزة ستعطيك المعلومات التي تحتاجها.

البروتوكولان الرئيسيان المستخدمان في خوادم الويب هما _Hypertext Transfer Protocol_ _(HTTP)_ و _Transmission Control Protocol_ _(TCP)_. كلا البروتوكولين هما بروتوكولا _request-response_ طلب-استجابة، مما يعني أن _client_ (عميل) يبدأ الطلبات requests و _server_ (خادوم) يستمع للطلبات ويوفر استجابة response للعميل. محتويات تلك الطلبات والاستجابات محددة بواسطة البروتوكولات.

TCP هو البروتوكول ذو المستوى الأدنى lower-level الذي يصف تفاصيل كيفية انتقال المعلومات من خادوم إلى آخر ولكنه لا يحدد ما هي تلك المعلومات. يبني HTTP على TCP من خلال تحديد محتويات الطلبات والاستجابات. من الممكن تقنيًا استخدام HTTP مع بروتوكولات أخرى، ولكن في الغالبية العظمى من الحالات، يرسل HTTP بياناته عبر TCP. سنعمل مع البايتات الخام raw bytes لطلبات واستجابات TCP و HTTP.

### الاستماع لاتصال TCP

يحتاج خادوم الويب الخاص بنا إلى الاستماع listening لاتصال TCP، لذا فهذا هو الجزء الأول الذي سنعمل عليه. توفر المكتبة القياسية وحدة `std::net` تتيح لنا القيام بذلك. لنصنع مشروعًا جديدًا بالطريقة المعتادة:

```console
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```

الآن أدخل الكود في القائمة 21-1 في _src/main.rs_ للبدء. سيستمع هذا الكود عند العنوان المحلي `127.0.0.1:7878` للتدفقات الواردة incoming TCP streams. عندما يحصل على تدفق وارد، سيطبع `Connection established!`.

<Listing number="21-1" file-name="src/main.rs" caption="الاستماع للتدفقات الواردة وطباعة رسالة عندما نستقبل تدفقًا">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-01/src/main.rs}}
```

</Listing>

باستخدام `TcpListener`، يمكننا الاستماع لاتصالات TCP على العنوان `127.0.0.1:7878`. في العنوان، القسم قبل النقطتين هو عنوان IP يمثل جهاز الكمبيوتر الخاص بك (هذا هو نفسه على كل جهاز كمبيوتر ولا يمثل كمبيوتر المؤلفين بشكل خاص)، و `7878` هو المنفذ port. اخترنا هذا المنفذ لسببين: لا يتم قبول HTTP عادة على هذا المنفذ، لذا فإن خادومنا من غير المحتمل أن يتعارض مع أي خادوم ويب آخر قد يكون لديك قيد التشغيل على جهازك، و 7878 هو _rust_ مكتوبة على الهاتف.

تعمل دالة `bind` في هذا السيناريو مثل دالة `new` من حيث أنها ستُرجع نسخة جديدة من `TcpListener`. تُسمى الدالة `bind` لأنه، في الشبكات، الاتصال بمنفذ port للاستماع يُعرف باسم "binding to a port" (ربط بمنفذ).

تُرجع دالة `bind` نتيجة `Result<T, E>`، مما يشير إلى أنه من الممكن أن يفشل الربط binding، على سبيل المثال، إذا قمنا بتشغيل نسختين من برنامجنا وبالتالي كان لدينا برنامجان يستمعان إلى نفس المنفذ. نظرًا لأننا نكتب خادومًا أساسيًا فقط لأغراض التعلم، فلن نقلق بشأن معالجة هذه الأنواع من الأخطاء؛ بدلاً من ذلك، نستخدم `unwrap` لإيقاف البرنامج إذا حدثت أخطاء.

تُرجع الطريقة method `incoming` على `TcpListener` مكررًا iterator يعطينا تسلسلاً من التدفقات streams (بشكل أكثر تحديدًا، تدفقات من نوع `TcpStream`). _stream_ (تدفق) واحد يمثل اتصالاً مفتوحًا بين العميل والخادوم. _Connection_ (اتصال) هو الاسم لعملية الطلب والاستجابة request and response الكاملة التي يتصل فيها العميل بالخادوم، ويولد الخادوم استجابة، ويغلق الخادوم الاتصال. على هذا النحو، سنقرأ من `TcpStream` لنرى ما أرسله العميل ثم نكتب استجابتنا إلى التدفق stream لإرسال البيانات مرة أخرى إلى العميل. بشكل عام، ستعالج حلقة `for` هذه كل اتصال connection بدوره وتنتج سلسلة من التدفقات streams لنتعامل معها.

في الوقت الحالي، تتكون معالجتنا للتدفق stream من استدعاء `unwrap` لإنهاء برنامجنا إذا كان للتدفق أي أخطاء؛ إذا لم تكن هناك أخطاء، يطبع البرنامج رسالة. سنضيف المزيد من الوظائف functionality لحالة النجاح في القائمة التالية. السبب في أننا قد نتلقى أخطاء من طريقة `incoming` عندما يتصل عميل بالخادوم هو أننا لا نكرر فعليًا على الاتصالات connections. بدلاً من ذلك، نكرر على _محاولات اتصال_ _connection attempts_. قد لا يكون الاتصال ناجحًا لعدة أسباب، الكثير منها خاص بنظام التشغيل. على سبيل المثال، للعديد من أنظمة التشغيل حد لعدد الاتصالات المفتوحة المتزامنة التي يمكنها دعمها؛ ستنتج محاولات الاتصال الجديدة التي تتجاوز هذا العدد خطأ حتى يتم إغلاق بعض الاتصالات المفتوحة.

لنحاول تشغيل هذا الكود! استدعِ `cargo run` في الطرفية terminal ثم قم بتحميل _127.0.0.1:7878_ في متصفح الويب. يجب أن يعرض المتصفح رسالة خطأ مثل "Connection reset" لأن الخادوم لا يرسل حاليًا أي بيانات. ولكن عندما تنظر إلى طرفيتك، يجب أن ترى عدة رسائل تمت طباعتها عندما اتصل المتصفح بالخادوم!

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

في بعض الأحيان سترى رسائل متعددة مطبوعة لطلب متصفح واحد؛ قد يكون السبب هو أن المتصفح يقدم طلبًا للصفحة بالإضافة إلى طلب لموارد أخرى، مثل أيقونة _favicon.ico_ التي تظهر في علامة تبويب المتصفح.

قد يكون أيضًا أن المتصفح يحاول الاتصال بالخادوم عدة مرات لأن الخادوم لا يستجيب بأي بيانات. عندما يخرج `stream` من النطاق scope ويتم إسقاطه dropped في نهاية الحلقة loop، يتم إغلاق الاتصال connection كجزء من تطبيق `drop` implementation. تتعامل المتصفحات أحيانًا مع الاتصالات المغلقة عن طريق إعادة المحاولة retry، لأن المشكلة قد تكون مؤقتة.

تفتح المتصفحات أيضًا في بعض الأحيان اتصالات متعددة بالخادوم دون إرسال أي طلبات بحيث إذا أرسلت طلبات لاحقًا، يمكن أن تحدث هذه الطلبات بشكل أسرع. عندما يحدث هذا، سيرى خادومنا كل اتصال، بغض النظر عما إذا كانت هناك أي طلبات عبر ذلك الاتصال. تقوم العديد من إصدارات المتصفحات المستندة إلى Chrome بذلك، على سبيل المثال؛ يمكنك تعطيل هذا التحسين باستخدام وضع التصفح الخاص أو استخدام متصفح مختلف.

العامل المهم هو أننا نجحنا في الحصول على مقبض handle لاتصال TCP!

تذكر إيقاف البرنامج بالضغط على <kbd>ctrl</kbd>-<kbd>C</kbd> عندما تنتهي من تشغيل إصدار معين من الكود. ثم أعد تشغيل البرنامج عن طريق استدعاء أمر `cargo run` بعد إجراء كل مجموعة من تغييرات الكود للتأكد من أنك تشغل أحدث كود.

### قراءة الطلب

لنطبق الوظيفة functionality لقراءة الطلب request من المتصفح! لفصل المهام concerns المتمثلة في الحصول أولاً على اتصال ثم اتخاذ بعض الإجراءات مع الاتصال، سنبدأ دالة جديدة لمعالجة الاتصالات. في دالة `handle_connection` الجديدة هذه، سنقرأ البيانات من تدفق TCP stream ونطبعها حتى نتمكن من رؤية البيانات المرسلة من المتصفح. غيّر الكود ليبدو مثل القائمة 21-2.

<Listing number="21-2" file-name="src/main.rs" caption="القراءة من `TcpStream` وطباعة البيانات">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-02/src/main.rs}}
```

</Listing>

نجلب `std::io::BufReader` و `std::io::prelude` إلى النطاق scope للوصول إلى السمات traits والأنواع types التي تتيح لنا القراءة من والكتابة إلى التدفق stream. في حلقة `for` في دالة `main`، بدلاً من طباعة رسالة تقول إننا أنشأنا اتصالاً، نستدعي الآن دالة `handle_connection` الجديدة ونمرر `stream` إليها.

في دالة `handle_connection`، نُنشئ نسخة جديدة من `BufReader` تلتف حول مرجع reference إلى `stream`. يضيف `BufReader` التخزين المؤقت buffering عن طريق إدارة الاستدعاءات لطرق سمة `std::io::Read` لنا.

نُنشئ متغيرًا باسم `http_request` لجمع أسطر الطلب request التي يرسلها المتصفح إلى خادومنا. نشير إلى أننا نريد جمع هذه الأسطر في متجه vector عن طريق إضافة توضيح النوع type annotation `Vec<_>`.

يطبق `BufReader` سمة `std::io::BufRead` trait، والتي توفر طريقة method `lines`. تُرجع طريقة `lines` مكررًا iterator لـ `Result<String, std::io::Error>` عن طريق تقسيم تدفق stream البيانات كلما رأى بايت سطر جديد newline byte. للحصول على كل `String`، نستخدم `map` و `unwrap` لكل `Result`. قد يكون `Result` خطأً إذا لم تكن البيانات UTF-8 صالحة أو إذا كانت هناك مشكلة في القراءة من التدفق stream. مرة أخرى، يجب على البرنامج الإنتاجي production program معالجة هذه الأخطاء بشكل أكثر رشاقة gracefully، لكننا نختار إيقاف البرنامج في حالة الخطأ من أجل البساطة.

يشير المتصفح إلى نهاية طلب HTTP عن طريق إرسال حرفي سطر جديد newline characters متتاليين، لذلك للحصول على طلب واحد من التدفق stream، نأخذ الأسطر حتى نحصل على سطر فارغ. بمجرد جمع الأسطر في المتجه vector، نطبعها باستخدام تنسيق التصحيح الجميل pretty debug formatting حتى نتمكن من إلقاء نظرة على التعليمات التي يرسلها متصفح الويب إلى خادومنا.

لنجرب هذا الكود! ابدأ البرنامج واطلب request في متصفح الويب مرة أخرى. لاحظ أننا سنظل نحصل على صفحة خطأ في المتصفح، لكن إخراج output برنامجنا في الطرفية terminal سيبدو الآن مشابهًا لهذا:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-02
cargo run
make a request to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

اعتمادًا على متصفحك، قد تحصل على إخراج output مختلف قليلاً. الآن بعد أن نطبع بيانات الطلب request data، يمكننا معرفة سبب حصولنا على اتصالات متعددة من طلب متصفح واحد من خلال النظر إلى المسار path بعد `GET` في السطر الأول من الطلب. إذا كانت الاتصالات المتكررة تطلب جميعها _/_، فنحن نعلم أن المتصفح يحاول جلب _/_ بشكل متكرر لأنه لا يحصل على استجابة response من برنامجنا.

لنحلل بيانات الطلب request data هذه لفهم ما يطلبه المتصفح من برنامجنا.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-closer-look-at-an-http-request"></a>
<a id="looking-closer-at-an-http-request"></a>

### نظرة أقرب على طلب HTTP

HTTP هو بروتوكول نصي text-based protocol، والطلب request يأخذ هذا التنسيق format:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

السطر الأول هو _request line_ (سطر الطلب) الذي يحمل معلومات حول ما يطلبه العميل. يشير الجزء الأول من سطر الطلب request line إلى الطريقة method المستخدمة، مثل `GET` أو `POST`، والتي تصف كيف يقدم العميل هذا الطلب request. استخدم عميلنا طلب `GET` request، مما يعني أنه يطلب معلومات.

الجزء التالي من سطر الطلب request line هو _/_، والذي يشير إلى _uniform resource identifier_ _(URI)_ (معرّف الموارد الموحد) الذي يطلبه العميل: URI يكاد يكون، ولكن ليس تمامًا، مثل _uniform resource locator_ _(URL)_ (محدد موقع الموارد الموحد). الفرق بين URIs و URLs ليس مهمًا لأغراضنا في هذا الفصل، ولكن مواصفات HTTP spec تستخدم مصطلح _URI_، لذا يمكننا فقط استبدال _URL_ ذهنيًا بـ _URI_ هنا.

الجزء الأخير هو إصدار HTTP version الذي يستخدمه العميل، ثم ينتهي سطر الطلب request line بتسلسل CRLF sequence. (_CRLF_ تعني _carriage return_ و _line feed_، وهي مصطلحات من أيام الآلة الكاتبة!) يمكن أيضًا كتابة تسلسل CRLF على شكل `\r\n`، حيث `\r` هو carriage return و `\n` هو line feed. يفصل _CRLF sequence_ (تسلسل CRLF) سطر الطلب request line عن بقية بيانات الطلب request data. لاحظ أنه عندما يتم طباعة CRLF، نرى بداية سطر جديد بدلاً من `\r\n`.

بالنظر إلى بيانات سطر الطلب request line data التي تلقيناها من تشغيل برنامجنا حتى الآن، نرى أن `GET` هو الطريقة method، _/_ هو URI الطلب request، و `HTTP/1.1` هو الإصدار version.

بعد سطر الطلب request line، الأسطر المتبقية بدءًا من `Host:` فصاعدًا هي الرؤوس headers. طلبات `GET` requests ليس لها جسم body.

حاول تقديم طلب request من متصفح مختلف أو طلب عنوان مختلف، مثل _127.0.0.1:7878/test_، لترى كيف تتغير بيانات الطلب request data.

الآن بعد أن عرفنا ما يطلبه المتصفح، لنرسل بعض البيانات!

### كتابة استجابة

سنطبق إرسال البيانات في استجابة response لطلب العميل client request. الاستجابات responses لها التنسيق التالي:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

السطر الأول هو _status line_ (سطر الحالة) الذي يحتوي على إصدار HTTP المستخدم في الاستجابة response، ورمز حالة رقمي numeric status code يلخص نتيجة الطلب request، وعبارة سبب reason phrase توفر وصفًا نصيًا لرمز الحالة status code. بعد تسلسل CRLF توجد أي رؤوس headers، وتسلسل CRLF آخر، وجسم body الاستجابة response.

فيما يلي مثال على استجابة response تستخدم إصدار HTTP 1.1 ولها رمز حالة status code 200، وعبارة سبب reason phrase OK، وبدون رؤوس headers، وبدون جسم body:

```text
HTTP/1.1 200 OK\r\n\r\n
```

رمز الحالة status code 200 هو استجابة النجاح القياسية. النص هو استجابة HTTP ناجحة صغيرة. لنكتب هذا إلى التدفق stream كاستجابتنا response لطلب request ناجح! من دالة `handle_connection`، احذف `println!` التي كانت تطبع بيانات الطلب request data واستبدلها بالكود في القائمة 21-3.

<Listing number="21-3" file-name="src/main.rs" caption="كتابة استجابة HTTP ناجحة صغيرة إلى التدفق">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-03/src/main.rs:here}}
```

</Listing>

يحدد السطر الجديد الأول متغير `response` الذي يحمل بيانات رسالة النجاح. ثم نستدعي `as_bytes` على `response` لتحويل بيانات السلسلة string data إلى بايتات bytes. تأخذ طريقة method `write_all` على `stream` مرجعًا `&[u8]` وترسل تلك البايتات مباشرة عبر الاتصال connection. نظرًا لأن عملية `write_all` يمكن أن تفشل، نستخدم `unwrap` على أي نتيجة خطأ error result كما كان من قبل. مرة أخرى، في تطبيق حقيقي real application، ستضيف معالجة الأخطاء error handling هنا.

مع هذه التغييرات، لنشغل كودنا ونقدم طلبًا request. لم نعد نطبع أي بيانات إلى الطرفية terminal، لذلك لن نرى أي إخراج output غير الإخراج من Cargo. عندما تحمّل _127.0.0.1:7878_ في متصفح الويب، يجب أن تحصل على صفحة فارغة بدلاً من خطأ. لقد قمت للتو بكتابة يدوية handcoded لاستقبال طلب HTTP وإرسال استجابة response!

### إرجاع HTML حقيقي

لننفذ الوظيفة functionality لإرجاع أكثر من صفحة فارغة. أنشئ الملف الجديد _hello.html_ في جذر دليل مشروعك root of your project directory، وليس في دليل _src_. يمكنك إدخال أي HTML تريده؛ تعرض القائمة 21-4 إمكانية واحدة.

<Listing number="21-4" file-name="hello.html" caption="ملف HTML نموذجي لإرجاعه في استجابة">

```html
{{#include ../listings/ch21-web-server/listing-21-05/hello.html}}
```

</Listing>

هذه وثيقة HTML5 بسيطة بعنوان heading وبعض النص. لإرجاع هذا من الخادوم server عند استقبال طلب request، سنعدل `handle_connection` كما هو موضح في القائمة 21-5 لقراءة ملف HTML، وإضافته إلى الاستجابة response كجسم body، وإرساله.

<Listing number="21-5" file-name="src/main.rs" caption="إرسال محتويات *hello.html* كجسم للاستجابة">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-05/src/main.rs:here}}
```

</Listing>

أضفنا `fs` إلى عبارة `use` statement لجلب وحدة module نظام الملفات filesystem الخاصة بالمكتبة القياسية إلى النطاق scope. يجب أن يبدو الكود لقراءة محتويات الملف إلى سلسلة string مألوفًا؛ استخدمناه عندما قرأنا محتويات ملف لمشروع I/O الخاص بنا في القائمة 12-4.

بعد ذلك، نستخدم `format!` لإضافة محتويات الملف كجسم body لاستجابة النجاح success response. لضمان استجابة HTTP صالحة valid، نضيف رأس `Content-Length` header، والذي يتم تعيينه على حجم جسم body استجابتنا response—في هذه الحالة، حجم `hello.html`.

شغّل هذا الكود مع `cargo run` وحمّل _127.0.0.1:7878_ في متصفحك؛ يجب أن ترى HTML الخاص بك معروضًا!

حاليًا، نحن نتجاهل بيانات الطلب request data في `http_request` ونرسل فقط محتويات ملف HTML بشكل غير مشروط unconditionally. وهذا يعني أنه إذا حاولت طلب requesting _127.0.0.1:7878/something-else_ في متصفحك، فستظل تحصل على نفس استجابة HTML response هذه. في الوقت الحالي، خادومنا server محدود جدًا ولا يقوم بما تفعله معظم خوادم الويب web servers. نريد تخصيص استجاباتنا responses حسب الطلب request وإرسال ملف HTML فقط لطلب صحيح well-formed request إلى _/_.

### التحقق من الطلب والاستجابة بشكل انتقائي

الآن، سيُرجع خادوم الويب web server الخاص بنا HTML في الملف بغض النظر عما طلبه العميل client. لنضف الوظيفة functionality للتحقق من أن المتصفح يطلب requesting _/_ قبل إرجاع ملف HTML وإرجاع خطأ error إذا طلب requesting المتصفح أي شيء آخر. لهذا نحتاج إلى تعديل `handle_connection`، كما هو موضح في القائمة 21-6. يتحقق هذا الكود الجديد من محتوى الطلب المستلم request received مقابل ما نعرفه عن شكل طلب request لـ _/_ ويضيف كتل `if` و `else` blocks لمعاملة الطلبات requests بشكل مختلف.

<Listing number="21-6" file-name="src/main.rs" caption="معالجة الطلبات إلى */* بشكل مختلف عن الطلبات الأخرى">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-06/src/main.rs:here}}
```

</Listing>

سننظر فقط إلى السطر الأول من طلب HTTP request، لذا بدلاً من قراءة الطلب request بالكامل في متجه vector، نستدعي `next` للحصول على العنصر الأول من المكرر iterator. يعتني أول `unwrap` بـ `Option` ويوقف البرنامج إذا لم يكن للمكرر iterator أي عناصر items. يتعامل `unwrap` الثاني مع `Result` وله نفس التأثير مثل `unwrap` الذي كان في `map` المضاف في القائمة 21-2.

بعد ذلك، نتحقق من `request_line` لنرى ما إذا كان يساوي سطر الطلب request line لطلب GET request إلى مسار path _/_. إذا كان الأمر كذلك، تُرجع كتلة `if` block محتويات ملف HTML الخاص بنا.

إذا لم يساوِ `request_line` طلب GET request إلى مسار path _/_، فهذا يعني أننا تلقينا طلبًا request آخر. سنضيف كودًا إلى كتلة `else` block في لحظة للاستجابة respond لجميع الطلبات requests الأخرى.

شغّل هذا الكود الآن واطلب request _127.0.0.1:7878_؛ يجب أن تحصل على HTML في _hello.html_. إذا قدمت أي طلب request آخر، مثل _127.0.0.1:7878/something-else_، فستحصل على خطأ اتصال connection error مثل تلك التي رأيتها عند تشغيل الكود في القائمة 21-1 والقائمة 21-2.

الآن لنضف الكود في القائمة 21-7 إلى كتلة `else` block لإرجاع استجابة response برمز حالة status code 404، الذي يشير إلى عدم العثور على المحتوى content للطلب request. سنُرجع أيضًا بعض HTML لصفحة لعرضها في المتصفح لتشير إلى الاستجابة response للمستخدم النهائي end user.

<Listing number="21-7" file-name="src/main.rs" caption="الاستجابة برمز الحالة 404 وصفحة خطأ إذا تم طلب أي شيء غير */*">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-07/src/main.rs:here}}
```

</Listing>

هنا، استجابتنا response لها سطر حالة status line برمز حالة status code 404 وعبارة السبب reason phrase `NOT FOUND`. سيكون جسم body الاستجابة response هو HTML في الملف _404.html_. ستحتاج إلى إنشاء ملف _404.html_ بجوار _hello.html_ لصفحة الخطأ error page؛ مرة أخرى، لا تتردد في استخدام أي HTML تريده، أو استخدم مثال HTML في القائمة 21-8.

<Listing number="21-8" file-name="404.html" caption="محتوى نموذجي للصفحة لإرسالها مع أي استجابة 404">

```html
{{#include ../listings/ch21-web-server/listing-21-07/404.html}}
```

</Listing>

مع هذه التغييرات، شغّل خادومك server مرة أخرى. طلب requesting _127.0.0.1:7878_ يجب أن يُرجع محتويات _hello.html_، وأي طلب request آخر، مثل _127.0.0.1:7878/foo_، يجب أن يُرجع خطأ error HTML من _404.html_.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-touch-of-refactoring"></a>

### إعادة الهيكلة

في الوقت الحالي، كتل `if` و `else` blocks لديها الكثير من التكرار repetition: كلاهما يقرأ الملفات files ويكتب محتويات الملفات إلى التدفق stream. الاختلافات الوحيدة هي سطر الحالة status line واسم الملف filename. لنجعل الكود أكثر إيجازًا concise عن طريق استخراج هذه الاختلافات في أسطر `if` و `else` منفصلة ستعيّن assign قيم سطر الحالة status line واسم الملف filename إلى المتغيرات variables؛ يمكننا بعد ذلك استخدام تلك المتغيرات variables بشكل غير مشروط unconditionally في الكود لقراءة الملف file وكتابة الاستجابة response. تُظهر القائمة 21-9 الكود الناتج بعد استبدال كتل `if` و `else` blocks الكبيرة.

<Listing number="21-9" file-name="src/main.rs" caption="إعادة هيكلة كتل `if` و `else` لتحتوي فقط على الكود الذي يختلف بين الحالتين">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-09/src/main.rs:here}}
```

</Listing>

الآن تُرجع كتل `if` و `else` blocks فقط القيم المناسبة لسطر الحالة status line واسم الملف filename في صفة tuple؛ ثم نستخدم destructuring لتعيين assign هاتين القيمتين إلى `status_line` و `filename` باستخدام نمط pattern في عبارة `let` statement، كما تمت مناقشته في الفصل 19.

الكود المكرر previously duplicated سابقًا الآن خارج كتل `if` و `else` blocks ويستخدم متغيرات variables `status_line` و `filename`. هذا يجعل من الأسهل رؤية الفرق بين الحالتين two cases، ويعني أن لدينا مكانًا واحدًا فقط لتحديث update الكود إذا أردنا تغيير كيفية عمل قراءة الملف file reading وكتابة الاستجابة response writing. سيكون سلوك behavior الكود في القائمة 21-9 هو نفسه كما في القائمة 21-7.

رائع! الآن لدينا خادوم ويب web server بسيط في حوالي 40 سطرًا من كود Rust يستجيب respond لطلب request واحد بصفحة محتوى content ويستجيب respond لجميع الطلبات requests الأخرى باستجابة response 404.

حاليًا، يعمل خادومنا server في خيط واحد single thread، مما يعني أنه يمكنه خدمة طلب request واحد فقط في كل مرة. لنفحص كيف يمكن أن تكون هذه مشكلة عن طريق محاكاة simulating بعض الطلبات البطيئة slow requests. ثم سنصلحها حتى يتمكن خادومنا server من معالجة طلبات متعددة multiple requests في وقت واحد at once.
