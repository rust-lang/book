## Kurulum

İlk adım Rust'ı yüklemektir. Rust'ı, Rust sürümlerini ve ilgili araçları yönetmek için bir
komut satırı aracı olan `rustup' aracılığıyla indireceğiz. İndirme işlemi için
bir internet bağlantısına ihtiyacınız olacak.

> Not: Herhangi bir nedenle `rustup` kullanmayı tercih etmiyorsanız, daha fazla seçenek için lütfen
> [Diğer Rust Kurulum Yöntemleri sayfasına] [otherinstall] bakın.

Aşağıdaki adımlar Rust derleyicisinin en son kararlı sürümünü yükler.
Rust'ın kararlılık garantileri, kitaptaki
derlenen tüm örneklerin daha yeni Rust sürümleriyle derlenmeye devam etmesini sağlar. Rust genellikle hata mesajlarını ve
uyarılarını geliştirdiği için çıktılar
sürümler arasında biraz farklılık gösterebilir. Başka bir deyişle,
bu adımları kullanarak yüklediğiniz herhangi bir yeni, kararlı Rust sürümü, bu kitabın içeriğiyle beklendiği gibi çalışmalıdır.

> ### Komut Satırı Gösterimi
>
> Bu bölümde ve kitap boyunca,
> terminalinde kullanılan bazı komutları göstereceğiz. Terminalde girmeniz gereken satırların tümü `$` ile başlar. > `$` karakterini yazmanıza gerek yoktur; bu,
> her komutun başlangıcını belirtmek için gösterilen komut satırı istemidir. $` ile başlamayan satırlar genellikle
> bir önceki komutun çıktısını gösterir. Ayrıca, PowerShell'e özgü
> örnekleri `$` yerine `>` kullanacaktır.

### Linux veya macOS üzerinde `rustup` kurulumu

Linux veya macOS kullanıyorsanız, bir terminal açın ve aşağıdaki komutu girin:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Komut bir betik indirir ve Rust'ın en son kararlı sürümünü yükleyen `rustup`
aracının kurulumunu başlatır. Parolanız için
adresine yönlendirilebilirsiniz. Yükleme başarılı olursa, aşağıdaki satır görünecektir:

```text
Rust is installed now. Great!
```

Ayrıca, Rust'ın
derlenmiş çıktılarını tek bir dosyada birleştirmek için kullandığı bir program olan _linker_'a da ihtiyacınız olacak. Muhtemelen sizde zaten bir tane vardır. Eğer
linker hataları alıyorsanız, genellikle
linker içeren bir C derleyicisi kurmalısınız. Bazı yaygın Rust paketleri
C koduna bağlı olduğundan ve bir C derleyicisine ihtiyaç duyacağından, bir C derleyicisi de yararlıdır.

macOS üzerinde, çalıştırarak bir C derleyicisi edinebilirsiniz:
```console
$ xcode-select --install
```

Linux kullanıcıları genellikle
dağıtımlarının belgelerine göre GCC veya Clang'ı yüklemelidir. Örneğin, Ubuntu kullanıyorsanız,
`build-essential` paketini yükleyebilirsiniz.

### Windows üzerinde `rustup` kurulumu

Windows'ta, [https://www.rust-lang.org/tools/install][install] adresine gidin ve
adresindeki Rust yükleme talimatlarını izleyin. Kurulumun bir noktasında
adresinden Visual Studio'yu yüklemeniz istenecektir. Bu, bir bağlayıcı ve programları derlemek için gereken yerel
kütüphanelerini sağlar. Bu adımla ilgili daha fazla yardıma ihtiyacınız olursa, bkz.
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]

Bu kitabın geri kalanında hem _cmd.exe_ hem de PowerShell'de çalışan komutlar kullanılmaktadır.
Belirli farklılıklar varsa, hangisinin kullanılacağını açıklayacağız.

### troubleshooting

Rust'ın doğru kurulup kurulmadığını kontrol etmek için bir kabuk açın ve şu
satırını girin:

```console
$ rustc --version
```

Yayınlanan en son
kararlı sürümü için sürüm numarasını, commit hash'ini ve commit tarihini aşağıdaki formatta görmelisiniz:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Eğer bu bilgiyi görüyorsanız, Rust'ı başarıyla kurmuşsunuz demektir! Eğer
bu bilgiyi göremiyorsanız, Rust'ın `%PATH%' sistem değişkeninizde olup olmadığını
aşağıdaki gibi kontrol edin.

Windows CMD'de şunu kullanın:

```console
> echo %PATH%
```

In PowerShell, use:

```powershell
> echo $env:Path
```

In Linux and macOS, use:

```console
$ echo $PATH
```

Tüm bunlar doğruysa ve Rust hala çalışmıyorsa, yardım alabileceğiniz birkaç
yer vardır. Diğer Rustacean'larla (kendimize taktığımız
aptalca bir takma ad) nasıl iletişime geçeceğinizi [topluluk sayfası][topluluk] adresinden öğrenin.

#### Update and Uninstall

Rust, `rustup` aracılığıyla kurulduktan sonra, yeni yayınlanan bir sürüme güncellemek
kolaydır. Kabuğunuzdan aşağıdaki güncelleme betiğini çalıştırın:

```console
$ rustup update
```

Rust ve `rustup`ı kaldırmak için
kabuğunuzdan aşağıdaki kaldırma betiğini çalıştırın:

```console
$ rustup self uninstall
```

### Local Documentation

Rust kurulumu ayrıca
adresinden çevrimdışı okuyabileceğiniz belgelerin yerel bir kopyasını da içerir. Yerel belgeleri tarayıcınızda
açmak için `rustup doc` çalıştırın.

Standart kütüphane tarafından bir tür veya işlev sağlandığında ve bunun ne işe yaradığından veya nasıl kullanılacağından
emin olmadığınızda, öğrenmek için uygulama programlama arayüzü
(API) belgelerini kullanın!

### Text Editors and Integrated Development Environments

Bu kitap, Rust kodu yazmak için hangi araçları kullandığınız konusunda hiçbir varsayımda bulunmaz.
Hemen hemen her metin editörü işinizi görecektir! Ancak, birçok metin editörü ve
entegre geliştirme ortamı (IDE) Rust için yerleşik desteğe sahiptir. Rust web sitesindeki [tools
sayfasında][tools] birçok düzenleyici ve IDE'nin oldukça güncel bir listesini her zaman bulabilirsiniz.

#### Working Offline with this Book

Birkaç örnekte, standart kütüphanenin ötesinde Rust paketlerini kullanacağız. Bu örnekler üzerinde
çalışmak için ya internet bağlantınızın olması
ya da bu bağımlılıkları önceden indirmiş olmanız gerekecektir. bağımlılıklarını önceden indirmek için aşağıdaki komutları çalıştırabilirsiniz. (
adresinde `cargo`nun ne olduğunu ve bu komutların her birinin ne işe yaradığını daha sonra ayrıntılı olarak açıklayacağız).

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```

Bu, bu paketlerin indirilmesini önbelleğe alacaktır, böylece daha sonra
adresinden indirmenize gerek kalmayacaktır. Bu komutu çalıştırdıktan sonra
`get-dependencies` klasörünü saklamanıza gerek yoktur. Bu komutu çalıştırdıysanız, ağı kullanmaya çalışmak yerine bu
önbelleğe alınmış sürümleri kullanmak için kitabın geri kalanındaki tüm `cargo` komutlarıyla birlikte
`--offline` bayrağını kullanabilirsiniz.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
