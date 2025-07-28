## Merhaba, Dünya!

Artık Rust'ı kurduğunuza göre, ilk Rust programınızı yazmanın zamanı geldi.
Yeni bir dil öğrenirken
ekrana `Hello, world!` metnini yazdıran küçük bir program yazmak gelenekseldir, bu yüzden burada da aynısını yapacağız!

> Not: Bu kitap komut satırına temel düzeyde aşina olunduğunu varsaymaktadır. Rust,
> düzenleme veya araçlarınız veya kodunuzun nerede bulunduğu hakkında özel bir talepte bulunmaz, bu nedenle
> komut satırı yerine entegre bir geliştirme ortamı (IDE) kullanmayı tercih ediyorsanız, en sevdiğiniz IDE'yi kullanmaktan çekinmeyin. Pek çok IDE artık
> Rust desteğine sahiptir; ayrıntılar için IDE'nin belgelerini kontrol edin. Rust
> ekibi `rust-analyzer' aracılığıyla mükemmel IDE desteği sağlamaya odaklanmaktadır. Daha fazla ayrıntı için
> [Appendix D][devtools]<!-- ignore --> adresine bakın.

### Proje Dizini Oluşturma

Rust kodunuzu saklamak için bir dizin oluşturarak başlayacaksınız. Kodunuzun nerede bulunduğu Rust için önemli değildir
ancak bu kitaptaki alıştırmalar ve projeler için
ev dizininizde bir _projects_ dizini oluşturmanızı ve tüm projelerinizi
orada tutmanızı öneririz.

Bir terminal açın ve
adresinde bir _projects_ dizini ve _projects_ dizini içinde “Hello, world!” projesi için bir dizin oluşturmak için aşağıdaki komutları girin.

Linux, macOS ve Windows'ta PowerShell için şunu girin:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD için şunu girin:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### Bir Rust Programı Yazmak ve Çalıştırmak

Ardından, yeni bir kaynak dosyası oluşturun ve _main.rs_ olarak adlandırın. Rust dosyaları her zaman
_.rs_ uzantısı ile biter. Dosya adınızda birden fazla kelime kullanıyorsanız,
kuralı bunları ayırmak için alt çizgi kullanmaktır. Örneğin, _helloworld.rs_ yerine
_hello_world.rs_ kullanın.

Şimdi yeni oluşturduğunuz _main.rs_ dosyasını açın ve Liste 1-1'deki kodu girin.

<Listeleme number="1-1" file-name="main.rs" caption="Merhaba, dünya!"` yazdıran bir program">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

Dosyayı kaydedin ve
_~/projects/hello_world_ dizinindeki terminal pencerenize geri dönün. Linux veya macOS üzerinde, dosyayı derlemek ve çalıştırmak için aşağıdaki
komutlarını girin:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

Windows'ta `./main` yerine `.\main` komutunu girin:

```powershell
> rustc main.rs
> .\main
Hello, world!
```

İşletim sisteminiz ne olursa olsun, `Hello, world!` dizesi
terminaline yazdırılmalıdır. Bu çıktıyı göremiyorsanız, yardım almanın yolları için
[“Sorun Giderme”][troubleshooting]<!-- ignore --> Kurulum
bölümüne geri dönün.

Eğer `Hello, world!` yazdırıldıysa, tebrikler! Resmi olarak bir Rust
programı yazdınız. Bu sizi bir Rust programcısı yapar-hoş geldiniz!

### Bir Rust Programının Anatomisi

Bu “Merhaba, dünya!” programını ayrıntılı olarak inceleyelim. İşte bulmacanın ilk parçası
:

```rust
fn main() {

}
```

Bu satırlar `main` adında bir fonksiyon tanımlar. `main` fonksiyonu özeldir:
her çalıştırılabilir Rust programında çalışan ilk koddur. Burada,
ilk satır parametresi olmayan ve
hiçbir şey döndürmeyen `main` adında bir fonksiyon tanımlar. Eğer parametreler olsaydı, bunlar `()` parantezlerinin içinde olurdu.

Fonksiyon gövdesi `{}` içine sarılmıştır. Rust, tüm
fonksiyon gövdelerinin etrafında küme parantezleri gerektirir. Açılış küme parantezini fonksiyon bildirimi ile aynı
satıra yerleştirmek ve araya bir boşluk eklemek iyi bir stildir.

> Not: Rust projelerinde standart bir stile bağlı kalmak istiyorsanız, kodunuzu
> belirli bir stilde biçimlendirmek için
> `rustfmt` adlı otomatik biçimlendirme aracını kullanabilirsiniz (`rustfmt` hakkında daha fazla bilgi için
> [Appendix D][devtools]<!-- ignore -->). Rust ekibi bu aracı
> `rustc` gibi standart Rust dağıtımına dahil etmiştir, bu nedenle bilgisayarınızda zaten
> yüklü olmalıdır!

main` fonksiyonunun gövdesi aşağıdaki kodu içerir:

```rust
println!("Hello, world!");
```

Bu satır, bu küçük programdaki tüm işi yapar:
ekranına metin yazdırır. Burada dikkat edilmesi gereken üç önemli ayrıntı var.

İlk olarak, `println!` bir Rust makrosu çağırıyor. Bunun yerine bir fonksiyon çağırmış olsaydı,
`println` olarak girilirdi (`!` olmadan). Rust makroları, Rust sözdizimini genişletmek için kod üreten
kodu yazmanın bir yoludur ve bunları [Bölüm 20][ch20-macros]<!-- ignore --> bölümünde daha
ayrıntılı olarak tartışacağız. Şimdilik, sadece
`!` kullanmanın normal bir
işlevi yerine bir makro çağırdığınız anlamına geldiğini ve makroların her zaman işlevlerle aynı kuralları izlemediğini bilmeniz gerekir.

İkinci olarak, `"Merhaba, dünya!"` dizesini görüyorsunuz. Bu dizeyi
argümanı olarak `println!`'e aktarıyoruz ve dize ekrana yazdırılıyor.

Üçüncü olarak, satırı bu
ifadesinin bittiğini ve bir sonrakinin başlamaya hazır olduğunu belirten bir noktalı virgül (`;`) ile bitiriyoruz. Rust kodunun çoğu satırı
noktalı virgülle biter.

### Derleme ve Çalıştırma Ayrı Adımlardır

Yeni oluşturduğunuz bir programı çalıştırdınız, şimdi
sürecindeki her adımı inceleyelim.

Bir Rust programını çalıştırmadan önce,
adresine `rustc` komutunu girerek ve ona kaynak dosyanızın adını vererek Rust derleyicisini kullanarak derlemelisiniz, örneğin
bu:

```console
$ rustc main.rs
```

C veya C++ geçmişiniz varsa, bunun `gcc`
veya `clang` ile benzer olduğunu fark edeceksiniz. Başarılı bir şekilde derlendikten sonra, Rust ikili bir çalıştırılabilir çıktı verir.

Linux, macOS ve Windows'ta PowerShell'de çalıştırılabilir dosyayı
adresinden kabuğunuzdaki `ls` komutunu girerek görebilirsiniz:

```console
$ ls
main  main.rs
```

Linux ve macOS'ta iki dosya görürsünüz. Windows'ta PowerShell ile
CMD kullanarak göreceğiniz aynı üç dosyayı görürsünüz. Windows'ta CMD ile
aşağıdaki komutu girebilirsiniz:

```cmd
> dir /B %= /B seçeneği yalnızca dosya adlarını göstermeyi söyler =%
main.exe
main.pdb
main.rs
```

Bu, _.rs_ uzantılı kaynak kod dosyasını,
 çalıştırılabilir dosyasını (Windows'ta _main.exe_, ancak diğer tüm platformlarda _main_) ve
Windows kullanırken, _.pdb_ uzantılı hata ayıklama bilgilerini içeren bir dosyayı gösterir.
Buradan _main_ veya _main.exe_ dosyasını aşağıdaki gibi çalıştırırsınız:

```console
$ ./main # ya da .\main windows da
```

Eğer _main.rs_ programınız “Merhaba, dünya!” programınız ise, bu satır terminalinize `Merhaba,
dünya!"` yazdırır.

Ruby, Python veya
JavaScript gibi dinamik bir dile daha aşinaysanız, bir programı
ayrı adımlar olarak derlemeye ve çalıştırmaya alışık olmayabilirsiniz. Rust _zamanından önce derlenen_ bir dildir, yani bir programı
derleyebilir ve çalıştırılabilir dosyayı başka birine verebilirsiniz ve Rust yüklü olmasa bile
çalıştırabilir. Eğer birine bir _.rb_, _.py_ veya
_.js_ dosyası verirseniz, Ruby, Python veya JavaScript uygulamasının
yüklü olması gerekir (sırasıyla). Ancak bu dillerde, programınızı
derlemek ve çalıştırmak için yalnızca bir komuta ihtiyacınız vardır. Dil tasarımında her şey bir ödünleşimdir.

Sadece `rustc` ile derleme basit programlar için iyidir, ancak projeniz
büyüdükçe, tüm seçenekleri yönetmek ve
kodunuzu paylaşmayı kolaylaştırmak isteyeceksiniz. Daha sonra, size
gerçek dünya Rust programları yazmanıza yardımcı olacak Cargo aracını tanıtacağız.

[troubleshooting]:ch01-01-installation.md#sorun-giderme
[devtools]:appendix-04-useful-development-tools.md
[ch20-macros]:ch20-05-macros.md
