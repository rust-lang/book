# Advanced Features

এই মুহূর্তে, আপনি Rust programming language এর most commonly used part গুলো শিখেছেন। Chapter 21 এ আরও একটি project করার আগে, চলুন language এর কিছু aspect দেখি যা মাঝে মাঝে আপনার প্রয়োজন হতে পারে, কিন্তু হয়তো প্রতিদিন ব্যবহার নাও করতে পারেন। আপনি এই chapter কে reference হিসেবে ব্যবহার করতে পারেন যখন আপনি কোনো unknown এর সম্মুখীন হন। এখানে cover করা feature গুলো specific situation এ useful। যদিও আপনি হয়তো প্রায়ই সেগুলোর দিকে reach করবেন না, তবুও আমরা নিশ্চিত করতে চাই যে Rust এর offer করার মতো সব feature এর উপর আপনার grasp আছে।

এই chapter এ, আমরা আলোচনা করব:

-   Unsafe Rust: Rust এর কিছু guarantee থেকে কিভাবে opt out করতে হয় এবং সেই guarantee গুলো manually uphold করার responsibility নেওয়া
-   Advanced traits: associated type, default type parameter, fully qualified syntax, supertraits, এবং trait এর সাথে সম্পর্কিত newtype pattern
-   Advanced types: newtype pattern, type alias, never type, এবং dynamically sized type নিয়ে আরও আলোচনা
-   Advanced function এবং closure: function pointer এবং closure return করা
-   Macro: compile time এ আরও code define করে এমন code define করার উপায়

এটা সবার জন্য কিছু না কিছু আছে এমন Rust feature এর একটি panoply! চলুন dive in করি!
