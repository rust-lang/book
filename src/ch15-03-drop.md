## `Drop` Trait'i ile Temizlikte Kod Çalıştırmak

Akıllı işaretçi deseninde önemli olan ikinci trait, bir değerin kapsamdan çıkmak üzere olduğunda ne olacağını özelleştirmenizi sağlayan `Drop` trait'idir. `Drop` trait'ini herhangi bir türde uygulayabilir ve bu kodu dosya veya ağ bağlantıları gibi kaynakları serbest bırakmak için kullanabilirsiniz.

`Drop`'u akıllı işaretçiler bağlamında tanıtıyoruz çünkü `Drop` trait'inin işlevselliği, akıllı işaretçi uygulanırken neredeyse her zaman kullanılır. Örneğin, bir `Box<T>` bırakıldığında, kutunun işaret ettiği heap alanı serbest bırakılır.

Bazı dillerde, bazı türler için, programcının bu türlerin bir örneğini kullanmayı bitirdiğinde belleği veya kaynakları serbest bırakacak kodu çağırması gerekir. Örneklere dosya tanıtıcıları, socket'ler ve kilitler dahildir. Eğer unutulursa, sistem aşırı yüklenip çökebilir. Rust'ta, bir değer kapsamdan çıktığında belirli bir kodun çalıştırılmasını belirtebilirsiniz ve derleyici bu kodu otomatik olarak ekler. Sonuç olarak, belirli bir türün örneğiyle işiniz bittiğinde programın her yerine temizlik kodu yerleştirme konusunda dikkatli olmanıza gerek yoktur—yine de kaynak sızıntısı yaşamazsınız!

Bir değer kapsamdan çıktığında çalışacak kodu, `Drop` trait'ini uygulayarak belirtirsiniz. `Drop` trait'i, `self`'e değiştirilebilir bir referans alan `drop` adlı bir metot uygulamanızı gerektirir. Rust'ın `drop`'u ne zaman çağırdığını görmek için, şimdilik `println!` ifadeleriyle bir `drop` uygulayalım.

15-14 numaralı listede, tek özel işlevi kapsamdan çıktığında `Dropping CustomSmartPointer!` yazdırmak olan bir `CustomSmartPointer` yapısı gösteriliyor. Bu, Rust'ın `drop` metodunu ne zaman çalıştırdığını göstermek için kullanılıyor.

<Listing number="15-14" file-name="src/main.rs" caption="Temizlik kodumuzu koyacağımız `Drop` trait'ini uygulayan bir `CustomSmartPointer` yapısı">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

</Listing>

`Drop` trait'i prelude'da yer aldığı için ayrıca içeri aktarmamıza gerek yok. `CustomSmartPointer` üzerinde `Drop` trait'ini uyguluyoruz ve `drop` metoduna `println!` çağrısı ekliyoruz. `drop` metodunun gövdesi, türünüzün bir örneği kapsamdan çıktığında çalışmasını istediğiniz mantığı koyacağınız yerdir. Burada, Rust'ın `drop`'u ne zaman çağırdığını görsel olarak göstermek için metin yazdırıyoruz.

`main`'de iki `CustomSmartPointer` örneği oluşturup ardından `CustomSmartPointers created` yazdırıyoruz. `main` sonunda, `CustomSmartPointer` örneklerimiz kapsamdan çıkacak ve Rust, `drop` metoduna koyduğumuz kodu çağıracak, son mesajımızı yazdıracaktır. Dikkat edin, `drop` metodunu açıkça çağırmamıza gerek yok.

Bu programı çalıştırdığımızda şu çıktıyı görürüz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

Rust, örneklerimiz kapsamdan çıktığında otomatik olarak `drop` çağırdı ve belirttiğimiz kodu çalıştırdı. Değişkenler, oluşturulduklarının ters sırasıyla bırakılır, bu yüzden önce `d`, sonra `c` bırakıldı. Bu örneğin amacı, `drop` metodunun nasıl çalıştığını görsel olarak göstermek; genellikle türünüzün çalıştırması gereken temizlik kodunu yazarsınız, bir yazı yazdırmazsınız.

<!-- Eski bağlantı, kaldırmayın -->

<a id="dropping-a-value-early-with-std-mem-drop"></a>

Ne yazık ki, otomatik `drop` işlevselliğini devre dışı bırakmak kolay değildir. `drop`'u devre dışı bırakmak genellikle gerekli değildir; `Drop` trait'inin amacı zaten bunun otomatik olarak halledilmesidir. Ancak bazen bir değeri erken temizlemek isteyebilirsiniz. Örneğin, kilitleri yöneten akıllı işaretçiler kullanırken, kilidi serbest bırakan `drop` metodunu zorla çalıştırmak isteyebilirsiniz ki aynı kapsamda başka kodlar kilidi alabilsin. Rust, `Drop` trait'inin `drop` metodunu elle çağırmanıza izin vermez; bunun yerine, bir değeri kapsamı bitmeden bırakmak istiyorsanız standart kütüphanedeki `std::mem::drop` fonksiyonunu çağırmalısınız.

15-14 numaralı listedeki `main` fonksiyonunu değiştirip, `Drop` trait'inin `drop` metodunu elle çağırmaya çalışırsak, 15-15 numaralı listede gösterildiği gibi bir derleyici hatası alırız.

<Listing number="15-15" file-name="src/main.rs" caption="Temizliği erken yapmak için `Drop` trait'inin `drop` metodunu elle çağırmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

</Listing>

Bu kodu derlemeye çalıştığımızda şu hatayı alırız:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

Bu hata mesajı, `drop`'u açıkça çağırmamıza izin verilmediğini belirtir. Hata mesajında _destructor_ terimi kullanılır; bu, bir örneği temizleyen fonksiyon için genel programlama terimidir. _Destructor_, bir örnek oluşturan _constructor_ ile benzerdir. Rust'taki `drop` fonksiyonu, belirli bir destructor'dur.

Rust, `drop`'u açıkça çağırmamıza izin vermez çünkü Rust, `main` sonunda değeri yine otomatik olarak bırakacaktır. Bu, Rust'ın aynı değeri iki kez temizlemeye çalışmasına, yani _çifte serbest bırakma_ (double free) hatasına yol açar.

Bir değer kapsamdan çıktığında otomatik olarak `drop` eklenmesini devre dışı bırakamayız ve `drop` metodunu açıkça çağıramayız. Bu yüzden, bir değeri erken temizlememiz gerekirse, `std::mem::drop` fonksiyonunu kullanırız.

`std::mem::drop` fonksiyonu, `Drop` trait'indeki `drop` metodundan farklıdır. Zorla bırakmak istediğimiz değeri argüman olarak vererek çağırırız. Fonksiyon prelude'da yer aldığı için, 15-15 numaralı listedeki `main` fonksiyonunu, 15-16 numaralı listede gösterildiği gibi `drop` fonksiyonunu çağıracak şekilde değiştirebiliriz.

<Listing number="15-16" file-name="src/main.rs" caption="Bir değeri kapsamdan çıkmadan önce açıkça bırakmak için `std::mem::drop` çağırmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

</Listing>

Bu kodu çalıştırdığınızda şu çıktıyı görürsünüz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

``Dropping CustomSmartPointer with data `some data`!`` metni, `CustomSmartPointer created.` ve `CustomSmartPointer dropped before the end of main.` metinleri arasında yazdırılır; bu da `drop` metodunun kodunun, `c`'yi o noktada bırakmak için çağrıldığını gösterir.

`Drop` trait'inde belirttiğiniz kodu, temizlik işlemini kolay ve güvenli hale getirmek için birçok şekilde kullanabilirsiniz: örneğin, kendi bellek ayırıcınızı oluşturmak için! `Drop` trait'i ve Rust'ın sahiplik sistemi sayesinde, temizlik yapmayı hatırlamanıza gerek yok çünkü Rust bunu otomatik olarak yapar.

Ayrıca, hâlâ kullanılan değerleri yanlışlıkla temizlemekten kaynaklanan sorunlar hakkında da endişelenmenize gerek yok: referansların her zaman geçerli olmasını sağlayan sahiplik sistemi, `drop`'un yalnızca değer artık kullanılmadığında ve yalnızca bir kez çağrılmasını da garanti eder.

Artık `Box<T>`'yi ve akıllı işaretçilerin bazı özelliklerini incelediğimize göre, standart kütüphanede tanımlı birkaç başka akıllı işaretçiye bakalım.
