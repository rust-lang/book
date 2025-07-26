## Fonksiyonlar

Fonksiyonlar Rust kodunda yaygındır. Dildeki en
önemli fonksiyonlardan birini zaten gördünüz: birçok programın giriş
noktası olan `main` fonksiyonu. Ayrıca
adresinde yeni fonksiyonlar bildirmenizi sağlayan `fn` anahtar sözcüğünü de gördünüz.

Rust kodu, tüm harflerin küçük olduğu ve alt çizgilerin kelimeleri ayırdığı işlev ve değişken
adları için geleneksel stil olarak _snake case_ kullanır.
İşte örnek bir fonksiyon tanımı içeren bir program:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Rust'ta bir fonksiyonu `fn` yazıp ardından bir fonksiyon adı ve
parantez kümesi girerek tanımlarız. Küme parantezleri
derleyiciye fonksiyonun gövdesinin nerede başlayıp nerede bittiğini söyler.

Tanımladığımız herhangi bir fonksiyonu, adını ve ardından bir dizi parantez
girerek çağırabiliriz. `another_function` program içinde tanımlandığı için,
adresinden `main` fonksiyonunun içinden çağrılabilir. Kaynak kodda `another_function`
_main` fonksiyonundan _sonra_ tanımladığımıza dikkat edin;
adresinden önce de tanımlayabilirdik. Rust, fonksiyonlarınızı nerede tanımladığınızı önemsemez, sadece çağıran tarafından görülebilecek bir kapsamda
bir yerde tanımlanmış olmaları yeterlidir.

Fonksiyonları
daha fazla keşfetmek için _functions_ adında yeni bir ikili proje başlatalım. another_function` örneğini _src/main.rs_ içine yerleştirin ve çalıştırın. aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Satırlar `main` fonksiyonunda göründükleri sırayla çalıştırılır.
Önce “Merhaba, dünya!” mesajı yazdırılır ve ardından `another_function`
çağrılır ve mesajı yazdırılır.

### Parametreler

Fonksiyonları,
fonksiyonun imzasının bir parçası olan özel değişkenler olan _parametrelere_ sahip olacak şekilde tanımlayabiliriz. Bir fonksiyonun parametreleri olduğunda,
bu parametreler için somut değerler sağlayabilirsiniz. Teknik olarak, somut
değerlere _argüman_ denir, ancak günlük konuşmalarda insanlar
_parametre_ ve _argüman_ kelimelerini bir fonksiyonun tanımındaki
değişkenler veya bir
fonksiyonunu çağırdığınızda aktarılan somut değerler için birbirinin yerine kullanma eğilimindedir.

Bu `another_function` versiyonunda bir parametre ekliyoruz:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Bu programı çalıştırmayı deneyin; aşağıdaki çıktıyı almalısınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

`another_function` bildiriminin `x` adında bir parametresi vardır. `x` parametresinin türü `i32` olarak belirtilmiştir. another_function` fonksiyonuna `5` değerini verdiğimizde,
`println!` makrosu `x` değerini içeren küme parantezi çiftinin
biçim dizesinde bulunduğu yere `5` değerini koyar.

İşlev imzalarında, her parametrenin türünü _must_ bildirmeniz gerekir. Bu
Rust'ın tasarımındaki kasıtlı bir karardır:
tanımlarında tür ek açıklamalarını zorunlu kılmak
derleyicinin hangi türü kastettiğinizi anlamak için kodu başka bir yerde kullanmanıza neredeyse hiç ihtiyaç duymayacağı anlamına gelir. Ayrıca derleyici, fonksiyonun hangi tipleri beklediğini bilirse
adresine daha yararlı hata mesajları verebilir.

Birden fazla parametre tanımlarken, parametre bildirimlerini
virgülle ayırın, aşağıdaki gibi:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Bu örnek, iki
parametresi olan `print_labeled_measurement` adında bir fonksiyon oluşturur. İlk parametre `value` olarak adlandırılır ve bir `i32`dir. İkincisi
`unit_label` olarak adlandırılır ve `char` türündedir. Fonksiyon daha sonra
hem `value` hem de `unit_label` içeren metni yazdırır.

Şimdi bu kodu çalıştırmayı deneyelim. Şu anda _functions_
projenizin _src/main.rs_ dosyasında bulunan programı yukarıdaki örnekle değiştirin ve `cargo
run` kullanarak çalıştırın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Fonksiyonu `value` değeri olarak `5` ve `unit_label` değeri olarak
`'h'` ile çağırdığımız için, program çıktısı bu değerleri içerir.

### İfadeler ve İfadeler

Fonksiyon gövdeleri, isteğe bağlı olarak bir
ifadesiyle biten bir dizi ifadeden oluşur. Şimdiye kadar ele aldığımız fonksiyonlar bir son
ifade içermiyordu, ancak bir ifadenin parçası olarak bir ifade gördünüz. Rust ifade tabanlı bir dil olduğu için, bu
anlamak için önemli bir ayrımdır. Diğer dillerde aynı ayrımlar yoktur, bu nedenle
deyimlerin ve ifadelerin ne olduğuna ve farklılıklarının fonksiyonların gövdelerini
nasıl etkilediğine bakalım.

- İfadeler
 bir değer döndürmeyen bazı eylemleri gerçekleştiren talimatlardır.
- İfadeler bir sonuç değerine göre değerlendirilir.

Şimdi bazı örneklere bakalım.

Aslında zaten deyimleri ve ifadeleri kullandık. Bir değişken oluşturmak ve
`let` anahtar sözcüğü ile ona bir değer atamak bir deyimdir. Liste 3-1'de,
`let y = 6;` bir deyimdir.

<Listing number="3-1" file-name="src/main.rs" caption="A `main` function declaration containing one statement">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

Fonksiyon tanımları da ifadelerdir; önceki örneğin tamamı kendi içinde bir
ifadesidir. (Aşağıda göreceğimiz gibi, bir fonksiyonu _çağırmak_ bir
deyimi değildir).

Deyimler değer döndürmez. Bu nedenle, aşağıdaki kodun yapmaya çalıştığı gibi, bir `let` deyimini
başka bir değişkene atayamazsınız; hata alırsınız:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Bu programı çalıştırdığınızda alacağınız hata şuna benzer:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

`let y = 6` deyimi bir değer döndürmez, bu nedenle
`x` için bağlanacak bir şey yoktur. Bu,
C ve Ruby gibi atamanın atamanın değerini döndürdüğü diğer dillerde olanlardan farklıdır. Bu
dillerinde, `x = y = 6` yazabilir ve hem `x` hem de `y` değerinin
`6` değerine sahip olmasını sağlayabilirsiniz; Rust'ta durum böyle değildir.

İfadeler bir değere göre değerlendirilir ve
Rust'ta yazacağınız kodun geri kalanının çoğunu oluşturur. Örneğin `5 + 6` gibi bir matematik işlemini düşünün; bu, `11` değerine değerlendirilen bir
ifadesidir. İfadeler
ifadelerinin bir parçası olabilir: Liste 3-1'de, `let y = 6;` ifadesindeki `6`, `6` değerine değerlendirilen bir
ifadesidir. Bir fonksiyonun çağrılması bir
ifadesidir. Makro çağırmak bir ifadedir. Örneğin,
küme parantezleriyle oluşturulan yeni bir kapsam bloğu bir ifadedir:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

This expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

bu durumda `4` olarak değerlendirilen bir bloktur. Bu değer `let` deyiminin bir parçası olarak `y`
adresine bağlanır. Şimdiye kadar gördüğünüz satırların çoğundan farklı olarak, `x + 1` satırının sonunda
noktalı virgül bulunmadığına dikkat edin.
İfadeler noktalı virgül içermez. Bir ifadenin
sonuna noktalı virgül eklerseniz, ifadeyi bir deyime dönüştürürsünüz ve bu durumda
değeri döndürmez. Fonksiyon geri dönüş değerlerini ve ifadeleri keşfederken bunu aklınızda tutun
sonraki.

### Dönüş Değerleri Olan Fonksiyonlar

Fonksiyonlar kendilerini çağıran koda değer döndürebilirler. Geri dönen
değerlerine isim vermeyiz, ancak türlerini bir oktan (`->`) sonra bildirmeliyiz. Rust'ta, fonksiyonun
dönüş değeri, bir fonksiyonun gövdesinin bloğundaki son
ifadesinin değeriyle eş anlamlıdır. Bir
fonksiyonundan `return` anahtar sözcüğünü kullanarak ve bir değer belirterek erken dönebilirsiniz, ancak çoğu
fonksiyonu son ifadeyi örtük olarak döndürür. İşte bir değer döndüren bir
fonksiyonu örneği:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

`five`
fonksiyonunda fonksiyon çağrıları, makrolar ve hatta `let` deyimleri bile yoktur; sadece `5` sayısı vardır. Bu,
Rust'ta mükemmel şekilde geçerli bir fonksiyondur. Fonksiyonun dönüş tipinin de `-> i32` olarak belirtildiğine dikkat edin. Bu kodu
çalıştırmayı deneyin; çıktı aşağıdaki gibi görünmelidir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

`five` içindeki `5` fonksiyonun dönüş değeridir, bu yüzden
dönüş tipi `i32`dir. Bunu daha ayrıntılı olarak inceleyelim. İki önemli nokta var:
ilk olarak, `let x = five();` satırı, bir değişkeni başlatmak için bir
fonksiyonunun geri dönüş değerini kullandığımızı gösteriyor. Çünkü `five` fonksiyonu bir `5` döndürür,
bu satır aşağıdakiyle aynıdır:

```rust
let x = 5;
```

İkinci olarak, `five` fonksiyonunun parametresi yoktur ve
dönüş değerinin türünü tanımlar, ancak fonksiyonun gövdesi noktalı virgül
olmadan yalnız bir `5`tir, çünkü değerini döndürmek istediğimiz bir ifadedir.

Başka bir örneğe bakalım:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Bu kodu çalıştırmak `X'in değeri: 6`. Ancak `x + 1` ifadesini içeren satırın sonuna bir
noktalı virgül koyarsak, bunu bir
ifadesinden bir deyime dönüştürürsek, bir hata alırız:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Bu kodun derlenmesi aşağıdaki gibi bir hata üretir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

Ana hata mesajı olan `uyumsuz tipler`, bu
kodundaki temel sorunu ortaya koymaktadır. plus_one` fonksiyonunun tanımı bir
`i32` döndüreceğini söyler, ancak ifadeler `()` ile ifade edilen bir değere değerlendirilmez,
birim türü. Bu nedenle, hiçbir şey döndürülmez, bu da
işlev tanımıyla çelişir ve bir hataya neden olur. Bu çıktıda Rust,
adresine bu sorunu düzeltmeye yardımcı olabilecek bir mesaj sunar: noktalı virgülün kaldırılmasını önerir, bu da
hatayı düzeltir.