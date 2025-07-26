## Jenerik Veri Tipleri

Fonksiyon imzaları veya
structs gibi öğeler için tanımlar oluşturmak için jenerikleri kullanırız ve bunları daha sonra birçok farklı somut veri türüyle kullanabiliriz. İlk olarak

 jeneriklerini kullanarak fonksiyonları, yapıları, enumları ve metotları nasıl tanımlayacağımıza bakalım. Daha sonra jeneriklerin kod performansını nasıl etkilediğini tartışacağız.

### Fonksiyon Tanımlarında

Jenerik kullanan bir fonksiyon tanımlarken, jenerikleri fonksiyonun
imzasına, genellikle
parametrelerinin ve dönüş değerinin veri tiplerini belirttiğimiz yere yerleştiririz. Bunu yapmak kodumuzu daha esnek hale getirir ve kod tekrarını önlerken fonksiyonumuzu çağıranlara
daha fazla işlevsellik sağlar.

En büyük fonksiyonumuzla devam edersek, Liste 10-4'te
her ikisi de bir dilimdeki en büyük değeri bulan iki fonksiyon gösterilmektedir. Daha sonra bunları jenerik kullanan tek bir
fonksiyonunda birleştireceğiz.

<Listing number="10-4" file-name="src/main.rs" caption="Two functions that differ only in their names and in the types in their signatures">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

</Listing>

Liste 10-3'te çıkardığımız `largest_i32` fonksiyonu
bir dilimdeki en büyük `i32`yi bulur. En büyük_char` fonksiyonu bir dilimdeki en büyük
`char`ı bulur. Fonksiyon gövdeleri aynı koda sahiptir, bu nedenle tek bir fonksiyona genel bir tip parametresi ekleyerek
yinelemeyi ortadan kaldıralım.

Yeni bir tek fonksiyonda tipleri parametrelendirmek için, tıpkı bir fonksiyonun değer parametreleri için yaptığımız gibi, tip
parametresini adlandırmamız gerekir. Tip parametresi adı olarak
herhangi bir tanımlayıcıyı kullanabilirsiniz. Ancak biz `T` kullanacağız çünkü
kuralına göre, Rust'ta tip parametre isimleri kısadır, genellikle sadece bir harftir ve
Rust'ın tip isimlendirme kuralı CamelCase'dir. _type_'ın kısaltması olan `T`, çoğu Rust programcısının varsayılan
seçimidir.

İşlevin gövdesinde bir parametre kullandığımızda, derleyicinin bu adın ne anlama geldiğini bilmesi için imzada
parametre adını bildirmemiz gerekir.
Benzer şekilde, bir fonksiyon imzasında bir tip parametre adı kullandığımızda, kullanmadan önce tip parametre adını bildirmek için
adresine sahibiz. Genel
`largest` işlevini tanımlamak için, tür adı bildirimlerini açılı parantezler içine,
`<>`, işlevin adı ile parametre listesi arasına aşağıdaki gibi yerleştiririz:

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

Bu tanımı şu şekilde okuyabiliriz: `largest` fonksiyonu
`T` tipi üzerinde geneldir. Bu fonksiyonun `list` adında bir parametresi vardır ve bu parametre
tipinde `T` değerlerinin bir dilimidir. En büyük` fonksiyonu
aynı `T` tipinde bir değere referans döndürecektir.

Listing 10-5, imzasında genel
veri tipini kullanan birleşik `largest` fonksiyon tanımını gösterir. Listeleme ayrıca
fonksiyonunu `i32` değerlerinden oluşan bir dilim ya da `char` değerleriyle nasıl çağırabileceğimizi de göstermektedir. Bu kodun henüz
derlenmeyeceğini unutmayın.

<Listing number="10-5" file-name="src/main.rs" caption="The `largest` function using generic type parameters; this doesn’t compile yet">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

</Listing>

Bu kodu şu anda derlersek, bu hatayı alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

Yardım metni bir _trait_ olan `std::cmp::PartialOrd`dan bahseder ve biz
bir sonraki bölümde traitler hakkında konuşacağız. Şimdilik, bu hatanın
`largest` gövdesinin `T`
'nin olabileceği tüm olası türler için çalışmayacağını belirttiğini bilin. Gövdede `T` türündeki değerleri karşılaştırmak istediğimiz için,
yalnızca değerleri sıralanabilen türleri kullanabiliriz. Karşılaştırmaları etkinleştirmek için, standart
kütüphanesi,
 türleri üzerinde uygulayabileceğiniz `std::cmp::PartialOrd` özelliğine sahiptir (bu özellik hakkında daha fazla bilgi için Ek C'ye bakın). Liste 10-5'i düzeltmek için,
yardım metninin önerisini takip edebilir ve `T` için geçerli türleri yalnızca
`PartialOrd` uygulayanlarla sınırlandırabiliriz. Listeleme daha sonra derlenecektir, çünkü standart
kütüphanesi hem `i32` hem de `char` üzerinde `PartialOrd` uygular.

### Struct Tanımlarında

Ayrıca, `<>` sözdizimini kullanarak bir veya daha fazla
alanında genel bir tür parametresi kullanmak için yapılar tanımlayabiliriz. Liste 10-6, herhangi bir tipteki
`x` ve `y` koordinat değerlerini tutmak için bir `Point<T>` struct'ı tanımlar.

<Listing number="10-6" file-name="src/main.rs" caption="A `Point<T>` struct that holds `x` and `y` values of type `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

</Listing>

Struct tanımlarında jenerikleri kullanmak için kullanılan sözdizimi
fonksiyon tanımlarında kullanılana benzer. İlk olarak, struct adından hemen sonra
köşeli parantez içinde tip parametresinin adını bildiririz. Ardından, struct tanımında somut veri
türlerini belirteceğimiz yerde genel
türünü kullanırız.

Nokta`<T>`yi tanımlamak için yalnızca bir genel tip kullandığımızdan, bu
tanımının `Nokta<T>` yapısının bazı `T` tipleri üzerinde genel olduğunu ve
`x` ve `y` alanlarının, bu tip ne olursa olsun, _ikisinin de_ aynı tip olduğunu söylediğini unutmayın. Eğer

 Liste 10-7'de olduğu gibi farklı tiplerde değerlere sahip bir `Point<T>` örneği yaratırsak, kodumuz derlenmeyecektir.

<Listing number="10-7" file-name="src/main.rs" caption="The fields `x` and `y` must be the same type because both have the same generic data type `T`.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

</Listing>

Bu örnekte, `x` öğesine `5` tamsayı değerini atadığımızda,
derleyicisine
`Point<T>` öğesinin bu örneği için `T` genel türünün bir tamsayı olacağını bildiririz. Daha sonra, `x` ile
aynı türe sahip olacak şekilde tanımladığımız `y` için `4.0` değerini belirttiğimizde, aşağıdaki gibi bir tür uyuşmazlığı hatası alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

`x` ve `y` türlerinin her ikisinin de jenerik olduğu ancak
farklı türlere sahip olabileceği bir `Point` yapısı tanımlamak için birden fazla jenerik tür parametresi kullanabiliriz. Örneğin,
Liste 10-8'de, `Point` tanımını `T`
ve `U` tipleri üzerinde genel olacak şekilde değiştiriyoruz, burada `x` `T` tipinde ve `y` `U` tipindedir.

<Listing number="10-8" file-name="src/main.rs" caption="A `Point<T, U>` generic over two types so that `x` and `y` can be values of different types">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

</Listing>

Artık gösterilen tüm `Point` örneklerine izin verilmektedir! Bir tanımda istediğiniz kadar genel
tip parametresi kullanabilirsiniz, ancak birkaç taneden fazla kullanmak
kodunuzun okunmasını zorlaştırır. Kodunuzda
çok sayıda genel tipe ihtiyaç duyuyorsanız
bu durum kodunuzun daha küçük parçalar halinde yeniden yapılandırılması gerektiğini gösterebilir.

### Enum Tanımlarında

Yapılarda yaptığımız gibi, genel veri tiplerini
varyantlarında tutmak için enum'ları tanımlayabiliriz. Bölüm 6'da kullandığımız standart
kütüphanesinin sağladığı `Option<T>` enumuna bir kez daha göz atalım:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Bu tanım şimdi size daha anlamlı gelecektir. Gördüğünüz gibi,
`Option<T>` enumu `T` tipi üzerinde geneldir ve iki çeşidi vardır:
`T` tipinde bir değer tutan `Some` ve herhangi bir değer tutmayan `None` çeşidi.
Opsiyon<T>` enumunu kullanarak, soyut bir
isteğe bağlı değer kavramını ifade edebiliriz ve `Opsiyon<T>` genel olduğundan, isteğe bağlı değerin türü ne olursa olsun bu soyutlamayı
kullanabiliriz.

Enumlar birden fazla jenerik tip de kullanabilir. Bölüm 9'da kullandığımız `Result`
enum tanımı buna bir örnektir:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Sonuç` enumu `T` ve `E` olmak üzere iki tip üzerinde geneldir ve iki çeşidi vardır:
`T` tipinde bir değer tutan `Ok` ve
`E` tipinde bir değer tutan `Err`. Bu tanım,
başarılı olabilecek (`T` türünde bir değer döndüren) veya başarısız olabilecek (`E` türünde bir hata döndüren)
 bir işleme sahip olduğumuz her yerde `Result` enumunu kullanmayı kolaylaştırır. Aslında, Listing 9-3'te bir
dosyasını açmak için kullandığımız şey buydu; burada
dosya başarıyla açıldığında `T`, `std::fs::File` türüyle dolduruldu ve dosyanın açılmasında sorun olduğunda `E`,
`std::io::Error` türüyle dolduruldu.

Kodunuzda yalnızca tuttukları değerlerin türlerinde farklılık gösteren birden fazla struct veya enum
tanımının bulunduğu durumları fark ettiğinizde
bunun yerine genel türleri kullanarak yinelemeden kaçınabilirsiniz.

### Yöntem Tanımlarında

Yapılar ve enumlar üzerinde yöntemler uygulayabilir (Bölüm 5'te yaptığımız gibi) ve tanımlarında
genel türlerini de kullanabiliriz. Liste 10-9, Liste 10-6'da tanımladığımız `Point<T>`
yapısını ve üzerinde uygulanan `x` adlı bir yöntemi göstermektedir.

<Listing number="10-9" file-name="src/main.rs" caption="Implementing a method named `x` on the `Point<T>` struct that will return a reference to the `x` field of type `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

</Listing>

Burada, `x` alanındaki veriye
referansını döndüren `Point<T>` üzerinde `x` adlı bir yöntem tanımladık.

T`yi `impl`den hemen sonra bildirmemiz gerektiğine dikkat edin, böylece
adresinde `Point<T>` türü üzerinde yöntemler uyguladığımızı belirtmek için `T`yi kullanabiliriz. Rust, `impl` türünden sonra `T` türünü bir
jenerik türü olarak bildirerek, `Point` türündeki
köşeli parantez içindeki türün somut bir türden ziyade jenerik bir tür olduğunu belirleyebilir. Bu jenerik parametre için
struct tanımında bildirilen jenerik
parametresinden farklı bir isim seçebilirdik, ancak aynı ismi kullanmak
gelenekseldir. Bir `impl` içinde genel bir
türü bildiren bir yöntem yazarsanız, bu yöntem,
somut türünün genel türün yerine geçmesi ne olursa olsun, türün herhangi bir örneği üzerinde tanımlanacaktır.

Ayrıca
türünde yöntemler tanımlarken genel türler üzerinde kısıtlamalar da belirleyebiliriz. Örneğin, herhangi bir jenerik tipe sahip `Point<T>` örnekleri yerine yalnızca `Point<f32>` örnekleri
üzerinde yöntemler uygulayabiliriz. Liste 10-10'da
somut `f32` tipini kullanıyoruz, yani `impl`den sonra herhangi bir tip bildirmiyoruz.

<Listing number="10-10" file-name="src/main.rs" caption="An `impl` block that only applies to a struct with a particular concrete type for the generic type parameter `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

</Listing>

Bu kod, `Point<f32>` türünün bir `distance_from_origin`
yöntemine sahip olacağı anlamına gelir; `T` türünün `f32` olmadığı diğer `Point<T>` örnekleri
bu yönteme sahip olmayacaktır. Yöntem, noktamızın (0.0, 0.0) koordinatlarındaki
noktasından ne kadar uzakta olduğunu ölçer ve
yalnızca kayan nokta türleri için kullanılabilen matematiksel işlemleri kullanır.

Bir struct tanımındaki jenerik tip parametreleri her zaman aynı struct'ın metot imzalarında kullandığınız
parametrelerle aynı değildir. Liste 10-11, örneği daha anlaşılır kılmak için `Point` yapısı için `X1` ve `Y1` genel
türlerini ve `mixup` yöntemi için `X2` `Y2`
imzasını kullanır. Yöntem, `self` `Point` (`X1` türünden) `x` değeri ve aktarılan `Point` (`Y2` türünden) `y`
değeri ile yeni bir `Point`
örneği oluşturur.

<Listing number="10-11" file-name="src/main.rs" caption="A method that uses generic types different from its struct’s definition">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

</Listing>

`main` fonksiyonunda, `x` alanı için `i32` türünde bir değer (`5`) ve `y` alanı için `f64` türünde bir değer (`10.4`) içeren bir `Point` tanımladık. `p2` değişkeni ise `x` alanı için bir string dilimi (`"Hello"`) ve `y` alanı için bir `char` türünde değer (`c`) içeren başka bir `Point` yapısıdır. `p1` üzerinde `p2` argümanıyla `mixup` fonksiyonunu çağırdığımızda elde ettiğimiz `p3`, `x` değerini `p1`'den aldığı için `i32` türünde bir `x` alanına sahip olacaktır. `p3` değişkeninin `y` alanı ise `p2`'den geldiği için `char` türünde olacaktır. `println!` makrosu çağrısı `p3.x = 5, p3.y = c` çıktısını verecektir.

Bu örneğin amacı, bazı genel tür parametrelerinin `impl` ile, bazılarının ise metod tanımıyla belirtildiği durumları göstermektir. Burada, `X1` ve `Y1` genel tür parametreleri `impl`'den sonra belirtilmiştir çünkü yapı tanımıyla ilişkilidir. `X2` ve `Y2` genel tür parametreleri ise yalnızca metodla ilgili olduğu için `fn mixup`'tan sonra belirtilmiştir.

### Generic Kullanan Kodun Performansı

Generic tür parametreleri kullanmanın çalışma zamanında bir maliyeti olup olmadığını merak ediyor olabilirsiniz. İyi haber şu ki, generic türler kullanmak programınızı somut türler kullandığınız duruma göre daha yavaş çalıştırmayacaktır.

Rust bunu, derleme zamanında
generics kullanarak kodun monomorfizasyonunu gerçekleştirerek başarır. Monomorfizasyon_
derlendiğinde kullanılan somut tiplerin içini doldurarak genel
kodunu özel koda dönüştürme işlemidir. Bu işlemde derleyici
Liste 10-5'teki genel işlevi oluşturmak için kullandığımız adımların tersini yapar: derleyici
genel kodun çağrıldığı tüm yerlere bakar ve genel kodun çağrıldığı somut türler
için kod oluşturur.

Standart kütüphanenin genel
`Option<T>` enumunu kullanarak bunun nasıl çalıştığına bakalım:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Rust bu kodu derlediğinde, monomorfizasyon gerçekleştirir. Bu
işlemi sırasında, derleyici `Option<T>`
örneklerinde kullanılan değerleri okur ve iki tür `Option<T>` tanımlar: biri `i32` ve diğer
`f64`. Bu nedenle, `Option<T>` genel tanımını `i32` ve `f64` için özelleşmiş iki
tanımına genişletir, böylece genel
tanımını özel olanlarla değiştirir.

Kodun monomorfize edilmiş versiyonu aşağıdakine benzer (
derleyicisi burada örnekleme için kullandığımızdan farklı isimler kullanır):

<Listing file-name="src/main.rs">

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

</Listing>

Genel `Option<T>`, derleyici tarafından
tarafından oluşturulan özel tanımlarla değiştirilir. Rust, jenerik kodu her örnekte
türünü belirten koda derlediğinden, jenerikleri kullanmak için çalışma zamanı maliyeti ödemeyiz. Kod
çalıştığında, her bir tanımı
eliyle çoğaltmış olsaydık yapacağı gibi çalışır. Monomorfizasyon süreci, Rust'ın jeneriklerini çalışma zamanında
son derece verimli hale getirir.
