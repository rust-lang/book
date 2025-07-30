## Modülleri Farklı Dosyalara Ayırma

Şimdiye kadar, bu bölümdeki tüm örneklerde bir dosyada birden fazla modül tanımlanmıştı.
Modüller büyüdükçe, kodda gezinmeyi kolaylaştırmak için tanımlarını ayrı bir
dosyaya taşımak isteyebilirsiniz.

Örneğin, Listing 7-17'deki birden fazla
restoran modülü içeren koddan başlayalım. Tüm modülleri crate kök dosyasında tanımlamak yerine, modülleri dosyalara ayıracağız.
Bu durumda, crate kök dosyası
_src/lib.rs_'dir, ancak bu prosedür, crate kök dosyası
_src/main.rs_ olan ikili crate'lerde de işe yarar.

İlk olarak, `front_of_house` modülünü kendi dosyasına ayıracağız. `front_of_house` modülünün küme parantezleri içindeki kodu kaldırın ve sadece
`mod front_of_house;` bildirimini bırakın, böylece _src/lib.rs_ dosyası Listing 7-21'de gösterilen kodu içersin.
Listing 7-22'deki _src/front_of_house.rs_ dosyasını oluşturana kadar bunun derlenmeyeceğini unutmayın.
Listing 7-22.
`front_of_house` modülünü kendi dosyasına ayırmak için

<Listing number="7-21" file-name="src/lib.rs" caption="Declaring the `front_of_house` module whose body will be in *src/front_of_house.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

</Listing>

Ardından, küme parantezleri içindeki kodu, Listing 7-22'de gösterildiği gibi
_src/front_of_house.rs_ adlı yeni bir dosyaya yerleştirin. Derleyici,
crate kökünde `front_of_house` adlı modül bildirimi ile karşılaştığı için
bu dosyaya bakacağını bilir.

<Listing number="7-22" file-name="src/front_of_house.rs" caption="Definitions inside the `front_of_house` module in *src/front_of_house.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

</Listing>

Modül ağacınızda bir dosyayı `mod` bildirimi kullanarak yalnızca bir kez yüklemeniz gerektiğini unutmayın.
Derleyici, dosyanın projenin bir parçası olduğunu (ve `mod`
ifadesini koyduğunuz yere göre modül ağacında kodun nerede bulunduğunu) öğrendikten sonra, projenizdeki diğer dosyalar,
[“Modül Ağacındaki Bir Öğeye Başvurmak için Yollar”][paths]<!-- ignore --> bölümünde anlatıldığı gibi, bildirildiği yere giden bir yol kullanarak yüklenen dosyanın koduna. Diğer bir deyişle,
`mod`, diğer programlama dillerinde gördüğünüz türden bir “include” işlemi
değildir.

Ardından, `hosting` modülünü kendi dosyasına çıkaracağız. Bu işlem biraz
farklıdır, çünkü `hosting`, kök modülün değil, `front_of_house` modülünün alt modülüdür.
`hosting` dosyasını, modül ağacındaki atalarının adını taşıyan yeni bir dizine yerleştireceğiz, bu durumda _src/front_of_house_.
`hosting` modülünü taşımaya başlamak için, _src/front_of_house.rs_ dosyasını

`hosting` modülünü taşımaya başlamak için, _src/front_of_house.rs_ dosyasını yalnızca
`hosting` modülünün bildirimini içerecek şekilde değiştiriyoruz:

<Listing file-name="src/front_of_house.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

</Listing>

Ardından, _src/front_of_house_ dizinini ve _hosting.rs_ dosyasını oluşturarak
`hosting` modülünde yapılan tanımları bu dosyaya ekliyoruz:

<Listing file-name="src/front_of_house/hosting.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

</Listing>

Bunun yerine _hosting.rs_ dosyasını _src_ dizinine koyarsak, derleyici
_hosting.rs_ kodunun crate kökünde bildirilen bir `hosting` modülünde olmasını
bekler ve `front_of_house` modülünün alt modülü olarak bildirilmesini beklemez.
Derleyicinin hangi modüllerin kodunu hangi dosyalar için kontrol edeceği kuralları,
dizinlerin ve dosyaların modül ağacına daha yakından uyması anlamına gelir.

> ### Alternatif Dosya Yolları
>
> Şimdiye kadar Rust derleyicisinin kullandığı en yaygın dosya yollarını ele aldık,
> ancak Rust eski tarz dosya yollarını da destekler. Crate kökünde bildirilen
> `front_of_house` adlı bir modül için, derleyici modülün kodunu şu konumlarda arar:
>
>
> - _src/front_of_house.rs_ (ele aldığımız)
> - _src/front_of_house/mod.rs_ (eski stil, hala desteklenen yol)
>
> `front_of_house` modülünün alt modülü olan `hosting` adlı bir modül için,
> derleyici modülün kodunu şu konumlarda arayacaktır:
>
> - _src/front_of_house/hosting.rs_ (incelediğimiz)
> - _src/front_of_house/hosting/mod.rs_ (eski stil, hala desteklenen yol)
>
> Aynı modül için her iki stili de kullanırsanız, derleyici hatası alırsınız.
> Aynı projedeki farklı modüller için her iki stili bir arada kullanmak
> mümkündür, ancak projenizi inceleyen kişiler için kafa karıştırıcı olabilir.
>
> _mod.rs_ adlı dosyaları kullanan stilin en büyük dezavantajı, projenizde
> çok sayıda _mod.rs_ adlı dosya bulunmasıdır. Bu da, dosyaları editörünüzde aynı anda açtığınızda
> kafa karıştırıcı olabilir.

Her modülün kodunu ayrı bir dosyaya taşıdık ve modül ağacı aynı kaldı.
`eat_at_restaurant` içindeki işlev çağrıları, tanımlar farklı dosyalarda olsa bile
herhangi bir değişiklik yapılmadan çalışacaktır. Bu teknik, modüllerin boyutu
büyüdükçe yeni dosyalara taşınmasına olanak tanır.

_src/lib.rs_ içindeki `pub use crate::front_of_house::hosting` ifadesinin de
değişmediğini ve `use` ifadesinin, kutu kapsamında derlenen dosyalar üzerinde
herhangi bir etkisi olmadığını unutmayın. `mod` anahtar sözcüğü modülleri bildirir ve Rust,
modüle girecek kodu modülle aynı ada sahip dosyada arar.

## Özet

Rust, bir paketi birden fazla kutuya ve bir kutuyu modüllere bölebilir, böylece

Rust, bir paketi birden fazla kutuya ve bir kutuyu modüllere bölebilmenizi sağlar, böylece
bir modülde tanımlanan öğelere başka bir modülden başvurabilirsiniz. Bunu,
mutlak veya göreli yollar belirterek yapabilirsiniz. Bu yollar, `use` deyimi ile
kapsama alınabilir, böylece o kapsamdaki öğeyi birden fazla kez kullanmak için
daha kısa bir yol kullanabilirsiniz. Modül kodu varsayılan olarak özeldir, ancak
`pub` anahtar sözcüğünü ekleyerek tanımları genel hale getirebilirsiniz.

Bir sonraki bölümde, düzenli kodunuzda kullanabileceğiniz standart kütüphanedeki bazı koleksiyon veri yapılarına
bakacağız.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md
