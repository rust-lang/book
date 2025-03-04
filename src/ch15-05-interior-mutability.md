## `RefCell<T>` এবং ইন্টেরিয়র মিউটেবিলিটি প্যাটার্ন

_ইন্টেরিয়র মিউটেবিলিটি_ হল Rust-এর একটি ডিজাইন প্যাটার্ন যা আপনাকে ডেটা মিউটেট করার অনুমতি দেয়, এমনকি যখন সেই ডেটার ইমিউটেবল রেফারেন্স থাকে; সাধারণত, এই কাজটি borrowing rule দ্বারা অনুমোদিত নয়। ডেটা মিউটেট করার জন্য, প্যাটার্নটি ডেটা স্ট্রাকচারের ভিতরে `unsafe` কোড ব্যবহার করে Rust-এর সাধারণ নিয়মগুলোকে বাঁকিয়ে দেয় যা মিউটেশন এবং borrowing নিয়ন্ত্রণ করে। Unsafe কোড কম্পাইলারকে নির্দেশ করে যে আমরা ম্যানুয়ালি নিয়মগুলো পরীক্ষা করছি কম্পাইলারের উপর নির্ভর না করে; আমরা Chapter 20-এ unsafe কোড নিয়ে আরও আলোচনা করব।

আমরা ইন্টেরিয়র মিউটেবিলিটি প্যাটার্ন ব্যবহার করে এমন type গুলো তখনই ব্যবহার করতে পারি যখন আমরা নিশ্চিত করতে পারি যে borrowing rule গুলো runtime-এ follow করা হবে, যদিও কম্পাইলার সেটি guarantee করতে পারে না। জড়িত `unsafe` কোডটি তখন একটি safe API-তে wrap করা হয় এবং বাইরের type টি এখনও immutable থাকে।

আসুন `RefCell<T>` টাইপটি দেখে এই concept টি explore করি যা ইন্টেরিয়র মিউটেবিলিটি প্যাটার্ন follow করে।

### `RefCell<T>` দিয়ে Runtime-এ Borrowing Rule গুলো Enforce করা

`Rc<T>`-এর বিপরীতে, `RefCell<T>` টাইপটি এটি ধারণ করা ডেটার উপর single ownership represent করে। তাহলে, `RefCell<T>`-কে `Box<T>`-এর মতো টাইপ থেকে কী আলাদা করে? Chapter 4-এ শেখা borrowing rule গুলো স্মরণ করুন:

-   যেকোনো সময়ে, আপনার কাছে _হয়_ (কিন্তু দুটোই নয়) একটি mutable reference অথবা যেকোনো সংখ্যক immutable reference থাকতে পারে।
-   Reference গুলো সব সময় valid হতে হবে।

Reference এবং `Box<T>`-এর ক্ষেত্রে, borrowing rule-গুলোর invariant গুলো compile time-এ enforce করা হয়। `RefCell<T>`-এর ক্ষেত্রে, এই invariant গুলো _runtime-এ_ enforce করা হয়। Reference-এর ক্ষেত্রে, আপনি যদি এই নিয়মগুলো ভাঙেন, তাহলে আপনি একটি compiler error পাবেন। `RefCell<T>`-এর ক্ষেত্রে, আপনি যদি এই নিয়মগুলো ভাঙেন, তাহলে আপনার প্রোগ্রাম panic করবে এবং exit করবে।

Compile time-এ borrowing rule গুলো check করার সুবিধা হল development process-এ error গুলো আরও তাড়াতাড়ি ধরা পড়বে এবং runtime performance-এর উপর কোনো প্রভাব পড়বে না কারণ সমস্ত analysis আগেই সম্পন্ন করা হয়েছে। সেই কারণে, বেশিরভাগ ক্ষেত্রে compile time-এ borrowing rule গুলো check করা হল সেরা পছন্দ, যে কারণে এটি Rust-এর default।

পরিবর্তে runtime-এ borrowing rule গুলো check করার সুবিধা হল যে certain memory-safe scenario গুলোর অনুমতি দেওয়া হয়, যেখানে সেগুলো compile-time check দ্বারা অনুমোদিত হত না। Static analysis, যেমন Rust compiler, সহজাতভাবে রক্ষণশীল। কোডের কিছু বৈশিষ্ট্য কোড analyze করে detect করা অসম্ভব: সবচেয়ে বিখ্যাত উদাহরণ হল Halting Problem, যা এই বইয়ের সুযোগের বাইরে কিন্তু research করার জন্য একটি interesting বিষয়।

যেহেতু কিছু analysis অসম্ভব, তাই যদি Rust compiler নিশ্চিত না হতে পারে যে কোডটি ownership rule গুলো মেনে চলছে, তাহলে এটি একটি সঠিক প্রোগ্রামকে reject করতে পারে; এইভাবে, এটি রক্ষণশীল। যদি Rust একটি incorrect প্রোগ্রাম accept করত, তাহলে user-রা Rust যে guarantee গুলো দেয় তাতে বিশ্বাস রাখতে পারত না। যাইহোক, যদি Rust একটি সঠিক প্রোগ্রাম reject করে, তাহলে প্রোগ্রামার অসুবিধার সম্মুখীন হবেন, কিন্তু কোনো catastrophic ঘটনা ঘটতে পারে না। `RefCell<T>` টাইপটি useful যখন আপনি নিশ্চিত যে আপনার কোড borrowing rule গুলো follow করে কিন্তু compiler সেটি বুঝতে এবং guarantee করতে অক্ষম।

`Rc<T>`-এর মতোই, `RefCell<T>` শুধুমাত্র single-threaded scenario-তে ব্যবহারের জন্য এবং আপনি যদি এটিকে multithreaded context-এ ব্যবহার করার চেষ্টা করেন তাহলে আপনাকে একটি compile-time error দেবে। আমরা Chapter 16-এ আলোচনা করব কিভাবে একটি multithreaded প্রোগ্রামে `RefCell<T>`-এর কার্যকারিতা পাওয়া যায়।

`Box<T>`, `Rc<T>`, বা `RefCell<T>` বেছে নেওয়ার কারণগুলোর একটি সংক্ষেপ এখানে দেওয়া হল:

-   `Rc<T>` একই ডেটার multiple owner-এর অনুমতি দেয়; `Box<T>` এবং `RefCell<T>`-এর single owner রয়েছে।
-   `Box<T>` compile time-এ check করা immutable বা mutable borrow-এর অনুমতি দেয়; `Rc<T>` শুধুমাত্র compile time-এ check করা immutable borrow-এর অনুমতি দেয়; `RefCell<T>` runtime-এ check করা immutable বা mutable borrow-এর অনুমতি দেয়।
-   যেহেতু `RefCell<T>` runtime-এ check করা mutable borrow-এর অনুমতি দেয়, তাই আপনি `RefCell<T>`-এর ভেতরের value-কে mutate করতে পারেন যদিও `RefCell<T>` immutable হয়।

একটি immutable value-এর ভেতরের value-কে mutate করা হল _ইন্টেরিয়র মিউটেবিলিটি_ প্যাটার্ন। আসুন এমন একটি পরিস্থিতি দেখি যেখানে ইন্টেরিয়র মিউটেবিলিটি useful এবং পরীক্ষা করি কীভাবে এটি সম্ভব।

### ইন্টেরিয়র মিউটেবিলিটির জন্য একটি Use Case: Mock অবজেক্ট

কখনও কখনও testing-এর সময় একজন প্রোগ্রামার অন্য type-এর জায়গায় একটি type ব্যবহার করেন, particular behavior পর্যবেক্ষণ করতে এবং assert করতে যে এটি সঠিকভাবে implement করা হয়েছে। এই placeholder type-টিকে _test double_ বলা হয়। এটিকে ফিল্মমেকিং-এ "স্টান্ট ডাবল"-এর মতো ভাবুন, যেখানে একজন ব্যক্তি একটি particular tricky scene করার জন্য একজন অভিনেতার পরিবর্তে আসেন এবং substitute করেন। Test double গুলো অন্য type-এর জন্য দাঁড়ায় যখন আমরা test চালাই। _Mock object_ হল test double-এর specific type যা একটি test চলাকালীন কী ঘটে তা record করে যাতে আপনি assert করতে পারেন যে সঠিক action গুলো ঘটেছে।

Rust-এ অন্যান্য language-এর মতো একই অর্থে অবজেক্ট নেই এবং Rust-এর standard library-তে অন্য কিছু language-এর মতো mock object functionality বিল্ট-ইন নেই। যাইহোক, আপনি निश्चितভাবে একটি struct তৈরি করতে পারেন যা একটি mock object-এর মতোই একই উদ্দেশ্যে কাজ করবে।

এখানে সেই scenario টি রয়েছে যা আমরা test করব: আমরা একটি লাইব্রেরি তৈরি করব যা একটি maximum value-এর বিপরীতে একটি value ট্র্যাক করে এবং current value টি maximum value-এর কতটা কাছাকাছি তার উপর ভিত্তি করে message পাঠায়। এই লাইব্রেরিটি ব্যবহার করা যেতে পারে একজন user-এর API call-এর সংখ্যার কোটা ট্র্যাক রাখতে, উদাহরণস্বরূপ।

আমাদের লাইব্রেরি শুধুমাত্র একটি value maximum-এর কতটা কাছাকাছি এবং কোন সময়ে কী message হওয়া উচিত তা ট্র্যাক করার কার্যকারিতা provide করবে। যে অ্যাপ্লিকেশনগুলো আমাদের লাইব্রেরি ব্যবহার করে সেগুলো message পাঠানোর mechanism provide করবে বলে আশা করা হচ্ছে: অ্যাপ্লিকেশনটি অ্যাপ্লিকেশনে একটি message রাখতে পারে, একটি email পাঠাতে পারে, একটি text message পাঠাতে পারে বা অন্য কিছু করতে পারে। লাইব্রেরির সেই detail জানার প্রয়োজন নেই। এটির যা প্রয়োজন তা হল এমন কিছু যা আমরা provide করব এমন একটি trait implement করে, যার নাম `Messenger`। Listing 15-20 লাইব্রেরির কোড দেখায়:

<Listing number="15-20" file-name="src/lib.rs" caption="একটি value maximum value-এর কতটা কাছাকাছি তা ট্র্যাক রাখতে এবং value নির্দিষ্ট স্তরে থাকলে সতর্ক করার জন্য একটি লাইব্রেরি">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

</Listing>

এই কোডের একটি গুরুত্বপূর্ণ অংশ হল `Messenger` trait-এ `send` নামক একটি method রয়েছে যা `self`-এর একটি immutable reference এবং message-এর text নেয়। এই trait টি হল সেই ইন্টারফেস যা আমাদের মক অবজেক্টকে implement করতে হবে যাতে মকটিকে একটি real object-এর মতোই ব্যবহার করা যায়। আরেকটি গুরুত্বপূর্ণ অংশ হল যে আমরা `LimitTracker`-এ `set_value` method-এর behavior test করতে চাই। আমরা `value` parameter-এর জন্য যা pass করি তা পরিবর্তন করতে পারি, কিন্তু `set_value` আমাদের জন্য কোনো কিছু return করে না যার উপর আমরা assertion করতে পারি। আমরা বলতে চাই যে যদি আমরা `Messenger` trait implement করে এমন কিছু এবং `max`-এর জন্য একটি particular value দিয়ে একটি `LimitTracker` তৈরি করি, যখন আমরা `value`-এর জন্য different number pass করি, তখন messenger-কে উপযুক্ত message গুলো পাঠাতে বলা হয়।

আমাদের এমন একটি মক অবজেক্ট দরকার যা, যখন আমরা `send` কল করি, তখন একটি email বা text message পাঠানোর পরিবর্তে, শুধুমাত্র সেই message গুলো ট্র্যাক রাখবে যেগুলো পাঠানোর কথা। আমরা মক অবজেক্টের একটি new instance তৈরি করতে পারি, মক অবজেক্ট ব্যবহার করে একটি `LimitTracker` তৈরি করতে পারি, `LimitTracker`-এ `set_value` method কল করতে পারি এবং তারপর চেক করতে পারি যে মক অবজেক্টে আমাদের প্রত্যাশিত message গুলো আছে কিনা। Listing 15-21 ঠিক এটি করার জন্য একটি মক অবজেক্ট implement করার একটি প্রচেষ্টা দেখায়, কিন্তু borrow checker এটির অনুমতি দেবে না:

<Listing number="15-21" file-name="src/lib.rs" caption="একটি `MockMessenger` implement করার একটি প্রচেষ্টা যা borrow checker দ্বারা অনুমোদিত নয়">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

</Listing>

এই test code-টি একটি `MockMessenger` struct define করে যাতে `sent_messages` field রয়েছে message গুলো ট্র্যাক রাখার জন্য `String` value-গুলোর একটি `Vec` সহ। আমরা একটি associated function `new` ও define করি যাতে new `MockMessenger` value তৈরি করা সুবিধাজনক হয় যেগুলো message-এর একটি empty list দিয়ে শুরু হয়। তারপর আমরা `MockMessenger`-এর জন্য `Messenger` trait implement করি যাতে আমরা একটি `LimitTracker`-কে একটি `MockMessenger` দিতে পারি। `Send` method-এর definition-এ, আমরা parameter হিসেবে pass করা message টি নিই এবং এটিকে `MockMessenger`-এর `sent_messages`-এর list-এ store করি।

Test-এ, আমরা test করছি যখন `LimitTracker`-কে `value` set করতে বলা হয় এমন কিছুতে যা `max` value-এর 75 শতাংশের বেশি। প্রথমে, আমরা একটি new `MockMessenger` তৈরি করি, যেটি message-এর একটি empty list দিয়ে শুরু হবে। তারপর আমরা একটি new `LimitTracker` তৈরি করি এবং এটিকে new `MockMessenger`-এর একটি reference এবং 100-এর একটি `max` value দিই। আমরা `LimitTracker`-এ `set_value` method-টিকে 80 value দিয়ে কল করি, যেটি 100-এর 75 শতাংশের বেশি। তারপর আমরা assert করি যে `MockMessenger` যে message গুলোর ট্র্যাক রাখছে তার list-এ এখন একটি message থাকা উচিত।

যাইহোক, এই test-এর একটি সমস্যা আছে, যেমনটি এখানে দেখানো হয়েছে:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

আমরা message-গুলোর ট্র্যাক রাখার জন্য `MockMessenger`-কে modify করতে পারি না, কারণ `send` method টি `self`-এর একটি immutable reference নেয়। আমরা error text থেকে `&mut self` ব্যবহার করার পরামর্শও নিতে পারি না `impl` method এবং `trait` definition-এ। আমরা শুধুমাত্র testing-এর স্বার্থে `Messenger` trait পরিবর্তন করতে চাই না। পরিবর্তে, আমাদের বিদ্যমান ডিজাইনের সাথে আমাদের test code-কে সঠিকভাবে কাজ করার একটি উপায় খুঁজে বের করতে হবে।

এটি এমন একটি পরিস্থিতি যেখানে ইন্টেরিয়র মিউটেবিলিটি সাহায্য করতে পারে! আমরা `sent_messages`-কে একটি `RefCell<T>`-এর মধ্যে store করব এবং তারপর `send` method টি `sent_messages` modify করে আমরা যে message গুলো দেখেছি সেগুলো store করতে সক্ষম হবে। Listing 15-22 দেখায় যে এটি দেখতে কেমন:

<Listing number="15-22" file-name="src/lib.rs" caption="বাইরের value-কে immutable বিবেচনা করা হলেও ভেতরের value mutate করতে `RefCell<T>` ব্যবহার করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

</Listing>

`Sent_messages` field-টি এখন `Vec<String>`-এর পরিবর্তে `RefCell<Vec<String>>` type-এর। `New` ফাংশনে, আমরা empty vector-এর চারপাশে একটি new `RefCell<Vec<String>>` instance তৈরি করি।

`Send` method-এর implementation-এর জন্য, প্রথম parameter টি এখনও `self`-এর একটি immutable borrow, যেটি trait definition-এর সাথে মেলে। আমরা `self.sent_messages`-এ `RefCell<Vec<String>>`-এ `borrow_mut` কল করি `RefCell<Vec<String>>`-এর ভেতরের value-টির একটি mutable reference পেতে, যেটি হল vector। তারপর আমরা test চলাকালীন পাঠানো message গুলোর ট্র্যাক রাখতে vector-এ mutable reference-এ `push` কল করতে পারি।

আমাদের শেষ যে পরিবর্তনটি করতে হবে তা হল assertion-এ: ভেতরের vector-এ কতগুলো item আছে তা দেখতে, আমরা vector-টির একটি immutable reference পেতে `RefCell<Vec<String>>`-এ `borrow` কল করি।

এখন আপনি দেখেছেন কিভাবে `RefCell<T>` ব্যবহার করতে হয়, আসুন এটি কীভাবে কাজ করে তা দেখি!

#### `RefCell<T>`-এর সাহায্যে Runtime-এ Borrow-এর ট্র্যাক রাখা

Immutable এবং mutable reference তৈরি করার সময়, আমরা যথাক্রমে `&` এবং `&mut` syntax ব্যবহার করি। `RefCell<T>`-এর ক্ষেত্রে, আমরা `borrow` এবং `borrow_mut` method ব্যবহার করি, যেগুলো `RefCell<T>`-এর অন্তর্গত safe API-এর অংশ। `Borrow` method টি স্মার্ট পয়েন্টার টাইপ `Ref<T>` রিটার্ন করে এবং `borrow_mut` স্মার্ট পয়েন্টার টাইপ `RefMut<T>` রিটার্ন করে। উভয় type-ই `Deref` implement করে, তাই আমরা সেগুলোকে regular reference-এর মতো treat করতে পারি।

`RefCell<T>` ট্র্যাক রাখে কতগুলো `Ref<T>` এবং `RefMut<T>` স্মার্ট পয়েন্টার বর্তমানে active রয়েছে। প্রতিবার যখন আমরা `borrow` কল করি, `RefCell<T>` active থাকা immutable borrow-এর সংখ্যা বাড়িয়ে দেয়। যখন একটি `Ref<T>` value scope-এর বাইরে চলে যায়, তখন immutable borrow-এর সংখ্যা এক কমে যায়। Compile-time borrowing rule-গুলোর মতোই, `RefCell<T>` আমাদের যেকোনো সময়ে অনেকগুলো immutable borrow বা একটি mutable borrow রাখার অনুমতি দেয়।

যদি আমরা এই নিয়মগুলো লঙ্ঘন করার চেষ্টা করি, তাহলে reference-এর মতো compiler error পাওয়ার পরিবর্তে, `RefCell<T>`-এর implementation runtime-এ panic করবে। Listing 15-23, Listing 15-22-এ `send`-এর implementation-এর একটি modification দেখায়। আমরা ইচ্ছাকৃতভাবে একই scope-এর জন্য দুটি active mutable borrow তৈরি করার চেষ্টা করছি এটা বোঝানোর জন্য যে `RefCell<T>` আমাদের runtime-এ এটি করা থেকে বিরত রাখে।

<Listing number="15-23" file-name="src/lib.rs" caption="একই scope-এ দুটি mutable reference তৈরি করা এটা দেখতে যে `RefCell<T>` প্যানিক করবে">

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

</Listing>

আমরা `borrow_mut` থেকে returned `RefMut<T>` স্মার্ট পয়েন্টারের জন্য একটি variable `one_borrow` তৈরি করি। তারপর আমরা একই ভাবে variable `two_borrow`-তে আরেকটি mutable borrow তৈরি করি। এটি একই scope-এ দুটি mutable reference তৈরি করে, যার অনুমতি নেই। যখন আমরা আমাদের লাইব্রেরির জন্য test চালাই, তখন Listing 15-23-এর কোডটি কোনো error ছাড়াই compile হবে, কিন্তু test fail করবে:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

লক্ষ্য করুন যে কোডটি `already borrowed: BorrowMutError` message দিয়ে panic করেছে। এইভাবে `RefCell<T>` runtime-এ borrowing rule-গুলোর violation গুলো handle করে।

এখানে যেমনটি করেছি, compile time-এর পরিবর্তে runtime-এ borrowing error গুলো ধরার অর্থ হল আপনি development process-এ আপনার কোডের ভুলগুলো আরও পরে খুঁজে পাবেন: সম্ভবত আপনার কোড production-এ deploy না হওয়া পর্যন্ত নয়। এছাড়াও, compile time-এর পরিবর্তে runtime-এ borrow-গুলোর ট্র্যাক রাখার ফলে আপনার কোড সামান্য runtime performance penalty বহন করবে। যাইহোক, `RefCell<T>` ব্যবহার করা একটি মক অবজেক্ট লেখা সম্ভব করে যা নিজেকে modify করে সেই message গুলোর ট্র্যাক রাখতে পারে যেগুলো এটি দেখেছে যখন আপনি এটিকে এমন একটি context-এ ব্যবহার করছেন যেখানে শুধুমাত্র immutable value-গুলোর অনুমতি রয়েছে। আপনি regular reference-এর চেয়ে বেশি functionality পেতে `RefCell<T>` ব্যবহার করতে পারেন এর trade-off গুলো থাকা সত্ত্বেও।

### `Rc<T>` এবং `RefCell<T>` একত্রিত করে Mutable ডেটার Multiple Owner থাকা

`RefCell<T>` ব্যবহার করার একটি সাধারণ উপায় হল `Rc<T>`-এর সাথে। স্মরণ করুন যে `Rc<T>` আপনাকে কিছু ডেটার multiple owner রাখার অনুমতি দেয়, কিন্তু এটি শুধুমাত্র সেই ডেটাতে immutable অ্যাক্সেস দেয়। যদি আপনার কাছে একটি `Rc<T>` থাকে যা একটি `RefCell<T>` ধারণ করে, তাহলে আপনি এমন একটি value পেতে পারেন যার multiple owner থাকতে পারে _এবং_ যাকে আপনি mutate করতে পারেন!

উদাহরণস্বরূপ, Listing 15-18-এর cons list উদাহরণটি স্মরণ করুন যেখানে আমরা multiple list-কে অন্য list-এর ownership share করার অনুমতি দেওয়ার জন্য `Rc<T>` ব্যবহার করেছি। যেহেতু `Rc<T>` শুধুমাত্র immutable value ধারণ করে, তাই আমরা list-গুলো তৈরি করার পরে সেগুলোর কোনো value পরিবর্তন করতে পারি না। আসুন `RefCell<T>` যোগ করি list-গুলোর value পরিবর্তন করার ক্ষমতা অর্জন করতে। Listing 15-24 দেখায় যে `Cons` definition-এ একটি `RefCell<T>` ব্যবহার করে, আমরা সমস্ত list-এ stored value-কে modify করতে পারি:

<Listing number="15-24" file-name="src/main.rs" caption="`List` তৈরি করতে `Rc<RefCell<i32>>` ব্যবহার করা যা আমরা mutate করতে পারি">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

</Listing>

আমরা `Rc<RefCell<i32>>`-এর একটি instance-এর একটি value তৈরি করি এবং এটিকে `value` নামক একটি variable-এ store করি যাতে আমরা পরে এটিকে সরাসরি অ্যাক্সেস করতে পারি। তারপর আমরা `a`-তে একটি `List` তৈরি করি একটি `Cons` variant দিয়ে যা `value` ধারণ করে। আমাদের `value` clone করতে হবে যাতে `a` এবং `value` উভয়েরই ভেতরের `5` value-টির ownership থাকে, `value` থেকে `a`-তে ownership transfer করার পরিবর্তে বা `a`-এর `value` থেকে borrow করার পরিবর্তে।

আমরা list `a`-কে একটি `Rc<T>`-তে wrap করি যাতে আমরা যখন list `b` এবং `c` তৈরি করি, তখন তারা উভয়েই `a`-কে refer করতে পারে, যেটি আমরা Listing 15-18-এ করেছিলাম।

`A`, `b` এবং `c`-তে list গুলো তৈরি করার পরে, আমরা `value`-এর value-তে 10 যোগ করতে চাই। আমরা `value`-তে `borrow_mut` কল করে এটি করি, যেটি স্বয়ংক্রিয় ডিরেফারেন্সিং feature ব্যবহার করে যা আমরা Chapter 5-এ আলোচনা করেছি ([“`->` অপারেটরটি কোথায়?”][wheres-the---operator] বিভাগটি দেখুন) `Rc<T>`-কে ভেতরের `RefCell<T>` value-তে dereference করতে। `Borrow_mut` method টি একটি `RefMut<T>` স্মার্ট পয়েন্টার রিটার্ন করে এবং আমরা এটিতে dereference operator ব্যবহার করি এবং ভেতরের value পরিবর্তন করি।

যখন আমরা `a`, `b` এবং `c` প্রিন্ট করি, তখন আমরা দেখতে পাই যে সেগুলোর সবগুলোর modified value রয়েছে 5-এর পরিবর্তে 15:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

এই technique টি বেশ neat! `RefCell<T>` ব্যবহার করে, আমাদের কাছে একটি বাহ্যিকভাবে immutable `List` value রয়েছে। কিন্তু আমরা `RefCell<T>`-এর method গুলো ব্যবহার করতে পারি যা এটির ইন্টেরিয়র মিউটেবিলিটিতে অ্যাক্সেস provide করে যাতে আমাদের প্রয়োজনের সময় আমরা আমাদের ডেটা modify করতে পারি। Borrowing rule-গুলোর runtime check গুলো আমাদের ডেটা রেস থেকে রক্ষা করে এবং আমাদের ডেটা স্ট্রাকচারে এই নমনীয়তার জন্য কখনও কখনও কিছুটা গতি trade করা সার্থক। মনে রাখবেন যে `RefCell<T>` মাল্টিথ্রেডেড কোডের জন্য কাজ করে না! `Mutex<T>` হল `RefCell<T>`-এর থ্রেড-নিরাপদ সংস্করণ এবং আমরা Chapter 16-এ `Mutex<T>` নিয়ে আলোচনা করব।

[wheres-the---operator]: ch05-03-method-syntax.html#wheres-the---operator
