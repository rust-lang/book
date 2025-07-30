## Hata Mesajlarını Standart Çıktı Yerine Standart Hata Çıktısına Yazma

Şu anda, tüm çıktılarımızı
`println!` makrosunu kullanarak terminale yazıyoruz. Çoğu terminalde iki tür çıktı vardır: genel bilgiler için _standart
çıktı_ (`stdout`) ve hata mesajları için _standart hata_ (`stderr`)
. Bu ayrım, kullanıcıların bir programın başarılı çıktısını bir dosyaya yönlendirirken
hata mesajlarını ekrana yazdırmayı seçmelerine olanak tanır.
`println!`

makrosu yalnızca standart çıktıya yazdırma yapabilir, bu nedenle standart hataya yazdırmak için başka bir şey kullanmamız gerekir.
### Hataların Nereye Yazdırıldığını Kontrol Etme

### Hataların Nereye Yazıldığını Kontrol Etme

Öncelikle, `minigrep` tarafından yazdırılan içeriğin, standart hataya yazdırmak istediğimiz
hata mesajları da dahil olmak üzere, şu anda standart çıktıya nasıl yazıldığını
inceleyelim. Bunu, standart çıktı akışını bir dosyaya yönlendirirken
kasıtlı olarak bir hata oluşturarak yapacağız. Standart hata akışını yönlendirmeyeceğiz,
bu nedenle standart hataya gönderilen tüm içerik ekranda görüntülenmeye devam
edecektir.

Komut satırı programlarının hata mesajlarını standart hata akışına göndermesi
beklenir, böylece standart çıktı akışını bir dosyaya yönlendirdiğimizde bile
ekranda hata mesajlarını görebiliriz. Programımız şu anda iyi çalışmıyor:
hata mesajı çıktısını bir dosyaya kaydettiğini göreceğiz!

Bu davranışı göstermek için, programı `>` ve standart çıktı akışını yönlendirmek istediğimiz dosya yolu
_output.txt_ ile çalıştıracağız. Herhangi bir argüman
geçirmeyeceğiz, bu da bir hataya neden olacaktır:

```console
$ cargo run > output.txt
```

`>` sözdizimi, kabuğa standart çıktının içeriğini ekrana değil
_output.txt_ dosyasına yazmasını söyler. Beklediğimiz hata mesajının ekrana
yazdırıldığını görmedik, bu da mesajın dosyaya yazdırılmış olması gerektiği
anlamına gelir. _output.txt_ dosyasının içeriği şöyledir:

```text
Problem parsing arguments: not enough arguments
```

Evet, hata mesajımız standart çıktıya yazdırılıyor. Bu tür hata mesajlarının standart hataya yazdırılması çok daha
yararlıdır, böylece yalnızca
başarılı bir çalışmanın verileri dosyaya kaydedilir. Bunu değiştireceğiz.

### Hataları Standart Hataya Yazdırma

Hata mesajlarının yazdırılma şeklini değiştirmek için Listing 12-24'teki kodu kullanacağız.
Bu bölümün başında yaptığımız yeniden düzenleme nedeniyle, hata mesajlarını
yazdırmak için kullanılan tüm kodlar tek bir işlevde, `main` içinde yer almaktadır. Standart kütüphane, standart hata akışına yazdırma işlemini gerçekleştiren
`eprintln!` makrosunu sağlar, bu nedenle `println!` işlevini çağırdığımız iki yeri değiştirerek
hataları yazdırmak için `eprintln!` işlevini kullanacağız.
.

<Listing number="12-24" file-name="src/main.rs" caption="Writing error messages to standard error instead of standard output using `eprintln!`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

Şimdi programı aynı şekilde, herhangi bir argüman olmadan ve
standart çıktıyı `>` ile yeniden yönlendirerek tekrar çalıştıralım:

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Şimdi ekranda hatayı görüyoruz ve _output.txt_ dosyası hiçbir şey içermiyor, bu da
komut satırı programlarından beklediğimiz davranış.

Programı, hata oluşturmayan ancak yine de
standart çıktıyı bir dosyaya yönlendiren argümanlarla tekrar çalıştıralım, şöyle:

```console
$ cargo run -- to poem.txt > output.txt
```

Terminalde herhangi bir çıktı görmeyeceğiz ve _output.txt_ dosyası sonuçlarımızı içerecektir:

<span class="filename">Filename: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

Bu, artık başarılı çıktılar için standart çıktıyı ve hata çıktıları için standart hatayı uygun şekilde kullandığımızı gösterir.


## Özet

Bu bölümde, şimdiye kadar öğrendiğiniz bazı temel kavramları özetledik ve
Rust'ta yaygın I/O işlemlerinin nasıl gerçekleştirileceğini ele aldık. Komut satırı
argümanları, dosyalar, ortam değişkenleri ve hataları yazdırmak için `eprintln!` makrosunu kullanarak,
artık komut satırı uygulamaları yazmaya hazırsınız. Önceki bölümlerdeki kavramlarla
birleştirildiğinde, kodunuz iyi organize olacak, verileri uygun veri
yapılarında etkili bir şekilde depolayacak, hataları iyi bir şekilde
ele alacak ve iyi test edilecektir.

Şimdi, fonksiyonel dillerden etkilenen bazı Rust özelliklerini
inceleyeceğiz: kapanışlar ve yineleyiciler.
