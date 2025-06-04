## ภาคผนวก B: Operators (ตัวดำเนินการ) และ Symbols (สัญลักษณ์)

ภาคผนวกนี้ประกอบด้วยอภิธานศัพท์ของ синтаксис Rust รวมถึงตัวดำเนินการและสัญลักษณ์อื่นๆ ที่ปรากฏด้วยตัวเองหรือในบริบทของ paths, generics, trait bounds, macros, attributes, comments, tuples, และวงเล็บ

### Operators (ตัวดำเนินการ)

ตาราง B-1 ประกอบด้วยตัวดำเนินการใน Rust, ตัวอย่างการปรากฏของตัวดำเนินการในบริบท, คำอธิบายสั้นๆ, และระบุว่าตัวดำเนินการนั้นสามารถ overload ได้หรือไม่ หากตัวดำเนินการสามารถ overload ได้ จะมีการระบุ trait ที่เกี่ยวข้องซึ่งใช้ในการ overload ตัวดำเนินการนั้น

<span class="caption">ตาราง B-1: ตัวดำเนินการ</span>

| ตัวดำเนินการ             | ตัวอย่าง                                                | คำอธิบาย                                                              | Overload ได้หรือไม่? |
| ------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------- | -------------- |
| `!`                       | `ident!(...)`, `ident!{...}`, `ident![...]`             | การขยาย Macro (Macro expansion)                                       |                |
| `!`                       | `!expr`                                                 | ส่วนกลับทางบิตไวส์หรือตรรกะ (Bitwise or logical complement)            | `Not`          |
| `!=`                      | `expr != expr`                                          | การเปรียบเทียบแบบไม่เท่ากัน (Nonequality comparison)                   | `PartialEq`    |
| `%`                       | `expr % expr`                                           | เศษเหลือจากการหาร (Arithmetic remainder)                               | `Rem`          |
| `%=`                      | `var %= expr`                                           | เศษเหลือจากการหารและการกำหนดค่า (Arithmetic remainder and assignment) | `RemAssign`    |
| `&`                       | `&expr`, `&mut expr`                                    | การยืม (Borrow)                                                       |                |
| `&`                       | `&type`, `&mut type`, `&'a type`, `&'a mut type`        | ชนิดพอยน์เตอร์ที่ถูกยืม (Borrowed pointer type)                        |                |
| `&`                       | `expr & expr`                                           | AND ทางบิตไวส์ (Bitwise AND)                                           | `BitAnd`       |
| `&=`                      | `var &= expr`                                           | AND ทางบิตไวส์และการกำหนดค่า (Bitwise AND and assignment)            | `BitAndAssign` |
| `&&`                      | `expr && expr`                                          | AND ตรรกะแบบ Short-circuiting (Short-circuiting logical AND)         |                |
| `*`                       | `expr * expr`                                           | การคูณ (Arithmetic multiplication)                                     | `Mul`          |
| `*=`                      | `var *= expr`                                           | การคูณและการกำหนดค่า (Arithmetic multiplication and assignment)        | `MulAssign`    |
| `*`                       | `*expr`                                                 | การ Dereference                                                       | `Deref`        |
| `*`                       | `*const type`, `*mut type`                              | พอยน์เตอร์ดิบ (Raw pointer)                                            |                |
| `+`                       | `trait + trait`, `'a + trait`                           | ข้อจำกัดชนิดแบบผสม (Compound type constraint)                         |                |
| `+`                       | `expr + expr`                                           | การบวก (Arithmetic addition)                                          | `Add`          |
| `+=`                      | `var += expr`                                           | การบวกและการกำหนดค่า (Arithmetic addition and assignment)             | `AddAssign`    |
| `,`                       | `expr, expr`                                            | ตัวคั่นอาร์กิวเมนต์และอิลิเมนต์ (Argument and element separator)       |                |
| `-`                       | `- expr`                                                | นิเสธ (Arithmetic negation)                                           | `Neg`          |
| `-`                       | `expr - expr`                                           | การลบ (Arithmetic subtraction)                                        | `Sub`          |
| `-=`                      | `var -= expr`                                           | การลบและการกำหนดค่า (Arithmetic subtraction and assignment)           | `SubAssign`    |
| `->`                      | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | ชนิดข้อมูลส่งคืนของฟังก์ชันและ closure (Function and closure return type) |                |
| `.`                       | `expr.ident`                                            | การเข้าถึงฟิลด์ (Field access)                                         |                |
| `.`                       | `expr.ident(expr, ...)`                                 | การเรียกเมธอด (Method call)                                           |                |
| `.`                       | `expr.0`, `expr.1`, etc.                                | การเข้าถึงสมาชิก Tuple (Tuple indexing)                               |                |
| `..`                      | `..`, `expr..`, `..expr`, `expr..expr`                  | ช่วงค่าแบบไม่รวมค่าขวา (Right-exclusive range literal)                | `PartialOrd`   |
| `..=`                     | `..=expr`, `expr..=expr`                                | ช่วงค่าแบบรวมค่าขวา (Right-inclusive range literal)                   | `PartialOrd`   |
| `..`                      | `..expr`                                                | синтаксисอัปเดต Struct literal (Struct literal update syntax)         |                |
| `..`                      | `variant(x, ..)`, `struct_type { x, .. }`               | การผูกรูปแบบ “และส่วนที่เหลือ” (“And the rest” pattern binding)       |                |
| `...`                     | `expr...expr`                                           | (เลิกใช้แล้ว, ให้ใช้ `..=` แทน) ในรูปแบบ: รูปแบบช่วงค่าแบบรวม       |                |
| `/`                       | `expr / expr`                                           | การหาร (Arithmetic division)                                          | `Div`          |
| `/=`                      | `var /= expr`                                           | การหารและการกำหนดค่า (Arithmetic division and assignment)              | `DivAssign`    |
| `:`                       | `pat: type`, `ident: type`                              | ข้อจำกัด (Constraints)                                                |                |
| `:`                       | `ident: expr`                                           | การกำหนดค่าเริ่มต้นฟิลด์ของ Struct (Struct field initializer)         |                |
| `:`                       | `'a: loop {...}`                                        | ป้ายกำกับลูป (Loop label)                                              |                |
| `;`                       | `expr;`                                                 | ตัวสิ้นสุดคำสั่งและไอเท็ม (Statement and item terminator)             |                |
| `;`                       | `[...; len]`                                            | ส่วนหนึ่งของ синтаксисอาร์เรย์ขนาดคงที่ (fixed-size array syntax)      |                |
| `<<`                      | `expr << expr`                                          | เลื่อนบิตไปทางซ้าย (Left-shift)                                       | `Shl`          |
| `<<=`                     | `var <<= expr`                                          | เลื่อนบิตไปทางซ้ายและการกำหนดค่า (Left-shift and assignment)           | `ShlAssign`    |
| `<`                       | `expr < expr`                                           | การเปรียบเทียบน้อยกว่า (Less than comparison)                         | `PartialOrd`   |
| `<=`                      | `expr <= expr`                                          | การเปรียบเทียบน้อยกว่าหรือเท่ากับ (Less than or equal to comparison)   | `PartialOrd`   |
| `=`                       | `var = expr`, `ident = type`                            | การกำหนดค่า/ความเท่ากัน (Assignment/equivalence)                      |                |
| `==`                      | `expr == expr`                                          | การเปรียบเทียบความเท่ากัน (Equality comparison)                       | `PartialEq`    |
| `=>`                      | `pat => expr`                                           | ส่วนหนึ่งของ синтаксисแขนง match (match arm syntax)                     |                |
| `>`                       | `expr > expr`                                           | การเปรียบเทียบมากกว่า (Greater than comparison)                       | `PartialOrd`   |
| `>=`                      | `expr >= expr`                                          | การเปรียบเทียบมากกว่าหรือเท่ากับ (Greater than or equal to comparison) | `PartialOrd`   |
| `>>`                      | `expr >> expr`                                          | เลื่อนบิตไปทางขวา (Right-shift)                                       | `Shr`          |
| `>>=`                     | `var >>= expr`                                          | เลื่อนบิตไปทางขวาและการกำหนดค่า (Right-shift and assignment)          | `ShrAssign`    |
| `@`                       | `ident @ pat`                                           | การผูกรูปแบบ (Pattern binding)                                        |                |
| `^`                       | `expr ^ expr`                                           | OR แบบเฉพาะทางบิตไวส์ (Bitwise exclusive OR)                          | `BitXor`       |
| `^=`                      | `var ^= expr`                                           | OR แบบเฉพาะทางบิตไวส์และการกำหนดค่า (Bitwise exclusive OR and assignment) | `BitXorAssign` |
| <code>&vert;</code>       | <code>pat &vert; pat</code>                             | ทางเลือกของรูปแบบ (Pattern alternatives)                               |                |
| <code>&vert;</code>       | <code>expr &vert; expr</code>                           | OR ทางบิตไวส์ (Bitwise OR)                                             | `BitOr`        |
| <code>&vert;=</code>      | <code>var &vert;= expr</code>                           | OR ทางบิตไวส์และการกำหนดค่า (Bitwise OR and assignment)               | `BitOrAssign`  |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code>                     | OR ตรรกะแบบ Short-circuiting (Short-circuiting logical OR)            |                |
| `?`                       | `expr?`                                                 | การส่งต่อข้อผิดพลาด (Error propagation)                               |                |

### Non-operator Symbols (สัญลักษณ์ที่ไม่ใช่ตัวดำเนินการ)

รายการต่อไปนี้ประกอบด้วยสัญลักษณ์ทั้งหมดที่ไม่ทำหน้าที่เป็นตัวดำเนินการ นั่นคือ สัญลักษณ์เหล่านี้ไม่ได้ทำงานเหมือนการเรียกฟังก์ชันหรือเมธอด

ตาราง B-2 แสดงสัญลักษณ์ที่ปรากฏด้วยตัวเองและสามารถใช้งานได้ในหลากหลายตำแหน่ง

<span class="caption">ตาราง B-2: синтаксисแบบสแตนด์อโลน</span>

| สัญลักษณ์                                    | คำอธิบาย                                                                 |
| --------------------------------------------- | ---------------------------------------------------------------------- |
| `'ident`                                      | lifetime หรือ loop label ที่มีชื่อ                                      |
| `...u8`, `...i32`, `...f64`, `...usize`, etc. | เลขลิเทอรัลของชนิดข้อมูลเฉพาะ (Numeric literal of specific type)       |
| `"..."`                                       | สตริงลิเทอรัล (String literal)                                         |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc.      | Raw string literal, ไม่มีการประมวลผล escape characters                 |
| `b"..."`                                      | Byte string literal; สร้างอาร์เรย์ของไบต์แทนที่จะเป็นสตริง             |
| `br"..."`, `br#"..."#`, `br##"..."##`, etc.   | Raw byte string literal, การรวมกันของ raw และ byte string literal      |
| `'...'`                                       | Character literal                                                      |
| `b'...'`                                      | ASCII byte literal                                                     |
| <code>&vert;...&vert; expr</code>             | Closure                                                                |
| `!`                                           | ชนิดข้อมูลว่างเปล่าเสมอ (Always empty bottom type) สำหรับฟังก์ชันที่ไม่มีการคืนค่า (diverging functions) |
| `_`                                           | การผูกรูปแบบที่ “ถูกละเว้น” (ignored” pattern binding); ยังใช้เพื่อทำให้อ่านเลขลิเทอรัลจำนวนเต็มได้ง่ายขึ้น |

ตาราง B-3 แสดงสัญลักษณ์ที่ปรากฏในบริบทของ path ผ่านลำดับชั้นของโมดูลไปยังไอเท็ม

<span class="caption">ตาราง B-3: синтаксисที่เกี่ยวข้องกับ Path</span>

| สัญลักษณ์                                  | คำอธิบาย                                                                                                                        |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `ident::ident`                          | Namespace path                                                                                                                  |
| `::path`                                | Path ที่สัมพันธ์กับ extern prelude ซึ่งเป็นที่ตั้งของ крейт อื่นๆ ทั้งหมด (เช่น path แบบสัมบูรณ์ที่ระบุชื่อ крейт อย่างชัดเจน) |
| `self::path`                            | Path ที่สัมพันธ์กับโมดูลปัจจุบัน (เช่น path แบบสัมพัทธ์ที่ระบุอย่างชัดเจน)                                                       |
| `super::path`                           | Path ที่สัมพันธ์กับโมดูลแม่ของโมดูลปัจจุบัน                                                                                       |
| `type::ident`, `<type as trait>::ident` | ค่าคงที่, ฟังก์ชัน, และไทป์ที่เกี่ยวข้อง (Associated constants, functions, and types)                                          |
| `<type>::...`                           | ไอเท็มที่เกี่ยวข้องสำหรับไทป์ที่ไม่สามารถตั้งชื่อได้โดยตรง (เช่น `<&T>::...`, `<[T]>::...`, etc.)                                |
| `trait::method(...)`                    | การชี้ชัดการเรียกเมธอดโดยการระบุชื่อ trait ที่กำหนดเมธอดนั้น                                                                     |
| `type::method(...)`                     | การชี้ชัดการเรียกเมธอดโดยการระบุชื่อไทป์ที่เมธอดนั้นถูกกำหนดไว้                                                                     |
| `<type as trait>::method(...)`          | การชี้ชัดการเรียกเมธอดโดยการระบุชื่อ trait และไทป์                                                                               |

ตาราง B-4 แสดงสัญลักษณ์ที่ปรากฏในบริบทของการใช้พารามิเตอร์ generic type

<span class="caption">ตาราง B-4: Generics</span>

| สัญลักษณ์                         | คำอธิบาย                                                                                                                                  |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `path<...>`                    | ระบุพารามิเตอร์ให้กับ generic type ในไทป์ (เช่น `Vec<u8>`)                                                                                 |
| `path::<...>`, `method::<...>` | ระบุพารามิเตอร์ให้กับ generic type, ฟังก์ชัน, หรือเมธอดในนิพจน์; มักเรียกว่า turbofish (เช่น `"42".parse::<i32>()`)                       |
| `fn ident<...> ...`            | กำหนดฟังก์ชัน generic                                                                                                                      |
| `struct ident<...> ...`        | กำหนด structure แบบ generic                                                                                                               |
| `enum ident<...> ...`          | กำหนด enumeration แบบ generic                                                                                                             |
| `impl<...> ...`                | กำหนด υλοποίηση (implementation) แบบ generic                                                                                                |
| `for<...> type`                | Higher-ranked lifetime bounds                                                                                                            |
| `type<ident=type>`             | Generic type ที่มี associated type อย่างน้อยหนึ่งรายการมีการกำหนดค่าเฉพาะ (เช่น `Iterator<Item=T>`)                                         |

ตาราง B-5 แสดงสัญลักษณ์ที่ปรากฏในบริบทของการจำกัดพารามิเตอร์ generic type ด้วย trait bounds

<span class="caption">ตาราง B-5: ข้อจำกัด Trait Bound</span>

| สัญลักษณ์                        | คำอธิบาย                                                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `T: U`                        | พารามิเตอร์ generic `T` ถูกจำกัดให้เป็นไทป์ที่ υλοποιώ (implement) `U`                                                                        |
| `T: 'a`                       | Generic type `T` ต้องมี lifetime ยาวนานกว่า `'a` (หมายความว่าไทป์นั้นไม่สามารถมีการอ้างอิงใดๆ ที่มี lifetime สั้นกว่า `'a` ได้โดยปริยาย) |
| `T: 'static`                  | Generic type `T` ไม่มี borrowed references อื่นใดนอกจาก `'static`                                                                          |
| `'b: 'a`                      | Generic lifetime `'b` ต้องมี lifetime ยาวนานกว่า `'a`                                                                                       |
| `T: ?Sized`                   | อนุญาตให้พารามิเตอร์ generic type เป็น dynamically sized type                                                                               |
| `'a + trait`, `trait + trait` | ข้อจำกัดชนิดแบบผสม (Compound type constraint)                                                                                              |

ตาราง B-6 แสดงสัญลักษณ์ที่ปรากฏในบริบทของการเรียกหรือกำหนด macros และการระบุ attributes บนไอเท็ม

<span class="caption">ตาราง B-6: Macros และ Attributes</span>

| สัญลักษณ์                                      | คำอธิบาย          |
| ------------------------------------------- | ------------------ |
| `#[meta]`                                   | Outer attribute    |
| `#![meta]`                                  | Inner attribute    |
| `$ident`                                    | การแทนที่ Macro (Macro substitution) |
| `$ident:kind`                               | Macro capture      |
| `$(…)…`                                     | การทำซ้ำ Macro (Macro repetition)   |
| `ident!(...)`, `ident!{...}`, `ident![...]` | การเรียก Macro (Macro invocation)   |

ตาราง B-7 แสดงสัญลักษณ์ที่สร้างคอมเมนต์

<span class="caption">ตาราง B-7: คอมเมนต์</span>

| สัญลักษณ์     | คำอธิบาย                |
| ---------- | ----------------------- |
| `//`       | คอมเมนต์แบบบรรทัด (Line comment)            |
| `//!`      | คอมเมนต์เอกสารภายในบรรทัด (Inner line doc comment)  |
| `///`      | คอมเมนต์เอกสารภายนอกบรรทัด (Outer line doc comment)  |
| `/*...*/`  | คอมเมนต์แบบบล็อก (Block comment)           |
| `/*!...*/` | คอมเมนต์เอกสารภายในบล็อก (Inner block doc comment) |
| `/**...*/` | คอมเมนต์เอกสารภายนอกบล็อก (Outer block doc comment) |

ตาราง B-8 แสดงบริบทที่มีการใช้วงเล็บ

<span class="caption">ตาราง B-8: วงเล็บ</span>

| สัญลักษณ์                   | คำอธิบาย                                                                                     |
| ------------------------ | ------------------------------------------------------------------------------------------- |
| `()`                     | Tuple ว่าง (หรือเรียกว่า unit), ทั้งแบบลิเทอรัลและแบบไทป์                                     |
| `(expr)`                 | นิพจน์ในวงเล็บ (Parenthesized expression)                                                     |
| `(expr,)`                | นิพจน์ Tuple แบบสมาชิกเดียว (Single-element tuple expression)                                 |
| `(type,)`                | ชนิด Tuple แบบสมาชิกเดียว (Single-element tuple type)                                         |
| `(expr, ...)`            | นิพจน์ Tuple (Tuple expression)                                                               |
| `(type, ...)`            | ชนิด Tuple (Tuple type)                                                                       |
| `expr(expr, ...)`        | นิพจน์การเรียกฟังก์ชัน; ยังใช้เพื่อกำหนดค่าเริ่มต้นให้กับ tuple `struct`s และ tuple `enum` variants |

ตาราง B-9 แสดงบริบทที่มีการใช้วงเล็บปีกกา

<span class="caption">ตาราง B-9: วงเล็บปีกกา</span>

| บริบท      | คำอธิบาย          |
| ------------ | ---------------- |
| `{...}`      | นิพจน์แบบบล็อก (Block expression) |
| `Type {...}` | `struct` literal |

ตาราง B-10 แสดงบริบทที่มีการใช้วงเล็บเหลี่ยม

<span class="caption">ตาราง B-10: วงเล็บเหลี่ยม</span>

| บริบท                                            | คำอธิบาย                                                                                                                       |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `[...]`                                            | อาร์เรย์ลิเทอรัล (Array literal)                                                                                               |
| `[expr; len]`                                      | อาร์เรย์ลิเทอรัลที่ประกอบด้วย `expr` จำนวน `len` ชุด                                                                          |
| `[type; len]`                                      | ชนิดอาร์เรย์ที่ประกอบด้วย `type` จำนวน `len` อินสแตนซ์                                                                         |
| `expr[expr]`                                       | การเข้าถึงสมาชิก Collection Overloadable (`Index`, `IndexMut`)                                                                 |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | การเข้าถึงสมาชิก Collection ที่ทำเหมือนการแบ่งส่วน Collection โดยใช้ `Range`, `RangeFrom`, `RangeTo`, หรือ `RangeFull` เป็น “index” |
