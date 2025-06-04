# ภาษาโปรแกรม Rust

[ภาษาโปรแกรม Rust](title-page.md)
[คำนำ](foreword.md)
[บทนำ](ch00-00-introduction.md)

## เริ่มต้นใช้งาน

- [เริ่มต้นใช้งาน](ch01-00-getting-started.md)
  - [การติดตั้ง](ch01-01-installation.md)
  - [Hello, World!](ch01-02-hello-world.md)
  - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [เขียนโปรแกรมเกมทายตัวเลข](ch02-00-guessing-game-tutorial.md)

- [แนวคิดการเขียนโปรแกรมทั่วไป](ch03-00-common-programming-concepts.md)
  - [ตัวแปรและการเปลี่ยนแปลงค่า](ch03-01-variables-and-mutability.md)
  - [ชนิดข้อมูล](ch03-02-data-types.md)
  - [ฟังก์ชัน](ch03-03-how-functions-work.md)
  - [คอมเมนต์](ch03-04-comments.md)
  - [การควบคุมการทำงานของโปรแกรม](ch03-05-control-flow.md)

- [ทำความเข้าใจ Ownership](ch04-00-understanding-ownership.md)
  - [Ownership คืออะไร?](ch04-01-what-is-ownership.md)
  - [การอ้างอิงและการยืม](ch04-02-references-and-borrowing.md)
  - [The Slice Type](ch04-03-slices.md)

- [การใช้ Structs เพื่อจัดกลุ่มข้อมูลที่เกี่ยวข้องกัน](ch05-00-structs.md)
  - [การกำหนดและการสร้าง Structs](ch05-01-defining-structs.md)
  - [ตัวอย่างโปรแกรมที่ใช้ Structs](ch05-02-example-structs.md)
  - [ синтаксис методов (Method Syntax)](ch05-03-method-syntax.md)

- [Enums และ Pattern Matching](ch06-00-enums.md)
  - [การกำหนด Enum](ch06-01-defining-an-enum.md)
  - [โครงสร้างควบคุมการทำงาน `match`](ch06-02-match.md)
  - [การควบคุมการทำงานแบบกระชับด้วย `if let` และ `let else`](ch06-03-if-let.md)

## ความรู้พื้นฐานเกี่ยวกับ Rust

- [การจัดการโปรเจกต์ที่ใหญ่ขึ้นด้วย Packages, Crates, และ Modules](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
  - [Packages และ Crates](ch07-01-packages-and-crates.md)
  - [การกำหนด Modules เพื่อควบคุมขอบเขตและการเข้าถึง](ch07-02-defining-modules-to-control-scope-and-privacy.md)
  - [Paths สำหรับการอ้างอิงไอเท็มใน Module Tree](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
  - [การนำ Paths เข้ามาใน Scope ด้วยคีย์เวิร์ด `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
  - [การแยก Modules ออกเป็นไฟล์ต่างๆ](ch07-05-separating-modules-into-different-files.md)

- [Collections ทั่วไป](ch08-00-common-collections.md)
  - [การเก็บรายการของค่าด้วย Vectors](ch08-01-vectors.md)
  - [การเก็บข้อความที่เข้ารหัสแบบ UTF-8 ด้วย Strings](ch08-02-strings.md)
  - [การเก็บ Keys พร้อมกับ Values ที่เชื่อมโยงกันใน Hash Maps](ch08-03-hash-maps.md)

- [การจัดการข้อผิดพลาด](ch09-00-error-handling.md)
  - [ข้อผิดพลาดที่ไม่สามารถกู้คืนได้ด้วย `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
  - [ข้อผิดพลาดที่สามารถกู้คืนได้ด้วย `Result`](ch09-02-recoverable-errors-with-result.md)
  - [จะ `panic!` หรือไม่ `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [Generic Types, Traits, และ Lifetimes](ch10-00-generics.md)
  - [Generic Data Types](ch10-01-syntax.md)
  - [Traits: การกำหนดพฤติกรรมร่วมกัน](ch10-02-traits.md)
  - [การตรวจสอบความถูกต้องของการอ้างอิงด้วย Lifetimes](ch10-03-lifetime-syntax.md)

- [การเขียนเทสต์อัตโนมัติ](ch11-00-testing.md)
  - [วิธีเขียนเทสต์](ch11-01-writing-tests.md)
  - [การควบคุมวิธีการรันเทสต์](ch11-02-running-tests.md)
  - [การจัดระเบียบเทสต์](ch11-03-test-organization.md)

- [โปรเจกต์ I/O: การสร้างโปรแกรม Command Line](ch12-00-an-io-project.md)
  - [การรับอาร์กิวเมนต์จาก Command Line](ch12-01-accepting-command-line-arguments.md)
  - [การอ่านไฟล์](ch12-02-reading-a-file.md)
  - [การ Refactor เพื่อปรับปรุง Modularity และการจัดการข้อผิดพลาด](ch12-03-improving-error-handling-and-modularity.md)
  - [การพัฒนาฟังก์ชันการทำงานของไลบรารีด้วย Test Driven Development](ch12-04-testing-the-librarys-functionality.md)
  - [การทำงานกับ Environment Variables](ch12-05-working-with-environment-variables.md)
  - [การเขียนข้อความแสดงข้อผิดพลาดไปยัง Standard Error แทน Standard Output](ch12-06-writing-to-stderr-instead-of-stdout.md)

## การคิดแบบ Rust

- [ฟีเจอร์ของภาษาเชิงฟังก์ชัน: Iterators และ Closures](ch13-00-functional-features.md)
  - [Closures: ฟังก์ชันที่ไม่ระบุชื่อที่สามารถจับภาพสภาพแวดล้อมของตนเองได้](ch13-01-closures.md)
  - [การประมวลผลชุดของไอเท็มด้วย Iterators](ch13-02-iterators.md)
  - [การปรับปรุงโปรเจกต์ I/O ของเรา](ch13-03-improving-our-io-project.md)
  - [การเปรียบเทียบประสิทธิภาพ: Loops กับ Iterators](ch13-04-performance.md)

- [เพิ่มเติมเกี่ยวกับ Cargo และ Crates.io](ch14-00-more-about-cargo.md)
  - [การปรับแต่ง Builds ด้วย Release Profiles](ch14-01-release-profiles.md)
  - [การเผยแพร่ Crate ไปยัง Crates.io](ch14-02-publishing-to-crates-io.md)
  - [Cargo Workspaces](ch14-03-cargo-workspaces.md)
  - [การติดตั้ง Binaries จาก Crates.io ด้วย `cargo install`](ch14-04-installing-binaries.md)
  - [การขยาย Cargo ด้วย Custom Commands](ch14-05-extending-cargo.md)

- [Smart Pointers](ch15-00-smart-pointers.md)
  - [การใช้ `Box<T>` เพื่อชี้ไปยังข้อมูลบน Heap](ch15-01-box.md)
  - [การใช้งาน Smart Pointers เหมือนกับการอ้างอิงทั่วไปด้วย `Deref`](ch15-02-deref.md)
  - [การรันโค้ดเมื่อ Cleanup ด้วย `Drop` Trait](ch15-03-drop.md)
  - [`Rc<T>`, Smart Pointer แบบนับการอ้างอิง](ch15-04-rc.md)
  - [`RefCell<T>` และรูปแบบ Interior Mutability](ch15-05-interior-mutability.md)
  - [Reference Cycles สามารถทำให้เกิด Memory Leak ได้](ch15-06-reference-cycles.md)

- [Concurrency แบบไม่หวาดหวั่น](ch16-00-concurrency.md)
  - [การใช้ Threads เพื่อรันโค้ดพร้อมกัน](ch16-01-threads.md)
  - [การใช้ Message Passing เพื่อถ่ายโอนข้อมูลระหว่าง Threads](ch16-02-message-passing.md)
  - [Shared-State Concurrency](ch16-03-shared-state.md)
  - [Concurrency ที่ขยายได้ด้วย `Send` และ `Sync` Traits](ch16-04-extensible-concurrency-sync-and-send.md)

- [พื้นฐานของการเขียนโปรแกรมแบบ Asynchronous: Async, Await, Futures, และ Streams](ch17-00-async-await.md)
  - [Futures และ синтаксис Async](ch17-01-futures-and-syntax.md)
  - [การประยุกต์ใช้ Concurrency กับ Async](ch17-02-concurrency-with-async.md)
  - [การทำงานกับ Futures จำนวนเท่าใดก็ได้](ch17-03-more-futures.md)
  - [Streams: Futures ตามลำดับ](ch17-04-streams.md)
  - [ดูรายละเอียด Traits สำหรับ Async อย่างใกล้ชิด](ch17-05-traits-for-async.md)
  - [Futures, Tasks, และ Threads](ch17-06-futures-tasks-threads.md)

- [ฟีเจอร์การเขียนโปรแกรมเชิงวัตถุของ Rust](ch18-00-oop.md)
  - [ลักษณะของภาษาเชิงวัตถุ](ch18-01-what-is-oo.md)
  - [การใช้ Trait Objects ที่อนุญาตให้มีค่าต่างชนิดกันได้](ch18-02-trait-objects.md)
  - [การนำรูปแบบการออกแบบเชิงวัตถุไปใช้งาน](ch18-03-oo-design-patterns.md)

## หัวข้อขั้นสูง

- [Patterns และ Matching](ch19-00-patterns.md)
  - [ทุกที่ที่สามารถใช้ Patterns ได้](ch19-01-all-the-places-for-patterns.md)
  - [Refutability: Pattern อาจไม่ Match หรือไม่](ch19-02-refutability.md)
  - [ синтаксис Pattern (Pattern Syntax)](ch19-03-pattern-syntax.md)

- [ฟีเจอร์ขั้นสูง](ch20-00-advanced-features.md)
  - [Unsafe Rust](ch20-01-unsafe-rust.md)
  - [Advanced Traits](ch20-02-advanced-traits.md)
  - [Advanced Types](ch20-03-advanced-types.md)
  - [Advanced Functions และ Closures](ch20-04-advanced-functions-and-closures.md)
  - [Macros](ch20-05-macros.md)

- [โปรเจกต์สุดท้าย: การสร้าง Web Server แบบ Multithreaded](ch21-00-final-project-a-web-server.md)
  - [การสร้าง Web Server แบบ Single-Threaded](ch21-01-single-threaded.md)
  - [การเปลี่ยน Server Single-Threaded ของเราให้เป็น Server Multithreaded](ch21-02-multithreaded.md)
  - [การ Shutdown และ Cleanup อย่างสวยงาม](ch21-03-graceful-shutdown-and-cleanup.md)

- [ภาคผนวก](appendix-00.md)
  - [A - Keywords](appendix-01-keywords.md)
  - [B - Operators และ Symbols](appendix-02-operators.md)
  - [C - Derivable Traits](appendix-03-derivable-traits.md)
  - [D - เครื่องมือพัฒนาที่มีประโยชน์](appendix-04-useful-development-tools.md)
  - [E - Editions](appendix-05-editions.md)
  - [F - คำแปลของหนังสือ](appendix-06-translation.md)
  - [G - Rust ถูกสร้างขึ้นมาอย่างไร และ “Nightly Rust”](appendix-07-nightly-rust.md)
