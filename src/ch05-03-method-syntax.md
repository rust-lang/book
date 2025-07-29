## Yöntem Sözdizimi

_Methods_ fonksiyonlara benzer: onları `fn` anahtar sözcüğü ve bir
adı, parametreleri ve bir dönüş değeri olabilir ve bazı kodlar içerirler
yöntem başka bir yerden çağrıldığında çalıştırılır. Fonksiyonların aksine,
yöntemler bir struct (veya bir enum veya bir trait) bağlamında tanımlanır
nesnesi, [Bölüm 6][enums]<!-- ignore --> ve [Bölüm 18][trait-objects]<!-- ignore -->, sırasıyla) ve ilk parametreleri
her zaman `self`, yöntemin uygulandığı yapının örneğini temsil eder
çağırdı.

### tanımlama yöntemleri

Parametre olarak bir `Rectangle` örneğine sahip olan `area` fonksiyonunu değiştirelim
ve bunun yerine `Rectangle` yapısında tanımlanmış bir `area` yöntemi oluşturun, gösterildiği gibi
Liste 5-13'te.

<Listing number="5-13" file-name="src/main.rs" caption="Defining an `area` method on the `Rectangle` struct">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

Fonksiyonu `Rectangle` bağlamında tanımlamak için bir `impl` başlatıyoruz
`Rectangle` için (uygulama) bloğu. Bu `impl` bloğu içindeki her şey
`Rectangle` tipi ile ilişkilendirilecektir. Sonra `area` fonksiyonunu hareket ettiririz
'i `impl` küme parantezleri içinde değiştirin ve ilk (ve bu durumda sadece)
parametresinin imzada ve gövdenin her yerinde `self` olması gerekir. İçinde
`main`, burada `area` fonksiyonunu çağırdık ve `rect1` fonksiyonunu argüman olarak geçirdik,
bunun yerine `Rectangle` üzerinde `area` yöntemini çağırmak için _method syntax_ kullanabiliriz
instance. Yöntem sözdizimi bir örnekten sonra gelir: bir nokta ekleriz ve ardından
yöntem adı, parantezler ve herhangi bir argüman.

`Area` imzasında, `rectangle: &Rectangle` yerine `&self` kullanıyoruz.
Aslında `&self`, `self: &Self` ifadesinin kısaltmasıdır. Bir `impl` bloğu içinde
`Self` tipi, `impl` bloğunun ait olduğu tip için bir takma addır. Yöntemler şunları içermelidir
ilk parametresi `Self` türünde `self` adlı bir parametreye sahip olduğundan, Rust
bunu ilk parametre yerinde sadece `self` ismiyle kısaltmanıza izin verir.
Yine de `self` kısaltmasının önünde `&` kullanmamız gerektiğine dikkat edin.
'de yaptığımız gibi, bu yöntemin `Self` örneğini ödünç aldığını gösterir.
`Rectangle: &Rectangle`. Yöntemler `self`in sahipliğini alabilir, `self`i ödünç alabilir
Burada yaptığımız gibi değişmez bir şekilde veya herhangi bir şekilde yapabildikleri gibi `kendini' değişken bir şekilde ödünç alabilirler.
diğer parametre.

Burada `&self` ifadesini seçmemizin nedeni, fonksiyonda `&Rectangle` ifadesini kullanmamızla aynıdır
versiyonu: sahiplik almak istemiyoruz ve sadece verileri okumak istiyoruz
struct'a yazmak değil. Eğer sahip olduğumuz örneği değiştirmek istersek
metodun ne yaptığının bir parçası olarak metodu çağırdıysak, `&mut self` ifadesini
ilk parametre. Örneklerin sahipliğini alan bir metoda sahip olmak
ilk parametre olarak sadece `self` kullanılması nadirdir; bu teknik genellikle
yöntem `self`i başka bir şeye dönüştürdüğünde kullanılır ve
çağıranın dönüşümden sonra orijinal örneği kullanmasını engeller.

İşlevler yerine yöntemlerin kullanılmasının ana nedeni, ek olarak
yöntem sözdizimi sağlamak ve `self` türünü her seferinde tekrarlamak zorunda kalmamak
yönteminin imzası, organizasyon içindir. Yapabileceğimiz her şeyi koyduk
bir türün örneğini tek bir `impl` bloğunda kullanarak gelecekteki kullanıcıları
kodumuzun çeşitli yerlerinde `Rectangle` yeteneklerini arayın.
sağladığımız kütüphane.

Bir yönteme struct'lardan biriyle aynı adı vermeyi seçebileceğimizi unutmayın
alanlar. Örneğin, `Dikdörtgen` üzerinde aşağıdaki gibi adlandırılmış bir yöntem tanımlayabiliriz
`genişlik`:


<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

Burada, `width` metodunun `true` değerini döndürmesini seçiyoruz, eğer
örneğinin `width` alanı `0`dan büyükse ve değer `false` ise
`0`: aynı isimli bir metodun içindeki bir alanı herhangi bir amaç için kullanabiliriz. İçinde
`main`, `rect1.width` ifadesini parantezle takip ettiğimizde, Rust bizim
metodu `width`. Parantez kullanmadığımızda, Rust alanı kastettiğimizi bilir
genişlik'.

Genellikle, ancak her zaman değil, bir yönteme istediğimiz bir alanla aynı adı verdiğimizde
yalnızca alandaki değeri döndürür ve başka hiçbir şey yapmaz. Bunun gibi yöntemler
_getters_ olarak adlandırılır ve Rust bunları struct
diğer bazı dillerde olduğu gibi alanlar. Getter'lar kullanışlıdır çünkü
alanına özel ancak yönteme genel erişim sağlar ve böylece bu alana salt okunur erişim sağlar
alanı türün genel API'sinin bir parçası olarak. Kamu ve özel sektörün ne olduğunu tartışacağız.
'de bir alanın veya yöntemin public veya private olarak nasıl belirleneceği ve [Bölüm 7][public]<!-- görmezden gel -->.


> ### `->` Operatörü nerede?
>
> C ve C++'da metotları çağırmak için iki farklı operatör kullanılır
> Eğer nesne üzerinde doğrudan bir metot çağırıyorsanız `.` ve eğer bir metot çağırıyorsanız `->`
> yöntemi nesnenin bir işaretçisi üzerinde çağırmak ve
> önce işaretçi. Başka bir deyişle, eğer `object` bir işaretçi ise,
> `object->something()`, `(*object).something()` ile benzerdir.
>
> Rust'ta `->` işlecine eşdeğer bir işleç yoktur; bunun yerine Rust'ta
> _automatic referencing and dereferencing_ olarak adlandırılan özellik. Yöntemleri çağırmak
> Rust'ta bu davranışa sahip birkaç yerden biri.
>
> Şöyle çalışır: Bir yöntemi `object.something()` ile çağırdığınızda, Rust
> otomatik olarak `&`, `&mut` veya `*` ekler, böylece `object` imzasıyla eşleşir
> yöntem. Başka bir deyişle, aşağıdakiler aynıdır:

>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> İlki çok daha temiz görünüyor. Bu otomatik referanslama davranışı çalışır
> Çünkü metotların net bir alıcısı vardır: `self` tipi. Alıcı göz önüne alındığında
> ve bir yöntemin adı ile karşılaştırıldığında, Rust yöntemin
> okuma (`&self`), mutasyona uğratma (`&mut self`) veya tüketme (`self`). Gerçek şu ki
> Rust'ın metot alıcıları için ödünç almayı örtük hale getirmesi
> sahipliğin pratikte ergonomik hale getirilmesi.


### Daha Fazla Parametreli Yöntemler

Şimdi `Rectangle` üzerinde ikinci bir yöntem uygulayarak yöntemleri kullanma alıştırması yapalım
struct. Bu sefer bir `Rectangle` örneğinin başka bir örnek almasını istiyoruz
'nin `Dikdörtgen` olarak tanımlanmasını ve ikinci `Dikdörtgen` tamamen sığabiliyorsa `true` döndürülmesini
içinde (ilk `Rectangle`); aksi takdirde, `false` döndürmelidir.
Yani, `can_hold` metodunu tanımladıktan sonra şunları yazabilmek istiyoruz
Liste 5-14'te gösterilen program.


<Listing number="5-14" file-name="src/main.rs" caption="Using the as-yet-unwritten `can_hold` method">

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

Beklenen çıktı aşağıdaki gibi görünecektir çünkü her iki boyut da
`rect2`, `rect1` boyutlarından daha küçüktür, ancak `rect3`, `rect1` boyutlarından daha geniştir.
`rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Bir yöntem tanımlamak istediğimizi biliyoruz, bu yüzden `impl Rectangle` içinde olacak
blok. Yöntemin adı `can_hold` olacaktır ve değişmez bir ödünç alacaktır
parametre olarak başka bir `Rectangle` öğesini alır. Dikdörtgenin tipinin ne olduğunu söyleyebiliriz
parametresi, yöntemi çağıran koda bakarak olacaktır:
`rect1.can_hold(&rect2)` metoduna değişmez bir ödünç olan `&rect2` parametresi aktarılır.
`rect2`, bir `Rectangle` örneği. Bu mantıklı çünkü sadece
`rect2`yi okuyun (yazmak yerine, bu da değişebilir bir ödünç almaya ihtiyacımız olduğu anlamına gelir),
ve `main`in `rect2`nin sahipliğini elinde tutmasını istiyoruz, böylece onu
`can_hold` yöntemini çağırır. `can_hold`un dönüş değeri bir
Boolean ve uygulama, genişlik ve yüksekliğin
'nin genişliği ve yüksekliği diğer `Rectangle`in genişliği ve yüksekliğinden büyüktür,
sırasıyla. Yeni `can_hold` metodunu `impl` bloğuna şuradan ekleyelim
Liste 5-13, Liste 5-15'te gösterilmiştir.


<Listing number="5-15" file-name="src/main.rs" caption="Implementing the `can_hold` method on `Rectangle` that takes another `Rectangle` instance as a parameter">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

Bu kodu Listing 5-14'teki `main` fonksiyonu ile çalıştırdığımızda
istenen çıktı. Metotlar birden fazla parametre alabilir ve bu parametreleri
imzası `self` parametresinden sonra gelir ve bu parametreler tıpkı
fonksiyonlardaki parametreler.

### İlişkili Fonksiyonlar

Bir `impl` bloğu içinde tanımlanan tüm fonksiyonlar _ilişkili fonksiyonlar_ olarak adlandırılır
çünkü `impl`den sonra adlandırılan türle ilişkilidirler. Şunları tanımlayabiliriz
ilk parametresi `self` olmayan ilişkili fonksiyonlar (ve dolayısıyla
yöntem değildir) çünkü çalışmak için türün bir örneğine ihtiyaç duymazlar.
Bunun gibi bir fonksiyonu zaten kullandık: `String::from` fonksiyonu
String` türü üzerinde tanımlanmıştır.

Yöntem olmayan ilişkili işlevler genellikle aşağıdaki yapıcılar için kullanılır
struct'ın yeni bir örneğini döndürecektir. Bunlar genellikle `new` olarak adlandırılır, ancak
`new` özel bir isim değildir ve dilin içinde yerleşik değildir. Örneğin, biz
'kare' adında ilişkili bir fonksiyon sağlamayı seçebilir.
bir boyut parametresi ve bunu hem genişlik hem de yükseklik olarak kullanın, böylece
aynı şeyi belirtmek zorunda kalmak yerine kare bir `Rectangle` oluşturmak daha kolaydır
değeri iki kez:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

Dönüş türündeki ve işlevin gövdesindeki `Self` anahtar sözcükleri şunlardır
Bu durumda `impl` anahtar sözcüğünden sonra görünen tür için takma adlar
`Rectangle`dir.

Bu ilişkili fonksiyonu çağırmak için, yapı adıyla birlikte `::` sözdizimini kullanırız;
`let sq = Rectangle::square(3);` bir örnektir. Bu fonksiyonun isim alanı
struct: `::` sözdizimi hem ilişkili işlevler hem de
modüller tarafından oluşturulan ad alanları. Modülleri [Bölüm 7][modules]<!-- ignore --> bölümünde tartışacağız.

### Çoklu `impl` Blokları

Her yapının birden fazla `impl` bloğuna sahip olmasına izin verilir. Örneğin, Listeleme
5-15, Liste 5-16'da gösterilen koda eşdeğerdir; bu kodda her bir yöntem
kendi `impl` bloğu.

<Listing number="5-16" caption="Rewriting Listing 5-15 using multiple `impl` blocks">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

Burada bu yöntemleri birden fazla `impl` bloğuna ayırmak için bir neden yoktur,
ancak bu geçerli bir sözdizimidir. Birden fazla `impl` bloğunun olduğu bir durum göreceğiz
genel tipleri ve özellikleri tartıştığımız Bölüm 10'da kullanışlıdır.

## Özet

Yapılar, etki alanınız için anlamlı olan özel türler oluşturmanıza olanak tanır. Tarafından
yapıları kullanarak, ilişkili veri parçalarını birbirlerine bağlı tutabilirsiniz
ve kodunuzu anlaşılır kılmak için her bir parçayı adlandırın. Impl` bloklarında şunları tanımlayabilirsiniz
fonksiyonlarıdır ve metotlar, türünüzle ilişkili bir tür
örneklerinin davranışını belirlemenize izin veren ilişkili işlev
yapılara sahiptir.

Ancak struct'lar özel tipler oluşturmanın tek yolu değildir: şimdi de
Araç kutunuza başka bir araç eklemek için Rust'ın enum özelliğini kullanın.

[enums]: ch06-00-enums.md
[trait-objects]: ch18-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md#Yolları-`pub`-Anahtar-Sözcüğü-ile-Açığa-Çıkarma
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.md
