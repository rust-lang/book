## Ek G - Rust Nasıl Yapılır ve “Nightly Rust”

Bu ek, Rust'ın nasıl üretildiği ve bunun bir Rust kullanıcısı olarak sizi nasıl etkilediği ile ilgilidir.
geliştirici.

### Durgunluk Olmadan İstikrar

Bir dil olarak Rust, kodunuzun kararlılığına çok önem verir. Biz istiyoruz ki
Pas, üzerine inşa edebileceğiniz kaya gibi sağlam bir temeldir ve eğer işler
sürekli değişiyor, bu imkansız olurdu. Aynı zamanda, eğer
yeni özellikleri denediğimizde, önemli kusurları daha sonra fark edebiliriz.
Artık bir şeyleri değiştiremeyeceğimiz zaman, onların serbest bırakılması.

Bizim bu soruna çözümümüz “durgunluk olmadan istikrar” dediğimiz şeydir,
ve yol gösterici prensibimiz şudur: asla bir üst modele geçmekten korkmamalısınız
kararlı Rust'ın yeni sürümü. Her yükseltme acısız olmalı, ancak aynı zamanda
size yeni özellikler, daha az hata ve daha hızlı derleme süreleri sunar.

### Choo, Choo! Kanalları Serbest Bırakın ve Trenlere Binin

Rust geliştirme bir _tren programı_ üzerinde çalışır. Yani, tüm geliştirme
Rust deposunun `master` dalında yapılır. Sürümler bir yazılımı takip eder
Cisco IOS ve diğer yazılımlar tarafından kullanılan sürüm treni modeli
projeler. Rust için üç _serbest bırakma kanalı_ vardır:

- Gecelik
- Beta
- Kararlı

Çoğu Rust geliştiricisi öncelikle kararlı kanalı kullanır, ancak isteyenler
Deneysel yeni özellikleri denemek için nightly veya beta kullanabilir.

İşte geliştirme ve yayınlama sürecinin nasıl işlediğine dair bir örnek: hadi
Rust ekibinin Rust 1.5 sürümü üzerinde çalıştığını varsayalım. Bu sürüm
Aralık 2015'te gerçekleşti, ancak bize gerçekçi bir versiyon sunacak
sayılar. Rust'a yeni bir özellik eklendi: yeni bir commit `master`
şube. Her gece, Rust'ın yeni bir gece sürümü üretilir. Her gün bir
yayın günü ve bu yayınlar yayın altyapımız tarafından oluşturulur
otomatik olarak. Yani zaman geçtikçe, yayınlarımız gecede bir kez bu şekilde görünüyor:


```text
nightly: * - - * - - *
```

Her altı haftada bir, yeni bir sürüm hazırlama zamanı! Yeni sürümün `beta` dalı
Rust deposu, nightly tarafından kullanılan `master` dalından ayrılır. Şimdi,
iki sürüm var:

```text
nightly: * - - * - - *
                     |
beta:                *
```

Çoğu Rust kullanıcısı beta sürümlerini aktif olarak kullanmaz, ancak beta sürümlerine karşı test
Rust'ın olası gerilemeleri keşfetmesine yardımcı olmak için CI sistemleri. Bu arada,
hala her gece bir yayın var:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Diyelim ki bir regresyon bulundu. İyi ki betayı test etmek için biraz zamanımız oldu
regresyon kararlı sürüme sızmadan önce yayınlayın! Düzeltme uygulanır
'a gönderilir, böylece nightly düzeltilir ve ardından düzeltme
'beta' dalı ve yeni bir beta sürümü üretilir:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

İlk betanın oluşturulmasından altı hafta sonra, kararlı sürümün zamanı geldi! Bu
beta` dalından `stable` dalı üretilir:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Yaşasın! Rust 1.5 tamamlandı! Ancak, bir şeyi unuttuk: çünkü altı
haftalar geçtikten sonra, Rust'ın _bir sonraki_ sürümü olan 1.6'nın yeni bir betasına da ihtiyacımız var.
Yani `beta`dan `stable` dalı çıktıktan sonra, `beta` dalının bir sonraki sürümü
tekrar “gece” dışında:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

Buna “tren modeli” denir çünkü her altı haftada bir, bir sürüm “trenden ayrılır”.
istasyonu", ancak yine de beta kanalından geçmeden önce bir yolculuk yapmak zorundadır.
kararlı bir sürüm olarak gelir.

Rust, saat gibi her altı haftada bir yayınlanır. Eğer bir Rust sürümünün tarihini biliyorsanız
yayınlandığında, bir sonrakinin tarihini bilebilirsiniz: altı hafta sonra. Güzel.
her altı haftada bir planlanmış sürümlere sahip olmanın yönü, bir sonraki trenin
yakında geliyor. Bir özellik belirli bir sürümü kaçırırsa, gerek yoktur
Endişelenmenize gerek yok: kısa bir süre sonra bir tane daha olacak! Bu, baskıyı azaltmaya yardımcı olur
son sürüm tarihine yakın bir zamanda muhtemelen cilalanmamış özellikleri gizlice eklemek için.

Bu süreç sayesinde, Rust'ın bir sonraki sürümüne her zaman göz atabilir ve
yükseltmenin kolay olduğunu kendiniz doğrulayın: eğer bir beta sürümü
beklendiği gibi çalışıyorsa, bunu ekibe bildirebilir ve teslimattan önce düzeltilmesini sağlayabilirsiniz.
bi̇r sonraki̇ kararli sürüm gerçekleşi̇yor! Bir beta sürümünde kırılma nispeten nadirdir, ancak
rustc' hala bir yazılım parçasıdır ve hatalar mevcuttur.

### Bakım süresi

Rust projesi en son kararlı sürümü desteklemektedir. Yeni bir kararlı
sürümü yayınlandığında, eski sürüm kullanım ömrünün sonuna (EOL) ulaşır. Bu şu anlama gelir
her sürüm altı hafta boyunca desteklenmektedir.

### Kararsız Özellikler

Bu sürüm modeliyle ilgili bir sorun daha var: kararsız özellikler. Rust bir
hangi özelliklerin etkin olduğunu belirlemek için “özellik bayrakları” adı verilen teknik
sürüm verildi. Yeni bir özellik aktif geliştirme aşamasındaysa
'master', ve bu nedenle, gece, ancak bir _özellik bayrağının_ arkasında. Eğer siz, bir
kullanıcısıysanız, devam eden çalışma özelliğini denemek istiyorsanız, deneyebilirsiniz, ancak
Rust'ın bir gecelik sürümünü kullanarak ve kaynak kodunuzu
tercih etmek için uygun bayrak.

Rust'ın beta veya kararlı bir sürümünü kullanıyorsanız, herhangi bir özelliği kullanamazsınız
bayraklar. Bu, yeni özelliklerle pratik kullanım elde etmemizi sağlayan anahtardır
onları sonsuza dek istikrarlı ilan etmeden önce. Kanamayı tercih etmek isteyenler
kenarı bunu yapabilir ve kaya gibi sağlam bir deneyim isteyenler
kararlı ve kodlarının bozulmayacağını biliyorlar. Durgunluk olmadan istikrar.

Bu kitap yalnızca kararlı özellikler hakkında bilgi içerir, çünkü devam eden
özellikler hala değişiyor ve kesinlikle bu kitapla bu kitap arasında farklı olacaklar.
kitabının ne zaman yazıldığını ve kararlı sürümlerde ne zaman etkinleştirildiklerini öğrenebilirsiniz. Bulabilirsin
nightly özellikleri için belgeler çevrimiçi.

### Rustup ve Rust Nightly'nin Rolü

Rustup, Rust'ın farklı sürüm kanalları arasında geçiş yapmayı kolaylaştırır, bir
küresel veya proje bazında. Varsayılan olarak, kararlı Rust yüklü olacaktır. için
Örneğin, gece yükleyin:

```console
$ rustup toolchain install nightly
```

Tüm _toolchains_ (Rust sürümleri ve ilgili araç zincirlerini) görebilirsiniz.
bileşenleri) de `rustup` ile yüklediniz. İşte bir örnek
yazarlarınızın Windows bilgisayarının:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Gördüğünüz gibi, kararlı araç zinciri varsayılandır. Çoğu Rust kullanıcısı kararlı
çoğu zaman. Çoğu zaman sabit kullanmak isteyebilirsiniz, ancak
Belirli bir projede gecelik olarak, çünkü son teknoloji bir özelliği önemsiyorsunuz.
Bunu yapmak için, söz konusu projenin dizininde `rustup override` özelliğini kullanarak
gece araç zincirini, o dizindeyken `rustup`ın kullanması gereken araç zinciri olarak ayarlayın:

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

Şimdi, `rustc` veya `cargo` öğesini her çağırdığınızda
_~/projects/needs-nightly_, `rustup` nightly kullandığınızdan emin olacaktır
Rust, varsayılan kararlı Rust yerine. Bu, şu durumlarda kullanışlı olur
bir sürü Rust projesi var!

### RFC Süreci ve Ekipler

Peki bu yeni özellikler hakkında nasıl bilgi edinebilirsiniz? Rust'ın geliştirme modeli şu şekildedir
bir _Request For Comments (RFC) süreci_. Eğer Rust'ta bir iyileştirme istiyorsanız,
RFC adı verilen bir öneri yazabilirsiniz.

Herkes Rust'ı geliştirmek için RFC yazabilir ve teklifler gözden geçirilir ve
birçok konu alt ekibinden oluşan Rust ekibi tarafından tartışılmaktadır. Burada
ekiplerin tam listesi [Rust'ın web sitesinde](https://www.rust-lang.org/governance) yer almaktadır.
Projenin her bir alanı: dil tasarımı, derleyici uygulaması,
altyapı, dokümantasyon ve daha fazlası. Uygun ekip aşağıdaki belgeleri okur
teklifini ve yorumları değerlendirir, kendi yorumlarını yazar ve sonunda
özelliği kabul etmek veya reddetmek için fikir birliği vardır.

Özellik kabul edilirse, Rust deposunda bir sorun açılır ve
Birisi bunu uygulayabilir. Bunu çok iyi bir şekilde uygulayan kişi
ilk etapta özelliği öneren kişi! Uygulama ne zaman
hazır olduğunda, tartıştığımız gibi bir özellik kapısının arkasındaki `master` dalına iner
[“Kararsız Özellikler”](#kararsız-özellikler)<!-- ignore --> bölümünde.

Bir süre sonra, gecelik sürümleri kullanan Rust geliştiricileri
yeni özelliği denemek için ekip üyeleri özelliği, nasıl kullanıldığını ve
üzerinde çalışılır ve kararlı Rust'a girip girmeyeceğine karar verilir.
Karar ilerlemek yönündeyse, özellik kapısı kaldırılır ve
özelliği artık kararlı olarak kabul ediliyor! Trenleri yeni bir kararlı sürüme doğru sürüyor
Rust'ın.
