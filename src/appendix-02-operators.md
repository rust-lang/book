## Ek B: Operatörler ve Semboller

Bu ek, Rust'ın sözdiziminin bir sözlüğünü içerir, operatörler ve
kendi başlarına veya yollar, jenerikler bağlamında görünen diğer semboller,
özellik sınırları, makrolar, nitelikler, yorumlar, tuple'lar ve parantezler.

### Operatörler

Tablo B-1, Rust'taki operatörleri ve operatörün nasıl kullanılacağına dair bir örneği içerir
bağlam içinde görünüp görünmediği, kısa bir açıklama ve bu operatörün
aşırı yüklenebilir. Eğer bir operatör aşırı yüklenebilir ise, ilgili özellik
operatörünün listelendiği aşırı yük.
3

<span class="caption">Table B-1: Operators</span>

| Operatör                  | Örnek                                                 | Açıklama                                                           | Aşırı yüklenebilir?  |
| ------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------- | -------------- |
| `!`                       | `ident!(...)`, `ident!{...}`, `ident![...]`             | Makro genişleme                                                       |                |
| `!`                       | `!expr`                                                 | Bitsel veya mantıksal tümleyen                                       | `Not`          |
| `!=`                      | `expr != expr`                                          | Kalite karşılaştırması yok                                               | `PartialEq`    |
| `%`                       | `expr % expr`                                           | Aritmetik kalan                                                | `Rem`          |
| `%=`                      | `var %= expr`                                           | Aritmetik kalan ve atama                                  | `RemAssign`    |
| `&`                       | `&expr`, `&mut expr`                                    | Ödünç almak                                                                |                |
| `&`                       | `&type`, `&mut type`, `&'a type`, `&'a mut type`        | Ödünç alınmış işaretçi türü                                               |                |
| `&`                       | `expr & expr`                                           | Bitsel AND                                                           | `BitAnd`       |
| `&=`                      | `var &= expr`                                           | Bitsel AND ve atama                                          | `BitAndAssign` |
| `&&`                      | `expr && expr`                                          | Kısa devre yapan mantıksal AND                                       |                |
| `*`                       | `expr * expr`                                           | Aritmetik çarpma                                            | `Mul`          |
| `*=`                      | `var *= expr`                                           | Aritmetik çarpma ve atama                              | `MulAssign`    |
| `*`                       | `*expr`                                                 | Dereferans                                                           | `Deref`        |
| `*`                       | `*const type`, `*mut type`                              | Ham işaretçi                                                          |                |
| `+`                       | `trait + trait`, `'a + trait`                           | Bileşik tip kısıtlaması                                              |                |
| `+`                       | `expr + expr`                                           | Aritmetik toplama                                                 | `Add`          |
| `+=`                      | `var += expr`                                           | Aritmetik toplama ve atama                                  | `AddAssign`    |
| `,`                       | `expr, expr`                                            | Argüman ve eleman ayırıcı                                      |                |
| `-`                       | `- expr`                                                | Aritmetik olumsuzlama                                                 | `Neg`          |
| `-`                       | `expr - expr`                                           | Aritmetik çıkarma                                             | `Sub`          |
| `-=`                      | `var -= expr`                                           | Aritmetik çıkarma ve atama                               | `SubAssign`    |
| `->`                      | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | İşlev ve kapanış dönüş türü                               |                |
| `.`                       | `expr.ident`                                            | Saha erişimi                                                   |                |
| `.`                       | `expr.ident(expr, ...)`                                 | Metot çağrısı                                                          |                |
| `.`                       | `expr.0`, `expr.1`, etc.                                | Tuple indeksleme                                                        |                |
| `..`                      | `..`, `expr..`, `..expr`, `expr..expr`                  | Sağa özel gerçek aralık                                         | `PartialOrd`   |
| `..=`                     | `..=expr`, `expr..=expr`                                | Doğru kapsayıcı gerçek aralık                                         | `PartialOrd`   |
| `..`                      | `..expr`                                                | Struct literal güncelleme sözdizimi                                          |                |
| `..`                      | `variant(x, ..)`, `struct_type { x, .. }`               | “Ve geri kalanı” desen bağlama                                        |                |
| `...`                     | `expr...expr`                                           | (Kullanımdan kaldırılmıştır, bunun yerine `..=` kullanın) Kalıp içinde: kapsayıcı aralık kalıbı |                |
| `/`                       | `expr / expr`                                           | Aritmetik bölme                                                   | `Div`          |
| `/=`                      | `var /= expr`                                           | Aritmetik bölme ve atama                                    | `DivAssign`    |
| `:`                       | `pat: type`, `ident: type`                              | Kısıtlamalar                                                           |                |
| `:`                       | `ident: expr`                                           | Struct alan başlatıcısı                                              |                |
| `:`                       | `'a: loop {...}`                                        | Döngü etiketi                                                           |                |
| `;`                       | `expr;`                                                 | Açıklama ve öğe sonlandırıcı                                         |                |
| `;`                       | `[...; len]`                                            | Sabit boyutlu dizi sözdiziminin bir parçası                                      |                |
| `<<`                      | `expr << expr`                                          | Sola kaydırma                                                            | `Shl`          |
| `<<=`                     | `var <<= expr`                                          | Sola kaydırma ve atama                                           | `ShlAssign`    |
| `<`                       | `expr < expr`                                           | Karşılaştırmadan daha az                                                  | `PartialOrd`   |
| `<=`                      | `expr <= expr`                                          | Karşılaştırmaya eşit veya daha az                                      | `PartialOrd`   |
| `=`                       | `var = expr`, `ident = type`                            | Atama/eşdeğerlik                                                |                |
| `==`                      | `expr == expr`                                          | Eşitlik karşılaştırması                                                   | `PartialEq`    |
| `=>`                      | `pat => expr`                                           | Maç kolu sözdiziminin bir parçası                                              |                |
| `>`                       | `expr > expr`                                           | Karşılaştırmadan daha büyük                                               | `PartialOrd`   |
| `>=`                      | `expr >= expr`                                          | Karşılaştırmadan büyük veya eşit                                   | `PartialOrd`   |
| `>>`                      | `expr >> expr`                                          | Sağa kaydırma                                                          | `Shr`          |
| `>>=`                     | `var >>= expr`                                          | Sağa kaydırma ve atama                                            | `ShrAssign`    |
| `@`                       | `ident @ pat`                                           | Desen bağlama                                                       |                |
| `^`                       | `expr ^ expr`                                           | Bitsel özel OR                                                 | `BitXor`       |
| `^=`                      | `var ^= expr`                                           | Bitsel özel OR ve atama                                  | `BitXorAssign` |
| <code>&vert;</code>       | <code>pat &vert; pat</code>                             | Desen alternatifleri                                                 |                |
| <code>&vert;</code>       | <code>expr &vert; expr</code>                           | Bitsel OR                                                     | `BitOr`        |
| <code>&vert;=</code>      | <code>var &vert;= expr</code>                           | Bitsel OR ve atama                                             | `BitOrAssign`  |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code>                     | Kısa devre yapan mantıksal OR                                           |                |
| `?`                       | `expr?`                                                 | Hata yayılımı                                                  |                |

### Operatör Olmayan Semboller

Aşağıdaki liste, operatör olarak işlev görmeyen tüm sembolleri içerir; yani
Yani, bir işlev veya yöntem çağrısı gibi davranmazlar.

Tablo B-2, kendi başlarına görünen ve çeşitli durumlarda geçerli olan sembolleri gösterir
yerler.

<span class="caption">Table B-2: Stand-Alone Syntax</span>

| Sembol                                        | Açıklama                                                            |
| --------------------------------------------- | ---------------------------------------------------------------------- |
| `'ident`                                      | Adlandırılmış ömür veya döngü etiketi                                       |
| `...u8`, `...i32`, `...f64`, `...usize`, etc. | Belirli türde sayısal değişmez                                    |
| `"..."`                                       | String literal                                                       |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc.      | Ham dize değişmezi, kaçış karakterleri işlenmez               |
| `b"..."`                                      | Bayt dizesi değişmezi; dize yerine bir bayt dizisi oluşturur  |
| `br"..."`, `br#"..."#`, `br##"..."##`, etc.   | Ham bayt dizesi değişmezi, ham ve bayt dizesi değişmezi kombinasyonu    |
| `'...'`                                       | Gerçek karakter                                                      |
| `b'...'`                                      | ASCII bayt değişmezi                                               |
| <code>&vert;...&vert; expr</code>             | Kapanış                                                                |
| `!`                                           | Ayrışan fonksiyonlar için daima boş alt tip                     |
| `_`                                           | “Ignored” kalıp bağlama; tamsayı değişmezlerini okunabilir hale getirmek için de kullanılır |

Tablo B-3, modül boyunca bir yol bağlamında görünen sembolleri göstermektedir
bir öğeye hiyerarşi.

<span class="caption">Table B-3: Path-Related Syntax</span>

| Sembol                                  | Açıklama                                                                                                                     |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `ident::ident`                          | Ad alanı yolu                                                                                                    |
| `::path`                                | Diğer tüm sandıkların köklendiği extern prelude'a göreli yol (yani, sandık adını içeren açıkça mutlak bir yol) |
| `self::path`                            | Geçerli modüle göreli yol (yani, açıkça göreli bir yol).                                                        |
| `super::path`                           | Geçerli modülün üst öğesine göre yol                                                                              |
| `type::ident`, `<type as trait>::ident` | İlişkili sabitler, fonksiyonlar ve tipler                                                                                      |
| `<type>::...`                           | Doğrudan adlandırılamayan bir tür için ilişkilendirilmiş öğe (örneğin, `<&T>::...`, `<[T]>::...`, vb.)                               |
| `trait::method(...)`                    | Bir yöntem çağrısını, onu tanımlayan özelliği adlandırarak belirsizleştirmek                                                              |
| `type::method(...)`                     | Tanımlandığı türü adlandırarak bir yöntem çağrısının belirsizliğini giderme                                                        |
| `<type as trait>::method(...)`          | Özellik ve türü adlandırarak bir yöntem çağrısının belirsizliğini giderme                                                                      |

Tablo B-4, genel tip kullanımı bağlamında görünen sembolleri göstermektedir
Parametreler.

<span class="caption">Table B-4: Generics</span>

| Sembol                         | Açıklama                                                                                                                              |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `path<...>`                    | Bir tipteki genel tip için parametreleri belirtir (örn,`Vec<u8>`)                                                                         |
| `path::<...>`, `method::<...>` | Bir ifadedeki genel tür, işlev veya yöntem için parametreleri belirtir; genellikle turbofish olarak adlandırılır (örneğin, `“42”.parse::<i32>()`) |
| `fn ident<...> ...`            | Genel işlev tanımlama                                                                                                             |
| `struct ident<...> ...`        | Genel yapıyı tanımlayın                                                                                                          |
| `enum ident<...> ...`          | Genel numaralandırmayı tanımlama                                                                                                          |
| `impl<...> ...`                | Genel uygulamayı tanımlayın                                                                                                          |
| `for<...> type`                | Daha yüksek sıralı yaşam süresi sınırları                                                                                                       |
| `type<ident=type>`             | Bir veya daha fazla ilişkili tipin belirli atamalara sahip olduğu genel bir tip (örneğin, `Iterator<Item=T>`)                                |

Tablo B-5, genel tipin kısıtlanması bağlamında görünen sembolleri göstermektedir
özellik sınırlarına sahip parametreler.

<span class="caption">Table B-5: Trait Bound Constraints</span>

| Symbol                        | Explanation                                                                                                                                |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `T: U`                        | Genel parametre `T`, `U` uygulayan türlerle sınırlandırılmıştır                                                                       |
| `T: 'a`                       | Genel `T` türü `a` yaşam süresinden daha uzun ömürlü olmalıdır (yani tür, `a` dan daha kısa yaşam sürelerine sahip herhangi bir referansı geçişli olarak içeremez) |
| `T: 'static`                  | `T` jenerik tipi `'static` olanlar dışında ödünç alınmış referanslar içermez                                    |
| `'b: 'a`                      | Genel yaşam süresi `b`, yaşam süresi `a` dan uzun olmalıdır                                                                                    |
| `T: ?Sized`                   | Genel tip parametresinin dinamik olarak boyutlandırılmış bir tip olmasına izin ver                                                                        |
| `'a + trait`, `trait + trait` | Bileşik tip kısıtlaması                                                                                                                  |

Tablo B-6, çağrı veya tanımlama bağlamında görünen sembolleri göstermektedir
makroları ve bir öğe üzerinde öznitelik belirtme.

<span class="caption">Table B-6: Macros and Attributes</span>

| Sembol                                      | Açıklama        |
| ------------------------------------------- | ------------------ |
| `#[meta]`                                   | Dış nitelik   |
| `#![meta]`                                  | İç nitelik   |
| `$ident`                                    | Makro ikamesi |
| `$ident:kind`                               | Makro çekim      |
| `$(…)…`                                     | Makro tekrarı  |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Makro çağırma   |

Tablo B-7 yorum oluşturan sembolleri göstermektedir.

<span class="caption">Table B-7: Comments</span>

| Sembol     | Açıklama             |
| ---------- | ----------------------- |
| `//`       | Hat kommenlerit            |
| `//!`      | İç satır doküman yorumu  |
| `///`      | Dış satır doküman yorumu  |
| `/*...*/`  | Blok yorumu           |
| `/*!...*/` | İç blok doküman yorumu |
| `/**...*/` | Dış blok doküman yorumu |

Tablo B-8'de parantezlerin kullanıldığı bağlamlar gösterilmektedir.

<span class="caption">Table B-8: Parentheses</span>

| Sembol                   | Açıklama                                                                                 |
| ------------------------ | ------------------------------------------------------------------------------------------- |
| `()`                     | Boş tuple (diğer adıyla birim), hem gerçek hem de tip                                             |
| `(expr)`                 | Parantezli ifade                                                                   |
| `(expr,)`                | Tek öğeli tuple ifadesi                                                             |
| `(type,)`                | Tek elemanlı tuple tipi                                                                   |
| `(expr, ...)`            | Tuple ifade                                                                            |
| `(type, ...)`            | Tuple ifade                                                                                 |
| `expr(expr, ...)`        | Fonksiyon çağrı ifadesi; tuple `struct` ve tuple `enum` varyantlarını başlatmak için de kullanılır |

Tablo B-9, küme parantezlerinin kullanıldığı bağlamları göstermektedir.

<span class="caption">Table B-9: Curly Brackets</span>

| Bağlam      | Açıklama      |
| ------------ | ---------------- |
| `{...}`      | Blok ifade |
| `Type {...}` | `struct` literal |

Tablo B-10 köşeli parantezlerin kullanıldığı bağlamları göstermektedir.

<span class="caption">Table B-10: Square Brackets</span>

| Bağlam                                            | cAçıklama                                                                                                                   |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `[...]`                                            | Gerçek dizi                                                                                                              |
| `[expr; len]`                                      | expr`nin `len` kopyalarını içeren dizi değişmezi                                                                               |
| `[type; len]`                                      | type` türünün `len` örneklerini içeren dizi türü                                                                               |
| `expr[expr]`                                       | Koleksiyon indeksleme. Aşırı yüklenebilir (`Index`, `IndexMut`)                                                                      |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Koleksiyon dilimleme gibi davranan koleksiyon indeksleme, “indeks” olarak `Range`, `RangeFrom`, `RangeTo` veya `RangeFull` kullanır |
