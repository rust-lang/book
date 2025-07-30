## `RefCell<T>` ve İçsel Değiştirilebilirlik Deseni

_İçsel değiştirilebilirlik_ (interior mutability), Rust'ta, o veriye değiştirilemez referanslar varken bile veriyi değiştirebilmenizi sağlayan bir tasarım desenidir; normalde, ödünç alma kuralları bu işlemi yasaklar. Bu desende, veri yapısının içinde Rust'ın mutasyon ve ödünç alma kurallarını esnetmek için `unsafe` kod kullanılır. Unsafe kod, derleyiciye kuralları bizim elle kontrol ettiğimizi, derleyicinin bizim için kontrol etmemesine gerek olmadığını belirtir; unsafe kodu 20. Bölümde daha ayrıntılı tartışacağız.

İçsel değiştirilebilirlik desenini kullanan türleri yalnızca, ödünç alma kurallarının çalışma zamanında da olsa takip edileceğinden emin olabiliyorsak kullanabiliriz; derleyici bunu garanti edemez. İlgili `unsafe` kod, güvenli bir API ile sarılır ve dıştaki tür hâlâ değiştirilemez olur.

Bu kavramı, içsel değiştirilebilirlik desenini takip eden `RefCell<T>` türüne bakarak inceleyelim.

### `RefCell<T>` ile Ödünç Alma Kurallarını Çalışma Zamanında Uygulamak

`Rc<T>`'den farklı olarak, `RefCell<T>` türü tuttuğu veri üzerinde tekil sahipliği temsil eder. Peki, `RefCell<T>`'yi `Box<T>` gibi bir türden farklı kılan nedir? 4. Bölümde öğrendiğiniz ödünç alma kurallarını hatırlayın:

- Herhangi bir anda _ya_ bir değiştirilebilir referansınız _ya da_ istediğiniz kadar değiştirilemez referansınız olabilir (ama ikisi birden olamaz).
- Referanslar her zaman geçerli olmalıdır.

Referanslar ve `Box<T>` ile, ödünç alma kurallarının değişmezlikleri derleme zamanında uygulanır. `RefCell<T>` ile ise bu değişmezlikler _çalışma zamanında_ uygulanır. Referanslarla bu kuralları bozarsanız derleyici hatası alırsınız. `RefCell<T>` ile bu kuralları bozarsanız, programınız panic ile sonlanır.

Ödünç alma kurallarını derleme zamanında kontrol etmenin avantajı, hataların geliştirme sürecinde daha erken yakalanması ve çalışma zamanında hiçbir performans etkisi olmamasıdır; çünkü tüm analiz önceden tamamlanır. Bu nedenlerle, çoğu durumda ödünç alma kurallarını derleme zamanında kontrol etmek en iyi seçimdir ve Rust'ın varsayılanı budur.

Ödünç alma kurallarını çalışma zamanında kontrol etmenin avantajı ise, derleme zamanı kontrolleriyle izin verilmeyen bazı bellek güvenli senaryolarına izin verilmesidir. Rust derleyicisi gibi statik analizler doğası gereği temkinlidir. Kodun bazı özelliklerini analiz ederek tespit etmek imkânsızdır: En ünlü örnek, bu kitabın kapsamı dışında olan ancak araştırmaya değer bir konu olan Durdurma Problemi'dir (Halting Problem).

Bazı analizler imkânsız olduğundan, Rust derleyicisi kodun sahiplik kurallarına uyduğundan emin olamazsa, doğru bir programı reddedebilir; bu açıdan temkinlidir. Rust yanlış bir programı kabul etseydi, kullanıcılar Rust'ın sunduğu garantilere güvenemezdi. Ancak, Rust doğru bir programı reddederse, programcı için can sıkıcı olur ama felaket bir şey olmaz. `RefCell<T>` türü, kodunuzun ödünç alma kurallarına uyduğundan emin olduğunuz ama derleyicinin bunu anlayıp garanti edemediği durumlarda faydalıdır.

`Rc<T>` gibi, `RefCell<T>` de yalnızca tek iş parçacıklı senaryolarda kullanılmalıdır ve çok iş parçacıklı bir bağlamda kullanmaya çalışırsanız derleme zamanı hatası alırsınız. Çok iş parçacıklı bir programda `RefCell<T>` işlevselliğini nasıl elde edeceğimizi 16. Bölümde tartışacağız.

İşte `Box<T>`, `Rc<T>` veya `RefCell<T>` seçme nedenlerinin özeti:

- `Rc<T>`, aynı verinin birden fazla sahibine olanak tanır; `Box<T>` ve `RefCell<T>` tek sahibine sahiptir.
- `Box<T>`, derleme zamanında kontrol edilen değiştirilemez veya değiştirilebilir ödünç almaya izin verir; `Rc<T>`, yalnızca derleme zamanında kontrol edilen değiştirilemez ödünç almaya izin verir; `RefCell<T>`, çalışma zamanında kontrol edilen değiştirilemez veya değiştirilebilir ödünç almaya izin verir.
- `RefCell<T>`, çalışma zamanında kontrol edilen değiştirilebilir ödünç almaya izin verdiği için, `RefCell<T>` değiştirilemez olsa bile içindeki değeri değiştirebilirsiniz.

Değiştirilemez bir değerin içindeki değeri değiştirmek _içsel değiştirilebilirlik_ desenidir. Şimdi, içsel değiştirilebilirliğin faydalı olduğu bir duruma bakalım ve bunun nasıl mümkün olduğunu inceleyelim.

### İçsel Değiştirilebilirlik: Değiştirilemez Bir Değere Değiştirilebilir Ödünç Alma

Ödünç alma kurallarının bir sonucu olarak, değiştirilemez bir değere sahip olduğunuzda onu değiştirilebilir olarak ödünç alamazsınız. Örneğin, bu kod derlenmez:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

Bu kodu derlemeye çalışırsanız şu hatayı alırsınız:

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

Ancak, bazı durumlarda bir değerin kendi metotlarında kendini değiştirmesi, ama diğer kodlara değiştirilemez görünmesi faydalı olabilir. Değerin metotları dışındaki kod, değeri değiştiremez. `RefCell<T>` kullanmak, içsel değiştirilebilirlik elde etmenin bir yoludur, ancak `RefCell<T>` ödünç alma kurallarını tamamen aşmaz: derleyicideki ödünç alma denetleyicisi bu içsel değiştirilebilirliğe izin verir ve kurallar çalışma zamanında kontrol edilir. Kuralları ihlal ederseniz, derleyici hatası yerine `panic!` alırsınız.

Şimdi, `RefCell<T>` kullanarak değiştirilemez bir değeri nasıl değiştirebileceğimizi ve bunun neden faydalı olduğunu gösteren pratik bir örnek inceleyelim.

#### İçsel Değiştirilebilirlik için Bir Kullanım: Mock Nesneler

Bazen test sırasında bir programcı, belirli bir davranışı gözlemlemek ve doğru şekilde uygulandığını doğrulamak için başka bir tür yerine bir tür kullanır. Bu geçici tür _test double_ olarak adlandırılır. Bunu, film çekimlerinde bir aktör yerine tehlikeli sahnelerde oynayan dublör gibi düşünebilirsiniz. Test double'lar, testleri çalıştırırken diğer türlerin yerine geçer. _Mock nesneler_, bir test sırasında neler olduğunu kaydeden ve doğru eylemlerin gerçekleştiğini doğrulamanıza olanak tanıyan özel bir test double türüdür.

Rust, diğer dillerdeki gibi nesne kavramına sahip değildir ve Rust'ın standart kütüphanesinde mock nesne işlevselliği yoktur. Ancak, aynı amaçlara hizmet edecek bir struct kesinlikle oluşturabilirsiniz.

Senaryomuz şu: Bir değeri, bir maksimum değere karşı takip eden ve mevcut değer maksimuma ne kadar yaklaştıysa ona göre mesajlar gönderen bir kütüphane oluşturacağız. Bu kütüphane, örneğin bir kullanıcının yapmasına izin verilen API çağrısı sayısını takip etmek için kullanılabilir.

Kütüphanemiz yalnızca, bir değerin maksimuma ne kadar yaklaştığını ve hangi zamanlarda hangi mesajların gönderilmesi gerektiğini takip etme işlevselliği sağlayacak. Kütüphanemizi kullanan uygulamalardan, mesajları göndermek için mekanizmayı sağlamaları beklenecek: uygulama bir mesajı ekrana yazabilir, e-posta gönderebilir, SMS gönderebilir veya başka bir şey yapabilir. Kütüphanenin bu ayrıntıyı bilmesine gerek yok. Tek ihtiyacı olan, bizim sağlayacağımız `Messenger` adlı bir trait'i uygulayan bir şeydir. 15-20 numaralı listede kütüphane kodu gösteriliyor.

<Listing number="15-20" file-name="src/lib.rs" caption="A library to keep track of how close a value is to a maximum value and warn when the value is at certain levels">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

</Listing>

Bir önemli kısım, `Messenger` trait'inin, `self`'e değiştirilemez bir referans ve mesaj metni alan bir `send` metoduna sahip olmasıdır. Bu trait, mock nesnemizin gerçek bir nesne gibi kullanılabilmesi için uygulaması gereken arayüzdür. Diğer önemli kısım ise, `LimitTracker` üzerindeki `set_value` metodunun davranışını test etmek istememizdir. `value` parametresi olarak ne verdiğimizi değiştirebiliriz, ancak `set_value` bize doğrulama yapabileceğimiz bir şey döndürmez. Şunu diyebilmek isteriz: Eğer bir `Messenger` trait'ini uygulayan bir şey ve belirli bir `max` değeriyle bir `LimitTracker` oluşturursak, farklı `value` değerleri verdiğimizde messenger'ın uygun mesajları göndermesi gerekir.

Gerçekten e-posta veya SMS göndermek yerine, gönderilen mesajları kaydeden bir mock nesneye ihtiyacımız var. Yeni bir mock nesne örneği oluşturup, bunu kullanan bir `LimitTracker` oluşturup, `set_value` metodunu çağırıp, mock nesnenin beklediğimiz mesajlara sahip olup olmadığını kontrol edebiliriz. 15-21 numaralı listede, bunu yapmaya çalışan bir mock nesne uygulama denemesi gösteriliyor, ancak ödünç alma denetleyicisi buna izin vermiyor.

<Listing number="15-21" file-name="src/lib.rs" caption="An attempt to implement a `MockMessenger` that isn’t allowed by the borrow checker">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

</Listing>

Bu test kodu, gönderilen mesajları takip etmek için `sent_messages` alanı `Vec<String>` olan bir `MockMessenger` struct'ı tanımlar. Ayrıca, mesajlar listesi boş olarak başlayan yeni `MockMessenger` değerleri oluşturmayı kolaylaştırmak için bir `new` fonksiyonu tanımlarız. Sonra, `Messenger` trait'ini `MockMessenger` için uygularız, böylece bir `MockMessenger`'ı bir `LimitTracker`'a verebiliriz. `send` metodunun tanımında, parametre olarak verilen mesajı alıp, `MockMessenger`'ın `sent_messages` listesine ekleriz.

Testte, `LimitTracker`'a `value` olarak maksimum değerin yüzde 75'inden fazla bir değer verildiğinde ne olacağını test ediyoruz. Önce, mesajlar listesi boş olan yeni bir `MockMessenger` oluşturuyoruz. Sonra, yeni bir `LimitTracker` oluşturup, ona yeni `MockMessenger`'a bir referans ve `max` değeri olarak `100` veriyoruz. `LimitTracker`'ın `set_value` metodunu `80` ile çağırıyoruz; bu, 100'ün yüzde 75'inden fazladır. Sonra, `MockMessenger`'ın takip ettiği mesajlar listesinde bir mesaj olması gerektiğini doğruluyoruz.

Ancak, burada bir sorun var:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

`MockMessenger`'ı, gönderilen mesajları takip edecek şekilde değiştiremiyoruz çünkü `send` metodu `self`'e değiştirilemez bir referans alıyor. Hata metnindeki öneriyi uygulayıp hem `impl` metodunda hem de trait tanımında `&mut self` kullanamayız. Sadece test için `Messenger` trait'ini değiştirmek istemiyoruz. Bunun yerine, mevcut tasarımla test kodumuzu doğru çalıştırmanın bir yolunu bulmamız gerekiyor.

İşte burada içsel değiştirilebilirlik devreye giriyor! `sent_messages`'ı bir `RefCell<T>` içinde saklayacağız ve böylece `send` metodu, gördüğümüz mesajları saklamak için `sent_messages`'ı değiştirebilecek. 15-22 numaralı listede bunun nasıl göründüğü gösteriliyor.

<Listing number="15-22" file-name="src/lib.rs" caption="Using `RefCell<T>` to mutate an inner value while the outer value is considered immutable">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

</Listing>

Artık `sent_messages` alanı, `Vec<String>` yerine `RefCell<Vec<String>>` türünde. `new` fonksiyonunda, boş vektörün etrafına yeni bir `RefCell<Vec<String>>` örneği oluşturuyoruz.

`send` metodunun uygulamasında, ilk parametre hâlâ trait tanımıyla uyumlu olarak `self`'e değiştirilemez bir ödünç alma. `self.sent_messages` içindeki `RefCell<Vec<String>>` üzerinde `borrow_mut` çağırarak, içteki vektöre değiştirilebilir bir referans alıyoruz. Sonra, vektöre `push` çağırarak test sırasında gönderilen mesajları takip edebiliyoruz.

Son yapmamız gereken değişiklik, doğrulamada: içteki vektörde kaç öğe olduğunu görmek için, `RefCell<Vec<String>>` üzerinde `borrow` çağırıp vektöre değiştirilemez bir referans alıyoruz.

Artık `RefCell<T>`'yi nasıl kullanacağınızı gördüğünüze göre, nasıl çalıştığını daha ayrıntılı inceleyelim!

#### `RefCell<T>` ile Ödünç Almaları Çalışma Zamanında Takip Etmek

Değiştirilemez ve değiştirilebilir referanslar oluştururken sırasıyla `&` ve `&mut` sözdizimini kullanırız. `RefCell<T>` ile ise, `RefCell<T>`'ye ait güvenli API'nin parçası olan `borrow` ve `borrow_mut` metotlarını kullanırız. `borrow` metodu akıllı işaretçi türü `Ref<T>` döndürür, `borrow_mut` ise `RefMut<T>` döndürür. Her iki tür de `Deref` uygular, bu yüzden onları normal referanslar gibi kullanabiliriz.

`RefCell<T>`, şu anda kaç tane `Ref<T>` ve `RefMut<T>` akıllı işaretçisinin aktif olduğunu takip eder. Her `borrow` çağrısında, `RefCell<T>` aktif değiştirilemez ödünç alma sayısını artırır. Bir `Ref<T>` değeri kapsamdan çıktığında, değiştirilemez ödünç alma sayısı 1 azalır. Derleme zamanı ödünç alma kurallarında olduğu gibi, `RefCell<T>` herhangi bir anda birden fazla değiştirilemez ödünç alma veya bir değiştirilebilir ödünç alma olmasına izin verir.

Bu kuralları ihlal etmeye çalışırsak, referanslarda olduğu gibi derleyici hatası almak yerine, `RefCell<T>`'nin uygulaması çalışma zamanında panic ile sonlanır. 15-23 numaralı listede, 15-22'deki `send` uygulamasının değiştirilmiş hali gösteriliyor. Aynı kapsamda iki değiştirilebilir ödünç alma oluşturmaya çalışıyoruz; bu, `RefCell<T>`'nin çalışma zamanında bunu engellediğini göstermek için.

<Listing number="15-23" file-name="src/lib.rs" caption="Creating two mutable references in the same scope to see that `RefCell<T>` will panic">

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

</Listing>

Birinci `RefMut<T>` akıllı işaretçisini `one_borrow` değişkenine atıyoruz. Sonra aynı şekilde ikinci bir değiştirilebilir ödünç alma oluşturup `two_borrow` değişkenine atıyoruz. Bu, aynı kapsamda iki değiştirilebilir referans oluşturur ve buna izin verilmez. 15-23 numaralı listedeki kodu çalıştırırsak, kod derleme hatası olmadan derlenir, ancak test başarısız olur:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

Kodun, `already borrowed: BorrowMutError` mesajıyla panic ettiğine dikkat edin. `RefCell<T>`, ödünç alma kurallarının ihlallerini çalışma zamanında böyle ele alır.

Burada, ödünç alma hatalarını derleme zamanı yerine çalışma zamanında yakalamayı seçmek, hataları geliştirme sürecinin ilerleyen aşamalarında bulabileceğiniz anlamına gelir: belki de kodunuz üretime alınana kadar. Ayrıca, ödünç almaları çalışma zamanında takip etmek, derleme zamanında takip etmeye göre küçük bir performans cezası getirir. Ancak, `RefCell<T>` kullanmak, yalnızca değiştirilemez değerlerin izin verildiği bir bağlamda kullanırken, gördüğü mesajları takip etmek için kendini değiştirebilen bir mock nesne yazmayı mümkün kılar. `RefCell<T>`'yi, sağladığı ek işlevsellik için, normal referansların sunduğundan daha fazlasına ihtiyacınız olduğunda, bu dezavantajlarına rağmen kullanabilirsiniz.

<!-- Eski bağlantı, kaldırmayın -->

<a id="having-multiple-owners-of-mutable-data-by-combining-rc-t-and-ref-cell-t"></a>

### `Rc<T>` ve `RefCell<T>` ile Değiştirilebilir Verinin Birden Fazla Sahibi Olmasına İzin Vermek

`RefCell<T>`'yi kullanmanın yaygın bir yolu, onu `Rc<T>` ile birlikte kullanmaktır. `Rc<T>`'nin bazı verilerin birden fazla sahibine sahip olmasına izin verdiğini, ancak yalnızca bu veriye değiştirilemez erişim sağladığını hatırlayın. Eğer bir `Rc<T>` içinde bir `RefCell<T>` tutarsanız, hem birden fazla sahibi olan _hem de_ değiştirilebilen bir değere sahip olabilirsiniz!

Örneğin, 15-18 numaralı listede, birden fazla listenin başka bir listenin sahipliğini paylaşmasına izin vermek için `Rc<T>` kullandığımız cons listesi örneğini hatırlayın. `Rc<T>` yalnızca değiştirilemez değerler tuttuğu için, listeleri oluşturduktan sonra listedeki herhangi bir değeri değiştiremeyiz. Şimdi, listedeki değerleri değiştirme yeteneği için `RefCell<T>` ekleyelim. 15-24 numaralı listede, `Cons` tanımında bir `RefCell<T>` kullanarak, tüm listelerde saklanan değeri değiştirebileceğimizi görebilirsiniz.

<Listing number="15-24" file-name="src/main.rs" caption="Using `Rc<RefCell<i32>>` to create a `List` that we can mutate">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

</Listing>

Bir `Rc<RefCell<i32>>` örneği oluşturup, daha sonra doğrudan erişebilmek için bunu `value` adlı bir değişkende saklıyoruz. Sonra, `a`'da, `value`'yu tutan bir `Cons` varyantı ile bir `List` oluşturuyoruz. Hem `a` hem de `value` içteki `5` değerinin sahibi olsun diye `value`'yu klonlamamız gerekiyor; aksi takdirde sahiplik `value`'dan `a`'ya taşınır ya da `a`, `value`'dan ödünç alırdı.

`a` listesini bir `Rc<T>` ile sarıyoruz, böylece `b` ve `c` listelerini oluşturduğumuzda, her ikisi de `a`'ya referans verebiliyor; bunu 15-18 numaralı listede de yapmıştık.

`a`, `b` ve `c` listelerini oluşturduktan sonra, `value` içindeki değere 10 eklemek istiyoruz. Bunu, `value` üzerinde `borrow_mut` çağırarak yapıyoruz; bu, [5. Bölümde][operatör-nerede] tartıştığımız otomatik dereference özelliğini kullanarak `Rc<T>`'yi içteki `RefCell<T>` değerine dereference eder. `borrow_mut` metodu bir `RefMut<T>` akıllı işaretçi döndürür ve bunun üzerinde dereference operatörünü kullanıp içteki değeri değiştiririz.

`a`, `b` ve `c`'yi yazdırdığımızda, hepsinin artık `5` yerine değiştirilmiş `15` değerine sahip olduğunu görebiliriz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

Bu teknik oldukça kullanışlı! `RefCell<T>` kullanarak, dışarıdan bakıldığında değiştirilemez bir `List` değerine sahibiz. Ama `RefCell<T>`'nin içsel değiştirilebilirlik sağlayan metotlarını kullanarak, verimizi gerektiğinde değiştirebiliyoruz. Ödünç alma kurallarının çalışma zamanı kontrolleri, veri yarışlarından korunmamızı sağlar ve veri yapılarımızda bu esneklik için bazen biraz hızdan feragat etmeye değer. Dikkat edin, `RefCell<T>` çok iş parçacıklı kodda çalışmaz! `Mutex<T>`, `RefCell<T>`'nin iş parçacığı güvenli sürümüdür ve 16. Bölümde `Mutex<T>`'yi tartışacağız.

[operatör-nerede]: ch05-03-method-syntax.md#`->`-Operatörü-nerede?
