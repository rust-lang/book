# Jenerik Tipler, Özellikler ve Yaşam Süreleri

Her programlama dili
kavramların çoğaltılmasıyla etkili bir şekilde başa çıkmak için araçlara sahiptir. Rust'ta böyle bir araç _generics_'tir:
somut tipler veya diğer özellikler için soyut stand-ins. Kod derlenirken ve çalıştırılırken
yerlerine neyin geleceğini bilmeden
jeneriklerin davranışlarını veya diğer jeneriklerle nasıl ilişki kurduklarını ifade edebiliriz.

Fonksiyonlar, `i32` veya `String` gibi somut bir
türü yerine bazı genel türlerden parametreler alabilir, aynı şekilde aynı kodu birden fazla somut değer üzerinde çalıştırmak için bilinmeyen
değerlerine sahip parametreler alabilirler. Aslında, 6. Bölümde `Option<T>` ile, 8. Bölümde `Vec<T>` ve
`HashMap<K, V>` ile ve 9. Bölümde `Result<T, E>` ile
jenerikleri zaten kullandık. Bu bölümde,
kendi türlerinizi, fonksiyonlarınızı ve metotlarınızı jeneriklerle nasıl tanımlayacağınızı keşfedeceksiniz!

Öncelikle kod tekrarını azaltmak için bir fonksiyonun nasıl çıkarılacağını gözden geçireceğiz. Daha sonra
aynı tekniği kullanarak
yalnızca parametrelerinin tipleri farklı olan iki fonksiyondan genel bir fonksiyon oluşturacağız. Ayrıca struct ve enum tanımlarında
jenerik tiplerinin nasıl kullanılacağını açıklayacağız.

Ardından, davranışı genel bir şekilde tanımlamak için _traits_'i nasıl kullanacağınızı öğreneceksiniz. Genel bir türü
herhangi bir tür yerine yalnızca belirli bir davranışa sahip türleri kabul edecek şekilde kısıtlamak için
özellikleri genel türlerle birleştirebilirsiniz.

Son olarak, _lifetimes_ konusunu ele alacağız:
derleyicisine referansların birbirleriyle nasıl ilişkili olduğu hakkında bilgi veren çeşitli jenerikler. Yaşam süreleri
derleyiciye ödünç alınan değerler hakkında yeterli bilgi vermemize olanak tanır; böylece
referansların
yardımımız olmadan yapabileceğinden daha fazla durumda geçerli olmasını sağlayabilir.

## Bir Fonksiyonu Ayıklayarak Çoğaltmayı Kaldırma

Jenerikler
kod tekrarını ortadan kaldırmak için belirli türleri birden fazla türü temsil eden bir yer tutucu ile değiştirmemize olanak tanır. Jenerik sözdizimine geçmeden önce,
belirli değerleri birden fazla değeri temsil eden bir
yer tutucuyla değiştiren bir fonksiyonu ayıklayarak
jenerik türleri içermeyen bir şekilde yinelemeyi nasıl kaldıracağımıza bakalım. Daha sonra aynı
tekniğini genel bir fonksiyonu ayıklamak için uygulayacağız! Bir fonksiyona çıkarabileceğiniz
çoğaltılmış kodu nasıl tanıyacağınıza bakarak, jenerikleri kullanabilen
çoğaltılmış kodu tanımaya başlayacaksınız.

Liste 10-1'deki bir listedeki en büyük
sayısını bulan kısa programla başlayacağız.

<Listing number="10-1" file-name="src/main.rs" caption="Finding the largest number in a list of numbers">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

</Listing>

Bir tam sayılar listesini `number_list` değişkeninde saklıyoruz ve listedeki ilk sayıya
referansını `largest` adlı bir değişkene yerleştiriyoruz. Daha sonra
listedeki tüm sayılar arasında yineleme yaparız ve geçerli sayı
`enbüyük` değişkeninde saklanan sayıdan büyükse, bu değişkendeki referansı değiştiririz.
Ancak mevcut sayı
şimdiye kadar görülen en büyük sayıdan küçük veya ona eşitse değişken değişmez ve kod
listedeki bir sonraki sayıya geçer. Listedeki tüm sayılar göz önünde bulundurulduktan sonra, `largest` değişkeni
adresinde en büyük sayıya atıfta bulunmalıdır; bu durumda bu sayı 100'dür.

Şimdi
sayılarından oluşan iki farklı listedeki en büyük sayıyı bulmakla görevlendirildik. Bunu yapmak için, Liste 10-1'deki kodu çoğaltmayı ve Liste 10-2'de gösterildiği gibi programın iki farklı yerinde
aynı mantığı kullanmayı seçebiliriz.

<Listing number="10-2" file-name="src/main.rs" caption="Code to find the largest number in *two* lists of numbers">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

</Listing>

Bu kod çalışsa da, kodu çoğaltmak sıkıcı ve hataya açıktır. Ayrıca
değiştirmek istediğimizde
kodu birden fazla yerde güncellemeyi hatırlamamız gerekir.

Bu yinelemeyi ortadan kaldırmak için, parametre olarak aktarılan herhangi bir tamsayı listesi üzerinde çalışan bir
işlevi tanımlayarak bir soyutlama oluşturacağız. Bu
çözümü kodumuzu daha anlaşılır hale getirir ve bir listedeki en büyük sayıyı
bulma kavramını soyut olarak ifade etmemizi sağlar.

Liste 10-3'te, en büyük sayıyı bulan kodu `largest` adlı bir
fonksiyonuna alıyoruz. Ardından, Liste 10-2'deki iki listede en büyük sayıyı
bulmak için fonksiyonu çağırıyoruz. Bu fonksiyonu gelecekte sahip olabileceğimiz `i32` değerlerinden oluşan başka bir
listesinde de kullanabiliriz.

<Listing number="10-3" file-name="src/main.rs" caption="Abstracted code to find the largest number in two lists">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

</Listing>

`En büyük` fonksiyonu, fonksiyona aktarabileceğimiz herhangi bir
somut `i32` değer dilimini temsil eden `list` adında bir parametreye sahiptir. Sonuç olarak,
fonksiyonu çağırdığımızda, kod
adresine ilettiğimiz belirli değerler üzerinde çalışır.

Özet olarak, kodu Liste 10-2'den
Liste 10-3'e değiştirmek için attığımız adımlar şunlardır:

1. Yinelenen kodu belirleyin.
1. Yinelenen kodu işlevin gövdesine alın ve işlev imzasında bu kodun
 girişlerini ve dönüş değerlerini belirtin.
1. Yinelenen kodun iki örneğini, bunun yerine işlevi çağıracak şekilde güncelleyin.

Daha sonra, kod tekrarını azaltmak için aynı adımları jeneriklerle kullanacağız. İşlev gövdesinin
belirli değerler yerine soyut bir `liste` üzerinde çalışabilmesi gibi
jenerikler de kodun soyut tipler üzerinde çalışmasına izin verir.

Örneğin, iki fonksiyonumuz olduğunu varsayalım: biri `i32` değerlerinden oluşan bir
dilimindeki en büyük öğeyi bulan ve diğeri `char`
değerlerinden oluşan bir dilimdeki en büyük öğeyi bulan. Bu yinelemeyi nasıl ortadan kaldırırız? Hadi öğrenelim!
