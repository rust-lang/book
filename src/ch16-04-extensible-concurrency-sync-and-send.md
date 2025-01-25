## Extensible Concurrency with the `Sync` and `Send` Traits

মজার বিষয় হলো, Rust language এ concurrency এর খুবই কম feature আছে। এই chapter এ আমরা concurrency নিয়ে যত feature আলোচনা করেছি তার প্রায় সবই standard library এর অংশ, language এর নয়। Concurrency handle করার জন্য আপনার option গুলো language বা standard library এর মধ্যে সীমাবদ্ধ নয়; আপনি নিজের concurrency feature লিখতে পারেন অথবা অন্যদের লেখা feature ব্যবহার করতে পারেন।

তবে, দুটি concurrency concept language এ দেওয়া আছে: `std::marker` traits `Sync` এবং `Send`.

### Allowing Transference of Ownership Between Threads with `Send`

`Send` marker trait নির্দেশ করে যে `Send` implement করা type এর value এর ownership thread এর মধ্যে transfer করা যেতে পারে। প্রায় সব Rust type `Send`, কিন্তু কিছু exception আছে, যার মধ্যে `Rc<T>` একটি: এটি `Send` হতে পারে না কারণ আপনি যদি একটি `Rc<T>` value clone করেন এবং clone এর ownership অন্য একটি thread এ transfer করার চেষ্টা করেন, তাহলে দুটি thread একই সময়ে reference count update করতে পারে। এই কারণে, `Rc<T>` single-threaded পরিস্থিতিতে ব্যবহারের জন্য implement করা হয়েছে যেখানে আপনি thread-safe performance penalty দিতে চান না।

তাই, Rust এর type system এবং trait bounds নিশ্চিত করে যে আপনি কোনোভাবেই accidentally unsafe ভাবে thread এর মধ্যে `Rc<T>` value send করতে পারবেন না। যখন আমরা Listing 16-14 এ এটা করার চেষ্টা করেছিলাম, তখন আমরা error পেয়েছিলাম `the trait Send is not implemented for Rc<Mutex<i32>>`। যখন আমরা `Arc<T>` এ switch করেছিলাম, যা `Send`, তখন code compile হয়েছিল।

যে type গুলো সম্পূর্ণরূপে `Send` type দিয়ে গঠিত, সেগুলো automatic ভাবে `Send` হিসেবে চিহ্নিত হয়। প্রায় সব primitive type `Send`, raw pointer ছাড়া, যা আমরা Chapter 20 এ আলোচনা করব।

### Allowing Access from Multiple Threads with `Sync`

`Sync` marker trait নির্দেশ করে যে `Sync` implement করা type কে একাধিক thread থেকে reference করা safe। অন্যভাবে বলতে গেলে, যেকোনো type `T` `Sync` হবে যদি `&T` (`T` এর immutable reference) `Send` হয়, মানে reference টি safe ভাবে অন্য thread এ send করা যেতে পারে। `Send` এর মতোই, primitive type গুলো `Sync` এবং যে type গুলো সম্পূর্ণরূপে `Sync` type দিয়ে গঠিত সেগুলোও `Sync`।

Smart pointer `Rc<T>` ও `Sync` নয় একই কারণে যে কারণে এটি `Send` নয়। `RefCell<T>` type (যা নিয়ে আমরা Chapter 15 এ আলোচনা করেছিলাম) এবং related `Cell<T>` type এর family ও `Sync` নয়। `RefCell<T>` runtime এ borrow checking এর যে implementation করে তা thread-safe নয়। Smart pointer `Mutex<T>` `Sync` এবং একাধিক thread এর সাথে access share করার জন্য ব্যবহার করা যেতে পারে যেমনটা আপনি [“Sharing a `Mutex<T>` Between Multiple Threads”][sharing-a-mutext-between-multiple-threads]<!-- ignore --> section এ দেখেছেন।

### Implementing `Send` and `Sync` Manually Is Unsafe

যেহেতু যে type গুলো `Send` এবং `Sync` trait দিয়ে তৈরি সেগুলো automatic ভাবে `Send` এবং `Sync` হয়, তাই আমাদের manually এই trait গুলো implement করতে হয় না। Marker trait হিসেবে, এগুলোর implement করার জন্য কোনো method ও নেই। এগুলো শুধু concurrency এর সাথে সম্পর্কিত invariant enforce করার জন্য দরকারি।

Manually এই trait গুলো implement করার সাথে unsafe Rust code implement করা জড়িত। আমরা Chapter 20 এ unsafe Rust code ব্যবহার করা নিয়ে আলোচনা করব; আপাতত, গুরুত্বপূর্ণ তথ্য হলো `Send` এবং `Sync` অংশ দিয়ে তৈরি নয় এমন নতুন concurrent type তৈরি করার জন্য safety guarantee গুলো ধরে রাখার জন্য সতর্কতার সাথে চিন্তা করতে হয়। [“The Rustonomicon”][nomicon] এ এই guarantee গুলো এবং কিভাবে সেগুলো ধরে রাখতে হয় সে সম্পর্কে আরও তথ্য আছে।

## Summary

এই বইয়ে concurrency নিয়ে এটাই শেষ নয়: পুরো পরের chapter টি async programming এর উপর focus করা হয়েছে, এবং Chapter 21 এর project এ এই chapter এর concept গুলো আরও বাস্তব পরিস্থিতিতে ব্যবহার করা হবে যা এখানে আলোচনা করা ছোট উদাহরণগুলোর চেয়ে অনেক বেশি।

আগেই উল্লেখ করা হয়েছে, Rust concurrency handle করার পদ্ধতিগুলোর খুব কম অংশই language এর অংশ হওয়ায়, অনেক concurrency সমাধান crate হিসেবে implement করা হয়েছে। Standard library এর চেয়ে এগুলো দ্রুত evolve হয়, তাই multithreaded পরিস্থিতিতে ব্যবহার করার জন্য current, state-of-the-art crate গুলো online এ search করতে ভুলবেন না।

Rust standard library message passing এর জন্য channel এবং smart pointer type যেমন `Mutex<T>` এবং `Arc<T>` প্রদান করে, যা concurrent context এ ব্যবহার করা safe। Type system এবং borrow checker নিশ্চিত করে যে এই সমাধানগুলো ব্যবহার করে code data race বা invalid reference এ শেষ হবে না। একবার আপনার code compile হয়ে গেলে, আপনি নিশ্চিত থাকতে পারেন যে এটি অন্যান্য language এর মতো hard-to-track-down bug ছাড়াই একাধিক thread এ run হবে। Concurrent programming আর ভয়ের কোনো বিষয় নয়: এগিয়ে যান এবং আপনার প্রোগ্রামগুলোকে fearlessly concurrent করুন!

[sharing-a-mutext-between-multiple-threads]: ch16-03-shared-state.html#sharing-a-mutext-between-multiple-threads
[nomicon]: ../nomicon/index.html
```
