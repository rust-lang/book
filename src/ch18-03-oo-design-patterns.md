## Implementing an Object-Oriented Design Pattern

_State pattern_ হলো একটি object-oriented design pattern। Pattern টির মূল বিষয় হলো আমরা এমন set of state define করি যা একটি value internally hold করতে পারে। State গুলো set of _state object_ দ্বারা represent করা হয়, এবং value এর behaviour এর state এর উপর depend করে change হয়। আমরা একটি blog post struct এর উদাহরণ নিয়ে কাজ করতে যাচ্ছি যেখানে এর state hold করার জন্য একটি field থাকবে, যা "draft", "review", বা "published" set থেকে একটি state object হবে।

State object গুলো functionality share করে: Rust এ, অবশ্যই আমরা object এবং inheritance এর পরিবর্তে struct এবং trait ব্যবহার করি। প্রত্যেক state object তার নিজের behaviour এর জন্য responsible এবং কখন এটি অন্য state এ change হওয়া উচিত তা govern করার জন্য responsible। State object hold করা value state গুলোর different behaviour বা কখন state গুলোর মধ্যে transition করতে হবে সে বিষয়ে কিছুই জানে না।

State pattern ব্যবহার করার advantage হলো, যখন program এর business requirement change হয়, তখন state hold করা value এর code change করার প্রয়োজন হবে না বা সেই value ব্যবহার করা code change করার প্রয়োজন হবে না। Rules change করার জন্য বা হয়তো আরও state object add করার জন্য আমাদের শুধুমাত্র state object গুলোর ভিতরের code update করার প্রয়োজন হবে।

প্রথমে, আমরা state pattern implement করতে যাচ্ছি more traditional object-oriented উপায়ে, তারপর আমরা এমন একটি approach ব্যবহার করব যা Rust এ আরও natural। চলুন state pattern ব্যবহার করে incrementally blog post workflow implement করা শুরু করি।

Final functionality টি দেখতে এমন হবে:

1. একটি blog post empty draft হিসেবে শুরু হবে।
2. যখন draft শেষ হয়ে যাবে, তখন post এর review request করা হবে।
3. যখন post approve করা হবে, তখন এটি publish হবে।
4. শুধুমাত্র published blog post print করার জন্য content return করবে, তাই unapproved post accidentally publish হওয়া উচিত নয়।

Post এ attempt করা অন্য change এর কোনো effect থাকা উচিত নয়। উদাহরণস্বরূপ, review request করার আগে যদি আমরা কোনো draft blog post approve করার চেষ্টা করি, তাহলে post টি unpublished draft হিসেবে থেকে যাওয়া উচিত।

Listing 18-11 এই workflow কে code form এ দেখায়: এটি `blog` নামের একটি library crate এ আমরা যে API implement করব তার একটি উদাহরণ। এটি এখনও compile হবে না কারণ আমরা এখনও `blog` crate implement করিনি।

<Listing number="18-11" file-name="src/main.rs" caption="Code যা `blog` crate এ আমরা যে desired behaviour চাই তা demonstrate করে">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

</Listing>

আমরা user কে `Post::new` দিয়ে নতুন draft blog post তৈরি করার allow করতে চাই। আমরা blog post এ text add করার allow করতে চাই। যদি আমরা approval এর আগে, post এর content immediately পাওয়ার চেষ্টা করি, তাহলে আমাদের কোনো text পাওয়া উচিত না কারণ post টি এখনও draft। আমরা demonstration purposes এর জন্য code এ `assert_eq!` add করেছি। এটার জন্য একটি excellent unit test হবে এটা assert করা যে একটি draft blog post `content` method থেকে একটি empty string return করে, কিন্তু আমরা এই উদাহরণের জন্য test লিখব না।

এরপর, আমরা post এর review request enable করতে চাই, এবং review এর জন্য wait করার সময় `content` কে একটি empty string return করা উচিত। যখন post approval পায়, তখন এটি publish হওয়া উচিত, মানে যখন `content` call করা হয় তখন post এর text return হবে।

লক্ষ্য করুন যে crate থেকে আমরা interact করছি এমন একমাত্র type হলো `Post` type। এই type টি state pattern ব্যবহার করবে এবং একটি value hold করবে যা তিনটি state object এর মধ্যে একটি হবে যা represent করে যে post টি draft, review এর জন্য waiting, বা published এর মতো বিভিন্ন state এ থাকতে পারে। একটি state থেকে অন্য state এ change করা `Post` type এর ভিতরে internally manage করা হবে। State গুলো change হয় আমাদের library এর user দ্বারা `Post` instance এর উপর call করা method এর response এ, কিন্তু তাদের state change directly manage করার প্রয়োজন নেই। এছাড়াও, user রা state নিয়ে ভুল করতে পারবে না, যেমন review করার আগে post publish করা।

### Defining `Post` and Creating a New Instance in the Draft State

চলুন library এর implementation শুরু করি! আমরা জানি আমাদের কিছু content hold করে এমন একটি public `Post` struct এর প্রয়োজন, তাই আমরা struct এর definition এবং `Post` এর একটি instance তৈরি করার জন্য associated public `new` function দিয়ে শুরু করব, যা Listing 18-12 এ দেখানো হয়েছে। আমরা একটি private `State` trait ও তৈরি করব যা এমন behaviour define করবে যা `Post` এর সব state object এর থাকা উচিত।

তারপর `Post` একটি `Option<T>` এর ভিতরে `Box<dyn State>` এর trait object hold করবে যা `state` নামের একটি private field এ state object hold করার জন্য। আপনি একটু পরেই দেখবেন কেন `Option<T>` প্রয়োজনীয়।

<Listing number="18-12" file-name="src/lib.rs" caption="`Post` struct এর definition এবং একটি `new` function যা একটি নতুন `Post` instance, একটি `State` trait, এবং একটি `Draft` struct তৈরি করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

</Listing>

`State` trait different post state দ্বারা shared behavior define করে। State object গুলো হলো `Draft`, `PendingReview`, এবং `Published`, এবং তারা সবাই `State` trait implement করবে। আপাতত, trait এ কোনো method নেই, এবং আমরা শুধু `Draft` state define করে শুরু করব কারণ আমরা চাই post টি এই state এ শুরু হোক।

যখন আমরা একটি নতুন `Post` তৈরি করি, তখন আমরা এর `state` field একটি `Some` value set করি যা একটি `Box` hold করে। এই `Box` `Draft` struct এর একটি নতুন instance point করে। এটা নিশ্চিত করে যখনই আমরা `Post` এর নতুন instance তৈরি করি, তখনই এটি draft হিসেবে শুরু হবে। যেহেতু `Post` এর `state` field private, তাই অন্য কোনো state এ `Post` তৈরি করার কোনো উপায় নেই! `Post::new` function এ, আমরা `content` field কে একটি নতুন, empty `String` set করি।

### Storing the Text of the Post Content

আমরা Listing 18-11 এ দেখেছিলাম যে আমরা `add_text` নামের একটি method call করতে চাই এবং এটিকে একটি `&str` pass করতে চাই যা তারপর blog post এর text content হিসেবে add হবে। আমরা `content` field কে `pub` হিসেবে expose করার পরিবর্তে এটিকে method হিসেবে implement করি, যাতে পরে আমরা এমন একটি method implement করতে পারি যা control করবে কিভাবে `content` field এর data read করা হবে। `add_text` method টি বেশ straightforward, তাই চলুন Listing 18-13 এ `impl Post` block এ implementation add করি:

<Listing number="18-13" file-name="src/lib.rs" caption="Post এর `content` এ text add করার জন্য `add_text` method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

</Listing>

`add_text` method `self` এ একটি mutable reference নেয়, কারণ আমরা সেই `Post` instance change করছি যার উপর আমরা `add_text` call করছি। তারপর আমরা `content` এ `String` এর উপর `push_str` call করি এবং saved `content` এ add করার জন্য `text` argument pass করি। এই behaviour post কোন state এ আছে তার উপর depend করে না, তাই এটা state pattern এর অংশ নয়। `add_text` method `state` field এর সাথে interact করে না, কিন্তু এটা সেই behaviour এর অংশ যা আমরা support করতে চাই।

### Ensuring the Content of a Draft Post Is Empty

এমনকি `add_text` call করে post এ কিছু content add করার পরেও, আমরা এখনও চাই যে `content` method একটি empty string slice return করুক কারণ post টি এখনও draft state এ আছে, যেমন Listing 18-11 এর 7 নম্বর লাইনে দেখানো হয়েছে। আপাতত, চলুন `content` method implement করি simplest জিনিস দিয়ে যা এই requirement fulfill করবে: সবসময় empty string slice return করে। একবার যখন আমরা post এর state change করার ability implement করব, তখন আমরা এটা change করব যাতে এটি publish হতে পারে। এখন পর্যন্ত, post শুধুমাত্র draft state এ থাকতে পারে, তাই post content সবসময় empty থাকা উচিত। Listing 18-14 এই placeholder implementation দেখায়:

<Listing number="18-14" file-name="src/lib.rs" caption="`Post` এর উপর `content` method এর জন্য একটি placeholder implementation add করা যা সবসময় empty string slice return করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

</Listing>

এই added `content` method এর সাথে, Listing 18-11 এর 7 নম্বর লাইন পর্যন্ত সবকিছু intended ভাবে কাজ করে।

### Requesting a Review of the Post Changes Its State

এরপর, আমাদের post এর review request করার জন্য functionality add করার প্রয়োজন, যা এর state `Draft` থেকে `PendingReview` এ change করবে। Listing 18-15 এই code দেখায়:

<Listing number="18-15" file-name="src/lib.rs" caption="`Post` এবং `State` trait এর উপর `request_review` method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

</Listing>

আমরা `Post` কে `request_review` নামের একটি public method দেই যা `self` এ একটি mutable reference নেবে। তারপর আমরা `Post` এর current state এ একটি internal `request_review` method call করি, এবং এই second `request_review` method current state consume করে এবং একটি নতুন state return করে।

আমরা `State` trait এ `request_review` method add করি; trait implement করা সব type এর এখন `request_review` method implement করার প্রয়োজন হবে। Note করুন যে method এর first parameter হিসেবে `self`, `&self`, বা `&mut self` থাকার পরিবর্তে, আমাদের `self: Box<Self>` আছে। এই syntax মানে হলো type hold করা `Box` এ call করলেই method টি valid। এই syntax `Box<Self>` এর ownership নেয়, old state কে invalid করে যাতে `Post` এর state value নতুন state এ transform হতে পারে।

Old state consume করার জন্য, `request_review` method এর state value এর ownership নেওয়ার প্রয়োজন। এখানেই `Post` এর `state` field এর `Option` কাজে লাগে: আমরা `state` field থেকে `Some` value নেওয়ার জন্য `take` method call করি এবং এর জায়গায় একটি `None` রাখি, কারণ Rust struct এ unpopulated field রাখার অনুমতি দেয় না। এটা আমাদের `Post` থেকে `state` value move করতে দেয় borrow করার পরিবর্তে। তারপর আমরা post এর `state` value এই operation এর result এ set করব।

আমরা `state` value এর ownership পাওয়ার জন্য directly `self.state = self.state.request_review();` এর মতো code দিয়ে directly set করার পরিবর্তে temporarily `state` কে `None` set করতে চাই। এটা নিশ্চিত করে যে `Post` নতুন state এ transform করার পর old `state` value ব্যবহার করতে পারবে না।

`Draft` এর উপর `request_review` method একটি নতুন, boxed instance return করে একটি নতুন `PendingReview` struct এর, যা একটি post review এর জন্য wait করার সময় state represent করে। `PendingReview` struct ও `request_review` method implement করে কিন্তু কোনো transformation করে না। বরং, এটা self return করে, কারণ যখন আমরা ইতিমধ্যে `PendingReview` state এ থাকা post এর review request করি, তখন এটির `PendingReview` state এ থাকা উচিত।

এখন আমরা state pattern এর advantage গুলো দেখতে শুরু করতে পারি: `Post` এর উপর `request_review` method একই থাকে তা `state` value যাই হোক না কেন। প্রত্যেক state তার নিজের rule এর জন্য responsible।

আমরা `Post` এর `content` method as is রেখে দেবো, একটি empty string slice return করে। আমরা এখন `Draft` state এর সাথে সাথে `PendingReview` state এ একটি `Post` রাখতে পারি, কিন্তু আমরা `PendingReview` state এ একই behaviour চাই। Listing 18-11 এখন 10 নম্বর লাইন পর্যন্ত কাজ করে!

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-the-approve-method-that-changes-the-behavior-of-content"></a>

### Adding `approve` to Change the Behavior of `content`

`approve` method `request_review` method এর similar হবে: যখন সেই state approve হবে তখন current state যা বলবে সে অনুযায়ী এটি `state` set করবে, যা Listing 18-16 এ দেখানো হয়েছে:

<Listing number="18-16" file-name="src/lib.rs" caption="`Post` এবং `State` trait এর উপর `approve` method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

</Listing>

আমরা `State` trait এ `approve` method add করি এবং `State` implement করা একটি নতুন struct add করি, `Published` state।

`PendingReview` এর উপর `request_review` যেভাবে কাজ করে, সেভাবে, যদি আমরা একটি `Draft` এর উপর `approve` call করি, তাহলে এর কোনো effect হবে না কারণ `approve` `self` return করবে। যখন আমরা `PendingReview` এর উপর `approve` call করি, তখন এটি `Published` struct এর একটি নতুন, boxed instance return করে। `Published` struct `State` trait implement করে, এবং `request_review` method এবং `approve` method উভয়ের জন্য, এটি self return করে, কারণ post টি সেই case গুলোতে `Published` state এই থাকা উচিত।

এখন আমাদের `Post` এর `content` method update করার প্রয়োজন। আমরা `content` থেকে return হওয়া value `Post` এর current state এর উপর depend করতে চাই, তাই আমরা `Post` কে একটি `content` method এ delegate করব যা এর `state` এ defined, যা Listing 18-17 এ দেখানো হয়েছে:

<Listing number="18-17" file-name="src/lib.rs" caption="`Post` এর `content` method update করে `State` এর উপর `content` method এ delegate করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src/lib.rs:here}}
```

</Listing>

যেহেতু goal হলো সব rule `State` implement করা struct এর ভিতরে রাখা, তাই আমরা `state` এ value এর উপর `content` method call করি এবং argument হিসেবে post instance (মানে `self`) pass করি। তারপর আমরা `state` value এর উপর `content` method ব্যবহার করে return হওয়া value return করি।

আমরা `Option` এর উপর `as_ref` method call করি কারণ আমরা value এর ownership এর পরিবর্তে `Option` এর ভিতরের value এর reference চাই। যেহেতু `state` একটি `Option<Box<dyn State>>`, তাই যখন আমরা `as_ref` call করি, তখন একটি `Option<&Box<dyn State>>` return হয়। যদি আমরা `as_ref` call না করি, তাহলে আমরা error পেতাম কারণ function parameter এর borrowed `&self` থেকে আমরা `state` move করতে পারব না।

তারপর আমরা `unwrap` method call করি, যা আমরা জানি কখনো panic করবে না, কারণ আমরা জানি যে `Post` এর method গুলো ensure করে যে যখন সেই method গুলো শেষ হয় তখন `state` এ সবসময় একটি `Some` value থাকে। এটা সেই case গুলোর মধ্যে একটি যা নিয়ে আমরা Chapter 9 এর [“Cases In Which You Have More Information Than the Compiler”][more-info-than-rustc]<!-- ignore --> section এ আলোচনা করেছিলাম যখন আমরা জানি যে `None` value never possible, যদিও compiler সেটা বুঝতে পারে না।

এই মুহূর্তে, যখন আমরা `&Box<dyn State>` এর উপর `content` call করি, তখন deref coercion `&` এবং `Box` এর উপর effect করবে তাই `content` method ultimately সেই type এর উপর call হবে যা `State` trait implement করে। এর মানে হলো আমাদের `State` trait definition এ `content` add করতে হবে, এবং সেখানেই আমরা কোন state এর উপর depend করে কি content return করবে তার logic রাখব, যা Listing 18-18 এ দেখানো হয়েছে:

<Listing number="18-18" file-name="src/lib.rs" caption="`State` trait এ `content` method add করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src/lib.rs:here}}
```

</Listing>

আমরা `content` method এর জন্য একটি default implementation add করি যা একটি empty string slice return করে। এর মানে হলো `Draft` এবং `PendingReview` struct এ আমাদের `content` implement করার প্রয়োজন নেই। `Published` struct `content` method override করবে এবং `post.content` এ value return করবে।

Note করুন যে এই method এ আমাদের lifetime annotation এর প্রয়োজন, যেমনটা আমরা Chapter 10 এ discuss করেছিলাম। আমরা argument হিসেবে `post` এ reference নিচ্ছি এবং সেই `post` এর part এ reference return করছি, তাই return হওয়া reference এর lifetime `post` argument এর lifetime এর সাথে related।

এবং আমরা done—Listing 18-11 এর সবকিছু এখন কাজ করে! আমরা blog post workflow এর rules দিয়ে state pattern implement করেছি। Rule related logic `Post` জুড়ে scattered থাকার পরিবর্তে state object এ থাকে।

> #### Why Not An Enum?
>
> আপনি হয়তো ভাবছিলেন কেন আমরা different possible post state variant হিসেবে `enum` ব্যবহার করিনি। এটা নিশ্চিতভাবেই একটি possible solution, try করুন এবং compare করে দেখুন আপনার কোনটি বেশি পছন্দ! একটি enum ব্যবহার করার একটি disadvantage হলো enum এর value check করা প্রত্যেক জায়গায় সব possible variant handle করার জন্য একটি `match` expression বা similar কিছু এর প্রয়োজন। এটা trait object solution এর চেয়ে বেশি repetitive হতে পারে।

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler

## Implementing an Object-Oriented Design Pattern

_State pattern_ হলো একটি object-oriented design pattern। Pattern টির মূল বিষয় হলো আমরা এমন set of state define করি যা একটি value internally hold করতে পারে। State গুলো set of _state object_ দ্বারা represent করা হয়, এবং value এর behaviour এর state এর উপর depend করে change হয়। আমরা একটি blog post struct এর উদাহরণ নিয়ে কাজ করতে যাচ্ছি যেখানে এর state hold করার জন্য একটি field থাকবে, যা "draft", "review", বা "published" set থেকে একটি state object হবে।

State object গুলো functionality share করে: Rust এ, অবশ্যই আমরা object এবং inheritance এর পরিবর্তে struct এবং trait ব্যবহার করি। প্রত্যেক state object তার নিজের behaviour এর জন্য responsible এবং কখন এটি অন্য state এ change হওয়া উচিত তা govern করার জন্য responsible। State object hold করা value state গুলোর different behaviour বা কখন state গুলোর মধ্যে transition করতে হবে সে বিষয়ে কিছুই জানে না।

State pattern ব্যবহার করার advantage হলো, যখন program এর business requirement change হয়, তখন state hold করা value এর code change করার প্রয়োজন হবে না বা সেই value ব্যবহার করা code change করার প্রয়োজন হবে না। Rules change করার জন্য বা হয়তো আরও state object add করার জন্য আমাদের শুধুমাত্র state object গুলোর ভিতরের code update করার প্রয়োজন হবে।

প্রথমে, আমরা state pattern implement করতে যাচ্ছি more traditional object-oriented উপায়ে, তারপর আমরা এমন একটি approach ব্যবহার করব যা Rust এ আরও natural। চলুন state pattern ব্যবহার করে incrementally blog post workflow implement করা শুরু করি।

Final functionality টি দেখতে এমন হবে:

1. একটি blog post empty draft হিসেবে শুরু হবে।
2. যখন draft শেষ হয়ে যাবে, তখন post এর review request করা হবে।
3. যখন post approve করা হবে, তখন এটি publish হবে।
4. শুধুমাত্র published blog post print করার জন্য content return করবে, তাই unapproved post accidentally publish হওয়া উচিত নয়।

Post এ attempt করা অন্য change এর কোনো effect থাকা উচিত নয়। উদাহরণস্বরূপ, review request করার আগে যদি আমরা কোনো draft blog post approve করার চেষ্টা করি, তাহলে post টি unpublished draft হিসেবে থেকে যাওয়া উচিত।

Listing 18-11 এই workflow কে code form এ দেখায়: এটি `blog` নামের একটি library crate এ আমরা যে API implement করব তার একটি উদাহরণ। এটি এখনও compile হবে না কারণ আমরা এখনও `blog` crate implement করিনি।

<Listing number="18-11" file-name="src/main.rs" caption="Code যা `blog` crate এ আমরা যে desired behaviour চাই তা demonstrate করে">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

</Listing>

আমরা user কে `Post::new` দিয়ে নতুন draft blog post তৈরি করার allow করতে চাই। আমরা blog post এ text add করার allow করতে চাই। যদি আমরা approval এর আগে, post এর content immediately পাওয়ার চেষ্টা করি, তাহলে আমাদের কোনো text পাওয়া উচিত না কারণ post টি এখনও draft। আমরা demonstration purposes এর জন্য code এ `assert_eq!` add করেছি। এটার জন্য একটি excellent unit test হবে এটা assert করা যে একটি draft blog post `content` method থেকে একটি empty string return করে, কিন্তু আমরা এই উদাহরণের জন্য test লিখব না।

এরপর, আমরা post এর review request enable করতে চাই, এবং review এর জন্য wait করার সময় `content` কে একটি empty string return করা উচিত। যখন post approve করা হবে, তখন এটি publish হওয়া উচিত, মানে যখন `content` call করা হয় তখন post এর text return হবে।

লক্ষ্য করুন যে crate থেকে আমরা interact করছি এমন একমাত্র type হলো `Post` type। এই type টি state pattern ব্যবহার করবে এবং একটি value hold করবে যা তিনটি state object এর মধ্যে একটি হবে যা represent করে যে post টি draft, review এর জন্য waiting, বা published এর মতো বিভিন্ন state এ থাকতে পারে। একটি state থেকে অন্য state এ change করা `Post` type এর ভিতরে internally manage করা হবে। State গুলো change হয় আমাদের library এর user দ্বারা `Post` instance এর উপর call করা method এর response এ, কিন্তু তাদের state change directly manage করার প্রয়োজন নেই। এছাড়াও, user রা state নিয়ে ভুল করতে পারবে না, যেমন review করার আগে post publish করা।

### Defining `Post` and Creating a New Instance in the Draft State

চলুন library এর implementation শুরু করি! আমরা জানি আমাদের কিছু content hold করে এমন একটি public `Post` struct এর প্রয়োজন, তাই আমরা struct এর definition এবং `Post` এর একটি instance তৈরি করার জন্য associated public `new` function দিয়ে শুরু করব, যা Listing 18-12 এ দেখানো হয়েছে। আমরা একটি private `State` trait ও তৈরি করব যা এমন behaviour define করবে যা `Post` এর সব state object এর থাকা উচিত।

তারপর `Post` একটি `Option<T>` এর ভিতরে `Box<dyn State>` এর trait object hold করবে যা `state` নামের একটি private field এ state object hold করার জন্য। আপনি একটু পরেই দেখবেন কেন `Option<T>` প্রয়োজনীয়।

<Listing number="18-12" file-name="src/lib.rs" caption="`Post` struct এর definition এবং একটি `new` function যা একটি নতুন `Post` instance, একটি `State` trait, এবং একটি `Draft` struct তৈরি করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

</Listing>

`State` trait different post state দ্বারা shared behavior define করে। State object গুলো হলো `Draft`, `PendingReview`, এবং `Published`, এবং তারা সবাই `State` trait implement করবে। আপাতত, trait এ কোনো method নেই, এবং আমরা শুধু `Draft` state define করে শুরু করব কারণ আমরা চাই post টি এই state এ শুরু হোক।

যখন আমরা একটি নতুন `Post` তৈরি করি, তখন আমরা এর `state` field একটি `Some` value set করি যা একটি `Box` hold করে। এই `Box` `Draft` struct এর একটি নতুন instance point করে। এটা নিশ্চিত করে যখনই আমরা `Post` এর নতুন instance তৈরি করি, তখনই এটি draft হিসেবে শুরু হবে। যেহেতু `Post` এর `state` field private, তাই অন্য কোনো state এ `Post` তৈরি করার কোনো উপায় নেই! `Post::new` function এ, আমরা `content` field কে একটি নতুন, empty `String` set করি।

### Storing the Text of the Post Content

আমরা Listing 18-11 এ দেখেছিলাম যে আমরা `add_text` নামের একটি method call করতে চাই এবং এটিকে একটি `&str` pass করতে চাই যা তারপর blog post এর text content হিসেবে add হবে। আমরা `content` field কে `pub` হিসেবে expose করার পরিবর্তে এটিকে method হিসেবে implement করি, যাতে পরে আমরা এমন একটি method implement করতে পারি যা control করবে কিভাবে `content` field এর data read করা হবে। `add_text` method টি বেশ straightforward, তাই চলুন Listing 18-13 এ `impl Post` block এ implementation add করি:

<Listing number="18-13" file-name="src/lib.rs" caption="Post এর `content` এ text add করার জন্য `add_text` method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

</Listing>

`add_text` method `self` এ একটি mutable reference নেয়, কারণ আমরা সেই `Post` instance change করছি যার উপর আমরা `add_text` call করছি। তারপর আমরা `content` এ `String` এর উপর `push_str` call করি এবং saved `content` এ add করার জন্য `text` argument pass করি। এই behaviour post কোন state এ আছে তার উপর depend করে না, তাই এটা state pattern এর অংশ নয়। `add_text` method `state` field এর সাথে interact করে না, কিন্তু এটা সেই behaviour এর অংশ যা আমরা support করতে চাই।

### Ensuring the Content of a Draft Post Is Empty

এমনকি `add_text` call করে post এ কিছু content add করার পরেও, আমরা এখনও চাই যে `content` method একটি empty string slice return করুক কারণ post টি এখনও draft state এ আছে, যেমন Listing 18-11 এর 7 নম্বর লাইনে দেখানো হয়েছে। আপাতত, চলুন `content` method implement করি simplest জিনিস দিয়ে যা এই requirement fulfill করবে: সবসময় empty string slice return করে। একবার যখন আমরা post এর state change করার ability implement করব, তখন আমরা এটা change করব যাতে এটি publish হতে পারে। এখন পর্যন্ত, post শুধুমাত্র draft state এ থাকতে পারে, তাই post content সবসময় empty থাকা উচিত। Listing 18-14 এই placeholder implementation দেখায়:

<Listing number="18-14" file-name="src/lib.rs" caption="`Post` এর উপর `content` method এর জন্য একটি placeholder implementation add করা যা সবসময় empty string slice return করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

</Listing>

এই added `content` method এর সাথে, Listing 18-11 এর 7 নম্বর লাইন পর্যন্ত সবকিছু intended ভাবে কাজ করে।

### Requesting a Review of the Post Changes Its State

এরপর, আমাদের post এর review request করার জন্য functionality add করার প্রয়োজন, যা এর state `Draft` থেকে `PendingReview` এ change করবে। Listing 18-15 এই code দেখায়:

<Listing number="18-15" file-name="src/lib.rs" caption="`Post` এবং `State` trait এর উপর `request_review` method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

</Listing>

আমরা `Post` কে `request_review` নামের একটি public method দেই যা `self` এ একটি mutable reference নেবে। তারপর আমরা `Post` এর current state এ একটি internal `request_review` method call করি, এবং এই second `request_review` method current state consume করে এবং একটি নতুন state return করে।

আমরা `State` trait এ `request_review` method add করি; trait implement করা সব type এর এখন `request_review` method implement করার প্রয়োজন হবে। Note করুন যে method এর first parameter হিসেবে `self`, `&self`, বা `&mut self` থাকার পরিবর্তে, আমাদের `self: Box<Self>` আছে। এই syntax মানে হলো type hold করা `Box` এ call করলেই method টি valid। এই syntax `Box<Self>` এর ownership নেয়, old state কে invalid করে যাতে `Post` এর state value নতুন state এ transform হতে পারে।

Old state consume করার জন্য, `request_review` method এর state value এর ownership নেওয়ার প্রয়োজন। এখানেই `Post` এর `state` field এর `Option` কাজে লাগে: আমরা `state` field থেকে `Some` value নেওয়ার জন্য `take` method call করি এবং এর জায়গায় একটি `None` রাখি, কারণ Rust struct এ unpopulated field রাখার অনুমতি দেয় না। এটা আমাদের `Post` থেকে `state` value move করতে দেয় borrow করার পরিবর্তে। তারপর আমরা post এর `state` value এই operation এর result এ set করব।

আমরা `state` value এর ownership পাওয়ার জন্য directly `self.state = self.state.request_review();` এর মতো code দিয়ে directly set করার পরিবর্তে temporarily `state` কে `None` set করতে চাই। এটা নিশ্চিত করে যে `Post` নতুন state এ transform করার পর old `state` value ব্যবহার করতে পারবে না।

`Draft` এর উপর `request_review` method একটি নতুন, boxed instance return করে একটি নতুন `PendingReview` struct এর, যা একটি post review এর জন্য wait করার সময় state represent করে। `PendingReview` struct ও `request_review` method implement করে কিন্তু কোনো transformation করে না। বরং, এটা self return করে, কারণ যখন আমরা ইতিমধ্যে `PendingReview` state এ থাকা post এর review request করি, তখন এটির `PendingReview` state এ থাকা উচিত।

এখন আমরা state pattern এর advantage গুলো দেখতে শুরু করতে পারি: `Post` এর উপর `request_review` method একই থাকে তা `state` value যাই হোক না কেন। প্রত্যেক state তার নিজের rule এর জন্য responsible।

আমরা `Post` এর `content` method as is রেখে দেবো, একটি empty string slice return করে। আমরা এখন `Draft` state এর সাথে সাথে `PendingReview` state এ একটি `Post` রাখতে পারি, কিন্তু আমরা `PendingReview` state এ একই behaviour চাই। Listing 18-11 এখন 10 নম্বর লাইন পর্যন্ত কাজ করে!

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-the-approve-method-that-changes-the-behavior-of-content"></a>

### Adding `approve` to Change the Behavior of `content`

`approve` method `request_review` method এর similar হবে: যখন সেই state approve হবে তখন current state যা বলবে সে অনুযায়ী এটি `state` set করবে, যা Listing 18-16 এ দেখানো হয়েছে:

<Listing number="18-16" file-name="src/lib.rs" caption="`Post` এবং `State` trait এর উপর `approve` method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

</Listing>

আমরা `State` trait এ `approve` method add করি এবং `State` implement করা একটি নতুন struct add করি, `Published` state।

`PendingReview` এর উপর `request_review` যেভাবে কাজ করে, সেভাবে, যদি আমরা একটি `Draft` এর উপর `approve` call করি, তাহলে এর কোনো effect হবে না কারণ `approve` `self` return করবে। যখন আমরা `PendingReview` এর উপর `approve` call করি, তখন এটি `Published` struct এর একটি নতুন, boxed instance return করে। `Published` struct `State` trait implement করে, এবং `request_review` method এবং `approve` method উভয়ের জন্য, এটি self return করে, কারণ post টি সেই case গুলোতে `Published` state এই থাকা উচিত।

এখন আমাদের `Post` এর `content` method update করার প্রয়োজন। আমরা `content` থেকে return হওয়া value `Post` এর current state এর উপর depend করতে চাই, তাই আমরা `Post` কে একটি `content` method এ delegate করব যা এর `state` এ defined, যা Listing 18-17 এ দেখানো হয়েছে:

<Listing number="18-17" file-name="src/lib.rs" caption="`Post` এর `content` method update করে `State` এর উপর `content` method এ delegate করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src/lib.rs:here}}
```

</Listing>

যেহেতু goal হলো সব rule `State` implement করা struct এর ভিতরে রাখা, তাই আমরা `state` এ value এর উপর `content` method call করি এবং argument হিসেবে post instance (মানে `self`) pass করি। তারপর আমরা `state` value এর উপর `content` method ব্যবহার করে return হওয়া value return করি।

আমরা `Option` এর উপর `as_ref` method call করি কারণ আমরা value এর ownership এর পরিবর্তে `Option` এর ভিতরের value এর reference চাই। যেহেতু `state` একটি `Option<Box<dyn State>>`, তাই যখন আমরা `as_ref` call করি, তখন একটি `Option<&Box<dyn State>>` return হয়। যদি আমরা `as_ref` call না করি, তাহলে আমরা error পেতাম কারণ function parameter এর borrowed `&self` থেকে আমরা `state` move করতে পারব না।

তারপর আমরা `unwrap` method call করি, যা আমরা জানি কখনো panic করবে না, কারণ আমরা জানি যে `Post` এর method গুলো ensure করে যে যখন সেই method গুলো শেষ হয় তখন `state` এ সবসময় একটি `Some` value থাকে। এটা সেই case গুলোর মধ্যে একটি যা নিয়ে আমরা Chapter 9 এর [“Cases In Which You Have More Information Than the Compiler”][more-info-than-rustc]<!-- ignore --> section এ আলোচনা করেছিলাম যখন আমরা জানি যে `None` value never possible, যদিও compiler সেটা বুঝতে পারে না।

এই মুহূর্তে, যখন আমরা `&Box<dyn State>` এর উপর `content` call করি, তখন deref coercion `&` এবং `Box` এর উপর effect করবে তাই `content` method ultimately সেই type এর উপর call হবে যা `State` trait implement করে। এর মানে হলো আমাদের `State` trait definition এ `content` add করতে হবে, এবং সেখানেই আমরা কোন state এর উপর depend করে কি content return করবে তার logic রাখব, যা Listing 18-18 এ দেখানো হয়েছে:

<Listing number="18-18" file-name="src/lib.rs" caption="`State` trait এ `content` method add করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src/lib.rs:here}}
```

</Listing>

আমরা `content` method এর জন্য একটি default implementation add করি যা একটি empty string slice return করে। এর মানে হলো `Draft` এবং `PendingReview` struct এ আমাদের `content` implement করার প্রয়োজন নেই। `Published` struct `content` method override করবে এবং `post.content` এ value return করবে।

Note করুন যে এই method এ আমাদের lifetime annotation এর প্রয়োজন, যেমনটা আমরা Chapter 10 এ discuss করেছিলাম। আমরা argument হিসেবে `post` এ reference নিচ্ছি এবং সেই `post` এর part এ reference return করছি, তাই return হওয়া reference এর lifetime `post` argument এর lifetime এর সাথে related।

এবং আমরা done—Listing 18-11 এর সবকিছু এখন কাজ করে! আমরা blog post workflow এর rules দিয়ে state pattern implement করেছি। Rule related logic `Post` জুড়ে scattered থাকার পরিবর্তে state object এ থাকে।

> #### Why Not An Enum?
>
> আপনি হয়তো ভাবছিলেন কেন আমরা different possible post state variant হিসেবে `enum` ব্যবহার করিনি। এটা নিশ্চিতভাবেই একটি possible solution, try করুন এবং compare করে দেখুন আপনার কোনটি বেশি পছন্দ! একটি enum ব্যবহার করার একটি disadvantage হলো enum এর value check করা প্রত্যেক জায়গায় সব possible variant handle করার জন্য একটি `match` expression বা similar কিছু এর প্রয়োজন। এটা trait object solution এর চেয়ে বেশি repetitive হতে পারে।

