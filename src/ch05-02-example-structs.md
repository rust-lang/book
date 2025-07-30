## Yapıları Kullanan Örnek Bir Program

Yapıları ne zaman kullanmak isteyebileceğimizi anlamak için
adresinde bir dikdörtgenin alanını hesaplayan bir program yazalım. Tek değişkenler kullanarak başlayacağız ve
daha sonra struct'ları kullanana kadar programı yeniden düzenleyeceğiz.

Cargo ile _rectangles_ adında,
piksel cinsinden belirtilen bir dikdörtgenin genişliğini ve yüksekliğini alacak ve dikdörtgenin alanını
hesaplayacak yeni bir ikili proje yapalım. Liste 5-8, projemizin _src/main.rs_ dosyasında
tam olarak bunu yapmanın bir yolunu içeren kısa bir programı göstermektedir.

<Listing number="5-8" file-name="src/main.rs" caption="Calculating the area of a rectangle specified by separate width and height variables">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

</Listing>

Şimdi, `cargo run` kullanarak bu programı çalıştırın:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Bu kod, her boyutta
`area` fonksiyonunu çağırarak dikdörtgenin alanını bulmayı başarıyor, ancak bu kodu açık
ve okunabilir hale getirmek için daha fazlasını yapabiliriz.

Bu kodla ilgili sorun `area` imzasında açıkça görülmektedir:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

Alan' fonksiyonunun bir dikdörtgenin alanını hesaplaması gerekiyor, ancak yazdığımız
fonksiyonunun iki parametresi var ve
programımızın hiçbir yerinde parametrelerin ilişkili olduğu açık değil. Genişlik ve yüksekliği birlikte gruplamak daha okunabilir ve daha
yönetilebilir olacaktır. Bunu yapmanın bir yolunu
Bölüm 3'ün [“The Tuple Type”][the-tuple-type]<!-- ignore -->
bölümünde tartışmıştık: tuples kullanarak.

### Tuples ile Yeniden Düzenleme

Liste 5-9, programımızın tuple kullanan başka bir versiyonunu göstermektedir.

<Listing number="5-9" file-name="src/main.rs" caption="Specifying the width and height of the rectangle with a tuple">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

</Listing>

Bir yönden, bu program daha iyi. Tuple'lar biraz yapı eklememize izin veriyor ve
artık sadece bir argüman geçiyoruz. Ancak başka bir yönden, bu versiyon daha az
açıktır: tuple'lar öğelerini adlandırmaz, bu nedenle
tuple'ın parçalarını indekslememiz gerekir, bu da hesaplamamızı daha az belirgin hale getirir.

Genişlik ve yüksekliği karıştırmak alan hesaplaması için önemli değildir, ancak
dikdörtgeni ekrana çizmek istiyorsak, bu önemli olacaktır! `Genişlik`in `0` indeksli tuple olduğunu ve `yükseklik`in
indeksli tuple olduğunu
aklımızda tutmamız gerekecektir. Kodumuzu kullanacak bir başkasının bunu anlaması ve
aklında tutması daha da zor olacaktır. Kodumuzdaki verilere
adresinin anlamını aktarmadığımız için, hata yapmak artık daha kolay.

### Yapılarla Yeniden Düzenleme: Daha Fazla Anlam Eklemek

Verileri etiketleyerek anlam katmak için yapıları kullanırız. Kullandığımız
tuple'ını, Liste 5-10'da gösterildiği gibi, bütün için bir adın yanı sıra
parçaları için adlar içeren bir struct'a dönüştürebiliriz.

<Listing number="5-10" file-name="src/main.rs" caption="Defining a `Rectangle` struct">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

</Listing>

Burada, bir struct tanımladık ve adını `Rectangle` koyduk. Kıvrımlı
parantezlerinin içinde, her ikisi de
tipinde `u32` olan alanları `width` ve `height` olarak tanımladık. Daha sonra, `main` içinde, genişliği `30` ve yüksekliği `50` olan özel bir `Rectangle`
örneği oluşturduk.

Şimdi `area` fonksiyonumuz, türü struct `Rectangle`
örneğinin değişmez bir ödünçlemesi olan
`rectangle` adını verdiğimiz bir parametre ile tanımlanmıştır. Bölüm 4'te belirtildiği gibi, yapının sahipliğini
almak yerine ödünç almak istiyoruz. Bu şekilde, `main` sahipliğini korur ve
`rect1` kullanarak devam edebilir, bu da fonksiyon imzasında `&` ve fonksiyonu çağırdığımız yerde
kullanmamızın nedenidir.

`area` fonksiyonu `Rectangle`
örneğinin `width` ve `height` alanlarına erişir (ödünç alınan bir struct örneğinin alanlarına erişmenin
alan değerlerini taşımadığını unutmayın, bu yüzden sık sık structların ödünç alındığını görürsünüz). Şimdi `area` için
fonksiyon imzamız tam olarak ne demek istediğimizi söylüyor: `Rectangle` örneğinin `width` ve `height` alanlarını kullanarak
alanını hesaplayın. Bu,
genişlik ve yüksekliğin birbiriyle ilişkili olduğunu ifade eder ve `0` ve `1` tuple indeks değerlerini kullanmak yerine değerleri
için açıklayıcı isimler verir. Bu, netlik için bir
kazanımıdır.

### Türetilmiş Özelliklerle Faydalı İşlevsellik Ekleme

Programımızın hata ayıklamasını yaparken
bir `Rectangle` örneğini yazdırabilmek ve tüm alanlarının değerlerini görebilmek yararlı olacaktır. Liste 5-11,
önceki bölümlerde kullandığımız [`println!` makrosunu][println]<!-- ignore --> kullanarak
adresini dener. Ancak bu işe yaramayacaktır.

<Listing number="5-11" file-name="src/main.rs" caption="Attempting to print a `Rectangle` instance">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

</Listing>

Bu kodu derlediğimizde, bu çekirdek mesajla bir hata alıyoruz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

`println!` makrosu birçok türde biçimlendirme yapabilir ve varsayılan olarak, kıvırcık
parantezleri `println!`'e `Display` olarak bilinen biçimlendirmeyi kullanmasını söyler: doğrudan son kullanıcı tüketimi için
çıktısı. Şimdiye kadar gördüğümüz ilkel tipler
varsayılan olarak `Display` uygular çünkü
bir `1` veya başka bir ilkel tipi kullanıcıya göstermenin tek bir yolu vardır. Ancak yapılarda,
`println!` çıktısını biçimlendirme şekli daha az nettir çünkü daha fazla
görüntüleme olasılığı vardır: Virgül istiyor musunuz, istemiyor musunuz? küme parantezlerini yazdırmak istiyor musunuz? Tüm alanlar gösterilmeli mi? Bu belirsizlik nedeniyle, Rust
ne istediğimizi tahmin etmeye çalışmaz ve yapıların `println!` ve `{}` yer tutucusu ile kullanmak için sağlanan bir
`Display` uygulaması yoktur.

Hataları okumaya devam edersek, bu yararlı notu bulacağız:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Hadi deneyelim! `println!` makro çağrısı şimdi `println!("rect1 isnb{rect1:?}");` şeklinde görünecektir. Küme parantezlerinin içine `:?` belirtecini koymak,
`println!` adresine `Debug` adlı bir çıktı biçimi kullanmak istediğimizi söyler. Debug` özelliği
struct'ımızı geliştiriciler için yararlı olacak şekilde yazdırmamızı sağlar, böylece kodumuzda hata ayıklarken değerini
görebiliriz.

Kodu bu değişiklikle derleyin. Kahretsin! Hala bir hata alıyoruz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Ancak yine de derleyici bize yardımcı bir not veriyor:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Rust, hata ayıklama bilgilerini yazdırmak için işlevsellik içerir, ancak
bu işlevselliği yapımız için kullanılabilir hale getirmeyi açıkça seçmemiz gerekir.
Bunu yapmak için, Listing 5-12'de gösterildiği gibi,
struct tanımından hemen önce `#[derive(Debug)]` dış niteliğini ekliyoruz.

<Listing number="5-12" file-name="src/main.rs" caption="Adding the attribute to derive the `Debug` trait and printing the `Rectangle` instance using debug formatting">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

</Listing>

Şimdi programı çalıştırdığımızda herhangi bir hata almayacağız ve aşağıdaki
çıktısını göreceğiz:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Güzel! Bu en güzel çıktı değil, ancak bu örnek için
tüm alanların değerlerini gösteriyor, bu da hata ayıklama sırasında kesinlikle yardımcı olacaktır. daha büyük yapılara sahip olduğumuzda, okunması biraz daha kolay bir çıktı elde etmek yararlıdır;
bu durumlarda, `println!` dizesinde `{:?}` yerine `{:#?}` kullanabiliriz. bu örnekte, `{:#?}` stilini kullanmak aşağıdaki çıktıyı verecektir:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

`Debug` formatını kullanarak bir değer yazdırmanın bir başka yolu da [`dbg!`makro][dbg]<!-- ignore --> kullanmaktır; bu makro bir ifadenin sahipliğini alır (referans alan `println!` makrosunun
aksine), kodunuzda bu `dbg!` makro çağrısının gerçekleştiği
dosya ve satır numarasını bu ifadenin
sonuç değeriyle birlikte yazdırır ve değerin sahipliğini geri verir.

> Not: `dbg!` makrosunu çağırmak, standart hata konsol akışına
> (`stderr`) yazdırır, buna karşılık `println!` makrosu standart çıktı
> konsol akışına (`stdout`) yazdırır. `stderr` ve `stdout` hakkında daha fazla bilgiyi
> [“Hata Mesajlarını Standart Çıktı Yerine Standart Hata Akışına Yazma”
> bölümünde, 12. Bölüm][err]<!-- ignore -->.

İşte,
`width` alanına atanan değerin yanı sıra `rect1` içindeki tüm yapının değerini de ilgilendiren bir örnek:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

`30 * scale` ifadesinin etrafına `dbg!` koyabiliriz ve `dbg!`
ifadenin değerinin sahipliğini geri verdiği için, `width` alanı,
`dbg!` çağrısı olmasaydı alacağı değerle aynı değeri alacaktır. `dbg!`'nin
`rect1`'in sahipliğini almasını istemediğimiz için, bir sonraki çağrıda `rect1`'e bir referans kullanırız.
Bu örneğin çıktısı şöyle görünür:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

İlk çıktının _src/main.rs_ dosyasının 10. satırından geldiğini görebiliriz. Burada
`30 * scale` ifadesini hata ayıklıyoruz ve sonuç değeri `60` oluyor (
`Debug` biçimlendirmesi tamsayılar için yalnızca değerlerini yazdırmak üzere uygulanmıştır).
`dbg!` çağrısı, `&rect1` değerini, yani `Rectangle` yapısını çıktılar. Bu çıktı,
`Rectangle` türünün güzel `Debug` biçimlendirmesini kullanır. `dbg!` makrosu, kodunuzun ne yaptığını anlamaya çalışırken
gerçekten yardımcı olabilir!
`Debug` özelliğine ek olarak, Rust, özel türlerimize yararlı davranışlar ekleyebildiğimiz

`Debug` özelliğine ek olarak, Rust, özel türlerimize yararlı davranışlar ekleyebilen
`derive` özniteliği ile kullanabileceğimiz bir dizi özellik sunar.
Bu özellikler ve davranışları [Ek C][app-c]<!--
ignore -->'da listelenmiştir. Bu özellikleri özel davranışlarla nasıl uygulayacağımızı ve
kendi özelliklerinizi nasıl oluşturacağınızı 10. Bölümde ele alacağız. `derive` dışında birçok
özellik de vardır; daha fazla bilgi için [Rust Referansı'nın “Özellikler”
bölümüne][attributes] bakın.

`area` işlevimiz çok spesifiktir: sadece dikdörtgenlerin alanını hesaplar.
Bu davranışı `Rectangle` yapımızla daha yakından ilişkilendirmek faydalı olacaktır,
çünkü başka hiçbir türle çalışmayacaktır. `area` işlevini `Rectangle` türümüzde tanımlanan bir `area` _yöntemi_
haline getirerek bu kodu nasıl yeniden düzenleyebileceğimize bakalım.
`Rectangle` türünde tanımlanan bir `area` _yöntemi_

[the-tuple-type]: ch03-02-data-types.md#tuple-türü
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.md
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.md
[attributes]: ../reference/attributes.html
