## Yapıları Tanımlama ve Instantiating

Yapılar, [“The Tuple Type”][tuples]<!--
ignore --> bölümünde ele alınan tuple'lara benzer, çünkü her ikisi de birden fazla ilişkili değeri tutar. Tuple'lar gibi, bir struct'ın
parçaları farklı türlerde olabilir. Tuple'lardan farklı olarak, bir struct'ta
değerlerin ne anlama geldiğinin açık olması için her bir veri parçasını adlandırırsınız. Bu
adlarının eklenmesi, yapıların tuple'lardan daha esnek olduğu anlamına gelir: bir örneğin değerlerini belirtmek veya bunlara erişmek için verilerin sırasına
güvenmek zorunda değilsiniz.

Bir struct tanımlamak için `struct` anahtar sözcüğünü gireriz ve struct'ın tamamını adlandırırız. Bir
struct'ın adı,
gruplandırılan veri parçalarının önemini tanımlamalıdır. Daha sonra küme parantezleri içinde
_fields_ olarak adlandırdığımız veri parçalarının adlarını ve türlerini tanımlarız. Örneğin, Liste 5-1, bir kullanıcı hesabı hakkında bilgi depolayan bir
yapısını göstermektedir.

<Listing number="5-1" file-name="src/main.rs" caption="A `User` struct definition">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

Bir yapıyı tanımladıktan sonra kullanmak için, her bir alan için somut değerler belirterek
bu yapının bir _instance_'ını oluştururuz. Bir örneği
yapının adını belirterek oluştururuz ve ardından _`key:
value`_ çiftlerini içeren küme parantezleri ekleriz; burada anahtarlar alanların adları ve değerler de bu alanlarda saklamak istediğimiz
verileridir. Alanları
adresinde struct içinde bildirdiğimiz sırayla belirtmek zorunda değiliz. Başka bir deyişle,
struct tanımı tür için genel bir şablon gibidir ve örnekler türün değerlerini oluşturmak için bu şablondaki
adresini belirli verilerle doldurur. örneği için, belirli bir kullanıcıyı Liste 5-2'de gösterildiği gibi bildirebiliriz.

<Listing number="5-2" file-name="src/main.rs" caption="Creating an instance of the `User` struct">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

Bir yapıdan belirli bir değer elde etmek için nokta gösterimini kullanırız. Örneğin,
bu kullanıcının e-posta adresine erişmek için `user1.email` kullanırız. Örnek
değiştirilebilir ise, nokta gösterimini kullanarak ve
belirli bir alana atayarak bir değeri değiştirebiliriz. Liste 5-3, değişebilir bir `User` örneğinin `email`
alanındaki değerin nasıl değiştirileceğini gösterir.

<Listing number="5-3" file-name="src/main.rs" caption="Changing the value in the `email` field of a `User` instance">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

Tüm örneğin değiştirilebilir olması gerektiğine dikkat edin; Rust,
yalnızca belirli alanları değiştirilebilir olarak işaretlememize izin vermez. Herhangi bir ifadede olduğu gibi, fonksiyon gövdesindeki son ifade olarak struct'ın yeni bir
örneğini oluşturabilir ve
bu yeni örneği dolaylı olarak döndürebiliriz.

Liste 5-4,
verilen e-posta ve kullanıcı adı ile bir `User` örneği döndüren bir `build_user` işlevini göstermektedir. Active` alanı `true` değerini alır ve
`sign_in_count` `1` değerini alır.

<Listing number="5-4" file-name="src/main.rs" caption="A `build_user` function that takes an email and username and returns a `User` instance">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

İşlev parametrelerini struct
alanlarıyla aynı adla adlandırmak mantıklıdır, ancak `email` ve `username` alan adlarını ve
değişkenlerini tekrarlamak zorunda kalmak biraz sıkıcıdır. Eğer yapının daha fazla alanı olsaydı, her bir ismi
tekrarlamak daha da can sıkıcı olurdu. Neyse ki, kullanışlı bir steno var!
<!-- Old heading. Do not remove or links may break. -->

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### Alan Başlangıç Kısaltmasını Kullanma

Parametre adları ve struct alan adları
Liste 5-4'te tamamen aynı olduğu için,
`build_user`'ı yeniden yazmak için _field init shorthand_ sözdizimini kullanabiliriz, böylece Liste 5-5'te gösterildiği gibi tamamen aynı şekilde davranır ancak
`username` ve `email` tekrarına sahip olmaz.

<Listing number="5-5" file-name="src/main.rs" caption="A `build_user` function that uses field init shorthand because the `username` and `email` parameters have the same name as struct fields">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

Burada,
adresinde `email` adında bir alana sahip olan `User` yapısının yeni bir örneğini oluşturuyoruz. Biz `email` alanının değerini `build_user` fonksiyonunun
`email` parametresindeki değere ayarlamak istiyoruz. `email` alanı ve
`email` parametresi aynı ada sahip olduğundan, `email: email` yerine sadece
`email` yazmamız gerekir.

### Struct Update Sözdizimi ile Diğer Örneklerden Örnek Oluşturma

Bir yapının
aynı türden başka bir örneğindeki değerlerin çoğunu içeren ancak bazılarını değiştiren yeni bir örneğini oluşturmak genellikle yararlıdır. Bunu
_struct update syntax_ kullanarak yapabilirsiniz.

İlk olarak, Liste 5-6'da güncelleme sözdizimi olmadan düzenli olarak `user2`
içinde yeni bir `User` örneğinin nasıl oluşturulacağını gösteriyoruz. `E-posta` için yeni bir değer belirliyoruz ancak
bunun dışında Liste `5-2`de oluşturduğumuz `user1` ile aynı değerleri kullanıyoruz.

<Listing number="5-6" file-name="src/main.rs" caption="Creating a new `User` instance using all but one of the values from `user1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

struct update sözdizimini kullanarak, Listing 5-7'de
gösterildiği gibi aynı etkiyi daha az kodla elde edebiliriz. Sözdizimi `..`,
açıkça ayarlanmamış kalan alanların verilen örnekteki alanlarla aynı değere sahip olması gerektiğini belirtir.

<Listing number="5-7" file-name="src/main.rs" caption="Using struct update syntax to set a new `email` value for a `User` instance but to use the rest of the values from `user1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

`Listing 5-7`deki kod ayrıca `user2` içinde `email` için
farklı bir değere sahip olan ancak `username`,
`active` ve `sign_in_count` alanları için `user1` ile aynı değerlere sahip olan bir örnek oluşturur. Geriye kalan tüm alanların değerlerini
`user1`deki karşılık gelen alanlardan alması gerektiğini belirtmek için `..user1` en son gelmelidir
, ancak
yapının tanımındaki alanların sırasına bakılmaksızın, herhangi bir sırada istediğimiz kadar
alan için değer belirtmeyi seçebiliriz.

struct güncelleme sözdiziminin bir atama gibi `=` kullandığına dikkat edin; bunun nedeni
adresinin [“
Move ile Etkileşime Giren Değişkenler ve Veriler”][move]<!-- ignore --> bölümünde gördüğümüz gibi verileri taşımasıdır. Bu örnekte, `user2` oluşturduktan sonra artık
`user1` kullanamayız çünkü
`user1`in `username` alanındaki `String` `user2`ye taşınmıştır. Eğer `user2`ye
hem `email` hem de `username` için yeni `String` değerleri vermiş olsaydık ve böylece `user1`den sadece `active` ve `sign_in_count`
değerlerini kullanmış olsaydık, `user1`, `user2` oluşturulduktan sonra da geçerli olurdu.
Hem `active` hem de `sign_in_count`, `Copy` özelliğini uygulayan türlerdir, bu nedenle
[“Stack-Only Data: Copy”][copy]<!-- ignore -->
bölümünde tartıştığımız davranış geçerli olacaktır. Bu örnekte `user1.email` türünü de kullanabiliriz,
çünkü değeri `user1` türünün dışına taşınmamıştır.

### Farklı Türler Oluşturmak için Adlandırılmış Alanlar Olmadan Tuple Yapılarını Kullanma

Rust ayrıca _tuple structs_ adı verilen ve tuple'lara benzeyen yapıları da destekler.
Tuple structs, struct adının sağladığı ek anlama sahiptir, ancak alanlarıyla ilişkili
adlarına sahip değildir; bunun yerine, yalnızca
alanlarının türlerine sahiptirler. Tuple struct'lar, tüm tuple'a bir ad vermek
ve tuple'ı diğer tuple'lardan farklı bir tür yapmak istediğinizde ve her
alanını normal bir struct'ta olduğu gibi adlandırmanın ayrıntılı veya gereksiz olacağı durumlarda kullanışlıdır.

Bir tuple struct tanımlamak için, `struct` anahtar sözcüğü ile başlayın ve struct adını
ve ardından tuple içindeki türleri yazın. Örneğin, burada `Color` ve `Point` adında iki
tuple struct tanımlıyor ve kullanıyoruz:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

`black` ve `origin` değerlerinin farklı türlerde olduğuna dikkat edin, çünkü bunlar farklı tuple yapılarının
örnekleridir. Tanımladığınız her struct kendi türündedir,
struct içindeki alanlar aynı türde olsa bile. Örneğin
için, `Color` türünde bir parametre alan bir fonksiyon, her iki tür de üç `i32`
değerinden oluşsa bile, argüman olarak
`Point` alamaz. Aksi takdirde, tuple struct örnekleri,
bunları tek tek parçalarına ayırabilmeniz ve tek bir değere erişmek için dizin tarafından
takip edilen bir `.` kullanabilmeniz açısından tuple'lara benzer. Tuple'lardan farklı olarak, tuple struct'lar
onları yıktığınızda struct'ın türünü adlandırmanızı gerektirir. örneği için, `origin` noktasındaki
değerlerini `x`, `y` ve `z` adlı değişkenlere yıkmak için `let Point(x, y, z) = origin;` yazacağız.

### Herhangi Bir Alanı Olmayan Birim Benzeri Yapılar

Herhangi bir alana sahip olmayan yapılar da tanımlayabilirsiniz! Bunlara
_unit-like structs_ denir, çünkü [“The Tuple Type”][tuples]<!-- ignore --> bölümünde bahsettiğimiz
birim tipi olan `()` ile benzer şekilde davranırlar. Birim benzeri
yapılar, bir tür üzerinde bir özellik uygulamanız gerektiğinde ancak
türün kendisinde saklamak istediğiniz herhangi bir veriye sahip olmadığınızda yararlı olabilir. Özellikleri
Bölüm 10'da tartışacağız. İşte `AlwaysEqual` adlı bir birim yapının
bildirilmesi ve örneklenmesine ilişkin bir örnek:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

`AlwaysEqual`ı tanımlamak için `struct` anahtar sözcüğünü, istediğimiz ismi ve
adresini ve ardından noktalı virgül kullanırız. Küme parantezlerine ya da parantezlere gerek yok! Daha sonra benzer bir şekilde `subject` değişkeninde bir
`AlwaysEqual` örneği elde edebiliriz: herhangi bir küme parantezi veya parantez olmadan tanımladığımız
adını kullanarak. Daha sonra
bu tür için öyle bir davranış uygulayacağımızı düşünün ki, her
`AlwaysEqual` örneği her zaman başka bir türün her örneğine eşit olsun, belki de
test amacıyla bilinen bir sonuca sahip olmak için. Bu davranışı uygulamak için
herhangi bir veriye ihtiyacımız olmayacaktır! Bölüm 10'da özellikleri nasıl tanımlayacağınızı ve
bunları birim benzeri yapılar da dahil olmak üzere herhangi bir tür üzerinde nasıl uygulayacağınızı göreceksiniz.

> ### Struct Verilerinin Sahipliği
>
> Liste 5-1'deki `Kullanıcı` struct tanımında, `&str` string dilim tipi yerine sahip olunan `String`
> tipini kullandık. Bu bilinçli bir seçimdir
> çünkü bu yapının her örneğinin tüm verilerine sahip olmasını ve
> bu verilerin tüm yapı geçerli olduğu sürece geçerli olmasını istiyoruz.
>
> Yapıların
> başka bir şeyin sahip olduğu verilere referansları saklaması da mümkündür, ancak bunu yapmak için Bölüm 10'da
> tartışacağımız bir Rust özelliği olan _lifetimes_ kullanımı gerekir. Yaşam süreleri, bir struct
> tarafından referans verilen verilerin struct olduğu sürece geçerli olmasını sağlar. Diyelim ki
> referansını aşağıdaki gibi yaşam sürelerini belirtmeden bir struct içinde saklamaya çalıştınız; bu işe yaramayacaktır:
>
> <Listing file-name="src/main.rs">
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> </Listing>
>
> Derleyici, yaşam süresi belirleyicilerine ihtiyaç duyduğundan şikayet edecektir:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> Bu hata hakkında daha fazla bilgi için `rustc --explain E0106` dosyasını deneyin.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> Bölüm 10'da,
> referanslarını yapılarda saklayabilmeniz için bu hataları nasıl düzelteceğimizi tartışacağız, ancak şimdilik, `&str` gibi referanslar yerine `String` gibi sahip olunan
> türlerini kullanarak bu gibi hataları düzelteceğiz.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.md#Tuple-Türü
[move]: ch04-01-what-is-ownership.md#Move-ile-Etkileşime-Giren-Değişkenler-ve-Veriler
[copy]: ch04-01-what-is-ownership.md#Yalnızca-Yığın-Veriler:-Kopyalama
