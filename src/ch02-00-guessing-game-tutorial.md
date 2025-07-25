# Programlama Bir Tahmin Oyunu

Birlikte uygulamalı bir proje üzerinde çalışarak Rust'a atlayalım! Bu
bölüm size
gerçek bir programda nasıl kullanacağınızı göstererek birkaç yaygın Rust kavramını tanıtıyor. let`, `match`, metotlar, ilişkili
fonksiyonları, harici crate`ler ve daha fazlası hakkında bilgi edineceksiniz! İlerleyen bölümlerde
bu fikirleri daha ayrıntılı olarak inceleyeceğiz. Bu bölümde, sadece
temellerini uygulayacaksınız.

Klasik bir başlangıç programlama problemini uygulayacağız: bir tahmin oyunu. İşte
nasıl çalıştığı: program 1 ile 100 arasında rastgele bir tamsayı üretecek. Daha sonra
oyuncudan bir tahmin girmesini isteyecektir. Tahmin girildikten sonra,
programı tahminin çok düşük mü yoksa çok yüksek mi olduğunu gösterecektir. Tahmin
doğru ise, oyun bir tebrik mesajı yazdıracak ve çıkacaktır.

## Yeni Bir Proje Oluşturma

Yeni bir proje oluşturmak için
Bölüm 1'de oluşturduğunuz _projects_ dizinine gidin ve Cargo kullanarak aşağıdaki gibi yeni bir proje oluşturun:

```console
$ cargo new guessing_game
$ cd guessing_game
```

The first command, `cargo new`, takes the name of the project (`guessing_game`)
as the first argument. The second command changes to the new project’s
directory.

Look at the generated _Cargo.toml_ file:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Bölüm 1'de gördüğünüz gibi, `cargo new`
için bir “Merhaba, dünya!” programı üretir. _src/main.rs_ dosyasına göz atın:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Şimdi bu “Merhaba, dünya!” programını derleyelim ve aynı adımda
`cargo run` komutunu kullanarak çalıştıralım:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

Bir proje üzerinde hızlı bir şekilde yineleme yapmanız gerektiğinde `run` komutu kullanışlıdır
bu oyunda yapacağımız gibi
bir sonrakine geçmeden önce her yinelemeyi hızlı bir şekilde test edin.

_src/main.rs_ dosyasını yeniden açın. Tüm kodu bu dosyaya yazacaksınız.

## Bir Tahminin İşlenmesi

Tahmin oyunu programının ilk bölümü kullanıcı girdisi isteyecek, bu girdiyi işleyecek
ve girdinin beklenen biçimde olup olmadığını kontrol edecektir. Başlamak için,
adresinden oyuncunun bir tahmin girmesine izin vereceğiz. Liste 2-1'deki kodu
_src/main.rs_ adresine girin.

<Listing number="2-1" file-name="src/main.rs" caption="Code that gets a guess from the user and prints it">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

Bu kod çok fazla bilgi içeriyor, bu yüzden satır satır üzerinden geçelim. kullanıcı girdisini almak ve ardından sonucu çıktı olarak yazdırmak için
`io` girdi/çıktı kütüphanesini kapsama almamız gerekir. `io` kütüphanesi, `std` olarak bilinen standart
kütüphanesinden gelmektedir:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Rust varsayılan olarak
her programın kapsamına getirdiği standart kütüphanede tanımlanmış bir dizi öğeye sahiptir. Bu kümeye _prelude_ denir ve
adresinde [standart kütüphane belgelerinde][prelude] her şeyi görebilirsiniz.

Eğer kullanmak istediğiniz bir tip prelude içinde değilse, o tipi
kapsamına bir `use` deyimi ile açıkça getirmeniz gerekir. std::io` kütüphanesini kullanmak
size
kullanıcı girdisini kabul etme yeteneği de dahil olmak üzere bir dizi yararlı özellik sağlar.

Bölüm 1`de gördüğünüz gibi, `main` fonksiyonu
programının giriş noktasıdır:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` sözdizimi yeni bir fonksiyon bildirir; parantezler, `()`,
parametre olmadığını gösterir; ve küme parantezi, `{`, fonksiyonun gövdesini başlatır.

`Bölüm 1`de de öğrendiğiniz gibi, `println!` ekrana
şeklinde bir dize yazdıran bir makrodur:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Bu kod, oyunun ne olduğunu belirten bir bilgi istemi yazdırıyor ve kullanıcıdan
girişini talep ediyor.

### Değerleri Değişkenlerle Saklama

Ardından, kullanıcı girdisini saklamak için aşağıdaki gibi bir _variable_ oluşturacağız:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Şimdi program ilginçleşiyor! Bu küçük
satırında çok şey oluyor. Değişkeni oluşturmak için `let` deyimini kullanıyoruz. İşte başka bir örnek:

```rust,ignore
let apples = 5;
```

Bu satır `apples` adında yeni bir değişken oluşturur ve onu 5 değerine bağlar. Rust'ta değişkenler varsayılan olarak değişmezdir, yani değişkene bir kez
değeri verdiğimizde, değer değişmez. Bu kavramı
Bölüm 3'teki [“Değişkenler ve Değişebilirlik”][variables-and-mutability]<!-- ignore -->
bölümünde ayrıntılı olarak tartışacağız. Bir değişkeni değiştirilebilir yapmak için,
değişken adının önüne `mut` ekleriz:

```rust,ignore
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

> Not: `//` sözdizimi,
> satırının sonuna kadar devam eden bir yorum başlatır. Rust yorumlardaki her şeyi yok sayar. Yorumları [Bölüm 3][comments]<!-- ignore --> bölümünde daha
> ayrıntılı olarak tartışacağız.

Tahmin oyunu programına dönecek olursak, artık `let mut guess` ifadesinin
`guess` adında bir değişebilir değişken tanıtacağını biliyorsunuz. Eşittir işareti (`=`) Rust'a
şimdi değişkene bir şey bağlamak istediğimizi söyler. Eşittir işaretinin sağında
`guess` değişkeninin bağlı olduğu değer yer alır, bu değer
`String::new` çağrısının sonucudur, bu fonksiyon `String` değişkeninin yeni bir örneğini döndürür.
[`String`][string]<!-- ignore -->, standart
kütüphanesi tarafından sağlanan, büyüyebilen, UTF-8 kodlu bir metin parçası olan bir string türüdür.

`::new` satırındaki `::` sözdizimi, `new` öğesinin `String` türünün ilişkili bir
işlevi olduğunu gösterir. Bir _ilişkili işlev_
bir tür üzerinde, bu durumda `String` üzerinde uygulanan bir işlevdir. Bu `new` fonksiyonu
yeni, boş bir string oluşturur. Birçok türde `new` fonksiyonuna rastlarsınız, çünkü bu
bir tür yeni değer oluşturan bir fonksiyonun ortak adıdır.

Tam olarak, `let mut guess = String::new();` satırı, şu anda bir `String`in yeni, boş bir örneğine bağlı olan değiştirilebilir bir
değişkeni oluşturmuştur. Vay canına!

### Kullanıcı Girdisi Alma

Programın ilk satırında `use std::io;` ile standart
kütüphanesinden giriş/çıkış işlevselliğini dahil ettiğimizi hatırlayın. Şimdi
adresine `io` modülünden `stdin` fonksiyonunu çağıracağız, bu da kullanıcı
girdisini işlememizi sağlayacak:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Eğer
programının başında `use std::io;` ile `io` modülünü içe aktarmamış olsaydık, bu fonksiyon çağrısını
`std::io::stdin` şeklinde yazarak fonksiyonu yine de kullanabilirdik. stdin` fonksiyonu bir
[`std::io::Stdin`][iostdin]<!-- ignore --> örneği döndürür, bu da terminaliniz için standart girdiye bir
tanıtıcısını temsil eden bir türdür.

Ardından, `.read_line(&mut guess)` satırı, kullanıcıdan girdi almak için standart girdi tanıtıcısında [`read_line`][read_line]<!--
ignore --> yöntemini çağırır.
Ayrıca, kullanıcı girdisini hangi
dizesinde saklayacağını söylemek için `&mut guess` argümanını `read_line` argümanı olarak geçiyoruz. read_line`ın tam görevi, kullanıcı standart girdiye ne yazarsa yazsın
almak ve bunu bir dizeye
 eklemektir (içeriğinin üzerine yazmadan), bu nedenle bu dizeyi bir
argümanı olarak geçiriyoruz. String argümanının değiştirilebilir olması gerekir, böylece metot
string'in içeriğini değiştirebilir.

&' bu argümanın bir _referans_ olduğunu gösterir, bu da size
kodunuzun birden fazla bölümünün
bu verileri birden fazla kez belleğe kopyalamasına gerek kalmadan bir veri parçasına erişmesine izin vermenin bir yolunu sunar. Referanslar karmaşık bir özelliktir,
ve Rust'ın en büyük avantajlarından biri
referanslarını kullanmanın ne kadar güvenli ve kolay olduğudur. Bu
programını bitirmek için bu detayların çoğunu bilmenize gerek yok. Şimdilik bilmeniz gereken tek şey, değişkenler gibi referansların da varsayılan olarak
değişmez olduğudur. Bu nedenle, değişebilir yapmak için
`&guess` yerine `&mut guess` yazmanız gerekir. (Bölüm 4 referansları daha ayrıntılı olarak
açıklayacaktır).

<!-- Old heading. Do not remove or links may break. -->

<a id="handling-potential-failure-with-the-result-type"></a>

### Handling Potential Failure with `Result`

We’re still working on this line of code. We’re now discussing a third line of
text, but note that it’s still part of a single logical line of code. The next
part is this method:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Bu kodu şu şekilde yazabilirdik:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Ancak, uzun bir satırın okunması zordur, bu nedenle en iyisi onu bölmektir. Bir yöntemi `.method_name()` sözdizimiyle çağırdığınızda
uzun
satırlarını bölmeye yardımcı olmak için bir yeni satır ve diğer boşlukları eklemek genellikle akıllıca olur. Şimdi
bu satırın ne yaptığını tartışalım.

Daha önce de belirtildiği gibi, `read_line` kullanıcının girdiği her şeyi kendisine ilettiğimiz
dizesine koyar, ancak aynı zamanda bir `Result` değeri de döndürür. [`Result`][result]<!--
ignore --> bir [_enumeration_][enums]<!-- ignore -->, genellikle _enum_ olarak adlandırılır,
birden fazla olası durumdan birinde olabilen bir türdür. Her
olası duruma bir _variant_ diyoruz.

[Bölüm 6][enumlar]<!-- ignore --> enumları daha ayrıntılı olarak ele alacaktır. Bu `Result` türlerinin amacı
hata işleme bilgilerini kodlamaktır.

`Result` türünün varyantları `Ok` ve `Err`dir. Ok` değişkeni
işleminin başarılı olduğunu gösterir ve başarıyla oluşturulan değeri içerir.
Error` değişkeni işlemin başarısız olduğu anlamına gelir ve işlemin nasıl veya neden başarısız olduğu hakkında
bilgi içerir.

Her tür değer gibi `Result` türündeki değerlerin de
adresinde tanımlanmış yöntemleri vardır. Bir `Result` örneğinin çağırabileceğiniz bir [`expect` yöntemi][expect]<!-- ignore -->
vardır. Bu `Result` örneği bir `Err` değeriyse, `expect`
programın çökmesine neden olur ve `expect` için
argümanı olarak aktardığınız mesajı görüntüler. Eğer `read_line` metodu bir `Err` değeri döndürürse, bu
muhtemelen temel işletim sisteminden gelen bir hatanın sonucu olacaktır.
Bu `Result` örneği bir `Ok` değeriyse, `expect`, `Ok` değerinin tuttuğu
dönüş değerini alacak ve kullanabilmeniz için size yalnızca bu değeri döndürecektir.
Bu durumda, bu değer kullanıcının girdisindeki bayt sayısıdır.

Eğer `expect` komutunu çağırmazsanız, program derlenir, ancak bir uyarı alırsınız:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust, `read_line`dan dönen `Result` değerini kullanmadığınız konusunda uyarır,
programın olası bir hatayı ele almadığını gösterir.

Uyarıyı bastırmanın doğru yolu aslında hata işleme kodu yazmaktır,
ancak bizim durumumuzda sadece bir sorun oluştuğunda bu programı çökertmek istiyoruz, bu yüzden
`expect` kullanabiliriz. Hatalardan kurtarma hakkında [Bölüm
9][recover]<!-- ignore --> bölümünde bilgi edineceksiniz.

### Değerleri `println!` Yer Tutucuları ile Yazdırma

Kapanan küme parantezi dışında,
adresinde şimdiye kadarki kodda tartışılacak yalnızca bir satır daha var:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Bu satır, kullanıcının girdisini içeren dizeyi yazdırır. küme parantezlerinin `{}` kümesi bir yer tutucudur: `{}`ı
bir değeri yerinde tutan küçük yengeç kıskaçları olarak düşünün. Bir değişkenin değerini yazdırırken, değişken adı
küme parantezlerinin içine girebilir. Bir
ifadesinin değerlendirilmesinin sonucunu yazdırırken, biçim dizesine boş küme parantezleri yerleştirin, ardından
biçim dizesini, her boş
küme parantezi yer tutucusuna aynı sırada yazdırılacak ifadelerin virgülle ayrılmış bir listesiyle izleyin. Bir değişkeni ve bir ifadenin sonucunu
tek bir `println!` çağrısında yazdırmak şu şekilde görünür:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

kodu `x = 5 ve y + 2 = 12` yazdıracaktır.

### İlk Bölümün Test Edilmesi

Tahmin oyununun ilk bölümünü test edelim. Cargo run` kullanarak çalıştırın:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

Bu noktada, oyunun ilk kısmı tamamlandı:
klavyesinden girdi alıyoruz ve ardından yazdırıyoruz.

## Gizli Sayı Oluşturma

Daha sonra, kullanıcının tahmin etmeye çalışacağı gizli bir sayı oluşturmamız gerekir. gizli numarası her seferinde farklı olmalıdır, böylece oyun bir kereden fazla
oynamak için eğlenceli olur. Oyunun çok
zor olmaması için 1 ile 100 arasında rastgele bir sayı kullanacağız. Rust henüz standart
kütüphanesinde rastgele sayı işlevselliğini içermiyor. Bununla birlikte, Rust ekibi
söz konusu işlevselliğe sahip bir [`rand` crate][randcrate] sağlamaktadır.

### Daha Fazla İşlevsellik Elde Etmek İçin Bir Sandık Kullanmak

Bir crate'in Rust kaynak kodu dosyalarının bir koleksiyonu olduğunu unutmayın. Oluşturmakta olduğumuz
projesi bir _binary crate_, yani çalıştırılabilir bir dosyadır. rand`
sandığı,
diğer programlarda kullanılması amaçlanan ve kendi başına çalıştırılamayan kodu içeren bir _library crate_'dir.

Cargo'nun harici crate'leri koordine etmesi, Cargo'nun gerçekten parladığı yerdir. adresinde `rand` kullanan bir kod yazmadan önce,
adresindeki_Cargo.toml_ dosyasını `rand` crate'ini bir bağımlılık olarak içerecek şekilde değiştirmemiz gerekiyor. Şimdi bu dosyayı açın ve
aşağıdaki satırı,
Cargo'nun sizin için oluşturduğu `[dependencies]` bölüm başlığının altına ekleyin. rand`ı tam olarak burada belirttiğimiz gibi,
bu sürüm numarasıyla belirttiğinizden emin olun, aksi takdirde bu eğitimdeki kod örnekleri çalışmayabilir:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

Cargo.toml_ dosyasında, bir başlığı takip eden her şey, başka bir bölüm başlayana kadar devam eden
bölümünün bir parçasıdır. dependencies]` içinde
Cargo'ya projenizin hangi harici sandıklara bağlı olduğunu ve bu sandıkların
hangi sürümlerine ihtiyaç duyduğunuzu söylersiniz. Bu durumda,
semantik sürüm belirteci `0.8.5` ile `rand` sandığını belirtiriz. Cargo, sürüm numaralarını yazmak için bir
standardı olan [Semantic
Versioning][semver]<!-- ignore --> (bazen _SemVer_ olarak da adlandırılır) standardını anlar. 0.8.5` belirteci aslında `^0.8.5` için
kısaltmasıdır, bu da en az 0.8.5 olan ancak
0.9.0'ın altında olan herhangi bir sürüm anlamına gelir.

Cargo bu sürümleri
0.8.5 sürümü ile uyumlu genel API'lere sahip olarak kabul eder ve bu belirtim,
'un bu bölümdeki kodla derlemeye devam edeceği en son yama sürümünü almanızı sağlar. Herhangi bir 0.9.0 veya üzeri sürümün
aşağıdaki örneklerde kullanılan API ile aynı API'ye sahip olacağı garanti edilmez.

Şimdi, hiçbir kodu değiştirmeden,
Liste 2-2'de gösterildiği gibi projeyi derleyelim.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="The output from running `cargo build` after adding the rand crate as a dependency">

```console
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

</Listing>

Farklı sürüm numaraları (ancak SemVer sayesinde hepsi
koduyla uyumlu olacaktır!) ve farklı satırlar (işletim
sistemine bağlı olarak) görebilirsiniz ve satırlar farklı bir sırada olabilir.

Harici bir bağımlılık eklediğimizde, Cargo
'un en son sürümlerini, bağımlılığın ihtiyaç duyduğu her şeyi,
verilerinin bir kopyası olan _registry_'den [Crates.io][cratesio] alır. Crates.io, Rust ekosistemindeki kişilerin
açık kaynaklı Rust projelerini başkalarının kullanması için yayınladıkları yerdir.

Kayıt defterini güncelledikten sonra Cargo, `[dependencies]` bölümünü kontrol eder ve
adresinde listelenen ve halihazırda indirilmemiş olan tüm crate'leri indirir. Bu durumda,
sadece `rand`ı bir bağımlılık olarak listelememize rağmen, Cargo ayrıca `rand`ın çalışmak için bağlı olduğu diğer crate`leri de
yakaladı. Sandıkları indirdikten sonra Rust
bunları derler ve ardından projeyi mevcut bağımlılıklarla derler.

Herhangi bir değişiklik yapmadan hemen tekrar `cargo build` çalıştırırsanız,
`Finished` satırı dışında herhangi bir çıktı almazsınız. Cargo zaten
bağımlılıkları indirdiğini ve derlediğini ve
dosyanızda onlar hakkında hiçbir şey değiştirmediğinizi bilir. Cargo ayrıca
kodunuzla ilgili hiçbir şeyi değiştirmediğinizi de bilir, bu yüzden onu da yeniden derlemez. yapacak bir şey olmadığından, basitçe çıkar.

Eğer _src/main.rs_ dosyasını açar, önemsiz bir değişiklik yapar ve sonra kaydedip
adresinden tekrar derlerseniz, sadece iki satır çıktı görürsünüz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

Bu satırlar Cargo'nun derlemeyi yalnızca
_src/main.rs_ dosyasında yaptığınız küçük değişiklikle güncellediğini gösteriyor. Bağımlılıklarınız değişmedi, bu nedenle Cargo
adresinden daha önce indirip derlediklerini yeniden kullanabileceğini biliyor.

#### _Cargo.lock_ Dosyası ile Tekrarlanabilir Derlemeler Sağlama

Cargo, siz veya bir başkası kodunuzu her derlediğinde aynı eseri yeniden oluşturabilmenizi sağlayan bir mekanizmaya sahiptir: Cargo, siz aksini belirtmedikçe yalnızca belirttiğiniz
bağımlılıklarının sürümlerini kullanacaktır. Örneğin,
gelecek hafta `rand` crate'inin 0.8.6 sürümünün çıktığını ve
sürümünün önemli bir hata düzeltmesi içerdiğini, ancak aynı zamanda
kodunuzu bozacak bir regresyon içerdiğini varsayalım. Bunun üstesinden gelmek için Rust, `cargo build` programını ilk çalıştırdığınızda
_Cargo.lock_ dosyasını oluşturur, bu nedenle şu anda _guessing_game_
dizininde buna sahibiz.

Bir projeyi ilk kez oluşturduğunuzda, Cargo kriterlere uyan bağımlılıkların tüm sürümlerini
bulur ve ardından bunları
_Cargo.lock_ dosyasına yazar. Gelecekte projenizi oluşturduğunuzda, Cargo
_Cargo.lock_ dosyasının var olduğunu görecek ve sürümleri tekrar bulmak için tüm işi yapmak yerine orada belirtilen sürümleri
kullanacaktır. Bu sayede
otomatik olarak yeniden üretilebilir bir yapıya sahip olursunuz. Başka bir deyişle, projeniz
_Cargo.lock_ dosyası sayesinde siz açıkça yükseltme yapana kadar 0.8.5 sürümünde kalacaktır.
_Cargo.lock_ dosyası tekrarlanabilir derlemeler için önemli olduğundan, genellikle
projenizdeki kodun geri kalanıyla birlikte kaynak kontrolüne kontrol edilir

#### Yeni Bir Sürüm Almak için Bir Sandığı Güncelleme

Bir sandığı güncellemek istediğinizde Cargo, _Cargo.lock_ dosyasını yok sayacak ve _Cargo.toml_ dosyasındaki spesifikasyonlarınıza uyan
en son sürümleri bulacak olan
`update` komutunu sağlar. Cargo daha sonra bu
sürümlerini _Cargo.lock_ dosyasına yazacaktır. Bu durumda, Cargo yalnızca 0.8.5'ten büyük ve 0.9.0'dan küçük
sürümlerini arayacaktır. Eğer `rand` crate'i
iki yeni sürümü 0.8.6 ve 0.9.0'ı yayınladıysa,
`cargo update'i çalıştırdığınızda aşağıdakileri görürsünüz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.9.0)
```

Cargo 0.9.0 sürümünü yok sayar. Bu noktada, _Cargo.lock_ dosyanızda
şu anda
kullandığınız `rand` crate sürümünün 0.8.6 olduğunu belirten bir değişiklik fark edeceksiniz. rand` sürüm 0.9.0 veya 0.9._x_
serisindeki herhangi bir sürümü kullanmak için, _Cargo.toml_ dosyasını bu şekilde görünecek şekilde güncellemeniz gerekir:

```toml
[dependencies]
rand = "0.9.0"
```

Bir sonraki `cargo build` çalıştırmanızda, Cargo mevcut
crates kayıtlarını güncelleyecek ve `rand` gereksinimlerinizi belirttiğiniz yeni
sürümüne göre yeniden değerlendirecektir.

[Cargo][doccargo]<!-- ignore --> ve [
ekosistemi][doccratesio]<!-- ignore --> hakkında söylenecek daha çok şey var, bunları Bölüm 14'te tartışacağız, ancak
şimdilik bilmeniz gereken tek şey bu. Cargo
kütüphanelerinin yeniden kullanımını çok kolay hale getirir, böylece Rustaceanlar
bir dizi paketten bir araya getirilen daha küçük projeler yazabilirler.

### Rastgele Sayı Oluşturma

Tahmin edilecek bir sayı üretmek için `rand` kullanmaya başlayalım. Bir sonraki adım, Liste 2-3'te gösterildiği gibi
_src/main.rs_ dosyasını güncellemektir.

<Listing number="2-3" file-name="src/main.rs" caption="Adding code to generate a random number">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

İlk olarak `use rand::Rng;` satırını ekliyoruz. Rng` özelliği
rasgele sayı üreteçlerinin uyguladığı yöntemleri tanımlar ve bu yöntemleri
kullanabilmemiz için bu özelliğin kapsam dahilinde olması gerekir. Bölüm 10 özellikleri ayrıntılı olarak ele alacaktır.

Daha sonra, ortaya iki satır ekliyoruz. İlk satırda,
`rand::thread_rng` fonksiyonunu çağırıyoruz ve bu fonksiyon bize kullanacağımız özel rastgele sayı
üretecini veriyor:
yürütmesinin mevcut iş parçacığı için yerel olan ve işletim sistemi tarafından tohumlanan bir fonksiyon. Ardından rastgele sayı üreteci üzerinde `gen_range`
yöntemini çağırıyoruz. Bu yöntem, `use rand::Rng;` deyimiyle kapsam içine aldığımız `Rng`
özelliği tarafından tanımlanır. `gen_range` metodu bir aralık ifadesini argüman olarak alır ve bu aralıkta bir
rastgele sayı üretir. Burada kullandığımız aralık ifadesi türü
`start..=end` biçimini alır ve alt ve üst sınırlarda kapsayıcıdır, bu nedenle
1 ile 100 arasında bir sayı istemek için `1..=100` belirtmemiz gerekir.

> Not: Bir sandıkta hangi özelliklerin kullanılacağını ve hangi yöntem ve işlevlerin
> çağrılacağını bilmeniz yeterli olmayacaktır, bu nedenle her sandık
> kullanım talimatlarını içeren belgelere sahiptir. Cargo'nun bir başka güzel özelliği de `cargo doc
> --open` komutunu çalıştırdığınızda tüm bağımlılıklarınız tarafından sağlanan belgeleri
> yerel olarak oluşturacak ve tarayıcınızda açacaktır. Örneğin, `rand` sandığındaki diğer
> işlevleriyle ilgileniyorsanız, `cargo doc --open` komutunu çalıştırın ve
> soldaki kenar çubuğundaki `rand` seçeneğine tıklayın.

İkinci yeni satır gizli numarayı yazdırır. Bu, programı test edebilmek için
geliştirirken kullanışlıdır, ancak bunu
son sürümünden sileceğiz. Program başlar başlamaz
cevabı yazdırırsa pek de oyun sayılmaz!

Programı birkaç kez çalıştırmayı deneyin:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Farklı rastgele sayılar almalısınız ve bunların hepsi
1 ile 100 arasındaki sayılar olmalıdır. Harika iş!

## Tahmini Gizli Sayı ile Karşılaştırma

Artık elimizde kullanıcı girdisi ve rastgele bir sayı olduğuna göre, bunları karşılaştırabiliriz. Bu adım
Liste 2-4'te gösterilmiştir. adresinde açıklayacağımız gibi, bu kodun henüz derlenmeyeceğini unutmayın.

<Listing number="2-4" file-name="src/main.rs" caption="Handling the possible return values of comparing two numbers">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

İlk olarak başka bir `use` deyimi ekleyerek
`std::cmp::Ordering` adlı bir türü standart kütüphaneden kapsam içine alıyoruz. Ordering` türü
başka bir enumdur ve `Less`, `Greater` ve `Equal` değişkenlerine sahiptir. Bunlar
iki değeri karşılaştırdığınızda mümkün olan üç sonuçtur.

Daha sonra alt kısma `Ordering` türünü kullanan beş yeni satır ekliyoruz. `cmp` yöntemi iki değeri karşılaştırır ve
karşılaştırılabilen herhangi bir şey üzerinde çağrılabilir. Karşılaştırmak istediğiniz şeye bir referans alır: burada
`guess` ile `secret_number` değerlerini karşılaştırıyor. Daha sonra `use` deyimiyle kapsama aldığımız
`Ordering` enumunun bir varyantını döndürür. [`match`][match]<!-- ignore --> ifadesini kullanarak,
`cmp` çağrısından döndürülen `Ordering` değişkeninin
`guess` ve `secret_number` değerleri ile ne yapacağına karar veririz.

Bir `match` ifadesi _arms_'dan oluşur. Bir kol,
ile eşleşecek bir _pattern_ ve `match` ifadesine verilen değerin
bu kolun paternine uyması durumunda çalıştırılması gereken koddan oluşur. Rust, `match` öğesine verilen değeri alır ve sırayla her bir kolun kalıbına
bakar. Kalıplar ve `match` yapısı
güçlü Rust özellikleridir:
kodunuzun karşılaşabileceği çeşitli durumları ifade etmenize izin verir ve hepsini ele aldığınızdan emin olurlar. Bu özellikler
sırasıyla Bölüm 6 ve Bölüm 19'da ayrıntılı olarak ele alınacaktır.

Burada kullandığımız `match` ifadesi ile bir örnek üzerinden gidelim. Diyelim ki
kullanıcı 50 tahmininde bulundu ve bu sefer rastgele oluşturulan gizli sayı
38.

Kod 50 ile 38`i karşılaştırdığında `cmp` metodu
`Ordering::Greater` değerini döndürecektir çünkü 50, 38`den büyüktür. match` ifadesi
adresinden `Ordering::Greater` değerini alır ve her bir kolun desenini kontrol etmeye başlar. İlk kolun kalıbı olan `Ordering::Less` değerine
bakar ve
`Ordering::Greater` değerinin `Ordering::Less` değeriyle eşleşmediğini görür, bu nedenle
adresindeki kodu yok sayar ve bir sonraki kola geçer. Bir sonraki kolun kalıbı
`Ordering::Greater` şeklindedir ve bu kalıp `Ordering::Greater` ile eşleşir! Bu koldaki ilişkili
kodu çalıştırılır ve ekrana `Too big!` yazdırılır. Match`
ifadesi ilk başarılı eşleşmeden sonra sona erer, bu nedenle bu senaryoda son
koluna bakmaz.

Ancak, Liste 2-4'teki kod henüz derlenmeyecektir. Hadi deneyelim:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Hatanın özü _uyumsuz tipler_ olduğunu belirtir. Rust
güçlü, statik bir tip sistemine sahiptir. Bununla birlikte, tip çıkarımına da sahiptir. `let mut guess = String::new()` yazdığımızda, Rust `guess`in
bir `String` olması gerektiği çıkarımını yapabildi ve bize türü yazdırmadı. Diğer
yandan, `secret_number` bir sayı türüdür. Rust'ın sayı türlerinden birkaçı 1
ile 100 arasında bir değere sahip olabilir: `i32`, 32 bitlik bir sayı; `u32`, işaretsiz 32 bitlik bir sayı; `i64`,
64 bitlik bir sayı; ve diğerleri. Aksi belirtilmedikçe, Rust varsayılan olarak
`secret_number` türü olan bir `i32` türünü kullanır, eğer Rust'ın farklı bir sayısal tür çıkarmasına neden olacak tür bilgisini
başka bir yere eklemediyseniz. Hatanın
nedeni, Rust'ın bir string ve bir sayı türünü karşılaştıramamasıdır.

Sonuçta, programın girdi olarak okuduğu `String`i bir
sayı türüne dönüştürmek istiyoruz, böylece onu sayısal olarak gizli sayıyla karşılaştırabiliriz. Bunu
bu satırı `main` fonksiyon gövdesine ekleyerek yapıyoruz:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Hat şu:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Biz `guess` adında bir değişken yaratıyoruz. Ama durun, program zaten
`guess` adında bir değişkene sahip değil mi? Var, ancak Rust bize yardımcı olarak
önceki `guess` değerini yeni bir değerle gölgelememize izin veriyor. _Shadowing_, örneğin
`guess_str` ve `guess` gibi iki benzersiz değişken oluşturmaya zorlamak yerine `guess`
değişken adını yeniden kullanmamızı sağlar. Bu konuyu
[Bölüm 3][shadowing]<!-- ignore --> adresinde daha ayrıntılı olarak ele alacağız, ancak şimdilik bu özelliğin
genellikle bir değeri bir türden başka bir türe dönüştürmek istediğinizde kullanıldığını bilin.

Bu yeni değişkeni `guess.trim().parse()` ifadesine bağlarız. İfadedeki `guess`
,
girişini bir dize olarak içeren orijinal `guess` değişkenini ifade eder. Bir `String` örneği üzerindeki `trim` yöntemi,
dizesini yalnızca sayısal veriler içerebilen bir `u32`ye dönüştürmeden önce yapmamız gereken, başlangıç ve sondaki
boşluklarını ortadan kaldıracaktır. Kullanıcı `read_line` komutunu yerine getirmek için
<kbd>enter</kbd> tuşuna basmalı ve tahminini girmelidir, bu da dizeye bir
satırsonu karakteri ekler. Örneğin, kullanıcı <kbd>5</kbd> yazarsa ve
<kbd>enter</kbd> tuşuna basarsa, `guess` şöyle görünür: `5\n`. `\\n`,
“satırsonu ”nu temsil eder. (Windows'ta <kbd>enter</kbd> tuşuna basıldığında satır başı
ve yeni satır, `\r\n` ile sonuçlanır). `Trim` yöntemi `\n` veya `\r\n` değerlerini eleyerek
adresine sadece `5` değerini verir.

Dizeler üzerinde [`parse` yöntemi][parse]<!-- ignore --> bir dizeyi
başka bir türe dönüştürür. Burada, bir dizeden bir sayıya dönüştürmek için kullanıyoruz. adresine girip Rust'a istediğimiz sayı türünü `let guess: u32` kullanarak söylememiz gerekiyor. Tahmin'den sonra gelen iki nokta üst üste
(`:`) Rust'a değişkenin türüne açıklama ekleyeceğimizi söyler. Rust'ın
birkaç yerleşik sayı türü vardır; burada görülen `u32` işaretsiz, 32 bitlik bir tamsayıdır.
Küçük pozitif bir sayı için iyi bir varsayılan seçimdir. Diğer sayı türlerini
adresinde [Bölüm 3][integers]<!-- ignore --> bölümünde öğreneceksiniz.

Ek olarak, bu örnek programdaki `u32` ek açıklaması ve
ile `secret_number` karşılaştırması, Rust'ın `secret_number'ın da bir
`u32` olması gerektiği sonucunu çıkaracağı anlamına gelir. Yani şimdi karşılaştırma aynı
tipindeki iki değer arasında olacak!

`Parse` metodu sadece mantıksal olarak
sayıya dönüştürülebilen karakterler üzerinde çalışacaktır ve bu nedenle kolayca hatalara neden olabilir. Örneğin,
dizesi `A👍%` içeriyorsa, bunu sayıya dönüştürmenin bir yolu yoktur. başarısız olabileceğinden, `parse` yöntemi, `read_line`
yönteminin yaptığı gibi bir `Result` türü döndürür (daha önce [“Handling Potential Failure with
`Result`”](#handling-potential-failure-with-result)<!-- ignore-->). bu `Sonuç`u yine `expect` yöntemini kullanarak aynı şekilde ele alacağız. Eğer `parse`
,
stringinden bir sayı oluşturamadığı için bir `Err` `Result` varyantı döndürürse, `expect` çağrısı oyunu çökertecek ve verdiğimiz mesajı yazdıracaktır.
Eğer `parse` stringi başarılı bir şekilde sayıya dönüştürebilirse, `Result` değerinin
`Ok` varyantını döndürecek ve `expect` değeri de
adresinden istediğimiz sayıyı döndürecektir.

Şimdi programı çalıştıralım:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
touch src/main.rs
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Güzel! Tahminden önce boşluklar eklenmesine rağmen, program yine de
kullanıcının 76 tahmininde bulunduğunu anladı. Farklı girdi türleriyle
farklı davranışını doğrulamak için programı birkaç kez çalıştırın: sayıyı doğru tahmin edin,
çok yüksek bir sayı tahmin edin ve çok düşük bir sayı tahmin edin.

Şu anda oyunun büyük bir kısmı çalışıyor, ancak kullanıcı yalnızca bir tahmin yapabiliyor.
Bir döngü ekleyerek bunu değiştirelim!

## Döngü ile Birden Fazla Tahmine İzin Verme

Loop` anahtar sözcüğü sonsuz bir döngü oluşturur. Kullanıcılara
sayıyı tahmin etmede daha fazla şans vermek için bir döngü ekleyeceğiz:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Gördüğünüz gibi, tahmin girişi isteminden itibaren her şeyi
bir döngüye taşıdık. Döngü içindeki satırları her biri için dört boşluk daha girintilediğinizden emin olun
ve programı tekrar çalıştırın. Program şimdi sonsuza kadar başka bir tahmin isteyecek,
bu da aslında yeni bir sorun ortaya çıkarıyor. Kullanıcı programdan çıkabilecek gibi görünmüyor!

Kullanıcı her zaman
<kbd>ctrl</kbd>-<kbd>c</kbd> klavye kısayolunu kullanarak programı yarıda kesebilir. Ancak bu doyumsuz
canavarından kaçmanın başka bir yolu daha var, [“Tahmin ile
Gizli Sayının Karşılaştırılması”](#comparing-the-guess-to-the-secret-number)<!-- ignore --> bölümündeki `parse' tartışmasında belirtildiği gibi: eğer
kullanıcı sayı olmayan bir cevap girerse, program çökecektir. Burada gösterildiği gibi, kullanıcının çıkmasına izin vermek için
adresinden yararlanabiliriz:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Quit' yazıldığında oyundan çıkılır, ancak fark edeceğiniz gibi,
adresine sayı olmayan başka bir girdi de girilir. Bu en hafif tabirle yetersizdir;
oyununun doğru sayı tahmin edildiğinde de durmasını istiyoruz.

### Doğru Tahminden Sonra Çıkmak

Bir `break` deyimi ekleyerek kullanıcı kazandığında oyunu bırakacak şekilde programlayalım:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

“Sen kazandın!”dan sonra `break` satırının eklenmesi,
kullanıcı gizli numarayı doğru tahmin ettiğinde programın döngüden çıkmasını sağlar. Döngüden çıkmak aynı zamanda
programdan çıkmak anlamına gelir, çünkü döngü `main`in son parçasıdır.

### Geçersiz Girdiyi İşleme

Oyunun davranışını daha da iyileştirmek için,
kullanıcı sayı olmayan bir girdi girdiğinde programı çökertmek yerine, oyunun sayı olmayan bir girdiyi yok saymasını sağlayalım, böylece
kullanıcısı tahmin etmeye devam edebilir. Bunu, Liste 2-5'te gösterildiği gibi, `guess`
adresinin bir `String`den bir `u32`ye dönüştürüldüğü satırı değiştirerek yapabiliriz.

<Listing number="2-5" file-name="src/main.rs" caption="Ignoring a non-number guess and asking for another guess instead of crashing the program">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

Bir hatada
adresinin çökmesinden hatayı ele almaya geçmek için `expect` çağrısından `match` ifadesine geçiyoruz. Parse` öğesinin bir `Result`
türü döndürdüğünü ve `Result` öğesinin `Ok` ve `Err` değişkenlerine sahip bir enum olduğunu unutmayın. Burada
bir `match` ifadesi kullanıyoruz, tıpkı `cmp`
yönteminin `Ordering` sonucuyla yaptığımız gibi.

Eğer `parse` dizeyi başarılı bir şekilde sayıya dönüştürebilirse,
sonuçta elde edilen sayıyı içeren bir `Ok` değeri döndürecektir. Bu `Ok` değeri
ilk kolun kalıbıyla eşleşecek ve `match` ifadesi sadece `parse` yönteminin ürettiği ve `Ok` değerinin içine koyduğu
`num` değerini döndürecektir. Bu sayı
oluşturduğumuz yeni `guess` değişkeninde tam istediğimiz yerde olacaktır.

Eğer `parse` stringi bir sayıya dönüştüremezse, hata hakkında daha fazla bilgi içeren bir
`Err` değeri döndürecektir. `Err` değeri
ilk `match` kolundaki `Ok(num)` kalıbıyla eşleşmez, ancak
ikinci koldaki `Err(_)` kalıbıyla eşleşir. Alt çizgi, `_`, bir
catch-all değeridir; bu örnekte, içlerinde hangi bilgi olursa olsun, tüm `Err`
değerleriyle eşleşmek istediğimizi söylüyoruz. Böylece program
ikinci kolun kodu olan `continue` kodunu çalıştırır, bu da programa
`loop`un bir sonraki yinelemesine gitmesini ve başka bir tahmin istemesini söyler. Böylece,
programı `parse` programının karşılaşabileceği tüm hataları görmezden gelir!

Şimdi programdaki her şey beklendiği gibi çalışmalıdır. Hadi deneyelim:
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Müthiş! Son bir küçük değişiklikle tahmin oyununu bitireceğiz. Hatırlayın
program hala gizli numarayı yazdırıyor. Bu
testi için iyi çalışıyordu, ancak oyunu mahvediyor. gizli numarasını çıktı olarak veren `println!` kısmını silelim. Liste 2-6 son kodu göstermektedir.

<Listing number="2-6" file-name="src/main.rs" caption="Complete guessing game code">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

Bu noktada, tahmin oyununu başarıyla inşa ettiniz. Tebrikler!

## Özet

Bu proje, sizi birçok yeni Rust kavramıyla tanıştırmanın uygulamalı bir yoluydu:
let`, `match`, fonksiyonlar, harici crate kullanımı ve daha fazlası. Sonraki
birkaç bölümde, bu kavramları daha ayrıntılı olarak öğreneceksiniz. Bölüm 3
değişkenler, veri
türleri ve fonksiyonlar gibi çoğu programlama dilinde bulunan kavramları kapsar ve bunların Rust'ta nasıl kullanılacağını gösterir. Bölüm 4, Rust'ı diğer dillerden farklı kılan bir özellik olan
sahipliğini araştırıyor. Bölüm 5
yapıları ve yöntem sözdizimini tartışır ve Bölüm 6 enumların nasıl çalıştığını açıklar.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: ../std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
