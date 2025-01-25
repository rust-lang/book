## Structs সংজ্ঞায়িত করা এবং ইনস্ট্যানশিয়েট করা

Structs, [“The Tuple Type”][tuples]<!-- ignore --> বিভাগে আলোচিত tuples এর মতো, কারণ উভয়ই একাধিক সম্পর্কিত মান ধারণ করে। tuples এর মতো, একটি struct এর অংশগুলি বিভিন্ন প্রকারের হতে পারে। tuples এর বিপরীতে, একটি struct এ আপনি ডেটার প্রতিটি অংশের নাম দেবেন যাতে মানগুলির অর্থ পরিষ্কার হয়। এই নামগুলি যোগ করার অর্থ হল struct গুলি tuples এর চেয়ে বেশি নমনীয়: একটি ইনস্ট্যান্সের মানগুলি নির্দিষ্ট করতে বা অ্যাক্সেস করতে আপনাকে ডেটার ক্রমের উপর নির্ভর করতে হবে না।

একটি struct সংজ্ঞায়িত করতে, আমরা `struct` কীওয়ার্ড লিখি এবং পুরো struct টির নাম দিই। একটি struct এর নাম ডেটার অংশগুলিকে একসাথে গোষ্ঠীভুক্ত করার তাৎপর্য বর্ণনা করা উচিত। তারপরে, কার্লি ব্র্যাকেটের ভিতরে, আমরা ডেটার অংশগুলির নাম এবং প্রকারগুলি সংজ্ঞায়িত করি, যেগুলিকে আমরা _fields_ বলি। উদাহরণস্বরূপ, Listing 5-1 একটি struct দেখায় যা একটি ব্যবহারকারীর অ্যাকাউন্ট সম্পর্কে তথ্য স্টোর করে।

<Listing number="5-1" file-name="src/main.rs" caption="একটি `User` struct এর সংজ্ঞা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

সংজ্ঞায়িত করার পরে একটি struct ব্যবহার করতে, আমরা প্রতিটি ফিল্ডের জন্য নির্দিষ্ট মান উল্লেখ করে সেই struct এর একটি _instance_ তৈরি করি। আমরা struct এর নামটি উল্লেখ করে এবং তারপরে _key: value_ জোড় ধারণকারী কার্লি ব্র্যাকেট যোগ করে একটি instance তৈরি করি, যেখানে কীগুলি হল ফিল্ডগুলির নাম এবং মানগুলি হল সেই ফিল্ডগুলিতে আমরা যে ডেটা স্টোর করতে চাই। struct এ আমরা যেভাবে ডিক্লেয়ার করেছি, সেই একই ক্রমে আমাদের ফিল্ডগুলি নির্দিষ্ট করতে হবে না। অন্য কথায়, struct সংজ্ঞাটি টাইপের জন্য একটি সাধারণ টেমপ্লেটের মতো, এবং ইনস্ট্যান্সগুলি সেই টেমপ্লেটটিকে নির্দিষ্ট ডেটা দিয়ে পূরণ করে টাইপের মান তৈরি করে। উদাহরণস্বরূপ, Listing 5-2 এ দেখানো হিসাবে, আমরা একটি নির্দিষ্ট ব্যবহারকারীকে ঘোষণা করতে পারি।

<Listing number="5-2" file-name="src/main.rs" caption="`User` struct এর একটি instance তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

একটি struct থেকে একটি নির্দিষ্ট মান পেতে, আমরা ডট নোটেশন ব্যবহার করি। উদাহরণস্বরূপ, এই ব্যবহারকারীর ইমেল ঠিকানা অ্যাক্সেস করতে, আমরা `user1.email` ব্যবহার করি। যদি instance টি mutable হয়, তবে আমরা ডট নোটেশন ব্যবহার করে এবং একটি নির্দিষ্ট ফিল্ডে অ্যাসাইন করে একটি মান পরিবর্তন করতে পারি। Listing 5-3 দেখায় কিভাবে একটি mutable `User` instance এর `email` ফিল্ডের মান পরিবর্তন করতে হয়।

<Listing number="5-3" file-name="src/main.rs" caption="একটি `User` instance এর `email` ফিল্ডের মান পরিবর্তন করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

মনে রাখবেন যে পুরো instance টিকে mutable হতে হবে; Rust আমাদের শুধুমাত্র কিছু ফিল্ডকে mutable হিসাবে চিহ্নিত করার অনুমতি দেয় না। যেকোনো এক্সপ্রেশনের মতো, আমরা একটি ফাংশন বডির শেষ এক্সপ্রেশন হিসাবে struct এর একটি নতুন instance তৈরি করতে পারি যাতে সেই নতুন instance টি অন্তর্নিহিতভাবে ফেরত আসে।

Listing 5-4 একটি `build_user` ফাংশন দেখায় যা প্রদত্ত ইমেল এবং ব্যবহারকারীর নাম সহ একটি `User` instance ফেরত দেয়। `active` ফিল্ডের মান `true` এবং `sign_in_count` এর মান `1` হয়।

<Listing number="5-4" file-name="src/main.rs" caption="একটি `build_user` ফাংশন যা একটি ইমেল এবং ব্যবহারকারীর নাম নেয় এবং একটি `User` instance ফেরত দেয়">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

ফাংশন প্যারামিটারগুলির নাম struct ফিল্ডগুলির মতোই রাখা যুক্তিযুক্ত, তবে `email` এবং `username` ফিল্ডের নাম এবং ভেরিয়েবলগুলি পুনরাবৃত্তি করা একটু ক্লান্তিকর। যদি struct এর আরও ফিল্ড থাকত, তবে প্রতিটি নামের পুনরাবৃত্তি করা আরও বিরক্তিকর হত। সৌভাগ্যক্রমে, একটি সুবিধাজনক সংক্ষিপ্ত রূপ আছে!

<!-- Old heading. Do not remove or links may break. -->

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### Field Init Shorthand ব্যবহার করা

যেহেতু Listing 5-4 এ প্যারামিটারের নাম এবং struct ফিল্ডের নামগুলি একেবারে একই, তাই আমরা `build_user` কে পুনরায় লেখার জন্য _field init shorthand_ সিনট্যাক্স ব্যবহার করতে পারি যাতে এটি অবিকল একই রকম আচরণ করে তবে `username` এবং `email` এর পুনরাবৃত্তি না হয়, যেমন Listing 5-5 এ দেখানো হয়েছে।

<Listing number="5-5" file-name="src/main.rs" caption="একটি `build_user` ফাংশন যা field init shorthand ব্যবহার করে কারণ `username` এবং `email` প্যারামিটারগুলির নাম struct ফিল্ডগুলির নামের মতোই">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

এখানে, আমরা `User` struct এর একটি নতুন instance তৈরি করছি, যার `email` নামের একটি ফিল্ড আছে। আমরা `email` ফিল্ডের মান `build_user` ফাংশনের `email` প্যারামিটারের মানে সেট করতে চাই। যেহেতু `email` ফিল্ড এবং `email` প্যারামিটারের নাম একই, তাই আমাদের `email: email` এর পরিবর্তে শুধুমাত্র `email` লিখতে হবে।

### Struct Update সিনট্যাক্স সহ অন্যান্য Instance থেকে Instance তৈরি করা

প্রায়শই একটি struct এর একটি নতুন instance তৈরি করা কার্যকর, যেখানে অন্য instance এর বেশিরভাগ মান থাকে, তবে কিছু পরিবর্তিত হয়। আপনি _struct update syntax_ ব্যবহার করে এটি করতে পারেন।

প্রথমে, Listing 5-6 এ আমরা দেখাই কিভাবে আপডেট সিনট্যাক্স ছাড়াই, `user2` এ একটি নতুন `User` instance তৈরি করতে হয়। আমরা `email` এর জন্য একটি নতুন মান সেট করি তবে অন্যথায় Listing 5-2 এ তৈরি করা `user1` থেকে একই মানগুলি ব্যবহার করি।

<Listing number="5-6" file-name="src/main.rs" caption="`user1` থেকে একটি মান ছাড়া বাকি সব মান ব্যবহার করে একটি নতুন `User` instance তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

struct update সিনট্যাক্স ব্যবহার করে, আমরা কম কোড দিয়ে একই প্রভাব অর্জন করতে পারি, যেমন Listing 5-7 এ দেখানো হয়েছে। `..` সিনট্যাক্সটি নির্দিষ্ট করে যে স্পষ্টভাবে সেট না করা অবশিষ্ট ফিল্ডগুলির মান প্রদত্ত instance এর ফিল্ডগুলির মানের মতো হওয়া উচিত।

<Listing number="5-7" file-name="src/main.rs" caption="একটি `User` instance এর জন্য একটি নতুন `email` মান সেট করতে struct update সিনট্যাক্স ব্যবহার করা কিন্তু `user1` থেকে বাকি মানগুলি ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

Listing 5-7 এর কোডটি `user2` এ একটি instance তৈরি করে যার `email` এর জন্য একটি ভিন্ন মান রয়েছে তবে `user1` থেকে `username`, `active` এবং `sign_in_count` ফিল্ডগুলির জন্য একই মান রয়েছে। `..user1` অবশ্যই শেষে আসতে হবে এটি নির্দিষ্ট করার জন্য যে কোনও অবশিষ্ট ফিল্ডের মান `user1` এর সংশ্লিষ্ট ফিল্ড থেকে নেওয়া উচিত, তবে আমরা struct এর সংজ্ঞার ফিল্ডগুলির ক্রম নির্বিশেষে, যেকোনো ক্রমে যতগুলি ফিল্ডের জন্য চাই মান নির্দিষ্ট করতে পারি।

মনে রাখবেন যে struct update সিনট্যাক্স অ্যাসাইনমেন্টের মতো `=` ব্যবহার করে; এর কারণ হল এটি ডেটা move করে, ঠিক যেমন আমরা [“Variables and Data Interacting with Move”][move]<!-- ignore --> বিভাগে দেখেছি। এই উদাহরণে, `user2` তৈরি করার পরে আমরা `user1` কে সামগ্রিকভাবে আর ব্যবহার করতে পারি না কারণ `user1` এর `username` ফিল্ডের `String` টি `user2` তে move করা হয়েছিল। যদি আমরা `user2` কে `email` এবং `username` উভয়ের জন্য নতুন `String` মান দিতাম এবং এইভাবে `user1` থেকে শুধুমাত্র `active` এবং `sign_in_count` মানগুলি ব্যবহার করতাম, তাহলে `user2` তৈরি করার পরেও `user1` বৈধ থাকত। `active` এবং `sign_in_count` উভয়ই এমন প্রকার যা `Copy` trait প্রয়োগ করে, তাই [“Stack-Only Data: Copy”][copy]<!-- ignore --> বিভাগে আমরা যে আচরণ নিয়ে আলোচনা করেছি তা প্রযোজ্য হবে। আমরা এখনও এই উদাহরণে `user1.email` ব্যবহার করতে পারি, যেহেতু এর মান move করা _হয়নি_।

### Named ফিল্ড ছাড়া Tuple Structs ব্যবহার করে বিভিন্ন প্রকার তৈরি করা

Rust এমন structs ও সমর্থন করে যা tuples এর মতো, যাদের _tuple structs_ বলা হয়। Tuple structs এর struct নামের অতিরিক্ত অর্থ রয়েছে তবে তাদের ফিল্ডগুলির সাথে যুক্ত নাম নেই; বরং, তাদের শুধুমাত্র ফিল্ডগুলির প্রকার রয়েছে। Tuple structs কার্যকর যখন আপনি পুরো tuple কে একটি নাম দিতে চান এবং tuple টিকে অন্যান্য tuples থেকে আলাদা প্রকার বানাতে চান, এবং যখন একটি সাধারণ struct এর মতো প্রতিটি ফিল্ডের নামকরণ করা ভার্বোস বা অপ্রয়োজনীয় হবে।

একটি tuple struct সংজ্ঞায়িত করতে, `struct` কীওয়ার্ড এবং tuple এর প্রকারগুলির পরে struct নাম দিয়ে শুরু করুন। উদাহরণস্বরূপ, এখানে আমরা `Color` এবং `Point` নামের দুটি tuple struct সংজ্ঞায়িত এবং ব্যবহার করি:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

মনে রাখবেন যে `black` এবং `origin` মানগুলি ভিন্ন প্রকারের কারণ তারা বিভিন্ন tuple struct এর instance। আপনি যে প্রতিটি struct সংজ্ঞায়িত করেন তা নিজস্ব প্রকার, এমনকি যদি struct এর ভিতরের ফিল্ডগুলির প্রকার একই থাকে। উদাহরণস্বরূপ, একটি ফাংশন যা `Color` টাইপের একটি প্যারামিটার নেয়, তা আর্গুমেন্ট হিসাবে `Point` নিতে পারে না, যদিও উভয় প্রকারই তিনটি `i32` মান দিয়ে তৈরি। অন্যথায়, tuple struct instance গুলি tuples এর মতোই, আপনি সেগুলিকে তাদের পৃথক অংশে destructure করতে পারেন এবং আপনি একটি `.` এবং তারপরে একটি পৃথক মান অ্যাক্সেস করার জন্য ইন্ডেক্স ব্যবহার করতে পারেন। tuples এর বিপরীতে, tuple structs এর জন্য যখন আপনি সেগুলিকে destructure করেন তখন struct এর প্রকারের নাম দিতে হয়। উদাহরণস্বরূপ, আমরা লিখব `let Point(x, y, z) = point`।

### কোনো ফিল্ড ছাড়া Unit-Like Structs

আপনি এমন structs ও সংজ্ঞায়িত করতে পারেন যেগুলির কোনো ফিল্ড নেই! এদের _unit-like structs_ বলা হয় কারণ তারা `()` এর মতো আচরণ করে, unit type যা আমরা [“The Tuple Type”][tuples]<!-- ignore --> বিভাগে উল্লেখ করেছি। Unit-like structs কার্যকর হতে পারে যখন আপনাকে কোনো টাইপের উপর একটি trait প্রয়োগ করতে হয় কিন্তু টাইপটিতে স্টোর করতে চান এমন কোনো ডেটা আপনার কাছে থাকে না। আমরা Chapter 10 এ traits নিয়ে আলোচনা করব। এখানে `AlwaysEqual` নামের একটি unit struct ঘোষণা এবং ইনস্ট্যান্ট করার একটি উদাহরণ দেওয়া হল:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

`AlwaysEqual` সংজ্ঞায়িত করতে, আমরা `struct` কীওয়ার্ড, আমরা যে নামটি চাই, এবং তারপরে একটি সেমিকোলন ব্যবহার করি। কার্লি ব্র্যাকেট বা বন্ধনীর কোনো প্রয়োজন নেই! তারপরে আমরা `subject` ভেরিয়েবলে `AlwaysEqual` এর একটি instance পেতে পারি: কোনো কার্লি ব্র্যাকেট বা বন্ধনী ছাড়াই, আমরা যে নামটি সংজ্ঞায়িত করেছি সেটি ব্যবহার করে। কল্পনা করুন যে পরে আমরা এই টাইপের জন্য এমন আচরণ প্রয়োগ করব যাতে `AlwaysEqual` এর প্রতিটি instance অন্য যেকোনো প্রকারের প্রতিটি instance এর সাথে সর্বদা সমান হয়, সম্ভবত পরীক্ষার উদ্দেশ্যে একটি পরিচিত ফলাফল পাওয়ার জন্য। সেই আচরণ প্রয়োগ করার জন্য আমাদের কোনো ডেটার প্রয়োজন হবে না! আপনি Chapter 10 এ traits সংজ্ঞায়িত করতে এবং unit-like structs সহ যেকোনো টাইপে প্রয়োগ করতে শিখবেন।

> ### Struct ডেটার Ownership
>
> Listing 5-1 এ `User` struct সংজ্ঞায়, আমরা `&str` string slice টাইপের পরিবর্তে owned `String` টাইপ ব্যবহার করেছি। এটি একটি ইচ্ছাকৃত পছন্দ কারণ আমরা চাই এই struct এর প্রতিটি instance যেন এর সমস্ত ডেটার মালিক হয় এবং সেই ডেটা ততক্ষণ পর্যন্ত বৈধ থাকে যতক্ষণ না পুরো struct টি বৈধ থাকে।
>
> structs এর জন্য অন্য কিছু দ্বারা মালিকানাধীন ডেটার reference স্টোর করাও সম্ভব, তবে এর জন্য _lifetimes_ ব্যবহার করতে হবে, এটি Rust এর একটি বৈশিষ্ট্য যা আমরা Chapter 10 এ আলোচনা করব। Lifetimes নিশ্চিত করে যে একটি struct দ্বারা reference করা ডেটা struct টি যতক্ষণ থাকে ততক্ষণ পর্যন্ত বৈধ থাকে। ধরুন আপনি lifetimes নির্দিষ্ট না করে একটি struct এ একটি reference স্টোর করার চেষ্টা করেন, যেমন নিম্নলিখিত; এটি কাজ করবে না:
>
> <Listing file-name="src/main.rs">
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> </Listing>
>
> কম্পাইলার অভিযোগ করবে যে এটির lifetime স্পেসিফায়ার দরকার:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> Chapter 10 এ, আমরা আলোচনা করব কিভাবে এই ত্রুটিগুলি ঠিক করতে হয় যাতে আপনি structs এ reference স্টোর করতে পারেন, তবে আপাতত, আমরা `&str` এর মতো reference এর পরিবর্তে `String` এর মতো owned type ব্যবহার করে এই ধরনের ত্রুটিগুলি ঠিক করব।

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
