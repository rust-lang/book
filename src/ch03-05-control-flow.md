## Control Flow

Bir koşulun `doğru` olup olmamasına bağlı olarak bazı kodları çalıştırma ve
Bir koşul `doğru` iken bazı kodları tekrar tekrar çalıştırmak temel yapı taşlarıdır
çoğu programlama dilinde. Kontrol etmenizi sağlayan en yaygın yapılar
Rust kodunun yürütme akışı `if` ifadeleri ve döngülerdir.

### `if` İfadeleri

Bir `if` ifadesi, koşullara bağlı olarak kodunuzu dallara ayırmanıza olanak tanır. Sen
bir koşul belirtin ve ardından "Bu koşul karşılanırsa, bu bloğu çalıştırın
kod bloğu. Koşul karşılanmazsa, bu kod bloğunu çalıştırmayın."

Keşfetmek için _projects_ dizininizde _branches_ adında yeni bir proje oluşturun
if` ifadesini girin. _src/main.rs_ dosyasına aşağıdakileri girin:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Tüm `if` ifadeleri `if` anahtar sözcüğü ile başlar ve ardından bir koşul gelir. İçinde
bu durumda, koşul `sayı` değişkeninin bir
değeri 5'ten küçükse. Koşul aşağıdaki gibi ise çalıştırılacak kod bloğunu yerleştiriyoruz
küme parantezleri içindeki koşuldan hemen sonra `true`. Kod blokları
'if' ifadelerindeki koşullarla ilişkili olanlara bazen _arms_ denir,
tıpkı [“Karşılaştırma”] bölümünde tartıştığımız `match` ifadelerindeki kollar gibi
Gizli Numarayı Tahmin Etme"][gizli-sayıyı-tahmin-etme-karşılaştırması]<!--
Bölüm 2'deki --> bölümünü göz ardı edin.

İsteğe bağlı olarak, bir `else` ifadesi de ekleyebiliriz, ki biz bunu yapmayı seçtik
burada, programa aşağıdaki durumlarda çalıştırılacak alternatif bir kod bloğu vermek için
koşulu `false` olarak değerlendirilir. Eğer bir `else` ifadesi sağlamazsanız ve
koşul `false` ise, program `if` bloğunu atlayacak ve devam edecektir
bir sonraki kod parçasına.

Bu kodu çalıştırmayı deneyin; aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Şimdi `number` değerini koşulu sağlayan bir değere değiştirmeyi deneyelim
Ne olacağını görmek için `false`:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Programı tekrar çalıştırın ve çıktıya bakın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Ayrıca, bu koddaki koşulun _must_ bir `bool` olması gerektiğini de belirtmek gerekir. Eğer
koşul bir `bool` değilse, bir hata alırız. Örneğin, şu komutu çalıştırmayı deneyin
aşağıdaki kod:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

if` koşulu bu kez `3` değerine değerlendirilir ve Rust bir
Hata:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Hata, Rust'ın bir `bool` beklediğini ancak bir tamsayı aldığını gösterir. Aksine
Ruby ve JavaScript gibi dillerde, Rust otomatik olarak
Boolean olmayan türleri Boolean'a dönüştürür. Açık olmalısınız ve her zaman
koşul olarak bir Boolean ile `if`. Eğer `if` kod bloğunun çalışmasını istiyorsak
sadece bir sayı `0`a eşit olmadığında, örneğin, `if`i değiştirebiliriz
aşağıdaki ifadeye:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Bu kodu çalıştırmak `sayı sıfırdan farklı bir şeydi' yazdıracaktır.

#### Birden Fazla Koşulu `else if` ile İşleme

Birden fazla koşulu `if` ve `else` ifadelerini bir `else if` içerisinde birleştirerek kullanabilirsiniz
ifade. Örneğin:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Bu programın izleyebileceği dört olası yol vardır. Çalıştırdıktan sonra şunları yapmalısınız
aşağıdaki çıktıya bakın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Bu program çalıştığında, her `if` ifadesini sırayla kontrol eder ve
koşulun `true` olarak değerlendirildiği ilk gövde. Unutmayın ki
6 sayısı 2'ye bölünebilir olmasına rağmen, `sayı 2'ye bölünebilir' çıktısını görmüyoruz,
ne de `sayı 4, 3 veya 2`ye bölünemez` metnini `else` metninde görmüyoruz
bloğu. Bunun nedeni Rust'ın bloğu yalnızca ilk `true` için çalıştırmasıdır.
koşulunu kullanır ve birini bulduğunda diğerlerini kontrol etmez bile.

Çok fazla `else if` ifadesi kullanmak kodunuzu karmaşıklaştırabilir, bu nedenle daha fazla
birden fazla ise, kodunuzu yeniden düzenlemek isteyebilirsiniz. Bölüm 6'da güçlü bir
Bu durumlar için `match` adı verilen Rust dallanma yapısı.

#### Bir `let` Deyimi İçinde `if` Kullanımı

if` bir ifade olduğu için, onu bir `let` ifadesinin sağ tarafında kullanabiliriz
deyimi ile sonucu bir değişkene atamak için, Liste 3-2'de olduğu gibi.

<Listing number="3-2" file-name="src/main.rs" caption="Assigning the result of an `if` expression to a variable">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

Sayı` değişkeni, `if` değişkeninin sonucuna bağlı olarak bir değere bağlanacaktır.
ifade. Ne olduğunu görmek için bu kodu çalıştırın:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Kod bloklarının içlerindeki son ifadeye göre değerlendirildiğini unutmayın ve
sayılar da kendi başlarına birer ifadedir. Bu durumda, sayının değeri
tüm `if` ifadesi hangi kod bloğunun yürütüleceğine bağlıdır. Bu, şu anlama gelir
'if'in her bir kolundan sonuç olma potansiyeline sahip değerler aşağıdaki gibi olmalıdır
Liste 3-2'de hem `if` kolunun hem de `else` kolunun sonuçları aynı tiptedir.
kolu `i32` tamsayılarıydı. Türler uyumsuzsa, aşağıdaki gibi
örneğinde olduğu gibi bir hata alırız:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Bu kodu derlemeye çalıştığımızda bir hata alacağız. if` ve `else` kolları
uyumsuz değer türlerine sahiptir ve Rust tam olarak nerede
Programdaki sorunu bulun:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

if` bloğundaki ifade bir tamsayı olarak değerlendirilir ve
else` bloğu bir string olarak değerlendirilir. Bu işe yaramaz çünkü değişkenler
tek bir türe sahiptir ve Rust'ın derleme zamanında
değişkeni kesin olarak `sayı`dır. Sayı` değişkeninin türünü bilmek
derleyici `number` kullandığımız her yerde türün geçerli olduğunu doğrular. Pas olmazdı
'sayı'nın türü yalnızca çalışma zamanında belirleniyorsa bunu yapabilirdi
derleyici daha karmaşık olacak ve kod hakkında daha az garanti verecektir.
herhangi bir değişken için birden fazla varsayımsal türü takip etmek zorunda kalırsa.

### Döngülerle Tekrarlama

Bir kod bloğunu birden fazla kez çalıştırmak genellikle yararlıdır. Bu görev için,
Rust, döngü içindeki kod boyunca çalışacak birkaç _loops_ sağlar
vücudu sonuna kadar çalıştırın ve ardından hemen baştan başlayın. Denemek için
döngüler ile _loops_ adında yeni bir proje yapalım.

Rust'ta üç çeşit döngü vardır: `loop`, `while` ve `for`. Her birini deneyelim.

#### `loop` ile Kod Tekrarı

Loop` anahtar sözcüğü Rust'a bir kod bloğunu tekrar tekrar çalıştırmasını söyler
sonsuza kadar veya siz açıkça durmasını söyleyene kadar.

Örnek olarak, _loops_ dizininizdeki _src/main.rs_ dosyasını şu şekilde değiştirin
Bunun gibi:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

When we run this program, we’ll see `again!` printed over and over continuously
until we stop the program manually. Most terminals support the keyboard shortcut
<kbd>ctrl</kbd>-<kbd>c</kbd> to interrupt a program that is stuck in a continual
loop. Give it a try:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

The symbol `^C` represents where you pressed <kbd>ctrl</kbd>-<kbd>c</kbd>.

You may or may not see the word `again!` printed after the `^C`, depending on
where the code was in the loop when it received the interrupt signal.

Fortunately, Rust also provides a way to break out of a loop using code. You
can place the `break` keyword within the loop to tell the program when to stop
executing the loop. Recall that we did this in the guessing game in the
[“Quitting After a Correct Guess”][quitting-after-a-correct-guess]<!-- ignore
--> section of Chapter 2 to exit the program when the user won the game by
guessing the correct number.

We also used `continue` in the guessing game, which in a loop tells the program
to skip over any remaining code in this iteration of the loop and go to the
next iteration.

#### Döngülerden Değer Döndürme

Döngünün kullanım alanlarından biri, başarısız olabileceğini bildiğiniz bir işlemi yeniden denemektir, örneğin
Bir iş parçacığının işini tamamlayıp tamamlamadığını kontrol etmek gibi. Ayrıca aşağıdakileri de geçirmeniz gerekebilir
Bu işlemin sonucunu döngüden çıkarıp kodunuzun geri kalanına aktarın. Yapmak için
Bunu yaptığınızda, döndürülmesini istediğiniz değeri `break` ifadesinden sonra ekleyebilirsiniz.
döngüyü durdurmak için kullanın; bu değer döngü dışında döndürülecektir, böylece
burada gösterildiği gibi kullanın:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Döngüden önce, `counter` adında bir değişken bildiriyoruz ve bunu
`0`. Daha sonra `result` adında bir değişken tanımlayarak bu değişkenden dönen değeri tutuyoruz.
döngü. Döngünün her iterasyonunda `counter` değişkenine `1` ekliyoruz,
ve sonra `sayı`nın `10`a eşit olup olmadığını kontrol ediyoruz. Eşit olduğu zaman
break` anahtar sözcüğünü `counter * 2` değeriyle değiştiriyoruz. Döngüden sonra, bir
'a değer atayan ifadeyi sonlandırmak için noktalı virgül. Son olarak, biz
Bu durumda `20` olan `result` içindeki değeri yazdırır.

Bir döngünün içinden de `return` yapabilirsiniz. break` sadece mevcut döngüden çıkarken
döngüsünde, `return` her zaman geçerli işlevden çıkar.

#### Birden Fazla Döngü Arasındaki Anlamı Belirsizleştirmek için Döngü Etiketleri

Döngü içinde döngüleriniz varsa, `break` ve `continue` en içteki döngüye uygulanır.
o noktada döngü. İsteğe bağlı olarak bir döngü üzerinde bir _loop label_ belirtebilirsiniz.
daha sonra bu anahtar sözcükleri belirtmek için `break` veya `continue` ile birlikte kullanabilirsiniz
en içteki döngü yerine etiketli döngüye uygulanır. Döngü etiketleri şöyle başlamalıdır
tek bir tırnak işareti ile. İşte iki iç içe döngü içeren bir örnek:


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Dış döngü `‘counting_up’ etiketine sahiptir ve 0'dan 2'ye kadar sayar.
Etiketsiz iç döngü 10'dan 9'a kadar geri sayar. İlk `break`
bir etiket belirtmezse yalnızca iç döngüden çıkacaktır. Break
'counting_up;` deyimi dış döngüden çıkacaktır. Bu kod yazdırılır:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### `while` ile Koşullu Döngüler

Bir programın genellikle bir döngü içinde bir koşulu değerlendirmesi gerekecektir. Bu sırada
koşul `doğru` ise, döngü çalışır. Koşul `doğru` olmaktan çıktığında
program `break` çağrısı yaparak döngüyü durdurur. Davranışı uygulamak mümkündür
Bunun gibi `loop`, `if`, `else` ve `break` kombinasyonlarını kullanarak
İsterseniz bunu şimdi bir programda deneyin. Ancak, bu model o kadar yaygındır ki
Rust bunun için `while` döngüsü adı verilen yerleşik bir dil yapısına sahiptir. İçinde
Liste 3-3, programı üç kez döngüye sokmak için `while` kullanırız ve her seferinde geri sayarız
zaman ve sonra, döngüden sonra bir mesaj yazdırın ve çıkın.

<Listing number="3-3" file-name="src/main.rs" caption="Using a `while` loop to run code while a condition evaluates to `true`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

Bu yapı, aşağıdakileri kullanırsanız gerekli olacak birçok iç içe geçmeyi ortadan kaldırır
`loop`, `if`, `else` ve `break`, ve daha net. Bir koşul iken
doğru` olarak değerlendirilirse kod çalışır; aksi takdirde döngüden çıkılır.

#### `for` ile Bir Koleksiyonda Döngü Oluşturma

öğeleri üzerinde döngü oluşturmak için `while` yapısını kullanmayı seçebilirsiniz.
koleksiyonu, örneğin bir dizi. Örneğin, Liste 3-4'teki döngü her bir
'a' dizisindeki eleman.

<Listing number="3-4" file-name="src/main.rs" caption="Looping through each element of a collection using a `while` loop">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

Burada kod, dizideki öğeler boyunca sayar. Dizinde başlar
`0`, ve sonra dizideki son indise ulaşana kadar döngüye girer (yani,
index < 5` artık `true` olmadığında). Bu kodu çalıştırmak her
dizideki eleman:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Beklendiği gibi, beş dizi değerinin tümü terminalde görünür. index` olsa bile
bir noktada `5` değerine ulaşırsa, döngü denemeden önce yürütmeyi durdurur
diziden altıncı bir değer almak için.

Ancak, bu yaklaşım hataya açıktır; aşağıdaki durumlarda programın paniklemesine neden olabiliriz
dizin değeri veya test koşulu yanlışsa. Örneğin, eğer değiştirdiyseniz
'a' dizisinin tanımını dört elemanlı olacak şekilde değiştirmiş ancak
koşulunu `while index < 4` olarak değiştirirseniz, kod panikleyecektir. Ayrıca yavaş, çünkü
derleyici, çalışma zamanı kodunu ekleyerek koşullu kontrolü gerçekleştirir.
döngü boyunca her yinelemede dizinin sınırları içinde olup olmadığını kontrol eder.

Daha özlü bir alternatif olarak, bir `for` döngüsü kullanabilir ve bazı kodlar çalıştırabilirsiniz
bir koleksiyondaki her öğe için. Bir `for` döngüsü Liste 3-5'teki koda benzer.

<Listing number="3-5" file-name="src/main.rs" caption="Looping through each element of a collection using a `for` loop">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

Bu kodu çalıştırdığımızda, Liste 3-4'teki ile aynı çıktıyı göreceğiz. Daha fazla
daha da önemlisi, artık kodun güvenliğini artırdık ve
Dizinin sonunu aşmaktan veya aşmamaktan kaynaklanabilecek hata olasılığı
yeterince ileri gitmek ve bazı öğeleri kaçırmak. for`dan üretilen makine kodu
döngüler de daha verimli olabilir, çünkü indeksin
her yinelemede dizinin uzunluğuyla karşılaştırılır.

for` döngüsünü kullanarak, aşağıdaki durumlarda başka herhangi bir kodu değiştirmeyi hatırlamanız gerekmez
yönteminde yaptığınız gibi, dizideki değer sayısını değiştirdiniz
Liste 3-4'te kullanılmıştır.

for` döngülerinin güvenliği ve kısalığı, onları en yaygın kullanılan döngü haline getirir
Rust'ta yapı. Bazı kodları çalıştırmak istediğiniz durumlarda bile
belirli sayıda kez, `while` döngüsü kullanan geri sayım örneğinde olduğu gibi
Listeleme 3-3'te, çoğu Rustacean bir `for` döngüsü kullanacaktır. Bunu yapmanın yolu
standart kütüphane tarafından sağlanan bir `Range` kullanmak olacaktır.
bir sayıdan başlayıp diğerinden önce biten sıradaki tüm sayılar
sayı.

Bir `for` döngüsü ve başka bir yöntem kullanarak geri sayımın nasıl görüneceği aşağıda verilmiştir
henüz konuşmadık, `rev`, aralığı tersine çevirmek için:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Bu kod biraz daha güzel, değil mi?

## Özet

Başardınız! Bu oldukça büyük bir bölümdü: değişkenler, skaler
ve bileşik veri türleri, fonksiyonlar, yorumlar, `if` ifadeleri ve döngüler! için
Bu bölümde tartışılan kavramlarla pratik yapmak için, aşağıdaki programları oluşturmayı deneyin
aşağıdakileri yapın:

- Sıcaklıkları Fahrenheit ve Celsius arasında dönüştürün.
- n*'inci Fibonacci sayısını oluşturun.
- Noel şarkısı “Noel'in On İki Günü ”nün sözlerini yazdırın.
  Şarkıdaki tekrardan yararlanarak.

Devam etmeye hazır olduğunuzda, Rust'ta bir kavram hakkında konuşacağız.
diğer programlama dillerinde yaygın olarak bulunur: sahiplik.


[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]: ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess