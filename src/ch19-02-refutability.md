## Refutability: Bir Desenin Eşleşememe İhtimali

Desenler iki biçimde gelir: refutable (eşleşemeyebilir) ve irrefutable (her zaman eşleşir). Herhangi bir olası değeri eşleştiren desenler _irrefutable_ (kesin eşleşen) desenlerdir. Örneğin, `let x = 5;` ifadesindeki `x` deseni, her şeyi eşleştirir ve bu nedenle eşleşememe ihtimali yoktur. Bazı olası değerlerle eşleşemeyen desenler ise _refutable_ (eşleşemeyebilir) desenlerdir. Örneğin, `if let Some(x) = a_value` ifadesindeki `Some(x)` deseni, eğer `a_value` değişkeninin değeri `None` ise eşleşmez.

Fonksiyon parametreleri, `let` ifadeleri ve `for` döngüleri yalnızca irrefutable desenleri kabul edebilir; çünkü değerler eşleşmediğinde programın yapabileceği anlamlı bir şey yoktur. `if let` ve `while let` ifadeleri ile `let...else` ifadesi ise hem refutable hem de irrefutable desenleri kabul eder; ancak derleyici, irrefutable desenler için uyarı verir; çünkü tanım gereği, bu yapılar olası bir başarısızlığı ele almak için tasarlanmıştır: bir koşulun işlevi, başarıya veya başarısızlığa göre farklı davranabilmesidir.

Genel olarak, refutable ve irrefutable desenler arasındaki farkı çok fazla düşünmenize gerek yoktur; ancak hata mesajlarında bu kavramla karşılaştığınızda ne anlama geldiğini bilmeniz gerekir. Bu gibi durumlarda, kodun beklenen davranışına göre ya deseni ya da kullandığınız yapıyı değiştirmeniz gerekir.

Şimdi, Rust'ın irrefutable bir desen gerektirdiği yerde refutable bir desen kullanmaya çalıştığımızda ve tam tersi durumda ne olacağını gösteren bir örneğe bakalım. 19-8 numaralı listede, bir `let` ifadesinde desen olarak refutable bir desen olan `Some(x)` kullandık. Tahmin edebileceğiniz gibi, bu kod derlenmeyecektir.

<Listing number="19-8" caption="`let` ile refutable bir desen kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

Eğer `some_option_value` bir `None` değeri olsaydı, `Some(x)` deseniyle eşleşemezdi; yani bu desen refutable'dır. Ancak, `let` ifadesi yalnızca irrefutable desenleri kabul eder; çünkü `None` değeriyle yapılacak geçerli bir şey yoktur. Derleme zamanında, Rust refutable bir deseni irrefutable bir desenin gerektiği yerde kullandığımızı belirterek hata verecektir:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

`Some(x)` deseniyle tüm olası değerleri kapsamadığımız (ve kapsayamayacağımız) için, Rust haklı olarak derleyici hatası üretir.

Eğer irrefutable bir desenin gerektiği yerde refutable bir desen kullanırsak, bunu deseni kullandığımız kodu değiştirerek düzeltebiliriz: `let` yerine `let else` kullanabiliriz. Böylece, desen eşleşmezse, süslü parantez içindeki kod atlanır ve kod geçerli şekilde devam edebilir. 19-9 numaralı listede, 19-8 numaralı listedeki kodun nasıl düzeltileceği gösterilmiştir.

<Listing number="19-9" caption="Refutable desenlerle `let...else` ve blok kullanmak, `let` yerine">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

Artık kodun bir çıkışı var! Bu kod tamamen geçerlidir; ancak, irrefutable bir desen kullanırsak uyarı alırız. Eğer `let...else` ifadesine her zaman eşleşen bir desen (örneğin `x`) verirsek, 19-10 numaralı listede gösterildiği gibi derleyici uyarı verecektir.

<Listing number="19-10" caption="`let...else` ile irrefutable desen kullanmaya çalışmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

Rust, irrefutable bir desenle `let...else` kullanmanın mantıklı olmadığını belirtir:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

Bu nedenle, match kolları refutable desenler kullanmalıdır; son kol ise kalan tüm değerleri eşleştiren irrefutable bir desen olmalıdır. Rust, yalnızca tek bir kola sahip bir `match` ifadesinde irrefutable desen kullanılmasına izin verir; ancak bu sözdizimi pek kullanışlı değildir ve daha basit bir `let` ifadesiyle değiştirilebilir.

Artık desenleri nerede kullanacağınızı ve refutable ile irrefutable desenler arasındaki farkı bildiğinize göre, şimdi desen oluşturmak için kullanabileceğimiz tüm sözdizimini ele alalım.
