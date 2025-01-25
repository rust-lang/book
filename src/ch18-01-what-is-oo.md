## Characteristics of Object-Oriented Languages

Programming community তে object-oriented consider করার জন্য একটি language এ কি কি feature থাকা উচিত সে বিষয়ে কোনো consensus নেই। Rust অনেক programming paradigm দ্বারা influenced, OOP সহ; উদাহরণস্বরূপ, আমরা Chapter 13 এ functional programming থেকে আসা feature গুলো explore করেছিলাম। Arguably, OOP language গুলো কিছু common characteristic share করে, যেমন objects, encapsulation, এবং inheritance। চলুন দেখা যাক সেই characteristic গুলোর প্রত্যেকটির মানে কি এবং Rust সেগুলোকে support করে কিনা।

### Objects Contain Data and Behavior

Erich Gamma, Richard Helm, Ralph Johnson, এবং John Vlissides (Addison-Wesley Professional, 1994) এর লেখা বই _Design Patterns: Elements of Reusable Object-Oriented Software_, colloquially _The Gang of Four_ book নামে পরিচিত, object-oriented design pattern এর একটি catalog। এটি OOP কে এভাবে define করে:

> Object-oriented program গুলো object দিয়ে তৈরি। একটি _object_ data এবং সেই data তে operate করা procedure দুটোই package করে। Procedure গুলোকে সাধারণত _method_ বা _operation_ বলা হয়।

এই definition ব্যবহার করে, Rust object-oriented: struct এবং enum এ data থাকে, এবং `impl` block struct এবং enum এ method provide করে। যদিও struct এবং enum method এর সাথে _object_ হিসেবে _call_ করা হয় না, তবুও তারা object এর Gang of Four এর definition অনুযায়ী same functionality provide করে।

### Encapsulation that Hides Implementation Details

OOP এর সাথে commonly associated আরেকটি aspect হলো _encapsulation_ এর idea, যার মানে হলো object ব্যবহার করা code এর কাছে object এর implementation detail accessible নয়। তাই, একটি object এর সাথে interact করার একমাত্র উপায় হলো এর public API; object ব্যবহার করা code এর internal এ reach করে data বা behaviour সরাসরি change করতে পারার কথা না। এটা programmer কে object ব্যবহার করা code change করার প্রয়োজন ছাড়াই object এর internal change এবং refactor করতে enable করে।

আমরা Chapter 7 এ discuss করেছিলাম কিভাবে encapsulation control করতে হয়: আমরা `pub` keyword ব্যবহার করে decide করতে পারি আমাদের code এর কোন module, type, function, এবং method public হওয়া উচিত, এবং default হিসেবে বাকি সবকিছু private থাকে। উদাহরণস্বরূপ, আমরা `AveragedCollection` নামে একটি struct define করতে পারি যার field এ `i32` value এর একটি vector থাকবে। Struct এ এমন একটি field ও থাকতে পারে যেখানে vector এর value গুলোর average থাকবে, মানে average কারো প্রয়োজন হলেই on demand compute করার প্রয়োজন নেই। অন্যভাবে বলতে গেলে, `AveragedCollection` আমাদের জন্য calculated average cache করে রাখবে। Listing 18-1 এ `AveragedCollection` struct এর definition দেওয়া আছে:

<Listing number="18-1" file-name="src/lib.rs" caption="একটি `AveragedCollection` struct যা integer এর একটি list এবং collection এর item গুলোর average maintain করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-01/src/lib.rs}}
```

</Listing>

Struct টি `pub` mark করা যাতে অন্য code এটি ব্যবহার করতে পারে, কিন্তু struct এর ভিতরের field গুলো private থাকে। এটি এই ক্ষেত্রে গুরুত্বপূর্ণ কারণ আমরা নিশ্চিত করতে চাই যখনই list থেকে কোনো value add বা remove করা হয়, তখনই average ও update হয়। এটা করার জন্য আমরা struct এর উপর `add`, `remove`, এবং `average` method implement করি, যা Listing 18-2 এ দেখানো হয়েছে:

<Listing number="18-2" file-name="src/lib.rs" caption="`AveragedCollection` এর উপর public method `add`, `remove`, এবং `average` এর implementation">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-02/src/lib.rs:here}}
```

</Listing>

`AveragedCollection` এর instance এ data access বা modify করার একমাত্র উপায় হলো public method `add`, `remove`, এবং `average`। যখন `add` method ব্যবহার করে `list` এ কোনো item add করা হয় অথবা `remove` method ব্যবহার করে remove করা হয়, তখন প্রত্যেকটির implementation private `update_average` method call করে যা `average` field ও update করে।

আমরা `list` এবং `average` field গুলো private রাখি যাতে external code `list` field এ সরাসরি item add বা remove করতে না পারে; অন্যথায়, যখন `list` change হয় তখন `average` field out of sync হয়ে যেতে পারে। `average` method `average` field এর value return করে, external code কে `average` read করার সুযোগ দেয় কিন্তু modify করার সুযোগ দেয় না।

যেহেতু আমরা `AveragedCollection` struct এর implementation detail encapsulate করেছি, তাই আমরা ভবিষ্যতে data structure এর মতো aspect গুলো easily change করতে পারি। উদাহরণস্বরূপ, আমরা `list` field এর জন্য `Vec<i32>` এর পরিবর্তে `HashSet<i32>` ব্যবহার করতে পারি। যতক্ষণ `add`, `remove`, এবং `average` public method এর signature same থাকে, ততক্ষণ `AveragedCollection` ব্যবহার করা code compile করার জন্য change করার প্রয়োজন হবে না। যদি আমরা `list` কে public বানাতাম, তাহলে এটা necessarily সঠিক হতো না: `HashSet<i32>` এবং `Vec<i32>` এর item add এবং remove করার জন্য ভিন্ন method আছে, তাই external code কে likely change করতে হতো যদি তারা `list` সরাসরি modify করত।

যদি একটি language কে object-oriented consider করার জন্য encapsulation একটি required aspect হয়, তাহলে Rust সেই requirement meet করে। Code এর বিভিন্ন অংশের জন্য `pub` ব্যবহার করার option implementation detail এর encapsulation enable করে।

### Inheritance as a Type System and as Code Sharing

_Inheritance_ হলো এমন একটি mechanism যার মাধ্যমে একটি object অন্য object এর definition থেকে element inherit করতে পারে, ফলে parent object এর data এবং behaviour আবার define না করেই gain করতে পারে।

যদি একটি object-oriented language হওয়ার জন্য inheritance থাকা প্রয়োজন হয়, তাহলে Rust object-oriented নয়। Macro ব্যবহার করা ছাড়া parent struct এর field এবং method implementation inherit করে এমন struct define করার কোনো উপায় নেই।

তবে, যদি আপনি আপনার programming toolbox এ inheritance রাখতে অভ্যস্ত হন, তাহলে আপনি Rust এ অন্য solution ব্যবহার করতে পারেন, যা মূলত inheritance ব্যবহার করার কারণের উপর depend করে।

আপনি দুটি main reason এর জন্য inheritance choose করতে পারেন। একটি হলো code reuse করার জন্য: আপনি একটি type এর জন্য particular behaviour implement করতে পারেন, এবং inheritance আপনাকে সেই implementation অন্য type এর জন্য reuse করতে enable করে। Rust code এ আপনি default trait method implementation ব্যবহার করে limited way তে এটা করতে পারেন, যা আপনি Listing 10-14 এ দেখেছিলেন যখন আমরা `Summary` trait এ `summarize` method এর একটি default implementation যোগ করেছিলাম। `Summary` trait implement করা যেকোনো type এ আর কোনো code ছাড়াই `summarize` method available থাকতো। এটা অনেকটা parent class এর একটি method এর implementation থাকা এবং inherit করা child class এর ও method এর implementation থাকার মতোই। আমরা `Summary` trait implement করার সময় `summarize` method এর default implementation override ও করতে পারি, যা parent class থেকে inherit করা method এর implementation override করা child class এর মতো।

Inheritance ব্যবহার করার অন্য কারণ হলো type system এর সাথে related: parent type এর মতো একই জায়গায় child type ব্যবহার করতে enable করা। এটাকে _polymorphism_ ও বলা হয়, যার মানে হলো runtime এ multiple object একে অপরের সাথে substitute করতে পারা যদি তারা কিছু particular characteristic share করে।

> ### Polymorphism
>
> অনেকের কাছে, polymorphism হলো inheritance এর synonym। কিন্তু এটা আসলে একটি আরও general concept যা এমন code refer করে যা multiple type এর data এর সাথে কাজ করতে পারে। Inheritance এর জন্য, সেই type গুলো সাধারণত subclass হয়।
>
> এর পরিবর্তে Rust different possible type এর উপর abstract করার জন্য generics এবং সেই type গুলো কি provide করবে তার উপর constraint impose করার জন্য trait bound ব্যবহার করে। এটাকে মাঝে মাঝে _bounded parametric polymorphism_ ও বলা হয়।

Inheritance recent সময়ে অনেক programming language এ programming design solution হিসেবে favor হারাচ্ছে কারণ এটি প্রায়ই প্রয়োজনের চেয়ে বেশি code share করার risk এ থাকে। Subclass এর সবসময় parent class এর সব characteristic share করা উচিত না কিন্তু inheritance এর সাথে সেটা করবে। এর কারণে program design কম flexible হতে পারে। এটি subclass এ এমন method call করার possibility introduce করে যা কোনো sense করে না বা error cause করে কারণ method গুলো subclass এর জন্য apply হয় না। এছাড়াও, কিছু language এ শুধুমাত্র single inheritance allow করবে (মানে একটি subclass শুধুমাত্র একটি class থেকে inherit করতে পারে), যা আরও বেশি করে program design এর flexibility restrict করে।

এই reason গুলোর জন্য, Rust inheritance এর পরিবর্তে trait object ব্যবহার করার ভিন্ন approach নেয়। চলুন দেখা যাক কিভাবে trait object Rust এ polymorphism enable করে।
