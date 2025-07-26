## Kurulum

İlk adım Rust'ı yüklemektir. Rust'ı, Rust sürümlerini ve ilgili araçları yönetmek için bir
komut satırı aracı olan `rustup' aracılığıyla indireceğiz. İndirme işlemi için
bir internet bağlantısına ihtiyacınız olacak.

> Not: Herhangi bir nedenle `rustup` kullanmayı tercih etmiyorsanız, daha fazla seçenek için lütfen
> [Diğer Rust Kurulum Yöntemleri sayfasına][otherinstall] bakın.

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
>
> örnekleri `$` yerine `>` kullanacaktır.

### Linux veya macOS üzerinde `rustup` kurulumu

Linux veya macOS kullanıyorsanız, bir terminal açın ve aşağıdaki komutu girin:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Komut bir komut dosyası indirir ve Rust'ın en son kararlı sürümünü yükleyen `rustup' aracının kurulumunu başlatır.

Parolanız için adresine yönlendirilebilirsiniz. Yükleme başarılı olursa, aşağıdaki satır görünecektir:

```text
Rust is installed now. Great!
```

Ayrıca, Rust'ın
derlenmiş çıktılarını tek bir dosyada birleştirmek için kullandığı bir program olan `_linker_`a da ihtiyacınız olacak. Muhtemelen sizde zaten bir tane vardır. 
Eğer linker hataları alıyorsanız, genellikle
linker içeren bir C derleyicisi kurmalısınız. C derleyicisi de kullanışlıdır çünkü bazı yaygın Rust paketleri
C koduna bağlıdır ve bir C derleyicisine ihtiyaç duyarlar.

macOS üzerinde, çalıştırarak bir C derleyicisi edinebilirsiniz:

macOS üzerinde, çalıştırarak bir C derleyicisi edinebilirsiniz:
```console
$ xcode-select --install
```

Linux kullanıcıları genellikle
dağıtımlarının belgelerine göre GCC veya Clang'ı yüklemelidir. Örneğin, Ubuntu kullanıyorsanız,
`build-essential` paketini yükleyebilirsiniz.

### Windows üzerinde `rustup` kurulumu

Windows'ta, [https://www.rust-lang.org/tools/install][install] adresine gidin ve Rust'ı yüklemek için
talimatlarını izleyin. Kurulumun bir noktasında
adresinden Visual Studio'yu yüklemeniz istenecektir. Bu, bir bağlayıcı ve programları derlemek için gereken yerel
kütüphanelerini sağlar. Bu adımla ilgili daha fazla yardıma ihtiyacınız varsa,
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc] adresine bakın.

Bu kitabın geri kalanında hem _cmd.exe_ hem de PowerShell'de çalışan komutlar kullanılmaktadır.
Belirli farklılıklar varsa, hangisinin kullanılacağını açıklayacağız.

### Sorun Giderme

Rust'ın doğru kurulup kurulmadığını kontrol etmek için bir kabuk açın ve şunu girin
line:

```console
$ rustc --version
```

Yayınlanan en son
kararlı sürümünün sürüm numarasını, commit hash'ini ve commit tarihini aşağıdaki formatta görmelisiniz:

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

PowerShell'de şunu kullanın:

```powershell
> echo $env:Path
```

Linux ve macOS'ta şunu kullanın:

```console
$ echo $PATH
```

Tüm bunlar doğruysa ve Rust hala çalışmıyorsa, yardım alabileceğiniz birkaç
yer vardır. Diğer Rustacean'larla (kendimize taktığımız aptalca bir
takma ad) nasıl iletişime
geçebileceğinizi [topluluk sayfası][community] adresinden öğrenebilirsiniz.

### Güncelleme ve Kaldırma

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

### Yerel Dokümantasyon

Rust kurulumu ayrıca
adresinden çevrimdışı okuyabileceğiniz belgelerin yerel bir kopyasını da içerir. Yerel belgeleri tarayıcınızda
açmak için `rustup doc` çalıştırın.

Standart kütüphane tarafından bir tür veya işlev sağlandığında ve ne işe yaradığından veya nasıl kullanılacağından
emin olmadığınızda, öğrenmek için uygulama programlama arayüzü
(API) belgelerini kullanın!

### Metin Düzenleyicileri ve Tümleşik Geliştirme Ortamları

Bu kitap, Rust kodu yazmak için hangi araçları kullandığınız konusunda hiçbir varsayımda bulunmaz.
Hemen hemen her metin editörü işinizi görecektir! Ancak, birçok metin editörü ve
entegre geliştirme ortamı (IDE) Rust için yerleşik desteğe sahiptir. Siz
Rust web sitesindeki [araçlar sayfasında][araçlar] birçok editör ve IDE'nin oldukça güncel bir listesini her zaman bulabilirsiniz.

### Bu Kitapla Çevrimdışı Çalışmak

Birkaç örnekte, standart kütüphanenin ötesinde Rust paketlerini kullanacağız. Bu örnekler üzerinde
çalışmak için ya internet bağlantınızın olması
ya da bu bağımlılıkları önceden indirmiş olmanız gerekecektir. bağımlılıklarını önceden indirmek için aşağıdaki komutları çalıştırabilirsiniz. (adresinde `cargo`nun ne olduğunu ve bu komutların her birinin ne işe yaradığını daha sonra ayrıntılı olarak açıklayacağız).

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```

Bu, bu paketler için indirmeleri önbelleğe alacak ve böylece daha sonra
adresinden indirmenize gerek kalmayacaktır. Bu komutu çalıştırdıktan sonra
`get-dependencies` klasörünü saklamanıza gerek yoktur. Bu komutu çalıştırdıysanız, ağı kullanmaya çalışmak yerine bu
önbelleğe alınmış sürümleri kullanmak için kitabın geri kalanındaki tüm `cargo` komutlarıyla birlikte
`--offline` bayrağını kullanabilirsiniz.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[araçlar]: https://www.rust-lang.org/tools

