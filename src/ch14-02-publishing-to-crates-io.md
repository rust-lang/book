## Crates.io'da bir Crate yayınlamak

crates.io](https://crates.io/)<!-- ignore --> adresindeki paketleri projemizin
bağımlılıkları olarak kullandık, ancak siz de kendi paketlerinizi yayınlayarak kodunuzu diğer insanlarla
paylaşabilirsiniz. [crates.io](https://crates.io/)<!-- ignore --> adresindeki crate kayıt defteri, paketlerinizin
kaynak kodunu dağıtır, bu nedenle öncelikle açık kaynak kodunu barındırır.

Rust ve Cargo, yayınladığınız paketin
insanlar tarafından bulunmasını ve kullanılmasını kolaylaştıran özelliklere sahiptir. Daha sonra bu özelliklerden bazılarından bahsedeceğiz ve ardından
bir paketin nasıl yayınlanacağını açıklayacağız.

### Faydalı Dokümantasyon Yorumları Yapmak

Paketlerinizi doğru bir şekilde belgelemek, diğer kullanıcıların bunları nasıl ve ne zaman
kullanacaklarını bilmelerine yardımcı olacaktır, bu nedenle belge yazmak için zaman ayırmaya değer. Bölüm
3'te, iki eğik çizgi, `//` kullanarak Rust kodunu nasıl yorumlayacağımızı tartıştık. Rust ayrıca
dokümantasyon için özel bir yorum türüne sahiptir, bu yorum uygun bir şekilde
_documentation comment_ olarak bilinir ve HTML dokümantasyonu oluşturur. HTML
,
crate'inizin nasıl _uygulandığı_ yerine nasıl _kullanılacağını_ bilmek isteyen programcılar için
amaçlanan genel API öğeleri için dokümantasyon yorumlarının içeriğini görüntüler.

Belgeleme yorumları iki yerine üç eğik çizgi, `///`, kullanır ve metni biçimlendirmek için
Markdown notasyonunu destekler. Belgeleme yorumlarını belgeledikleri öğeden hemen
önce yerleştirin. Liste 14-1, `my_crate` adlı bir sandıktaki `add_one` işlevi için
belgeleme yorumlarını göstermektedir.

<img alt="Rendered HTML documentation for the `add_one` function of `my_crate`" src="img/trpl14-01.png" class="center" />

<span class="caption">Figure 14-1: HTML documentation for the `add_one`
function</span>

#### Sık Kullanılan Bölümler

HTML'de “Örnekler” başlıklı bir
bölümü oluşturmak için Liste 14-1'deki `# Örnekler' Markdown başlığını kullandık. İşte
yazarlarının belgelerinde yaygın olarak kullandıkları diğer bazı bölümler:

- Panikler**: Belgelenen işlevin
 panik yaratabileceği senaryolar. Programlarının paniklemesini istemeyen fonksiyonu çağıranlar
 bu durumlarda fonksiyonu çağırmadıklarından emin olmalıdırlar.
- Hatalar**: Fonksiyon bir `Sonuç' döndürüyorsa, oluşabilecek
 hata türlerini ve hangi koşulların bu hataların
 döndürülmesine neden olabileceğini açıklamak, arayanlara yardımcı olabilir, böylece
 farklı hata türlerini farklı şekillerde ele almak için kod yazabilirler.
- Güvenlik**: Eğer fonksiyon çağırmak için `güvensiz` ise (güvensizliği
 Bölüm 20'de tartışıyoruz), fonksiyonun neden güvensiz olduğunu açıklayan
 ve fonksiyonun çağıranların uymasını beklediği değişmezleri kapsayan bir bölüm olmalıdır.

Çoğu dokümantasyon açıklamasında bu bölümlerin hepsine gerek yoktur, ancak bu, kodunuzun kullanıcıların
bilmek isteyeceği yönlerini size hatırlatmak için
iyi bir kontrol listesidir.

#### Test Olarak Dokümantasyon Yorumları

Belgelendirme yorumlarınıza örnek kod blokları eklemek,
adresinde kütüphanenizin nasıl kullanılacağını göstermeye yardımcı olabilir ve bunu yapmanın ek bir bonusu vardır: `cargo
test` komutunu çalıştırmak, belgelerinizdeki kod örneklerini test olarak çalıştıracaktır! Hiçbir şey
örnekli dokümantasyondan daha iyi değildir. Ancak hiçbir şey
dokümantasyonun
yazılmasından bu yana kod değiştiği için çalışmayan örneklerden daha kötü olamaz. Listing 14-1'deki `add_one`
fonksiyonunun dokümantasyonu ile `cargo test`i çalıştırırsak, test sonuçlarında
aşağıdaki gibi görünen bir bölüm göreceğiz:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

```metin
 Doc-tests my_crate

1 test çalıştırılıyor
test src/lib.rs - add_one (satır 5) ... tamam

test sonucu: tamam. 1 geçti; 0 başarısız; 0 yok sayıldı; 0 ölçüldü; 0 filtrelendi; 0.27s içinde bitti
```

Şimdi,
örneğindeki `assert_eq!` panik yapacak şekilde fonksiyonu veya örneği değiştirirsek ve `cargo test`i tekrar çalıştırırsak, doküman testlerinin
örnek ve kodun birbiriyle senkronize olmadığını yakaladığını göreceğiz!
#### Commenting Contained Items

The style of doc comment `//!` adds documentation to the item that *contains*
the comments rather than to the items *following* the comments. We typically use
these doc comments inside the crate root file (_src/lib.rs_ by convention) or
inside a module to document the crate or the module as a whole.

For example, to add documentation that describes the purpose of the `my_crate`
crate that contains the `add_one` function, we add documentation comments that
start with `//!` to the beginning of the _src/lib.rs_ file, as shown in Listing
14-2.

<Listing number="14-2" file-name="src/lib.rs" caption="Documentation for the `my_crate` crate as a whole">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

</Listing>

Notice there isn’t any code after the last line that begins with `//!`. Because
we started the comments with `//!` instead of `///`, we’re documenting the item
that contains this comment rather than an item that follows this comment. In
this case, that item is the _src/lib.rs_ file, which is the crate root. These
comments describe the entire crate.

When we run `cargo doc --open`, these comments will display on the front
page of the documentation for `my_crate` above the list of public items in the
crate, as shown in Figure 14-2.

<img alt="Rendered HTML documentation with a comment for the crate as a whole" src="img/trpl14-02.png" class="center" />

<span class="caption">Figure 14-2: Rendered documentation for `my_crate`,
including the comment describing the crate as a whole</span>

Documentation comments within items are useful for describing crates and
modules especially. Use them to explain the overall purpose of the container to
help your users understand the crate’s organization.

### `pub use` ile Kullanışlı Bir Genel API Dışa Aktarma

Genel API'nizin yapısı, bir
crate yayınlarken göz önünde bulundurulması gereken önemli bir husustur. Sandığınızı kullanan kişiler
bu yapıya sizden daha az aşinadır ve eğer sandığınız
geniş bir modül hiyerarşisine sahipse kullanmak istedikleri parçaları bulmakta zorluk çekebilirler.

Bölüm 7'de, `pub` anahtar sözcüğünü kullanarak öğeleri nasıl herkese açık hale getireceğimizi ve
`use` anahtar sözcüğüyle öğeleri bir kapsama nasıl dahil edeceğimizi ele aldık. Ancak, bir sandık geliştirirken size mantıklı gelen
yapısı kullanıcılarınız için çok
uygun olmayabilir. Yapılarınızı birden fazla düzey içeren bir
hiyerarşisi içinde düzenlemek isteyebilirsiniz, ancak bu durumda hiyerarşinin derinliklerinde tanımladığınız
türünü kullanmak isteyen kişiler
türünün var olduğunu bulmakta zorlanabilir. Ayrıca `use
my_crate::UsefulType;` yerine `use
my_crate::some_module::another_module::UsefulType;` girmek zorunda kalmaktan da rahatsız olabilirler.

İyi haber şu ki, yapı başkalarının başka bir kütüphaneden
kullanması için uygun değilse, iç organizasyonunuzu yeniden düzenlemeniz gerekmez:
bunun yerine, `pub use` kullanarak özel yapınızdan farklı bir
genel yapı oluşturmak için öğeleri yeniden dışa aktarabilirsiniz. *Yeniden dışa aktarma* bir konumdaki genel bir
öğesini alır ve sanki bunun yerine diğer konumda
tanımlanmış gibi başka bir konumda genel hale getirir.

Örneğin, sanatsal kavramları modellemek için `art` adında bir kütüphane oluşturduğumuzu varsayalım.
Bu kütüphanede iki modül bulunmaktadır: `PrimaryColor` ve `SecondaryColor` adlı iki enum
içeren bir `kinds` modülü ve Listing 14-3'te gösterildiği gibi `mix` adlı bir
işlevi içeren bir `utils` modülü.

<Listing number="14-3" file-name="src/lib.rs" caption="Öğeleri `kinds` ve `utils` modülleri halinde düzenlenmiş bir `art` kütüphanesi">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

</Listing>

Şekil 14-3, `cargo doc` tarafından oluşturulan bu sandık
için belgelerin ön sayfasının nasıl görüneceğini göstermektedir.

<img alt="`kinds` ve `utils` modüllerini listeleyen `art` sandığı için oluşturulmuş belgeler" src="img/trpl14-03.png" class="center" />

<span class="caption">Şekil 14-3: `kinds` ve `utils` modüllerini listeleyen `art`
belgesinin ön sayfası</span>

PrimaryColor` ve SecondaryColor` türlerinin
ön sayfasında listelenmediğine ve `mix` işlevinin de listelenmediğine dikkat edin. Bunları görmek için
adresinden `kinds` ve `utils` seçeneklerine tıklamamız gerekiyor.

Bu kütüphaneye bağlı olan başka bir sandık,
öğelerini `art` kapsamına getiren ve
şu anda tanımlanmış olan modül yapısını belirten `use` ifadelerine ihtiyaç duyacaktır. Liste 14-4,
`PrimaryColor` ve `mix` öğelerini `art` crate'inden kullanan bir crate örneğini göstermektedir.

<Listing number="14-4" file-name="src/main.rs" caption="İç yapısı dışa aktarılmış `art` sandığının öğelerini kullanan bir sandık">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

</Listing>

`Liste 14-4`teki `art` sandığını kullanan kodun yazarı,
adresinden `PrimaryColor`un `kinds` modülünde ve `mix`in
`utils` modülünde olduğunu anlamak zorunda kalmıştır. `Art` crate'in modül yapısı, onu kullananlardan çok `art` crate üzerinde çalışan
geliştiricileri ile ilgilidir. Dahili
yapısı,
`art` crate`inin nasıl kullanılacağını anlamaya çalışan biri için herhangi bir yararlı bilgi içermez, aksine kafa karışıklığına neden olur, çünkü onu kullanan
geliştiricileri nereye bakacaklarını bulmak ve
modül adlarını `use` deyimlerinde belirtmek zorundadır.

Dahili organizasyonu genel API'den kaldırmak için, Listing 14-3'teki
`art` crate kodunu değiştirerek, Listing 14-5'te gösterildiği gibi
öğelerini üst düzeyde yeniden dışa aktarmak için `pub use` deyimleri ekleyebiliriz.

<Liste numarası="14-5" file-name="src/lib.rs" caption="Öğeleri yeniden dışa aktarmak için `pub use` deyimleri ekleme">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

</Listing>

Bu sandık için `cargo doc` tarafından oluşturulan API belgeleri artık
adresini listeleyecek ve Şekil 14-4'te gösterildiği gibi ön sayfada yeniden ihraçları bağlayacak, böylece
`PrimaryColor` ve `SecondaryColor` türlerini ve `mix` işlevini bulmak daha kolay hale gelecektir.

<img alt="Ön sayfadaki yeniden ihraçlarla birlikte `art` sandığı için işlenmiş belgeler" src="img/trpl14-04.png" class="center" />

<span class="caption">Şekil 14-4: Yeniden dışa aktarımları listeleyen `art`
belgesinin ön sayfası</span>

`art` crate kullanıcıları Liste 14-4'te gösterildiği gibi Liste
14-3'teki dahili yapıyı görmeye ve kullanmaya devam edebilir ya da Liste 14-6'da gösterildiği gibi Liste 14-5'teki daha kullanışlı
yapısını kullanabilirler.

```rust,ignore

<Liste numarası="14-6" file-name="src/main.rs" caption="`art` sandığından yeniden ihraç edilen öğeleri kullanan bir program">

{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

</Listing>

İç içe geçmiş çok sayıda modülün bulunduğu durumlarda, en üst
seviyesindeki türleri `pub use` ile yeniden dışa aktarmak, sandığı kullanan
kişilerin deneyiminde önemli bir fark yaratabilir. Pub use`un bir diğer yaygın kullanımı, mevcut crate'teki bir bağımlılığın
tanımlarını yeniden ihraç ederek o crate'in
tanımlarını crate'inizin genel API'sinin bir parçası haline getirmektir.

Kullanışlı bir genel API yapısı oluşturmak bir bilimden çok bir sanattır ve
kullanıcılarınız için en iyi çalışan API'yi bulmak için yineleyebilirsiniz. pub
use` seçeneğini seçmek, crate'inizi dahili olarak nasıl yapılandıracağınız konusunda size esneklik sağlar ve
bu dahili yapıyı kullanıcılarınıza sunduğunuzdan ayırır. İç yapılarının
genel API'lerinden farklı olup olmadığını görmek için yüklediğiniz sandıkların kodlarından bazılarına
bakın.

### Bir Crates.io Hesabı Oluşturma

Herhangi bir sandık yayınlamadan önce
[crates.io](https://crates.io/)<!-- ignore --> adresinde bir hesap oluşturmanız ve bir API belirteci almanız gerekir. Bunu yapmak için
[crates.io](https://crates.io/)<!-- ignore --> adresindeki ana sayfayı ziyaret edin ve bir GitHub hesabı aracılığıyla
adresinde oturum açın. (GitHub hesabı şu anda bir gerekliliktir, ancak
sitesi gelecekte hesap oluşturmanın diğer yollarını destekleyebilir). oturum açtıktan sonra,
[https://crates.io/me/](https://crates.io/me/)<!-- ignore --> adresinden hesap ayarlarınızı ziyaret edin ve
API anahtarınızı alın. Ardından `cargo login` komutunu çalıştırın ve sorulduğunda API anahtarınızı aşağıdaki gibi yapıştırın:

``console
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```

Bu komut Cargo'yu API belirteciniz hakkında bilgilendirecek ve bunu yerel olarak
_~/.cargo/credentials.toml_ adresinde saklayacaktır. Bu belirtecin bir _secret_ olduğunu unutmayın:
başka kimseyle paylaşmayın. Herhangi bir nedenle herhangi biriyle paylaşırsanız,
bunu iptal etmeli ve [crates.io](https://crates.io/)<!-- adresinde yeni bir belirteç oluşturmalısınız.
-->.

### Yeni Bir Kasaya Meta Veri Ekleme

Diyelim ki yayınlamak istediğiniz bir sandık var. Yayınlamadan önce,
adresinden crate'in
_Cargo.toml_ dosyasının `[package]` bölümüne bazı meta veriler eklemeniz gerekir.

Sandığınızın benzersiz bir isme ihtiyacı olacaktır. Yerel olarak bir sandık üzerinde çalışırken,
bir sandığı istediğiniz gibi adlandırabilirsiniz. Ancak,
[crates.io](https://crates.io/)<!-- ignore --> adresindeki sandık adları ilk gelene,
ilk hizmet esasına göre tahsis edilir. Bir sandık adı alındıktan sonra, başka hiç kimse bu adla bir sandık
yayınlayamaz. Bir sandık yayınlamaya çalışmadan önce
adresinde kullanmak istediğiniz ismi arayın. İsim kullanılmışsa, başka bir isim bulmanız ve

 yayınlamak için yeni ismi kullanmak üzere `[package]` bölümü altındaki Cargo.toml_ dosyasındaki `name` alanını aşağıdaki gibi düzenlemeniz gerekecektir:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
```

Benzersiz bir isim seçmiş olsanız bile, bu noktada sandığı
yayınlamak için `cargo publish` çalıştırdığınızda, bir uyarı ve ardından bir hata alırsınız:

<!-- manual-regeneration
Oluşturulan pakette
 başka hiçbir değişiklik yapmadan kayıtsız bir adla yeni bir paket oluşturun, bu nedenle açıklama ve lisans alanları eksiktir.
cargo publish
sadece aşağıdaki ilgili satırları kopyalayın
-->

‘’``console
$ cargo publish
 crates.io dizini güncelleniyor
Uyarı: manifest'in açıklaması, lisansı, lisans dosyası, belgeleri, ana sayfası veya deposu yok.
Daha fazla bilgi için https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata adresine bakın.
--snip--
error: failed to publish to registry at https://crates.io

Nedeni:
 uzak sunucu bir hatayla yanıt verdi (durum 400 Kötü İstek): eksik veya boş meta veri alanları: açıklama, lisans. Bu alanların yapılandırılması hakkında daha fazla bilgi için lütfen https://doc.rust-lang.org/cargo/reference/manifest.html adresine bakın
```

Bu, bazı önemli bilgileri eksik bıraktığınız için bir hatayla sonuçlanır:
açıklaması ve lisans gereklidir, böylece insanlar kasanızın ne yaptığını
ve hangi koşullar altında kullanabileceklerini bilirler. Cargo.toml_ dosyasına
sadece bir ya da iki cümlelik bir açıklama ekleyin, çünkü bu açıklama
arama sonuçlarında sandığınızla birlikte görünecektir. Lisans alanı için bir _lisans tanımlayıcı değeri_ vermeniz gerekir.
Linux Foundation's Software Package Data Exchange (SPDX)][spdx] bu değer için kullanabileceğiniz
tanımlayıcılarını listeler. Örneğin,
adresinden sandığınızı MIT Lisansını kullanarak lisansladığınızı belirtmek için `MIT` tanımlayıcısını ekleyin:

<span class="filename">Dosya adı: Cargo.toml</span>

````toml
[package]
name = "guessing_game"
license = "MIT"
```

SPDX'te görünmeyen bir lisans kullanmak istiyorsanız,
bu lisansın metnini bir dosyaya yerleştirmeniz, dosyayı projenize dahil etmeniz ve ardından

 `license` anahtarını kullanmak yerine bu dosyanın adını belirtmek için `license-file` kullanmanız gerekir.

Projeniz için hangi lisansın uygun olduğuna ilişkin rehberlik bu kitabın kapsamı
dışındadır. Rust topluluğundaki birçok kişi projelerini
adresinde Rust ile aynı şekilde `MIT OR Apache-2.0` şeklinde ikili bir lisans kullanarak lisanslamaktadır. Bu uygulama
, projeniz için birden fazla lisansa sahip olmak için
adresini `OR` ile ayırarak birden fazla lisans tanımlayıcısı da belirtebileceğinizi göstermektedir.

Benzersiz bir ad, sürüm, açıklamanız ve bir lisans eklendiğinde, yayınlanmaya hazır bir proje için
_Cargo.toml_  dosyası aşağıdaki gibi görünebilir:

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

[Cargo’s documentation](https://doc.rust-lang.org/cargo/) başkalarının
kasanızı daha kolay keşfedebilmesini ve kullanabilmesini sağlamak için belirtebileceğiniz diğer
meta verilerini açıklar.

### Publishing to Crates.io

Artık bir hesap oluşturduğunuza, API token'ınızı kaydettiğinize, sandığınıza
için bir isim seçtiğinize ve gerekli meta verileri belirttiğinize göre yayınlamaya hazırsınız!
Bir sandık yayınlamak, başkalarının kullanması için
[crates.io](https://crates.io/)<!-- ignore --> adresine belirli bir sürümü yükler.

Dikkatli olun, çünkü bir yayınlama _kalıcıdır_. Sürüm asla
üzerine yazılamaz ve kod belirli durumlar dışında silinemez.
Crates.io'nun ana hedeflerinden biri, kalıcı bir kod arşivi olarak hareket etmektir, böylece
adresindeki sandıklara bağlı olan tüm projelerin
[crates.io](https://crates.io/)<!-- ignore --> çalışmaya devam edecektir. sürüm silmelerine izin vermek bu hedefi gerçekleştirmeyi imkansız hale getirecektir. Ancak,
yayınlayabileceğiniz sandık sürümlerinin sayısında bir sınırlama yoktur.

`cargo publish`'ı gene çalışıtırın şimdi başarılı olduğunu göreceksiniz:

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
    Packaged 6 files, 1.2KiB (895.0B compressed)
   Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
   Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
   Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
    Uploaded guessing_game v0.1.0 to registry `crates-io`
note: waiting for `guessing_game v0.1.0` to be available at registry
`crates-io`.
You may press ctrl-c to skip waiting; the crate should be available shortly.
   Published guessing_game v0.1.0 at registry `crates-io`
```

Congratulations! You’ve now shared your code with the Rust community, and
anyone can easily add your crate as a dependency of their project.

### Publishing a New Version of an Existing Crate

crate'inizde değişiklikler yaptığınızda ve yeni bir sürüm yayınlamaya hazır olduğunuzda,
_Cargo.toml_ dosyanızda belirtilen `version` değerini değiştirin ve
yeniden yayınlayın. Yaptığınız değişikliklerin türüne göre
uygun bir sonraki sürüm numarasının ne olduğuna karar vermek için [Anlamsal Sürümleme Kuralları][semver] kullanın yaptığınız değişiklik türlerine göre
uygun bir sonraki sürüm numarasının ne olduğuna karar vermek için.
Ardından yeni sürümü yüklemek için `cargo publish` çalıştırın.

<!-- Old link, do not remove -->

<a id="removing-versions-from-cratesio-with-cargo-yank"></a>

### Crates.io'dan `cargo yank` ile Sürümlerin Kullanımdan Kaldırılması

Bir sandığın önceki sürümlerini kaldıramasanız da,
gelecekteki projelerin bunları yeni bir bağımlılık olarak eklemesini önleyebilirsiniz. Bu, bir
crate sürümü bir nedenden dolayı bozulduğunda kullanışlıdır. Bu gibi durumlarda, Cargo
bir sandık sürümünü sıralamayı destekler.

Bir sürümü _Yanking_ yapmak yeni projelerin o sürüme bağlı olmasını engellerken
ona bağlı olan tüm mevcut projelerin devam etmesine izin verir. Esasen, bir
yankı, _Cargo.lock_ içeren tüm projelerin bozulmayacağı ve gelecekte üretilen
_Cargo.lock_ dosyalarının yankı sürümünü kullanmayacağı anlamına gelir.

Bir sandığın bir sürümünü çekmek için,
daha önce yayınladığınız sandığın dizininde, `cargo yank` komutunu çalıştırın ve hangi sürümü çekmek istediğinizi belirtin
yank. Örneğin, `guessing_game` adlı bir sandık yayınladıysak ve
1.0.1 sürümünü çekmek istiyorsak, `guessing_game` için proje dizininde
çalıştırırız:
<!-- manual-regeneration:
cargo yank carol-test --version 2.1.0
cargo yank carol-test --version 2.1.0 --undo
-->

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```

Komutuna `--undo` ekleyerek, bir yank'ı geri alabilir ve
projelerinin tekrar bir sürüme bağlı olarak başlamasına izin verebilirsiniz:

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1
```

Bir yank _herhangi bir kodu silmez_. Örneğin, yanlışlıkla
yüklenen gizli dizileri silemez. Böyle bir durumda, bu sırları derhal sıfırlamanız gerekir.
[spdx]: https://spdx.org/licenses/
[semver]: https://semver.org/
