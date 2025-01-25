## Advanced Types

Rust type system এ কিছু feature আছে যা আমরা এখন পর্যন্ত mention করেছি কিন্তু আলোচনা করিনি। আমরা কেন newtype type হিসেবে useful তা examinig করার সময় general ভাবে newtype নিয়ে discuss করে শুরু করব। তারপর আমরা type alias এ move করব, যা newtype এর similar একটি feature কিন্তু একটু different semantics এর সাথে। আমরা `!` type এবং dynamically sized type নিয়েও discuss করব।

### Using the Newtype Pattern for Type Safety and Abstraction

> Note: এই section এ assume করা হয়েছে যে আপনি আগের section [“Using the
> Newtype Pattern to Implement External Traits on External
> Types.”][using-the-newtype-pattern]<!-- ignore --> পড়েছেন।

Newtype pattern এমন task এর জন্য useful যা আমরা এখন পর্যন্ত আলোচনা করেছি তার বাইরেও, including statically enforcing এটা নিশ্চিত করা যে value গুলো কখনো confused হবে না এবং value এর unit indicate করে। Listing 20-16 এ unit indicate করার জন্য newtype ব্যবহারের একটি উদাহরণ দেখেছিলেন: মনে করুন `Millimeters` এবং `Meters` struct newtype এ `u32` value wrap করেছে। যদি আমরা `Millimeters` type এর parameter দিয়ে একটি function লিখি, তাহলে আমরা এমন program compile করতে পারতাম না যা accidentally `Meters` type এর value বা plain `u32` value দিয়ে সেই function call করার চেষ্টা করে।

আমরা newtype pattern ব্যবহার করে একটি type এর কিছু implementation detail abstract করতে পারি: new type টি একটি public API expose করতে পারে যা private inner type এর API থেকে different।

Newtype internal implementation ও hide করতে পারে। উদাহরণস্বরূপ, আমরা একটি `People` type provide করতে পারি যা `HashMap<i32, String>` wrap করে যেখানে person এর ID তাদের name এর সাথে associated store করা হয়। `People` ব্যবহার করা code শুধুমাত্র আমরা provide করা public API এর সাথে interact করবে, যেমন `People` collection এ একটি name string add করার জন্য একটি method; সেই code এর এটা জানার প্রয়োজন নেই যে আমরা internally name এ `i32` ID assign করি। Newtype pattern হল encapsulation achieve করার একটি lightweight উপায় implementation detail hide করার জন্য, যা আমরা Chapter 18 এর [“Encapsulation that Hides Implementation Details”][encapsulation-that-hides-implementation-details]<!-- ignore --> section এ discuss করেছিলাম।

### Creating Type Synonyms with Type Aliases

Rust একটি existing type কে অন্য name দেওয়ার জন্য _type alias_ declare করার ability provide করে। এর জন্য আমরা `type` keyword ব্যবহার করি। উদাহরণস্বরূপ, আমরা `i32` এর alias `Kilometers` create করতে পারি, যেমন:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

এখন, alias `Kilometers` হলো `i32` এর একটি _synonym_; Listing 20-16 এ আমরা যে `Millimeters` এবং `Meters` type তৈরি করেছিলাম তার বিপরীতে, `Kilometers` কোনো separate, new type নয়। যে value গুলোর type `Kilometers` সেই value গুলো `i32` type এর value এর মতোই treat করা হবে:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

যেহেতু `Kilometers` এবং `i32` same type, তাই আমরা দুটি type এর value add করতে পারি এবং `i32` parameter নেওয়া function এ `Kilometers` value pass করতে পারি। তবে, এই method ব্যবহার করে, আমরা আগের discuss করা newtype pattern থেকে type check করার যে benefit পাই তা পাই না। অন্যভাবে বলতে গেলে, যদি আমরা কোথাও `Kilometers` এবং `i32` value mix up করি, তাহলে compiler কোনো error দেবে না।

Type synonym এর main use case হলো repetition reduce করা। উদাহরণস্বরূপ, আমাদের এমন একটি lengthy type থাকতে পারে:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Function signature এ এবং type annotation হিসেবে এই lengthy type সব জায়গায় লেখা ক্লান্তিকর এবং error prone হতে পারে। Listing 20-25 এর মতো code এ full project থাকার imagine করুন।

<Listing number="20-25" caption="অনেক জায়গায় একটি long type ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

Type alias repetition reduce করে এই code কে আরও manageable করে তোলে। Listing 20-26 এ, আমরা verbose type এর জন্য `Thunk` নামের একটি alias introduce করেছি এবং type এর সব ব্যবহার shorter alias `Thunk` দিয়ে replace করতে পারি।

<Listing number="20-26" caption="Repetition reduce করার জন্য `Thunk` নামের একটি type alias introduce করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

এই code পড়া এবং লেখা অনেক বেশি সহজ! একটি type alias এর জন্য meaningful name select করা আপনার intent communicate করতে help করতে পারে (_thunk_ হলো পরে evaluate করা code এর জন্য একটি word, তাই এটা closure এর জন্য একটি appropriate name যা store করা হয়)।

Type alias ও commonly `Result<T, E>` type এর সাথে repetition reduce করার জন্য ব্যবহার করা হয়। Standard library তে `std::io` module consider করুন। I/O operation প্রায়ই `Result<T, E>` return করে যখন operation কাজ করতে fail হয় এমন situation handle করার জন্য। এই library তে একটি `std::io::Error` struct আছে যা সব possible I/O error represent করে। `std::io` এর অনেক function `Result<T, E>` return করবে যেখানে `E` হলো `std::io::Error`, যেমন `Write` trait এর এই function গুলো:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

`Result<..., Error>` অনেকবার repeat হয়েছে। তাই, `std::io` তে এই type alias declaration আছে:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

যেহেতু এই declaration `std::io` module এ আছে, তাই আমরা fully qualified alias `std::io::Result<T>` ব্যবহার করতে পারি; মানে, `Result<T, E>` যেখানে `E` filled in করা হয়েছে `std::io::Error` দিয়ে। `Write` trait function signature দেখতে এমন হয়:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

Type alias দুটি উপায়ে help করে: এটা code কে লিখতে _এবং_ `std::io` জুড়ে একটি consistent interface দেয়। যেহেতু এটি একটি alias, তাই এটি শুধুমাত্র অন্য একটি `Result<T, E>`, যার মানে আমরা যেকোনো method ব্যবহার করতে পারি যা `Result<T, E>` এ কাজ করে, সেই সাথে `?` operator এর মতো special syntax ও ব্যবহার করতে পারি।

### The Never Type that Never Returns

Rust এ `!` নামে একটি special type আছে যা type theory lingo তে _empty type_ নামে পরিচিত কারণ এর কোনো value নেই। আমরা এটাকে _never type_ call করতে prefer করি কারণ যখন একটি function কখনো return করবে না তখন এটি return type এর জায়গায় stand in করে। এখানে একটি উদাহরণ দেওয়া হলো:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

এই code কে "function `bar` never return করবে" হিসেবে read করা হয়। যে function গুলো never return করে সেগুলোকে _diverging function_ বলা হয়। আমরা `!` type এর value তৈরি করতে পারি না তাই `bar` কখনো return করতে পারে না।

কিন্তু এমন type এর use কি যার জন্য আপনি কখনো value তৈরি করতে পারবেন না? Chapter 2 এর Listing 2-5 এর code মনে করুন, number guessing game এর একটি অংশ; আমরা এর কিছুটা এখানে Listing 20-27 এ reproduce করেছি।

<Listing number="20-27" caption="একটি `match` যার একটি arm এ `continue` দিয়ে শেষ হয়">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

</Listing>

সেই সময়ে, আমরা এই code এর কিছু detail skip করেছিলাম। Chapter 6 এ [“The `match` Control Flow Operator”][the-match-control-flow-operator]<!-- ignore --> section এ, আমরা discuss করেছিলাম যে `match` arm গুলোকে same type return করতে হবে। তাই, উদাহরণস্বরূপ, নিচের code টি কাজ করে না:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

এই code এ `guess` এর type integer _এবং_ string দুটোই হতে হবে, এবং Rust require করে `guess` এর শুধুমাত্র একটি type থাকতে হবে। তাহলে `continue` কি return করে? কিভাবে আমরা একটি arm থেকে `u32` return করার allow পেয়েছিলাম এবং Listing 20-27 এ অন্য একটি arm `continue` দিয়ে শেষ করতে পেরেছিলাম?

আপনি হয়তো guess করেছেন, `continue` এর একটি `!` value আছে। মানে, যখন Rust `guess` এর type compute করে, তখন এটি দুটো match arm দেখে, আগেরটিতে `u32` value আছে এবং পরেরটিতে `!` value আছে। যেহেতু `!` এর কখনো কোনো value থাকতে পারে না, তাই Rust decide করে যে `guess` এর type `u32` হবে।

এই behaviour describe করার formal way হলো `!` type এর expression গুলো অন্য যেকোনো type এ coerce হতে পারে। আমাদের এই `match` arm `continue` দিয়ে শেষ করার allow করা হয় কারণ `continue` কোনো value return করে না; এর পরিবর্তে, এটি control back loop এর শুরুতে move করে, তাই `Err` case এ, আমরা কখনো `guess` এ value assign করি না।

Never type `panic!` macro এর সাথেও useful। মনে করুন `unwrap` function যা আমরা `Option<T>` value এর উপর call করি value produce করার জন্য বা এই definition এর সাথে panic করার জন্য:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

এই code এ, Listing 20-27 এর `match` এ যা হয় এখানেও same জিনিস হয়: Rust দেখে যে `val` এর type `T` এবং `panic!` এর type `!` , তাই overall `match` expression এর result হলো `T`। এই code কাজ করে কারণ `panic!` কোনো value produce করে না; এটি program end করে দেয়। `None` case এ, আমরা `unwrap` থেকে value return করব না, তাই এই code valid।

শেষ একটি expression যার type `!` তা হলো একটি `loop`:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

এখানে, loop কখনো শেষ হয় না, তাই `!` হলো expression এর value। তবে, যদি আমরা একটি `break` include করতাম, তাহলে এটা সত্য হতো না, কারণ `break` পেলে loop terminate হতো।

### Dynamically Sized Types and the `Sized` Trait

Rust কে তার type গুলো সম্পর্কে কিছু details জানার প্রয়োজন, যেমন কোনো particular type এর value এর জন্য কত space allocate করতে হবে। এটা তার type system এর একটি corner কে প্রথমে একটু confusing করে দেয়: _dynamically sized type_ এর concept। মাঝে মাঝে _DSTs_ বা _unsized type_ হিসেবে refer করা হয়, এই type গুলো আমাদের এমন value ব্যবহার করে code লেখার allow করে যার size আমরা শুধু runtime এই জানতে পারি।

চলুন একটি dynamically sized type এর detail dig in করি যাকে `str` বলা হয়, যা আমরা পুরো বইয়ে ব্যবহার করেছি। ঠিক, `&str` নয়, `str` নিজে একটি DST। Runtime পর্যন্ত আমরা string এর length জানতে পারি না, মানে আমরা `str` type এর variable তৈরি করতে পারি না, বা `str` type এর argument ও নিতে পারি না। নিচের code consider করুন, যা কাজ করে না:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust এর যেকোনো particular type এর value এর জন্য কত memory allocate করতে হবে তা জানার প্রয়োজন, এবং একটি type এর সব value একই amount memory ব্যবহার করতে হবে। যদি Rust আমাদের এই code লেখার allow করত, তাহলে এই দুটি `str` value কে same amount space নিতে হতো। কিন্তু তাদের length different: `s1` এর 12 byte storage এর প্রয়োজন এবং `s2` এর 15 byte এর প্রয়োজন। এই কারণে dynamically sized type hold করে এমন variable তৈরি করা possible নয়।

তাহলে আমরা কি করব? এই ক্ষেত্রে, আপনি ইতিমধ্যে উত্তর জানেন: আমরা `s1` এবং `s2` এর type কে `str` এর পরিবর্তে `&str` বানাই। Chapter 4 এর [“String Slices”][string-slices]<!-- ignore --> section থেকে মনে করুন slice data structure শুধুমাত্র slice এর starting position এবং length store করে। তাই যদিও একটি `&T` হলো একটি single value যা store করে যেখানে `T` located তার memory address, একটি `&str` হলো _দুটি_ value: `str` এর address এবং এর length। তাই, আমরা compile time এ `&str` value এর size জানতে পারি: এটা `usize` এর double length। মানে, `&str` যে string refer করে তা যতই long হোক না কেন, আমরা সবসময় `&str` এর size জানি। In general, dynamically sized type Rust এ এভাবেই ব্যবহার করা হয়: তাদের dynamic information এর size store করে এমন metadata এর একটি extra bit থাকে। Dynamically sized type এর golden rule হলো আমাদের dynamically sized type এর value সবসময় কোনো pointer এর পিছনে রাখতে হবে।

আমরা `str` কে সব ধরনের pointer এর সাথে combine করতে পারি: উদাহরণস্বরূপ, `Box<str>` বা `Rc<str>`। আসলে, আপনি এটা আগে দেখেছেন কিন্তু ভিন্ন dynamically sized type এর সাথে: trait। প্রত্যেক trait একটি dynamically sized type যাকে আমরা trait এর name ব্যবহার করে refer করতে পারি। Chapter 18 এর [“Using Trait Objects That Allow for Values of Different Types”][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore --> section এ, আমরা mention করেছিলাম যে trait কে trait object হিসেবে ব্যবহার করার জন্য, আমাদের সেগুলোকে pointer এর পিছনে রাখতে হবে, যেমন `&dyn Trait` বা `Box<dyn Trait>` (`Rc<dyn Trait>` ও কাজ করত)।

DSTs এর সাথে কাজ করার জন্য, Rust একটি `Sized` trait provide করে এটা determine করার জন্য যে কোনো type এর size compile time এ জানা যায় কিনা। এই trait automatically implement করা হয় এমন সবকিছুর জন্য যার size compile time এ জানা যায়। এছাড়াও, Rust implicitly প্রত্যেক generic function এ `Sized` এর উপর একটি bound add করে। মানে, এইরকম একটি generic function definition:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

আসলে এমনভাবে treat করা হয় যেন আমরা এটা লিখেছি:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

Default হিসেবে, generic function শুধুমাত্র এমন type এ কাজ করবে যেগুলোর compile time এ size জানা যায়। তবে, আপনি এই restriction relax করার জন্য নিচের special syntax ব্যবহার করতে পারেন:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

`?Sized` এর উপর trait bound এর মানে হলো “`T` `Sized` হতে পারে বা নাও হতে পারে” এবং এই notation default override করে যে generic type এর compile time এ known size থাকতে হবে। এই meaning এর সাথে `?Trait` syntax শুধুমাত্র `Sized` এর জন্য available, অন্য কোনো trait এর জন্য নয়।

আরও note করুন যে আমরা `t` parameter এর type কে `T` থেকে `&T` তে switch করেছি। যেহেতু type টি `Sized` নাও হতে পারে, তাই আমাদের এটিকে কোনো pointer এর পিছনে ব্যবহার করার প্রয়োজন। এই ক্ষেত্রে, আমরা একটি reference choose করেছি।

এরপর, আমরা function এবং closure নিয়ে আলোচনা করব!

[encapsulation-that-hides-implementation-details]: ch18-01-what-is-oo.html#encapsulation-that-hides-implementation-details
[string-slices]: ch04-03-slices.html#string-slices
[the-match-control-flow-operator]: ch06-02-match.html#the-match-control-flow-operator
[using-trait-objects-that-allow-for-values-of-different-types]: ch18-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[using-the-newtype-pattern]: ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
