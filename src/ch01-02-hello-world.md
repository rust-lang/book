## !Hello, World

الآن بعد أن قمت بتثبيت (install) Rust، حان الوقت لكتابة برنامج (program) Rust الأول الخاص بك. من التقاليد عند تعلم لغة جديدة أن تكتب برنامجاً صغيراً يطبع النص (text) `!Hello, world` على الشاشة، لذلك سنفعل الشيء نفسه هنا!

> ملاحظة: يفترض هذا الكتاب معرفة أساسية بسطر الأوامر (command line). لا يفرض Rust أي متطلبات محددة حول التحرير (editing) أو الأدوات (tools) أو مكان تواجد الكود الخاص بك، لذلك إذا كنت تفضل استخدام IDE بدلاً من سطر الأوامر، فلا تتردد في استخدام IDE المفضل لديك. تمتلك العديد من IDEs الآن درجة معينة من دعم Rust؛ تحقق من وثائق IDE للحصول على التفاصيل. كان فريق Rust يركز على تمكين دعم رائع لـ IDE عبر `rust-analyzer`. راجع [الملحق D][devtools]<!-- ignore --> للحصول على مزيد من التفاصيل.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-project-directory"></a>

### إعداد دليل المشروع

ستبدأ بإنشاء دليل (directory) لتخزين كود Rust الخاص بك. لا يهم Rust مكان تواجد الكود الخاص بك، ولكن بالنسبة للتمارين (exercises) والمشاريع (projects) في هذا الكتاب، نقترح إنشاء دليل _projects_ في دليل home الخاص بك والاحتفاظ بجميع مشاريعك هناك.

افتح طرفية وأدخل الأوامر التالية لإنشاء دليل _projects_ ودليل لمشروع "!Hello, world" داخل دليل _projects_.

بالنسبة لـ Linux و macOS و PowerShell على Windows، أدخل هذا:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

بالنسبة لـ Windows CMD، أدخل هذا:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

<!-- Old headings. Do not remove or links may break. -->

<a id="writing-and-running-a-rust-program"></a>

### أساسيات برنامج Rust

بعد ذلك، أنشئ ملف مصدر (source file) جديد وسمّه _main.rs_. تنتهي ملفات Rust دائماً بامتداد (extension) _.rs_. إذا كنت تستخدم أكثر من كلمة واحدة في اسم الملف، فإن الاتفاقية (convention) هي استخدام شرطة سفلية (underscore) لفصلهم. على سبيل المثال، استخدم _hello_world.rs_ بدلاً من _helloworld.rs_.

الآن افتح ملف _main.rs_ الذي أنشأته للتو وأدخل الكود في القائمة 1-1.

<Listing number="1-1" file-name="main.rs" caption="برنامج يطبع `!Hello, world`">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

احفظ الملف وارجع إلى نافذة الطرفية في دليل _~/projects/hello_world_. على Linux أو macOS، أدخل الأوامر التالية لتجميع الملف وتشغيله:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

على Windows، أدخل الأمر `.\main` بدلاً من `./main`:

```powershell
> rustc main.rs
> .\main
Hello, world!
```

بغض النظر عن نظام التشغيل الخاص بك، يجب أن يُطبع النص `!Hello, world` إلى الطرفية. إذا لم ترَ هذا الناتج، ارجع إلى جزء ["استكشاف الأخطاء وإصلاحها"][troubleshooting]<!-- ignore --> من قسم التثبيت للحصول على طرق للحصول على المساعدة.

إذا تم طباعة `!Hello, world`، تهانينا! لقد كتبت رسمياً برنامج Rust. هذا يجعلك مبرمج Rust - مرحباً بك!

<!-- Old headings. Do not remove or links may break. -->

<a id="anatomy-of-a-rust-program"></a>

### تشريح برنامج Rust

لنراجع برنامج "!Hello, world" هذا بالتفصيل. إليك الجزء الأول من اللغز:

```rust
fn main() {

}
```

تعرّف هذه الأسطر دالة (function) تُسمى `main`. دالة (function) `main` خاصة: إنها دائماً أول كود يتم تشغيله في كل برنامج Rust قابل للتنفيذ (executable). هنا، يعلن السطر الأول دالة (function) تُسمى `main` لا تحتوي على معاملات (parameters) ولا تُرجع (return) شيئاً. إذا كانت هناك معاملات (parameters)، فستكون داخل الأقواس (parentheses) (`()`).

يتم تغليف جسم الدالة (function body) في `{}`. يتطلب Rust أقواس معقوفة (curly brackets) حول جميع أجسام الدوال (function bodies). من الأسلوب الجيد (good style) وضع القوس المعقوف الافتتاحي (opening curly bracket) على نفس سطر إعلان الدالة (function declaration)، مع إضافة مسافة واحدة بينهما.

> ملاحظة: إذا كنت ترغب في الالتزام بأسلوب قياسي (standard style) عبر مشاريع Rust، يمكنك استخدام أداة تنسيق تلقائية (automatic formatter) تُسمى `rustfmt` لتنسيق الكود الخاص بك بأسلوب معين (مزيد من المعلومات حول `rustfmt` في [الملحق D][devtools]<!-- ignore -->). قام فريق Rust بتضمين هذه الأداة مع توزيع Rust القياسي (standard distribution)، كما هو الحال مع `rustc`، لذلك يجب أن تكون مثبتة بالفعل على جهاز الكمبيوتر الخاص بك!

يحتوي جسم دالة `main` على الكود التالي:

```rust
println!("Hello, world!");
```

يقوم هذا السطر بكل العمل في هذا البرنامج الصغير: إنه يطبع النص (text) إلى الشاشة. هناك ثلاثة تفاصيل مهمة يجب ملاحظتها هنا.

أولاً، `println!` يستدعي ماكرو (macro) Rust. إذا كان قد استدعى دالة (function) بدلاً من ذلك، لكان سيُكتب كـ `println` (بدون `!`). ماكروهات (macros) Rust هي طريقة لكتابة كود يولّد كوداً (code that generates code) لتوسيع بنية Rust، وسنناقشها بمزيد من التفصيل في [الفصل 20][ch20-macros]<!-- ignore -->. في الوقت الحالي، تحتاج فقط إلى معرفة أن استخدام `!` يعني أنك تستدعي ماكرو (macro) بدلاً من دالة عادية (normal function) وأن الماكرو (macros) لا تتبع دائماً نفس القواعد (rules) مثل الدوال (functions).

ثانياً، ترى النص (string) `"!Hello, world"`. نمرر هذا النص كحجة (argument) إلى `println!`، ويتم طباعة النص إلى الشاشة.

ثالثاً، ننهي السطر بفاصلة منقوطة (semicolon) (`;`)، والتي تشير إلى أن هذا التعبير (expression) قد انتهى، والتالي جاهز للبدء. تنتهي معظم أسطر كود Rust بفاصلة منقوطة (semicolon).

<!-- Old headings. Do not remove or links may break. -->

<a id="compiling-and-running-are-separate-steps"></a>

### الترجمة والتنفيذ

لقد قمت للتو بتشغيل برنامج تم إنشاؤه حديثاً، لذلك دعنا نفحص كل خطوة في العملية.

قبل تشغيل برنامج Rust، يجب عليك تجميعه (compile) باستخدام مصرِّف (compiler) Rust عن طريق إدخال أمر (command) `rustc` وتمرير اسم ملف المصدر (source file) الخاص بك، مثل هذا:

```console
$ rustc main.rs
```

إذا كانت لديك خلفية في C أو C++، ستلاحظ أن هذا يشبه `gcc` أو `clang`. بعد الترجمة (compilation) بنجاح، يخرج Rust ملفاً تنفيذياً ثنائياً (binary executable).

على Linux و macOS و PowerShell على Windows، يمكنك رؤية الملف التنفيذي عن طريق إدخال أمر `ls` في shell الخاص بك:

```console
$ ls
main  main.rs
```

على Linux و macOS، سترى ملفين. مع PowerShell على Windows، سترى نفس الملفات الثلاثة التي ستراها باستخدام CMD. مع CMD على Windows، ستدخل ما يلي:

```cmd
> dir /B %= خيار /B يعني إظهار أسماء الملفات فقط =%
main.exe
main.pdb
main.rs
```

يُظهر هذا ملف الكود المصدري (source code file) بامتداد (extension) _.rs_، والملف التنفيذي (executable file) (_main.exe_ على Windows، ولكن _main_ على جميع الأنظمة الأساسية الأخرى)، وعند استخدام Windows، ملف يحتوي على معلومات التصحيح (debugging information) بامتداد _.pdb_. من هنا، تقوم بتشغيل ملف _main_ أو _main.exe_، مثل هذا:

```console
$ ./main # أو .\main على Windows
```

إذا كان _main.rs_ الخاص بك هو برنامج "!Hello, world"، فإن هذا السطر يطبع `!Hello, world` إلى طرفيتك.

إذا كنت أكثر إلماماً بلغة ديناميكية (dynamic language)، مثل Ruby أو Python أو JavaScript، فقد لا تكون معتاداً على ترجمة (compiling) وتشغيل (running) برنامج كخطوات منفصلة (separate steps). Rust هي لغة _مجمّعة مسبقاً (ahead-of-time compiled language)_، مما يعني أنه يمكنك ترجمة (compile) برنامج وإعطاء الملف التنفيذي (executable) لشخص آخر، ويمكنه تشغيله حتى بدون تثبيت (installing) Rust. إذا أعطيت شخصاً ملف _.rb_ أو _.py_ أو _.js_، فإنهم يحتاجون إلى تثبيت تنفيذ (implementation) Ruby أو Python أو JavaScript (على التوالي). ولكن في تلك اللغات، تحتاج فقط إلى أمر واحد لتجميع وتشغيل برنامجك. كل شيء عبارة عن مقايضة (trade-off) في تصميم اللغة (language design).

الترجمة فقط باستخدام `rustc` جيد للبرامج البسيطة، ولكن مع نمو مشروعك، ستحتاج إلى إدارة جميع الخيارات وتسهيل مشاركة الكود الخاص بك. بعد ذلك، سنقدم لك أداة Cargo، والتي ستساعدك في كتابة برامج Rust الواقعية.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html
