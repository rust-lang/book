# Akıllı İşaretçiler

_Bir işaretçi_ (pointer), bellekte bir adres içeren bir değişken için genel bir kavramdır. Bu adres, başka bir veriye işaret eder veya onu "gösterir". Rust'taki en yaygın işaretçi türü, 4. Bölümde öğrendiğiniz referanstır. Referanslar `&` sembolüyle gösterilir ve işaret ettikleri değeri ödünç alırlar. Referansların veri göstermenin dışında özel bir yeteneği yoktur ve ek bir maliyeti yoktur.

_ Akıllı işaretçiler_ ise, bir işaretçi gibi davranan ancak ek olarak bazı meta veriler ve yetenekler barındıran veri yapılarıdır. Akıllı işaretçi kavramı Rust'a özgü değildir: akıllı işaretçiler ilk olarak C++'da ortaya çıkmış ve başka dillerde de mevcuttur. Rust, standart kütüphanede referansların sunduğundan daha fazla işlevsellik sağlayan çeşitli akıllı işaretçi türleri tanımlar. Genel kavramı keşfetmek için, birkaç farklı akıllı işaretçi örneğine bakacağız; bunlardan biri de _referans sayımı_ yapan bir akıllı işaretçi türüdür. Bu işaretçi, sahiplerin sayısını takip ederek verinin birden fazla sahibi olmasına olanak tanır ve hiç sahip kalmadığında veriyi temizler.

Rust'ın sahiplik ve ödünç alma kavramı ile, referanslar ve akıllı işaretçiler arasında ek bir fark daha vardır: Referanslar yalnızca veriyi ödünç alırken, çoğu durumda akıllı işaretçiler işaret ettikleri verinin _sahibi_ olurlar.

Akıllı işaretçiler genellikle yapılar (struct) kullanılarak uygulanır. Sıradan bir yapıdan farklı olarak, akıllı işaretçiler `Deref` ve `Drop` trait'lerini uygular. `Deref` trait'i, akıllı işaretçi yapısının bir örneğinin referans gibi davranmasını sağlar; böylece kodunuzu hem referanslarla hem de akıllı işaretçilerle çalışacak şekilde yazabilirsiniz. `Drop` trait'i ise, akıllı işaretçi kapsamdan çıktığında çalışacak kodu özelleştirmenize olanak tanır. Bu bölümde, bu iki trait'i tartışacak ve neden akıllı işaretçiler için önemli olduklarını göstereceğiz.

Akıllı işaretçi deseni Rust'ta sıkça kullanılan genel bir tasarım deseni olduğundan, bu bölümde mevcut tüm akıllı işaretçileri ele almayacağız. Birçok kütüphanenin kendi akıllı işaretçileri vardır ve siz de kendi akıllı işaretçinizi yazabilirsiniz. Standart kütüphanedeki en yaygın akıllı işaretçileri ele alacağız:

- Yığın üzerinde değer ayırmak için `Box<T>`
- Çoklu sahipliğe olanak tanıyan referans sayımı türü `Rc<T>`
- Derleme zamanında değil, çalışma zamanında ödünç alma kurallarını uygulayan `RefCell<T>` üzerinden erişilen `Ref<T>` ve `RefMut<T>`

Buna ek olarak, değiştirilemez bir türün içteki bir değeri değiştirmeye olanak tanıyan _içsel değiştirilebilirlik_ (interior mutability) desenini de ele alacağız. Ayrıca, referans döngülerini, bunların nasıl bellek sızıntısına yol açabileceğini ve nasıl önlenebileceğini tartışacağız.

Haydi başlayalım!
