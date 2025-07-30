## `Send` ve `Sync` Trait'leri ile Genişletilebilir Eşzamanlılık

<!-- Eski bağlantı, silmeyin -->

<a id="extensible-concurrency-with-the-sync-and-send-traits"></a>

İlginç bir şekilde, bu bölümde şimdiye kadar bahsettiğimiz neredeyse tüm eşzamanlılık özellikleri dilin değil, standart kütüphanenin bir parçasıydı. Eşzamanlılığı yönetmek için seçenekleriniz dil veya standart kütüphane ile sınırlı değildir; kendi eşzamanlılık özelliklerinizi yazabilir veya başkalarının yazdıklarını kullanabilirsiniz.

Ancak, dilin içine gömülü olan temel eşzamanlılık kavramları arasında, standart kütüphanede yer alan `std::marker` trait'leri `Send` ve `Sync` bulunur.

### `Send` ile Sahipliğin Thread'ler Arasında Aktarılmasına İzin Vermek

`Send` işaretleyici trait'i, `Send`'i uygulayan türlerin sahipliğinin thread'ler arasında aktarılabileceğini belirtir. Neredeyse her Rust türü `Send`'i uygular, ancak bazı istisnalar vardır; örneğin `Rc<T>`: Eğer bir `Rc<T>` değerini klonlayıp klonun sahipliğini başka bir thread'e aktarmaya çalışırsanız, her iki thread de aynı anda referans sayacını güncelleyebilir. Bu nedenle, `Rc<T>` thread-güvenli performans maliyetini ödemek istemediğiniz tek thread'li durumlar için tasarlanmıştır.

Bu nedenle, Rust'ın tip sistemi ve trait sınırları, bir `Rc<T>` değerini yanlışlıkla thread'ler arasında güvenli olmayan şekilde göndermenizi engeller. 16-14 numaralı listede bunu yapmaya çalıştığımızda, `` the trait `Send` is not implemented for `Rc<Mutex<i32>>` `` hatasını almıştık. `Send`'i uygulayan `Arc<T>`'ye geçtiğimizde ise kod derlendi.

Tamamen `Send` türlerinden oluşan herhangi bir tür de otomatik olarak `Send` olarak işaretlenir. Ham işaretçiler dışında, neredeyse tüm ilkel türler `Send`'dir; ham işaretçileri 20. bölümde ele alacağız.

### `Sync` ile Birden Fazla Thread'den Erişime İzin Vermek

`Sync` işaretleyici trait'i, `Sync`'i uygulayan türlerin birden fazla thread'den referansla erişilmesinin güvenli olduğunu belirtir. Başka bir deyişle, herhangi bir `T` türü, `&T` (T'ye değiştirilemez referans) `Send`'i uyguluyorsa `Sync`'i de uygular; yani referans başka bir thread'e güvenle gönderilebilir. `Send`'e benzer şekilde, ilkel türlerin hepsi `Sync`'tir ve tamamen `Sync` türlerinden oluşan türler de `Sync`'tir.

Akıllı işaretçi `Rc<T>`, `Send`'i uygulamadığı gibi `Sync`'i de uygulamaz. 15. bölümde bahsettiğimiz `RefCell<T>` ve ilgili `Cell<T>` ailesi de `Sync` değildir. `RefCell<T>`'nin çalışma zamanında yaptığı ödünç alma denetimi thread-güvenli değildir. Akıllı işaretçi `Mutex<T>` ise `Sync`'i uygular ve birden fazla thread ile paylaşılabilir; bunu [“Birden Fazla Thread Arasında `Mutex<T>` Paylaşmak”][sharing-a-mutext-between-multiple-threads] bölümünde görmüştünüz.

### `Send` ve `Sync`'i Elle Uygulamak Güvensizdir

Tamamen `Send` ve `Sync` trait'lerini uygulayan türlerden oluşan türler de otomatik olarak bu trait'leri uygular; bu yüzden bu trait'leri elle uygulamamız gerekmez. İşaretleyici trait'ler oldukları için, uygulayacak herhangi bir metotları da yoktur. Sadece eşzamanlılıkla ilgili kuralları zorlamak için kullanışlıdırlar.

Bu trait'leri elle uygulamak, unsafe Rust kodu yazmayı gerektirir. Unsafe Rust kodunu 20. bölümde ele alacağız; şimdilik bilmeniz gereken, `Send` ve `Sync` parçalardan oluşmayan yeni eşzamanlı türler oluşturmak, güvenlik garantilerini sağlamak için dikkatli düşünmeyi gerektirir. [“The Rustonomicon”][nomicon] bu garantiler ve bunları nasıl koruyacağınız hakkında daha fazla bilgi sunar.

## Özet

Bu kitapta eşzamanlılık konusunu son kez görmüyorsunuz: bir sonraki bölüm async programlamaya odaklanacak ve 21. bölümdeki projede, burada ele alınan kavramlar daha gerçekçi bir durumda kullanılacak.

Daha önce de belirtildiği gibi, Rust'ın eşzamanlılığı nasıl yönettiğinin çok azı dilin bir parçasıdır; birçok eşzamanlılık çözümü crate olarak uygulanır. Bu crate'ler standart kütüphaneden daha hızlı gelişir, bu yüzden çoklu thread'li durumlarda kullanmak için güncel, en iyi crate'leri çevrimiçi aramayı unutmayın.

Rust standart kütüphanesi, mesajlaşma için kanallar ve `Mutex<T>`, `Arc<T>` gibi eşzamanlı ortamlarda güvenle kullanılabilen akıllı işaretçi türleri sunar. Tip sistemi ve ödünç alma denetleyicisi, bu çözümleri kullanan kodun veri yarışlarına veya geçersiz referanslara yol açmamasını sağlar. Kodunuzu derlemeyi başardığınızda, diğer dillerde yaygın olan, izlenmesi zor hatalar olmadan kodunuzun birden fazla thread'de mutlu bir şekilde çalışacağından emin olabilirsiniz. Eşzamanlı programlama artık korkulacak bir kavram değil: gidin ve programlarınızı korkusuzca eşzamanlı yapın!

[sharing-a-mutext-between-multiple-threads]: ch16-03-shared-state.md#birden-fazla-thread-arasında-mutext-paylaşmak
[nomicon]: ../nomicon/index.html
