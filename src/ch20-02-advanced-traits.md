## İleri Düzey Trait'ler

Trait'leri ilk olarak ["Trait'ler: Paylaşılan Davranışları Tanımlama"][traits-defining-shared-behavior]<!-- ignore --> başlıklı 10. Bölüm'de ele almıştık, ancak daha gelişmiş detaylara değinmemiştik. Artık Rust hakkında daha fazla şey bildiğinize göre, işin inceliklerine girebiliriz.

<!-- Eski bağlantı, lütfen silmeyin -->

<a id="specifying-placeholder-types-in-trait-definitions-with-associated-types"></a>

### İlişkili Tipler

_İlişkili tipler_ (associated types), bir trait ile bir tip yer tutucusunu birbirine bağlar; böylece trait metod tanımlarında bu yer tutucu tipler imzalarda kullanılabilir. Bir trait'i uygulayan kişi, belirli bir implementasyon için yer tutucu tip yerine kullanılacak somut tipi belirtir. Bu sayede, trait'i uygulayana kadar bu tiplerin tam olarak ne olduğunu bilmeden, bazı tipleri kullanan bir trait tanımlayabiliriz.

Bu bölümdeki gelişmiş özelliklerin çoğunu nadiren ihtiyaç duyulan şeyler olarak tanımladık. İlişkili tipler ise ortada bir yerde: Kitabın geri kalanında açıklanan özelliklerden daha az kullanılırlar, ancak bu bölümdeki diğer birçok özelliğe göre daha yaygındırlar.

İlişkili tipe sahip bir trait'e örnek olarak, standart kütüphanenin sağladığı `Iterator` trait'i verilebilir. İlişkili tipin adı `Item`'dır ve `Iterator` trait'ini uygulayan tipin üzerinde yineleme yaptığı değerlerin tipini temsil eder. `Iterator` trait'inin tanımı, Liste 20-13'te gösterilmiştir.

<Listing number="20-13" caption="İlişkili tipi `Item` olan `Iterator` trait'inin tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-13/src/lib.rs}}
```

</Listing>

`Item` tipi bir yer tutucudur ve `next` metodunun tanımı, bu metodun `Option<Self::Item>` tipinde değerler döndüreceğini gösterir. `Iterator` trait'ini uygulayanlar, `Item` için somut tipi belirtir ve `next` metodu, bu somut tipte bir değeri içeren bir `Option` döndürür.

İlişkili tipler, jeneriklere (generics) benzer bir kavram gibi görünebilir; çünkü jenerikler de bir fonksiyonu hangi tiplerle çalışabileceğini belirtmeden tanımlamamıza olanak tanır. Bu iki kavram arasındaki farkı incelemek için, `Item` tipinin `u32` olarak belirtildiği ve `Iterator` trait'inin `Counter` adlı bir tipe uygulandığı bir örneğe bakalım:

<Listing file-name="src/lib.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

</Listing>

Bu söz dizimi jeneriklerle olan söz dizimine benziyor. Peki neden `Iterator` trait'ini jeneriklerle, Liste 20-14'te gösterildiği gibi tanımlamıyoruz?

<Listing number="20-14" caption="Jeneriklerle tanımlanmış varsayımsal bir `Iterator` trait'i">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-14/src/lib.rs}}
```

</Listing>

Fark şu: Jenerikler kullanıldığında, Liste 20-14'te olduğu gibi, her implementasyonda tipleri belirtmemiz gerekir; çünkü ayrıca `Iterator<String> for Counter` veya başka herhangi bir tip için de implementasyon yapabiliriz, yani `Counter` için `Iterator` trait'inin birden fazla implementasyonunu yazabiliriz. Başka bir deyişle, bir trait jenerik parametreye sahipse, o trait bir tipe her seferinde jenerik tip parametrelerinin somut tiplerini değiştirerek birden fazla kez uygulanabilir. `Counter` üzerinde `next` metodunu kullandığımızda, hangi `Iterator` implementasyonunu kullanmak istediğimizi belirtmek için tip açıklamaları sağlamamız gerekir.

İlişkili tiplerle ise, tipleri belirtmemize gerek yoktur çünkü bir trait'i bir tipe birden fazla kez uygulayamayız. 20-13'teki tanımda ilişkili tipler kullanıldığında, `Item` tipinin ne olacağını yalnızca bir kez seçebiliriz; çünkü yalnızca bir `impl Iterator for Counter` olabilir. `next` metodunu `Counter` üzerinde çağırdığımız her yerde `u32` değerlerinde bir iterator istediğimizi belirtmemize gerek yoktur.

İlişkili tipler ayrıca, trait'in sözleşmesinin bir parçası haline gelir: trait'i uygulayanlar, ilişkili tip yer tutucusu için bir tip sağlamak zorundadır. İlişkili tiplerin adı genellikle tipin nasıl kullanılacağını tanımlar ve API belgelerinde ilişkili tipleri belgelemek iyi bir uygulamadır.

### Varsayılan Jenerik Tip Parametreleri ve Operatör Aşırı Yükleme

Jenerik tip parametreleri kullandığımızda, jenerik tip için varsayılan bir somut tip belirtebiliriz. Bu, trait'in uygulayıcılarının varsayılan tip işe yarıyorsa somut bir tip belirtme gereksinimini ortadan kaldırır. Bir jenerik tipi `<PlaceholderType=ConcreteType>` sözdizimiyle tanımlarken varsayılan bir tip belirlersiniz.

Bu tekniğin kullanışlı olduğu bir durum örneği, belirli durumlarda bir operatörün (örneğin `+`) davranışını özelleştirdiğiniz _operatör aşırı yüklemesi_ dir.

Rust, kendi operatörlerinizi oluşturmanıza veya rastgele operatörleri aşırı yüklemenize izin vermez. Ancak, `std::ops` ile listelenen işlemleri ve ilgili trait'leri, ilgili trait'leri uygulayarak aşırı yükleyebilirsiniz. Örneğin, Liste 20-15'te `+` operatörünü iki `Point` örneğini toplamak için aşırı yüklüyoruz. Bunu, `Add` trait'ini bir `Point` yapısı üzerinde uygulayarak yapıyoruz.

<Listing number="20-15" file-name="src/main.rs" caption="`Point` örnekleri için `+` operatörünü aşırı yüklemek için `Add` trait'ini uygulama">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-15/src/main.rs}}
```

</Listing>

`add` metodu, iki `Point` örneğinin `x` değerlerini ve `y` değerlerini toplayarak yeni bir `Point` oluşturur. `Add` trait'inin, `add` metodundan dönen tipin belirlenmesini sağlayan bir ilişkili tipi olan `Output` vardır.

Bu kodda varsayılan jenerik tip, `Add` trait'indedir. İşte tanımı:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Bu kod genel olarak tanıdık görünmelidir: bir metodu ve bir ilişkili tipi olan bir trait. Yeni kısım ise `Rhs=Self`: bu sözdizimi _varsayılan tip parametreleri_ olarak adlandırılır. `Rhs` jenerik tip parametresi (sağ taraf tipi için kısaltma), `add` metodundaki `rhs` parametresinin tipini tanımlar. Eğer `Add` trait'ini `Rhs` için somut bir tip belirtmeden uygularsak, `Rhs`'nin tipi varsayılan olarak `Self` olur; bu da `Add`yi uyguladığımız tip olacaktır.

`Add`yi `Point` için uyguladığımızda, `Rhs` için varsayılanı kullandık çünkü iki `Point` örneğini toplamak istedik. Şimdi, varsayılan `Rhs` tipi yerine `Add` trait'ini `Millimeters` üzerinde `Meters` ile uygulamak istediğimiz bir örneğe bakalım. Bu, `Add` trait'inin `Rhs` tip parametresinin değerini varsayılan `Self` yerine ayarlamak için `impl Add<Meters>` belirterek yapılır; bu, Liste 20-16'da gösterilmiştir.

<Listing number="20-16" file-name="src/lib.rs" caption="`Millimeters` üzerinde `Add` trait'ini uygulayarak `Millimeters` ile `Meters`'i toplama">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-16/src/lib.rs}}
```

</Listing>

`Millimeters` ve `Meters`'i toplamak için, varsayılan `Self` yerine `Rhs` tip parametresinin değerini `Meters` olarak belirtiyoruz.

İki ana şekilde varsayılan tip parametrelerini kullanacaksınız:

1. Mevcut kodu bozmadan bir tipi uzatmak için
2. Çoğu kullanıcının ihtiyaç duymayacağı belirli durumlarda özelleştirmeye izin vermek için

Standart kütüphanenin `Add` trait'i, ikinci amaç için bir örnektir: genellikle, benzer tipleri toplarsınız, ancak `Add` trait'i bunun ötesinde özelleştirme olanağı sağlar. `Add` trait'inin tanımında varsayılan bir tip parametresi kullanmak, ek parametreyi çoğu zaman belirtmenize gerek kalmaması anlamına gelir. Başka bir deyişle, varsayılan bir tip parametresi kullanmak, trait'in kullanımını kolaylaştırır.

İlk amaç, tersine benzer: mevcut bir trait'e bir tip parametresi eklemek istiyorsanız, varsayılan bir tip vererek, trait'in işlevselliğini uzatmanıza olanak tanır; bu, mevcut implementasyon kodunu bozmaz.

<!-- Eski bağlantı, lütfen silmeyin -->

<a id="fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name"></a>

### Aynı İsimli Metotlar Arasında Ayrım Yapma

Rust'ta, bir trait'in başka bir trait ile aynı isme sahip bir metoda sahip olmasını engelleyen hiçbir şey yoktur; ayrıca, her iki trait'i de bir tipe uygulamanızı engelleyen hiçbir şey yoktur. Bir tipe doğrudan, trait'lerden biriyle aynı isme sahip bir metot uygulamak da mümkündür.

Aynı isimli metodları çağırırken, Rust'a hangi metodun kullanılacağını belirtmek zorundasınız. Listing 20-17'de, her biri `fly` adlı bir metoda sahip iki trait tanımladık: `Pilot` ve `Wizard`. Ardından, doğrudan bir `fly` metoduna sahip `Human` tipi üzerinde her iki trait'i de uyguluyoruz. Her `fly` metodu farklı bir şey yapıyor.

<Listing number="20-17" file-name="src/main.rs" caption="İki trait tanımlanıyor, her biri `fly` metoduna sahip ve `Human` tipi üzerinde uygulanıyor, ayrıca `Human` üzerinde doğrudan aynı isimli bir `fly` metodu tanımlanıyor.">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-17/src/main.rs:here}}
```

</Listing>

`Human` örneğinin üzerinde `fly` çağırdığımızda, derleyici varsayılan olarak doğrudan tip üzerinde tanımlanan metodu çağırır, bu da Liste 20-18'de gösterilmiştir.

<Listing number="20-18" file-name="src/main.rs" caption="Bir `Human` örneği üzerinde `fly` çağırma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-18/src/main.rs:here}}
```

</Listing>

Bu kodu çalıştırmak `*kolları çılgınca sallıyor*` çıktısını verecektir ve bu, Rust'ın doğrudan `Human` üzerinde tanımlanan `fly` metodunu çağırdığını gösterir.

`Pilot` veya `Wizard` trait'lerinden birinin `fly` metodunu çağırmak için, hangi `fly` metodunu kastettiğimizi belirtmek için daha açık bir sözdizimi kullanmamız gerekir. Liste 20-19 bu sözdizimini göstermektedir.

<Listing number="20-19" file-name="src/main.rs" caption="Hangi trait'in `fly` metodunu çağırmak istediğimizi belirtme">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-19/src/main.rs:here}}
```

</Listing>

Metodun önüne trait adını yazarak, Rust'a hangi `fly` implementasyonunu çağırmak istediğimizi netleştiriyoruz. Ayrıca `Human::fly(&person)` yazabiliriz; bu, Liste 20-19'da kullandığımız `person.fly()` ile eşdeğerdir, ancak eğer ayrım yapmamıza gerek yoksa yazması biraz daha uzundur.

Bu kodu çalıştırmak aşağıdaki çıktıyı verecektir:

```console
{{#include ../listings/ch20-advanced-features/listing-20-19/output.txt}}
```

`fly` metodu bir `self` parametresi aldığından, eğer iki _tip_ da bir _trait_'i implement ediyorsa, Rust hangi trait implementasyonunu kullanacağını `self`'in tipine göre belirleyebilir.

Ancak, bir metod olmayan ilişkili fonksiyonlar `self` parametresine sahip değildir. Aynı isimle birden fazla tip veya trait olduğunda ve bunlar metod olmayan fonksiyonlar tanımlıyorsa, Rust hangi tipi kastettiğinizi her zaman bilemez; bu yüzden _tam nitelikli sözdizimi_ kullanmanız gerekir. Örneğin, Liste 20-20'de, tüm yavru köpekleri _Spot_ olarak adlandırmak isteyen bir hayvan barınağı için bir trait oluşturuyoruz. `baby_name` adlı ilişkili bir metod olmayan fonksiyon içeren bir `Animal` trait'i tanımlıyoruz. `Animal` trait'i `Dog` yapısı için uygulanıyor ve burada da doğrudan `baby_name` adlı bir ilişkili metod olmayan fonksiyon sağlıyoruz.

<Listing number="20-20" file-name="src/main.rs" caption="Bir fonksiyona sahip bir trait ve aynı isme sahip bir fonksiyona sahip bir tip üzerinde trait'i uygulama">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-20/src/main.rs}}
```

</Listing>

Yavru köpekleri Spot olarak adlandırma kodunu, `Dog` üzerinde doğrudan tanımlanan `baby_name` ilişkili fonksiyonunda uyguluyoruz. `Dog` tipi ayrıca `Animal` trait'ini uygular; bu trait, tüm hayvanların sahip olduğu özellikleri tanımlar. Yavru köpekler, köpek yavruları olarak adlandırılır ve bu, `Dog` üzerindeki `Animal` trait'inin implementasyonunda, `baby_name` fonksiyonunda ifade edilir.

`main` fonksiyonunda, `Dog::baby_name` fonksiyonunu çağırıyoruz; bu, doğrudan `Dog` üzerinde tanımlanan ilişkili fonksiyonu çağırır. Bu kod aşağıdaki çıktıyı verir:

```console
{{#include ../listings/ch20-advanced-features/listing-20-20/output.txt}}
```

Bu çıktı, istediğimiz şey değildir. `Dog` üzerinde uyguladığımız `Animal` trait'ine ait `baby_name` fonksiyonunu çağırmak istiyoruz, böylece kod `Bir yavru köpeğe Spot denir` çıktısını versin. Liste 20-19'da kullandığımız ayrım yapma tekniği burada işe yaramaz; eğer `main`i Liste 20-21'deki kod gibi değiştirirsek, derleme hatası alırız.

<Listing number="20-21" file-name="src/main.rs" caption="`Animal` trait'ine ait `baby_name` fonksiyonunu çağırmaya çalışma, ancak Rust hangi implementasyonu kullanacağını bilemiyor">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-21/src/main.rs:here}}
```

</Listing>

`Animal::baby_name` bir `self` parametresine sahip olmadığından ve `Animal` trait'ini uygulayan başka tipler olabileceğinden, Rust hangi `Animal::baby_name` implementasyonunu kullanmak istediğimizi bilemez. Bu derleyici hatasını alırız:

```console
{{#include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

`Animal` için `Dog` implementasyonunu kullanmak istediğimizi belirtmek ve başka bir tip için `Animal` implementasyonunu kullanmak istemediğimizi belirtmek için tam nitelikli sözdizimini kullanmamız gerekir. Liste 20-22, tam nitelikli sözdizimini nasıl kullanacağımızı göstermektedir.

<Listing number="20-22" file-name="src/main.rs" caption="`Animal` trait'ine ait `baby_name` fonksiyonunu `Dog` üzerinde çağırmak için tam nitelikli sözdizimini kullanma">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-22/src/main.rs:here}}
```

</Listing>

Köşeli parantez içindeki bir tip açıklaması veriyoruz; bu, `Dog` tipini bu fonksiyon çağrısı için bir `Animal` olarak ele almak istediğimizi belirtiyor. Bu kod artık istediğimiz çıktıyı verecektir:

```console
{{#include ../listings/ch20-advanced-features/listing-20-22/output.txt}}
```

Genel olarak, tam nitelikli sözdizimi şu şekilde tanımlanır:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

Metod olmayan ilişkili fonksiyonlar için, bir `receiver` olmayacaktır: yalnızca diğer argümanların listesi olacaktır. Fonksiyonları veya metodları çağırırken bu sözdizimini her yerde kullanabilirsiniz. Ancak, Rust'ın programdaki diğer bilgilere dayanarak hangi implementasyonun çağrılacağını belirleyebileceği durumlarda, bu sözdiziminin herhangi bir kısmını atlayabilirsiniz. Bu daha ayrıntılı sözdizimini yalnızca aynı isme sahip birden fazla implementasyon olduğunda ve Rust'ın hangi implementasyonu çağırmak istediğinizi belirlemesine yardımcı olması gerektiğinde kullanmanız gerekir.

<!-- Eski bağlantı, lütfen silmeyin -->

<a id="using-supertraits-to-require-one-traits-functionality-within-another-trait"></a>

### Süpertrait'ler Kullanma

Bazen, bir trait tanımının başka bir trait'e bağlı olduğu durumlar yazarsınız: ilk trait'i uygulamak için, o tipin ayrıca ikinci trait'i de uygulamasını istemek istersiniz. Bunu, trait tanımınızın ikinci trait'in ilişkili öğelerini kullanabilmesi için yaparsınız. Trait'inizinin bağımlı olduğu trait'e _süpertrait_ denir.

Örneğin, bir `OutlinePrint` trait'i oluşturmak istiyoruz; bu trait'in `outline_print` metodu, verilen bir değeri yıldızlarla çerçevelenmiş şekilde yazdıracaktır. Yani, bir `Point` yapısı düşünün; bu yapı, `Display` standart kütüphane trait'ini uyguluyor ve sonucu `(x, y)` şeklinde veriyor. `x` değeri 1 ve `y` değeri 3 olan bir `Point` örneği üzerinde `outline_print` çağırdığımızda, aşağıdaki çıktıyı vermelidir:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

`outline_print` metodunun implementasyonunda, `Display` trait'inin işlevselliğini kullanmak istiyoruz. Bu nedenle, `OutlinePrint` trait'inin yalnızca `Display`'i de uygulayan tipler için çalışacağını belirtmemiz gerekir ve bu, trait tanımında `OutlinePrint: Display` belirterek yapılır. Bu teknik, trait'e bir trait sınırı eklemeye benzer. Liste 20-23, `OutlinePrint` trait'inin bir implementasyonunu göstermektedir.

<Listing number="20-23" file-name="src/main.rs" caption="`Display`'den gelen işlevselliği gerektiren `OutlinePrint` trait'inin implementasyonu">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-23/src/main.rs:here}}
```

</Listing>

`OutlinePrint`'in `Display` trait'ini gerektirdiğini belirttiğimiz için, `Display`'i uygulayan herhangi bir tip için otomatik olarak uygulanmış olan `to_string` fonksiyonunu kullanabiliriz. Eğer trait adından sonra bir iki nokta üst üste ve `Display` trait'ini belirtmeden `to_string` kullanmaya çalışırsak, `to_string` metodunun mevcut olmadığına dair bir hata alırız.

`OutlinePrint`'i `Display`'i uygulamayan bir tipe, örneğin `Point` yapısına uygulamaya çalıştığımızda ne olduğunu görelim:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

</Listing>

`Display`'in gerekli olduğu ancak uygulanmadığına dair bir hata alırız:

```console
{{#include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

Bunu düzeltmek için, `Point` üzerinde `Display`'i uyguluyoruz ve böylece `OutlinePrint`'in gerektirdiği kısıtlamayı karşılıyoruz:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

</Listing>

Ardından, `Point` üzerinde `OutlinePrint` trait'ini implement etmek başarılı olacaktır ve bir `Point` örneğini yıldızlarla çerçevelenmiş şekilde görüntülemek için `outline_print` çağırabiliriz.

### Dış Trait'leri Dış Tipler Üzerinde Uygulamak için Yeni Tip Desenini Kullanma

10. Bölüm'de ["Bir Tip Üzerinde Trait Uygulama"][implementing-a-trait-on-a-type]<!-- ignore --> başlığında, bir trait'i bir tip üzerinde uygulamanın yalnızca trait'in veya tipin ya da her ikisinin de yerel olduğu durumlarda mümkün olduğunu belirten _yabancı kural_ (orphan rule) dan bahsetmiştik. Bu kısıtlamayı aşmanın bir yolu, _yeni tip deseni_ (newtype pattern) adı verilen bir tekniği kullanmaktır; bu, bir demet yapısında yeni bir tip oluşturmayı içerir. (Demet yapıları, 5. Bölüm'de ["Farklı Tipler Oluşturmak için İsimlendirilmemiş Alanlarla Demet Yapıları Kullanma"][tuple-structs]<!-- ignore --> başlığında ele alınmıştır.) Demet yapısı bir alan içerecek şekilde tanımlanır ve trait'ini uygulamak istediğimiz tipe ince bir sarıcı görevi görür. Böylece, sarıcı tip yerel hale gelir ve trait'i bu sarıcı üzerinde uygulayabiliriz. _Yeni tip_ terimi, Haskell programlama dilinden gelmektedir. Bu deseni kullanmanın çalışma zamanı performansında herhangi bir cezası yoktur ve sarıcı tip, derleme zamanında elenir.

Örneğin, `Display`'i `Vec<T>` üzerinde uygulamak istiyoruz; ancak yabancı kural, bunu doğrudan yapmamıza izin vermez çünkü `Display` trait'i ve `Vec<T>` tipi, dışarıda tanımlanmıştır. `Vec<T>`'yi tutan bir `Wrapper` yapısı oluşturabiliriz; ardından `Display`'i `Wrapper` üzerinde uygulayabiliriz ve `Vec<T>` değerini kullanabiliriz, bu da Liste 20-24'te gösterilmiştir.

<Listing number="20-24" file-name="src/main.rs" caption="`Vec<String>` etrafında bir `Wrapper` tipi oluşturarak `Display`'i uygulama">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-24/src/main.rs}}
```

</Listing>

`Display`'in implementasyonu, içteki `Vec<T>`'ye erişmek için `self.0` kullanır; çünkü `Wrapper` bir demet yapısıdır ve `Vec<T>` sırasıyla 0 indeksindeki öğedir. Böylece, `Wrapper` üzerinde `Display` trait'inin işlevselliğini kullanabiliriz.

Bu tekniği kullanmanın dezavantajı, `Wrapper`'ın yeni bir tip olmasıdır, bu yüzden tutmakta olduğu değerin yöntemlerine sahip değildir. `Wrapper`'ı, `Vec<T>` gibi davranabilmesi için, `Wrapper` üzerinde `Vec<T>`'nin tüm yöntemlerini doğrudan uygulamak zorunda kalırız; bu da yöntemlerin `self.0`'a yönlendirilmesini sağlar. Eğer yeni tipin, içteki tipin sahip olduğu her yönteme sahip olmasını istiyorsak, `Deref` trait'ini `Wrapper` üzerinde uygulamak, içteki tipe geri dönecek şekilde bir çözüm olacaktır (bunu, 15. Bölüm'de ["Akıllı İşaretçileri `Deref` Trait'i ile Normal Referanslar Gibi Kullanma"][smart-pointer-deref]<!-- ignore --> başlığında ele aldık). Eğer `Wrapper` tipinin, içteki tipin tüm yöntemlerine sahip olmasını istemiyorsak, örneğin `Wrapper` tipinin davranışını kısıtlamak için, yalnızca istediğimiz yöntemleri manuel olarak uygulamamız gerekir.

Bu yeni tip deseni, trait'lerle ilgili olmasa bile kullanışlıdır. Şimdi odaklanmamızı değiştirip Rust'ın tip sistemiyle etkileşimde bulunmanın bazı ileri düzey yollarına bakalım.

[newtype]: ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
[implementing-a-trait-on-a-type]: ch10-02-traits.html#implementing-a-trait-on-a-type
[traits-defining-shared-behavior]: ch10-02-traits.html#traits-defining-shared-behavior
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
