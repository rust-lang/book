## ภาคผนวก A: Keywords (คำสงวน)

รายการต่อไปนี้ประกอบด้วยคำสงวน (keywords) ที่ถูกจองไว้สำหรับการใช้งานในปัจจุบันหรือในอนาคตโดยภาษา Rust ด้วยเหตุนี้ คำเหล่านี้จึงไม่สามารถใช้เป็นตัวระบุ (identifiers) ได้ (ยกเว้นการใช้เป็น raw identifiers ตามที่เราจะกล่าวถึงในส่วน “[Raw Identifiers][raw-identifiers]<!-- ignore -->”) ตัวระบุคือชื่อของฟังก์ชัน, ตัวแปร, พารามิเตอร์, ฟิลด์ของ struct, โมดูล, крейт, ค่าคงที่, มาโคร, ค่า static, แอททริบิวต์, ไทป์, เทรต, หรือไลฟ์ไทม์

[raw-identifiers]: #raw-identifiers

### Keywords ที่ใช้งานอยู่ในปัจจุบัน

ต่อไปนี้คือรายการของ keywords ที่ใช้งานอยู่ในปัจจุบัน พร้อมคำอธิบายฟังก์ชันการทำงาน:

- `as` - ทำการแปลงชนิดข้อมูลพื้นฐาน (primitive casting), ชี้ชัดเทรต (trait) ที่มีไอเท็มนั้นอยู่, หรือเปลี่ยนชื่อไอเท็มในคำสั่ง `use`
- `async` - คืนค่า `Future` แทนที่จะบล็อกเธรดปัจจุบัน
- `await` - หยุดการทำงานชั่วคราวจนกว่าผลลัพธ์ของ `Future` จะพร้อมใช้งาน
- `break` - ออกจากลูปทันที
- `const` - กำหนดไอเท็มค่าคงที่ (constant items) หรือพอยน์เตอร์ดิบค่าคงที่ (constant raw pointers)
- `continue` - ไปยังการวนซ้ำรอบถัดไปของลูป
- `crate` - ในเส้นทางโมดูล (module path) อ้างอิงถึงรากของ крейт (crate root)
- `dyn` - การจัดส่งแบบไดนามิก (dynamic dispatch) ไปยัง trait object
- `else` - ส่วนสำรองสำหรับโครงสร้างควบคุมการทำงาน `if` และ `if let`
- `enum` - กำหนด enumeration (การแจงนับ)
- `extern` - เชื่อมโยงฟังก์ชันหรือตัวแปรภายนอก
- `false` - ค่าตรรกะเท็จ (Boolean false literal)
- `fn` - กำหนดฟังก์ชันหรือชนิดของฟังก์ชันพอยน์เตอร์ (function pointer type)
- `for` - วนซ้ำไอเท็มจาก iterator, υλοποιώ (implement) เทรต, หรือระบุ higher-ranked lifetime
- `if` - แตกเงื่อนไขตามผลลัพธ์ของนิพจน์เงื่อนไข (conditional expression)
- `impl` - υλοποιώ (implement) ฟังก์ชันการทำงานแบบ inheren หรือของเทรต
- `in` - ส่วนหนึ่งของ синтаксис `for` loop
- `let` - กำหนดค่าให้กับตัวแปร
- `loop` - วนซ้ำอย่างไม่มีเงื่อนไข
- `match` - จับคู่ค่ากับรูปแบบ (patterns)
- `mod` - กำหนดโมดูล
- `move` - ทำให้ closure ครอบครอง (take ownership) captures ทั้งหมดของมัน
- `mut` - ระบุความสามารถในการเปลี่ยนแปลงค่า (mutability) ในการอ้างอิง, พอยน์เตอร์ดิบ, หรือการผูกรูปแบบ (pattern bindings)
- `pub` - ระบุการมองเห็นแบบสาธารณะ (public visibility) ในฟิลด์ของ struct, บล็อก `impl`, หรือโมดูล
- `ref` - ผูกโดยการอ้างอิง (bind by reference)
- `return` - คืนค่าออกจากฟังก์ชัน
- `Self` - ชื่อแทนไทป์ (type alias) สำหรับไทป์ที่เรากำลังกำหนดหรือ υλοποιώ (implement)
- `self` - ประธานของเมธอด (method subject) หรือโมดูลปัจจุบัน
- `static` - ตัวแปรโกลบอล (global variable) หรือ lifetime ที่คงอยู่ตลอดการทำงานของโปรแกรม
- `struct` - กำหนดโครงสร้าง (structure)
- `super` - โมดูลแม่ (parent module) ของโมดูลปัจจุบัน
- `trait` - กำหนดเทรต (trait)
- `true` - ค่าตรรกะจริง (Boolean true literal)
- `type` - กำหนดชื่อแทนไทป์ (type alias) หรือ associated type
- `union` - กำหนด [union][union]<!-- ignore -->; เป็น keyword ก็ต่อเมื่อใช้ในการประกาศ union เท่านั้น
- `unsafe` - ระบุโค้ด, ฟังก์ชัน, เทรต, หรือการ υλοποιώ (implementations) ที่ไม่ปลอดภัย (unsafe)
- `use` - นำสัญลักษณ์ (symbols) เข้ามาในขอบเขต (scope); ระบุ captures ที่แม่นยำสำหรับ generic และ lifetime bounds
- `where` - ระบุ clauses ที่จำกัดไทป์
- `while` - วนซ้ำตามเงื่อนไขตามผลลัพธ์ของนิพจน์

[union]: ../reference/items/unions.html

### Keywords ที่สงวนไว้สำหรับอนาคต

Keywords ต่อไปนี้ยังไม่มีฟังก์ชันการทำงานใดๆ แต่ถูกสงวนไว้โดย Rust สำหรับการใช้งานที่อาจเกิดขึ้นในอนาคต:

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `gen`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

### Raw Identifiers

_Raw identifiers_ คือ синтаксис ที่ช่วยให้คุณสามารถใช้ keywords ในตำแหน่งที่ไม่ได้รับอนุญาตตามปกติ คุณใช้ raw identifier โดยการเติม `r#` หน้า keyword

ตัวอย่างเช่น `match` เป็น keyword หากคุณพยายามคอมไพล์ฟังก์ชันต่อไปนี้ที่ใช้ `match` เป็นชื่อ:

<span class="filename">ชื่อไฟล์: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

คุณจะได้รับข้อผิดพลาดนี้:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

ข้อผิดพลาดแสดงให้เห็นว่าคุณไม่สามารถใช้ keyword `match` เป็นชื่อฟังก์ชันได้ หากต้องการใช้ `match` เป็นชื่อฟังก์ชัน คุณต้องใช้ синтаксис raw identifier ดังนี้:

<span class="filename">ชื่อไฟล์: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

โค้ดนี้จะคอมไพล์ได้โดยไม่มีข้อผิดพลาด สังเกตคำนำหน้า `r#` ที่ชื่อฟังก์ชันทั้งในส่วนการกำหนดและในส่วนที่เรียกใช้ฟังก์ชันใน `main`

Raw identifiers ช่วยให้คุณสามารถใช้คำใดก็ได้ที่คุณเลือกเป็นตัวระบุ แม้ว่าคำนั้นจะเป็น keyword ที่สงวนไว้ก็ตาม สิ่งนี้ให้อิสระแก่เรามากขึ้นในการเลือกชื่อตัวระบุ รวมถึงช่วยให้เราสามารถผสานรวมกับโปรแกรมที่เขียนด้วยภาษาอื่นซึ่งคำเหล่านี้ไม่ใช่ keywords นอกจากนี้ raw identifiers ยังช่วยให้คุณสามารถใช้ไลบรารีที่เขียนด้วย Rust edition ที่แตกต่างจากที่ крейт ของคุณใช้ ตัวอย่างเช่น `try` ไม่ใช่ keyword ใน edition 2015 แต่เป็น keyword ใน edition 2018, 2021 และ 2024 หากคุณขึ้นอยู่กับไลบรารีที่เขียนโดยใช้ edition 2015 และมีฟังก์ชัน `try` คุณจะต้องใช้ синтаксис raw identifier `r#try` ในกรณีนี้ เพื่อเรียกใช้ฟังก์ชันนั้นจากโค้ดของคุณใน edition ที่ใหม่กว่า ดู [ภาคผนวก E][appendix-e]<!-- ignore --> สำหรับข้อมูลเพิ่มเติมเกี่ยวกับ editions

[appendix-e]: appendix-05-editions.html
