## Yineleyiciler ile Bir Dizi Öğeyi İşlemek

Yineleyici (iterator) deseni, bir dizi öğe üzerinde sırayla bir işlem yapmanıza olanak tanır. Bir yineleyici, her bir öğe üzerinde yineleme yapma ve dizinin ne zaman bittiğini belirleme mantığından sorumludur. Yineleyicileri kullandığınızda, bu mantığı kendiniz tekrar tekrar yazmak zorunda kalmazsınız.

Rust'ta yineleyiciler _tembeldir_ (lazy), yani onları tüketen ve bitiren bir metot çağırmadığınız sürece hiçbir etkileri olmaz. Örneğin, 13-10 numaralı listede yer alan kod, `Vec<T>` üzerinde tanımlı `iter` metodunu çağırarak `v1` vektöründeki öğeler üzerinde bir yineleyici oluşturur. Bu kodun kendisi tek başına herhangi bir iş yapmaz.

<Listing number="13-10" file-name="src/main.rs" caption="Bir yineleyici oluşturmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

Yineleyici, `v1_iter` değişkeninde saklanır. Bir yineleyici oluşturduktan sonra, onu çeşitli şekillerde kullanabiliriz. 3-5 numaralı listede, bir dizi üzerinde `for` döngüsüyle yineleme yapıp her bir öğe üzerinde bir kod çalıştırmıştık. O zamanlar, bunun perde arkasında bir yineleyici oluşturup tükettiğini belirtmemiştik.

13-11 numaralı listede, yineleyicinin oluşturulması ile kullanılmasını `for` döngüsünde ayırıyoruz. `for` döngüsü `v1_iter` içindeki yineleyiciyi kullandığında, yineleyicideki her bir öğe döngünün bir yinelemesinde kullanılır ve her değeri yazdırır.

<Listing number="13-11" file-name="src/main.rs" caption="Bir `for` döngüsünde yineleyici kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

Standart kütüphanesinde yineleyiciler olmayan dillerde, muhtemelen aynı işlevselliği bir değişkeni 0'dan başlatıp, bu değişkeni vektördeki bir değeri almak için indeks olarak kullanıp, değişkeni vektördeki toplam öğe sayısına ulaşana kadar döngüde artırarak yazardınız.

Yineleyiciler, bu mantığın tamamını sizin yerinize halleder ve hata yapabileceğiniz tekrarlı kodları azaltır. Ayrıca, yineleyiciler aynı mantığı yalnızca vektörler gibi indekslenebilen veri yapılarıyla değil, birçok farklı diziyle kullanmanıza esneklik sağlar. Şimdi yineleyicilerin bunu nasıl başardığını inceleyelim.

### `Iterator` Özelliği ve `next` Metodu

Tüm yineleyiciler, standart kütüphanede tanımlı olan `Iterator` adlı bir özelliği (trait) uygular. Özelliğin tanımı şu şekildedir:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // varsayılan uygulamalı metotlar gizlendi
}
```

Bu tanımda yeni bir sözdizimi kullanıldığını görebilirsiniz: `type Item` ve `Self::Item`, bu özellik ile ilişkili bir _ilişkili tür_ (associated type) tanımlar. İlişkili türleri 20. Bölümde ayrıntılı olarak ele alacağız. Şimdilik bilmeniz gereken, bu kodun `Iterator` özelliğini uygulamanın bir `Item` türü de tanımlamanızı gerektirdiği ve bu `Item` türünün `next` metodunun dönüş türünde kullanıldığıdır. Yani, `Item` türü yineleyiciden dönecek olan türdür.

`Iterator` özelliği, uygulayıcıların yalnızca bir metot tanımlamasını gerektirir: `next` metodu. Bu metot, yineleyicinin bir öğesini `Some` ile sarmalanmış olarak döndürür ve yineleme bittiğinde `None` döndürür.

Yineleyicilerde `next` metodunu doğrudan çağırabiliriz; 13-12 numaralı listede, vektörden oluşturulan bir yineleyici üzerinde `next`'in tekrar tekrar çağrılmasıyla hangi değerlerin döndüğünü görebilirsiniz.

<Listing number="13-12" file-name="src/lib.rs" caption="Bir yineleyicide `next` metodunu çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

Burada, `v1_iter`'ı değiştirilebilir (mutable) yapmamız gerektiğine dikkat edin: `next` metodunu çağırmak, yineleyicinin dizide nerede olduğunu takip etmek için kullandığı iç durumu değiştirir. Yani, bu kod yineleyiciyi _tüketir_ veya kullanır. `next`'in her çağrısı, yineleyiciden bir öğe alır. `for` döngüsü kullandığımızda `v1_iter`'ı değiştirilebilir yapmamıza gerek yoktu çünkü döngü, `v1_iter`'ın sahipliğini alır ve arka planda onu değiştirilebilir yapar.

Ayrıca, `next`'ten aldığımız değerlerin vektördeki değerlere değiştirilemez referanslar olduğuna dikkat edin. `iter` metodu, değiştirilemez referanslar üzerinde yineleyen bir yineleyici üretir. Eğer v1'in sahipliğini alıp sahip olunan değerler döndüren bir yineleyici oluşturmak istersek, `iter` yerine `into_iter` çağırabiliriz. Benzer şekilde, değiştirilebilir referanslar üzerinde yineleme yapmak istersek, `iter_mut` çağırabiliriz.

### Yineleyiciyi Tüketen Metotlar

`Iterator` özelliği, standart kütüphane tarafından sağlanan varsayılan uygulamalara sahip birçok farklı metot içerir; bu metotlar hakkında daha fazla bilgi almak için `Iterator` özelliğinin standart kütüphane API dokümantasyonuna bakabilirsiniz. Bu metotların bazıları, tanımlarında `next` metodunu çağırır; bu nedenle, `Iterator` özelliğini uygularken `next` metodunu tanımlamanız gerekir.

`next` metodunu çağıran metotlara _tüketici adaptörler_ (consuming adapters) denir, çünkü bunları çağırmak yineleyiciyi tüketir. Bir örnek, yineleyicinin sahipliğini alıp, `next`'i tekrar tekrar çağırarak tüm öğeleri toplayan ve yineleme tamamlandığında toplamı döndüren `sum` metodudur. 13-13 numaralı listede, `sum` metodunun kullanımını gösteren bir test var.

<Listing number="13-13" file-name="src/lib.rs" caption="Yineleyicideki tüm öğelerin toplamını almak için `sum` metodunu çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

`sum`'ı çağırdıktan sonra `v1_iter`'ı tekrar kullanmamıza izin verilmez çünkü `sum`, çağrıldığı yineleyicinin sahipliğini alır.

### Başka Yineleyiciler Üreten Metotlar

_Yineleyici adaptörleri_ (iterator adapters), `Iterator` özelliğinde tanımlı, yineleyiciyi tüketmeyen metotlardır. Bunun yerine, orijinal yineleyicinin bazı yönlerini değiştirerek farklı yineleyiciler üretirler.

13-14 numaralı listede, her öğe üzerinde bir kapanış çağıran `map` adlı yineleyici adaptör metodunun çağrılmasına bir örnek görebilirsiniz. `map` metodu, değiştirilmiş öğeleri üreten yeni bir yineleyici döndürür. Buradaki kapanış, vektördeki her öğeyi 1 artıran yeni bir yineleyici oluşturur.

<Listing number="13-14" file-name="src/main.rs" caption="Yeni bir yineleyici oluşturmak için yineleyici adaptörü `map`'i çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

Ancak, bu kod bir uyarı üretir:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

13-14 numaralı listedeki kod hiçbir şey yapmaz; belirttiğimiz kapanış asla çağrılmaz. Uyarı bize nedenini hatırlatır: yineleyici adaptörleri tembeldir ve burada yineleyiciyi tüketmemiz gerekir.

Bu uyarıyı düzeltmek ve yineleyiciyi tüketmek için, 12-1 numaralı listede `env::args` ile kullandığımız `collect` metodunu kullanacağız. Bu metot, yineleyiciyi tüketir ve elde edilen değerleri bir koleksiyon veri tipinde toplar.

13-15 numaralı listede, `map`'ten dönen yineleyici üzerinde yineleme yaparak elde edilen sonuçları bir vektörde toplıyoruz. Bu vektör, orijinal vektördeki her öğenin 1 artırılmış halini içerecek.

<Listing number="13-15" file-name="src/main.rs" caption="Yeni bir yineleyici oluşturmak için `map` metodunu, ardından yeni yineleyiciyi tüketip bir vektör oluşturmak için `collect` metodunu çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

`map` bir kapanış aldığı için, her öğe üzerinde istediğimiz işlemi belirtebiliriz. Bu, kapanışların bazı davranışları özelleştirmenize olanak tanırken, yineleyici davranışını tekrar kullanmanızı sağlayan güzel bir örnektir.

Birden fazla yineleyici adaptörünü zincirleyerek karmaşık işlemleri okunabilir bir şekilde gerçekleştirebilirsiniz. Ancak, tüm yineleyiciler tembel olduğundan, yineleyici adaptörlerinden sonuç almak için bir tüketici adaptör metodu çağırmanız gerekir.

### Ortamlarını Yakalayan Kapanışlar Kullanmak

Birçok yineleyici adaptörü, argüman olarak kapanışlar alır ve genellikle yineleyici adaptörlerine argüman olarak belirteceğimiz kapanışlar, ortamlarını yakalayan kapanışlar olur.

Bu örnek için, bir kapanış alan `filter` metodunu kullanacağız. Kapanış, yineleyiciden bir öğe alır ve bir `bool` döndürür. Kapanış `true` döndürürse, değer `filter` tarafından üretilen yinelemede yer alır. Kapanış `false` döndürürse, değer dahil edilmez.

13-16 numaralı listede, ortamından `shoe_size` değişkenini yakalayan bir kapanış ile `filter` kullanıyoruz ve bir `Shoe` yapı örnekleri koleksiyonu üzerinde yineleme yapıyoruz. Yalnızca belirtilen numaradaki ayakkabıları döndürecek.

<Listing number="13-16" file-name="src/lib.rs" caption="`shoe_size`'ı yakalayan bir kapanış ile `filter` metodunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

`shoes_in_size` fonksiyonu, parametre olarak bir ayakkabı vektörünün sahipliğini ve bir ayakkabı numarasını alır. Belirtilen numaradaki ayakkabıların iç
