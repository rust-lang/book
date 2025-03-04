## রেফারেন্স সাইকেল মেমরি লিক করতে পারে

Rust-এর মেমরি সুরক্ষার গ্যারান্টিগুলো অ্যাক্সিডেন্টালি মেমরি তৈরি করা কঠিন করে তোলে, যা কখনও clean up করা হয় না (_মেমরি লিক_ নামে পরিচিত)। মেমরি লিক সম্পূর্ণরূপে প্রতিরোধ করা Rust-এর গ্যারান্টিগুলোর মধ্যে একটি নয়, অর্থাৎ Rust-এ মেমরি লিক মেমরি নিরাপদ। আমরা `Rc<T>` এবং `RefCell<T>` ব্যবহার করে দেখতে পারি যে Rust মেমরি লিকের অনুমতি দেয়: এমন রেফারেন্স তৈরি করা সম্ভব যেখানে আইটেমগুলো একে অপরের দিকে একটি চক্রে নির্দেশ করে। এটি মেমরি লিক তৈরি করে কারণ চক্রের প্রতিটি আইটেমের রেফারেন্স গণনা কখনও 0-এ পৌঁছাবে না এবং value গুলো কখনও ড্রপ হবে না।

### একটি রেফারেন্স সাইকেল তৈরি করা

আসুন দেখি কিভাবে একটি রেফারেন্স সাইকেল ঘটতে পারে এবং কিভাবে এটি প্রতিরোধ করা যায়, `List` enum-এর definition এবং Listing 15-25-এর একটি `tail` মেথড দিয়ে শুরু করি:

<Listing number="15-25" file-name="src/main.rs" caption="একটি cons list definition যা একটি `RefCell<T>` ধারণ করে যাতে আমরা modify করতে পারি একটি `Cons` variant কিসের দিকে point করছে">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs}}
```

</Listing>

আমরা Listing 15-5 থেকে `List` definition-এর আরেকটি variation ব্যবহার করছি। `Cons` variant-এর দ্বিতীয় element টি এখন `RefCell<Rc<List>>`, অর্থাৎ Listing 15-24-এর মতো `i32` value modify করার ক্ষমতা থাকার পরিবর্তে, আমরা `List` value-টিকে modify করতে চাই যেটি একটি `Cons` variant point করছে। আমরা একটি `tail` মেথডও যোগ করছি যাতে আমাদের জন্য দ্বিতীয় item অ্যাক্সেস করা সুবিধাজনক হয় যদি আমাদের কাছে একটি `Cons` variant থাকে।

Listing 15-26-এ, আমরা একটি `main` ফাংশন যোগ করছি যা Listing 15-25-এর definition গুলো ব্যবহার করে। এই কোডটি `a`-তে একটি list এবং `b`-তে একটি list তৈরি করে যা `a`-এর list-এর দিকে point করে। তারপর এটি `a`-এর list-টিকে `b`-এর দিকে point করার জন্য modify করে, একটি রেফারেন্স সাইকেল তৈরি করে। এই প্রক্রিয়ার বিভিন্ন পয়েন্টে reference count গুলো কী তা দেখানোর জন্য পথের মধ্যে `println!` স্টেটমেন্ট রয়েছে।

<Listing number="15-26" file-name="src/main.rs" caption="দুটি `List` value-এর একটি রেফারেন্স সাইকেল তৈরি করা যা একে অপরের দিকে point করছে">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

</Listing>

আমরা `a` variable-এ একটি `List` value ধারণকারী একটি `Rc<List>` ইন্সট্যান্স তৈরি করি, `5, Nil`-এর একটি initial list সহ। তারপর আমরা `b` variable-এ আরেকটি `List` value ধারণকারী একটি `Rc<List>` ইন্সট্যান্স তৈরি করি যাতে 10 value রয়েছে এবং `a`-এর list-এর দিকে point করে।

আমরা `a`-কে modify করি যাতে এটি `Nil`-এর পরিবর্তে `b`-এর দিকে point করে, একটি cycle তৈরি করে। আমরা `tail` মেথড ব্যবহার করে `a`-তে `RefCell<Rc<List>>`-এর একটি reference পেতে করি, যেটি আমরা `link` variable-এ রাখি। তারপর আমরা `RefCell<Rc<List>>`-এ `borrow_mut` মেথড ব্যবহার করে ভেতরের value-টিকে একটি `Rc<List>` থেকে পরিবর্তন করি যা একটি `Nil` value ধারণ করে `b`-এর `Rc<List>`-এ।

যখন আমরা এই কোডটি চালাই, আপাতত শেষ `println!` টিকে commented out রেখে, আমরা এই আউটপুট পাব:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

`A` এবং `b` উভয়ের `Rc<List>` ইন্সট্যান্সের reference count 2 হয় `a`-এর list-টিকে `b`-এর দিকে point করার জন্য পরিবর্তন করার পরে। `Main`-এর শেষে, Rust `b` variable টি ড্রপ করে, যা `b` `Rc<List>` ইন্সট্যান্সের reference count 2 থেকে 1-এ কমিয়ে দেয়। `Rc<List>`-এর মেমরি heap-এ এই সময়ে ড্রপ করা হবে না, কারণ এর reference count 1, 0 নয়। তারপর Rust `a` ড্রপ করে, যা `a` `Rc<List>` ইন্সট্যান্সের reference count-ও 2 থেকে 1-এ কমিয়ে দেয়। এই ইন্সট্যান্সের মেমরিও ড্রপ করা যাবে না, কারণ অন্য `Rc<List>` ইন্সট্যান্স এখনও এটির দিকে refer করছে। List-এর জন্য allocate করা মেমরি চিরতরে uncollected থাকবে। এই reference cycle টি visualize করার জন্য, আমরা Figure 15-4-এ ডায়াগ্রামটি তৈরি করেছি।

<img alt="তালিকার রেফারেন্স সাইকেল" src="img/trpl15-04.svg" class="center" />

<span class="caption">Figure 15-4: List `a` এবং `b`-এর একটি reference cycle একে অপরের দিকে point করছে</span>

আপনি যদি শেষ `println!` টিকে un-comment করেন এবং প্রোগ্রামটি চালান, তাহলে Rust `a` থেকে `b` থেকে `a`-এর দিকে point করে এই cycle-টি প্রিন্ট করার চেষ্টা করবে, এবং এভাবে, যতক্ষণ না এটি stack overflow করে।

একটি real-world প্রোগ্রামের তুলনায়, এই উদাহরণে একটি reference cycle তৈরি করার পরিণতিগুলো খুব ভয়াবহ নয়: আমরা reference cycle তৈরি করার পরপরই, প্রোগ্রামটি শেষ হয়ে যায়। যাইহোক, যদি একটি আরও complex প্রোগ্রাম একটি চক্রে প্রচুর মেমরি allocate করত এবং দীর্ঘ সময় ধরে রাখত, তাহলে প্রোগ্রামটি প্রয়োজনের চেয়ে বেশি মেমরি ব্যবহার করত এবং সিস্টেমকে overwhelm করতে পারত, যার ফলে available মেমরি শেষ হয়ে যেত।

Reference cycle তৈরি করা সহজে করা যায় না, তবে এটি অসম্ভবও নয়। যদি আপনার কাছে `RefCell<T>` value থাকে যাতে `Rc<T>` value বা interior mutability এবং reference counting সহ type-গুলোর similar nested combination থাকে, তাহলে আপনাকে নিশ্চিত করতে হবে যে আপনি cycle তৈরি করবেন না; আপনি সেগুলোকে ধরার জন্য Rust-এর উপর নির্ভর করতে পারবেন না। একটি reference cycle তৈরি করা আপনার প্রোগ্রামের একটি logic bug হবে যা minimize করার জন্য আপনার automated test, code review এবং অন্যান্য software development practice ব্যবহার করা উচিত।

Reference cycle এড়ানোর আরেকটি সমাধান হল আপনার ডেটা স্ট্রাকচারগুলোকে এমনভাবে reorganize করা যাতে কিছু reference ownership প্রকাশ করে এবং কিছু reference না করে। ফলস্বরূপ, আপনি কিছু ownership relationship এবং কিছু non-ownership relationship দিয়ে তৈরি cycle পেতে পারেন এবং শুধুমাত্র ownership relationship গুলোই প্রভাবিত করে যে একটি value ড্রপ করা যেতে পারে কিনা। Listing 15-25-এ, আমরা সব সময় চাই যে `Cons` variant গুলো তাদের list-এর owner হোক, তাই ডেটা স্ট্রাকচার reorganize করা সম্ভব নয়। আসুন parent node এবং child node দিয়ে তৈরি graph ব্যবহার করে একটি উদাহরণ দেখি এটা দেখার জন্য কখন non-ownership relationship গুলো reference cycle প্রতিরোধ করার একটি উপযুক্ত উপায়।

### Reference Cycle প্রতিরোধ করা: একটি `Rc<T>`-কে একটি `Weak<T>`-তে পরিণত করা

এখন পর্যন্ত, আমরা প্রদর্শন করেছি যে `Rc::clone` কল করা একটি `Rc<T>` ইন্সট্যান্সের `strong_count` বাড়ায় এবং একটি `Rc<T>` ইন্সট্যান্স শুধুমাত্র তখনই clean up করা হয় যদি এর `strong_count` 0 হয়। আপনি `Rc::downgrade` কল করে এবং `Rc<T>`-এর একটি reference pass করে একটি `Rc<T>` ইন্সট্যান্সের মধ্যে value-টির একটি _weak reference_-ও তৈরি করতে পারেন। _Strong reference_ হল কিভাবে আপনি একটি `Rc<T>` ইন্সট্যান্সের ownership share করতে পারেন। _Weak reference_ গুলো ownership relationship প্রকাশ করে না এবং সেগুলোর count `Rc<T>` ইন্সট্যান্স কখন clean up করা হবে তা প্রভাবিত করে না। সেগুলো একটি reference cycle-এর কারণ হবে না কারণ কিছু weak reference জড়িত যেকোনো cycle ভেঙে যাবে একবার জড়িত value-গুলোর strong reference count 0 হলে।

আপনি যখন `Rc::downgrade` কল করেন, তখন আপনি `Weak<T>` type-এর একটি স্মার্ট পয়েন্টার পান। `Rc<T>` ইন্সট্যান্সে `strong_count` 1 বাড়ানোর পরিবর্তে, `Rc::downgrade` কল করলে `weak_count` 1 বাড়ে। `Rc<T>` টাইপটি `strong_count`-এর মতোই কতগুলো `Weak<T>` রেফারেন্স বিদ্যমান তা ট্র্যাক রাখতে `weak_count` ব্যবহার করে। পার্থক্য হল `Rc<T>` ইন্সট্যান্স clean up করার জন্য `weak_count`-এর 0 হওয়ার প্রয়োজন নেই।

যেহেতু `Weak<T>` যে value-টিকে refer করে সেটি ড্রপ করা হতে পারে, তাই `Weak<T>` যে value-টির দিকে point করছে সেটি দিয়ে কিছু করতে, আপনাকে নিশ্চিত করতে হবে যে value টি এখনও বিদ্যমান। এটি একটি `Weak<T>` ইন্সট্যান্সে `upgrade` method কল করে করুন, যেটি একটি `Option<Rc<T>>` রিটার্ন করবে। যদি `Rc<T>` value টি এখনও ড্রপ করা না হয়ে থাকে তাহলে আপনি `Some`-এর একটি result পাবেন এবং যদি `Rc<T>` value টি ড্রপ করা হয়ে থাকে তাহলে `None`-এর একটি result পাবেন। যেহেতু `upgrade` একটি `Option<Rc<T>>` রিটার্ন করে, তাই Rust নিশ্চিত করবে যে `Some` case এবং `None` case handle করা হয়েছে এবং কোনো invalid pointer থাকবে না।

একটি উদাহরণ হিসেবে, যে item গুলো শুধুমাত্র next item সম্পর্কে জানে এমন একটি list ব্যবহার করার পরিবর্তে, আমরা একটি tree তৈরি করব যার item গুলো তাদের children item _এবং_ তাদের parent item গুলো সম্পর্কে জানে।

#### একটি Tree ডেটা স্ট্রাকচার তৈরি করা: চাইল্ড নোড সহ একটি `Node`

শুরু করার জন্য, আমরা এমন node দিয়ে একটি tree তৈরি করব যা তাদের child node গুলো সম্পর্কে জানে। আমরা `Node` নামক একটি struct তৈরি করব যা তার নিজস্ব `i32` value এবং সেইসাথে এর children `Node` value-গুলোর reference ধারণ করে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

আমরা চাই একটি `Node` তার children-দের owner হোক এবং আমরা সেই ownership variable গুলোর সাথে share করতে চাই যাতে আমরা tree-এর প্রতিটি `Node` সরাসরি অ্যাক্সেস করতে পারি। এটি করার জন্য, আমরা `Vec<T>` item গুলোকে `Rc<Node>` type-এর value হিসেবে define করি। আমরা এটাও চাই যে কোন node গুলো অন্য node-এর child তা modify করতে, তাই `children`-এ `Vec<Rc<Node>>`-এর চারপাশে আমাদের একটি `RefCell<T>` আছে।

এরপরে, আমরা আমাদের struct definition ব্যবহার করব এবং value 3 এবং কোনো child ছাড়া `leaf` নামক একটি `Node` ইন্সট্যান্স এবং value 5 এবং `leaf`-কে তার children-দের মধ্যে একটি হিসেবে নিয়ে `branch` নামক আরেকটি ইন্সট্যান্স তৈরি করব, যেমনটি Listing 15-27-এ দেখানো হয়েছে:

<Listing number="15-27" file-name="src/main.rs" caption="কোনো child ছাড়া একটি `leaf` নোড এবং `leaf`-কে তার children-দের মধ্যে একটি হিসেবে নিয়ে একটি `branch` নোড তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

</Listing>

আমরা `leaf`-এ `Rc<Node>` clone করি এবং `branch`-এ store করি, অর্থাৎ `leaf`-এর `Node`-এর এখন দুটি owner রয়েছে: `leaf` এবং `branch`। আমরা `branch.children`-এর মাধ্যমে `branch` থেকে `leaf`-এ যেতে পারি, কিন্তু `leaf` থেকে `branch`-এ যাওয়ার কোনো উপায় নেই। কারণ হল `leaf`-এর `branch`-এর কোনো reference নেই এবং জানে না যে তারা related। আমরা চাই `leaf` জানুক যে `branch` হল এর parent। আমরা এরপরে সেটি করব।

#### একটি Child থেকে তার Parent-এর একটি Reference যোগ করা

Child node-টিকে তার parent সম্পর্কে অবগত করতে, আমাদের `Node` struct definition-এ একটি `parent` field যোগ করতে হবে। সমস্যা হল `parent`-এর type কী হওয়া উচিত তা decide করা। আমরা জানি এতে একটি `Rc<T>` থাকতে পারে না, কারণ এটি `leaf.parent`-এর `branch`-এর দিকে point করা এবং `branch.children`-এর `leaf`-এর দিকে point করা একটি reference cycle তৈরি করবে, যার ফলে তাদের `strong_count` value গুলো কখনও 0 হবে না।

অন্যভাবে relationship গুলো সম্পর্কে চিন্তা করলে, একটি parent node-এর তার children-দের owner হওয়া উচিত: যদি একটি parent node ড্রপ করা হয়, তাহলে তার child node গুলোও ড্রপ করা উচিত। যাইহোক, একটি child-এর তার parent-এর owner হওয়া উচিত নয়: যদি আমরা একটি child node ড্রপ করি, তাহলে parent-এর এখনও existing থাকা উচিত। এটি weak reference-এর জন্য একটি case!

তাই `Rc<T>`-এর পরিবর্তে, আমরা `parent`-এর type-টিকে `Weak<T>` ব্যবহার করব, specifically একটি `RefCell<Weak<Node>>`। এখন আমাদের `Node` struct definition দেখতে এরকম:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

একটি node তার parent node-কে refer করতে সক্ষম হবে কিন্তু তার parent-এর owner নয়। Listing 15-28-এ, আমরা `main` update করি এই new definition ব্যবহার করার জন্য যাতে `leaf` node-এর তার parent, `branch`-কে refer করার একটি উপায় থাকে:

<Listing number="15-28" file-name="src/main.rs" caption="তার parent node `branch`-এর একটি weak reference সহ একটি `leaf` নোড">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

</Listing>

`Leaf` node তৈরি করা Listing 15-27-এর মতোই, `parent` field ছাড়া: `leaf` কোনো parent ছাড়াই শুরু হয়, তাই আমরা একটি new, empty `Weak<Node>` reference instance তৈরি করি।

এই সময়ে, যখন আমরা `upgrade` method ব্যবহার করে `leaf`-এর parent-এর একটি reference পাওয়ার চেষ্টা করি, তখন আমরা একটি `None` value পাই। আমরা প্রথম `println!` স্টেটমেন্ট থেকে আউটপুটে এটি দেখতে পাই:

```text
leaf parent = None
```

যখন আমরা `branch` node তৈরি করি, তখন এটির `parent` field-এ একটি new `Weak<Node>` reference থাকবে, কারণ `branch`-এর কোনো parent node নেই। আমাদের কাছে এখনও `branch`-এর children-দের মধ্যে একটি হিসেবে `leaf` রয়েছে। একবার আমাদের কাছে `branch`-এ `Node` ইন্সট্যান্স থাকলে, আমরা `leaf`-কে modify করে এটিকে তার parent-এর একটি `Weak<Node>` reference দিতে পারি। আমরা `leaf`-এর `parent` field-এ `RefCell<Weak<Node>>`-এ `borrow_mut` method ব্যবহার করি এবং তারপর `branch`-এ `Rc<Node>` থেকে `branch`-এর একটি `Weak<Node>` reference তৈরি করতে `Rc::downgrade` ফাংশনটি ব্যবহার করি।

যখন আমরা `leaf`-এর parent-কে আবার প্রিন্ট করি, তখন এবার আমরা `branch` ধারণকারী একটি `Some` variant পাব: এখন `leaf` তার parent অ্যাক্সেস করতে পারে! যখন আমরা `leaf` প্রিন্ট করি, তখন আমরা Listing 15-26-এর মতো একটি stack overflow-তে শেষ হওয়া cycle-টিও এড়াই; `Weak<Node>` reference গুলো `(Weak)` হিসেবে প্রিন্ট করা হয়:

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

অসীম আউটপুটের অভাব নির্দেশ করে যে এই কোডটি একটি reference cycle তৈরি করেনি। আমরা `Rc::strong_count` এবং `Rc::weak_count` কল করে পাওয়া value গুলো দেখেও এটি বলতে পারি।

#### `strong_count` এবং `weak_count`-এর পরিবর্তনগুলো Visualizing করা

আসুন দেখি কিভাবে `Rc<Node>` ইন্সট্যান্সগুলোর `strong_count` এবং `weak_count` value গুলো পরিবর্তিত হয় একটি new inner scope তৈরি করে এবং `branch`-এর creation-কে সেই scope-এ move করে। এটি করার মাধ্যমে, আমরা দেখতে পাব `branch` তৈরি হলে এবং তারপর scope-এর বাইরে চলে গেলে drop হলে কী ঘটে। Modification গুলো Listing 15-29-এ দেখানো হয়েছে:

<Listing number="15-29" file-name="src/main.rs" caption="একটি ভেতরের scope-এ `branch` তৈরি করা এবং strong ও weak reference count গুলো পরীক্ষা করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

</Listing>

`Leaf` তৈরি হওয়ার পরে, এর `Rc<Node>`-এর strong count 1 এবং weak count 0 থাকে। ভেতরের scope-এ, আমরা `branch` তৈরি করি এবং এটিকে `leaf`-এর সাথে associate করি, সেই সময়ে, যখন আমরা count গুলো প্রিন্ট করি, তখন `branch`-এ `Rc<Node>`-এর strong count 1 এবং weak count 1 থাকবে (`leaf.parent`-এর জন্য `branch`-এর দিকে `Weak<Node>` দিয়ে point করা)। যখন আমরা `leaf`-এ count গুলো প্রিন্ট করি, তখন আমরা দেখব এটির strong count 2 হবে, কারণ `branch`-এ এখন `leaf`-এর `Rc<Node>`-এর একটি clone রয়েছে যা `branch.children`-এ store করা আছে, কিন্তু এখনও 0-এর একটি weak count থাকবে।

যখন ভেতরের scope শেষ হয়, তখন `branch` scope-এর বাইরে চলে যায় এবং `Rc<Node>`-এর strong count কমে 0 হয়, তাই এর `Node` ড্রপ করা হয়। `Leaf.parent` থেকে 1-এর weak count-এর `Node` ড্রপ করা হবে কিনা তার উপর কোনো প্রভাব নেই, তাই আমরা কোনো মেমরি লিক পাই না!

যদি আমরা scope-এর শেষের পরে `leaf`-এর parent অ্যাক্সেস করার চেষ্টা করি, তাহলে আমরা আবার `None` পাব। প্রোগ্রামের শেষে, `leaf`-এ `Rc<Node>`-এর strong count 1 এবং weak count 0 থাকে, কারণ variable `leaf` এখন আবার `Rc<Node>`-এর একমাত্র reference।

Count এবং value dropping manage করে এমন সমস্ত logic `Rc<T>` এবং `Weak<T>` এবং তাদের `Drop` trait-এর implementation-গুলোতে তৈরি করা হয়েছে। `Node`-এর definition-এ child থেকে তার parent-এর relationship একটি `Weak<T>` reference হওয়া উচিত specify করে, আপনি parent node-গুলোকে child node-গুলোর দিকে point করাতে পারবেন এবং এর বিপরীতে একটি reference cycle এবং মেমরি লিক তৈরি না করে।

## সারসংক্ষেপ

এই chapter-এ আলোচনা করা হয়েছে কিভাবে স্মার্ট পয়েন্টার ব্যবহার করে regular reference-এর সাথে Rust default ভাবে যে গ্যারান্টি এবং trade-off গুলো করে সেগুলো থেকে আলাদা গ্যারান্টি এবং trade-off করা যায়। `Box<T>` type-টির একটি known আকার রয়েছে এবং heap-এ allocate করা ডেটার দিকে point করে। `Rc<T>` type টি heap-এর ডেটার reference-এর সংখ্যা ট্র্যাক রাখে যাতে ডেটার multiple owner থাকতে পারে। `RefCell<T>` type তার ইন্টেরিয়র মিউটেবিলিটি সহ আমাদের এমন একটি type দেয় যা আমরা ব্যবহার করতে পারি যখন আমাদের একটি immutable type প্রয়োজন কিন্তু সেই type-এর একটি ভেতরের value পরিবর্তন করতে হবে; এটি compile time-এর পরিবর্তে runtime-এ borrowing rule গুলোও enforce করে।

এছাড়াও `Deref` এবং `Drop` trait নিয়ে আলোচনা করা হয়েছে, যেগুলো স্মার্ট পয়েন্টারগুলোর অনেক functionality enable করে। আমরা reference cycle গুলো explore করেছি যা মেমরি লিক করতে পারে এবং কিভাবে `Weak<T>` ব্যবহার করে সেগুলো প্রতিরোধ করা যায়।

যদি এই chapter টি আপনার আগ্রহ জাগিয়ে তোলে এবং আপনি আপনার নিজের স্মার্ট পয়েন্টার implement করতে চান, তাহলে আরও useful তথ্যের জন্য ["The Rustonomicon"][nomicon] দেখুন।

এরপরে, আমরা Rust-এ concurrency নিয়ে আলোচনা করব। আপনি কয়েকটি new smart pointer সম্পর্কেও জানতে পারবেন।

[nomicon]: ../nomicon/index.html
