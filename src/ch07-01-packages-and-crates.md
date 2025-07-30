## Paketler ve Kratlar

Modül sisteminin ele alacağımız ilk bölümleri paketler ve kratlardır.

Bir _krat_, Rust derleyicisinin bir seferde ele aldığı en küçük kod parçasıdır.
`cargo` yerine `rustc` komutunu çalıştırıp tek bir kaynak kod dosyası
geçirseniz bile (aynı şekilde, “Rust Programı Yazma ve Çalıştırma” başlıklı
bölümünde yaptığımız gibi), derleyici bu dosyayı bir kutu olarak değerlendirir. Kutular modüller içerebilir
ve modüller, ilerleyen bölümlerde göreceğimiz gibi, kutu ile birlikte derlenen diğer dosyalarda tanımlanabilir.


Bir kutu iki şekilde olabilir: ikili kutu veya kütüphane kutusu.
_İkili crate'ler_, komut satırı programı veya sunucu gibi çalıştırılabilir bir dosyaya derleyebileceğiniz programlardır.
Her birinin, çalıştırılabilir dosya çalıştırıldığında ne olacağını tanımlayan
`main` adlı bir işlevi olmalıdır. Şimdiye kadar oluşturduğumuz tüm crate'ler
ikili crate'lerdi.

_Kütüphane crate'leri_ `main` işlevine sahip değildir ve çalıştırılabilir bir dosyaya derlenmezler.
Bunun yerine, Bunun yerine, birden fazla projeyle paylaşılmak üzere tasarlanmış işlevsellikleri tanımlarlar.
Örneğin, [Bölüm 2][rand]<!-- ignore -->'da kullandığımız `rand` crate, rastgele sayılar üreten bir işlevsellik sağlar.
Rustaceans “crate” dediğinde, çoğu zaman kütüphane crate'i kastederler ve
‘crate’ kelimesini genel programlama kavramı olan “kütüphane” ile eşanlamlı olarak kullanırlar.

_Crate kökü_, Rust derleyicisinin başlangıç noktası olan ve
crate'inizin kök modülünü oluşturan bir kaynak dosyadır (modülleri [“Kapsamı ve Gizliliği Kontrol Etmek için Modülleri Tanımlama”][modules]<!-- ignore --> bölümünde ayrıntılı olarak açıklayacağız).

_Paket_, bir dizi işlevsellik sağlayan bir veya daha fazla crate'in bir araya getirilmesiyle oluşan bir bundeldir.
Bir paket, Bir paket, bu crate'lerin nasıl derleneceğini açıklayan bir _Cargo.toml_ dosyası içerir.
Cargo, aslında kodunuzu derlemek için kullandığınız komut satırı aracının ikili crate'ini içeren bir pakettir.
Cargo paketi, ikili crate'in bağlı olduğu bir kütüphane crate'i de içerir.
Diğer projeler, Cargo komut satırı aracının kullandığı mantığı kullanmak için Cargo kütüphane crate'ine bağlı olabilir.
Bir paket, istediğiniz kadar ikili crate içerebilir, ancak en fazla bir
kütüphane crate'i içerebilir.

Bir paket istediğiniz kadar ikili kutu içerebilir, ancak en fazla bir
kütüphane kutusu içerebilir. Bir paket, kütüphane veya ikili kutu olsun, en az bir
kutu içermelidir.

Bir paket oluşturduğumuzda neler olduğunu inceleyelim. İlk olarak
`cargo new my-project` komutunu giriyoruz:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

`cargo new my-project` komutunu çalıştırdıktan sonra, Cargo'nun ne oluşturduğunu görmek için `ls` komutunu kullanırız.
Proje dizininde, bize bir paket sağlayan _Cargo.toml_ dosyası vardır.
Ayrıca, _main.rs_ dosyasını içeren _src_ dizini de vardır. Metin düzenleyicinizde _Cargo.toml_ dosyasını açın ve
_src/main.rs_ dosyasına hiçbir şekilde atıfta bulunulmadığını unutmayın. Cargo,
src/main.rs_ dosyasının, paketle aynı ada sahip bir ikili krate'in krate kökü olduğu
konvansiyonunu izler. Benzer şekilde, Cargo, paket dizini
src/lib.rs_ dosyasını içeriyorsa, paketin paketle aynı ada sahip bir kütüphane krate'i
içerdiğini ve src/lib.rs_ dosyasının krate kökü olduğunu bilir. Cargo, kütüphaneyi veya ikili dosyayı derlemek için kutu kök
dosyalarını `rustc`'ye geçirir.

Burada, yalnızca _src/main.rs_ içeren bir paketimiz var, yani yalnızca
`my-project` adlı bir ikili kutu içeriyor. Bir paket _src/main.rs_
ve _src/lib.rs_ içeriyorsa, iki kutuya sahiptir: bir ikili ve bir kütüphane, her ikisi de paketle aynı
adlıdır. Bir paket, dosyaları _src/bin_ dizinine yerleştirerek birden fazla ikili kutuya sahip olabilir: her dosya ayrı bir ikili kutu olacaktır.
Örneğin, aşağıdaki kod bir paket oluşturur:

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.md
[rand]: ch02-00-guessing-game-tutorial.md#rastgele-sayı-oluşturma
