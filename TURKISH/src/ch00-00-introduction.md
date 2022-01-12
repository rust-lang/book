# Giriş
> Not: Kitabın bu baskısı [No Starch Press'teki][nsp] [Rust Programlama Dili][nsprust] Kitabının İngilizce baskısı ile aynıdır.

[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/

Rust üzerine bir giriş kitabı olan *Rust Programlama Dili*'ne hoş geldiniz. Rust programlama dili, daha hızlı ve daha güvenilir programlar yazmanıza yardımcı olur. Üst düzey ergonomi ve düşük seviyeli kontrol programlama dili tasarımlarında bir çelişki gibi görünüyor olsa da Rust bu çelişkiye meydan okur. Rust, bir yandan bellek kullanımı gibi geleneksel olarak düşük seviyeli kontrol ile ilişkilendirilen tüm zorlukları ortadan kaldırırken, diğer yandan sağladığı güçlü teknik kapasite ve olağanüstü geliştirici deneyimini dengeleyerek bu ayrıntıları rahatlıkla kontrol etmenizi sağlar.


## Rust Kimler İçin

Rust, çeşitli nedenlerden dolayı pek çok insan için idealdir. Bu insanların ait olduğu önemli üretim gruplardan birkaçına bakalım.

### Geliştirici Ekipleri

Rust, sistem programlama bilgileri farklı düzeylerde olan kalabalık geliştirici ekipleri arasında işbirliğini tesis eden verimli bir araç olduğunu kanıtlıyor. Düşük seviyeli kod, pek çok dilde kapsamlı testler ve deneyimli geliştiriciler tarafından, kodun dikkatle incelenmesiyle yakalanabilen çözümü zor hatalara eğilimlidir. Rust derleyicisi, eşzamanlılık hataları dahil bu türden hatalı kodların derlenmesini reddederek adeta bir bekçi rolü oynar. Böylelikle derleyiciyi takımın bir üyesi olarak gören geliştirici ekibi, değerli zamanlarını hataları takip etmek yerine, programın mantığına odaklanarak geçirebilirler.

Rust sistem programlama dünyası için çağdaş geliştirici araçları da sunar:

* Rust ile birlikte gelen bağımlılık yöneticisi ve derleme aracı olan Cargo, Rust ekosisteminde bağımlılıkları ekleme, derleme ve yönetmeyi sancısız ve tutarlı hale getirir.
* Rustfmt ise geliştiriciler arasında tutarlı bir kodlama tarzı oluşturur.
* Rust Dil Sunucusu, kod tamamlama ve satır içi hata mesajları için *Entegre Geliştirme Ortamı (IDE)*  entegrasyonunu destekler.

Bunlar ve Rust ekosistemindeki diğer araçları kullanan geliştiriciler, sistem düzeyinde kod yazarken daha üretken olabilirler.

### Öğrenciler

Rust, öğrenciler ve sistem kavramlarını öğrenmekle ilgilenenler için tasarlanmıştır. Pek çok kişi Rust kullanarak işletim sistemleri geliştirme gibi alanları öğrenmiştir. Rust topluluğu oldukça misafirperver olup öğrencilerin sorularını heves ve heyacanla yanıtlamaktan çekinmezler. Bu kitap gibi girişimler aracılığıyla Rust ekipleri, sistem konseptlerini mümkün olduğunca çok kişi için, özellikle de programlamaya yeni başlayanlar için erişebilir hale getirmek istiyorlar.

### Şirketler

Büyüklü küçüklü yüzlerce şirket üretimlerinde çeşitli görevler için Rust'ı kullanıyorlar. Bu görevler arasında komut satırı araçları, web hizmetleri, DevOps araçları, gömülü cihazlar, ses, video analizi ve kod dönüştürme, kripto para birimleri, biyoinformatik, arama motorları, IOT uygulamaları, makine öğrenimi ve hatta Firefox web tarayıcısının önemli bölümleri bile bulunmakta.

### Açık Kaynak Geliştiricileri

Rust, Rust programlama dili, topluluğu, geliştirici araçları ve kütüphanelerinin oluşumuna katkı sağlamak isteyen kişiler içindir. Rust diline katkıda bulunmanızı çok isteriz.

### Hız ve İstikrara Değer Verenler

Rust, bir dilden hız ve istikrar bekleyenler içindir.
Aslında hız demekle, hem Rust ile oluşturabileceğiniz programların hızını, hem de Rust'ın kodlama sürecinde sağladığı hızı kastediyoruz. Rust'ın derleyici kontrolleri, yeni özellik ekleme ve kodun yeniden düzenlenmesi aşamalarında kararlılık sağlaması onu, benzer denetimlerin olmadığı, geliştiricilerin genellikle değişiklik yapmaktan kaçındıkları programlama dillerinden ayırır. Sıfır maliyetli soyutlamalar, elle yazılmış kodlar gibi hızlı biçimde düşük seviyeli kodlara derlenebilen üst düzey özellikler için çabalayan Rust, güvenle çalışan kodları hızlı çalışan kodlar haline getirmeye çalışır.

Burada bahsedilen büyük ilgi gurupları dışında Rust, değişik konu ve geliştirme alanlarıyla alakalı pekçok kullanıcıya da destek olmayı umuyor. Sonuç olarak Rust'ın hedefi, geliştiricilerin onlarca yıldır verdiği ödünleri, güvenlik, üretkenlik, hız ve kullanılabilirlik sağlayarak ortadan kaldırmaktır. Rust'ın bu olanaklarını deneyerek sizin için yararlı olup olmayacağına karar verin.

## Bu Kitap Kimler İçin

Halihazırda bu kitabın içeriği, okuyucusunun herhangi bir programlama dilinde kod yazdığı kabulüne dayanarak hazırlandığından, farklı programlama geçmişlerine sahip geniş bir izleyici kitlesine uygun olarak hazırlanmıştır. Kitapta programlamanın ne olduğu veya nasıl düşünülmesi gerektiği konusuna zaman ayırmadık. Eğer programlama konusunda yeniyseniz, işe programlamaya giriş konusunda yazılmış kitaplardan başlamanızı öneririz.

## Bu Kitap Nasıl Kullanılır

Genel olarak bu kitabın baştan sona doğru sırayla okunması amaçlanmıştır. Sonraki bölümler, önceki bölümlerde işlenen kavramlar üzerine inşa edilmektedir. Genellikle önceki bölümlerde etraflıca incelenmeyen konuların ayrıntılarına daha sonraki bölümlerde değinilmektedir. Bu kitapta, kavramsal ve proje olarak ayrılmış iki ayrı kısım bulunmaktadır. Rust hakkındaki bilinmesi gereken konular kavramsal kısımda işlenirken, öğrenilen konuların uygulamalarını proje kısmında gerçekleştireceğiz. Kitabın 2, 12 ve 20. bölümleri proje, diğer bölümler ise kavramsal kısımlarını oluşturmaktadır. 

Bölüm 1, Rust'ın nasıl kurulacağını, bir "Merhaba Dünya!" programının nasıl yazılacağını, Rust'ın paket yöneticisi ve yapım aracı olan Cargo'nun nasıl kullanılacağını anlatır. 

Bölüm 2, Rust diline uygulamalı giriş olarak tasarlandığından, bu bölümde işlenen yüksek düzeydeki kavramların ayrıntılarına sonraki bölümlerde değinilecektir. Kodlarla hemen haşır neşir olmak isteyenler için bu bölüm kol ve paçaların sıvanacağı yerdir.

Dilerseniz Rust'ın diğer programlama dillerindeki benzer özelliklerini tartıştığımız 3. Bölümü atlayarak, doğrudan Rust'ın mülkiyet sistemini anlatan kitabın 4. Bölümüne geçiş yapabilirsiniz. Eğer tüm ayrıntıları öğrenmek isteyen titiz bir öğrenciyseniz, bir sonraki bölüme geçmeden önce proje kısmı olan 2. Bölümü  atlayarak 3. Bölüme geçebilir, sonrasında öğrendiklerinizi uygulamak üzere yeniden 2. Bölüme dönebilirsiniz.

Bölüm 5, yapılar ve bundan böyle metot olarak adlandıracağımız yapı işlevlerini, Bölüm 6 ise, `enum`lar (numaralandırmalar), örüntü eşleme ifadeleri (`match` expressions) ve `if let` kontrol akış yapılarını içerir. Rust'ta özel türlerinizi oluştururken yapılar ve `enum`lardan fazlasıyla yararlanacaksınız.

Bölüm 7 ise, kodunuz ve genel uygulama programlama arayüzünü (API) düzenleyebilmek için Rust'ın modül sistemi ve görünürlük kuralları hakkında bilgi verir. 

Bölüm 8, vektörler, diziler ve eşleme haritaları (`hash maps`) gibi standart kütüphane tarafından sağlanan yaygın veri yapılarını anlatır.

Bölüm 9'da ise Rust'ın hata işleme felsefesini ve tekniklerini inceleyeceğiz.

Bölüm 10, farklı türlerin tek bir türmüş gibi davranabileği kodları yazmanıza olanak sağlayan `generics` veri türleri, özellikler ve yaşam süreleri hakkında ayrıntılı bilgiler içerir.

Bölüm 11 ise, Rust'ın güvenlik garantilerine rağmen program mantığınızın doğru olup olmadığından emin olabilmeniz için gerekli olan testlerle ilgilidir.

Bölüm 12'de, metni dosyalarda arayan `grep` komut satırı aracından bir işlev alt kümesi oluşturarak, önceki bölümlerde öğrendiğimiz çoğu kavramı kullanarak bilgilerimizi pekiştirmeye çalışacağız.

Bölüm 13, Rust'ın işlevsel programlama dillerinden esinlendiği özellikler olan kapamalar ve yineleyicilere odaklanıyor. 

Bölüm 14'te, Cargo'yu derinlemesine inceleyecek, kendi kütüphanelerinizi başkalarıyla paylaşmanın en iyi yollarından bahsedeceğiz. 

Bölüm 15, standart kütüphanenin sunduğu akıllı işaretçileri ve bu işaretçilerin işlevselliğini sağlayan özellikleri anlatır.

Bölüm 16'da, eşzamanlı programlamanın çeşitli modellerini inceleyecek ve Rust'ın paralel görevleri çok sayıda iş parçacığına nasıl korkusuzca dağıtmamıza yardım ettiğini konuşacağız.

Bölüm 17, Rust deyimlerini aşina olduğunuz nesne yönelimli programlama ilkeleriyle karşılaştırır.

Bölüm 18, Rust programlarında fikirleri ifade etmenin güçlü birer yolu olan örüntüler ve örüntü eşleştirme üzerine bir başvuru kaynağıdır.

Bölüm 19, Güvenli olmayan Rust kodları, makrolar, yaşam süreleri, özellikler, türler, işlevler, ve kapamalar hakkında fazladan ayrıntılar gibi bir dizi ilginç ve gelişmiş konuları içerir.

Bölüm 20'de, eşzamanlı çoklu görevleri düşük seviyede bir program olarak çalıştıran web sunucusu projesini bitireceğiz.

Son olarak dil hakkında başvuru niteliğinde yararlı bilgiler içeren bazı ekler aşağıda listelenmektedir.
Ek A, Rust'ın anahtar kelimelerini içerir.
Ek B, Rust programlama dilinin işleç ve sembollerine yer verir.
Ek C, Standart kütüphanenin sağladığı türetilebilir özellikleri kapsar.
Ek D, Bazı faydalı geliştirme araçlarına atıfta bulunur.
Ek E'de ise, Rust'ın sürümlerine yer verilmektedir.

Öğrenim sürenizce kitabın bazı bölümleri atlamak isterseniz, bunun kitabı yanlış okuduğunuz anlamına gelmediğini bilin ve bunu yapmaktan çekinmeyin. Herhangi bir güçlükle karşılaştığınızda önceki bölümlere dönmeniz gerekse bile size uygun olan öğrenim yolunu uygulamaktan çekinmeyin.

Rust öğrenme sürecinin önemli bir parçası, derleyicinin görüntülediği hata mesajlarının nasıl okunacağını öğrenmektir. Bu mesajlar sizi doğru koda yönlendireceğinden, pekçok hata senaryosunu içeren derlenmeyen örnekler vereceğiz. Rastgele bir örneği kopyalayıp çalıştırır ve bir hata alırsanız bunun hata gösterimi olup olmadığını anlamak için ilişkili metni okuduğunuzdan emin olun. Maskotumuz Ferris'i dikkatle takip ederseniz, çalışmaması gereken kodları kolayca anlayabilirsiniz. 


| Ferris                                                                 | Anlamı                                          |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    | Bu kod derlenmez!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              | Bu kod panik üretir!                   |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              | Bu kod bloğu güvenli olmayan kod içerir.|
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| Bu kod beklenen davranışı üretmiyor. |


Çoğu durumda, hangi kod sürümünün çalışması gerektiğine dair sizi yönlendireceğiz.

## Kaynak Kodu

Bu kitabın oluşmasını sağlayan kaynak kodlara [GitHub][kitap]

[kitap]: https://github.com/RustDili/rust-book-tr/tree/main/TURKISH/src üzerinden ulaşabilirsiniz.
