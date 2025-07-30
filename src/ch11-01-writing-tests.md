## Testler Nasıl Yazılır

Testler, test dışı kodun beklenen şekilde çalıştığını doğrulayan Rust işlevleridir.
Test işlevlerinin gövdeleri genellikle şu üç
işlemi gerçekleştirir:

- Gerekli verileri veya durumu ayarlayın.
- Test etmek istediğiniz kodu çalıştırın.
- Sonuçların beklediğiniz gibi olduğunu doğrulayın.

Rust'un bu eylemleri gerçekleştiren testler yazmak için özel olarak sağladığı özelliklere
bakalım. Bunlar arasında `test` özniteliği, birkaç makro ve
`should_panic` özniteliği bulunur.

### Test İşlevinin Yapısı

En basit haliyle, Rust'ta bir test, `test`
özniteliği ile açıklanmış bir işlevdir. Özellikler, Rust kod parçaları hakkında meta verileridir; bir örnek,
5. Bölümde yapılarla kullandığımız `derive` özelliğidir. Bir işlevi
test işlevine dönüştürmek için, `fn` önündeki satıra `#[test]` ekleyin.
`cargo test` komutuyla testlerinizi çalıştırdığınızda, Rust,
açıklamalı işlevleri çalıştıran ve her test işlevinin başarılı olup olmadığını
raporlayan bir test çalıştırıcı ikili dosyası oluşturur.

Cargo ile yeni bir kütüphane projesi oluşturduğumuzda, içinde test
fonksiyonu bulunan bir test modülü otomatik olarak oluşturulur. Bu modül,
testlerinizi yazmak için bir şablon sağlar, böylece her yeni projeye başladığınızda
tam yapısını ve sözdizimini aramak zorunda kalmazsınız. İstediğiniz kadar
ek test fonksiyonu ve test modülü ekleyebilirsiniz!

Herhangi bir kodu test etmeden önce, şablon testi ile deneyler yaparak testlerin nasıl çalıştığına dair bazı yönleri inceleyeceğiz.
Ardından, yazdığımız bazı kodları çağıran ve davranışlarının doğru olduğunu doğrulayan bazı gerçek dünya testleri yazacağız.
İki sayıyı toplayacak olan `adder` adlı yeni bir kütüphane projesi oluşturalım:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

`adder` kütüphanenizdeki _src/lib.rs_ dosyasının içeriği şöyle görünmelidir
Listing 11-1.

<Listing number="11-1" file-name="src/lib.rs" caption="The code generated automatically by `cargo new`">

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
echo "$ cargo test" > output.txt
RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 cargo test >> output.txt 2>&1
git diff output.txt # commit any relevant changes; discard irrelevant ones
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

</Listing>

Dosya, test etmek için bir örnek `add` işleviyle başlar.
Şimdilik, yalnızca `it_works` işlevine odaklanalım.

Şimdilik, yalnızca `it_works` işlevine odaklanalım. `#[test]`
açıklamasına dikkat edin: bu öznitelik, bunun bir test işlevi olduğunu gösterir, böylece test
çalıştırıcısı bu işlevi bir test olarak ele alır. `tests` modülünde, yaygın senaryolar oluşturmaya veya yaygın işlemleri gerçekleştirmeye yardımcı olmak için test
işlevi olmayan işlevler de olabilir, bu nedenle hangi işlevlerin test olduğunu her zaman belirtmemiz gerekir.
Örnek işlev gövdesi, `assert_eq!` makrosunu kullanarak, `add` işlevinin 2 ve 2 ile çağrıldığında

Örnek işlev gövdesi, `assert_eq!` makrosunu kullanarak, 2 ve 2 ile `add` işlevini çağırmanın sonucunu içeren `result`
değerinin 4'e eşit olduğunu doğrular. Bu
doğrulama, tipik bir testin formatına örnek teşkil eder. Bu testin başarılı olduğunu görmek için
onu çalıştıralım.

`cargo test` komutu, Listing
11-2'de gösterildiği gibi projemizdeki tüm testleri çalıştırır.

<Listing number="11-2" caption="The output from running the automatically generated test">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

</Listing>

Cargo testi derledi ve çalıştırdı. `running 1 test` satırını görüyoruz. Bir sonraki
satırda, `tests::it_works` adlı oluşturulan test işlevinin adı ve
bu testin sonucunun `ok` olduğu gösteriliyor. Genel özet `test
sonucu: ok.` tüm testlerin başarılı olduğu anlamına gelir ve `1
başarılı; 0 başarısız` kısmı, başarılı veya başarısız olan testlerin toplam sayısını gösterir.

Bir testi, belirli bir durumda çalıştırılmaması için
yoksayılmış olarak işaretlemek mümkündür; bunu bu bölümün ilerleyen kısımlarında [“Özel Olarak İstenmedikçe Bazı Testleri Yoksayma”][ignoring]<!-- ignore --> bölümünde ele alacağız. Burada
bunu yapmadığımız için özet `0 ignored` (0 yoksayıldı) olarak gösterilir. Ayrıca,
`cargo test` komutuna bir argüman geçirerek, adı bir
dizgiyle eşleşen testleri çalıştırabiliriz; buna _filtreleme_ denir ve bunu [“Adına Göre Testlerin Bir Alt Kümesini Çalıştırma”][subset]<!-- ignore --> bölümünde ele alacağız. Burada
çalıştırılan testleri filtrelemedik, bu nedenle özetin sonunda `0 filtered out` (0 filtrelendi) yazıyor.

`0 ölçüldü` istatistiği, performansı ölçen benchmark testleri içindir.
Benchmark testleri, bu yazının yazıldığı tarihte, yalnızca gece Rust sürümünde mevcuttur. Daha fazla bilgi için
[benchmark testleri hakkındaki belgelere][bench] bakın.

Test çıktısının `Doc-tests adder` ile başlayan sonraki kısmı,
herhangi bir dokümantasyon testinin sonuçları içindir. Henüz herhangi bir dokümantasyon testimiz yok,
ancak Rust, API dokümantasyonumuzda görünen tüm kod örneklerini derleyebilir.
Bu özellik, belgelerinizin ve kodunuzun senkronize olmasını sağlar! Belgeleme testlerinin nasıl yazılacağını
14. bölümün [“Test Olarak Belgeleme Yorumları”][doc-comments]<!-- ignore --> bölümünde ele alacağız. Şimdilik,
`Doc-tests` çıktısını göz ardı edeceğiz.

Testi kendi ihtiyaçlarımıza göre özelleştirmeye başlayalım. İlk olarak,
`it_works` işlevinin adını `exploration` gibi farklı bir adla değiştirin, şöyle:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Ardından `cargo test` komutunu tekrar çalıştırın. Çıktıda artık `it_works` yerine `exploration` yazıyor
:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Şimdi başka bir test ekleyeceğiz, ancak bu sefer başarısız olan bir test yapacağız! Testler,
test işlevinde bir şey paniklediğinde başarısız olur. Her test yeni bir
iş parçacığında çalıştırılır ve ana iş parçacığı bir test iş parçacığının öldüğünü gördüğünde, test
başarısız olarak işaretlenir. 9. Bölümde, paniğe kapılmanın en basit yolunun
`panic!` makrosunu çağırmak olduğunu konuşmuştuk. Yeni testi
`another` adlı bir işlev olarak girin, böylece _src/lib.rs_ dosyanız Listing 11-3 gibi görünür.

<Listing number="11-3" file-name="src/lib.rs" caption="Adding a second test that will fail because we call the `panic!` macro">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

</Listing>

`cargo test` komutunu kullanarak testleri tekrar çalıştırın. Çıktı, Listing
11-4 gibi görünmelidir. Bu, `exploration` testimizin başarılı olduğunu ve `another` testimizin başarısız olduğunu gösterir.

<Listing number="11-4" caption="Test results when one test passes and one test fails">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

</Listing>

<!-- manual-regeneration
rg panicked listings/ch11-writing-automated-tests/listing-11-03/output.txt
check the line number of the panic matches the line number in the following paragraph
 -->

`ok` yerine, `test tests::another` satırı `FAILED` gösterir. Bireysel sonuçlar ve özet arasında iki yeni
bölüm görünür: ilki
her test başarısızlığının ayrıntılı nedenini gösterir. Bu durumda,
`tests::another`'ın _src/lib.rs_ dosyasının 17. satırında `Make
this test fail` mesajıyla paniğe kapıldığı için başarısız olduğu bilgisini alırız. Bir sonraki bölümde
sadece başarısız olan tüm testlerin adları listelenir; bu, çok sayıda
test ve çok sayıda ayrıntılı başarısız test çıktısı olduğunda yararlıdır. Başarısız olan bir
testin adını kullanarak sadece o testi çalıştırıp daha kolay hata ayıklama yapabiliriz; testleri çalıştırma yöntemleri hakkında daha fazla bilgiyi
[“Testlerin Çalıştırılma Şeklinin Kontrol Edilmesi”][controlling-how-tests-are-run]<!-- ignore --> bölümünde daha ayrıntılı olarak ele alacağız.

Özet satırı sonunda görüntülenir: genel olarak, test sonucumuz `FAILED`.
Bir test başarılı, bir test başarısız oldu.

Farklı senaryolarda test sonuçlarının nasıl göründüğünü gördük,
şimdi testlerde yararlı olan `panic!` dışındaki bazı makroları inceleyelim.

### `assert!` Makrosu ile Sonuçları Kontrol Etme

Standart kütüphane tarafından sağlanan `assert!` makrosu, bir testteki bazı koşulların `true` olarak değerlendirildiğinden emin olmak istediğinizde
kullanışlıdır. `assert!` makrosuna Boolean olarak değerlendirilen bir argüman veririz. Değer
`true` ise hiçbir şey olmaz ve test geçer. Değer `false` ise,
`assert!` makrosu `panic!`'i çağırarak testin başarısız olmasına neden olur. `assert!`
makrosunu kullanmak, kodumuzun istediğimiz şekilde çalıştığını kontrol etmemize yardımcı olur.

Bölüm 5, Listing 5-15'te, bir `Rectangle` yapısı ve bir `can_hold`
yöntemi kullandık; bunlar Listing 11-5'te tekrarlanmaktadır. Bu kodu
_src/lib.rs_ dosyasına koyalım, ardından `assert!` makrosunu kullanarak bunun için bazı testler yazalım.

<Listing number="11-5" file-name="src/lib.rs" caption="The `Rectangle` struct and its `can_hold` method from Chapter 5">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs}}
```

</Listing>

`can_hold` yöntemi bir Boolean değeri döndürür, bu da `assert!` makrosu için mükemmel bir kullanım örneği olduğu anlamına gelir.
Listing 11-6'da, genişliği 8 ve yüksekliği 7 olan bir `Rectangle` örneği oluşturarak
`can_hold` yöntemini çalıştıran bir test yazıyoruz. Bu testte, genişliği 8 ve
yüksekliği 7 olan bir `Rectangle` örneği oluşturuyor ve bu örneğin, genişliği 5 ve yüksekliği 1 olan başka bir `Rectangle` örneğini
barındırabildiğini doğruluyoruz.v

<Listing number="11-6" file-name="src/lib.rs" caption="A test for `can_hold` that checks whether a larger rectangle can indeed hold a smaller rectangle">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

</Listing>

`tests` modülü içindeki `use super::*;` satırına dikkat edin. `tests` modülü,
[“Modül Ağacındaki Bir Öğeye Başvurma Yolları”][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -->
bölümünde ele aldığımız olağan görünürlük kurallarına uyan normal bir modüldür.
`tests` modülü bir iç modül olduğu için, `tests` modülü bir iç modül olduğu için, dış modüldeki test edilecek
kodu iç modülün kapsamına almamız gerekir. Burada bir glob kullanıyoruz, böylece dış modülde tanımladığımız her şey bu
`tests` modülünde kullanılabilir hale geliyor.
Testimize `larger_can_hold_smaller` adını verdik ve ihtiyacımız olan iki

Testimize `larger_can_hold_smaller` adını verdik ve ihtiyacımız olan iki
`Rectangle` örneğini oluşturduk. Ardından `assert!` makrosunu çağırdık ve
`larger.can_hold(&smaller)` çağrısının sonucunu ona aktardık. Bu ifade
`true` değerini döndürmesi gerektiğinden, testimiz başarılı olmalıdır. Hadi öğrenelim!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

Geçiyor! Başka bir test daha ekleyelim, bu sefer daha küçük bir
dikdörtgenin daha büyük bir dikdörtgeni içermemesi gerektiğini doğrulayan bir test:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Bu durumda `can_hold` işlevinin doğru sonucu `false` olduğundan,
bu sonucu `assert!` makrosuna aktarmadan önce tersine çevirmemiz gerekir. Sonuç olarak,
`can_hold` `false` döndürdüğünde testimiz başarılı olacaktır:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

Tİki test de başarılı! Şimdi kodumuza bir hata eklediğimizde test sonuçlarımızın ne olacağını görelim.
`can_hold` yönteminin uygulamasını, genişlikleri karşılaştırırken
büyük-küçük işaretini küçük-büyük işaretiyle değiştirerek değiştireceğiz:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

Testleri çalıştırdığınızda şu sonuçlar elde edilir:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

Testlerimiz hatayı yakaladı! `larger.width` değeri `8` ve `smaller.width` değeri
`5` olduğu için, `can_hold` içindeki genişliklerin karşılaştırması artık `false` sonucunu veriyor: 8, 5'ten
küçük değil.

### `assert_eq!` ve `assert_ne!` Makrolarıyla Eşitliği Test Etme

İşlevselliği doğrulamanın yaygın bir yolu, test edilen kodun sonucu ile kodun döndürmesini beklediğiniz değer arasındaki eşitliği test etmektir.
Bunu, `assert!` makrosunu kullanarak ve ona `==` operatörünü kullanarak bir ifade aktararak yapabilirsiniz.
Ancak, bu test o kadar yaygın bir testtir ki, standart kütüphane
bu testi daha rahat bir şekilde gerçekleştirmek için `assert_eq!` ve `assert_ne!` adlı bir çift makro sağlar. Ancak, bu o kadar yaygın bir testtir ki, standart kütüphane
bu testi daha kolay gerçekleştirmek için bir çift makro sağlar:
`assert_eq!` ve `assert_ne!`. Bu makrolar, sırasıyla iki argümanın eşitliğini veya
eşit olmadığını karşılaştırır. Ayrıca, onaylama başarısız olursa iki değeri de yazdırırlar,
bu da testin neden başarısız olduğunu görmeyi kolaylaştırır; tersine,
`assert!` makrosu, `==`
ifadesi için `false` değeri aldığını belirtir, ancak `false` değerine yol açan değerleri yazdırmaz.

Listing 11-7'de, parametresine `2` ekleyen `add_two` adlı bir fonksiyon yazıyoruz,
ardından bu fonksiyonu `assert_eq!` makrosunu kullanarak test ediyoruz.

<Listing number="11-7" file-name="src/lib.rs" caption="Testing the function `add_two` using the `assert_eq!` macro">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

</Listing>

Geçip geçmediğini kontrol edelim!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

`add_two(2)` çağrısının sonucunu tutan `result` adlı bir değişken oluşturuyoruz.
Ardından `result` ve `4` değerlerini `assert_eq!` makrosuna argüman olarak
aktarıyoruz. Bu testin çıktı satırı `test tests::it_adds_two...
 ok` şeklindedir ve `ok` metni testimizin başarılı olduğunu gösterir!

Kodumuza bir hata ekleyerek `assert_eq!` başarısız olduğunda nasıl göründüğünü görelim.
`add_two` işlevinin uygulamasını değiştirerek yerine `3` ekleyin:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Testleri tekrar çalıştırın:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Testimiz hatayı yakaladı! `tests::it_adds_two` testi başarısız oldu ve mesaj
bize başarısız olan onaylamanın `left == right` olduğunu ve `left`
ve `right` değerlerinin ne olduğunu söylüyor. Bu mesaj hata ayıklamaya başlamamıza yardımcı oluyor: `left`
argümanı, `add_two(2)` çağrısının sonucunu içeren, `5` idi, ancak
`right` argümanı `4` idi. Bu durumun, özellikle çok sayıda test yaptığımızda
çok yararlı olacağını tahmin edebilirsiniz.

Bazı dillerde ve test çerçevelerinde, eşitlik
iddia işlevlerinin parametreleri `expected` ve `actual` olarak adlandırılır ve
argümanları belirtme sırası önemlidir. Ancak Rust'ta bunlar `left` ve
`right` olarak adlandırılır ve beklediğimiz değeri ve kodun ürettiği değeri
belirtme sırası önemli değildir. Bu testteki onaylamayı
`assert_eq!(4, result)` olarak yazabiliriz, bu da `` onaylama `left == right` başarısız`` mesajının
görüntülenmesiyle sonuçlanır.

`assert_ne!` makrosu, verdiğimiz iki değer eşit değilse geçer,
eşitse başarısız olur. Bu makro, bir değerin ne olacağından emin olmadığımız,
ancak kesinlikle ne olmaması gerektiğini bildiğimiz durumlarda en kullanışlıdır.
Örneğin, girdisini bir şekilde değiştireceği garanti edilen bir fonksiyonu test ediyorsak,
ancak girdinin değiştirilme şekli testleri yaptığımız günün
haftanın hangi günü olduğuna bağlıysa, en iyi iddia, fonksiyonun çıktısının girdisine eşit
olmadığı olabilir.

Yüzeyin altında, `assert_eq!` ve `assert_ne!` makroları sırasıyla
`==` ve `!=` operatörlerini kullanır. Assertionlar başarısız olduğunda, bu makrolar
argümanlarını hata ayıklama biçimlendirmesini kullanarak yazdırır, bu da karşılaştırılan değerlerin
`PartialEq` ve `Debug` özelliklerini uygulaması gerektiği anlamına gelir. Tüm temel türler ve
standart kütüphane türlerinin çoğu bu özellikleri uygular. Kendiniz tanımladığınız yapılar ve sıralamalar için,
bu türlerin eşitliğini doğrulamak üzere `PartialEq` özelliğini uygulamalısınız.
Ayrıca, doğrulama başarısız olduğunda değerleri yazdırmak için `Debug` özelliğini de uygulamalısınız.
Her iki özellik de türetilebilir özellikler olduğundan,
Bölüm 5'teki Listing 5-12'de belirtildiği gibi, bu genellikle yapı veya sıralama tanımınıza
`#[derive(PartialEq, Debug)]` eklemek kadar basittir. Bu ve diğer türetilebilir özellikler hakkında daha fazla
bilgi için Ek C, [“Türetilebilir Özellikler,”][derivable-traits]<!-- ignore --> bölümüne bakın.

### Özel Hata Mesajları Ekleme

Ayrıca, `assert!`, `assert_eq!` ve `assert_ne!` makrolarına isteğe bağlı argümanlar olarak
hata mesajıyla birlikte yazdırılacak özel bir mesaj da ekleyebilirsiniz. Gerekli argümanlardan sonra belirtilen tüm
argümanlar,
`format!` makrosuna aktarılır (bkz. [“`+` Operatörü veya `format!` Makrosu ile Birleştirme”][concatenation-with-the--operator-or-the-format-macro]<!--
ignore --> bölümünde ele alınmıştır), bu nedenle `{}`
yer tutucuları ve bu yer tutuculara girecek değerleri içeren bir biçim dizesi aktarabilirsiniz. Özel mesajlar, bir onaylamanın ne anlama geldiğini belgelemek için kullanışlıdır;
bir test başarısız olduğunda, koddaki sorunun ne olduğu hakkında daha iyi
bir fikir sahibi olursunuz.

Örneğin, insanları isimleriyle selamlayan bir işlevimiz olduğunu ve
işleve aktardığımız ismin çıktıda göründüğünü test etmek istediğimizi varsayalım:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

Bu programın gereksinimleri henüz kararlaştırılmadı ve
selamlamanın başındaki `Hello` metninin değişeceği konusunda oldukça eminiz.
Gereksinimler değiştiğinde testi güncellemek istemediğimize karar verdik,
bu nedenle `greeting` işlevinden döndürülen değerle tam olarak eşit olup olmadığını
kontrol etmek yerine, çıktının giriş parametresinin metnini içerdiğini
doğrulayacağız.

Şimdi, `greeting` işlevini `name` parametresini hariç tutacak şekilde değiştirerek
bu koda bir hata ekleyelim ve varsayılan test hatasının nasıl göründüğünü görelim:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

Bu testi çalıştırdığınızda aşağıdaki sonuçlar elde edilir:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

Bu sonuç sadece iddianın başarısız olduğunu ve iddianın hangi satırda olduğunu gösterir.
Daha yararlı bir hata mesajı,
`greeting` işlevinden alınan değeri yazdırır.
`greeting` işlevinden aldığımız gerçek değerle doldurulmuş bir yer tutucu içeren bir biçim dizesi içeren özel bir hata mesajı ekleyelim:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Şimdi testi çalıştırdığımızda, daha bilgilendirici bir hata mesajı alacağız:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

Test çıktısında gerçekte elde ettiğimiz değeri görebiliriz, bu da bize
beklediğimiz şey yerine olanları hata ayıklamada yardımcı olur.

### `should_panic` ile Panik Kontrolü

Dönüş değerlerini kontrol etmenin yanı sıra, kodumuzun
hata koşullarını beklediğimiz gibi işlediğini kontrol etmek de önemlidir. Örneğin, Bölüm 9, Listing 9-13'te oluşturduğumuz `Guess` türünü
düşünelim. `Guess` türünü kullanan diğer kodlar, `Guess` örneklerinin yalnızca
1 ile 100 arasındaki değerleri içereceğine dair garantiye dayanır. Bu aralığın
dışındaki bir değerle `Guess` örneği oluşturmaya çalışıldığında panik oluştuğunu
garanti eden bir test yazabiliriz.

Bunu, test işlevimize `should_panic` özniteliğini ekleyerek yaparız. İşlevin içindeki kod panik yaparsa
test geçer; işlevin içindeki kod panik yapmazsa
test başarısız olur.

Listing 11-8, `Guess::new`'un hata koşullarının
beklediğimiz şekilde gerçekleştiğini kontrol eden bir testi gösterir.

<Listing number="11-8" file-name="src/lib.rs" caption="Testing that a condition will cause a `panic!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

</Listing>

`#[test]` özniteliğinden sonra ve uygulandığı test işlevinden önce `#[should_panic]` özniteliğini yerleştiriyoruz. Bu testin başarılı olduğu durumda sonucu inceleyelim:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

Güzel görünüyor! Şimdi, değer 100'den büyükse `new` işlevinin paniğe kapılacağı koşulunu kaldırarak kodumuza bir hata ekleyelim:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Listing 11-8'deki testi çalıştırdığımızda, test başarısız olacaktır:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

Bu durumda çok yardımcı bir mesaj almıyoruz, ancak test işlevine baktığımızda,
`#[should_panic]` ile işaretlendiğini görüyoruz. Aldığımız hata, test işlevindeki kodun
panik durumuna neden olmadığı anlamına geliyor.

`should_panic` kullanan testler kesin olmayabilir. Bir `should_panic` testi,
beklediğimizden farklı bir nedenden dolayı panik yapsa bile
geçer. `should_panic` testlerini daha kesin hale getirmek için, `should_panic` özniteliğine isteğe bağlı bir
`expected` parametresi ekleyebiliriz. Test donanımı,
hata mesajının sağlanan metni içerdiğinden emin olacaktır. Örneğin,
Listing 11-9'daki `Guess` için değiştirilmiş kodu ele alalım. Burada `new` işlevi,
değerin çok küçük veya
çok büyük olmasına bağlı olarak farklı mesajlarla paniğe kapılır.

<Listing number="11-9" file-name="src/lib.rs" caption="Testing for a `panic!` with a panic message containing a specified substring">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

</Listing>

Bu test başarılı olacaktır çünkü `should_panic` özniteliğinin
`expected` parametresine girdiğimiz değer, `Guess::new`
işlevinin paniklediği mesajın bir alt dizesidir. Beklediğimiz panik mesajının tamamını
belirtmiş olabilirdik, bu durumda mesaj `Guess value must be less than or equal to
100, got 200` olurdu. Ne belirleyeceğiniz, panik mesajının ne kadarının benzersiz veya dinamik olduğuna ve
testinizin ne kadar kesin olmasını istediğinize bağlıdır. Bu durumda, panik
mesajının bir alt dizesi, test işlevindeki kodun `else if value > 100` durumunu
yürütmesini sağlamak için yeterlidir.

`expected` mesajı olan bir `should_panic` testi başarısız olduğunda ne olduğunu görmek için,
`if value < 1` ve `else if value > 100` bloklarının gövdelerini değiştirerek kodumuza tekrar bir hata ekleyelim:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

Bu sefer `should_panic` testini çalıştırdığımızda, test başarısız olacaktır:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

Hata mesajı, bu testin beklediğimiz gibi gerçekten paniklediğini gösteriyor,
ancak panik mesajı beklenen `100'den küçük veya eşit` dizisini içermiyordu.
Bu durumda aldığımız panik mesajı `Tahmin değeri 1'den büyük veya eşit
olmalıdır, 200 alındı.` Şimdi hatanın nerede olduğunu bulmaya başlayabiliriz!
Testlerde `Result<T, E>` kullanma

### Testlerde `Result<T, E>` kullanımı

Şimdiye kadar yaptığımız testlerin tümü başarısız olduğunda paniklemiştir.
`Result<T, E>` kullanan testler de yazabiliriz! Listing 11-1'deki test, `Result<T,
E>` kullanmak ve paniklemek yerine `Err` döndürmek üzere yeniden yazılmıştır:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

`it_works` işlevi artık `Result<(), String>` dönüş türünü kullanıyor. İşlevin
gövdesinde, `assert_eq!` makrosunu çağırmak yerine, test başarılı olduğunda
`Ok(())`, test başarısız olduğunda ise içinde `String` bulunan bir `Err` döndürüyoruz.
`Result<T, E>` döndüren testler yazmak, testlerin gövdesinde soru işareti

Testleri `Result<T, E>` döndürecek şekilde yazmak, testlerin gövdesinde soru
işareti operatörünü kullanmanıza olanak tanır. Bu, içlerindeki herhangi bir işlem `Err` varyantı döndürdüğünde başarısız olması gereken testleri yazmak için
kullanışlı bir yol olabilir.

`Result<T,
E>` kullanan testlerde `#[should_panic]` ek açıklaması kullanamazsınız. Bir işlemin `Err` varyantını döndürdüğünü doğrulamak için, `Result<T, E>` değerinde
soru işareti operatörünü _kullanmayın_. Bunun yerine,
`assert!(value.is_err())` kullanın.

Test yazmanın çeşitli yollarını öğrendiğinize göre, testlerimizi çalıştırdığımızda neler olduğunu
inceleyelim ve `cargo
test` ile kullanabileceğimiz farklı seçenekleri keşfedelim.

[concatenation-with-the--operator-or-the-format-macro]: ch08-02-strings.md#-operatörü-veya-format-makrosu-ile-birleştirme
[bench]: ../unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.md#özel-olarak-talep-edilmedikçe-bazı-testleri-göz-ardı-etmek
[subset]: ch11-02-running-tests.md#adına-göre-testlerin-bir-alt-kümesini-çalıştırma
[controlling-how-tests-are-run]: ch11-02-running-tests.md#testlerin-nasıl-çalıştırılacağını-kontrol-etme
[derivable-traits]: appendix-03-derivable-traits.md
[doc-comments]: ch14-02-publishing-to-crates-io.md#dokümantasyon-yorumları-test-olarak
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md
