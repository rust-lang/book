# নির্ভীক Concurrency

Concurrent প্রোগ্রামিং নিরাপদে এবং দক্ষতার সাথে হ্যান্ডেল করা Rust-এর অন্যতম প্রধান লক্ষ্য। _Concurrent প্রোগ্রামিং_, যেখানে একটি প্রোগ্রামের বিভিন্ন অংশ স্বাধীনভাবে execute করে, এবং _parallel প্রোগ্রামিং_, যেখানে একটি প্রোগ্রামের বিভিন্ন অংশ একই সময়ে execute করে, এগুলি ক্রমশ গুরুত্বপূর্ণ হয়ে উঠছে কারণ আরও বেশি সংখ্যক কম্পিউটার তাদের multiple প্রসেসরগুলোর সুবিধা নিচ্ছে। ঐতিহাসিকভাবে, এই প্রেক্ষাপটগুলোতে প্রোগ্রামিং কঠিন এবং ত্রুটিপ্রবণ ছিল। Rust এই situation পরিবর্তন করতে চায়।

প্রাথমিকভাবে, Rust টিম ভেবেছিল যে মেমরি নিরাপত্তা নিশ্চিত করা এবং concurrency সমস্যা প্রতিরোধ করা দুটি আলাদা চ্যালেঞ্জ যা different method দিয়ে সমাধান করতে হবে। সময়ের সাথে সাথে, টিম আবিষ্কার করেছে যে ownership এবং type system হল মেমরি নিরাপত্তা _এবং_ concurrency সমস্যাগুলো manage করার জন্য একটি powerful tool-এর set! Ownership এবং type checking-এর সুবিধা নিয়ে, অনেক concurrency error Rust-এ compile-time error, runtime error নয়। অতএব, আপনাকে একটি runtime concurrency bug ঘটার exact circumstances গুলো reproduce করার চেষ্টা করার জন্য প্রচুর সময় ব্যয় করার পরিবর্তে, incorrect কোড compile হতে অস্বীকার করবে এবং সমস্যাটি ব্যাখ্যা করে একটি error উপস্থাপন করবে। ফলস্বরূপ, আপনি আপনার কোডটি production-এ পাঠানোর পরে potentially ঠিক করার পরিবর্তে এটিতে কাজ করার সময় ঠিক করতে পারেন। আমরা Rust-এর এই দিকটির নাম দিয়েছি _নির্ভীক_ _concurrency_। নির্ভীক concurrency আপনাকে এমন কোড লিখতে দেয় যা সূক্ষ্ম ত্রুটিমুক্ত এবং নতুন বাগ প্রবর্তন না করে refactor করা সহজ।

> দ্রষ্টব্য: সরলতার জন্য, আমরা অনেক সমস্যাকে আরও সুনির্দিষ্টভাবে _concurrent এবং/অথবা parallel_ বলার পরিবর্তে _concurrent_ হিসাবে refer করব। যদি এই বইটি concurrency এবং/অথবা parallelism সম্পর্কে হত, তাহলে আমরা আরও specific হতাম। এই chapter-এর জন্য, অনুগ্রহ করে মানসিকভাবে _concurrent_ ব্যবহার করার সময় _concurrent এবং/অথবা parallel_ substitute করুন।

অনেক language concurrent সমস্যাগুলো হ্যান্ডেল করার জন্য যে সমাধানগুলো offer করে সে সম্পর্কে dogmatic। উদাহরণস্বরূপ, Erlang-এর message-passing concurrency-র জন্য elegant functionality রয়েছে কিন্তু thread-গুলোর মধ্যে state share করার জন্য শুধুমাত্র অস্পষ্ট উপায় রয়েছে। Possible solution গুলোর শুধুমাত্র একটি subset-কে support করা higher-level language গুলোর জন্য একটি যুক্তিসঙ্গত কৌশল, কারণ একটি higher-level language কিছু control ত্যাগ করে abstraction অর্জনের সুবিধাগুলোর প্রতিশ্রুতি দেয়। যাইহোক, lower-level language গুলো থেকে যেকোনো পরিস্থিতিতে best performance সহ সমাধান provide করার আশা করা হয় এবং hardware-এর উপর কম abstraction থাকে। অতএব, Rust আপনার পরিস্থিতি এবং requirement-এর জন্য উপযুক্ত যেকোনো উপায়ে problem গুলো model করার জন্য বিভিন্ন ধরনের tool offer করে।

এই chapter-এ আমরা যে বিষয়গুলো cover করব সেগুলো হল:

-   কিভাবে একই সময়ে multiple code-এর অংশ run করার জন্য thread তৈরি করতে হয়
-   _Message-passing_ concurrency, যেখানে channel গুলো thread-গুলোর মধ্যে message পাঠায়
-   _Shared-state_ concurrency, যেখানে multiple thread-এর কিছু ডেটার অ্যাক্সেস থাকে
-   `Sync` এবং `Send` trait, যেগুলো Rust-এর concurrency গ্যারান্টিগুলোকে user-defined type-এর পাশাপাশি standard library দ্বারা provide করা type গুলোতেও extend করে
