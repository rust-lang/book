# Smart Pointers

একটি _পয়েন্টার_ হল একটি variable-এর জন্য একটি general concept যাতে মেমরির একটি address থাকে। এই address টি অন্য কোনো ডেটাকে refer করে, বা "পয়েন্ট করে"। Rust-এ সবচেয়ে common ধরনের পয়েন্টার হল একটি reference, যেটি সম্পর্কে আপনি Chapter 4-এ শিখেছেন। Reference গুলো `&` চিহ্ন দ্বারা নির্দেশিত হয় এবং যে value-টিকে point করে সেটিকে borrow করে। ডেটা refer করা ছাড়া এগুলোর অন্য কোনো special capabilities নেই এবং কোনো overhead নেই।

অন্যদিকে, _স্মার্ট পয়েন্টারগুলো_ হল ডেটা স্ট্রাকচার যা পয়েন্টারের মতো কাজ করে কিন্তু অতিরিক্ত মেটাডেটা এবং capabilities-ও রাখে। স্মার্ট পয়েন্টারের concept টি Rust-এর জন্য unique নয়: স্মার্ট পয়েন্টারগুলোর উৎপত্তি C++-এ এবং অন্যান্য language-এও রয়েছে। Rust-এর standard library-তে বিভিন্ন ধরনের স্মার্ট পয়েন্টার define করা আছে যা reference-এর মাধ্যমে provide করা functionality-এর চেয়েও বেশি কিছু provide করে। General concept টি explore করার জন্য, আমরা স্মার্ট পয়েন্টারের কয়েকটি ভিন্ন উদাহরণ দেখব, যার মধ্যে একটি _reference counting_ স্মার্ট পয়েন্টার type রয়েছে। এই পয়েন্টারটি আপনাকে ডেটার একাধিক owner রাখার অনুমতি দেয় owner-এর সংখ্যা ট্র্যাক করে এবং যখন কোনো owner অবশিষ্ট থাকে না, তখন ডেটা clean up করে।

Rust-এর, ownership এবং borrowing-এর concept সহ, reference এবং স্মার্ট পয়েন্টারের মধ্যে একটি additional পার্থক্য রয়েছে: যেখানে reference গুলো শুধুমাত্র ডেটা borrow করে, সেখানে অনেক ক্ষেত্রে স্মার্ট পয়েন্টারগুলো যে ডেটা point করে তার _owner_ হয়।

যদিও আমরা সেগুলোকে সেই সময়ে সেই নামে ডাকিনি, আমরা ইতিমধ্যেই এই বইয়ে কয়েকটি স্মার্ট পয়েন্টারের সম্মুখীন হয়েছি, যার মধ্যে Chapter 8-এর `String` এবং `Vec<T>` রয়েছে। এই দুটি type-কেই স্মার্ট পয়েন্টার হিসেবে গণ্য করা হয় কারণ তারা কিছু মেমরির owner এবং আপনাকে এটিকে manipulate করার অনুমতি দেয়। এগুলোর metadata এবং extra capabilities বা guarantee-ও রয়েছে। উদাহরণস্বরূপ, `String` তার capacity-কে metadata হিসেবে store করে এবং এর ডেটা সব সময় valid UTF-8 হবে তা নিশ্চিত করার additional ability রাখে।

স্মার্ট পয়েন্টারগুলো সাধারণত struct ব্যবহার করে implement করা হয়। একটি ordinary struct-এর বিপরীতে, স্মার্ট পয়েন্টারগুলো `Deref` এবং `Drop` trait গুলোকে implement করে। `Deref` trait-টি স্মার্ট পয়েন্টার struct-এর একটি instance-কে একটি reference-এর মতো আচরণ করার অনুমতি দেয় যাতে আপনি আপনার কোড লিখতে পারেন reference বা স্মার্ট পয়েন্টার উভয়ের সাথেই কাজ করার জন্য। `Drop` trait আপনাকে সেই কোডটি কাস্টমাইজ করার অনুমতি দেয় যা স্মার্ট পয়েন্টারের একটি instance scope-এর বাইরে চলে গেলে run হয়। এই chapter-এ, আমরা এই দুটি trait নিয়ে আলোচনা করব এবং প্রদর্শন করব কেন সেগুলো স্মার্ট পয়েন্টারগুলোর জন্য important।

যেহেতু স্মার্ট পয়েন্টার প্যাটার্নটি Rust-এ প্রায়শই ব্যবহৃত একটি general design pattern, তাই এই chapter-এ existing সমস্ত স্মার্ট পয়েন্টার cover করা হবে না। অনেক লাইব্রেরির নিজস্ব স্মার্ট পয়েন্টার রয়েছে এবং আপনি নিজেও লিখতে পারেন। আমরা standard library-এর সবচেয়ে common স্মার্ট পয়েন্টারগুলো cover করব:

-   `Box<T>` heap-এ value allocate করার জন্য
-   `Rc<T>`, একটি reference counting type যা multiple ownership-এর অনুমতি দেয়
-   `Ref<T>` এবং `RefMut<T>`, `RefCell<T>`-এর মাধ্যমে অ্যাক্সেস করা হয়, এমন একটি type যা compile time-এর পরিবর্তে runtime-এ borrowing rule গুলো enforce করে

এছাড়াও, আমরা _ইন্টেরিয়র মিউটেবিলিটি_ প্যাটার্নটি কভার করব যেখানে একটি immutable type একটি ভেতরের value mutate করার জন্য একটি API expose করে। আমরা _reference cycle_ নিয়েও আলোচনা করব: কীভাবে সেগুলো মেমরি লিক করতে পারে এবং কীভাবে সেগুলো প্রতিরোধ করা যায়।

আসুন শুরু করা যাক!
