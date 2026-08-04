# Giriş

> Not: Kitabın bu baskısı, [No Starch Press][nsp]'ten basılı ve e-kitap
> biçiminde edinilebilen [Rust Programlama Dili][nsprust] ile aynıdır.

[nsprust]: https://nostarch.com/rust-programming-language-3rd-edition
[nsp]: https://nostarch.com/

_Rust Programlama Diline_ hoş geldiniz: Rust hakkında bir giriş kitabı. Rust
programlama dili daha hızlı, daha güvenilir yazılım yazmanıza yardımcı olur.
Üst düzey ergonomi ve alt düzey kontrol, programlama dili tasarımında çoğu
zaman çelişir; Rust bu çatışmaya meydan okur. Güçlü teknik kapasite ile harika
bir geliştirici deneyimini dengeleyen Rust, geleneksel olarak böyle bir
kontrole eşlik eden tüm zorluklar olmadan alt düzey ayrıntıları (bellek
kullanımı gibi) kontrol etme seçeneği sunar.

## Rust Kimler İçin

Rust, çeşitli nedenlerle birçok kişi için idealdir. En önemli gruplardan
birkaçına bakalım.

### Geliştirici Takımları

Rust, değişen düzeylerde sistem programlama bilgisine sahip büyük geliştirici
takımları arasında iş birliği için üretken bir araç olduğunu kanıtlıyor. Alt
düzey kod, çoğu başka dilde ancak kapsamlı testler ve deneyimli geliştiriciler
tarafından yapılan dikkatli kod incelemeleriyle yakalanabilen çeşitli ince
hatalara açıktır. Rust'ta derleyici, eşzamanlılık hataları da dahil olmak
üzere bu kaçamak hatalara sahip kodu derlemeyi reddederek bir bekçi rolü
oynar. Takım, derleyiciyle birlikte çalışarak zamanını hataların peşinde
koşmak yerine programın mantığına odaklamak için harcayabilir.

Rust ayrıca sistem programlama dünyasına çağdaş geliştirici araçları getirir:

- Dahili bağımlılık yöneticisi ve derleme aracı olan Cargo, bağımlılıkları
  eklemeyi, derlemeyi ve yönetmeyi Rust ekosistemi genelinde acısız ve tutarlı
  hâle getirir.
- `rustfmt` biçimlendirme aracı, geliştiriciler arasında tutarlı bir kod
  stili sağlar.
- Rust Dil Sunucusu, kod tamamlama ve satır içi hata mesajları için tümleşik
  geliştirme ortamı (IDE) tümleştirmesini güçlendirir.

Geliştiriciler, Rust ekosistemindeki bu ve diğer araçları kullanarak sistem
düzeyinde kod yazarken üretken olabilirler.

### Öğrenciler

Rust, sistem kavramlarını öğrenmekle ilgilenen öğrenciler ve kişiler içindir.
Rust kullanarak birçok kişi işletim sistemi geliştirme gibi konuları
öğrendi. Topluluk çok misafirperver ve öğrencilerin sorularını yanıtlamaktan
memnuniyet duyuyor. Rust ekipleri, bu kitap gibi çalışmalarla sistem
kavramlarını daha fazla kişi için, özellikle de programlamaya yeni başlayanlar
için daha erişilebilir kılmak istiyor.

### Şirketler

Yüzlerce büyük ve küçük şirket; komut satırı araçları, web hizmetleri, DevOps
araçları, gömülü cihazlar, ses ve video analizi ve dönüştürme,
kripto para birimleri, biyoinformatik, arama motorları, Nesnelerin İnterneti
uygulamaları, makine öğrenimi ve hatta Firefox web tarayıcısının önemli
bölümleri dahil olmak üzere çeşitli görevler için üretimde Rust kullanıyor.

### Açık Kaynak Geliştiricileri

Rust, Rust programlama dilini, topluluğunu, geliştirici araçlarını ve
kütüphanelerini inşa etmek isteyen kişiler içindir. Rust diline katkıda
bulunmanızı çok isteriz.

### Hız ve İstikrarı Değer Verenler

Rust, bir dilde hız ve istikrar arzulayan kişiler içindir. Hız derken hem Rust
kodunun ne kadar hızlı çalışabileceğini hem de Rust'ın programları ne kadar
hızlı yazmanıza izin verdiğini kastediyoruz. Rust derleyicisinin kontrolleri,
özellik eklemeleri ve yeniden düzenleme yoluyla istikrarı sağlar. Bu, bu
kontrollere sahip olmayan dillerdeki, geliştiricilerin çoğu zaman değiştirmeye
korktuğu kırılgan miras kodun tam tersidir. Sıfır maliyetli soyutlamalar için
çabalayarak (elle yazılan kod kadar hızlı alt düzey koda derlenen üst düzey
özellikler) Rust, güvenli kodun aynı zamanda hızlı kod olmasını hedefler.

Rust dili daha pek çok kullanıcıyı da desteklemeyi umuyor; burada
bahsedilenler yalnızca en büyük paydaşlardan bazılarıdır. Genel olarak Rust'ın
en büyük tutkusu, güvenlik _ve_ üretkenlik, hız _ve_ ergonomi sağlayarak
programcıların onlarca yıldır kabul ettiği takasları ortadan kaldırmaktır.
Rust'a bir şans verin ve seçimlerinin size uyup uymadığına bakın.

## Bu Kitap Kimler İçin

Bu kitap, başka bir programlama dilinde kod yazdığınızı varsayar, ancak hangi
dil olduğu konusunda herhangi bir varsayımda bulunmaz. Materyali, çok çeşitli
programlama geçmişlerinden gelenler için geniş ölçüde erişilebilir kılmaya
çalıştık. Programlamanın _ne_ olduğu ya da onun hakkında nasıl düşünüleceği
üzerine çok fazla zaman harcamıyoruz. Eğer programlamada tamamen yeniyseniz,
programlamaya özel olarak giriş sağlayan bir kitap okumanız sizin için daha
iyi olur.

## Bu Kitap Nasıl Kullanılır

Genel olarak bu kitap, onu baştan sona sırayla okuduğunuzu varsayar. Sonraki
bölümler önceki bölümlerdeki kavramlar üzerine kurulur ve önceki bölümler
belirli bir konunun ayrıntılarına girmeyebilir, ancak konuyu daha sonraki bir
bölümde yeniden ele alır.

Bu kitapta iki tür bölüm bulacaksınız: kavram bölümleri ve proje bölümleri.
Kavram bölümlerinde Rust'ın bir yönünü öğreneceksiniz. Proje bölümlerinde,
şimdiye kadar öğrendiklerinizi uygulayarak birlikte küçük programlar inşa
edeceğiz. Bölüm 2, Bölüm 12 ve Bölüm 21 proje bölümleridir; geri kalanı
kavram bölümleridir.

**Bölüm 1**, Rust'ın nasıl kurulacağını, "Merhaba, dünya!" programının nasıl
yazılacağını ve Rust'ın paket yöneticisi ve derleme aracı olan Cargo'nun nasıl
kullanılacağını açıklar. **Bölüm 2**, size bir sayı tahmin oyunu inşa ettiren,
Rust'ta program yazmaya uygulamalı bir giriştir. Burada kavramları üst düzeyde
ele alıyoruz ve sonraki bölümler ek ayrıntı sağlayacaktır. Hemen işe
koyulmak istiyorsanız Bölüm 2 tam size göre. Bir sonrakine geçmeden önce her
ayrıntıyı öğrenmeyi tercih eden özellikle titiz bir öğrenciyseniz, Bölüm 2'yi
atlayıp diğer programlama dillerindekilere benzer Rust özelliklerini ele alan
**Bölüm 3'e** doğrudan geçmek isteyebilirsiniz; ardından, öğrendiğiniz
ayrıntıları uygulayan bir proje üzerinde çalışmak istediğinizde Bölüm 2'ye
dönebilirsiniz.

**Bölüm 4'te** Rust'ın sahiplik sistemini öğreneceksiniz. **Bölüm 5** struct
ve metodları tartışır. **Bölüm 6**, enum'ları, `match` ifadelerini ve `if let`
ile `let...else` kontrol akışı yapılarını kapsar. Özel türler oluşturmak için
struct ve enum kullanacaksınız.

**Bölüm 7'de** Rust'ın modül sistemini ve kodunuzu ve onun genel uygulama
programlama arayüzünü (API) düzenlemek için gizlilik kurallarını
öğreneceksiniz. **Bölüm 8**, standart kütüphanenin sağladığı bazı yaygın
koleksiyon veri yapılarını tartışır: vektörler, string'ler ve hash map'ler.
**Bölüm 9**, Rust'ın hata yönetimi felsefesini ve tekniklerini inceler.

**Bölüm 10**, birden çok türe uygulanan kod tanımlamanıza güç veren generic,
trait ve yaşam sürelerine dalar. **Bölüm 11** tamamen testlerle ilgilidir;
Rust'ın güvenlik garantileriyle bile programınızın mantığının doğru olduğundan
emin olmak için bu gereklidir. **Bölüm 12'de**, dosyalar içinde metin arayan
`grep` komut satırı aracının işlevselliğinin bir alt kümesinin kendi
uygulamamızı inşa edeceğiz. Bunun için önceki bölümlerde tartıştığımız
kavramların çoğunu kullanacağız.

**Bölüm 13**, fonksiyonel programlama dillerinden gelen Rust özellikleri olan
closure ve yineleyicileri inceler. **Bölüm 14'te**, Cargo'yu daha derinlemesine
inceleyeceğiz ve kütüphanelerinizi başkalarıyla paylaşmak için en iyi
uygulamalardan bahsedeceğiz. **Bölüm 15**, standart kütüphanenin sağladığı
akıllı işaretçileri ve işlevlerini mümkün kılan trait'leri tartışır.

**Bölüm 16'da**, farklı eşzamanlı programlama modellerini gözden geçirecek ve
Rust'ın birden çok iş parçacığında korkusuzca programlamanıza nasıl yardımcı
olduğundan bahsedeceğiz. **Bölüm 17'de**, Rust'ın async ve await sözdizimini,
görevler, future'lar ve stream'ler ile bunların sağladığı hafif eşzamanlılık
modelini inceleyerek bunun üzerine inşa ediyoruz.

**Bölüm 18**, Rust deyimlerinin aşina olabileceğiniz nesne yönelimli
programlama ilkeleriyle nasıl karşılaştırıldığına bakar. **Bölüm 19**, Rust
programları boyunca fikirleri ifade etmenin güçlü yolları olan desenler ve
desen eşleştirme üzerine bir başvurudur. **Bölüm 20**, unsafe Rust, makrolar ve
yaşam süreleri, trait'ler, türler, fonksiyonlar ve closure'lar hakkında daha
fazlası dahil olmak üzere çeşitli ileri düzey ilginç konular içerir.

**Bölüm 21'de**, alt düzey çok iş parçacıklı bir web sunucusu
uygulayacağımız bir projeyi tamamlayacağız!

Son olarak, bazı ekler dil hakkında yararlı bilgileri daha başvuru benzeri bir
biçimde içerir. **Ek A**, Rust'ın anahtar kelimelerini, **Ek B**, Rust'ın
operatörlerini ve sembollerini, **Ek C**, standart kütüphanenin sağladığı
türetilebilir trait'leri, **Ek D** bazı yararlı geliştirme araçlarını kapsar ve
**Ek E**, Rust edition'larını açıklar. **Ek F'de** kitabın çevirilerini
bulabilirsiniz ve **Ek G'de** Rust'ın nasıl yapıldığını ve nightly Rust'ın ne
olduğunu ele alacağız.

Bu kitabı okumanın yanlış bir yolu yoktur: Atlayarak geçmek istiyorsanız,
buyurun! Herhangi bir kafa karışıklığı yaşarsanız önceki bölümlere geri dönmeniz
gerekebilir. Ama size ne işe yararsa onu yapın.

<span id="ferris"></span>

Rust öğrenme sürecinin önemli bir parçası, derleyicinin görüntülediği hata
mesajlarını okumayı öğrenmektir: Bunlar sizi çalışan koda yönlendirecektir. Bu
nedenle, derlemeyen birçok örnek ve her durumda derleyicinin size göstereceği
hata mesajını sağlayacağız. Rastgele bir örneği girip çalıştırırsanız
derlenmeyebileceğini bilin! Çalıştırmaya çalıştığınız örneğin hata vermesinin
amaçlanıp amaçlanmadığını görmek için çevreleyen metni okuduğunuzdan emin
olun. Çoğu durumda, derlenmeyen herhangi bir kodun doğru sürümüne sizi
yönlendireceğiz. Ferris ayrıca çalışması amaçlanmayan kodu ayırt etmenize
yardımcı olacaktır:

| Ferris                                                                                                               | Anlamı                              |
| -------------------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Soru işaretli Ferris"/>                       | Bu kod derlenmiyor!                 |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ellerini kaldırmış Ferris"/>                            | Bu kod panic veriyor!               |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Bir pençesi kalkık, omuz silken Ferris"/> | Bu kod istenen davranışı üretmiyor. |

Çoğu durumda, derlenmeyen herhangi bir kodun doğru sürümüne sizi
yönlendireceğiz.

## Kaynak Kodu

Bu kitabın üretildiği kaynak dosyalar [GitHub'da][book] bulunabilir.

[book]: https://github.com/rust-kitabi-turkce/rust-turkce-kitap
