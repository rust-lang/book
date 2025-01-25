# Patterns and Matching

_Patterns_ হলো Rust এ type এর structure এর সাথে match করার জন্য special syntax, complex এবং simple দুটোই। Pattern এর সাথে `match` expression এবং অন্যান্য construct ব্যবহার করা program এর control flow এর উপর আরও control দেয়। একটি pattern এ নিচের কিছু combination থাকে:

-   Literals
-   Destructured array, enum, struct, বা tuples
-   Variables
-   Wildcards
-   Placeholders

কিছু example pattern হলো `x`, `(a, 3)`, এবং `Some(Color::Red)`। Pattern valid এমন context এ, এই component গুলো data এর shape describe করে। তারপর আমাদের program কোনো particular piece of code run করা continue করার জন্য data এর সঠিক shape আছে কিনা তা determine করার জন্য pattern এর সাথে value match করে।

Pattern ব্যবহার করার জন্য, আমরা এটিকে কোনো value এর সাথে compare করি। যদি pattern value এর সাথে match করে, তাহলে আমরা আমাদের code এ value এর অংশগুলো ব্যবহার করি। Chapter 6 এ match expression মনে করুন যেখানে pattern ব্যবহার করা হয়েছিল, যেমন coin-sorting machine এর উদাহরণ। যদি value pattern এর shape fit করে, তাহলে আমরা named piece গুলো ব্যবহার করতে পারি। যদি না করে, তাহলে pattern এর সাথে associated code run হবে না।

এই chapter টি হলো pattern related সব কিছু নিয়ে একটি reference। আমরা pattern ব্যবহার করার valid জায়গা, refutable এবং irrefutable pattern এর মধ্যে পার্থক্য, এবং pattern syntax এর বিভিন্ন type নিয়ে আলোচনা করব যা আপনি দেখতে পারেন। Chapter এর শেষে, আপনি clear way তে অনেক concept express করার জন্য pattern ব্যবহার করতে শিখবেন।
