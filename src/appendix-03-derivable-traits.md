## Ek C: Türetilebilir Özellikler

Kitabın çeşitli yerlerinde, `derive` özelliğini tartıştık.
bir struct veya enum tanımına uygulayabilirsiniz. `derive` niteliği şunları oluşturur
üzerinde kendi varsayılan uygulaması ile bir özelliği uygulayacak kod
sözdizimi ile ek açıklama eklediğiniz tür.

Bu ekte, standarttaki tüm özelliklerin bir referansını sunuyoruz
ile kullanabileceğiniz kütüphane. Her bölüm şunları kapsar:

- Bu özelliği türeten operatörler ve yöntemler neyi mümkün kılacaktır?
- `derive` tarafından sağlanan özelliğin uygulaması ne yapar
- Özelliğin uygulanması tür hakkında ne anlama gelir?
- Özelliği uygulamanıza izin verilen veya verilmeyen koşullar
- Özellik gerektiren işlemlere örnekler

Eğer `derive` niteliği tarafından sağlanandan farklı bir davranış istiyorsanız, [standart kütüphane belgelerine bakın](../std/index.md)<!-- ignore -->
manuel olarak nasıl uygulanacağına ilişkin ayrıntılar için her bir özellik için.

Burada listelenen özellikler, standart kütüphane tarafından tanımlanan tek özelliklerdir.
türlerinizde `derive` kullanılarak uygulanabilir. Burada tanımlanan diğer özellikler
standart kütüphanesi mantıklı varsayılan davranışa sahip değildir, bu nedenle
Bunları, başarmaya çalıştığınız şey için mantıklı olan şekilde uygulayın.

Türetilemeyen bir özelliğe örnek olarak `Display` verilebilir.
son kullanıcılar için biçimlendirme. Her zaman aşağıdakiler için uygun yolu düşünmelisiniz
bir tipi son kullanıcıya gösterme. Son kullanıcı tipin hangi kısımlarını
görmelerine izin verilir mi? Hangi kısımları ilgili bulurlar? Verilerin hangi formatı
onlar için en uygun olanı mı? Rust derleyicisi bu içgörüye sahip değildir, bu yüzden
sizin için uygun varsayılan davranışı sağlayamaz.

Bu ekte verilen türetilebilir özelliklerin listesi kapsamlı değildir:
kütüphaneler kendi özellikleri için `derive` özelliğini uygulayabilir, böylece
özelliklerini `derive` ile gerçekten açık uçlu olarak kullanabilirsiniz. derive` uygulamasını gerçekleştirme
prosedürel bir makro kullanmayı içerir, bu da
Bölüm 20'nin [“Makrolar”][makrolar]<!-- yoksay --> bölümü.

### Programcı Çıktısı için `Debug`

Debug` özelliği, biçim dizelerinde hata ayıklama biçimlendirmesini etkinleştirir.
`{}` yer tutucularının içine `:?` ekleyerek belirtebilirsiniz.

Debug` özelliği, hata ayıklama için bir türün örneklerini yazdırmanıza olanak tanır
Böylece siz ve türünüzü kullanan diğer programcılar bir örneği inceleyebilir
bir programın yürütülmesinde belirli bir noktada.

Örneğin, `assert_eq!` makrosunun kullanımında `Debug` özelliği gereklidir.
makro. Bu makro, aşağıdaki durumlarda bağımsız değişken olarak verilen örneklerin değerlerini yazdırır
eşitlik iddiası başarısız olur, böylece programcılar iki örneğin neden
eşit.

### Eşitlik Karşılaştırmaları için `PartialEq` ve `Eq`

`PartialEq` özelliği, bir türün örneklerini karşılaştırarak aşağıdakileri kontrol etmenizi sağlar
eşitliğini sağlar ve `==` ve `!=` operatörlerinin kullanımını mümkün kılar.

`PartialEq` türetildiğinde `eq` yöntemi uygulanır. `PartialEq` türetildiğinde
yapılarında, iki örnek yalnızca _all_ alanları eşitse eşittir ve
herhangi bir alan eşit değilse örnekler eşit değildir. Enumlar üzerinde türetildiğinde,
her varyant kendisine eşittir ve diğer varyantlara eşit değildir.

Örneğin, `PartialEq` özelliğinin kullanılması gereklidir.
Bir türün iki örneğini karşılaştırabilmesi gereken `assert_eq!` makrosu
eşitlik için.

`Eq` özelliğinin hiçbir yöntemi yoktur. Amacı, her değer için şu sinyali vermektir
ek açıklamalı türde, değer kendisine eşittir. `Eq` özelliği yalnızca
'yi uygulayan tiplere de uygulanır, ancak `PartialEq` uygulayan tüm tipler
`PartialEq` uygulayanlar `Eq` uygulayabilir. Bunun bir örneği kayan noktadır
sayı türleri: kayan noktalı sayıların uygulanması, iki
not-a-number (`NaN`) değerinin örnekleri birbirine eşit değildir.


Bir `HashMap<K, V>` içindeki anahtarlar için `Eq` gerekli olduğunda bir örnek
`HashMap<K, V>` iki anahtarın aynı olup olmadığını söyleyebilir. 

### Bir `HashMap<K, V>` içindeki anahtarlar için `Eq` gerekli olduğunda bir örnek
`HashMap<K, V>` iki anahtarın aynı olup olmadığını söyleyebilir.

`PartialOrd` özelliği, sıralama için bir türün örneklerini karşılaştırmanıza olanak tanır
amaçlar. `ParsialOrd` türünü uygulayan bir tür `<`, `>` ile kullanılabilir,
`<=` ve `>=` operatörlerini kullanabilirsiniz. Sadece `PartialOrd` özelliğini türlere uygulayabilirsiniz
aynı zamanda `PartialEq` yöntemini de uygular.

`PartialOrd` türevi, `partial_cmp` yöntemini uygular ve bu yöntem bir
`Option<Ordering>`, verilen değerler bir sıralama üretmediğinde `None` olacaktır.
sıralama. Bir sıralama üretmeyen bir değer örneği, buna rağmen
Bu türdeki çoğu değer karşılaştırılabilir, bir sayı olmayan (`NaN`) kayan
nokta değeri. Herhangi bir kayan noktalı sayı ve `NaN` ile `partial_cmp` çağrısı
kayan noktalı değer `None` döndürür.

Yapılar üzerinde türetildiğinde, `PartialOrd` iki örneği karşılaştırmak için
her bir alandaki değer, alanların yapıda görünme sırasına göre
tanım. Enumlar üzerinde türetildiğinde, daha önce bildirilen enum'un varyantları
enum tanımı, daha sonra listelenen varyantlardan daha az kabul edilir.

Örneğin `gen_range` yöntemi için `PartialOrd` özelliği gereklidir
tarafından belirtilen aralıkta rastgele bir değer üreten `rand` crate'inden
aralık ifadesi.

`Sıra` özelliği, açıklamalı değerin herhangi iki değeri için
türünde, geçerli bir sıralama mevcut olacaktır. `Sıra` özelliği `cmp` yöntemini uygular,
bir `Option<Ordering>` yerine bir `Ordering` döndürür, çünkü geçerli bir
sıralama her zaman mümkün olacaktır. Sadece `Ord` özelliğini tiplere uygulayabilirsiniz
ve `Eq` (ve `Eq` için `PartialEq` gerekir) uygulamalarını da gerçekleştirir. Ne zaman
structs ve enums üzerinde türetilmişse, `cmp` türetilmiş yapılarla aynı şekilde davranır
uygulamasının `Partial_cmp` için yaptığı gibi `PartialOrd` ile yapar.

Bir `BTreeSet<T>` içinde değerlerin depolanması `Ord`un gerekli olduğu durumlara bir örnektir,
Değerlerin sıralama düzenine göre veri depolayan bir veri yapısı.

### Değerleri Çoğaltmak için `Clone` ve `Copy`

`Clone` özelliği, bir değerin derin bir kopyasını açıkça oluşturmanıza olanak tanır ve
çoğaltma işlemi keyfi kod çalıştırmayı ve yığın kopyalamayı içerebilir
veri. Bakınız [Etkileşime Giren Değişkenler ve Veriler Clone"][variables-and-data-interacting-with-clone]<!-- ignore --> Bölüm 4'te
`Clone` hakkında daha fazla bilgi için.

`Clone` yönteminin türetilmesi, `clone` yöntemi için uygulandığında
tüm tür, türün her bir parçası üzerinde `clone` çağrısı yapar. Bu, tüm
türündeki alanlar veya değerler de `Clone` türetmek için `Clone` uygulamalıdır.

`Clone`un gerekli olduğu durumlara bir örnek olarak, `to_vec` metodunun bir
dilim. Dilim, içerdiği tip örneklerine sahip değildir, ancak vektör
'den döndürülen `to_vec` örneklerine sahip olması gerekecektir, bu nedenle `to_vec` çağrıları
her öğe üzerinde `clone`. Bu nedenle, dilimde saklanan tür `Clone` uygulamalıdır.

The `Copy` trait allows you to duplicate a value by only copying bits stored on
the stack; no arbitrary code is necessary. See [“Stack-Only Data: Copy”][stack-only-data-copy]<!-- ignore --> in Chapter 4 for more information on
`Copy`.

`Copy` özelliği, programcıların aşağıdakileri yapmasını önlemek için herhangi bir yöntem tanımlamaz
Bu yöntemlerin aşırı yüklenmesi ve keyfi kodların kullanılamayacağı varsayımının ihlal edilmesi
çalıştırılıyor. Bu şekilde, tüm programcılar bir değerin kopyalanmasının
çok hızlıdır.

Tüm parçaları `Copy` uygulayan herhangi bir tür üzerinde `Copy` türetebilirsiniz. Bir tür
'yi uygulayan bir tür `Copy`yi de uygulamalıdır, çünkü `Clone`u uygulayan bir tür
`Copy`, `Clone` ile aynı görevi yerine getiren önemsiz bir `Clone` uygulamasına sahiptir.
Copy`.

`Copy` özelliği nadiren gereklidir; `Copy` özelliğini uygulayan türler
optimizasyonları mevcuttur, yani `clone` çağırmak zorunda değilsiniz, bu da
kodu daha özlü hale getirir.

`Copy` ile mümkün olan her şeyi `Clone` ile de gerçekleştirebilirsiniz, ancak
kodu daha yavaş olabilir veya bazı yerlerde `clone` kullanmak zorunda kalabilir.

### Bir Değeri Sabit Boyutlu Bir Değere Eşlemek için `Hash`

`Hash` özelliği, isteğe bağlı boyutta bir türün örneğini almanıza ve
bir hash fonksiyonu kullanarak bu örneği sabit boyutlu bir değerle eşler. Türetme
`Hash`, `hash` yöntemini uygular. Hash`in türetilmiş uygulaması
yöntemi, türün her bir parçası üzerinde `hash` çağrısının sonucunu birleştirir,
yani `Hash` türetmek için tüm alanlar veya değerler de `Hash` uygulamalıdır.

`Hash`in ne zaman gerekli olduğuna bir örnek, anahtarların bir `HashMap<KV>` içinde depolanmasıdır
Verileri verimli bir şekilde depolamak için.

### Varsayılan Değerler için `Default`

`Default` özelliği, bir tür için varsayılan bir değer oluşturmanıza olanak tanır. Türetme
`Default`, `default` fonksiyonunu uygular. Türetilmiş uygulama
`default` işlevi, türün her bir parçası üzerinde `default` işlevini çağırır,
yani türdeki tüm alanlar veya değerler de `Default` uygulamalıdır.
Default` türetir.

`Default::default` fonksiyonu genellikle struct ile birlikte kullanılır
güncelleme sözdizimi [“Struct Update Syntax ile Diğer Örneklerden Örnekler Oluşturma”][creating-instances-from-other-instances-with-struct-update-syntax]<!
görmezden gel --> Bölüm 5'te. Bir yapının birkaç alanını özelleştirebilir ve ardından
kullanarak geri kalan alanlar için varsayılan bir değer kullanın.
`..Default::default()`.

üzerinde `unwrap_or_default` yöntemini kullandığınızda `Default` özelliği gereklidir.
Örneğin `Option<T>` örnekleri. Eğer `Option<T>` `None` ise, yöntem
`unwrap_or_default`, tür için `Default::default` sonucunu döndürür
`T`, `Option<T>` içinde saklanır.

[creating-instances-from-other-instances-with-struct-update-syntax]: ch05-01-defining-structs.md#Struct-Update-ile-Diğer-Örneklerden-Örnek-Oluşturma
[stack-only-data-copy]: ch04-01-what-is-ownership.md#Yalnızca-Yığın-Veriler:-Kopyalama
[variables-and-data-interacting-with-clone]: ch04-01-what-is-ownership.md#Klon-ile-Etkileşime-Giren-Değişkenler-ve-Veriler
[makrolar]: ch20-05-macros.md#Makrolar
