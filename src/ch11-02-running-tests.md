## Testlerin Nasıl Çalıştırılacağını Kontrol Etme

`cargo run` komutu kodunuzu derleyip elde edilen ikili dosyayı çalıştırdığı gibi,
`cargo test` komutu da kodunuzu test modunda derleyip elde edilen test
ikili dosyasını çalıştırır. `cargo test` komutu tarafından üretilen ikili dosyanın varsayılan davranışı,
tüm testleri paralel olarak çalıştırmak ve test çalıştırmaları sırasında üretilen çıktıyı yakalamaktır.
Bu, çıktının görüntülenmesini engelleyerek test sonuçlarıyla ilgili çıktının okunmasını kolaylaştırır. Ancak, bu varsayılan davranışı değiştirmek için komut satırı
seçenekleri belirtebilirsiniz.

Bazı komut satırı seçenekleri `cargo test` komutuna, bazıları ise sonuçta ortaya çıkan test
ikili dosyasına gider. Bu iki tür argümanı ayırmak için, `cargo test` komutuna giden argümanları
ayırıcı `--` işaretinden sonra, test ikili dosyasına giden argümanları ise
ayırıcı işaretinden sonra listelersiniz. `cargo test --help` komutunu çalıştırdığınızda `cargo test` ile kullanabileceğiniz seçenekler
görüntülenir ve `cargo test -- --help` komutunu çalıştırdığınızda ayırıcıdan sonra kullanabileceğiniz seçenekler
görüntülenir. Bu seçenekler [rustc kitabının][rustc] [“Testler” bölümünde][tests] de belgelenmiştir.

[tests]: https://doc.rust-lang.org/rustc/tests/index.html
[rustc]: https://doc.rust-lang.org/rustc/index.html

### Testleri Paralel veya Ardışık Olarak Çalıştırma

Birden fazla test çalıştırdığınızda, varsayılan olarak bunlar iş parçacıkları kullanılarak paralel olarak çalışır,
yani daha hızlı tamamlanır ve geri bildirim daha hızlı alınır. Testler aynı anda çalıştığı için,
testlerinizin birbirine veya paylaşılan bir duruma, örneğin
geçerli çalışma dizini veya ortam değişkenleri gibi paylaşılan bir ortama bağlı olmadığından emin olmalısınız.
Örneğin, her testinizin diskte

Örneğin, her testinizin diskte _test-output.txt_ adlı bir dosya oluşturan ve
bu dosyaya bazı veriler yazan bir kod çalıştırdığını varsayalım. Ardından her test,
bu dosyadaki verileri okur ve dosyanın her testte farklı olan belirli bir değer
içerdiğini doğrular. Testler aynı anda çalıştığı için, bir test, başka bir testin
dosyayı yazıp okuduğu süre içinde dosyayı üzerine yazabilir. İkinci test, kodun yanlış olması nedeniyle değil,
testlerin paralel olarak çalışırken birbirlerini etkilemesi nedeniyle
başarısız olur. Bir çözüm, her testin farklı bir dosyaya yazdığından emin olmaktır;
diğer bir çözüm ise testleri tek tek çalıştırmaktır.

Testleri paralel olarak çalıştırmak istemiyorsanız veya kullanılan iş parçacığı sayısını daha ayrıntılı
olarak kontrol etmek istiyorsanız, `--test-threads` bayrağını
ve kullanmak istediğiniz iş parçacığı sayısını test ikili dosyasına gönderebilirsiniz. Aşağıdaki örneğe
bakın:

```console
$ cargo test -- --test-threads=1
```

Test iş parçacığı sayısını `1` olarak ayarlayarak programa herhangi bir
paralellik kullanmamasını söylüyoruz. Testleri tek bir iş parçacığı kullanarak çalıştırmak,
paralel olarak çalıştırmaktan daha uzun sürer, ancak testler durum paylaşımı yaparsa
birbirlerini etkilemezler.

### İşlev Çıktısını Gösterme

Varsayılan olarak, bir test geçerse, Rust'un test kütüphanesi standart çıktıya yazdırılan
her şeyi yakalar. Örneğin, bir testte `println!` çağırırsak ve test
geçerse, terminalde `println!` çıktısını görmeyiz; sadece testin geçtiğini
belirten satırı görürüz. Bir test başarısız olursa, standart çıktıya yazdırılan her şeyi
başarısızlık mesajının geri kalanıyla birlikte görürüz.

Örnek olarak, Listing 11-10, parametresinin değerini yazdırıp 10 değerini döndüren
aptalca bir fonksiyonun yanı sıra, geçen ve başarısız olan birer test içerir.

<Listing number="11-10" file-name="src/lib.rs" caption="Tests for a function that calls `println!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

</Listing>

Bu testleri `cargo test` ile çalıştırdığımızda, aşağıdaki çıktıyı göreceğiz:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

Bu çıktının hiçbir yerinde, başarılı test çalıştırıldığında yazdırılan `I got the value 4` ifadesini görmüyoruz.
Bu çıktı yakalanmıştır.
Başarısız testin çıktısı olan `I got the value 8`, test özeti çıktısının bölümünde görünür ve
testin başarısız olma nedenini de gösterir.

Geçen testlerin basılan değerlerini de görmek istersek, Rust'a
`--show-output` ile başarılı testlerin çıktısını da göstermesini söyleyebiliriz:

```console
$ cargo test -- --show-output
```

Listing 11-10'daki testleri `--show-output` bayrağıyla tekrar çalıştırdığımızda,
aşağıdaki çıktıyı görürüz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### Adına Göre Testlerin Bir Alt Kümesini Çalıştırma

Bazen, tam bir test dizisini çalıştırmak uzun zaman alabilir. Belirli bir alanda kod üzerinde çalışıyorsanız,
sadece o kodla ilgili testleri çalıştırmak isteyebilirsiniz.
Çalıştırmak istediğiniz testlerin adını veya adlarını `cargo test` komutuna argüman olarak geçirerek,
hangi testleri çalıştıracağınızı seçebilirsiniz.

Testlerin bir alt kümesini nasıl çalıştıracağımızı göstermek için, önce Listing 11-11'de gösterildiği gibi
`add_two` işlevimiz için üç test oluşturacağız ve hangilerini çalıştıracağımızı seçeceğiz.

<Listing number="11-11" file-name="src/lib.rs" caption="Three tests with three different names">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

</Listing>

Daha önce gördüğümüz gibi, herhangi bir argüman geçirmeden testleri çalıştırırsak, tüm
testler paralel olarak çalışacaktır:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### Tekli Testleri Çalıştırma

Herhangi bir test fonksiyonunun adını `cargo test` komutuna aktararak sadece o testi çalıştırabiliriz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Sadece `one_hundred` adlı test çalıştırıldı; diğer iki test bu adla eşleşmedi.
Test çıktısı, sonunda `2 filtered out` ifadesini göstererek çalıştırılmayan başka testler olduğunu
bize bildirir.

Bu şekilde birden fazla testin adını belirleyemeyiz; `cargo test` komutuna verilen ilk değer
kullanılır. Ancak birden fazla testi çalıştırmanın bir yolu vardır.

#### Birden Fazla Testi Çalıştırmak için Filtreleme

Test adının bir kısmını belirtebiliriz ve adı bu değerle eşleşen tüm testler
çalıştırılır. Örneğin, testlerimizin ikisinin adı `add` içerdiğinden,
`cargo test add` komutunu çalıştırarak bu ikisini çalıştırabiliriz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

Bu komut, adında `add` geçen tüm testleri çalıştırdı ve `one_hundred` adlı testi filtreledi.
Ayrıca, bir testin bulunduğu modülün testin adının bir parçası haline geldiğini de unutmayın.
Böylece, modülün adını filtreleyerek modüldeki tüm testleri çalıştırabiliriz.

### Özel olarak talep edilmedikçe bazı testleri göz ardı etmek

Bazen bazı özel testlerin yürütülmesi çok zaman alabilir, bu nedenle
`cargo test` komutunu çalıştırırken bunları hariç tutmak isteyebilirsiniz. Çalıştırmak istediğiniz tüm testleri argüman olarak listelemek yerine,
zaman alan testleri `ignore` özniteliğini kullanarak
burada gösterildiği gibi hariç tutmak için açıklama ekleyebilirsiniz:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

`#[test]`'den sonra, hariç tutmak istediğimiz teste `#[ignore]` satırını ekliyoruz.
Şimdi testlerimizi çalıştırdığımızda, `it_works` çalışıyor, ancak `expensive_test` çalışmıyor:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

`expensive_test` işlevi `ignored` olarak listelenmiştir. Yalnızca
yoksayılan testleri çalıştırmak istiyorsak, `cargo test -- --ignored` komutunu kullanabiliriz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

Hangi testlerin çalıştırılacağını kontrol ederek, `cargo test` sonuçlarının
hızlı bir şekilde döndürülmesini sağlayabilirsiniz. `ignored` testlerinin sonuçlarını kontrol etmenin mantıklı olduğu bir noktada
ve sonuçları beklemek için zamanınız varsa,
bunun yerine `cargo test -- --ignored` komutunu çalıştırabilirsiniz. İhmal edilmiş olsun ya da olmasın tüm testleri çalıştırmak istiyorsanız,
`cargo test -- --include-ignored` komutunu çalıştırabilirsiniz.