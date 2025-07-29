## Modül Ağacındaki Bir Öğeye Başvurma Yolları

Rust'a bir modül ağacında bir öğeyi nerede bulacağını göstermek için, aynı
Bir dosya sisteminde gezinirken bir yol kullanma şeklimiz. Bir fonksiyonu çağırmak için
yolunu bilmek.

Bir yol iki şekilde olabilir:

- Bir _mutlak yol_, bir sandık kökünden başlayan tam yoldur; kod için
  için mutlak yol sandık adı ile başlar ve
  kodu geçerli `crate`ten başlar, `crate` değişmezi ile başlar.
- Bir _göreceli yol_ geçerli modülden başlar ve `self`, `super` veya
  geçerli modülde bir tanımlayıcı.

Hem mutlak hem de göreli yolların ardından bir veya daha fazla tanımlayıcı gelir
çift iki nokta üst üste (`::`) ile ayrılmıştır.

Liste 7-1'e dönersek, `add_to_waitlist` fonksiyonunu çağırmak istediğimizi varsayalım.
Bu, `add_to_waitlist` fonksiyonunun yolunun ne olduğunu sormakla aynı şeydir.
Liste 7-3, bazı modüller ve fonksiyonlarla birlikte Liste 7-1'i içerir
kaldırıldı.

Yeni bir fonksiyondan `add_to_waitlist` fonksiyonunu çağırmanın iki yolunu göstereceğiz,
sandık kökünde tanımlanan `eat_at_restaurant`. Bu yollar doğrudur, ancak
Bu örneğin derlenmesini engelleyecek başka bir sorun daha var
olduğu gibi. Nedenini birazdan açıklayacağız.

`Eat_at_restaurant` fonksiyonu kütüphane crate'imizin genel API'sinin bir parçasıdır, bu yüzden
bunu `pub` anahtar sözcüğü ile işaretleriz. İçinde ["Yolları `pub` ile Göstermek Keyword"][pub]<!-- ignore --> bölümünde, `pub' hakkında daha fazla ayrıntıya gireceğiz.

<Listing number="7-3" file-name="src/lib.rs" caption="Calling the `add_to_waitlist` function using absolute and relative paths">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

</Listing>

İlk kez `eat_at_restaurant` içinde `add_to_waitlist` fonksiyonunu çağırıyoruz,
mutlak bir yol kullanırız. add_to_waitlist` işlevi aynı yerde tanımlanmıştır
crate` anahtar sözcüğünü `eat_at_restaurant` olarak kullanabileceğimiz anlamına gelir.
mutlak bir yol başlatır. Daha sonra birbirini izleyen modüllerin her birini
add_to_waitlist`e doğru yol alırız. Aynı dosya sistemine sahip bir dosya sistemi hayal edebilirsiniz
yapısı: `/front_of_house/hosting/add_to_waitlist` yolunu şu şekilde belirtiriz
add_to_waitlist` programını çalıştırın; `crate` adını kullanarak
crate root, kabuğunuzda dosya sistemi kökünden başlamak için `/` kullanmak gibidir.

İkinci kez `eat_at_restaurant` içinde `add_to_waitlist` çağırdığımızda, bir
göreli yol. Yol, modülün adı olan `front_of_house` ile başlar
modül ağacında `eat_at_restaurant` ile aynı seviyede tanımlanmıştır. Burada
dosya sistemi eşdeğeri yolu kullanmak olacaktır
`front_of_house/hosting/add_to_waitlist`. Bir modül adı ile başlamak şu anlama gelir
yolun göreceli olduğunu gösterir.

Göreceli veya mutlak bir yol kullanmayı seçmek, vereceğiniz bir karardır
Projenize bağlı olarak, taşınma olasılığınızın daha yüksek olup olmadığına bağlıdır.
kodunu kullanan koddan ayrı olarak veya onunla birlikte öğe tanım kodu
öğesi. Örneğin, `front_of_house` modülünü taşıdıysak ve
“restoranda_yemek” işlevini “müşteri_deneyimi” adlı bir modüle yerleştirirsek
'add_to_waitlist' için mutlak yolu güncellemeniz gerekir, ancak göreli yol
hala geçerli olacaktır. Ancak, `eat_at_restaurant` işlevini taşırsak
ayrı ayrı `dining` adlı bir modüle, mutlak yol
add_to_waitlist` çağrısı aynı kalacaktır, ancak göreli yolun
güncellenecektir. Genel olarak tercihimiz mutlak yolları belirtmektir çünkü
daha çok kod tanımlarını ve öğe çağrılarını aşağıdakilerden bağımsız olarak taşımak isteyeceğiz
Birbirlerini.

Liste 7-3'ü derlemeye çalışalım ve neden henüz derlenmediğini öğrenelim! Liste 7-3
Aldığımız hatalar Liste 7-4'te gösterilmektedir.

<Listing number="7-4" caption="Compiler errors from building the code in Listing 7-3">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

</Listing>

Hata mesajları `hosting` modülünün özel olduğunu söylüyor. Başka bir deyişle, biz
'hosting' modülü ve ‘add_to_waitlist’ için doğru yollara sahip
fonksiyonunu kullanmamıza izin verir, ancak Rust bunları kullanmamıza izin vermez çünkü
özel bölümler. Rust'ta tüm öğeler (fonksiyonlar, metotlar, yapılar, enumlar,
modüller ve sabitler) varsayılan olarak üst modüllere özeldir. Eğer isterseniz
Bir fonksiyon veya yapı gibi bir öğeyi özel yapmak için onu bir modülün içine koyarsınız.

Bir üst modüldeki öğeler alt modüllerdeki özel öğeleri kullanamaz, ancak
alt modüllerdeki öğeler ata modüllerindeki öğeleri kullanabilir. Bu
çünkü alt modüller uygulama ayrıntılarını sarar ve gizler, ancak alt
modülleri tanımlandıkları bağlamı görebilir. Bizim ile devam etmek için
Metafor, gizlilik kurallarını bir şirketin arka ofisi gibi düşünün.
restoran: orada olup bitenler restoran müşterilerine özeldir, ancak
ofis müdürleri işlettikleri restoranda her şeyi görebiliyor ve yapabiliyor.

Rust, modül sisteminin bu şekilde çalışmasını seçti, böylece iç mekanı gizlemek
uygulama ayrıntıları varsayılandır. Bu şekilde, uygulamanın hangi kısımlarının
İç kodu, dış kodu bozmadan değiştirebilirsiniz. Bununla birlikte, Rust
alt modüllerin kodunun iç kısımlarını dış ataya gösterme seçeneği
modüllerinde `pub` anahtar sözcüğünü kullanarak bir öğeyi herkese açık hale getirebilirsiniz.

### Yolları `pub` Anahtar Sözcüğü ile Açığa Çıkarma

Liste 7-4'teki `hosting` modülünün şu şekilde olduğunu söyleyen hataya geri dönelim
özel. Ana modüldeki `eat_at_restaurant` fonksiyonunun aşağıdaki özelliklere sahip olmasını istiyoruz
alt modüldeki `add_to_waitlist` fonksiyonuna erişim, bu nedenle
Listeleme 7-5'te gösterildiği gibi `pub` anahtar sözcüğüyle `hosting` modülü.

<Listing number="7-5" file-name="src/lib.rs" caption="Declaring the `hosting` module as `pub` to use it from `eat_at_restaurant`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

</Listing>

Ne yazık ki, Liste 7-5'teki kod hala derleyici hatalarıyla sonuçlanmaktadır, çünkü
Liste 7-6'da gösterilmiştir.

<Listing number="7-6" caption="Compiler errors from building the code in Listing 7-5">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

</Listing>

Ne oldu? `mod hosting` anahtar sözcüğünün önüne `pub` anahtar sözcüğünü eklemek
modülü herkese açık. Bu değişiklikle, eğer `front_of_house` modülüne erişebiliyorsak, şunları yapabiliriz
`hosting`e erişebilir. Ancak `hosting`in _içeriği_ hala özeldir; bu da
public modülü içeriğini herkese açık hale getirmez. Bir modül üzerindeki `pub` anahtar sözcüğü
yalnızca ata modüllerindeki kodun ona başvurmasına izin verir, iç koduna erişmesine izin vermez.
Modüller birer konteyner olduğu için, sadece
modülünü halka açık hale getirmemiz gerekir; daha ileri gitmeli ve bir veya daha fazla
modül içindeki öğeler de geneldir.

Liste 7-6'daki hatalar `add_to_waitlist' fonksiyonunun özel olduğunu söylemektedir.
Gizlilik kuralları yapılar, enumlar, fonksiyonlar ve metotlar için geçerli olduğu gibi
modülleri.

Ayrıca `add_to_waitlist` fonksiyonunu `pub` ekleyerek public hale getirelim
Listeleme 7-7'de olduğu gibi, tanımından önce anahtar sözcüğü.

<Listing number="7-7" file-name="src/lib.rs" caption="Adding the `pub` keyword to `mod hosting` and `fn add_to_waitlist` lets us call the function from `eat_at_restaurant`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

</Listing>

Şimdi kod derlenecek! Neden `pub` anahtar sözcüğünü eklemenin kullanmamıza izin verdiğini görmek için
gizlilik kurallarına göre `eat_at_restaurant` içindeki bu yollara bakalım
mutlak ve göreceli yollarda.

Mutlak yolda, crate modülümüzün kökü olan `crate` ile başlarız
ağaç. `Front_of_house` modülü sandık kökünde tanımlanmıştır. Bir yandan
`front_of_house` herkese açık değildir, çünkü `eat_at_restaurant` işlevi
`front_of_house` ile aynı modülde tanımlanmıştır (yani, `eat_at_restaurant`
ve `front_of_house` kardeştir), `front_of_house` öğesine şuradan başvurabiliriz
`eat_at_restaurant`. Sırada `pub` ile işaretlenmiş `hosting` modülü var. Yapabiliriz
'in üst modülüne erişebilir, böylece `hosting`e erişebiliriz. Son olarak
`add_to_waitlist` fonksiyonu `pub` ile işaretlenmiştir ve ebeveynine erişebiliriz
modülünü kullanıyorsanız, bu fonksiyon çağrısı çalışır!

Göreceli yolda mantık, mutlak yol ile aynıdır, ancak
ilk adım: sandık kökünden başlamak yerine, yol
`front_of_house`. `Front_of_house` modülü aynı modül içinde tanımlanmıştır
olarak `eat_at_restaurant`, bu nedenle modülden başlayan göreli yol
`eat_at_restaurant` tanımlandığında çalışır. Sonra, çünkü `hosting` ve
add_to_waitlist` `pub` ile işaretlenir, yolun geri kalanı çalışır ve bu
fonksiyon çağrısı geçerlidir!

Diğer projelerin kodunuzu kullanabilmesi için kütüphane sandığınızı paylaşmayı planlıyorsanız,
Genel API'niz, sandığınızın kullanıcılarıyla yaptığınız sözleşmedir.
kodunuzla etkileşime girebilirler. Yönetimle ilgili birçok husus vardır
İnsanların API'nize güvenmesini kolaylaştırmak için genel API'nizde yapılan değişiklikler
sandık. Bu hususlar bu kitabın kapsamı dışındadır; eğer
Bu konuyla ilgilenenler [The Rust API Guidelines][api-guidelines] bölümüne bakabilirler.

> #### İkili ve Kitaplık İçeren Paketler için En İyi Uygulamalar
>
> Bir paketin hem _src/main.rs_ ikili sandığını içerebileceğinden bahsetmiştik
> kökünün yanı sıra _src/lib.rs_ kütüphane sandık köküne sahip olacak ve her iki sandık da
> varsayılan olarak paket adı. Tipik olarak, bu modele sahip paketler
Hem bir kütüphane hem de bir ikili sandık içeren > sadece yeterli koda sahip olacaktır.
> kütüphanede tanımlanan kodu çağıran bir yürütülebilir dosyayı başlatmak için binary crate
> sandık. Bu, diğer projelerin en çok işlevsellikten yararlanmasını sağlar.
kütüphane sandığının kodu paylaşılabildiği için > paket sağlar.
>
> Modül ağacı _src/lib.rs_ içinde tanımlanmalıdır. Daha sonra, herhangi bir genel öğe
> yolları paketin adıyla başlatarak ikili sandıkta kullanılabilir.
> İkili sandık, kütüphane sandığının bir kullanıcısı haline gelir, tıpkı tamamen
> harici sandık kütüphane sandığını kullanacaktır: yalnızca genel API'yi kullanabilir.
> Bu, iyi bir API tasarlamanıza yardımcı olur; sadece yazar siz değilsiniz, aynı zamanda bir
> müşteri!
>
> Bölüm 12][ch12]<!-- ignore -->'de bu organizasyonu göstereceğiz
> hem ikili bir sandık içerecek bir komut satırı programı ile pratik yapın
> ve bir kütüphane sandığı.

### Göreli Yolları `super` ile Başlatma

yerine üst modülde başlayan göreli yollar oluşturabiliriz.
'nin başında `super` kullanarak geçerli modülü veya sandık kökünü
path. Bu, bir dosya sistemi yolunu `..` sözdizimiyle başlatmak gibidir, yani
ana dizine gitmek için. `Super` kullanımı bir öğeye referans vermemizi sağlar
ana modülde olduğunu biliyoruz, bu da modülün yeniden düzenlenmesini sağlayabilir
Modül ebeveynle yakından ilişkili olduğunda ağaç daha kolay ancak ebeveyn
bir gün modül ağacında başka bir yere taşınabilir.

Liste 7-8'deki kodu düşünün; bu kod bir şefin
yanlış bir siparişi düzeltir ve müşteriye bizzat getirir. Bu
modülünde tanımlanan `fix_incorrect_order` fonksiyonu, `back_of_house` modülünü çağırır.
yolunu belirterek üst modülde tanımlanan `deliver_order` işlevini
`süper` ile başlayan `deliver_order`.

<Listing number="7-8" file-name="src/lib.rs" caption="Calling a function using a relative path starting with `super`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

</Listing>

`fix_incorrect_order` işlevi `back_of_house` modülündedir, bu nedenle şunları yapabiliriz
`back_of_house`un ana modülüne gitmek için `super` kullanın, bu durumda
kök olan `crate`. Oradan `deliver_order`ı ararız ve buluruz.
Başarılar! Bizce `back_of_house` modülü ve `deliver_order` fonksiyonu
birbirleriyle aynı ilişki içinde kalmaları ve taşınmaları muhtemeldir
Sandığın modül ağacını yeniden düzenlemeye karar verirsek birlikte. Bu nedenle, biz
'süper'i kullandık, böylece gelecekte kodu güncellemek için daha az yerimiz olacak eğer bu
kodu farklı bir modüle taşınır.

### Yapıları ve Enumları Herkese Açık Hale Getirme

Yapıları ve enumları public olarak belirtmek için `pub` da kullanabiliriz, ancak
`struct ve enum`larla `pub` kullanımına ilişkin birkaç ekstra ayrıntı. Eğer `pub` kullanırsak
bir struct tanımından önce, struct'ı herkese açık hale getiririz, ancak struct'ın alanları
hala özel olacaktır. Her bir alanı duruma göre herkese açık hale getirebilir veya getirmeyebiliriz
temel alır. Liste 7-9'da, public bir `back_of_house::Breakfast` yapısı tanımladık
genel bir `toast` alanı ancak özel bir `seasonal_fruit` alanı ile. Bu modeller
müşterinin istediği ekmek türünü seçebildiği bir restoranda olduğu gibi
yemekle birlikte gelir, ancak yemeğe hangi meyvenin eşlik edeceğine şef karar verir.
mevsiminde ve stokta ne olduğuna bağlı. Mevcut meyveler hızla değişir, bu nedenle
Müşteriler meyve seçemiyor, hatta hangi meyveyi alacaklarını bile göremiyorlar.

<Listing number="7-9" file-name="src/lib.rs" caption="A struct with some public fields and some private fields">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

</Listing>

Çünkü `back_of_house::Breakfast` yapısındaki `toast` alanı herkese açıktır,
`eat_at_restaurant` içinde `toast` alanına nokta kullanarak yazabilir ve okuyabiliriz
notasyonu. 'seasonal_fruit' alanını şu şekilde kullanamayacağımıza dikkat edin
`eat_at_restaurant`, çünkü `seasonal_fruit` özeldir. Eklemeyi kaldırmayı deneyin
satırında `seasonal_fruit` alan değerini değiştirerek nasıl bir hata aldığınızı görün!

Ayrıca, `back_of_house::Breakfast` özel bir alana sahip olduğu için
yapısının, bir
örneğini (biz burada ona `summer` adını verdik) oluşturduk. Eğer `Kahvaltı`
böyle bir fonksiyona sahip olsaydı, `Breakfast` örneğini
'restorantta_yemek' çünkü private'ın değerini ayarlayamadık
eat_at_restaurant` içindeki `seasonal_fruit` alanı.

Buna karşılık, bir enum'u public yaparsak, tüm varyantları public olur. Biz
Liste 7-10'da gösterildiği gibi, sadece `enum` anahtar sözcüğünden önce `pub` sözcüğüne ihtiyaç duyar.

<Listing number="7-10" file-name="src/lib.rs" caption="Designating an enum as public makes all its variants public.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

</Listing>

`Appetizer` enumunu public yaptığımız için, `Soup` ve `Salad` enumlarını kullanabiliriz.
eat_at_restaurant` içindeki varyantlar.

Varyantları herkese açık olmadığı sürece enumlar pek kullanışlı değildir; bu can sıkıcı olurdu
tüm enum varyantlarını her durumda `pub` ile açıklamak zorunda kalmamak için varsayılan
enum varyantları için public olması gerekir. Yapılar, genellikle kendi yapıları olmadan
alanları herkese açıktır, bu nedenle struct alanları her şeyin genel kuralına uyar
ile açıklanmadığı sürece varsayılan olarak özeldir.

Henüz ele almadığımız `pub` ile ilgili bir durum daha var ve o da
Son modül sistemi özelliğimiz: `use` anahtar sözcüğü. Biz `use` anahtar sözcüğünü tek başına ele alacağız
ve ardından `pub` ve `use` öğelerini nasıl birleştireceğimizi göstereceğiz.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md#yolları-pub-anahtar-sözcüğü-ile-açığa-çıkarma
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.md
