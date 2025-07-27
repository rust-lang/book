## Desen Sözdizimi

Bu bölümde, desenlerde geçerli olan tüm sözdizimini bir araya getiriyor ve her birini ne zaman ve neden kullanmak isteyebileceğinizi tartışıyoruz.

### Sabitlerle Eşleştirme

6. bölümde gördüğünüz gibi, desenleri doğrudan sabitlerle eşleştirebilirsiniz. Aşağıdaki kodda bazı örnekler verilmiştir:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

Bu kod, `x`'in değeri `1` olduğu için `one` yazdırır. Bu sözdizimi, kodunuzun belirli bir somut değeri aldığında bir işlem yapmasını istediğinizde kullanışlıdır.

### İsimlendirilmiş Değişkenlerle Eşleştirme

İsimlendirilmiş değişkenler, herhangi bir değeri eşleştiren irrefutable desenlerdir ve bu kitapta birçok kez kullandık. Ancak, `match`, `if let` veya `while let` ifadelerinde isimlendirilmiş değişkenler kullandığınızda bir karmaşıklık ortaya çıkar. Bu tür ifadelerin her biri yeni bir kapsam başlattığından, bu ifadeler içinde desenin parçası olarak tanımlanan değişkenler, yapının dışındaki aynı isimli değişkenleri gölgeler; bu, tüm değişkenler için geçerlidir. 19-11 numaralı listede, değeri `Some(5)` olan `x` adında bir değişken ve değeri `10` olan bir `y` değişkeni tanımlıyoruz. Ardından, `x` değerinde bir `match` ifadesi oluşturuyoruz. Match kollarındaki desenlere ve sonda yer alan `println!` ifadesine bakın ve kodun ne yazdıracağını çalıştırmadan veya okumaya devam etmeden önce tahmin etmeye çalışın.

<Listing number="19-11" file-name="src/main.rs" caption="Bir kolunda mevcut `y` değişkenini gölgeleyen yeni bir değişken tanımlayan `match` ifadesi">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

</Listing>

`match` ifadesi çalıştığında neler olduğunu adım adım inceleyelim. İlk match kolundaki desen, `x`'in tanımlı değeriyle eşleşmez, bu yüzden kod devam eder.

İkinci match kolundaki desen, `Some` değerinin içindeki herhangi bir değeri eşleştirecek yeni bir `y` değişkeni tanımlar. `match` ifadesinin içinde yeni bir kapsamda olduğumuz için, bu yeni bir `y` değişkenidir; başta değeri `10` olan `y` ile aynı değildir. Bu yeni `y` bağlaması, `Some` içindeki herhangi bir değeri eşleştirir; bu da `x`'te olan değerdir. Bu nedenle, bu yeni `y`, `x`'teki `Some`'un iç değerine bağlanır. O değer `5`'tir, bu yüzden o kolun ifadesi çalışır ve `Matched, y = 5` yazdırılır.

Eğer `x` değeri `Some(5)` yerine `None` olsaydı, ilk iki kolun desenleri eşleşmezdi, bu yüzden değer, alt çizgiye (`_`) eşleşirdi. Alt çizgi kolunun deseninde `x` değişkenini tanımlamadık, bu yüzden ifadede kullanılan `x`, hala gölgelenmemiş olan dıştaki `x` olurdu. Bu durumda, `match` ifadesi `Default case, x = None` yazdırırdı.

`match` ifadesi tamamlandığında, kapsamı sona erer ve içteki `y`'nin kapsamı da biter. Son `println!` ifadesi `at the end: x = Some(5), y = 10` yazdırır.

Dıştaki `x` ve `y` değerlerini karşılaştıran bir `match` ifadesi oluşturmak için, mevcut `y` değişkenini gölgeleyen yeni bir değişken tanımlamak yerine, bir match guard koşulu kullanmamız gerekir. Match guard'ları daha sonra ["Match Guard'larla Ekstra Koşullar"](#extra-conditionals-with-match-guards)<!-- ignore --> başlığında ele alacağız.

### Birden Fazla Desen

`match` ifadelerinde, `|` sözdizimini kullanarak birden fazla deseni eşleştirebilirsiniz; bu, desen _veya_ operatörüdür. Örneğin, aşağıdaki kodda, `x` değerini match kollarına karşı eşleştiriyoruz; ilk kolun bir _veya_ seçeneği var, yani `x`'in değeri o koldaki değerlerden herhangi biriyle eşleşirse, o kolun kodu çalışır:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

Bu kod `one or two` yazdırır.

### `..=` ile Değer Aralıklarını Eşleştirme

`..=` sözdizimi, kapsayıcı bir değer aralığıyla eşleşmemizi sağlar. Aşağıdaki kodda, bir desen verilen aralıktaki herhangi bir değerle eşleşirse, o kol çalışır:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

Eğer `x` değeri `1`, `2`, `3`, `4` veya `5` ise, ilk kol eşleşir. Bu sözdizimi, aynı fikri ifade etmek için `|` operatörünü kullanmaktan daha kullanışlıdır; eğer `|` kullansaydık, `1 | 2 | 3 | 4 | 5` yazmamız gerekirdi. Özellikle 1 ile 1000 arasındaki herhangi bir sayıyı eşleştirmek istiyorsak, aralık belirtmek çok daha kısadır!

Derleyici, aralığın boş olmadığını derleme zamanında kontrol eder ve Rust'ın bir aralığın boş olup olmadığını anlayabildiği tek türler `char` ve sayısal değerlerdir; bu nedenle aralıklar yalnızca sayısal veya `char` değerlerle kullanılabilir.

İşte `char` değerlerinin aralıklarını kullanan bir örnek:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Rust, `'c'` harfinin ilk desenin aralığında olduğunu anlar ve `early ASCII letter` yazdırır.

### Değerleri Parçalarına Ayırmak için Destructuring

Desenleri, struct, enum ve tuple'ları parçalarına ayırmak (destructure) ve bu değerlerin farklı kısımlarını kullanmak için de kullanabiliriz. Her bir değer türünü adım adım inceleyelim.

#### Struct'ları Parçalarına Ayırmak

19-12 numaralı listede, iki alanı (`x` ve `y`) olan bir `Point` struct'ı gösteriliyor; bunu bir `let` ifadesinde bir desenle parçalarına ayırabiliriz.

<Listing number="19-12" file-name="src/main.rs" caption="Bir struct'ın alanlarını ayrı değişkenlere ayırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

</Listing>

Bu kod, `p` struct'ının `x` ve `y` alanlarının değerleriyle eşleşen `a` ve `b` değişkenlerini oluşturur. Bu örnek, desendeki değişken isimlerinin struct alan isimleriyle aynı olmak zorunda olmadığını gösterir. Ancak, değişken isimlerini alan isimleriyle eşleştirmek, hangi değişkenin hangi alandan geldiğini hatırlamayı kolaylaştırdığı için yaygındır. Bu yaygın kullanım nedeniyle ve `let Point { x: x, y: y } = p;` yazmak çok fazla tekrar içerdiğinden, Rust struct alanlarını eşleştiren desenler için bir kısayol sunar: yalnızca struct alanının adını yazmanız yeterlidir ve desenden oluşturulan değişkenler aynı isimlere sahip olur. 19-13 numaralı liste, 19-12 numaralı listedeki kodla aynı şekilde davranır; ancak `let` deseninde oluşturulan değişkenler `x` ve `y` olur.

<Listing number="19-13" file-name="src/main.rs" caption="Struct alanlarını struct alanı kısayoluyla parçalarına ayırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

</Listing>

Bu kod, `p` değişkeninin `x` ve `y` alanlarıyla eşleşen `x` ve `y` değişkenlerini oluşturur. Sonuç olarak, `x` ve `y` değişkenleri `p` struct'ından gelen değerleri içerir.

Ayrıca, tüm alanlar için değişken oluşturmak yerine, struct deseninin bir parçası olarak sabit değerler de kullanabiliriz. Bu, bazı alanları belirli değerlere göre test etmemizi, diğer alanları ise parçalarına ayırmak için değişken oluşturmamızı sağlar.

19-14 numaralı listede, bir `match` ifadesiyle `Point` değerlerini üç duruma ayırıyoruz: doğrudan `x` ekseninde olan noktalar (`y = 0` olduğunda), `y` ekseninde olan noktalar (`x = 0` olduğunda) veya hiçbir eksende olmayan noktalar.

<Listing number="19-14" file-name="src/main.rs" caption="Bir desende sabit değerlerle eşleştirerek parçalarına ayırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

</Listing>

İlk kol, `y` alanının değeri `0` ise, `x` ekseninde olan herhangi bir noktayı eşleştirir. Desen yine de bu kolda kullanabileceğimiz bir `x` değişkeni oluşturur.

Benzer şekilde, ikinci kol, `x` alanının değeri `0` ise, `y` ekseninde olan herhangi bir noktayı eşleştirir ve `y` alanının değeri için bir `y` değişkeni oluşturur. Üçüncü kol ise herhangi bir sabit belirtmez, bu nedenle diğer tüm `Point` değerleriyle eşleşir ve hem `x` hem de `y` alanları için değişkenler oluşturur.

Bu örnekte, `p` değeri, `x` değeri `0` içerdiği için ikinci kola eşleşir; bu kod `On the y axis at 7` yazdırır.

Bir `match` ifadesinin, ilk eşleşen deseni bulduktan sonra diğer kolları kontrol etmeyi bıraktığını unutmayın; bu nedenle, `Point { x: 0, y: 0}` hem `x` ekseninde hem de `y` eksenindedir, ancak bu kod yalnızca `On the x axis at 0` yazdırır.

#### Enum'ları Parçalarına Ayırmak

Bu kitapta enum'ları parçalarına ayırdık (örneğin, 6. bölümdeki 6-5 numaralı listede), ancak henüz enum'ı parçalarına ayıran desenin, enum içinde depolanan verinin tanımına karşılık geldiğini açıkça tartışmadık. Örneğin, 19-15 numaralı listede, 6-2 numaralı listeden alınan `Message` enum'ını kullanıyor ve her iç değeri parçalarına ayıracak desenlerle bir `match` yazıyoruz.

<Listing number="19-15" file-name="src/main.rs" caption="Farklı türde değerler tutan enum varyantlarını parçalarına ayırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

</Listing>

Bu kod, `Change color to red 0, green 160, and blue 255` yazdırır. `msg` değerini değiştirerek diğer kolların kodunun çalışmasını görebilirsiniz.

Veri içermeyen enum varyantları için, örneğin `Message::Quit`, değeri daha fazla parçalarına ayıramayız. Yalnızca `Message::Quit` sabit değeriyle eşleştirebiliriz ve bu desende değişken yoktur.

Struct benzeri enum varyantları için, örneğin `Message::Move`, struct'ları eşleştirmek için kullandığımız desene benzer bir desen kullanabiliriz. Varyant adından sonra süslü parantezler açar ve alanları değişkenlerle listeleriz; böylece bu kolda kullanmak üzere parçalarına ayırırız. Burada, 19-13 numaralı listede olduğu gibi kısayol formunu kullanıyoruz.

Tuple benzeri enum varyantları için, örneğin bir elemanlı tuple tutan `Message::Write` ve üç elemanlı tuple tutan `Message::ChangeColor`, desen, tuple'ları eşleştirmek için kullandığımız desene benzer. Desendeki değişken sayısı, eşleştirdiğimiz varyanttaki eleman sayısıyla aynı olmalıdır.

#### İç İçe Struct ve Enum'ları Parçalarına Ayırmak

Şimdiye kadar örneklerimizde yalnızca bir seviye derinliğinde struct veya enum'ları eşleştirdik; ancak eşleştirme, iç içe geçmiş öğelerde de çalışır! Örneğin, 19-15 numaralı listedeki kodu, `ChangeColor` mesajında RGB ve HSV renklerini destekleyecek şekilde 19-16 numaralı listede yeniden düzenleyebiliriz.

<Listing number="19-16" caption="İç içe enum'larda eşleştirme">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

</Listing>

`match` ifadesindeki ilk kolun deseni, bir `Message::ChangeColor` enum varyantını ve onun içinde bir `Color::Rgb` varyantını eşleştirir; ardından desen, üç içteki `i32` değerine bağlanır. İkinci kolun deseni de bir `Message::ChangeColor` enum varyantını eşleştirir, ancak içteki enum `Color::Hsv` ile eşleşir. Bu karmaşık koşulları, iki enum dahil olsa bile tek bir `match` ifadesinde belirtebiliriz.

#### Struct ve Tuple'ları Parçalarına Ayırmak

Destructuring desenlerini daha da karmaşık şekillerde karıştırabilir, eşleştirebilir ve iç içe kullanabiliriz. Aşağıdaki örnek, bir tuple'ın içine struct ve tuple'lar yerleştirip, tüm ilkel değerleri parçalarına ayırdığımız karmaşık bir destructure işlemi gösteriyor:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

Bu kod, karmaşık türleri bileşenlerine ayırmamıza ve ilgilendiğimiz değerleri ayrı ayrı kullanmamıza olanak tanır.

Destructuring ile desen kullanmak, bir struct'ın her alanındaki değeri birbirinden bağımsız olarak kullanmak gibi, değerlerin parçalarını kullanmanın pratik bir yoludur.

### Bir Desende Değerleri Yok Saymak

Bazen bir desende değerleri yok saymak faydalı olabilir; örneğin, bir `match` ifadesinin son kolunda, kalan tüm olası değerleri hesaba katmak için bir yakalama (catch-all) kolu oluşturmak isteyebilirsiniz. Bir desende tüm değerleri veya değerlerin bazı kısımlarını yok saymanın birkaç yolu vardır: `_` deseni kullanmak (daha önce gördünüz), başka bir desenin içinde `_` kullanmak, ismi alt çizgiyle başlayan bir isim kullanmak veya kalan değerleri yok saymak için `..` kullanmak. Şimdi bu desenlerin her birini nasıl ve neden kullanacağımızı inceleyelim.

<!-- Eski bağlantı, kaldırmayın -->

<a id="ignoring-an-entire-value-with-_"></a>

#### `_` ile Tüm Değeri Yok Saymak

Alt çizgiyi, herhangi bir değeri eşleştiren ancak değere bağlanmayan joker desen olarak kullandık. Bu, özellikle bir `match` ifadesinin son kolunda kullanışlıdır; ancak, bir desenin kullanıldığı her yerde, fonksiyon parametreleri dahil, kullanılabilir. 19-17 numaralı listede gösterilmiştir.

<Listing number="19-17" file-name="src/main.rs" caption="Bir fonksiyon imzasında `_` kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-17/src/main.rs}}
```

</Listing>

Bu kod, ilk argüman olarak verilen `3` değerini tamamen yok sayar ve `This code only uses the y parameter: 4` yazdırır.

Çoğu durumda, artık ihtiyaç duymadığınız bir fonksiyon parametresi varsa, imzayı değiştirip kullanılmayan parametreyi çıkarmak daha iyidir. Ancak, bir trait'i uygularken belirli bir tür imzasına ihtiyacınız olup, fonksiyon gövdesinde o parametreye ihtiyacınız yoksa, fonksiyon parametresini yok saymak özellikle faydalı olabilir. Böylece, bir isim kullansaydınız alacağınız kullanılmayan parametre uyarısından kaçınmış olursunuz.

<a id="ignoring-parts-of-a-value-with-a-nested-_"></a>

#### Bir Değerin Parçalarını İç İçe `_` ile Yok Saymak

Bir değerin yalnızca bir kısmını yok saymak istediğimizde, `_`'yı başka bir desenin içinde de kullanabiliriz; örneğin, yalnızca bir değerin bir kısmını test etmek istiyor ancak ilgili kodda diğer kısımlara ihtiyacımız yoksa. 19-18 numaralı listede, bir ayarın değerini yöneten kod gösteriliyor. İş gereksinimleri, kullanıcının mevcut bir özelleştirmeyi üzerine yazmasına izin verilmemesini, ancak ayar kaldırıldığında ve şu anda ayarlanmamışsa bir değer verilebilmesini gerektiriyor.

<Listing number="19-18" caption="`Some` varyantlarının içindeki değeri kullanmamız gerekmediğinde desenlerde alt çizgi kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-18/src/main.rs:here}}
```

</Listing>

Bu kod, `Can't overwrite an existing customized value` ve ardından `setting is Some(5)` yazdırır. İlk match kolunda, her iki `Some` varyantının içindeki değerleri eşleştirmemiz veya kullanmamız gerekmez; ancak, hem `setting_value` hem de `new_setting_value` `Some` varyantı olduğunda bu durumu test etmemiz gerekir. Bu durumda, ayarın neden değiştirilmediğini yazdırırız ve `setting_value` değişmez.

Diğer tüm durumlarda (ya `setting_value` ya da `new_setting_value` `None` ise), ikinci koldaki `_` deseniyle ifade edilir, `new_setting_value`'yu `setting_value` yapmaya izin veririz.

Aynı desende birden fazla yerde alt çizgi kullanarak belirli değerleri yok sayabiliriz. 19-19 numaralı listede, beş elemanlı bir tuple'da ikinci ve dördüncü değerleri yok sayma örneği gösterilmiştir.

<Listing number="19-19" caption="Bir tuple'ın birden fazla parçasını yok saymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-19/src/main.rs:here}}
```

</Listing>

Bu kod, `Some numbers: 2, 8, 32` yazdırır ve `4` ile `16` değerleri yok sayılır.

<!-- Eski bağlantı, kaldırmayın -->

<a id="ignoring-an-unused-variable-by-starting-its-name-with-_"></a>

#### İsmi Alt Çizgiyle Başlayan Kullanılmayan Değişken

Bir değişken oluşturup onu hiçbir yerde kullanmazsanız, Rust genellikle kullanılmayan değişkenin bir hata olabileceği için uyarı verir. Ancak, bazen henüz kullanmayacağınız bir değişken oluşturmak faydalı olabilir; örneğin, prototipleme yaparken veya bir projeye yeni başlarken. Bu durumda, Rust'a kullanılmayan değişken hakkında uyarı vermemesi için değişkenin adını alt çizgiyle başlatabilirsiniz. 19-20 numaralı listede, iki kullanılmayan değişken oluşturuyoruz; ancak bu kodu derlediğimizde yalnızca biri hakkında uyarı almalıyız.

<Listing number="19-20" file-name="src/main.rs" caption="Kullanılmayan değişken uyarısı almamak için değişken adını alt çizgiyle başlatmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-20/src/main.rs}}
```

</Listing>

Burada, `y` değişkenini kullanmadığımız için uyarı alırız; ancak `_x` için uyarı almayız.

Yalnızca `_` kullanmak ile alt çizgiyle başlayan bir isim kullanmak arasında ince bir fark vardır. `_x` sözdizimi, değeri değişkene bağlar; oysa `_` hiçbir zaman değere bağlanmaz. Bu farkın önemli olduğu bir durumu göstermek için, 19-21 numaralı liste bir hata örneği sunar.

<Listing number="19-21" caption="Alt çizgiyle başlayan kullanılmayan bir değişken yine de değeri bağlar ve değerin sahipliğini alabilir.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-21/src/main.rs:here}}
```

</Listing>

Burada hata alırız; çünkü `s` değeri `_s`'ye taşınır ve `s`'yi tekrar kullanmamız engellenir. Ancak, yalnızca alt çizgi kullanırsak, değer hiçbir zaman bağlanmaz. 19-22 numaralı liste, bu nedenle hata vermeden derlenir.

<Listing number="19-22" caption="Alt çizgi kullanmak değeri bağlamaz.">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-22/src/main.rs:here}}
```

</Listing>

Bu kod sorunsuz çalışır; çünkü `s` hiçbir şeye bağlanmaz ve taşınmaz.

<a id="ignoring-remaining-parts-of-a-value-with-"></a>

#### `..` ile Bir Değerin Kalan Parçalarını Yok Saymak

Birçok parçaya sahip değerlerle, belirli parçaları kullanmak ve geri kalanını yok saymak için `..` sözdizimini kullanabiliriz; bu, her göz ardı edilen değer için alt çizgileri listeleme gereğini ortadan kaldırır. `..` deseni, kalanı yok saymak için eşleştiğimiz desenin geri kalanında açıkça eşleştirmediğimiz değerlerin herhangi bir parçasını yok sayar. 19-23 numaralı listede, üç boyutlu uzayda bir koordinat tutan bir `Point` struct'ımız var. `match` ifadesinde, yalnızca `x` koordinatı üzerinde işlem yapmak ve `y` ve `z` alanlarındaki değerleri yok saymak istiyoruz.

<Listing number="19-23" caption="Sadece `x` için eşleşip `y` ve `z` alanlarını yoksayarak bir `Point` değerinin tüm alanlarını yoksaymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-23/src/main.rs}}
```

</Listing>

`x` değerini listeleyip ardından sadece `..` desenini ekliyoruz. Bu, özellikle çok sayıda alanı olan struct'larla çalışırken, yalnızca bir veya iki alana ihtiyaç duyduğumuz durumlarda, `y: _` ve `z: _` listelemekten daha hızlıdır.

`..` sözdizimi, ihtiyaç duyduğu kadar genişleyecektir. 19-24 numaralı liste, bir tuple ile `..` kullanımını göstermektedir.

<Listing number="19-24" file-name="src/main.rs" caption="Bir tuple'daki yalnızca ilk ve son değerleri eşleştirip, diğer tüm değerleri yoksaymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-24/src/main.rs}}
```

</Listing>

Bu kodda, ilk ve son değerler `first` ve `last` ile eşleştirilir. `..` ortadaki her şeyi eşleştirip yoksayacaktır.

Ancak, `..` kullanımı belirsiz olmamalıdır. Eşleşmesi gereken ve hangi değerlerin yoksayılacağı konusunda belirsizlik varsa, Rust bize bir hata verecektir. 19-25 numaralı liste, belirsiz bir şekilde `..` kullanmaya çalıştığımız bir örneği göstermektedir; bu nedenle derlenmeyecektir.

<Listing number="19-25" file-name="src/main.rs" caption="Belirsiz bir şekilde `..` kullanma girişimi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-25/src/main.rs}}
```

</Listing>

Bu örneği derlediğimizde şu hatayı alırız:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-25/output.txt}}
```

Rust'ın, `second` ile bir değer eşleştirmeden önce kaç değeri yoksayacağını ve sonrasında kaç değeri yoksayacağını belirlemesi imkansızdır. Bu kod, `2`'yi yoksaymak, `second`'ı `4`'e bağlamak ve ardından `8`, `16` ve `32`'yi yoksaymak isteyip istemediğimizi veya `2` ve `4`'ü yoksaymak, `second`'ı `8`'e bağlamak ve ardından `16` ve `32`'yi yoksaymak isteyip istemediğimizi belirlemek için iki anlamına gelebilir. Değişken adı `second`, Rust için özel bir anlam ifade etmez, bu yüzden bu şekilde `..` kullanmak derleyici hatasına yol açar.

### Match Guard'larla Ekstra Koşullar

Bir _match guard_, bir `match` kolundaki desenden sonra belirtilen ek bir `if` koşuludur ve o kolun seçilmesi için o kolun da eşleşmesi gerekir. Match guard'lar, bir desenin tek başına ifade edemeyeceği daha karmaşık fikirleri ifade etmek için faydalıdır. Ancak, yalnızca `match` ifadelerinde mevcuttur, `if let` veya `while let` ifadelerinde değil.

Koşul, desende oluşturulan değişkenleri kullanabilir. 19-26 numaralı liste, ilk kolu `Some(x)` deseni olan ve ayrıca `x % 2 == 0` (bu, sayı çiftse `true` olacaktır) koşuluna sahip bir `match` örneğini gösterir.

<Listing number="19-26" caption="Bir desene match guard eklemek">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-26/src/main.rs:here}}
```

</Listing>

Bu örnek, `The number 4 is even` yazdırır. `num`, ilk kolun deseniyle karşılaştırıldığında, `Some(4)` deseni `Some(x)` ile eşleşir. Ardından, `x`'in 2 ile bölümünden kalanının 0'a eşit olup olmadığını kontrol eden match guard devreye girer ve eğer öyleyse, ilk kol seçilir.

Eğer `num` değeri `Some(5)` olsaydı, ilk kolun match guard'ı `false` olurdu çünkü 5'in 2'ye bölümünden kalan 1'dir ve bu 0'a eşit değildir. Rust, ikinci kola geçer; bu kol, bir match guard'a sahip olmadığı için herhangi bir `Some` varyantıyla eşleşir.

`if x % 2 == 0` koşulunu bir desende ifade etmenin bir yolu yoktur, bu yüzden match guard, bu mantığı ifade etme yeteneği sağlar. Bu ek ifade yeteneğinin dezavantajı, derleyicinin match guard ifadeleriyle ilgili olarak tamlık kontrolü yapmamasıdır.

19-11 numaralı listede, match guard'ların gölgeleme sorununu nasıl çözdüğümüzü belirtmiştik. Hatırlarsanız, `match` ifadesinde yeni bir değişken oluşturmuştuk; bu, dışarıdaki değişkenle aynı isme sahip yeni bir değişken anlamına geliyordu. Bu yeni değişken, dışarıdaki değişkenin değerine karşı test edilemezdi. 19-27 numaralı liste, bu sorunu çözmek için bir match guard nasıl kullanabileceğimizi gösterir.

<Listing number="19-27" file-name="src/main.rs" caption="Bir eşleşme koruyucusu kullanarak dış değişkenle eşitliği test etme">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-27/src/main.rs}}
```

</Listing>

Bu kod artık `Default case, x = Some(5)` yazdırır. İkinci match kolundaki desen, dıştaki `y` değişkenini gölgeleyen yeni bir `y` değişkeni tanımlamaz; bu nedenle, match guard'da dıştaki `y` kullanılabilir. Deseni `Some(y)` olarak belirtmek yerine, yeni bir değişken olan `Some(n)` olarak belirtiriz. Bu, `match` dışında bir `n` değişkeni olmadığı için hiçbir şeyi gölgelemez. `n` ile `y` arasındaki karşılaştırma, dıştaki `y` ile aynı değere sahip bir değeri aramamıza olanak tanır.

Match guard'da `|` operatörünü de kullanarak birden fazla deseni belirtebilirsiniz; match guard koşulu tüm desenler için geçerli olacaktır. 19-28 numaralı liste, `|` kullanan bir deseni match guard ile birleştirmenin önceliğini gösterir. Bu örneğin önemli kısmı, `if y` match guard'ının hem `4`, hem `5`, hem de `6` için geçerli olduğudur; bu, yalnızca `6` için geçerli olduğu izlenimini verebilir.

<Listing number="19-28" caption="Bir desenle match guard'ı birleştirirken öncelik">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-28/src/main.rs:here}}
```

</Listing>

Match koşulu, `x`'in değeri `4`, `5` veya `6` olduğunda ve `y`'nin `true` olduğu durumlarda eşleşir. Bu kod çalıştığında, ilk kolun deseni `x` `4` olduğu için eşleşir, ancak `if y` match guard'ı `false` olduğu için ilk kol seçilmez. Kod, eşleşen ikinci kola geçer ve bu durumda `no` yazdırılır. Bunun nedeni, bir match guard'ın, belirtilen değerler listesinin sonundaki değere değil, tümüne uygulanmasıdır.

```text
(4 | 5 | 6) if y => ...
```

şeklinde davranır, bu da şu anlama gelir:

```text
4 | 5 | (6 if y) => ...
```

Kod çalıştırıldığında, öncelik davranışı açıktır: eğer match guard yalnızca `|` operatörüyle belirtilen değerler listesinin sonundaki değere uygulanıyorsa, kol eşleşir ve program `yes` yazdırır.

### `@` Bağlantıları

_At_ operatörü `@`, bir deseni eşleştirirken aynı anda o değeri tutan bir değişken oluşturmamıza olanak tanır. 19-29 numaralı listede, bir `Message::Hello` `id` alanının `3..=7` aralığında olup olmadığını test etmek istiyoruz. Ayrıca, kolun kodunda kullanabilmek için değeri `id` değişkenine bağlamak istiyoruz.

<Listing number="19-29" caption="Bir desende eşleştirirken bir değere bağlanmak için `@` kullanımı">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-29/src/main.rs:here}}
```

</Listing>

Bu örnek, `Found an id in range: 5` yazdırır. `id @` belirterek, aralığın eşleştiği değeri bir `id` adlı değişkende tutuyoruz ve aynı zamanda değerin aralık desenine uyup uymadığını test ediyoruz.

Deseni yalnızca bir aralık belirterek kullandığımız ikinci kol, o kolun koduyla birlikte `id` alanının gerçek değerini içeren bir değişkeni içermez. `id` alanının değeri 10, 11 veya 12 olabilir, ancak o desenle eşleşen kod, `id` alanındaki değeri bilmez. Desen kodu, o kolun kodunda kullanmak üzere `id` alanının değerini kaydetmediğimiz için o değeri kullanamaz.

Son kol, bir aralık belirtmeden yalnızca bir değişken belirttiğimiz için, o kolun kodunda `id` adlı bir değişkenle o değeri kullanma imkanına sahibiz. Ancak, bu kolun kodunda, ilk iki kol gibi, `id` alanının değerine herhangi bir test uygulamadık: bu desen, herhangi bir değeri eşleştirir.

`@` kullanmak, bir deseni test etmemizi ve bir değeri bir desende bağlamamızı sağlar.

## Özet

Rust'ın desenleri, farklı veri türlerini ayırt etmekte çok faydalıdır. `match` ifadelerinde kullanıldığında, Rust desenlerinizin her olası değeri kapsadığından emin olur; aksi takdirde programınız derlenmez. `let` ifadelerinde ve fonksiyon parametrelerinde desenler, bu yapıları daha kullanışlı hale getirir, değerleri daha küçük parçalara ayırma ve bu parçaları değişkenlere atama olanağı tanır. İhtiyaçlarımıza uygun basit veya karmaşık desenler oluşturabiliriz.

Kitabın sondan bir önceki bölümünde, Rust'ın çeşitli özelliklerinin bazı ileri düzey yönlerini inceleyeceğiz.
