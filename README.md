# Rust Programlama Dili

![Derleme Durumu](https://github.com/rust-lang/book/workflows/CI/badge.svg)

Bu depo, “Rust Programlama Dili” kitabının kaynağını içerir.

[Kitap, No Starch Press tarafından basılı olarak da temin edilebilir][nostarch].
[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Kitabı çevrimiçi olarak ücretsiz olarak da okuyabilirsiniz. Lütfen kitabı, en son [stable], [beta] veya [nightly] Rust sürümleriyle birlikte gönderilen haliyle inceleyin. Bu sürümlerdeki sorunların, bu depoda zaten düzeltilmiş olabileceğini unutmayın, çünkü bu sürümler daha seyrek güncellenmektedir.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

Kitapta yer alan tüm kod listelerinin kodlarını indirmek için [sürümler] bölümüne bakın.
[releases]: https://github.com/rust-lang/book/releases

## Gereksinimler

Kitabı oluşturmak için [mdBook] gerekir, ideal olarak [bu dosyada][rust-mdbook] rust-lang/rust'un kullandığı sürümle aynı sürüm.

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/master/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --locked --version <version_num>
```

Kitap ayrıca bu deponun bir parçası olan iki mdbook eklentisini kullanır. Bunları yüklemezseniz, derleme sırasında uyarılar görürsünüz ve çıktı doğru görünmez, ancak kitabı yine de derleyebilirsiniz. Eklentileri kullanmak için şunu çalıştırmalısınız:

```bash
$ cargo install --locked --path packages/mdbook-trpl --force
```

## Building

Kitabı oluşturmak için şunu yazın:

```bash
$ mdbook build
```

Çıktı, `book` alt dizininde olacaktır. Kontrol etmek için, web tarayıcınızda açın.

_Firefox:_

```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_

```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Testleri çalıştırmak için:

```bash
$ cd packages/trpl
$ mdbook test --library-path packages/trpl/target/debug/deps
```

## Katkı Sağlama

Yardımınıza ihtiyacımız var! Aradığımız katkı türleri hakkında bilgi almak için lütfen [CONTRIBUTING.md][contrib] dosyasına bakın.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Kitap [basılı][nostarch] olduğundan ve kitabın çevrimiçi sürümünü mümkün olduğunca basılı sürüme yakın tutmak istediğimizden,
sorununuzu veya çekme isteğinizi ele almamız normalde alıştığınız süreden daha uzun sürebilir.
Şu ana kadar, [Rust Editions](https://doc.rust-lang.org/edition-guide/) ile eşzamanlı olarak daha büyük bir revizyon yapıyoruz.
Bu büyük revizyonlar arasında, sadece hataları düzelteceğiz.

Şimdiye kadar, [Rust Editions](https://doc.rust-lang.org/edition-guide/) ile eşzamanlı olarak daha büyük bir revizyon yapıyoruz. Bu büyük revizyonlar arasında, sadece hataları düzelteceğiz. Sorununuz veya çekme isteğiniz bir hatayı düzeltmekle sınırlı değilse, bir sonraki büyük revizyon çalışmamıza kadar beklemede kalabilir: bu, aylar veya yıllar sürebilir. Sabrınız için teşekkür ederiz!

### Çeviriler

Kitabın çevirisine yardımcı olmak ister misiniz? [Çeviriler] etiketine bakarak şu anda devam eden çalışmalara katılabilirsiniz.
Yeni bir dil üzerinde çalışmaya başlamak için yeni bir konu açın!
Birden fazla dil için [mdbook desteği] bekliyoruz,
ancak siz çalışmaya başlayabilirsiniz!

[Çeviriler]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook desteği]: https://github.com/rust-lang/mdBook/issues/5

## Yazım Denetimi

Kaynak dosyalarında yazım hatalarını taramak için, `ci` dizininde bulunan `spellcheck.sh`
komut dosyasını kullanabilirsiniz. Bu komut dosyası, geçerli kelimelerin bulunduğu bir sözlüğe ihtiyaç duyar ve bu sözlük `ci/dictionary.txt` dosyasında bulunur. Komut dosyası yanlış bir pozitif sonuç verirse (örneğin, komut dosyasının geçersiz saydığı `BTreeMap` kelimesini kullandınız), bu kelimeyi `ci/dictionary.txt` dosyasına eklemeniz gerekir (tutarlılık için sıralı sırayı koruyun).