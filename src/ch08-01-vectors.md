## Vektörlerle Değer Listelerini Depolama

İlk olarak inceleyeceğimiz koleksiyon türü, _vektör_ olarak da bilinen `Vec<T>` türüdür.
Vektörler, tek bir veri yapısında birden fazla değeri depolamanıza olanak tanır ve
tüm değerleri bellekte yan yana yerleştirir. Vektörler yalnızca aynı türdeki değerleri
depolayabilir. Bir dosyanın satırları veya alışveriş sepetindeki ürünlerin fiyatları gibi
bir öğe listesi olduğunda kullanışlıdırlar.

### Yeni Bir Vektör Oluşturma

Yeni bir boş vektör oluşturmak için, Listing 8-1'de gösterildiği gibi `Vec::new` işlevini çağırırız.
Listing 8-1.

<Listing number="8-1" caption="Creating a new, empty vector to hold values of type `i32`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

</Listing>

Burada bir tür açıklaması eklediğimizi unutmayın. Bu vektöre herhangi bir
değer eklemediğimiz için Rust, hangi tür öğeleri depolamayı amaçladığımızı
bilmiyor. Bu önemli bir noktadır. Vektörler jeneriklerle uygulanır;
kendi türlerinizle jeneriklerin nasıl kullanılacağını 10. Bölümde ele alacağız. Şimdilik,
standart kütüphane tarafından sağlanan `Vec<T>` türünün herhangi bir türü barındırabileceğini bilin.
Belirli bir türü barındıracak bir vektör oluşturduğumuzda, türü köşeli parantezler içinde
belirtebiliriz. Listing 8-1'de, Rust'a `v` içindeki `Vec<T>`'nin
`i32` türündeki öğeleri barındıracağını söyledik.

Çoğu zaman, başlangıç değerleriyle bir `Vec<T>` oluşturacaksınız ve Rust,
saklamak istediğiniz değerin türünü çıkaracaktır, bu nedenle bu tür
açıklamayı nadiren yapmanız gerekecektir. Rust, verdiğiniz değerleri tutan yeni bir vektör oluşturan
`vec!` makrosunu kullanışlı bir şekilde sağlar. Listing 8-2, `1`, `2` ve `3` değerlerini tutan yeni bir
`Vec<i32>` oluşturur. Tamsayı türü `i32`'dir
çünkü bu, Bölüm 3'ün [“Veri Türleri”][data-types]<!-- ignore --> bölümünde tartıştığımız gibi varsayılan tamsayı türüdür.


<Listing number="8-2" caption="Creating a new vector containing values">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

</Listing>

Başlangıçta `i32` değerleri verdiğimiz için Rust, `v` türünün
`Vec<i32>` olduğunu çıkarabilir ve tür açıklaması gerekli değildir. Şimdi, bir vektörü nasıl
değiştireceğimize bakacağız.

### Vektörü Güncelleme

Bir vektör oluşturup ona öğeler eklemek için, Listing 8-3'te gösterildiği gibi `push` yöntemini kullanabiliriz.
Listing 8-3. Vektörü Güncelleme

<Listing number="8-3" caption="Using the `push` method to add values to a vector">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

</Listing>

Herhangi bir değişkenle olduğu gibi, değerini değiştirmek istiyorsak,
Bölüm 3'te tartışıldığı gibi `mut` anahtar kelimesini kullanarak onu değiştirilebilir hale getirmeliyiz. İçine koyduğumuz sayılar
hepsi `i32` türündedir ve Rust bunu veriden çıkarır, bu nedenle
`Vec<i32>` ek açıklamasına ihtiyacımız yoktur.

### Vektörlerin Elemanlarını Okuma

Bir vektörde depolanan bir değere başvurmanın iki yolu vardır: indeksleme veya
`get` yöntemini kullanma. Aşağıdaki örneklerde, daha fazla netlik sağlamak için bu
işlevlerden döndürülen değerlerin türlerini açıklama ekledik.

Listing 8-4, indeksleme sözdizimi ve `get` yöntemi ile bir vektördeki bir değere
erişmenin her iki yöntemini de göstermektedir.

<Listing number="8-4" caption="Using indexing syntax and using the `get` method to access an item in a vector">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

</Listing>

Burada birkaç ayrıntıya dikkat edin. Üçüncü öğeyi almak için `2` indeks değerini kullanıyoruz,
çünkü vektörler sıfırdan başlayarak sayılarla indekslenir. `&` ve `[]` kullanarak,
indeks değerindeki öğeye bir referans elde ediyoruz. `get` yöntemini,
argüman olarak geçirilen indeksle kullandığımızda, `match` ile kullanabileceğimiz
bir `Option<&T>` elde ediyoruz.

Rust, bir öğeye referans vermek için bu iki yolu sunar, böylece mevcut öğelerin aralığı dışında bir
indeks değeri kullanmaya çalıştığınızda programın nasıl davranacağını seçebilirsiniz.
Örnek olarak, beş öğeden oluşan bir vektörümüz olduğunda ve her iki teknikle de
indeks 100'deki bir öğeye erişmeye çalıştığımızda ne olacağını görelim,
Listing 8-5'te gösterildiği gibi.

<Listing number="8-5" caption="Attempting to access the element at index 100 in a vector containing five elements">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

</Listing>

Bu kodu çalıştırdığımızda, ilk `[]` yöntemi, var olmayan bir öğeye başvurduğu için programın çökmesine neden olur.
Bu yöntem, vektörün sonunu geçen bir öğeye erişilmeye çalışıldığında programın çökmesini istediğinizde en iyi şekilde kullanılır.
`get` yöntemi, vektörün dışındaki bir indeks geçirildiğinde, paniğe kapılmadan
`None` döndürür.

`get` yöntemine vektörün dışındaki bir indeks aktarıldığında, panik yapmadan
`None` değerini döndürür. Bu yöntemi, normal koşullar altında vektörün aralığı
dışındaki bir öğeye erişim
olasılığı varsa kullanabilirsiniz. Kodunuzda, Bölüm 6'da tartışıldığı gibi
`Some(&element)` veya `None` durumlarını ele alacak bir mantığa sahip olacaktır. Örneğin, indeks
bir kişinin girdiği bir sayıdan gelebilir. Eğer yanlışlıkla çok büyük bir sayı girerse ve program
`None` değerini alırsa, kullanıcıya mevcut vektörde kaç öğe olduğunu söyleyebilir ve
geçerli bir değer girmesi için ona bir şans daha verebilirsiniz. Bu, yazım hatası nedeniyle programı
çökertmekten daha kullanıcı dostu olacaktır!

Program geçerli bir referansa sahip olduğunda, ödünç alma denetleyicisi, bu referansın
ve vektörün içeriğine yapılan diğer tüm referansların geçerli kalmasını sağlamak için
sahiplik ve ödünç alma kurallarını (Bölüm 4'te ele alınmıştır) uygular. Aynı
kapsamda değiştirilebilir ve değiştirilemez referansların olamayacağı
kuralını hatırlayın. Bu kural, Listing 8-6'da geçerlidir. Burada, vektördeki ilk öğeye değiştirilemez bir referans tutuyoruz
ve sonuna bir öğe eklemeye çalışıyoruz. Bu
program, fonksiyonun ilerleyen kısımlarında da bu öğeye referans vermeye çalışırsak
çalışmayacaktır.

<Listing number="8-6" caption="Attempting to add an element to a vector while holding a reference to an item">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

</Listing>

Bu kodu derlemek şu hatayı verecektir:

```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

Listing 8-6'daki kod çalışması gerektiği gibi görünebilir: neden ilk öğeye yapılan bir referans
vektörün sonundaki değişiklikleri umursamalı? Bu hata,
vektörlerin çalışma şekliyle ilgilidir: vektörler değerleri bellekte yan yana
yerleştirdikleri için, vektörün sonuna yeni bir öğe eklemek,
yeterli alan yoksa yeni bellek ayırmayı ve eski öğeleri yeni alana kopyalamayı gerektirebilir.
yeterli alan yoksa, tüm öğeleri vektörün
şu anda depolandığı yerde yan yana yerleştirmek için. Bu durumda, ilk öğeye yapılan referans,
ayrılmış belleği işaret eder. Ödünç alma kuralları, programların
bu duruma düşmesini önler.

> Not: `Vec<T>` türünün uygulama ayrıntıları hakkında daha fazla bilgi için [“The
> Rustonomicon”][nomicon] bölümüne bakın.

### Vektördeki Değerleri İterasyon

Vektördeki her bir öğeye sırayla erişmek için, tek tek erişmek için indeksleri kullanmak yerine tüm öğeleri iterasyon yaparız.
Listing 8-7, bir vektördeki her bir öğeye değişmez referanslar elde etmek için `for` döngüsünün nasıl kullanıldığını gösterir.
Listing 8-7. Değişmez referanslar elde etmek için `for` döngüsünün kullanımı
`i32` values and print them.

<Listing number="8-7" caption="Printing each element in a vector by iterating over the elements using a `for` loop">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

</Listing>

Değiştirilebilir bir vektördeki her bir öğeye değiştirilebilir referanslar üzerinden yineleme yaparak
tüm öğelerde değişiklikler yapabiliriz. Listing 8-8'deki `for` döngüsü
her bir öğeye `50` ekleyecektir.

<Listing number="8-8" caption="Iterating over mutable references to elements in a vector">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

</Listing>

Değiştirilebilir referansın işaret ettiği değeri değiştirmek için, `+=`
işlemini kullanmadan önce `i` içindeki değere ulaşmak için
`*` dereference operatörünü kullanmamız gerekir. Dereference operatörü hakkında daha fazla bilgiyi, Bölüm 15'in [“Değere Referansı Takip Etme”][deref]<!-- ignore --> bölümünde bulabilirsiniz.
Bir vektör üzerinde, değiştirilebilir veya değiştirilemez olsun, yineleme yapmak

Bir vektör üzerinde, değişmez veya değişken olsun, yineleme yapmak, ödünç alma denetleyicisinin kuralları nedeniyle güvenlidir.
Listing 8-7 ve Listing 8-8'deki `for` Listing 8-7 ve Listing 8-8'deki `for`
döngü gövdelerine öğeler eklemeye veya çıkarmaya çalışırsak, Listing 8-6'daki kodda aldığımız
hataya benzer bir derleyici hatası alırız. `for` döngüsünün tuttuğu vektöre yapılan referans,
tüm vektörün aynı anda değiştirilmesini engeller.
Listing 8-7. Değişken bir vektör üzerinde yineleme

### Enum Kullanarak Birden Fazla Türü Depolama

Vektörler yalnızca aynı türdeki değerleri depolayabilir. Bu durum
rahatsız edici olabilir; farklı türdeki öğelerin bir listesini depolamak
gereken kullanım durumları kesinlikle vardır. Neyse ki, enum'un varyantları
aynı enum türü altında tanımlanır, bu nedenle farklı türdeki öğeleri temsil
etmek için bir tür gerektiğinde, bir enum tanımlayıp kullanabiliriz!

Örneğin, bir elektronik tablodaki bir satırdan değerler almak istediğimizi varsayalım.
Bu satırdaki bazı sütunlar tamsayılar, bazıları kayan noktalı sayılar ve
bazıları da dizeler içeriyor. Farklı değer türlerini tutacak varyantlara sahip bir enum tanımlayabiliriz
ve tüm enum varyantları aynı tür olarak kabul edilecektir: enum türü.
Ardından, bu enum'u tutacak bir vektör oluşturabiliriz ve böylece, nihayetinde,
farklı türleri tutabiliriz. Bunu Listing 8-9'da gösterdik.

<Listing number="8-9" caption="Defining an `enum` to store values of different types in one vector">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

</Listing>

Rust, derleme sırasında vektörde hangi türlerin olacağını bilmelidir, böylece
her bir öğeyi depolamak için yığın üzerinde tam olarak ne kadar bellek gerekeceğini bilir.
Bu vektörde hangi türlerin izin verildiğini de açıkça belirtmeliyiz. Rust,
bir vektörün herhangi bir türü barındırmasına izin verseydi, bir veya daha fazla
türün vektörün öğeleri üzerinde gerçekleştirilen işlemlerde hatalara neden olma
ihtimali olurdu. Bir enum ve bir `match` ifadesi kullanmak, Rust'un derleme sırasında
her olası durumun ele alınmasını sağlayacağı anlamına gelir, bu konu 6. Bölüm'de

Bir programın çalışma zamanında bir vektörde depolamak için alacağı türlerin
tümünü bilmiyorsanız, enum tekniği işe yaramayacaktır. Bunun yerine, 18. Bölüm'de
ele alacağımız trait nesnesini kullanabilirsiniz.

Vektörleri kullanmanın en yaygın yollarından bazılarını ele aldığımıza göre, standart kütüphanede `Vec<T>` üzerinde tanımlanan birçok
yararlı yöntemin tümünü içeren [API belgelerini][vec-api]<!-- ignore --> gözden geçirmeyi unutmayın. Örneğin,
`push` yöntemine ek olarak, `pop` yöntemi son öğeyi kaldırır ve döndürür.
Örnek 1.10.1. Vektör öğelerinin son öğesini döndürme

### Vektörün Düşürülmesi Elemanlarını Düşürür

Diğer tüm `struct`'lar gibi, vektör de kapsam dışına çıktığında serbest bırakılır,
Listing 8-10'da belirtildiği gibi.

<Listing number="8-10" caption="Showing where the vector and its elements are dropped">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

</Listing>

Vektör silindiğinde, içindeki tüm içerikler de silinir, yani
içerdiği tamsayılar temizlenir. Ödünç alma denetleyicisi, vektörün içeriğine yapılan tüm
bağlantıların yalnızca vektörün kendisi geçerli olduğu sürece
kullanılmasını sağlar.

Bir sonraki koleksiyon türüne geçelim: `String`!

[data-types]: ch03-02-data-types.md#veri-türleri
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.md
[deref]: ch15-02-deref.md#akıllı-i̇şaretçileri-deref-ile-normal-referanslar-gibi-kullanmak