## Komut Satırı Argümanlarını Kabul Etme

Her zamanki gibi `cargo new` ile yeni bir proje oluşturalım. Projemize, sisteminizde zaten mevcut olabilecek `grep` aracından ayırt etmek için
`minigrep` adını vereceğiz.
`cargo new minigrep`

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

İlk görev, `minigrep`'in iki komut satırı argümanını kabul etmesini sağlamaktır:
dosya yolu ve aranacak dize. Yani, programımızı
`cargo run` ile çalıştırabilmek istiyoruz, iki tire ile sonraki argümanların
`cargo` için değil programımız için olduğunu belirtmek istiyoruz, aranacak bir dize ve
aranacak dosyanın yolu, şöyle:

```console
$ cargo run -- searchstring example-filename.txt
```

Şu anda, `cargo new` tarafından oluşturulan program, ona verdiğimiz argümanları
işleyemiyor. [crates.io](https://crates.io/) adresinde bulunan bazı mevcut kütüphaneler,
komut satırı argümanlarını kabul eden bir program yazmaya yardımcı olabilir, ancak bu
kavramı yeni öğreniyorsunuz, bu yüzden bu özelliği kendimiz uygulayalım.

### Argüman Değerlerini Okuma

`minigrep`'in kendisine verdiğimiz komut satırı argümanlarının değerlerini okuyabilmesi için,
Rust'un standart kütüphanesinde bulunan `std::env::args` işlevine ihtiyacımız olacak.
Bu işlev, `minigrep`'e verilen komut satırı argümanlarının bir yineleyicisini döndürür.
Yineleyiciler hakkında ayrıntılı bilgiyi [Bölüm 13][ch13]'te ele alacağız<!-- ignore Yineleyicileri [Bölüm 13][ch13]<!-- ignore
-->'da ayrıntılı olarak ele alacağız. Şu anda yineleyiciler hakkında sadece iki ayrıntıyı bilmeniz yeterlidir: yineleyiciler
bir dizi değer üretir ve bir yineleyici üzerinde `collect` yöntemini çağırarak
onu, tüm öğeleri içeren bir vektör gibi bir koleksiyona dönüştürebiliriz.
the iterator produces.

Listing 12-1'deki kod, `minigrep` programınızın kendisine iletilen tüm komut
satırı argümanlarını okumasına ve ardından bu değerleri bir vektörde toplamasına olanak tanır.

<Listing number="12-1" file-name="src/main.rs" caption="Collecting the command line arguments into a vector and printing them">

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

</Listing>

Öncelikle, `use` deyimi ile `std::env` modülünü kapsam içine alıyoruz, böylece
`args` işlevini kullanabiliriz. `std::env::args` işlevinin iki düzeyde modül içinde
iç içe olduğunu unutmayın. [Bölüm 7][ch7-idiomatic-use]<!-- ignore -->'da tartıştığımız gibi, istenen işlevin
birden fazla modülde iç içe olduğu durumlarda, işlev yerine üst modülü
kapsama alanına almayı tercih ettik. Böylece, `std::env`'deki diğer işlevleri
kolayca kullanabiliriz. Ayrıca, `use std::env::args` ekleyip
fonksiyonu sadece `args` ile çağırmaktan daha az belirsizdir, çünkü `args` kolayca
mevcut modülde tanımlanmış bir fonksiyonla karıştırılabilir.

> ### `args` İşlevi ve Geçersiz Unicode
>
> Herhangi bir argüman geçersiz Unicode içeriyorsa `std::env::args` paniğe kapılacaktır.
> Programınız geçersiz Unicode içeren argümanları kabul etmesi gerekiyorsa, bunun yerine
> `std::env::args_os` işlevini kullanın. Bu işlev, `String` değerleri yerine `OsString` değerleri
> üreten bir yineleyici döndürür. Burada basitlik açısından
> `std::env::args` kullanmayı tercih ettik, çünkü `OsString` değerleri platforma göre
> farklılık gösterir ve `String` değerlerine göre kullanımı daha karmaşıktır.

`main`'in ilk satırında `env::args`'ı çağırıyoruz ve hemen ardından
`collect`'i kullanarak yineleyiciyi, yineleyici tarafından üretilen tüm değerleri içeren
bir vektöre dönüştürüyoruz. `collect` işlevini birçok türde
koleksiyon oluşturmak için kullanabiliriz, bu nedenle `args` türünü açıkça belirtiriz ve
dize vektörü istediğimizi belirtiriz. Rust'ta türleri belirtmek çok nadiren gerekse de,
`collect`, Rust'un istediğiniz koleksiyon türünü çıkaramadığı için
sık sık belirtmeniz gereken bir işlevdir.

Son olarak, vektörü hata ayıklama makrosunu kullanarak yazdırıyoruz. Kodu önce
argüman olmadan, sonra iki argümanla çalıştıralım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

Vektördeki ilk değerin `“target/debug/minigrep”` olduğunu ve bunun
ikili dosyamızın adı olduğunu unutmayın. Bu, C'deki argüman listesinin davranışıyla
uyumludur ve programların yürütme sırasında çağrıldıkları adları kullanmalarına olanak tanır.
Programın adını mesajlarda yazdırmak veya programı çalıştırmak için kullanılan komut satırı
takma adına göre programın davranışını değiştirmek istediğinizde, program adına erişebilmek
genellikle kullanışlıdır. Ancak bu bölümün amaçları doğrultusunda, bunu göz ardı edip
sadece ihtiyacımız olan iki argümanı kaydedeceğiz.

### Argüman Değerlerini Değişkenlere Kaydetme

Program şu anda komut satırı argümanları olarak belirtilen değerlere erişebilmektedir.
Şimdi, programın geri kalanında bu değerleri kullanabilmek için iki argümanın değerlerini
değişkenlere kaydetmemiz gerekiyor. Bunu Listing
12-2'de yapıyoruz.

<Listing number="12-2" file-name="src/main.rs" caption="Creating variables to hold the query argument and file path argument">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

</Listing>

Vektörü yazdırdığımızda gördüğümüz gibi, programın adı vektörün ilk
değerini `args[0]`'da alır, bu yüzden argümanları 1. indeksden başlatıyoruz.
`minigrep`'in aldığı ilk argüman aradığımız dizidir, bu yüzden ilk argümana
`query` değişkeninde bir referans koyuyoruz. İkinci argüman
dosya yolu olacaktır, bu yüzden ikinci argümana bir referans
`file_path` değişkenine koyuyoruz.

Kodun istediğimiz gibi çalıştığını kanıtlamak için bu değişkenlerin değerlerini geçici olarak yazdırıyoruz.
Bu programı `test`
ve `sample.txt` argümanlarıyla tekrar çalıştıralım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

Harika, program çalışıyor! İhtiyacımız olan argümanların değerleri
doğru değişkenlere kaydediliyor. Daha sonra, kullanıcı argüman sağlamadığında
gibi bazı olası hatalı durumlarla başa çıkmak için hata işleme ekleyeceğiz;
şimdilik bu durumu göz ardı edip bunun yerine dosya okuma yetenekleri eklemeye
odaklanacağız.

[ch13]: ch13-00-functional-features.md
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.md#i̇diomatik-use-yollarının-oluşturulması
