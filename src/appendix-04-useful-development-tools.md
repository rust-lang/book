## ภาคผนวก D - เครื่องมือพัฒนาที่มีประโยชน์

ในภาคผนวกนี้ เราจะพูดถึงเครื่องมือพัฒนาที่มีประโยชน์บางอย่างที่โปรเจกต์ Rust มอบให้ เราจะดูการจัดรูปแบบอัตโนมัติ วิธีการแก้ไขคำเตือนอย่างรวดเร็ว linter และการผสานรวมกับ IDE

### การจัดรูปแบบอัตโนมัติด้วย `rustfmt`

เครื่องมือ `rustfmt` จะจัดรูปแบบโค้ดของคุณใหม่ตามสไตล์โค้ดของชุมชน โปรเจกต์ที่ทำงานร่วมกันหลายแห่งใช้ `rustfmt` เพื่อป้องกันการโต้เถียงเกี่ยวกับสไตล์ที่จะใช้ในการเขียน Rust: ทุกคนจัดรูปแบบโค้ดของตนโดยใช้เครื่องมือนี้

การติดตั้ง Rust จะมี rustfmt มาให้โดยปริยาย ดังนั้นคุณควรมีโปรแกรม `rustfmt` และ `cargo-fmt` อยู่ในระบบของคุณแล้ว คำสั่งทั้งสองนี้คล้ายกับ `rustc` และ `cargo` ตรงที่ `rustfmt` ช่วยให้สามารถควบคุมได้ละเอียดยิ่งขึ้น และ `cargo-fmt` เข้าใจแบบแผนของโปรเจกต์ที่ใช้ Cargo หากต้องการจัดรูปแบบโปรเจกต์ Cargo ใดๆ ให้ป้อนคำสั่งต่อไปนี้:

```sh
$ cargo fmt
```

การรันคำสั่งนี้จะจัดรูปแบบโค้ด Rust ทั้งหมดใน крейт ปัจจุบัน ซึ่งควรจะเปลี่ยนเฉพาะสไตล์โค้ด ไม่ใช่ความหมายของโค้ด

คำสั่งนี้จะให้ `rustfmt` และ `cargo-fmt` แก่คุณ คล้ายกับวิธีที่ Rust ให้ทั้ง `rustc` และ `cargo` แก่คุณ หากต้องการจัดรูปแบบโปรเจกต์ Cargo ใดๆ ให้ป้อนคำสั่งต่อไปนี้:

```console
$ cargo fmt
```

การรันคำสั่งนี้จะจัดรูปแบบโค้ด Rust ทั้งหมดใน крейт ปัจจุบัน ซึ่งควรจะเปลี่ยนเฉพาะสไตล์โค้ด ไม่ใช่ความหมายของโค้ด สำหรับข้อมูลเพิ่มเติมเกี่ยวกับ `rustfmt` โปรดดู [เอกสารประกอบ][rustfmt]

[rustfmt]: https://github.com/rust-lang/rustfmt

### แก้ไขโค้ดของคุณด้วย `rustfix`

เครื่องมือ `rustfix` รวมอยู่ในการติดตั้ง Rust และสามารถแก้ไขคำเตือนของคอมไพเลอร์โดยอัตโนมัติซึ่งมีวิธีแก้ไขปัญหาที่ชัดเจนซึ่งน่าจะเป็นสิ่งที่คุณต้องการ เป็นไปได้ว่าคุณเคยเห็นคำเตือนของคอมไพเลอร์มาก่อน ตัวอย่างเช่น พิจารณาโค้ดนี้:

<span class="filename">ชื่อไฟล์: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

ที่นี่ เรากำลังกำหนดตัวแปร `x` ให้เป็น mutable แต่เราไม่เคยเปลี่ยนแปลงค่าของมันเลย Rust เตือนเราเกี่ยวกับเรื่องนั้น:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

คำเตือนแนะนำให้เราลบ keyword `mut` เราสามารถใช้คำแนะนำนั้นโดยอัตโนมัติโดยใช้เครื่องมือ `rustfix` โดยการรันคำสั่ง `cargo fix`:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

เมื่อเราดูไฟล์ _src/main.rs_ อีกครั้ง เราจะเห็นว่า `cargo fix` ได้เปลี่ยนโค้ดแล้ว:

<span class="filename">ชื่อไฟล์: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

ตอนนี้ตัวแปร `x` เป็น immutable และคำเตือนจะไม่ปรากฏอีกต่อไป

คุณยังสามารถใช้คำสั่ง `cargo fix` เพื่อเปลี่ยนโค้ดของคุณระหว่าง Rust editions ต่างๆ ได้ Editions จะกล่าวถึงใน [ภาคผนวก E][editions]

### Lints เพิ่มเติมด้วย Clippy

เครื่องมือ Clippy คือชุดของ lints เพื่อวิเคราะห์โค้ดของคุณเพื่อให้คุณสามารถตรวจจับข้อผิดพลาดทั่วไปและปรับปรุงโค้ด Rust ของคุณได้ Clippy รวมอยู่ในการติดตั้ง Rust มาตรฐาน

หากต้องการรัน lints ของ Clippy บนโปรเจกต์ Cargo ใดๆ ให้ป้อนคำสั่งต่อไปนี้:

```console
$ cargo clippy
```

ตัวอย่างเช่น สมมติว่าคุณเขียนโปรแกรมที่ใช้ค่าประมาณของค่าคงที่ทางคณิตศาสตร์ เช่น ค่าพาย ดังโปรแกรมนี้:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

การรัน `cargo clippy` บนโปรเจกต์นี้จะส่งผลให้เกิดข้อผิดพลาดนี้:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

ข้อผิดพลาดนี้แจ้งให้คุณทราบว่า Rust มีค่าคงที่ `PI` ที่แม่นยำกว่าอยู่แล้ว และโปรแกรมของคุณจะถูกต้องมากขึ้นหากคุณใช้ค่าคงที่นั้นแทน จากนั้นคุณจะต้องเปลี่ยนโค้ดของคุณเพื่อใช้ค่าคงที่ `PI` โค้ดต่อไปนี้จะไม่ส่งผลให้เกิดข้อผิดพลาดหรือคำเตือนใดๆ จาก Clippy:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

สำหรับข้อมูลเพิ่มเติมเกี่ยวกับ Clippy โปรดดู [เอกสารประกอบ][clippy]

[clippy]: https://github.com/rust-lang/rust-clippy

### การผสานรวม IDE โดยใช้ `rust-analyzer`

เพื่อช่วยในการผสานรวม IDE ชุมชน Rust แนะนำให้ใช้ [`rust-analyzer`][rust-analyzer]<!-- ignore --> เครื่องมือนี้คือชุดของยูทิลิตี้ที่เน้นคอมไพเลอร์ซึ่งสื่อสารด้วย [Language Server Protocol][lsp]<!-- ignore --> ซึ่งเป็นข้อกำหนดสำหรับ IDE และภาษาโปรแกรมในการสื่อสารระหว่างกัน ไคลเอ็นต์ต่างๆ สามารถใช้ `rust-analyzer` ได้ เช่น [ปลั๊กอิน Rust analyzer สำหรับ Visual Studio Code][vscode]

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

ไปที่ [หน้าแรก][rust-analyzer]<!-- ignore --> ของโปรเจกต์ `rust-analyzer` สำหรับคำแนะนำในการติดตั้ง จากนั้นติดตั้งการสนับสนุน language server ใน IDE เฉพาะของคุณ IDE ของคุณจะได้รับความสามารถต่างๆ เช่น การเติมโค้ดอัตโนมัติ การข้ามไปยังคำจำกัดความ และข้อผิดพลาดแบบอินไลน์

[rust-analyzer]: https://rust-analyzer.github.io
[editions]: appendix-05-editions.md
