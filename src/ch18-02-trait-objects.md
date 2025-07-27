## Ortak Davranışları Soyutlamak için Trait Nesneleri Kullanmak

<!-- Eski başlıklar. Lütfen silmeyin, bağlantılar bozulabilir. -->

<a id="using-trait-objects-that-allow-for-values-of-different-types"></a>

8. bölümde, vektörlerin yalnızca tek bir türde öğe saklayabildiğinden bahsetmiştik. 8-9 numaralı listede, tamsayı, ondalık ve metin tutabilen varyantlara sahip bir `SpreadsheetCell` enum'u tanımlayarak bir çözüm üretmiştik. Böylece, her hücrede farklı türde veri saklayabiliyor ve yine de bir satırı temsil eden bir vektör oluşturabiliyorduk. Kod derlenirken hangi türlerin değiştirilebilir olacağını biliyorsak, bu gayet iyi bir çözümdür.

Ancak bazen, kütüphanemizi kullanan kişinin belirli bir durumda geçerli olan türler kümesini genişletebilmesini isteriz. Bunu nasıl başarabileceğimizi göstermek için, listedeki her öğe üzerinde `draw` metodunu çağırarak ekrana çizen örnek bir grafiksel kullanıcı arayüzü (GUI) aracı oluşturacağız—bu, GUI araçlarında yaygın bir tekniktir. `gui` adında bir kütüphane crate'i oluşturacağız ve bu crate, bir GUI kütüphanesinin yapısını içerecek. Bu crate, kullanıcıların kullanabileceği bazı türleri (ör. `Button` veya `TextField`) içerebilir. Ayrıca, `gui` kullanıcıları kendi çizilebilir türlerini de oluşturmak isteyeceklerdir: örneğin, bir programcı `Image`, bir diğeri ise `SelectBox` ekleyebilir.

Kütüphaneyi yazarken, diğer programcıların hangi türleri oluşturmak isteyeceğini bilemeyiz ve tanımlayamayız. Ama biliyoruz ki, `gui` birçok farklı türde değeri takip etmeli ve her birinin üzerinde `draw` metodunu çağırabilmeli. `draw` metodunu çağırdığımızda tam olarak ne olacağını bilmemize gerek yok; sadece, değerin bu metoda sahip olması yeterli.

Bunu kalıtım (inheritance) olan bir dilde yapmak isteseydik, `draw` adında bir metoda sahip `Component` adında bir sınıf tanımlardık. Diğer sınıflar (`Button`, `Image`, `SelectBox` gibi) `Component`'ten kalıtım alır ve böylece `draw` metodunu devralırdı. Her biri, kendi özel davranışını tanımlamak için `draw` metodunu geçersiz kılabilirdi (override); ancak framework, tüm bu türleri `Component` örneğiymiş gibi ele alabilir ve `draw` çağırabilirdi. Rust'ta kalıtım olmadığı için, `gui` kütüphanesini kullanıcıların kütüphaneyle uyumlu yeni türler oluşturmasına izin verecek şekilde farklı bir şekilde yapılandırmamız gerekir.

### Ortak Davranış için Trait Tanımlamak

`gui`'da olmasını istediğimiz davranışı uygulamak için, `Draw` adında ve içinde bir adet `draw` metodu olan bir trait tanımlayacağız. Sonra, trait nesnesi (trait object) alan bir vektör tanımlayabiliriz. Bir _trait nesnesi_, hem belirttiğimiz trait'i uygulayan bir türün örneğine hem de çalışma zamanında trait metodlarını aramak için kullanılan bir tabloya işaret eder. Bir trait nesnesi oluşturmak için bir tür pointer (ör. `&` referansı veya `Box<T>` akıllı işaretçisi), ardından `dyn` anahtar kelimesi ve ardından ilgili trait belirtilir. (Trait nesnelerinin neden pointer ile kullanılması gerektiğini 20. bölümde ["Dinamik Boyutlu Tipler ve `Sized` Trait"] [dynamically-sized]<!-- ignore --> kısmında ele alacağız.) Trait nesnelerini, generic veya somut bir tür yerine kullanabiliriz. Trait nesnesi kullandığımız her yerde, Rust'ın tip sistemi derleme zamanında o bağlamda kullanılan her değerin trait nesnesinin trait'ini uyguladığından emin olur. Sonuç olarak, tüm olası türleri derleme zamanında bilmemize gerek yoktur.

Rust'ta, struct ve enum'lara "nesne" demekten kaçındığımızı, diğer dillerdeki nesnelerden ayırmak için daha önce belirtmiştik. Bir struct veya enum'da, alanlardaki veri ve `impl` bloklarındaki davranış ayrıdır; diğer dillerde ise veri ve davranış tek bir kavramda (nesne) birleşir. Trait nesneleri, diğer dillerdeki nesnelerden farklıdır; çünkü trait nesnesine veri ekleyemeyiz. Trait nesneleri, diğer dillerdeki nesneler kadar genel amaçlı değildir: özel amaçları, ortak davranışlar arasında soyutlama sağlamaktır.

18-3 numaralı listede, `draw` adında bir metodu olan `Draw` trait'inin nasıl tanımlanacağı gösteriliyor.

<Listing number="18-3" file-name="src/lib.rs" caption="`Draw` trait'inin tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

</Listing>

Bu söz dizimi, 10. bölümde trait tanımlamayı tartışırken gördüklerimize benziyor. Sırada yeni bir söz dizimi var: 18-4 numaralı listede, `components` adında bir vektör tutan `Screen` adında bir struct tanımlanıyor. Bu vektörün türü:
