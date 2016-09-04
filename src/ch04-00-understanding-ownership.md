# Understanding Ownership

<!-- Since we go into a lot of detail about strings here too, do you want to alter the title of the chapter to include strings, so a reader looking at the ToC would easily know where to go? -->

Ownership is Rust's most unique feature, and
enables Rust to make memory safety guarantees without needing a garbage
collector. It's therefore important to understand how owndership works in Rust. In this chapter we'll talk about ownership as well as several related features: borrowing, slices,
and how Rust lays things out in memory.
