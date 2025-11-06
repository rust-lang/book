## التعليقات (Comments)

يسعى جميع المبرمجين لجعل شيفرتهم (code) سهلة الفهم، ولكن في بعض الأحيان يكون
التوضيح الإضافي مبرراً. في هذه الحالات، يترك المبرمجون _تعليقات (comments)_ في
شيفرتهم المصدرية (source code) التي سيتجاهلها المُصرِّف (compiler) ولكن قد يجدها الأشخاص الذين يقرؤون
الشيفرة المصدرية (source code) مفيدة.

إليك تعليقاً (comment) بسيطاً:

```rust
// hello, world
```

في Rust، يبدأ أسلوب التعليق (comment) الاصطلاحي تعليقاً (comment) بشرطتين مائلتين، ويستمر
التعليق (comment) حتى نهاية السطر. بالنسبة للتعليقات (comments) التي تمتد إلى ما بعد سطر
واحد، ستحتاج إلى تضمين `//` في كل سطر، مثل هذا:

```rust
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.
```

يمكن أيضاً وضع التعليقات (comments) في نهاية الأسطر التي تحتوي على شيفرة (code):

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

ولكنك ستراها أكثر استخداماً بهذا الشكل، مع التعليق (comment) في سطر
منفصل فوق الشيفرة (code) التي يشرحها:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

تحتوي Rust أيضاً على نوع آخر من التعليقات (comments)، وهي تعليقات التوثيق (documentation comments)، والتي سنناقشها
في قسم [«نشر صندوق على Crates.io»][publishing]<!-- ignore -->
من الفصل 14.

[publishing]: ch14-02-publishing-to-crates-io.html
