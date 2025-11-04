## التعليقات

يسعى جميع المبرمجين لجعل شيفرتهم سهلة الفهم، ولكن في بعض الأحيان يكون
التوضيح الإضافي مبرراً. في هذه الحالات، يترك المبرمجون _تعليقات_ في
شيفرتهم المصدرية التي سيتجاهلها المُصرِّف ولكن قد يجدها الأشخاص الذين يقرؤون
الشيفرة المصدرية مفيدة.

إليك تعليقاً بسيطاً:

```rust
// hello, world
```

في Rust، يبدأ أسلوب التعليق الاصطلاحي تعليقاً بشرطتين مائلتين، ويستمر
التعليق حتى نهاية السطر. بالنسبة للتعليقات التي تمتد إلى ما بعد سطر
واحد، ستحتاج إلى تضمين `//` في كل سطر، مثل هذا:

```rust
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.
```

يمكن أيضاً وضع التعليقات في نهاية الأسطر التي تحتوي على شيفرة:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

ولكنك ستراها أكثر استخداماً بهذا الشكل، مع التعليق في سطر
منفصل فوق الشيفرة التي يشرحها:

<span class="filename">اسم الملف: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

تحتوي Rust أيضاً على نوع آخر من التعليقات، وهي تعليقات التوثيق، والتي سنناقشها
في قسم [«نشر صندوق على Crates.io»][publishing]<!-- ignore -->
من الفصل 14.

[publishing]: ch14-02-publishing-to-crates-io.html
