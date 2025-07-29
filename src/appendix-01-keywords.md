## Ek A: Anahtar Kelimeler

Aşağıdaki liste, mevcut veya gelecekteki kullanımlar için ayrılmış anahtar sözcükleri içerir
Rust dili tarafından kullanılır. Bu nedenle, tanımlayıcı olarak kullanılamazlar (ancak bölümünde tartışacağımız gibi ham tanımlayıcılar olarak "[Raw Identifiers][raw-identifiers]<!-- ignore -->" bölümü). Tanımlayıcılar isimlerdir
fonksiyonlar, değişkenler, parametreler, struct alanları, modüller, crate'ler, sabitler,
makrolar, statik değerler, nitelikler, türler, özellikler veya yaşam süreleri.

[raw-identifiers]: #raw-identifiers

### Şu Anda Kullanımda Olan Anahtar Kelimeler

Aşağıda şu anda kullanımda olan anahtar kelimelerin işlevleriyle birlikte bir listesi yer almaktadır
tarif edildi.

- `as` - ilkel döküm gerçekleştirin, içeren belirli özelliği belirsizleştirin
  bir öğeyi veya `use` ifadelerindeki öğeleri yeniden adlandırın
- `async` - mevcut iş parçacığını engellemek yerine bir `Future` döndürür
- `await` - bir `Future` sonucu hazır olana kadar yürütmeyi askıya alır
- `break` - bir döngüden hemen çıkın
- `const` - sabit öğeler veya sabit ham işaretçiler tanımlar
- `continue` - bir sonraki döngü yinelemesine devam eder
- `crate` - bir modül yolunda, sandık kökünü ifade eder
- `dyn` - bir trait nesnesine dinamik gönderim
- `else` - `if` ve `if let` kontrol akışı yapıları için geri dönüş
- `enum` - bir numaralandırma tanımlar
- `extern` - harici bir fonksiyon veya değişken bağlar
- `false` - Boolean yanlış değişmezi
- `fn` - bir fonksiyon veya fonksiyon işaretçisi tipi tanımlar
- `for` - bir yineleyicideki öğeler üzerinde döngü yapın, bir özellik uygulayın veya bir
  yüksek dereceli yaşam süresi
- `if` - koşullu bir ifadenin sonucuna göre dallanma
- `impl` - içsel veya özellik işlevselliğini uygular
- `in` - `for` döngüsü sözdiziminin bir parçası
- `let` - bir değişken bağlar
- `loop` - koşulsuz döngü
- `match` - bir değeri kalıplarla eşleştirir
- `mod` - bir modül tanımlayın
- `move` - bir kapanışın tüm yakalamalarının sahipliğini almasını sağlar
- `mut` - referanslarda, ham işaretçilerde veya kalıp bağlamalarında değişebilirliği belirtir
- `pub` - struct alanlarında, `impl` bloklarında veya modüllerde genel görünürlüğü belirtir
- `ref` - referans ile bağlama
- `return` - fonksiyondan geri dönüş
- `Self` - tanımladığımız veya uyguladığımız tür için bir tür takma adı
- `self` - yöntem öznesi veya geçerli modül
- `static` - global değişken veya tüm program yürütmesi boyunca süren yaşam süresi
- `struct` - bir yapı tanımlar
- `super` - geçerli modülün ana modülü
- `trait` - bir özellik tanımlayın
- `true` - Boolean gerçek değişmez
- `type` - bir tür takma adı veya ilişkili tür tanımlar
- `union` - bir [union][union]<!-- ignore --> tanımlar; kullanıldığında yalnızca bir anahtar sözcüktür
  bir birlik bildirgesinde
- `unsafe` - güvenli olmayan kod, fonksiyon, özellik veya uygulamaları belirtir
- `use` - sembolleri kapsam içine alın; genel ve özel semboller için kesin yakalamalar belirleyin
  hayatım

- `self` - yöntem öznesi veya geçerli modül
- `static` - global değişken veya tüm program yürütmesi boyunca süren yaşam süresi
- `struct` - bir yapı tanımlar
- `super` - geçerli modülün ana modülü
- `trait` - bir özellik tanımlayın
- `true` - Boolean gerçek değişmez
- `type` - bir tür takma adı veya ilişkili tür tanımlar
- union` - bir [union][union]<!-- ignore --> tanımlar; kullanıldığında yalnızca bir anahtar sözcüktür
  bir birlik bildirgesinde
- `unsafe` - güvenli olmayan kod, fonksiyon, özellik veya uygulamaları belirtir
- `use` - sembolleri kapsam içine alın; genel ve özel semboller için kesin yakalamalar belirleyin
  ömür boyu sınırlar
- `where` - bir türü kısıtlayan tümceleri belirtir
- `while` - bir ifadenin sonucuna bağlı olarak koşullu döngü

[union]: ../reference/items/unions.md

### Gelecekte Kullanılmak Üzere Ayrılmış Anahtar Kelimeler

Aşağıdaki anahtar sözcükler henüz herhangi bir işleve sahip değildir ancak
Gelecekteki potansiyel kullanım için pas.

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

### Ham Tanımlayıcılar

_Raw identifiers_ anahtar sözcükleri kullanamayacağınız yerlerde kullanmanızı sağlayan sözdizimidir
normalde izin verilir. Bir anahtar sözcüğün önüne `r#` ekleyerek ham bir tanımlayıcı kullanırsınız.

Örneğin, `match` bir anahtar kelimedir. Aşağıdaki fonksiyonu derlemeye çalışırsanız
adı olarak `match` seçeneğini kullanır:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

bu hatayı alırsınız:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

Hata, `match` anahtar sözcüğünü işlev olarak kullanamayacağınızı gösterir
tanımlayıcı. Bir fonksiyon adı olarak `match` kullanmak için, ham
tanımlayıcı sözdizimi, bunun gibi:

<span class="filename">Filename: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Bu kod herhangi bir hata olmadan derlenecektir. Fonksiyon üzerindeki `r#` önekine dikkat edin
adının yanı sıra işlevin `main` içinde nerede çağrıldığı da belirtilir.

Ham tanımlayıcılar, tanımlayıcı olarak seçtiğiniz herhangi bir kelimeyi kullanmanıza izin verir, hatta
bu sözcük ayrılmış bir anahtar sözcük olabilir. Bu bize seçim yapmak için daha fazla özgürlük verir
tanımlayıcı adları ile yazılmış programlarla entegre olmamızı sağlar.
Bu kelimelerin anahtar kelime olmadığı diller. Buna ek olarak, ham tanımlayıcılar
crate'inizin kullandığından farklı bir Rust sürümünde yazılmış kütüphaneleri kullanmanızı sağlar.
Örneğin, `try` 2015 sürümünde bir anahtar sözcük değildir, ancak 2018, 2021 sürümlerinde vardır,
ve 2024 sürümleri. Eğer 2015 kullanılarak yazılmış bir kütüphaneye bağımlı iseniz
sürümüne ve bir `try` işlevine sahipse, ham tanımlayıcı sözdizimini kullanmanız gerekir,
Bu durumda, sonraki sürümlerde kodunuzdan bu işlevi çağırmak için `r#try`.
Sürümler hakkında daha fazla bilgi için [Appendix E][appendix-e]<!-- ignore --> bölümüne bakın.

[appendix-e]: appendix-05-editions.md
