## Kapsam ve Gizliliği Kontrol Etmek için Modül Tanımlama

Bu bölümde modüller ve modül sisteminin diğer parçaları hakkında konuşacağız,
yani _paths_, öğeleri adlandırmanıza izin verir; `use` anahtar sözcüğü bir
yolunu kapsam içine almak; ve öğeleri herkese açık hale getirmek için `pub` anahtar sözcüğü. Ayrıca şunları da tartışacağız
as` anahtar sözcüğü, harici paketler ve glob operatörü.

### Modüller Hile Sayfası

Modüllerin ve yolların ayrıntılarına geçmeden önce, burada hızlı bir
modüller, yollar, `use` anahtar sözcüğü ve `pub` anahtar sözcüğünün nasıl çalıştığı hakkında referans
ve çoğu geliştiricinin kodlarını nasıl düzenlediğini açıklayacağız. Biz gideceğiz.
Bu bölüm boyunca bu kuralların her birinin örneklerini inceleyeceğiz, ancak bu bir
modüllerin nasıl çalıştığını hatırlatmak için harika bir yer.

- **Kasa kökünden başlayın**: Bir crate derlenirken, derleyici ilk olarak
  sandık kök dosyasına bakar (genellikle bir kütüphane sandığı için _src/lib.rs_ veya
  ikili bir sandık için _src/main.rs_) kodun derlenmesi için.
- **Modülleri bildirme**: Crate kök dosyasında yeni modüller bildirebilirsiniz;
  diyelim ki `mod garden;` ile bir “garden” modülü bildirdiniz. Derleyici bakacaktır
  bu yerlerde modülün kodu için:
  - Satır içi, `mod`dan sonra gelen noktalı virgülün yerini alan küme parantezleri içinde
    bahçe`
  - _src/garden.rs_ dosyasında
  - _src/garden/mod.rs_ dosyasında
- **Alt modüllerin bildirilmesi**: Crate kökü dışındaki herhangi bir dosyada
  alt modülleri bildirir. Örneğin, `mod vegetables;` ifadesini
  _src/garden.rs_. Derleyici, alt modülün kodunu _rc/garden.rs_ içinde arayacaktır.
  Bu yerlerde üst modül için adlandırılmış dizin:
  - Satır içi, `mod vegetables` i doğrudan takip eden, bunun yerine küme parantezleri içinde
    noktalı virgülün
  - _src/garden/vegetables.rs_ dosyasında
  - _src/garden/vegetables/mod.rs_ dosyasında
- **Modüllerdeki koda giden yollar**: Bir modül sandığınızın bir parçası olduğunda şunları yapabilirsiniz
  aynı sandıktaki başka herhangi bir yerden o modüldeki koda atıfta bulunduğu sürece
  gizlilik kurallarının izin verdiği şekilde, kodun yolunu kullanarak. Örneğin, bir
  Bahçe sebzeleri modülündeki `Asparagus` türü şu adreste bulunacaktır
  `crate::garden::vegetables::Asparagus`.
- **Özel ve genel**: Bir modül içindeki kod ana modülden özeldir
  modülleri varsayılan olarak. için

- **Modüllerdeki koda giden yollar**: Bir modül sandığınızın bir parçası olduğunda şunları yapabilirsiniz
  aynı sandıktaki başka herhangi bir yerden o modüldeki koda atıfta bulunduğu sürece
  gizlilik kurallarının izin verdiği şekilde, kodun yolunu kullanarak. Örneğin, bir
  Bahçe sebzeleri modülündeki `Asparagus` türü şu adreste bulunur
  `crate::garden::vegetables::Asparagus`.
- **Özel ve genel**: Bir modül içindeki kod ana modülden özeldir
  modülleri varsayılan olarak. Bir modülü public yapmak için `pub mod` ile deklare edin
  yerine `mod` kullanın. Genel bir modül içindeki öğeleri de genel yapmak için
  bildirimlerinden önce `pub`.
- `use` anahtar sözcüğü**: Bir kapsam içinde, `use` anahtar sözcüğü aşağıdakiler için kısayollar oluşturur
  uzun yolların tekrarını azaltmak için öğeler. Başvurulabilecek herhangi bir kapsamda
  `crate::garden::vegetables::Asparagus` ile bir kısayol oluşturabilirsiniz.
  `crate::garden::vegetables::Asparagus;` ve o andan itibaren sadece
  kapsamındaki bu türü kullanmak için `Asparagus` yazın.

Burada, bu kuralları gösteren `backyard` adında ikili bir sandık oluşturuyoruz.
`Crate`in yine `backyard` olarak adlandırılan dizini şu dosyaları içerir
dizinler:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

Bu durumda crate kök dosyası _src/main.rs_'dir ve şunları içerir:

<Listing file-name="src/main.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

</Listing>

Pub mod garden;` satırı derleyiciye, bulduğu kodu
_src/garden.rs_, yani:

<Listing file-name="src/garden.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

</Listing>

Burada, `pub mod vegetables;` _src/garden/vegetables.rs_ dosyasındaki kodun
de dahildir. Bu kod:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

Şimdi bu kuralların ayrıntılarına girelim ve bunları uygulamalı olarak gösterelim!

### Modüllerde İlgili Kodları Gruplama

Modüller_ okunabilirlik ve kolay yeniden kullanım için bir sandık içindeki kodu düzenlememizi sağlar.
Modüller ayrıca öğelerin _gizliliğini_ kontrol etmemizi sağlar, çünkü bir
modülü varsayılan olarak özeldir. Özel öğeler dahili uygulama ayrıntılarıdır
dışarıda kullanım için mevcut değildir. Modüller ve öğeler yapmayı seçebiliriz
içlerinde halka açıktır, bu da onları harici kodun kullanmasına ve bağımlı olmasına izin verecek şekilde açığa çıkarır
onlara.

Örnek olarak, bir kütüphane sandığının işlevselliğini sağlayan bir kütüphane sandığı yazalım
restoran. Fonksiyonların imzalarını tanımlayacağız ancak gövdelerini bırakacağız
yerine kodun organizasyonuna konsantre olmak için boş
bir restoranın uygulanması.

Restoran endüstrisinde, bir restoranın bazı bölümleri şu şekilde adlandırılır
Evin önü_ ve diğerleri de evin arkası_ olarak adlandırılır. Evin ön kısmı
müşterilerdir; bu, ana bilgisayarların müşterileri nereye oturttuğunu, sunucuların
siparişler ve ödemeler, barmenler ise içecekleri hazırlar. Evin arka tarafı
şefler ve aşçılar mutfakta çalışır, bulaşıkçılar temizlik yapar ve yöneticiler
idari işler.

Kasamızı bu şekilde yapılandırmak için, işlevlerini iç içe geçmiş olarak düzenleyebiliriz
modüller. cargo new komutunu çalıştırarak `restaurant` adında yeni bir kütüphane oluşturun
restaurant --lib`. Ardından Liste 7-1'deki kodu _src/lib.rs_ dosyasına girerek
bazı modülleri ve işlev imzalarını tanımlayın; bu kod evin ön kısmıdır
Bölüm.

<Listing number="7-1" file-name="src/lib.rs" caption="A `front_of_house` module containing other modules that then contain functions">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

</Listing>

Bir modülü `mod` anahtar sözcüğü ve ardından modülün adı ile tanımlarız
(bu durumda, `front_of_house`). Modülün gövdesi daha sonra kıvrımlı
parantezler. Modüllerin içine, bu durumda olduğu gibi başka modüller yerleştirebiliriz
modülleri `hosting` ve `serving`. Modüller ayrıca diğer modüller için tanımlar da tutabilir.
structs, enums, constants, traits gibi öğeler ve Liste 7-1'de olduğu gibi,
fonksiyonlar.

Modülleri kullanarak, ilgili tanımları bir arada gruplayabilir ve nedenini adlandırabiliriz
ilişkilidirler. Bu kodu kullanan programcılar, kodda aşağıdakilere göre gezinebilirler
tüm tanımları okumak zorunda kalmak yerine gruplar, daha kolay hale getirir
kendileriyle ilgili tanımları bulmak için. Yeni işlevler ekleyen programcılar
bu kodun, programı düzenli tutmak için kodu nereye yerleştireceğini bilecektir.

Daha önce, _src/main.rs_ ve _src/lib.rs_ dosyalarının crate olarak adlandırıldığından bahsetmiştik
kökler. İsimlerinin nedeni, bu ikisinden herhangi birinin içeriğinin
dosyaları, crate'in modül yapısının kökünde `crate` adlı bir modül oluşturur,
_modül ağacı_ olarak bilinir.

Liste 7-2, Liste 7-1'deki yapı için modül ağacını göstermektedir.

<Listing number="7-2" caption="The module tree for the code in Listing 7-1">

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

</Listing>

Bu ağaç, bazı modüllerin diğer modüllerin içinde nasıl yuvalandığını gösterir; örneğin,
`hosting` `front_of_house` içinde yuvalanır. Ağaç ayrıca bazı modüllerin
_siblings_, yani aynı modülde tanımlanmışlardır; `hosting` ve
'servis' `front_of_house' içinde tanımlanmış kardeşlerdir. Eğer modül A ise
B modülünün içinde yer alıyorsa, A modülünün B modülünün _çocuğu_ olduğunu söyleriz ve
B modülünün A modülünün _ebeveyni_ olduğuna dikkat edin.
'crate' adlı örtük modülün altında köklenir.

Modül ağacı size dosya sisteminizdeki dizin ağacını hatırlatabilir.
bilgisayar; bu çok yerinde bir karşılaştırma! Tıpkı bir dosya sistemindeki dizinler gibi,
kodunuzu düzenlemek için modüller kullanırsınız. Ve tıpkı bir dizindeki dosyalar gibi
modüllerimizi bulmak için bir yola ihtiyacımız var.