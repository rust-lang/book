# Büyüyen Projeleri Paketler, Kasalar ve Modüllerle Yönetme

Büyük programlar yazdıkça, kodunuzu düzenlemek giderek daha kolay hale gelecektir.
Önemli. İlgili işlevleri gruplayarak ve kodu farklı işlevlerle ayırarak
özellikleri, belirli bir kodu uygulayan kodu nerede bulacağınızı netleştireceksiniz
ve bir özelliğin nasıl çalıştığını değiştirmek için nereye gidileceği.

Şimdiye kadar yazdığımız programlar tek bir dosyada tek bir modüldeydi. Olarak
proje büyüdükçe, kodu birden fazla modüle bölerek düzenlemelisiniz
ve ardından birden fazla dosya. Bir paket birden fazla ikili kasa içerebilir ve
isteğe bağlı olarak bir kütüphane sandığı. Bir paket büyüdükçe, parçaları
dış bağımlılıklar haline gelen ayrı kasalar. Bu bölüm tüm
bu teknikler. Birbiriyle ilişkili bir dizi projeden oluşan çok büyük projeler için
birlikte gelişen paketler, Cargo _workspaces_ sağlar, ki biz bunu ele alacağız
Bölüm 14'teki [“Kargo Çalışma Alanları”][çalışma alanları]<!-- yoksay --> bölümünde.

Ayrıca, yeniden kullanmanıza olanak tanıyan uygulama ayrıntılarını kapsüllemeyi de tartışacağız
kodu daha yüksek bir seviyede: bir kez bir işlemi uyguladığınızda, diğer kodlar
nasıl olduğunu bilmek zorunda kalmadan kodunuzu genel arayüzü aracılığıyla çağırın.
uygulama çalışır. Kod yazma şekliniz, hangi bölümlerin herkese açık olduğunu tanımlar
ve hangi kısımların sizin kullanacağınız özel uygulama detayları olduğu
değiştirme hakkını saklı tutar. Bu, ayrıntı miktarını sınırlamanın başka bir yoludur
kafanızda tutmanız gerekir.

İlgili bir kavram da kapsamdır: kodun yazıldığı iç içe geçmiş bağlamın bir
“kapsam dahilinde” olarak tanımlanan isimler kümesi. Okurken, yazarken ve
Kod derlerken, programcıların ve derleyicilerin belirli bir
Belirli bir noktadaki isim bir değişken, fonksiyon, struct, enum, modül anlamına gelir,
sabit veya başka bir öğe ve bu öğenin ne anlama geldiği. Kapsamlar oluşturabilir ve
hangi adların kapsam içinde veya dışında olduğunu değiştirin. Aynı ada sahip iki öğeye sahip olamazsınız
Aynı kapsamda aynı isim; isim çakışmalarını çözmek için araçlar mevcuttur.

Rust, kodunuzu yönetebilmenizi sağlayan bir dizi özelliğe sahiptir.
Hangi detayların açık, hangi detayların gizli olduğu da dahil olmak üzere organizasyon,
ve programlarınızdaki her bir kapsamda hangi isimlerin yer aldığı. Bu özellikler, bazen
toplu olarak _modül sistemi_ olarak anılır, şunları içerir:

* **Paketler**: Sandık oluşturmanıza, test etmenize ve paylaşmanıza olanak tanıyan bir Cargo özelliği
* **Kasalar**: Bir kütüphane veya çalıştırılabilir dosya üreten bir modül ağacı
* **Modüller ve kullanım**: Organizasyon, kapsam ve gizliliği kontrol etmenize izin verin
yollar
* **Yollar**: Yapı, fonksiyon veya modül gibi bir öğeyi adlandırmanın bir yolu

Bu bölümde, tüm bu özellikleri ele alacağız, nasıl etkileşimde bulunduklarını tartışacağız ve
Kapsamı yönetmek için bunları nasıl kullanacağınızı açıklayın. Sonunda, sağlam bir bilgiye sahip olmalısınız
Modül sistemini anlamak ve kapsamlarla bir profesyonel gibi çalışabilmek!

[workspaces]: ch14-03-cargo-workspaces.md

