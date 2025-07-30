## Referans Döngüleri Bellek Sızıntısına Yol Açabilir


Rust'ın bellek güvenliği garantileri, yanlışlıkla asla temizlenmeyen (buna _bellek sızıntısı_ denir) bellek oluşturmayı zorlaştırır, ancak imkansız kılmaz. Bellek sızıntılarını tamamen önlemek Rust'ın garantilerinden biri değildir; yani, bellek sızıntıları Rust'ta bellek açısından güvenlidir. Rust'ın bellek sızıntılarına izin verdiğini `Rc<T>` ve `RefCell<T>` kullanarak görebiliriz: öğelerin birbirine döngüsel olarak referans verdiği durumlar oluşturmak mümkündür. Bu, her döngüdeki öğenin referans sayısı asla 0'a ulaşmayacağı ve değerler asla düşürülmeyeceği için bellek sızıntılarına yol açar.

### Referans Döngüsü Oluşturmak

Bir referans döngüsünün nasıl oluşabileceğine ve bunu nasıl önleyebileceğimize bakalım; önce `List` enum'unun ve bir `tail` metodunun tanımıyla başlayalım (Liste 15-25).

<Listing number="15-25" file-name="src/main.rs" caption="Bir `Cons` varyantının neyi işaret ettiğini değiştirebilmemiz için `RefCell<T>` tutan bir cons liste tanımı">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs}}
```

</Listing>

Burada, Liste 15-5'teki `List` tanımının başka bir varyasyonunu kullanıyoruz. `Cons` varyantındaki ikinci eleman artık `RefCell<Rc<List>>`, yani Liste 15-24'te yaptığımız gibi `i32` değerini değiştirmek yerine, bir `Cons` varyantının işaret ettiği `List` değerini değiştirmek istiyoruz. Ayrıca, bir `Cons` varyantına sahipsek ikinci öğeye kolayca erişebilmemiz için bir `tail` metodu ekliyoruz.

Liste 15-26'da, Liste 15-25'teki tanımları kullanan bir `main` fonksiyonu ekliyoruz. Bu kod, `a`'da bir liste ve `b`'de `a`'ya işaret eden bir liste oluşturuyor. Sonra, `a` listesini `b`'ye işaret edecek şekilde değiştirerek bir referans döngüsü oluşturuyoruz. Süreç boyunca referans sayılarını göstermek için `println!` ifadeleri var.

<Listing number="15-26" file-name="src/main.rs" caption="Birbirine işaret eden iki `List` değerinden oluşan bir referans döngüsü oluşturmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

</Listing>

`a` değişkeninde bir `Rc<List>` örneği oluşturup, başlangıçta `5, Nil` listesini tutuyoruz. Ardından, `b` değişkeninde başka bir `Rc<List>` örneği oluşturup, değeri `10` olan ve `a` listesini işaret eden bir liste elde ediyoruz.

`a`'yı, `Nil` yerine `b`'yi işaret edecek şekilde değiştirerek bir döngü oluşturuyoruz. Bunu, `tail` metodunu kullanarak `a`'daki `RefCell<Rc<List>>`'e bir referans alıp, bunu `link` değişkenine atayarak yapıyoruz. Sonra, `Ref

Bu kodu çalıştırdığımızda (şimdilik son `println!` satırı yorum satırı olarak bırakılmışken) şu çıktıyı alırız:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

Hem `a` hem de `b`'deki `Rc<List>` örneklerinin referans sayısı, `a` listesini `b`'ye işaret edecek şekilde değiştirdikten sonra 2 olur. `main` fonksiyonunun sonunda, Rust önce `b` değişkenini düşürür ve bu, `b`'deki `Rc<List>` örneğinin referans sayısını 2'den 1'e düşürür. Yığında tutulan `Rc<List>`'in belleği bu noktada düşürülmez, çünkü referans sayısı 1'dir, 0 değildir. Sonra Rust, `a`'yı düşürür ve bu da `a`'daki `Rc<List>` örneğinin referans sayısını 2'den 1'e düşürür. Bu örneğin belleği de düşürülemez, çünkü diğer `Rc<List>` örneği hâlâ ona referans vermektedir. Listeye ayrılan bellek sonsuza kadar toplanmadan kalacaktır. Bu referans döngüsünü görselleştirmek için Şekil 15-4'ü oluşturduk.

<img alt="'a' etiketli bir dikdörtgen, içinde 5 olan bir dikdörtgene işaret ediyor. 'b' etiketli bir dikdörtgen, içinde 10 olan bir dikdörtgene işaret ediyor. 5 içeren dikdörtgen 10 içeren dikdörtgene, 10 içeren dikdörtgen de tekrar 5 içeren dikdörtgene işaret ederek bir döngü oluşturuyor" src="img/trpl15-04.svg" class="center" />

<span class="caption">Şekil 15-4: Birbirine işaret eden `a` ve `b` listelerinden oluşan bir referans döngüsü</span>

Son `println!` satırının yorumunu kaldırıp programı çalıştırırsanız, Rust bu döngüyü, `a`'nın `b`'ye, `b`'nin tekrar `a`'ya işaret etmesiyle sonsuza kadar devam ettirir ve sonunda yığın taşmasına (stack overflow) yol açar.

Gerçek bir programla karşılaştırıldığında, bu örnekte referans döngüsü oluşturmanın sonuçları çok ciddi değildir: Döngüyü oluşturduktan hemen sonra program sona erer. Ancak, daha karmaşık bir programda döngüde çok fazla bellek ayrılır ve uzun süre tutulursa, program ihtiyacından fazla bellek kullanır ve sistemi aşırı yükleyerek mevcut belleğin tükenmesine neden olabilir.

Referans döngüleri oluşturmak kolay değildir, ancak imkânsız da değildir. Eğer `RefCell<T>` değerleriniz içinde `Rc<T>` değerleri veya benzer iç içe geçmiş, içsel değiştirilebilirlik ve referans sayımı kombinasyonları varsa, döngü oluşturmadığınızdan emin olmalısınız; Rust'ın bunu yakalamasına güvenemezsiniz. Referans döngüsü oluşturmak, programınızda bir mantık hatası olur ve bunu en aza indirmek için otomatik testler, kod incelemeleri ve diğer yazılım geliştirme uygulamalarını kullanmalısınız.

Referans döngülerini önlemenin bir başka yolu da veri yapılarınızı, bazı referansların sahiplik (ownership) ilişkisi, bazılarının ise sahiplik dışı ilişki ifade edecek şekilde yeniden düzenlemektir. Sonuç olarak, bazı sahiplik ilişkileri ve bazı sahiplik dışı ilişkilerden oluşan döngüleriniz olabilir ve yalnızca sahiplik ilişkileri bir değerin düşürülüp düşürülemeyeceğini etkiler. 15-25 numaralı listede, her zaman `Cons` varyantlarının listelerine sahip olmasını istiyoruz, bu nedenle veri yapısını yeniden düzenlemek mümkün değil. Şimdi, ebeveyn ve çocuk düğümlerden oluşan grafikler kullanarak, sahiplik dışı ilişkilerin referans döngülerini önlemede uygun bir yol olduğu bir örneğe bakalım.

<!-- Eski bağlantı, silmeyin -->

<a id="preventing-reference-cycles-turning-an-rct-into-a-weakt"></a>

### Referans Döngülerini `Weak<T>` Kullanarak Önlemek

Şimdiye kadar, `Rc::clone` çağrısının bir `Rc<T>` örneğinin `strong_count` (güçlü referans sayısı) değerini artırdığını ve bir `Rc<T>` örneğinin yalnızca `strong_count` değeri 0 ise temizlendiğini gösterdik. Ayrıca, bir `Rc<T>` örneğinin içindeki değere zayıf bir referans oluşturmak için `Rc::downgrade` fonksiyonunu çağırabilir ve bir `Rc<T>` referansı geçebilirsiniz. _Güçlü referanslar_, bir `Rc<T>` örneğinin sahipliğini paylaşmanın yoludur. _Zayıf referanslar_ ise sahiplik ilişkisi ifade etmez ve sayıları, bir `Rc<T>` örneğinin ne zaman temizleneceğini etkilemez. Zayıf referanslar, herhangi bir döngüde yer alsalar bile, ilgili değerlerin güçlü referans sayısı 0 olduğunda döngü kırılır ve sızıntı oluşmaz.

`Rc::downgrade` çağrıldığında, `Weak<T>` tipinde akıllı bir işaretçi elde edersiniz. `Rc<T>` örneğinde `strong_count` değerini 1 artırmak yerine, `Rc::downgrade` çağrısı `weak_count` değerini 1 artırır. `Rc<T>` tipi, kaç tane `Weak<T>` referansı olduğunu takip etmek için `weak_count` kullanır, tıpkı `strong_count` gibi. Farkı ise, `Rc<T>` örneğinin temizlenmesi için `weak_count` değerinin 0 olması gerekmez.

`Weak<T>`'nin işaret ettiği değerin düşürülmüş olabileceğinden, bir `Weak<T>`'nin işaret ettiği değerle bir şey yapmak için, değerin hâlâ var olduğundan emin olmalısınız. Bunu, bir `Weak<T>` örneği üzerinde `upgrade` metodunu çağırarak yapabilirsiniz; bu, size bir `Option<Rc<T>>` döndürür. Eğer `Rc<T>` değeri henüz düşürülmemişse `Some`, düşürülmüşse `None` sonucu alırsınız. `upgrade` metodu `Option<Rc<T>>` döndürdüğü için, Rust hem `Some` hem de `None` durumlarının ele alınmasını zorunlu kılar ve geçersiz bir işaretçi oluşmaz.

Örnek olarak, yalnızca bir sonraki öğeyi bilen bir liste yerine, hem çocuklarını hem de ebeveynini bilen düğümlerden oluşan bir ağaç oluşturacağız.

#### Bir Ağaç Veri Yapısı Oluşturmak: Çocuk Düğümleri Olan Bir `Node`

Başlangıç olarak, çocuk düğümlerini bilen düğümlerden oluşan bir ağaç inşa edeceğiz. Kendi `i32` değerini ve çocuk `Node` referanslarını tutan bir `Node` yapısı oluşturacağız:

<span class="filename">Dosya Adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

Çocuklarını sahiplenmesini ve bu sahipliği değişkenlerle paylaşabilmeyi istiyoruz, böylece ağaçtaki her `Node`'a doğrudan erişebiliriz. Bunu yapmak için, `Vec<T>` öğelerini `Rc<Node>` tipinde tanımlıyoruz. Ayrıca, hangi düğümlerin başka bir düğümün çocuğu olduğunu değiştirmek istediğimiz için, `children` alanında `Vec<Rc<Node>>`'ı saran bir `RefCell<T>` kullanıyoruz.

Şimdi, yapı tanımımızı kullanarak, değeri `3` olan ve çocuğu olmayan bir `leaf` düğümü ve değeri `5` olan, `leaf`'i çocuğu olarak tutan bir `branch` düğümü oluşturalım (15-27 numaralı listede gösterildiği gibi).

<Listing number="15-27" file-name="src/main.rs" caption="Çocuğu olmayan bir `leaf` düğümü ve `leaf`'i çocuğu olarak tutan bir `branch` düğümü oluşturmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

</Listing>

`leaf`'deki `Rc<Node>`'u klonlayıp `branch`'de saklıyoruz; bu, `leaf`'deki `Node`'un artık iki sahibi olduğu anlamına gelir: `leaf` ve `branch`. `branch.children` üzerinden `branch`'den `leaf`'e ulaşabiliriz, ancak `leaf`'ten `branch`'e ulaşmanın bir yolu yoktur. Çünkü `leaf`, `branch`'e referans vermez ve aralarındaki ilişkiyi bilmez. `leaf`'in, `branch`'in ebeveyni olduğunu bilmesini istiyoruz. Bunu şimdi yapacağız.

#### Bir Çocuğun Ebeveynine Referans Eklemek

Çocuk düğümün ebeveynini bilmesini sağlamak için, `Node` yapı tanımımıza bir `parent` alanı eklememiz gerekiyor. Sorun, `parent`'ın tipinin ne olması gerektiğine karar vermekte. Bunun bir `Rc<T>` olamayacağını biliyoruz, çünkü bu durumda `leaf.parent`'ın `branch`'i, `branch.children`'ın ise `leaf`'i işaret etmesiyle bir referans döngüsü oluşur ve bunların `strong_count` değerleri asla 0 olmaz.

İlişkileri başka bir şekilde düşünürsek, bir ebeveyn düğüm çocuklarının sahibi olmalıdır: Bir ebeveyn düğüm düşürülürse, çocuk düğümler de düşürülmelidir. Ancak, bir çocuk ebeveyninin sahibi olmamalıdır: Bir çocuk düğüm düşürülürse, ebeveyn hâlâ var olmalıdır. Bu, zayıf referanslar için uygun bir durumdur!

Bu nedenle, `Rc<T>` yerine, `parent` alanının tipini `Weak<T>` olarak belirleyeceğiz, özellikle de `RefCell<Weak<Node>>` olarak. Artık `Node` yapı tanımımız şöyle görünüyor:

<span class="filename">Dosya Adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

Bir düğüm, ebeveynine referans verebilecek ama ona sahip olmayacak. 15-28 numaralı listede, bu yeni tanımı kullanacak şekilde `main` fonksiyonunu güncelliyoruz, böylece `leaf` düğümü ebeveyni olan `branch`'e referans verebilecek.

<Listing number="15-28" file-name="src/main.rs" caption="Bir `leaf` düğümünün, ebeveyni olan `branch` düğümüne zayıf referansla işaret etmesi">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

</Listing>

`leaf` düğümünü oluşturmak, 15-27 numaralı listedekine benzer; tek farkı, `parent` alanı: `leaf` başta ebeveyni olmadan başlar, bu yüzden yeni, boş bir `Weak<Node>` referansı oluşturuyoruz.

Bu noktada, `leaf`'in ebeveynine `upgrade` metodu ile erişmeye çalışırsak, `None` değeri alırız. Bunu ilk `println!` ifadesinin çıktısında görebiliriz:

```text
leaf parent = None
```

`branch` düğümünü oluşturduğumuzda, onun da `parent` alanında yeni bir `Weak<Node>` referansı olur, çünkü `branch`'in ebeveyni yoktur. Yine de, `branch`'in çocukları arasında `leaf` vardır. `branch`deki `Node` örneğini elde ettikten sonra, `leaf`'i, ebeveynine (`branch`) zayıf bir referans verecek şekilde değiştirebiliriz. Bunun için, `leaf`'in `parent` alanındaki `RefCell<Weak<Node>>` üzerinde `borrow_mut` metodunu kullanırız ve ardından `branch`'deki `Rc<Node>`'dan `Rc::downgrade` fonksiyonunu çağırarak bir `Weak<Node>` referansı oluştururuz.

`leaf`'in ebeveynini tekrar yazdırdığımızda, bu sefer `Some` varyantı içinde `branch`'i görürüz: Artık `leaf` ebeveynine erişebiliyor! Ayrıca, `leaf`'i yazdırdığımızda, 15-26 numaralı listede olduğu gibi sonsuz döngüye girip yığın taşmasına neden olmadan, `Weak<Node>` referansları `(Weak)` olarak yazdırılır:

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

Sonsuz çıktı olmaması, bu kodun referans döngüsü oluşturmadığını gösterir. Ayrıca, `Rc::strong_count` ve `Rc::weak_count` fonksiyonlarını çağırarak elde ettiğimiz değerlerden de bunu anlayabiliriz.

#### `strong_count` ve `weak_count` Değerlerindeki Değişiklikleri Görselleştirmek

`Rc<Node>` örneklerinin `strong_count` ve `weak_count` değerlerinin nasıl değiştiğine bakalım. Bunu görmek için yeni bir iç scope (kapsam) oluşturup, `branch`'in oluşturulmasını bu scope'a taşıyoruz. Böylece, `branch` oluşturulup scope sona erdiğinde neler olduğunu görebiliriz. Değişiklikler 15-29 numaralı listede gösterilmiştir.

<Listing number="15-29" file-name="src/main.rs" caption="`branch`'i bir iç scope'ta oluşturmak ve güçlü/zayıf referans sayılarını incelemek">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

</Listing>

`leaf` oluşturulduktan sonra, onun `Rc<Node>`'unda güçlü referans sayısı 1, zayıf referans sayısı 0'dır. İç scope'ta, `branch` oluşturulup `leaf` ile ilişkilendirildiğinde, çıktıda `branch`'deki `Rc<Node>`'un güçlü referans sayısı 1, zayıf referans sayısı 1 olur (`leaf.parent`'ın `branch`'i `Weak<Node>` ile işaretlemesi nedeniyle). `leaf`'teki sayıları yazdırdığımızda, `branch.children`'da `leaf`'in `Rc<Node>`'u klonlandığı için güçlü referans sayısı 2, zayıf referans sayısı ise hâlâ 0 olur.

İç scope sona erdiğinde, `branch` scope dışına çıkar ve `Rc<Node>`'un güçlü referans sayısı 0'a düşer, böylece ilgili `Node` düşürülür. `leaf.parent`'ın zayıf referans sayısı 1 olmasının, `Node`'un düşürülüp düşürülmemesi üzerinde etkisi yoktur, bu yüzden bellek sızıntısı oluşmaz!

Scope sona erdikten sonra, `leaf`'in ebeveynine tekrar erişmeye çalışırsak yine `None` alırız. Programın sonunda, `leaf`'teki `Rc<Node>`'un güçlü referans sayısı 1, zayıf referans sayısı 0'dır; çünkü artık yalnızca `leaf` değişkeni bu `Rc<Node>`'a referans vermektedir.

Tüm bu referans sayısı yönetimi ve değer düşürme mantığı, `Rc<T>` ve `Weak<T>` ile bunların `Drop` trait'inin implementasyonlarında yerleşiktir. `Node` tanımında, çocuktan ebeveynе ilişkinin `Weak<T>` referansı olmasını belirterek, ebeveyn düğümlerin çocuklara ve çocukların ebeveynlere işaret etmesini sağlayabilir, referans döngüsü ve bellek sızıntısı oluşmasını önleyebilirsiniz.

## Özet

Bu bölümde, akıllı işaretçileri kullanarak Rust'ın varsayılan olarak sunduğu referanslardan farklı garantiler ve ödünleşimler elde etmeyi gördük. `Box<T>` tipi, bilinen boyutta olup yığında ayrılmış veriye işaret eder. `Rc<T>` tipi, yığındaki veriye yapılan referansların sayısını takip ederek, verinin birden fazla sahibi olmasını sağlar. `RefCell<T>` tipi ise, içsel değiştirilebilirliğiyle, değiştirilemez bir tipte iç değeri değiştirmemiz gerektiğinde kullanılabilir; ayrıca ödünç alma kurallarını derleme zamanında değil, çalışma zamanında uygular.

Ayrıca, akıllı işaretçilerin pek çok işlevselliğini sağlayan `Deref` ve `Drop` trait'lerini de ele aldık. Bellek sızıntısına yol açabilen referans döngülerini ve bunları `Weak<T>` kullanarak nasıl önleyebileceğimizi inceledik.

Eğer bu bölüm ilginizi çektiyse ve kendi akıllı işaretçilerinizi uygulamak istiyorsanız, daha fazla bilgi için ["The Rustonomicon"][nomicon] kaynağına göz atabilirsiniz.

Sıradaki bölümde, Rust'ta eşzamanlılık (concurrency) konusunu ele alacağız. Ayrıca birkaç yeni akıllı işaretçiyle de tanışacaksınız.

[nomicon]: ../nomicon/index.html
