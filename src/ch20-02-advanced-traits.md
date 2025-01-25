## Advanced Traits

আমরা প্রথমে Chapter 10 এর [“Traits: Defining Shared Behavior”][traits-defining-shared-behavior]<!-- ignore --> section এ trait cover করেছিলাম, কিন্তু আমরা আরও advanced details নিয়ে আলোচনা করিনি। এখন যেহেতু আপনি Rust সম্পর্কে আরও বেশি কিছু জানেন, তাই আমরা nitty-gritty তে যেতে পারি।

### Specifying Placeholder Types in Trait Definitions with Associated Types

_Associated type_ trait এর সাথে একটি type placeholder connect করে যাতে trait method definition তাদের signature এ এই placeholder type ব্যবহার করতে পারে। Trait এর implementor particular implementation এর জন্য placeholder type এর পরিবর্তে ব্যবহার করার জন্য concrete type specify করবে। এইভাবে, আমরা কিছু type ব্যবহার করে trait define করতে পারি সেই type গুলো exactly কি তা জানার প্রয়োজন ছাড়াই যতক্ষণ না trait implement করা হচ্ছে।

আমরা এই chapter এ most advanced feature describe করেছি as being rarely needed। Associated type মাঝামাঝি কোথাও: এই বইয়ের বাকি অংশে explained feature গুলোর চেয়ে more rarely ব্যবহার করা হয় কিন্তু এই chapter এ discuss করা অন্য অনেক feature এর চেয়ে more commonly ব্যবহার করা হয়।

Associated type সহ trait এর একটি উদাহরণ হলো `Iterator` trait যা standard library provide করে। Associated type এর নাম হলো `Item` এবং `Iterator` trait implement করা type iterate করছে এমন value এর type represent করে। `Iterator` trait এর definition Listing 20-13 এ দেখানো হয়েছে।

<Listing number="20-13" caption="`Iterator` trait এর definition যার একটি associated type `Item` আছে">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-13/src/lib.rs}}
```

</Listing>

`Item` type টি একটি placeholder, এবং `next` method এর definition দেখায় যে এটি `Option<Self::Item>` type এর value return করবে। `Iterator` trait এর implementor `Item` এর concrete type specify করবে, এবং `next` method সেই concrete type এর value contain করে এমন `Option` return করবে।

Associated type generic এর মতো similar concept মনে হতে পারে, কারণ latter আমাদের function define করার allow করে যা কি type handle করতে পারে তা specify না করে। দুটি concept এর মধ্যে difference examine করার জন্য, আমরা `Counter` নামের একটি type এর উপর `Iterator` trait এর implementation দেখব যেখানে `Item` type specify করা হয়েছে `u32` হিসেবে:

<Listing file-name="src/lib.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

</Listing>

এই syntax generic এর syntax এর সাথে comparable মনে হয়। তাহলে Listing 20-14 এ দেখানো হিসাবে, generic দিয়ে `Iterator` trait define না করার কারণ কি?

<Listing number="20-14" caption="Generic ব্যবহার করে `Iterator` trait এর একটি hypothetical definition">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-14/src/lib.rs}}
```

</Listing>

Difference হলো যখন Listing 20-14 এর মতো generic ব্যবহার করা হয়, তখন আমাদের প্রত্যেক implementation এ type annotate করতে হয়; কারণ আমরা `Counter` এর জন্য `Iterator<String>` বা অন্য যেকোনো type implement করতে পারি, `Counter` এর জন্য আমাদের `Iterator` এর multiple implementation থাকতে পারত। অন্যভাবে বলতে গেলে, যখন একটি trait এ একটি generic parameter থাকে, তখন এটিকে একটি type এর জন্য multiple বার implement করা যেতে পারে, প্রতিবার generic type parameter এর concrete type change করে। যখন আমরা `Counter` এ `next` method ব্যবহার করি, তখন আমরা কোন `Iterator` এর implementation ব্যবহার করতে চাই তা indicate করার জন্য আমাদের type annotation provide করার প্রয়োজন হতো।

Associated type এর সাথে, type annotate করার প্রয়োজন নেই কারণ আমরা একটি type এর উপর multiple বার trait implement করতে পারি না। Associated type ব্যবহার করে define করা Listing 20-13 এর সাথে, আমরা শুধুমাত্র একবার `Item` এর type choose করতে পারি, কারণ `Counter` এর জন্য শুধুমাত্র একটি `impl Iterator` থাকতে পারে। `Counter` এ `next` call করি এমন সব জায়গায় আমাদের specify করার প্রয়োজন নেই যে আমরা `u32` value এর একটি iterator চাই।

Associated type ও trait contract এর অংশ হয়ে যায়: trait এর implementor কে associated type placeholder এর জন্য stand in করার জন্য একটি type provide করতে হবে। Associated type এর প্রায়ই এমন একটি name থাকে যা describe করে কিভাবে type ব্যবহার করা হবে, এবং API documentation এ associated type document করা ভালো practice।

### Default Generic Type Parameters and Operator Overloading

যখন আমরা generic type parameter ব্যবহার করি, তখন আমরা generic type এর জন্য একটি default concrete type specify করতে পারি। এটা করলে trait implementor এর concrete type specify করার প্রয়োজন eliminate হয় যদি default type কাজ করে। আপনি `<PlaceholderType=ConcreteType>` syntax দিয়ে generic type declare করার সময় একটি default type specify করেন।

এমন situation এর একটি দারুণ উদাহরণ যেখানে এই technique useful তা হলো _operator overloading_, যেখানে আপনি particular situation এ operator (যেমন `+`) এর behaviour customize করেন।

Rust আপনাকে নিজের operator তৈরি করার বা arbitrary operator overload করার allow করে না। কিন্তু আপনি operator এবং corresponding trait এর সাথে associated traits implement করে `std::ops` এ listed operation customize করতে পারেন। উদাহরণস্বরূপ, Listing 20-15 এ আমরা দুটি `Point` instance একসাথে add করার জন্য `+` operator overload করি। আমরা `Point` struct এর উপর `Add` trait implement করে এটা করি:

<Listing number="20-15" file-name="src/main.rs" caption="`Point` instance এর জন্য `+` operator overload করার জন্য `Add` trait implement করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-15/src/main.rs}}
```

</Listing>

`add` method দুটি `Point` instance এর `x` value এবং দুটি `Point` instance এর `y` value add করে একটি নতুন `Point` তৈরি করে। `Add` trait এর `Output` নামের একটি associated type আছে যা `add` method থেকে return হওয়া type determine করে।

এই code এ default generic type `Add` trait এর ভিতরে আছে। এখানে এর definition দেওয়া হলো:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

এই code টি generally familiar লাগা উচিত: একটি method এবং একটি associated type সহ একটি trait। নতুন part হলো `Rhs=Self`: এই syntax কে _default type parameter_ বলা হয়। `Rhs` generic type parameter (short for “right hand side”) `add` method এ `rhs` parameter এর type define করে। যখন আমরা `Add` trait implement করি তখন যদি `Rhs` এর জন্য concrete type specify না করি, তাহলে `Rhs` এর type default ভাবে `Self` হবে, যা হবে যে type এর উপর আমরা `Add` implement করছি।

যখন আমরা `Point` এর জন্য `Add` implement করি, তখন আমরা `Rhs` এর জন্য default ব্যবহার করেছিলাম কারণ আমরা দুটি `Point` instance add করতে চেয়েছিলাম। চলুন `Add` trait implement করার একটি উদাহরণ দেখি যেখানে আমরা default ব্যবহার করার পরিবর্তে `Rhs` type customize করতে চাই।

আমাদের কাছে দুটি struct আছে, `Millimeters` এবং `Meters`, যা different unit এ value hold করে। অন্য struct এ একটি existing type এর thin wrapping কে _newtype pattern_ বলা হয়, যা আমরা [“Using the Newtype Pattern to Implement External Traits on External Types”][newtype]<!-- ignore --> section এ বিস্তারিত discuss করেছি। আমরা millimeters এর value meter এর value এর সাথে add করতে চাই এবং চাই `Add` এর implementation conversion সঠিকভাবে করুক। আমরা `Millimeters` এর জন্য `Add` implement করতে পারি যেখানে `Meters` হলো `Rhs`, যা Listing 20-16 এ দেখানো হয়েছে।

<Listing number="20-16" file-name="src/lib.rs" caption="`Millimeters` এর উপর `Add` trait implement করে `Millimeters` কে `Meters` এ add করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-16/src/lib.rs}}
```

</Listing>

`Millimeters` এবং `Meters` add করার জন্য, আমরা `impl Add<Meters>` specify করি `Rhs` type parameter এর value set করার জন্য default `Self` ব্যবহার করার পরিবর্তে।

আপনি দুটি main way তে default type parameter ব্যবহার করবেন:

- Existing code break না করে একটি type extend করার জন্য
- Specific case এ customization allow করার জন্য যা বেশিরভাগ user এর প্রয়োজন হবে না

Standard library এর `Add` trait হলো second purpose এর একটি উদাহরণ: সাধারণত, আপনি same ধরনের দুটি type add করবেন, কিন্তু `Add` trait এর বাইরে customize করার ability provide করে। `Add` trait definition এ default type parameter ব্যবহার করার মানে হলো আপনাকে বেশিরভাগ সময় extra parameter specify করার প্রয়োজন নেই। অন্যভাবে বলতে গেলে, implementation এর কিছু boilerplate এর প্রয়োজন নেই, যা trait ব্যবহার করা সহজ করে।

First purpose second এর similar কিন্তু reverse: যদি আপনি existing trait এ একটি type parameter add করতে চান, তাহলে আপনি code implement করার существу code break না করে trait এর functionality extend করার allow করার জন্য এটিকে default দিতে পারেন।

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Rust এ কোনো কিছুই একটি trait কে অন্য trait এর method এর same name এর method রাখার থেকে prevent করে না, বা Rust আপনাকে একটি type এ দুটো trait implement করা থেকেও prevent করে না। Trait থেকে method এর same name এর সাথে type এ directly একটি method implement করাও possible।

Same name এর method call করার সময়, আপনাকে Rust কে বলতে হবে যে আপনি কোনটি ব্যবহার করতে চান। Listing 20-17 এর code consider করুন যেখানে আমরা দুটি trait define করেছি, `Pilot` এবং `Wizard`, যাদের উভয়ের `fly` নামের একটি method আছে। তারপর আমরা `Human` type এ দুটো trait implement করি, যেখানে ইতিমধ্যে `fly` নামের একটি method implement করা আছে। প্রত্যেক `fly` method ভিন্ন কিছু করে।

<Listing number="20-17" file-name="src/main.rs" caption="`Human` type এ implement করা `fly` নামের method সহ এবং দুটি trait define করা যাদের একটি `fly` method আছে এবং `Human` type এ implement করা আছে">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-17/src/main.rs:here}}
```

</Listing>

যখন আমরা `Human` এর instance এর উপর `fly` call করি, compiler default ভাবে সেই method call করে যা type এর উপর directly implemented, যেমন Listing 20-18 এ দেখানো হয়েছে।

<Listing number="20-18" file-name="src/main.rs" caption="`Human` এর instance এর উপর `fly` call করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-18/src/main.rs:here}}
```

</Listing>

এই code run করলে print হবে `*waving arms furiously*`, দেখায় যে Rust directly `Human` এ implemented `fly` method call করেছে।

`Pilot` trait বা `Wizard` trait থেকে `fly` method call করার জন্য, আমরা কোন `fly` method কে বোঝাতে চাচ্ছি তা specify করার জন্য আমাদের আরও explicit syntax ব্যবহার করার প্রয়োজন। Listing 20-19 এই syntax demonstrate করে।

<Listing number="20-19" file-name="src/main.rs" caption="কোন trait এর `fly` method আমরা call করতে চাই তা specify করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-19/src/main.rs:here}}
```

</Listing>

Method name এর আগে trait name specify করলে Rust এর কাছে clear হয়ে যায় যে আমরা `fly` এর কোন implementation call করতে চাই। আমরা `Human::fly(&person)` ও লিখতে পারতাম, যা Listing 20-19 এ ব্যবহার করা `person.fly()` এর equivalent, কিন্তু যদি disambiguate করার প্রয়োজন না হয়, তাহলে এটা লিখতে একটু বেশি long।

এই code run করলে নিচের output print হবে:

```console
{{#include ../listings/ch20-advanced-features/listing-20-19/output.txt}}
```

যেহেতু `fly` method একটি `self` parameter নেয়, তাই যদি আমাদের কাছে দুটি _type_ থাকত যা একটি _trait_ implement করত, তাহলে Rust `self` এর type এর উপর ভিত্তি করে কোন trait এর implementation ব্যবহার করতে হবে তা figure out করতে পারত।

তবে, associated function যা method নয় সেগুলোর `self` parameter নেই। যখন multiple type বা trait থাকে যা same function name এর সাথে non-method function define করে, তখন Rust সবসময় জানে না আপনি কোন type মিন করছেন যতক্ষণ না আপনি _fully qualified syntax_ ব্যবহার করছেন। উদাহরণস্বরূপ, Listing 20-20 এ আমরা একটি animal shelter এর জন্য একটি trait তৈরি করি যারা সব baby dog এর নাম `Spot` রাখতে চায়। আমরা `baby_name` নামের associated non-method function সহ `Animal` trait তৈরি করি। `Animal` trait struct `Dog` এর জন্য implement করা, যেখানে আমরা directly `baby_name` নামের একটি associated non-method function provide করি।

<Listing number="20-20" file-name="src/main.rs" caption="একটি trait যার একটি associated function আছে এবং একটি type যার একই name এর associated function আছে যা trait ও implement করে">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-20/src/main.rs}}
```

</Listing>

আমরা `Dog` এ directly define করা `baby_name` associated function এ সব puppy এর নাম Spot রাখার জন্য code implement করি। `Dog` type ও trait `Animal` implement করে, যা describe করে সব animal এর characteristic কি। Baby dog দের puppy বলা হয়, এবং সেটা `Dog` এর উপর `Animal` trait এর `baby_name` function implementation এ express করা হয়েছে।

`main` এ, আমরা `Dog::baby_name` function call করি, যা directly `Dog` এর উপর define করা associated function call করে। এই code নিচের print করে:

```console
{{#include ../listings/ch20-advanced-features/listing-20-20/output.txt}}
```

এই output টি আমরা যা চেয়েছিলাম তা নয়। আমরা `Dog` এ implement করা `Animal` trait এর `baby_name` function call করতে চাই যাতে code print করে `A baby dog is called a puppy`। Listing 20-19 এ আমরা trait name specify করার technique এখানে help করে না; যদি আমরা `main` কে Listing 20-21 এর code এ change করি, তাহলে আমরা একটি compilation error পাব।

<Listing number="20-21" file-name="src/main.rs" caption="`Animal` trait থেকে `baby_name` function call করার চেষ্টা করা, কিন্তু Rust জানে না কোন implementation ব্যবহার করতে হবে">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-21/src/main.rs:here}}
```

</Listing>

যেহেতু `Animal::baby_name` এর কোনো `self` parameter নেই, এবং এমন type থাকতে পারে যারা `Animal` trait implement করে, তাই Rust figure out করতে পারে না যে আমরা `Animal::baby_name` এর কোন implementation চাই। আমরা এই compiler error টি পাব:

```console
{{#include ../listings/ch20-advanced-features/listing-20-21/output.txt}}
```

Disambiguate করার জন্য এবং Rust কে বলার জন্য যে আমরা অন্য কোনো type এর জন্য `Animal` এর implementation এর পরিবর্তে `Dog` এর জন্য `Animal` এর implementation ব্যবহার করতে চাই, আমাদের fully qualified syntax ব্যবহার করার প্রয়োজন। Listing 20-22 দেখায় কিভাবে fully qualified syntax ব্যবহার করতে হয়।

<Listing number="20-22" file-name="src/main.rs" caption="`Dog` এ implement করা `Animal` trait থেকে `baby_name` function call করতে চাই তা specify করার জন্য fully qualified syntax ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-22/src/main.rs:here}}
```

</Listing>

আমরা angle bracket এর ভিতরে Rust কে একটি type annotation provide করছি, যা indicate করে যে আমরা `Animal` trait থেকে `baby_name` method call করতে চাই যা `Dog` এ implement করা হয়েছে, এটা বলার মাধ্যমে যে আমরা এই function call এর জন্য `Dog` type কে `Animal` হিসেবে treat করতে চাই। এই code টি এখন আমরা যা চাই তা print করবে:

```console
{{#include ../listings/ch20-advanced-features/listing-20-22/output.txt}}
```

In general, fully qualified syntax define করা হয় এভাবে:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

Associated function এর জন্য যা method নয়, সেখানে কোনো `receiver` থাকবে না: শুধুমাত্র অন্য argument এর list থাকবে। আপনি সব জায়গায় fully qualified syntax ব্যবহার করতে পারেন যেখানে আপনি function বা method call করেন। তবে, আপনি এই syntax এর যেকোনো part omit করতে পারেন যা Rust program এর অন্য information থেকে figure out করতে পারে। আপনি শুধুমাত্র এই verbose syntax ব্যবহার করার প্রয়োজন সেই সব case এ যেখানে multiple implementation আছে যা same name ব্যবহার করে এবং Rust কে identify করতে help এর প্রয়োজন হয় আপনি কোন implementation call করতে চান।

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

মাঝে মাঝে, আপনি এমন একটি trait definition লিখতে পারেন যা অন্য trait এর উপর depend করে: একটি type এর first trait implement করার জন্য, আপনি require করতে চান যে type টির second trait ও implement করা উচিত। আপনি এটা করবেন যাতে আপনার trait definition second trait এর associated item গুলো ব্যবহার করতে পারে। আপনার trait definition rely করে এমন trait কে আপনার trait এর _supertrait_ বলা হয়।

উদাহরণস্বরূপ, ধরুন আমরা `outline_print` method সহ একটি `OutlinePrint` trait তৈরি করতে চাই যা একটি given value এমনভাবে format করে print করবে যাতে এটি asterisk এর ভিতরে frame করা থাকে। মানে, একটি `Point` struct দেওয়া আছে যা standard library trait `Display` implement করে `(x, y)` result করার জন্য, যখন আমরা একটি `Point` instance এ `outline_print` call করব যার `x` এর জন্য `1` এবং `y` এর জন্য `3` আছে, তখন এটা নিচের মতো print করবে:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

`outline_print` method এর implementation এ, আমরা `Display` trait এর functionality ব্যবহার করতে চাই। তাই, আমাদের specify করার প্রয়োজন যে `OutlinePrint` trait শুধুমাত্র এমন type এর জন্য কাজ করবে যা `Display` ও implement করে এবং `OutlinePrint` এর প্রয়োজনীয় functionality provide করে। আমরা trait definition এ `OutlinePrint: Display` specify করে এটা করতে পারি। এই technique trait এ trait bound add করার similar। Listing 20-23 `OutlinePrint` trait এর একটি implementation দেখায়।

<Listing number="20-23" file-name="src/main.rs" caption="`OutlinePrint` trait implement করা যা `Display` থেকে functionality require করে">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-23/src/main.rs:here}}
```

</Listing>

যেহেতু আমরা specified করেছি যে `OutlinePrint` এর `Display` trait require করে, তাই আমরা `to_string` function ব্যবহার করতে পারি যা automatically যেকোনো type এর জন্য implement করা হয়েছে যা `Display` implement করে। যদি আমরা colon add না করে `to_string` ব্যবহার করার চেষ্টা করতাম এবং trait name এর পরে `Display` trait specify না করতাম, তাহলে আমরা একটি error পেতাম যেখানে বলা হতো যে current scope এ type `&Self` এর জন্য `to_string` নামের কোনো method পাওয়া যায় নি।

চলুন দেখি কি ঘটে যখন আমরা `Point` struct এর মতো `Display` implement করে না এমন type এর উপর `OutlinePrint` implement করার চেষ্টা করি:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

</Listing>

আমরা একটি error পাই যেখানে বলা হয়েছে যে `Display` require করা হয়েছে কিন্তু implement করা হয়নি:

```console
{{#include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

এটা fix করার জন্য, আমরা `Point` এর উপর `Display` implement করি এবং `OutlinePrint` এর require করা constraint satisfy করি, যেমন:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

</Listing>

তাহলে `Point` এর উপর `OutlinePrint` trait implement করা successfully compile হবে, এবং আমরা asterisk এর outline এর ভিতরে show করার জন্য `Point` instance এ `outline_print` call করতে পারি।

### Using the Newtype Pattern to Implement External Traits on External Types

Chapter 10 এর [“Implementing a Trait on a Type”][implementing-a-trait-on-a-type]<!-- ignore --> section এ, আমরা orphan rule mention করেছিলাম যা বলে যে আমাদের শুধুমাত্র তখনই একটি type এর উপর trait implement করার allow আছে যখন হয় trait অথবা type আমাদের crate এ local হয়। Tuple struct এ একটি নতুন type তৈরি করে আমরা _newtype pattern_ ব্যবহার করে এই restriction avoid করতে পারি। (আমরা Chapter 5 এর [“Using Tuple Structs without Named Fields to Create Different Types”][tuple-structs]<!-- ignore --> section এ tuple struct cover করেছি।) Tuple struct এ একটি field থাকবে এবং এটি এমন একটি type এর thin wrapper হবে যার জন্য আমরা trait implement করতে চাই। তারপর wrapper type আমাদের crate এ local হবে, এবং আমরা wrapper এর উপর trait implement করতে পারব। _Newtype_ হলো একটি term যা Haskell programming language থেকে originate হয়েছে। এই pattern ব্যবহার করার জন্য কোনো runtime performance penalty নেই, এবং wrapper type compile time এ elided হয়ে যায়।

উদাহরণস্বরূপ, ধরুন আমরা `Vec<T>` এর উপর `Display` implement করতে চাই, যা orphan rule prevent করে আমাদের directly করার থেকে কারণ `Display` trait এবং `Vec<T>` type আমাদের crate এর বাইরে define করা হয়েছে। আমরা একটি `Wrapper` struct তৈরি করতে পারি যা `Vec<T>` এর একটি instance hold করে; তারপর আমরা `Wrapper` এর উপর `Display` implement করতে পারি এবং `Vec<T>` value ব্যবহার করতে পারি, যা Listing 20-24 এ দেখানো হয়েছে।

<Listing number="20-24" file-name="src/main.rs" caption="`Display` implement করার জন্য `Vec<String>` এর চারপাশে একটি `Wrapper` type তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-24/src/main.rs}}
```

</Listing>

`Display` এর implementation inner `Vec<T>` access করার জন্য `self.0` ব্যবহার করে, কারণ `Wrapper` একটি tuple struct এবং `Vec<T>` tuple এ index 0 এ থাকা item। তারপর আমরা `Wrapper` এর উপর `Display` trait এর functionality ব্যবহার করতে পারি।

এই technique ব্যবহার করার downside হলো `Wrapper` একটি নতুন type, তাই এটির সেই value এর method নেই যা এটি hold করে। `Wrapper` কে `Vec<T>` এর মতো treat করার allow করার জন্য আমাদের `Vec<T>` এর সব method directly `Wrapper` এর উপর implement করতে হতো যাতে method গুলো `self.0` এ delegate করে। যদি আমরা চাইতাম new type এর inner type এর সব method থাকুক, তাহলে inner type return করার জন্য `Wrapper` এর উপর `Deref` trait implement করা (Chapter 15 এর [“Treating Smart Pointers Like Regular References with the `Deref` Trait”][smart-pointer-deref]<!-- ignore --> section এ discuss করা হয়েছে) একটি solution হতে পারত। যদি আমরা না চাই `Wrapper` type এর inner type এর সব method থাকুক—উদাহরণস্বরূপ, `Wrapper` type এর behaviour restrict করার জন্য—তাহলে আমাদের শুধু যে method গুলো আমরা চাই সেগুলো manually implement করতে হতো।

এই newtype pattern ও useful এমনকি যখন trait involve থাকে না। চলুন focus change করি এবং Rust এর type system এর সাথে interact করার কিছু advanced way দেখি।

[newtype]: ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
[implementing-a-trait-on-a-type]: ch10-02-traits.html#implementing-a-trait-on-a-type
[traits-defining-shared-behavior]: ch10-02-traits.html#traits-defining-shared-behavior
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
