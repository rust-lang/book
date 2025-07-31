## Desenlerin Kullanılabildiği Tüm Yerler

Desenler, Rust'ta birçok yerde karşımıza çıkar ve aslında farkında olmadan onları sıkça kullanmış olabilirsiniz! Bu bölümde, desenlerin geçerli olduğu tüm yerleri ele alacağız.

### `match` Kolları

6. bölümde tartışıldığı gibi, desenleri `match` ifadelerinin kollarında kullanırız. Biçimsel olarak, `match` ifadeleri, `match` anahtar kelimesi, eşlenecek bir değer ve bir veya daha fazla desen ve bu desene uyan değer için çalıştırılacak bir ifadeden oluşur. Şöyle görünür:

<!--
  Elle biçimlendirildi, çünkü Markdown kod bloğunda gövde içinde kodu italik yapamıyor!
-->

<pre><code>match <em>DEĞER</em> {
    <em>DESEN</em> => <em>İFADE</em>,
    <em>DESEN</em> => <em>İFADE</em>,
    <em>DESEN</em> => <em>İFADE</em>,
}</code></pre>

Örneğin, 6-5 numaralı listede, `x` değişkenindeki bir `Option<i32>` değeriyle eşleşen bir `match` ifadesi vardı:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

Bu `match` ifadesindeki desenler, okların solundaki `None` ve `Some(i)`'dir.

`match` ifadeleriyle ilgili bir gereklilik, _kapsamlı_ (exhaustive) olmalarıdır; yani, `match` ifadesindeki değer için tüm olasılıkların hesaba katılması gerekir. Tüm olasılıkları kapsadığınızdan emin olmanın bir yolu, son kola bir joker desen (catch-all) eklemektir: örneğin, herhangi bir değeri eşleyen bir değişken adı asla başarısız olmaz ve kalan tüm durumları kapsar.

Özel olarak `_` deseni her şeyi eşler, ancak hiçbir değişkene bağlanmaz; bu yüzden genellikle son `match` kolunda kullanılır. `_` deseni, örneğin belirtilmeyen herhangi bir değeri yok saymak istediğinizde faydalı olabilir. `_` desenini bu bölümün ilerleyen kısımlarında ["Bir Desende Değerleri Yok Saymak"] [ignoring-values-in-a-pattern]<!-- ignore --> başlığı altında daha ayrıntılı ele alacağız.

### let İfadeleri

Bu bölüme kadar, desenleri yalnızca `match` ve `if let` ile kullandığımızı açıkça tartışmıştık; ancak aslında desenleri başka yerlerde de kullandık, örneğin `let` ifadelerinde. Örneğin, aşağıdaki gibi basit bir değişken atamasını düşünün:

```rust
let x = 5;
```

Her `let` ifadesi kullandığınızda aslında bir desen kullanmış oluyorsunuz, farkında olmasanız bile! Biçimsel olarak, bir `let` ifadesi şöyle görünür:

<!--
  Elle biçimlendirildi, çünkü Markdown kod bloğunda gövde içinde kodu italik yapamıyor!
-->

<pre>
<code>let <em>DESEN</em> = <em>İFADE</em>;</code>
</pre>

`let x = 5;` gibi ifadelerde, DESEN kısmında bir değişken adı vardır ve bu, desenin özellikle basit bir biçimidir. Rust, ifadeyi desenle karşılaştırır ve bulduğu tüm adları atar. Yani, `let x = 5;` örneğinde, `x` burada "buraya uyanı `x` değişkenine bağla" anlamına gelen bir desendir. `x` adının tüm desen olması, bu desenin "her şeyi, ne olursa olsun `x` değişkenine bağla" anlamına gelmesini sağlar.

Desenlerin eşleşme yönünü `let` ifadesinde daha net görebilmek için, bir demet (tuple) yapısını `let` ile çözümleyen 19-1 numaralı listeye bakalım.


<Listing number="19-1" caption="Bir deseni kullanarak bir demeti çözümleme ve aynı anda üç değişken oluşturma">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

</Listing>

Burada, bir demet, bir desenle eşleştirilir. Rust, `(1, 2, 3)` değerini `(x, y, z)` deseniyle karşılaştırır ve değer ile desenin, her ikisinde de eleman sayısının aynı olduğunu görerek eşleştiğini belirler; böylece Rust, `1` değerini `x`'e, `2` değerini `y`'ye ve `3` değerini `z`'ye bağlar. Bu demet desenini, içinde üç tane bireysel değişken desenini iç içe geçmiş gibi düşünebilirsiniz.

Eğer desendeki eleman sayısı, demetteki eleman sayısıyla uyuşmuyorsa, genel tür eşleşmeyecek ve bir derleyici hatası alacağız. Örneğin, 19-2 numaralı listede, üç elemanlı bir demeti iki değişkene çözümlemeye çalıştığımızda oluşan hata gösterilmektedir.

<Listing number="19-2" caption="Değişken sayısının demetteki eleman sayısıyla uyuşmadığı bir deseni yanlışlıkla oluşturma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

Bu kodu derlemeye çalışmak, şu tür hatasına yol açar:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-02/output.txt}}
```

Hatanın üstesinden gelmek için, demetteki bir veya daha fazla değeri `_` veya `..` kullanarak yok sayabiliriz; bunu, bu bölümün ilerleyen kısımlarında ["Bir Desende Değerleri Yok Saymak"] [ignoring-values-in-a-pattern]<!-- ignore --> başlığı altında göreceksiniz. Eğer sorun, desendeki fazla değişken sayısından kaynaklanıyorsa, o zaman türlerin eşleşmesini sağlamak için değişkenleri çıkararak demetteki eleman sayısıyla değişken sayısını eşitleyacak şekilde düzeltmemiz gerekir.

### Koşullu if let İfadeleri

6. bölümde, `if let` ifadelerini esasen yalnızca bir durumu eşleştiren bir `match` ifadesinin daha kısa bir yolunu yazmak için nasıl kullanacağımızı tartıştık. İsteğe bağlı olarak, `if let` ifadesinin eşleşmediği durumlarda çalıştırılacak bir kod içeren karşılık gelen bir `else` bloğu olabilir.

19-3 numaralı listede, `if let`, `else if` ve `else if let` ifadelerini karıştırıp eşleştirmenin de mümkün olduğunu gösteriyoruz. Bunu yapmak, bir dizi koşul için arka arkaya kontrol yapmamıza olanak tanır; bu koşulların her biri, bir değeri desenlerle karşılaştırmak için kullanılır. Ayrıca, Rust, bir dizi `if let`, `else if` ve `else if let` kolundaki koşulların birbirleriyle ilişkili olmasını gerektirmez.

19-3 numaralı listedeki kod, arka planda hangi rengi kullanacağımıza karar verirken bir dizi kontrol yapar. Bu örnekte, gerçek bir programın kullanıcıdan alabileceği sabit kodlu değerlerle değişkenler oluşturduk.

<Listing number="19-3" file-name="src/main.rs" caption="Karıştırma `if let`, `else if`, `else if let` ve `else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

</Listing>

Eğer kullanıcı bir favori renk belirtirse, arka plan o renk olarak ayarlanır. Eğer hiçbir favori renk belirtilmemişse ve bugün Salı ise, arka plan rengi yeşil olur. Aksi takdirde, eğer kullanıcı yaşı bir dize olarak belirtmişse ve bunu bir sayıya başarıyla ayrıştırabiliyorsak, renk ya mor ya da turuncu olur; bu, sayının değerine bağlıdır. Eğer bu koşullardan hiçbiri geçerli değilse, arka plan rengi mavi olur.

Bu koşullu yapı, karmaşık gereksinimleri desteklememizi sağlar. Buradaki sabit kodlu değerlerle, bu örnek `Arka plan rengi olarak mor kullanılıyor` çıktısını verecektir.

Ayrıca, `if let` ifadelerinin, `match` kollarında olduğu gibi mevcut değişkenleri gölgeleyerek yeni değişkenler tanıtabileceğini görebilirsiniz: `if let Ok(age) = age` satırı, `Ok` varyantının içindeki değeri içeren yeni bir `age` değişkeni tanıtır ve bu, mevcut `age` değişkenini gölgeler. Bu, `if age > 30` koşulunu o blok içinde yerleştirmemiz gerektiği anlamına gelir; bu iki koşulu `if let Ok(age) = age && age > 30` şeklinde birleştiremeyiz. Karşılaştırmak istediğimiz yeni `age` değişkeni, yeni kapsam açılana kadar geçerli değildir.

`if let` ifadelerinin bir dezavantajı, derleyicinin kapsayıcılığı kontrol etmemesidir; oysa `match` ifadelerinde kontrol eder. Eğer son `else` bloğunu atlar ve bu nedenle bazı durumları ele almayı kaçırırsak, derleyici bize olası bir mantık hatası hakkında bilgi vermez.

### `while let` Koşullu Döngüleri

`if let` ile yapısal olarak benzer olan `while let` koşullu döngüsü, bir desen eşleşmeye devam ettiği sürece `while` döngüsünün çalışmasına olanak tanır. 19-4 numaralı listede, bir `while let` döngüsünün, bir `Option` yerine bir `Result` kontrolü yaparak, iş parçacıkları arasında gönderilen mesajları beklerken nasıl kullanılacağını gösteriyoruz.

<Listing number="19-4" caption="Bir `while let` döngüsü kullanarak `rx.recv()` her `Ok` döndüğünde değerleri yazdırma">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

Bu örnek, sırasıyla `1`, `2` ve `3` değerlerini yazdırır. `recv` yöntemi, kanalın alıcı tarafındaki ilk mesajı alır ve bir `Ok(değer)` döndürür. 16. bölümde `recv` ile ilk kez karşılaştığımızda, hatayı doğrudan açmış veya bir `for` döngüsü kullanarak bir yineleyici gibi etkileşimde bulunmuştuk. Ancak, 19-4 numaralı listede gösterildiği gibi, `recv` yöntemi her mesaj geldiğinde bir `Ok` döndürdüğünden ve ardından gönderen tarafı kapandığında bir `Err` ürettiğinden, `while let` kullanabiliriz.

### `for` Döngüleri

Bir `for` döngüsünde, `for` anahtar kelimesinin hemen ardından gelen değer bir desendir. Örneğin, `for x in y` ifadesindeki `x` bir desendir. 19-5 numaralı listede, bir `for` döngüsünde bir demeti *çözümlemek* veya parçalarına ayırmak için bir deseni nasıl kullanacağımızı gösteriyoruz.


<Listing number="19-5" caption="Bir `for` döngüsünde bir deseni kullanarak bir demeti çözümleme">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

19-5 numaralı listedeki kod, aşağıdaki çıktıyı verir:


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

Bir yineleyiciyi, bir değer ve o değerin dizinini üretecek şekilde `enumerate` yöntemiyle uyarlıyoruz; bu, bir demet içine yerleştirilir. Üretilen ilk değer, `(0, 'a')` demetidir. Bu değer, `(index, value)` desenine eşlendiğinde, `index` değeri `0` ve `value` değeri `'a'` olur ve çıktının ilk satırını yazdırır.

### Fonksiyon Parametreleri

Fonksiyon parametreleri de desenler olabilir. 19-6 numaralı listede, `x` adında bir `i32` türünde bir parametre alan `foo` adlı bir fonksiyon tanımlıyoruz; bu, muhtemelen artık tanıdık gelecektir.

<Listing number="19-6" caption="Bir fonksiyon imzasında parametrelerde desenler kullanımı">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

`x` kısmı bir desendir! `let` ile yaptığımız gibi, bir fonksiyonun argümanlarında bir demeti desene göre eşleştirebiliriz. 19-7 numaralı listede, bir demeti çözümleyerek bir fonksiyona geçiriyoruz.

<Listing number="19-7" file-name="src/main.rs" caption="Bir demeti çözümleyen parametrelerle bir fonksiyon">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

Bu kod, `Mevcut konum: (3, 5)` çıktısını verir. `&(3, 5)` değerleri, `&(x, y)` deseniyle eşleşir; bu nedenle, `x` değeri `3` ve `y` değeri `5` olur.

Ayrıca, kapanış (closure) parametre listelerinde de desenleri, fonksiyon parametre listelerinde olduğu gibi kullanabiliriz; çünkü kapanışlar, 13. bölümde tartışıldığı gibi, fonksiyonlara benzer.

Bu noktada, desenleri kullanmanın birkaç yolunu gördünüz, ancak desenler her kullanıldıkları yerde aynı şekilde çalışmaz. Bazı yerlerde, desenlerin ırkçı olmaması gerekir; diğer durumlarda, ırkçı olabilirler. Bu iki kavramı bir sonraki bölümde tartışacağız.

[ignoring-values-in-a-pattern]: ch19-03-pattern-syntax.md#bir-desende-değerleri-yok-saymak
