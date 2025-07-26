<!-- Eski başlık. Kaldırmayın, bağlantılar bozulabilir. -->

<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## Kapanışlar: Ortamlarını Yakalayabilen İsimsiz Fonksiyonlar

Rust'taki kapanışlar, bir değişkende saklayabileceğiniz veya başka fonksiyonlara argüman olarak geçirebileceğiniz isimsiz fonksiyonlardır. Kapanışı bir yerde oluşturup, daha sonra farklı bir bağlamda çağırarak değerlendirebilirsiniz. Fonksiyonlardan farklı olarak, kapanışlar tanımlandıkları kapsamdan değerleri yakalayabilirler. Bu kapanış özelliklerinin kodun yeniden kullanılmasını ve davranışın özelleştirilmesini nasıl sağladığını göstereceğiz.

<!-- Eski başlıklar. Kaldırmayın, bağlantılar bozulabilir. -->

<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### Kapanışlarla Ortamı Yakalamak

Öncelikle, kapanışların tanımlandıkları ortamdan değerleri daha sonra kullanmak üzere nasıl yakalayabileceğimizi inceleyeceğiz. Senaryo şu: Zaman zaman, tişört şirketimiz promosyon olarak posta listemizden birine özel, sınırlı sayıda bir tişört hediye ediyor. Posta listesindeki kişiler profillerine isteğe bağlı olarak favori renklerini ekleyebiliyor. Eğer seçilen kişinin favori rengi ayarlanmışsa, o renkte tişört alıyor. Eğer favori renk belirtilmemişse, şirkette en çok bulunan renkten bir tişört veriliyor.

Bunu uygulamanın birçok yolu var. Bu örnekte, basitlik için yalnızca `Kırmızı` ve `Mavi` varyantlarına sahip bir `ShirtColor` enum'u kullanacağız. Şirketin stoğunu ise, içinde mevcut tişört renklerini tutan bir `Vec<ShirtColor>` içeren `shirts` alanına sahip bir `Inventory` yapısı ile temsil ediyoruz. `Inventory` üzerinde tanımlı `giveaway` metodu, hediye tişört kazanan kişinin isteğe bağlı renk tercihini alır ve kişiye verilecek tişört rengini döndürür. Bu kurulum, Liste 13-1'de gösterilmiştir.

<Listing number="13-1" file-name="src/main.rs" caption="Tişört şirketi hediye durumu">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

`main`'de tanımlanan `store`'da bu promosyon için dağıtılacak iki mavi ve bir kırmızı tişört kalmıştır. `giveaway` metodunu, kırmızı tişört tercihi olan bir kullanıcı ve tercihi olmayan bir kullanıcı için çağırıyoruz.

Bu kod birçok şekilde uygulanabilirdi, fakat burada kapanışlara odaklanmak için, öğrendiğiniz kavramlar dışında yalnızca `giveaway` metodunun gövdesinde bir kapanış kullandık. `giveaway` metodunda, kullanıcı tercihini `Option<ShirtColor>` türünde bir parametre olarak alıyoruz ve `user_preference` üzerinde `unwrap_or_else` metodunu çağırıyoruz. [`Option<T>` üzerindeki `unwrap_or_else` metodu][unwrap-or-else]<!-- ignore --> standart kütüphane tarafından tanımlanmıştır. Bir argüman alır: argüman almayan ve bir `T` (bu durumda `ShirtColor`) döndüren bir kapanış. Eğer `Option<T>` `Some` varyantıysa, `unwrap_or_else` içindeki değeri döndürür. Eğer `Option<T>` `None` ise, `unwrap_or_else` kapanışı çağırır ve kapanışın döndürdüğü değeri döndürür.

`unwrap_or_else`'e argüman olarak `|| self.most_stocked()` kapanış ifadesini veriyoruz. Bu, kendisi parametre almayan bir kapanıştır (eğer kapanışın parametreleri olsaydı, iki dikey çizgi arasına yazılırdı). Kapanışın gövdesi `self.most_stocked()` fonksiyonunu çağırır. Kapanışı burada tanımlıyoruz ve `unwrap_or_else`'in implementasyonu gerekirse kapanışı daha sonra çalıştıracak.

Bu kodu çalıştırdığınızda şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Buradaki ilginç bir nokta, mevcut `Inventory` örneği üzerinde `self.most_stocked()` çağıran bir kapanış geçirmiş olmamızdır. Standart kütüphanenin bizim tanımladığımız `Inventory` veya `ShirtColor` türleri ya da bu senaryoda kullanmak istediğimiz mantık hakkında hiçbir şey bilmesine gerek yoktu. Kapanış, `self` `Inventory` örneğine değiştirilemez bir referans yakalar ve belirttiğimiz kodu `unwrap_or_else` metoduna iletir. Fonksiyonlar ise ortamlarını bu şekilde yakalayamazlar.

### Kapanışlarda Tür Çıkarımı ve Açık Tür Bildirimi

Fonksiyonlar ve kapanışlar arasında başka farklar da vardır. Kapanışlar genellikle parametrelerin veya dönüş değerinin türünü, `fn` fonksiyonlarında olduğu gibi belirtmenizi gerektirmez. Fonksiyonlarda tür bildirimleri gereklidir çünkü türler, kullanıcılarınıza açıkça sunulan bir arayüzün parçasıdır. Bu arayüzü katı şekilde tanımlamak, herkesin bir fonksiyonun hangi türde değerler kullandığı ve döndürdüğü konusunda hemfikir olmasını sağlamak için önemlidir. Kapanışlar ise böyle açık bir arayüzde kullanılmaz: değişkenlerde saklanır ve isimlendirilmeden, kütüphanenizin kullanıcılarına sunulmadan kullanılır.

Kapanışlar genellikle kısa ve yalnızca dar bir bağlamda geçerlidir. Bu sınırlı bağlamlarda, derleyici parametrelerin ve dönüş değerinin türünü, çoğu değişkenin türünü çıkarabildiği gibi çıkarabilir (nadir durumlarda kapanış tür bildirimlerine de ihtiyaç duyulabilir).

Değişkenlerde olduğu gibi, açıklığı ve netliği artırmak için tür bildirimleri ekleyebiliriz, ancak bu, gereğinden fazla ayrıntılı olmamıza neden olur. Bir kapanış için tür bildirimleri eklemek, Liste 13-2'de gösterildiği gibi olur. Bu örnekte, kapanışı argüman olarak geçtiğimiz yerde tanımlamak yerine, bir değişkende saklıyoruz.

<Listing number="13-2" file-name="src/main.rs" caption="Kapanışta parametre ve dönüş türlerinin isteğe bağlı olarak belirtilmesi">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

Tür bildirimleri eklendiğinde, kapanışların sözdizimi fonksiyonlarınkine daha çok benzer. Burada, parametresine 1 ekleyen bir fonksiyon ve aynı davranışa sahip bir kapanış tanımlıyoruz. İlgili kısımları hizalamak için bazı boşluklar ekledik. Bu, kapanış sözdiziminin fonksiyon sözdizimine ne kadar benzediğini, ancak boru işaretleri ve isteğe bağlı sözdizimi miktarı dışında nasıl farklılaştığını gösterir:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

İlk satırda bir fonksiyon tanımı, ikinci satırda ise tam tür bildirimli bir kapanış tanımı var. Üçüncü satırda, kapanış tanımından tür bildirimlerini kaldırıyoruz. Dördüncü satırda ise, kapanış gövdesi yalnızca bir ifade içerdiği için süslü parantezleri kaldırıyoruz. Bunların hepsi geçerli tanımlardır ve çağrıldıklarında aynı davranışı gösterirler. `add_one_v3` ve `add_one_v4` satırlarında, kapanışların kullanılacağı yerden türlerin çıkarılması gerekir. Bu, `let v = Vec::new();` ifadesinin de tür çıkarımı için ya tür bildirimi ya da vektöre bir değer eklenmesini gerektirmesine benzer.

Kapanış tanımlarında, derleyici parametrelerin ve dönüş değerinin her biri için birer somut tür çıkarır. Örneğin, Liste 13-3'te yalnızca aldığı değeri döndüren kısa bir kapanış tanımı gösterilmiştir. Bu kapanış, örnek amacı dışında çok kullanışlı değildir. Tanımda tür bildirimi yoktur. Tür bildirimi olmadığı için, kapanışı herhangi bir türle çağırabiliriz; burada ilk olarak `String` ile çağırıyoruz. Sonra, `example_closure`'ı bir tamsayı ile çağırmaya çalışırsak hata alırız.

<Listing number="13-3" file-name="src/main.rs" caption="Türleri çıkarılan bir kapanışı iki farklı türle çağırmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

Derleyici bize şu hatayı verir:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

İlk kez `example_closure`'ı bir `String` değeriyle çağırdığımızda, derleyici `x`'in ve kapanışın dönüş değerinin türünü `String` olarak çıkarır. Bu türler, `example_closure` kapanışında kilitlenir ve aynı kapanışla farklı bir tür kullanmaya çalıştığımızda tür hatası alırız.

### Referansları Yakalamak veya Sahipliği Taşımak

Kapanışlar, ortamlarından değerleri üç şekilde yakalayabilir: değiştirilemez ödünç alma, değiştirilebilir ödünç alma ve sahipliği alma. Kapanış, gövdesinde yakalanan değerlerle ne yaptığına göre hangisini kullanacağına karar verir.

Liste 13-4'te, yalnızca değeri yazdırmak için değiştirilemez bir referansa ihtiyaç duyduğu için, `list` adlı vektöre değiştirilemez bir referans yakalayan bir kapanış tanımlıyoruz.

<Listing number="13-4" file-name="src/main.rs" caption="Değiştirilemez referans yakalayan bir kapanış tanımlamak ve çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

Bu örnek ayrıca, bir değişkenin bir kapanış tanımına bağlanabileceğini ve daha sonra kapanışı, değişken adı ve parantez kullanarak bir fonksiyon adıymış gibi çağırabileceğimizi gösterir.

Aynı anda birden fazla değiştirilemez referansa sahip olabileceğimiz için, `list` kapanış tanımından önce, tanımdan sonra ama kapanış çağrılmadan önce ve kapanış çağrıldıktan sonra da erişilebilir. Bu kod derlenir, çalışır ve şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Sonraki örnekte, Liste 13-5'te, kapanış gövdesini `list` vektörüne bir eleman ekleyecek şekilde değiştiriyoruz. Kapanış artık değiştirilebilir bir referans yakalıyor.

<Listing number="13-5" file-name="src/main.rs" caption="Değiştirilebilir referans yakalayan bir kapanış tanımlamak ve çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

Bu kod derlenir, çalışır ve şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Artık `borrows_mutably` kapanışının tanımı ile çağrılması arasında bir `println!` yok: `borrows_mutably` tanımlandığında, `list`'e değiştirilebilir bir referans yakalar. Kapanış çağrıldıktan sonra tekrar kullanılmadığı için değiştirilebilir ödünç alma sona erer. Kapanış tanımı ile çağrısı arasında, yazdırmak için değiştirilemez ödünç alma artık izinli değildir çünkü değiştirilebilir ödünç alma varken başka ödünç almalar yapılamaz. Oraya bir `println!` ekleyip alacağınız hata mesajını görebilirsiniz!

Kapanışın gövdesi sahipliği doğrudan gerektirmese bile, ortamındaki değerlerin sahipliğini kapanışa zorla taşımak isterseniz, parametre listesinin başına `move` anahtar kelimesini ekleyebilirsiniz.

Bu teknik, genellikle bir kapanışı yeni bir iş parçacığına geçirirken, verinin yeni iş parçacığı tarafından sahiplenilmesini sağlamak için kullanılır. İş parçacıklarını ve neden kullanmak isteyebileceğinizi 16. Bölümde ayrıntılı olarak ele alacağız, ancak şimdilik, bir kapanışın sahipliği almasını gerektiren bir iş parçacığı başlatmayı kısaca inceleyelim. Liste 13-6, Liste 13-4'ün vektörü ana iş parçacığı yerine yeni bir iş parçacığında yazdıracak şekilde değiştirilmiş halini gösterir.

<Listing number="13-6" file-name="src/main.rs" caption="İş parçacığı için kapanışın `list`'in sahipliğini almasını zorlamak için `move` kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

Yeni bir iş parçacığı başlatıyoruz ve iş parçacığına argüman olarak çalıştırması için bir kapanış veriyoruz. Kapanış gövdesi listeyi yazdırıyor. Liste 13-4'te, kapanış yalnızca yazdırmak için `list`'i değiştirilemez bir referansla yakalamıştı. Bu örnekte, kapanış gövdesi hala yalnızca değiştirilemez bir referansa ihtiyaç duysa da, kapanış tanımının başına `move` anahtar kelimesini ekleyerek `list`'in kapanışa taşınmasını belirtmemiz gerekiyor. Ana iş parçacığı, yeni iş parçacığı çağrılmadan önce daha fazla işlem yaparsa, yeni iş parçacığı ana iş parçacığından önce veya sonra bitebilir. Ana iş parçacığı `list`'in sahipliğini koruyup yeni iş parçacığından önce biterse ve `list`'i bırakırsa, iş parçacığındaki değiştirilemez referans geçersiz olurdu. Bu nedenle, derleyici, yeni iş parçacığına verilen kapanışa `list`'in taşınmasını zorunlu kılar. `move` anahtar kelimesini kaldırmayı veya kapanış tanımından sonra ana iş parçacığında `list`'i kullanmayı deneyin, hangi derleyici hatalarını alacağınızı göreceksiniz!

<!-- Eski başlıklar. Kaldırmayın, bağlantılar bozulabilir. -->

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Kapanışlardan Değerleri Taşımak ve `Fn` Özellikleri

Bir kapanış ortamından bir referans yakaladığında veya bir değerin sahipliğini aldığında (yani kapanışa _ne taşındığını_ etkilediğinde), kapanış gövdesindeki kod, kapanış daha sonra çalıştırıldığında referanslara veya değerlere ne olacağını tanımlar (yani kapanıştan _ne çıkarılacağını_ etkiler).

Bir kapanış gövdesi şunları yapabilir: yakalanan bir değeri kapanıştan dışarı taşıyabilir, yakalanan değeri değiştirebilir, değeri ne taşıyabilir ne de değiştirebilir veya ortamdan hiçbir şey yakalamayabilir.

Bir kapanışın ortamdan değerleri nasıl yakaladığı ve işlediği, kapanışın hangi trait'leri uyguladığını etkiler ve trait'ler, fonksiyonların ve yapıların hangi tür kapanışları kullanabileceğini belirtmesini sağlar. Kapanışlar, gövdelerinin değerlerle ne yaptığına bağlı olarak, bu üç `Fn` trait'inden birini, ikisini veya üçünü birden otomatik olarak uygular:

* `FnOnce`, yalnızca bir kez çağrılabilen kapanışlar için geçerlidir. Tüm kapanışlar en azından bu trait'i uygular çünkü tüm kapanışlar çağrılabilir. Gövdesinden yakalanan değerleri dışarı taşıyan bir kapanış yalnızca `FnOnce`'ı uygular ve diğer `Fn` trait'lerini uygulamaz çünkü yalnızca bir kez çağrılabilir.
* `FnMut`, gövdesinden yakalanan değerleri dışarı taşımayan, ancak yakalanan değerleri değiştirebilen kapanışlar için geçerlidir. Bu kapanışlar birden fazla kez çağrılabilir.
* `Fn`, gövdesinden yakalanan değerleri dışarı taşımayan ve değiştirmeyen, ayrıca ortamdan hiçbir şey yakalamayan kapanışlar için geçerlidir. Bu kapanışlar, ortamlarını değiştirmeden birden fazla kez çağrılabilir, bu da örneğin bir kapanışın aynı anda birden fazla kez çağrılması gereken durumlarda önemlidir.

`Option<T>` üzerinde kullandığımız `unwrap_or_else` metodunun tanımına bakalım:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Burada, `T`, `Option`'ın `Some` varyantındaki değerin türünü temsil eden genel türdür. Bu tür, aynı zamanda `unwrap_or_else` fonksiyonunun dönüş türüdür: örneğin, bir `Option<String>` üzerinde `unwrap_or_else` çağıran kod bir `String` alacaktır.

Sonra, `unwrap_or_else` fonksiyonunun ek bir genel tür parametresi `F` olduğunu görüyoruz. `F` türü, `unwrap_or_else` çağrılırken sağladığımız kapanışın türüdür.

Genel tür `F` üzerindeki trait sınırı `FnOnce() -> T`'dir, yani `F` bir kez çağrılabilmeli, argüman almamalı ve bir `T` döndürmelidir. Trait sınırında `FnOnce` kullanmak, `unwrap_or_else`'in kapanışı en fazla bir kez çağıracağını belirtir. `unwrap_or_else`'in gövdesinde, eğer `Option` `Some` ise, `f` çağrılmaz. Eğer `Option` `None` ise, `f` bir kez çağrılır. Tüm kapanışlar `FnOnce`'ı uyguladığı için, `unwrap_or_else` üç tür kapanışı da kabul eder ve olabildiğince esnektir.

> Not: Eğer yapmak istediğimiz şey ortamdan bir değer yakalamayı gerektirmiyorsa, bir fonksiyonun adını, bir kapanış yerine, `Fn` trait'lerinden birini uygulayan bir yerde kullanabiliriz. Örneğin, bir `Option<Vec<T>>` değeri üzerinde, değer `None` ise yeni, boş bir vektör almak için `unwrap_or_else(Vec::new)` çağırabiliriz. Derleyici, fonksiyon tanımı için uygun olan `Fn` trait'ini otomatik olarak uygular.

Şimdi, dilimlerde tanımlı olan ve standart kütüphanede bulunan `sort_by_key` metoduna bakalım; bu metodun neden `FnOnce` yerine `FnMut` trait sınırı kullandığını görelim. Kapanış, dilimdeki mevcut öğeye bir referans olarak bir argüman alır ve sıralanabilir bir türde bir değer döndürür. Bu fonksiyon, bir dilimi her bir öğenin belirli bir özelliğine göre sıralamak istediğinizde kullanışlıdır. Liste 13-7'de, bir dizi `Rectangle` örneğimiz var ve bunları `width` alanına göre küçükten büyüğe sıralamak için `sort_by_key` kullanıyoruz.

<Listing number="13-7" file-name="src/main.rs" caption="Dikdörtgenleri genişliğe göre sıralamak için `sort_by_key` kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

Bu kod şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

`sort_by_key`'in bir `FnMut` kapanış alacak şekilde tanımlanmasının nedeni, kapanışı dilimdeki her bir öğe için birden fazla kez çağırmasıdır. `|r| r.width` kapanışı ortamdan hiçbir şey yakalamaz, değiştirmez veya dışarı taşımaz, bu nedenle trait sınırı gereksinimlerini karşılar.

Buna karşılık, Liste 13-8'de yalnızca `FnOnce` trait'ini uygulayan bir kapanış örneği gösterilmiştir, çünkü ortamdan bir değeri dışarı taşır. Derleyici, bu kapanışı `sort_by_key` ile kullanmamıza izin vermez.

<Listing number="13-8" file-name="src/main.rs" caption="`sort_by_key` ile bir `FnOnce` kapanış kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

Bu, `sort_by_key` kapanışı sıralama sırasında kaç kez çağırdığını saymak için ortamdan bir `String` olan `value`'ı `sort_operations` vektörüne ekleyerek saymaya çalışan, yapay ve karmaşık bir örnektir (ve çalışmaz). Kapanış, ortamdan `value`'ı yakalar ve ardından sahipliğini `sort_operations` vektörüne aktararak dışarı taşır. Bu kapanış yalnızca bir kez çağrılabilir; ikinci kez çağrılmaya çalışılırsa, `value` ortamda artık bulunmayacağı için çalışmaz! Bu nedenle, bu kapanış yalnızca `FnOnce`'ı uygular. Bu kodu derlemeye çalıştığımızda, kapanışın ortamdan bir değeri dışarı taşıyamayacağına dair bir hata alırız çünkü kapanışın `FnMut` uygulaması gerekir:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

Hata, kapanış gövdesinde ortamdan bir değerin dışarı taşındığı satırı işaret eder. Bunu düzeltmek için, kapanış gövdesini ortamdan değer taşımayacak şekilde değiştirmemiz gerekir. Ortamda bir sayaç tutup, kapanış gövdesinde bu sayacı artırmak, kapanışın kaç kez çağrıldığını saymanın daha basit bir yoludur. Liste 13-9'daki kapanış, yalnızca sayaç olan `num_sort_operations`'a değiştirilebilir bir referans yakaladığı için `sort_by_key` ile çalışır ve birden fazla kez çağrılabilir:

<Listing number="13-9" file-name="src/main.rs" caption="`sort_by_key` ile bir `FnMut` kapanış kullanmak mümkündür">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

`Fn` trait'leri, kapanışları kullanan fonksiyonları veya türleri tanımlarken veya kullanırken önemlidir. Sonraki bölümde yineleyicileri tartışacağız. Birçok yineleyici metodu kapanış argümanları alır, bu nedenle bu kapanış ayrıntılarını aklınızda bulundurun!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
